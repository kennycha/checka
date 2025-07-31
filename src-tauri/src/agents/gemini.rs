use super::{AgentMonitor, AgentStatus};
use libc::{c_int, c_void};
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_char;
use std::process::Command;

const PROC_ALL_PIDS: u32 = 1;
const PROC_PIDPATHINFO_MAXSIZE: usize = 4096;

extern "C" {
    fn proc_listpids(p_type: u32, typeinfo: u32, buffer: *mut c_void, buffersize: c_int) -> c_int;
    fn proc_pidpath(pid: c_int, buffer: *mut c_char, buffersize: u32) -> c_int;
}

use std::sync::Mutex;

pub struct GeminiMonitor {
    cached_pid: Mutex<Option<i32>>,
}

impl GeminiMonitor {
    pub fn new() -> Self {
        Self {
            cached_pid: Mutex::new(None),
        }
    }

    fn get_all_pids() -> Vec<i32> {
        let mut pids = vec![0i32; 2048];
        unsafe {
            let size = proc_listpids(PROC_ALL_PIDS, 0, pids.as_mut_ptr() as *mut c_void, (pids.len() * mem::size_of::<i32>()) as i32);
            let count = size as usize / mem::size_of::<i32>();
            pids.truncate(count);
        }
        pids.into_iter().filter(|&p| p > 0).collect()
    }

    fn get_process_path(pid: i32) -> Option<String> {
        let mut buffer = vec![0u8; PROC_PIDPATHINFO_MAXSIZE];
        unsafe {
            let ret = proc_pidpath(pid, buffer.as_mut_ptr() as *mut c_char, PROC_PIDPATHINFO_MAXSIZE as u32);
            if ret > 0 {
                let cstr = CStr::from_ptr(buffer.as_ptr() as *const c_char);
                return Some(cstr.to_string_lossy().into_owned());
            }
        }
        None
    }

    fn get_process_args(pid: i32) -> Option<String> {
        let output = Command::new("ps")
            .args(&["-p", &pid.to_string(), "-o", "args="])
            .output()
            .ok()?;
        
        if output.status.success() {
            let args = String::from_utf8_lossy(&output.stdout);
            Some(args.trim().to_string())
        } else {
            None
        }
    }

    fn get_process_cpu_percent(pid: i32) -> Option<f64> {
        let output = Command::new("ps")
            .args(&["-p", &pid.to_string(), "-o", "pcpu="])
            .output()
            .ok()?;
        
        if output.status.success() {
            let cpu_str = String::from_utf8_lossy(&output.stdout);
            cpu_str.trim().parse::<f64>().ok()
        } else {
            None
        }
    }

    fn get_process_cwd(pid: i32) -> Option<String> {
        let output = Command::new("lsof")
            .args(&["-a", "-d", "cwd", "-p", &pid.to_string(), "-F", "n"])
            .output()
            .ok()?;
        
        if output.status.success() {
            let lsof_output = String::from_utf8_lossy(&output.stdout);
            for line in lsof_output.lines() {
                if line.starts_with('n') {
                    return Some(line[1..].to_string());
                }
            }
        }
        None
    }

    fn get_current_dir() -> Option<String> {
        std::env::current_dir()
            .ok()
            .and_then(|p| p.to_str().map(|s| s.to_string()))
    }

    fn is_same_project(process_cwd: &str, current_dir: &str) -> bool {
        // In production mode, require exact match
        if !cfg!(debug_assertions) {
            return process_cwd == current_dir;
        }
        
        // In development mode, allow parent/child directory relationships
        let process_path = std::path::Path::new(process_cwd);
        let current_path = std::path::Path::new(current_dir);
        
        // Check if process_cwd is the parent of current_dir (typical dev scenario)
        if let Some(parent) = current_path.parent() {
            if parent == process_path {
                return true;
            }
        }
        
        // Check if current_dir is the parent of process_cwd
        if let Some(parent) = process_path.parent() {
            if parent == current_path {
                return true;
            }
        }
        
        // Exact match
        process_cwd == current_dir
    }
}

impl AgentMonitor for GeminiMonitor {
    fn get_status(&self) -> AgentStatus {
        let current_dir = match Self::get_current_dir() {
            Some(dir) => dir,
            None => return AgentStatus::Off,
        };

        let mut cached_pid = self.cached_pid.lock().unwrap();

        // 캐시된 PID가 있으면 해당 PID만 체크 (전체 스캔 안함!)
        if let Some(pid) = *cached_pid {
            // 프로세스가 아직 살아있는지 간단히 체크
            if Self::get_process_cpu_percent(pid).is_some() {
                // 살아있으면 CPU만 체크
                if let Some(cpu_percent) = Self::get_process_cpu_percent(pid) {
                    return if cpu_percent > 1.0 {
                        AgentStatus::Processing
                    } else {
                        AgentStatus::Waiting
                    };
                }
            }
            // PID가 죽었으면 캐시 초기화
            *cached_pid = None;
        }

        // 캐시가 없을 때만 전체 스캔
        let pids = Self::get_all_pids();

        for pid in pids {
            if let Some(path) = Self::get_process_path(pid) {
                let is_gemini_process = if path.contains("python") || path.contains("python3") {
                    Self::get_process_args(pid).map_or(false, |args| {
                        (args.contains("gemini") || 
                         args.contains("google-generativeai") ||
                         args.contains("google-ai") ||
                         args.contains("google.generativeai") ||
                         args.contains("genai") ||
                         args.contains("bard")) && 
                        !args.contains("--version")
                    })
                } else if path.contains("gemini") || path.contains("google-ai") {
                    Self::get_process_args(pid).map_or(false, |args| !args.contains("--version"))
                } else if path.contains("node") {
                    Self::get_process_args(pid).map_or(false, |args| {
                        (args.contains("@google-ai") ||
                         args.contains("google-generativeai") ||
                         args.contains("gemini-cli") ||
                         args.contains("gemini") ||
                         args.contains("bard-cli")) &&
                        !args.contains("--version")
                    })
                } else {
                    false
                };

                if is_gemini_process {
                    if let Some(process_cwd) = Self::get_process_cwd(pid) {
                        if Self::is_same_project(&process_cwd, &current_dir) {
                            *cached_pid = Some(pid);
                            if let Some(cpu_percent) = Self::get_process_cpu_percent(pid) {
                                return if cpu_percent > 1.0 {
                                    AgentStatus::Processing
                                } else {
                                    AgentStatus::Waiting
                                };
                            } else {
                                return AgentStatus::Waiting;
                            }
                        }
                    }
                }
            }
        }
        
        AgentStatus::Off
    }

    fn get_name(&self) -> &'static str {
        "Gemini CLI"
    }

    fn is_available(&self) -> bool {
        // Check if Google Gemini CLI tools are available
        
        // Try common Gemini CLI commands
        let gemini_commands = [
            ("gemini", vec!["--version"]),
            ("google-ai", vec!["--version"]),
            ("bard", vec!["--version"]),
        ];
        
        for (cmd, args) in &gemini_commands {
            if Command::new(cmd).args(args).output().is_ok() {
                return true;
            }
        }
        
        // Check for Python-based installations
        let python_checks = [
            ("python3", vec!["-c", "import google.generativeai"]),
            ("python", vec!["-c", "import google.generativeai"]),
            ("pip3", vec!["show", "google-generativeai"]),
            ("pip", vec!["show", "google-generativeai"]),
        ];
        
        for (cmd, args) in &python_checks {
            if let Ok(output) = Command::new(cmd).args(args).output() {
                if output.status.success() {
                    return true;
                }
            }
        }
        
        // Check for Node.js-based installations
        let node_checks = [
            ("npm", vec!["list", "@google-ai/generativelanguage"]),
            ("npm", vec!["list", "google-generativeai"]),
            ("npx", vec!["--version"]), // Basic check if npx is available
        ];
        
        for (cmd, args) in &node_checks {
            if let Ok(output) = Command::new(cmd).args(args).output() {
                if output.status.success() {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    if output_str.contains("google") || output_str.contains("generative") {
                        return true;
                    }
                }
            }
        }
        
        false
    }
}

