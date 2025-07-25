use crate::agents::{AgentMonitor, AgentInfo, AgentStatus, claude_code::ClaudeCodeMonitor};

pub struct AgentManager {
    monitors: Vec<Box<dyn AgentMonitor>>,
}

impl AgentManager {
    pub fn new() -> Self {
        let mut monitors: Vec<Box<dyn AgentMonitor>> = vec![];
        
        // Add Claude Code monitor (only enabled agent for Phase 1)
        let claude_monitor = ClaudeCodeMonitor::new();
        if claude_monitor.is_available() {
            monitors.push(Box::new(claude_monitor));
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

    pub fn get_active_count(&self) -> usize {
        self.monitors
            .iter()
            .filter(|monitor| monitor.get_status().is_active())
            .count()
    }

    pub fn get_processing_count(&self) -> usize {
        self.monitors
            .iter()
            .filter(|monitor| matches!(monitor.get_status(), AgentStatus::Processing))
            .count()
    }

    pub fn get_waiting_count(&self) -> usize {
        self.monitors
            .iter()
            .filter(|monitor| matches!(monitor.get_status(), AgentStatus::Waiting))
            .count()
    }

    pub fn has_active_agents(&self) -> bool {
        self.get_active_count() > 0
    }

    pub fn get_summary(&self) -> AgentSummary {
        let agent_info = self.get_all_agent_info();
        let processing_count = self.get_processing_count();
        let waiting_count = self.get_waiting_count();
        let total_agents = self.monitors.len();

        AgentSummary {
            total_agents,
            processing_count,
            waiting_count,
            active_count: processing_count + waiting_count,
            agents: agent_info,
            last_updated: chrono::Local::now().format("%H:%M:%S").to_string(),
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
}

impl Default for AgentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_manager_creation() {
        let manager = AgentManager::new();
        
        // Should have at least Claude Code monitor if available
        let summary = manager.get_summary();
        println!("Available agents: {}", summary.total_agents);
        
        // Test that summary structure is correct
        assert_eq!(summary.active_count, summary.processing_count + summary.waiting_count);
    }

    #[test]
    fn test_agent_info_retrieval() {
        let manager = AgentManager::new();
        let agent_info = manager.get_all_agent_info();
        
        // Should have agent info
        for info in agent_info {
            println!("Agent: {}, Status: {:?}, Available: {}", 
                    info.name, info.status, info.available);
        }
    }
}