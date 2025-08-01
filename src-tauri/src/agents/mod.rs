use serde::{Deserialize, Serialize};

pub mod claude_code;
pub mod gemini;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AgentStatus {
    Off,
    Processing,
    Waiting,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub name: String,
    pub status: AgentStatus,
    pub available: bool,
    pub last_updated: Option<String>,
}

pub trait AgentMonitor: Send + Sync {
    fn get_status(&self) -> AgentStatus;
    fn get_name(&self) -> &'static str;
    fn is_available(&self) -> bool;
    
    fn get_info(&self) -> AgentInfo {
        AgentInfo {
            name: self.get_name().to_string(),
            status: self.get_status(),
            available: self.is_available(),
            last_updated: Some(chrono::Local::now().format("%H:%M:%S").to_string()),
        }
    }
}