# Checka

> Do you ever miss when your AI agent finishes a task while you're working on something else?

Checka is a macOS menubar app that monitors the status of AI coding agents in real-time. Keep track of Claude Code, Cursor Agent, Gemini CLI, and GitHub Copilot CLI at a glance!

## Key Features

- **Real-time Monitoring**: Check if AI agents are processing, waiting, or idle in real-time
- **At-a-glance Status**: Instantly see agent activity status through the menubar icon

## Supported AI Agents

- Claude Code
- (WIP) Cursor Agent
- (WIP) Gemini CLI
- (WIP) GitHub Copilot CLI

## System Requirements

- macOS 10.15 (Catalina) or later
- Intel Mac or Apple Silicon Mac

## Installation

Run Checka locally:

```bash
# Clone the repository
git clone https://github.com/kennycha/checka.git
cd checka

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev
```

The app will launch automatically and work with full AI agent detection capabilities.

> **Note**: Due to macOS security restrictions, only development mode provides full functionality for AI agent monitoring.

## Usage

1. **Menubar Robot Icon**: The robot's eye color indicates status
   - Gray eyes: All agents are off
   - Green eyes: Agents are processing
   - Yellow eyes: Agents are waiting only
2. **Open Panel**: Click the menubar robot icon to open the detailed status panel
3. **Check Status**: View each agent's status (processing, waiting, or off)
4. **Quit App**: Use the "Quit" button at the bottom of the panel to exit the app

## Development Story

As programming increasingly involves delegating code writing to AI agents, a pattern emerged of requesting tasks and then handling other work like planning or team collaboration.

However, I often missed when AI agents completed their tasks, creating "dead time" where I wasn't aware work was finished.

I built this app to eliminate missed opportunities and maximize efficiency in asynchronous collaboration between myself and AI agents.

## Development

### Prerequisites

- [Node.js](https://nodejs.org/) (LTS version)
- [pnpm](https://pnpm.io/) package manager
- [Rust](https://rustup.rs/) toolchain
- Xcode Command Line Tools (macOS)

### Development Commands

```bash
# Start development server
pnpm tauri dev

# Build for production
pnpm tauri build

# Frontend development only
pnpm dev
```

## Tech Stack

- **Frontend**: React 19 + TypeScript + Vite
- **Backend**: Rust + Tauri v2
- **Package Manager**: pnpm
- **Platforms**: macOS (Intel + Apple Silicon)

## Credits

This app is based on [ahkohd](https://github.com/ahkohd)'s [tauri-macos-menubar-app-example](https://github.com/ahkohd/tauri-macos-menubar-app-example).

## Contributing

Please report bugs or suggest features in [Issues](https://github.com/kennycha/checka/issues)!

## License

MIT License - Feel free to use!

---

_Made with ❤️ for developers who love AI coding tools_
