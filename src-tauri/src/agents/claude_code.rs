use super::{AgentMonitor, AgentStatus};
use sysinfo::{System, ProcessesToUpdate};
use std::process::Command;

pub struct ClaudeCodeMonitor {}

impl ClaudeCodeMonitor {
    pub fn new() -> Self {
        Self {}
    }
}

impl AgentMonitor for ClaudeCodeMonitor {
    fn get_status(&self) -> AgentStatus {
        let mut system = System::new_all();
        system.refresh_processes(ProcessesToUpdate::All, true);
        
        let claude_process = system.processes().values().find(|process| {
            let process_name = process.name().to_str().unwrap_or("");
            let cmd = process.cmd();
            
            if !cmd.is_empty() {
                let cmd_string = cmd.iter()
                    .filter_map(|s| s.to_str())
                    .collect::<Vec<&str>>()
                    .join(" ");
                
                if process_name == "node" {
                    (cmd_string.starts_with("claude ") && !cmd_string.contains("--version")) ||
                    (cmd_string.contains("/bin/claude") && !cmd_string.contains("--version")) ||
                    cmd_string.contains("@anthropic-ai/claude-code")
                } else {
                    process_name == "claude"
                }
            } else {
                process_name == "claude"
            }
        });
        
        if let Some(process) = claude_process {
            let pid = process.pid();
            
            // 시스템 명령어로 실제 CPU 사용률 확인
            let actual_cpu = if let Ok(output) = Command::new("ps")
                .args(&["-p", &pid.to_string(), "-o", "pcpu="])
                .output()
            {
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .parse::<f64>()
                    .unwrap_or(0.0)
            } else {
                0.0
            };
            
            if actual_cpu > 1.0 {
                AgentStatus::Processing
            } else {
                AgentStatus::Waiting
            }
        } else {
            AgentStatus::Off
        }
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