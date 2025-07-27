export type AgentStatus = 'Off' | 'Processing' | 'Waiting' | { Error: string };

export interface AgentInfo {
  name: string;
  status: AgentStatus;
  available: boolean;
  last_updated?: string;
}

export interface AgentSummary {
  total_agents: number;
  processing_count: number;
  waiting_count: number;
  active_count: number;
  agents: AgentInfo[];
  last_updated: string;
  current_directory: string;
}

export function getStatusString(status: AgentStatus): string {
  if (typeof status === 'string') {
    return status;
  }
  return `Error: ${status.Error}`;
}

export function getStatusColor(status: AgentStatus): string {
  if (typeof status === 'string') {
    switch (status) {
      case 'Off': return '#6b7280'; // gray
      case 'Processing': return '#10b981'; // green
      case 'Waiting': return '#f59e0b'; // yellow
      default: return '#6b7280';
    }
  }
  return '#ef4444'; // red for error
}

export function getStatusIcon(status: AgentStatus): string {
  if (typeof status === 'string') {
    switch (status) {
      case 'Off': return '⚪';
      case 'Processing': return '🟢';
      case 'Waiting': return '🟡';
      default: return '⚪';
    }
  }
  return '🔴'; // red for error
}