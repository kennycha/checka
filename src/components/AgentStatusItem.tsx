import { AgentInfo, getStatusString, getStatusColor, getStatusIcon } from '../types';
import './AgentStatusItem.css';

interface AgentStatusItemProps {
  agent: AgentInfo;
}

export function AgentStatusItem({ agent }: AgentStatusItemProps) {
  const statusString = getStatusString(agent.status);
  const statusColor = getStatusColor(agent.status);
  const statusIcon = getStatusIcon(agent.status);

  return (
    <div className="agent-status-item">
      <div className="agent-status-left">
        <span className="status-icon">{statusIcon}</span>
        <span className="agent-name">{agent.name}</span>
      </div>
      <div className="agent-status-right">
        <span 
          className="status-text"
          style={{ color: statusColor }}
        >
          {statusString}
        </span>
        {!agent.available && (
          <span className="unavailable-indicator" title="Not installed">
            ⚠️
          </span>
        )}
      </div>
    </div>
  );
}