use crate::agents::{AgentMonitor, AgentInfo, AgentStatus, claude_code::ClaudeCodeMonitor, gemini::GeminiMonitor};

pub struct AgentManager {
    monitors: Vec<Box<dyn AgentMonitor>>,
}

impl AgentManager {
    pub fn new() -> Self {
        let mut monitors: Vec<Box<dyn AgentMonitor>> = vec![];
        
        // Add Claude Code monitor
        let claude_monitor = ClaudeCodeMonitor::new();
        if claude_monitor.is_available() {
            monitors.push(Box::new(claude_monitor));
        }
        
        // Add Gemini monitor
        let gemini_monitor = GeminiMonitor::new();
        if gemini_monitor.is_available() {
            monitors.push(Box::new(gemini_monitor));
        }
        
        Self {
            monitors,
        }
    }


    pub fn get_all_agent_info(&self) -> Vec<AgentInfo> {
        self.monitors
            .iter()
            .map(|monitor| monitor.get_info())
            .collect()
    }

    pub fn get_summary(&self) -> AgentSummary {
        // 한 번만 모든 에이전트 정보를 가져와서 재사용
        let agent_info = self.get_all_agent_info();
        
        // agent_info에서 카운트 정보를 계산 (get_status() 재호출 없음)
        let processing_count = agent_info.iter()
            .filter(|info| matches!(info.status, AgentStatus::Processing))
            .count();
        let waiting_count = agent_info.iter()
            .filter(|info| matches!(info.status, AgentStatus::Waiting))
            .count();
        
        let total_agents = self.monitors.len();
        
        let current_directory = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "Unknown".to_string());

        AgentSummary {
            total_agents,
            processing_count,
            waiting_count,
            active_count: processing_count + waiting_count,
            agents: agent_info,
            last_updated: chrono::Local::now().format("%H:%M:%S").to_string(),
            current_directory,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgentSummary {
    pub total_agents: usize,
    pub processing_count: usize,
    pub waiting_count: usize,
    pub active_count: usize,
    pub agents: Vec<AgentInfo>,
    pub last_updated: String,
    pub current_directory: String,
}

impl Default for AgentManager {
    fn default() -> Self {
        Self::new()
    }
}
