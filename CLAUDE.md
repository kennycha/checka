# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Checka is a macOS menubar application built with Tauri v2 that monitors the status of AI coding agents running in the background. The app allows developers to work on other tasks while keeping track of AI agent activity (Claude Code, Cursor Agent, GitHub Copilot CLI, and Gemini CLI) through a simple menubar interface.

### Project Goals

- Monitor 4 AI agents: Claude Code, Cursor Agent, GitHub Copilot CLI, Gemini CLI
- Show processing count in menubar with loading indicator
- Provide detailed agent status in dropdown panel
- Enable asynchronous workflow where developers can do other work while agents process tasks

## Tech Stack

- **Frontend**: React 19 with TypeScript and Vite
- **Backend**: Rust with Tauri v2
- **Package Manager**: pnpm
- **Build System**: Vite + Cargo

## Development Commands

### Frontend Development

- `pnpm dev` - Start Vite dev server (port 1420)
- `pnpm build` - Build frontend (TypeScript compilation + Vite build)
- `pnpm preview` - Preview production build

### Tauri Development

- `pnpm tauri dev` - Run Tauri in development mode (starts both frontend and backend)
- `pnpm tauri build` - Build production application
- `pnpm tauri` - Access Tauri CLI commands

### Rust Development

- `bacon` or `bacon clippy` - Run clippy linter in watch mode (configured in bacon.toml)
- `bacon check` - Run cargo check in watch mode
- `bacon test` - Run tests in watch mode
- `cargo clippy` - Run Rust linter
- `cargo check` - Type check Rust code
- `cargo test` - Run Rust tests

## Architecture

### AI Agent Monitoring System

The app uses a plugin-based architecture to monitor different AI agents:

```rust
trait AgentMonitor {
    fn get_status(&self) -> AgentStatus;
    fn get_name(&self) -> &'static str;
    fn is_available(&self) -> bool;
}

enum AgentStatus {
    Off,        // Agent not running
    Processing,  // Active work (high CPU usage)
    Waiting,     // Running but waiting for input (low CPU usage)
    Error(String),
}
```

### Supported Agents

1. **Claude Code**: Process monitoring of Node.js-based `@anthropic-ai/claude-code`
2. **Cursor Agent**: Cursor IDE AI agent activity detection
3. **GitHub Copilot CLI**: `gh copilot` command monitoring
4. **Gemini CLI**: Google Gemini CLI tool monitoring

### Monitoring Strategy

- **Process-based detection**: Find agent processes by name/command pattern
- **CPU usage analysis**: Distinguish between processing (>1% CPU) and blocked states
- **Availability check**: Auto-detect installed agents
- **Periodic updates**: 2-3 second intervals

### UI/UX Design

**Menubar Display:**

- Off state: Gray dot ⚪
- Active agents: Spinner + count 🔄2

**Dropdown Panel:**

```
┌─ AI Agents (2/4 active) ────────┐
│ 🟢 Claude Code    Processing    │
│ 🟡 Cursor Agent   Waiting       │
│ ⚪ GitHub Copilot Off          │
│ ⚪ Gemini CLI     Off          │
│ ────────────────────────────── │
│ Last updated: 2s ago           │
│ ⚙️ Settings                    │
└────────────────────────────────┘
```

### Frontend (React/TypeScript)

- **src/App.tsx**: Agent list display and status visualization
- **src/main.tsx**: React app entry point
- **src/components/**: Agent status components and UI elements

### Backend (Rust/Tauri)

- **src/main.rs**: Application entry point with agent monitoring setup
- **src/agents/**: Individual agent monitor implementations
- **src/agent_manager.rs**: Central agent coordination and status aggregation
- **src/command.rs**: Tauri commands for agent status retrieval
- **src/tray.rs**: System tray with dynamic status display
- **src/fns.rs**: Menubar panel positioning and macOS integration

### Key Dependencies

- **tauri-nspanel**: Custom macOS panel functionality (from ahkohd/tauri-nspanel)
- **monitor**: Monitor detection utilities (from ahkohd/tauri-toolkit)
- **sysinfo**: Process monitoring and system information
- **notify**: File system watching capabilities

## Configuration Files

- **tauri.conf.json**: Tauri application configuration
- **vite.config.ts**: Vite build configuration (port 1420, ignores src-tauri)
- **bacon.toml**: Rust development tools configuration
- **Cargo.toml**: Rust dependencies and features

## Implementation Approach

### Phase 1: Core Agent Monitoring

1. Implement `AgentMonitor` trait and `AgentManager`
2. Create individual monitors for each of the 4 agents
3. Set up process detection and CPU monitoring
4. Basic status aggregation and updates

### Phase 2: UI Integration

1. Update menubar icon to show processing count
2. Implement dropdown with agent status list
3. Add loading indicators and status colors
4. Real-time status updates every 2-3 seconds

### Phase 3: Configuration & Polish

1. Settings panel for agent enable/disable
2. Configurable check intervals
3. Error handling and recovery
4. Performance optimization

## Development Notes

- Based on existing Tauri menubar boilerplate structure
- Uses macOS private APIs for proper menubar panel behavior
- Process monitoring requires appropriate system permissions
- Agent detection patterns may need refinement based on actual tool behavior
- Consider trademark/branding restrictions when using agent logos or names
