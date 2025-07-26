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

    // Use requestAnimationFrame for smoother updates
    let lastTime = 0;
    let animationId: number;

    const animate = (currentTime: number) => {
      if (currentTime - lastTime >= 1000) {
        // 1 second
        fetchAgentSummary();
        lastTime = currentTime;
      }
      animationId = requestAnimationFrame(animate);
    };

    animationId = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(animationId);
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
        <h2>Checka</h2>
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
        <button className="quit-button" onClick={() => invoke("quit_app")}>
          Quit
        </button>
      </div>
    </div>
  );
}

export default App;
