use super::{AgentMonitor, AgentStatus};
use libc::{c_int, c_void};
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_char;
use std::process::Command;

const PROC_ALL_PIDS: u32 = 1;

extern "C" {
    fn proc_listpids(p_type: u32, typeinfo: u32, buffer: *mut c_void, buffersize: c_int) -> c_int;
    fn proc_pidpath(pid: c_int, buffer: *mut c_char, buffersize: u32) -> c_int;
}

const PROC_PIDPATHINFO_MAXSIZE: usize = 4096;

pub struct ClaudeCodeMonitor {}

impl ClaudeCodeMonitor {
    pub fn new() -> Self {
        Self {}
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
        // Use ps command to get command line arguments for specific PID
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
        // Use ps command to get current CPU percentage for specific PID
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
        // Use lsof command to get current working directory for specific PID
        let output = Command::new("lsof")
            .args(&["-a", "-d", "cwd", "-p", &pid.to_string(), "-F", "n"])
            .output()
            .ok()?;
        
        if output.status.success() {
            let lsof_output = String::from_utf8_lossy(&output.stdout);
            // Parse lsof output: lines starting with 'n' contain the path
            for line in lsof_output.lines() {
                if line.starts_with('n') {
                    return Some(line[1..].to_string()); // Remove 'n' prefix
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
}

impl AgentMonitor for ClaudeCodeMonitor {
    fn get_status(&self) -> AgentStatus {
        let current_dir = match Self::get_current_dir() {
            Some(dir) => dir,
            None => return AgentStatus::Off,
        };
        let pids = Self::get_all_pids();
        
        for pid in pids {
            if let Some(path) = Self::get_process_path(pid) {
                // Check if this is a node process - then check command line args
                if path.contains("node") {
                    if let Some(args) = Self::get_process_args(pid) {
                        
                        // Check if args contain claude (project-specific detection)
                        if (args.contains("claude") || 
                            args.contains("@anthropic-ai/claude-code") ||
                            args.contains("claude-code") ||
                            args.contains("npx claude") ||
                            (args.contains("npx") && args.contains("claude"))) && 
                           !args.contains("--version") {
                            
                            // Check if this process is running in current directory
                            if let Some(process_cwd) = Self::get_process_cwd(pid) {
                                if process_cwd == current_dir {
                                    if let Some(cpu_percent) = Self::get_process_cpu_percent(pid) {
                                        // CPU 사용률이 1% 이상이면 Processing, 아니면 Waiting
                                        return if cpu_percent > 1.0 {
                                            AgentStatus::Processing
                                        } else {
                                            AgentStatus::Waiting
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Also check direct claude executable
                if path.contains("claude") || path.contains("@anthropic-ai/claude-code") {
                    // Check if this process is running in current directory
                    if let Some(process_cwd) = Self::get_process_cwd(pid) {
                        if process_cwd == current_dir {
                            if let Some(cpu_percent) = Self::get_process_cpu_percent(pid) {
                                return if cpu_percent > 1.0 {
                                    AgentStatus::Processing
                                } else {
                                    AgentStatus::Waiting
                                };
                            }
                        }
                    }
                }
            }
        }
        
        AgentStatus::Off
    }

    fn get_name(&self) -> &'static str {
        "Claude Code"
    }

    fn is_available(&self) -> bool {
        // Check if Claude Code CLI is installed by trying to run --version
        match Command::new("claude").arg("--version").output() {
            Ok(_) => true,
            Err(_) => {
                // Also try checking for npx claude-code
                match Command::new("npx").args(&["@anthropic-ai/claude-code", "--version"]).output() {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_code_monitor_creation() {
        let monitor = ClaudeCodeMonitor::new();
        assert_eq!(monitor.get_name(), "Claude Code");
    }

    #[test]
    fn test_claude_code_status_when_off() {
        let monitor = ClaudeCodeMonitor::new();
        // When no Claude Code process is running, should be off
        let status = monitor.get_status();
        // This test will pass when Claude Code is not running
        println!("Current status: {:?}", status);
    }
}