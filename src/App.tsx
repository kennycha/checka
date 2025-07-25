import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { AgentSummary } from "./types";
import { AgentStatusItem } from "./components/AgentStatusItem";

import "./App.css";

function App() {
  const [agentSummary, setAgentSummary] = useState<AgentSummary | null>(null);
  const [loading, setLoading] = useState(true);

  const fetchAgentSummary = async () => {
    try {
      const summary = await invoke<AgentSummary>("get_agent_summary");
      setAgentSummary(summary);
      setLoading(false);
    } catch (error) {
      console.error("Failed to fetch agent summary:", error);
      setLoading(false);
    }
  };

  useEffect(() => {
    invoke("init");
    fetchAgentSummary();

    // Poll for updates every 3 seconds
    const interval = setInterval(fetchAgentSummary, 3000);
    return () => clearInterval(interval);
  }, []);

  if (loading) {
    return (
      <div className="container">
        <div className="loading">
          <div className="spinner"></div>
          <p>Loading agents...</p>
        </div>
      </div>
    );
  }

  if (!agentSummary) {
    return (
      <div className="container">
        <p className="error">Failed to load agent data</p>
      </div>
    );
  }

  return (
    <div className="container">
      <div className="header">
        <h2>AI Agents</h2>
        <div className="summary">
          <span className="active-count">
            {agentSummary.active_count}/{agentSummary.total_agents} active
          </span>
        </div>
      </div>

      <div className="agent-list">
        {agentSummary.agents.map((agent) => (
          <AgentStatusItem key={agent.name} agent={agent} />
        ))}
      </div>

      <div className="footer">
        <span className="last-updated">
          Last updated: {agentSummary.last_updated}
        </span>
      </div>
    </div>
  );
}

export default App;
