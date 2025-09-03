# POE2 Overlord

A comprehensive game monitoring and activity tracking overlay for Path of Exile 2, built with **Tauri 2** and **React 19**. Features real-time log monitoring, time tracking, and server status monitoring with a modern, modular architecture.

![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Rust](https://img.shields.io/badge/rust-1.77.2+-orange.svg)
![Node](https://img.shields.io/badge/node-18.0+-green.svg)
![Tauri](https://img.shields.io/badge/tauri-2.8.3-purple.svg)

## 🚀 Features

### Core Functionality
- **Real-time Log Monitoring**: Watches POE2 client log files for scene changes and server connections
- **Time Tracking**: Tracks time spent in different game locations (zones, acts, hideouts)
- **Game Process Monitoring**: Automatically detects and monitors Path of Exile 2 game processes
- **Server Status Monitoring**: Real-time server ping monitoring and connection status
- **Activity Logging**: Comprehensive activity log with scene change events and timestamps

### Technical Features
- **Native Performance**: Built with Tauri 2 for optimal performance and small binary size
- **Modern UI**: Beautiful, responsive interface built with React 19 and Tailwind CSS 4
- **Type-safe Routing**: TanStack Router for type-safe navigation with code splitting
- **Event-driven Architecture**: Real-time updates using Tokio broadcast channels
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Modular Design**: Clean separation between frontend and backend with clear interfaces

## 🎯 Current Capabilities

### Game Monitoring
- **Process Detection**: Automatically detects when Path of Exile 2 is running
- **Log File Watching**: Real-time monitoring of POE2 client log files
- **Scene Change Detection**: Detects zone, act, and hideout transitions
- **Server Connection Tracking**: Monitors server connections and ping status

### Time Tracking
- **Session Management**: Tracks active and completed time tracking sessions
- **Location Statistics**: Aggregates data for zones, acts, and hideouts
- **Real-time Updates**: Live updates of time tracking data
- **Persistent Storage**: Saves tracking data to JSON files

### User Interface
- **Dashboard**: Overview of game status and recent activity
- **Activity Monitor**: Real-time log display with filtering
- **Time Tracking Dashboard**: Session management and statistics
- **Settings**: Configuration management for log paths and preferences
- **Status Bar**: Real-time status indicators and monitoring information

## 🛠️ Technology Stack

### Frontend
- **React 19** with TypeScript 5.8 - Latest React with modern features and improved performance
- **TanStack Router** - Type-safe routing with code splitting and performance optimizations
- **Tailwind CSS 4** - Latest version with improved performance and utility-first CSS framework
- **Heroicons** - Modern icon library with consistent design
- **Vite 7** - Fast development server and build tool
- **ESLint 9** - Code quality and consistency

### Backend
- **Rust 1.77.2** with Tauri 2.8.3 framework - Cross-platform desktop app framework
- **Tokio** - Async runtime for background tasks, file monitoring, and event broadcasting
- **sysinfo** - System information and process monitoring with async support
- **notify** - Cross-platform file system event monitoring
- **serde** - Serialization/deserialization with JSON support
- **chrono** - Date and time handling with timezone support
- **thiserror** - Custom error type definitions

## 📋 Prerequisites

- **Rust**: 1.77.2 or higher
- **Node.js**: 18.0 or higher
- **Yarn**: Latest version (workspaces support required)

### System Dependencies (Linux)

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# Arch Linux
sudo pacman -S webkit2gtk base-devel curl wget openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg

# Fedora
sudo dnf install webkit2gtk3-devel.x86_64 openssl-devel curl wget libappindicator-gtk3-devel librsvg2-devel
```

## 🚀 Quick Start

1. **Clone the repository**

   ```bash
   git clone https://github.com/yourusername/poe2-overlord.git
   cd poe2-overlord
   ```

2. **Install dependencies**

   ```bash
   yarn install
   ```

3. **Run in development mode**

   ```bash
   yarn tauri:dev
   ```

4. **Build for production**
   ```bash
   yarn build:all
   ```

## 📁 Project Structure

```
poe2-overlord/
├── packages/
│   ├── frontend/                    # React frontend application
│   │   ├── src/
│   │   │   ├── routes/              # TanStack Router-based routing system
│   │   │   │   ├── __root.tsx       # Root layout with navigation
│   │   │   │   ├── index.tsx        # Dashboard/home page
│   │   │   │   ├── activity.tsx     # Activity monitoring and log display
│   │   │   │   ├── time-tracking.tsx # Time tracking sessions and statistics
│   │   │   │   └── settings.tsx     # Application configuration
│   │   │   ├── components/          # Reusable UI components
│   │   │   │   ├── button.tsx       # Button component with variants
│   │   │   │   ├── form/            # Form components with validation
│   │   │   │   ├── log-monitor/     # Log monitoring components
│   │   │   │   ├── status-bar/      # Status and monitoring components
│   │   │   │   ├── time-tracking/   # Time tracking components
│   │   │   │   └── index.ts         # Component exports
│   │   │   ├── hooks/               # Custom React hooks
│   │   │   │   ├── useGameProcess.ts # Game process management
│   │   │   │   ├── useTimeTracking.ts # Time tracking state management
│   │   │   │   └── useZoneMonitoring.ts # Zone monitoring and events
│   │   │   ├── types/               # TypeScript type definitions
│   │   │   ├── utils/               # Utility functions and Tauri integration
│   │   │   └── main.tsx             # Application entry point with router
│   │   ├── package.json             # Frontend dependencies
│   │   └── vite.config.ts           # Vite configuration with TanStack Router
│   └── backend/                     # Rust backend application
│       ├── src/
│       │   ├── commands/            # Tauri command handlers
│       │   │   ├── config_commands.rs # Configuration management
│       │   │   ├── log_commands.rs  # Log monitoring and file operations
│       │   │   └── time_tracking_commands.rs # Time tracking session management
│       │   ├── handlers/            # Application setup and event handling
│       │   │   ├── service_initializer.rs # Service initialization
│       │   │   ├── log_event_handler.rs # Log monitoring events
│       │   │   └── time_tracking_handler.rs # Time tracking events
│       │   ├── models/              # Data structures and types
│       │   │   ├── config.rs        # Application configuration
│       │   │   ├── events.rs        # Event types and structures
│       │   │   ├── process.rs       # Process information
│       │   │   └── time_tracking.rs # Time tracking data structures
│       │   ├── services/            # Business logic and core functionality
│       │   │   ├── configuration_manager.rs # Configuration management
│       │   │   ├── log_analyzer.rs  # Log file analysis and monitoring
│       │   │   ├── session_tracker.rs # Time tracking session management
│       │   │   ├── server_monitor.rs # Server status monitoring
│       │   │   └── event_dispatcher.rs # Event broadcasting system
│       │   ├── parsers/             # Specialized parsers for different data formats
│       │   │   ├── core/            # Core parsing infrastructure
│       │   │   ├── parsers/         # Scene change and server parsers
│       │   │   └── detection/       # Scene type detection
│       │   ├── errors.rs            # Comprehensive error handling system
│       │   ├── lib.rs               # Core application logic and command registration
│       │   └── main.rs              # Application entry point
│       ├── tests/                   # Comprehensive test suite
│       ├── Cargo.toml               # Rust dependencies
│       └── tauri.conf.json          # Tauri configuration
├── package.json                     # Root workspace configuration
└── README.md                        # This file
```

## 🎮 Usage

### Getting Started

1. **Launch the Application**
   - Run `yarn tauri:dev` for development or use the built executable
   - The application will start with the main dashboard

2. **Configure Log Path**
   - Go to Settings to configure the Path of Exile 2 client log path
   - The application will attempt to auto-detect the default path
   - Ensure the log file is accessible and readable

3. **Start Monitoring**
   - Launch Path of Exile 2
   - The application will automatically detect the game process
   - Log monitoring will begin automatically when the game starts

### Features Overview

#### Dashboard
- **Game Status**: Real-time Path of Exile 2 process monitoring
- **Quick Actions**: Access to Activity Monitor, Time Tracking, and Settings
- **Recent Activity**: Overview of recent game events and status

#### Activity Monitor
- **Real-time Log Display**: Live view of POE2 client log events
- **Scene Change Detection**: Automatic detection of zone, act, and hideout transitions
- **Server Connection Tracking**: Monitor server connections and ping status
- **Event Filtering**: Filter events by type and time range

#### Time Tracking Dashboard
- **Active Sessions**: View currently active time tracking sessions
- **Session History**: Historical data of completed sessions
- **Location Statistics**: Aggregated statistics for different game locations
- **Session Management**: Start, end, and manage time tracking sessions

#### Settings
- **Configuration Management**: Set log file paths and preferences
- **Log Level Control**: Adjust logging verbosity
- **Path Validation**: Verify and test configured paths

## 🔧 Configuration

### Application Configuration

The application stores configuration in the user's config directory:
- **Linux/macOS**: `~/.config/poe2-overlord/config.json`
- **Windows**: `%APPDATA%\poe2-overlord\config.json`

#### Available Settings
- **Log File Path**: Path to the Path of Exile 2 client log file
- **Log Level**: Logging verbosity (trace, debug, info, warn, error)
- **Auto-detection**: Automatic detection of default log file paths

### Window Configuration

Window behavior can be customized through `packages/backend/tauri.conf.json`:
- **Window size**: Adjust `width`, `height`, `minWidth`, `minHeight`
- **Position**: Set default `x`, `y` coordinates
- **Transparency**: Toggle `transparent` property
- **Always on top**: Control `alwaysOnTop` behavior
- **Resizable**: Enable/disable window resizing

## 🚧 Development

### Available Scripts

#### Root Level Scripts
- `yarn dev` - Start Tauri in development mode (full-stack)
- `yarn build` - Build the complete Tauri application
- `yarn format` - Format both frontend and backend code
- `yarn lint` - Run linting for both frontend and backend
- `yarn test` - Run tests for both frontend and backend
- `yarn install:all` - Install all dependencies (Node.js and Rust)

#### Frontend Scripts
- `yarn workspace @poe2-overlord/frontend dev` - Start Vite dev server (frontend only)
- `yarn workspace @poe2-overlord/frontend build` - Build the frontend
- `yarn workspace @poe2-overlord/frontend lint` - Run ESLint
- `yarn workspace @poe2-overlord/frontend format` - Format frontend code

#### Backend Scripts
- `yarn workspace @poe2-overlord/backend build` - Build the Rust backend
- `yarn workspace @poe2-overlord/backend test` - Run Rust tests
- `yarn workspace @poe2-overlord/backend lint` - Run Clippy linting
- `yarn workspace @poe2-overlord/backend format` - Format Rust code

### Development Workflow

1. **Full-Stack Development**: Use `yarn dev` for complete development with hot reloading
2. **Frontend Only**: Use `yarn workspace @poe2-overlord/frontend dev` for frontend development
3. **Backend Testing**: Use `yarn workspace @poe2-overlord/backend test` for Rust unit tests
4. **Code Quality**: Use `yarn lint` and `yarn format` for code quality and consistency

### Adding Features

#### Frontend Development
1. **Components**: Add React components in `packages/frontend/src/components/`
2. **Routes**: Add new routes in `packages/frontend/src/routes/`
3. **Hooks**: Add custom hooks in `packages/frontend/src/hooks/`
4. **Types**: Add TypeScript types in `packages/frontend/src/types/`

#### Backend Development
1. **Commands**: Add Tauri commands in `packages/backend/src/commands/`
2. **Services**: Add business logic in `packages/backend/src/services/`
3. **Models**: Add data structures in `packages/backend/src/models/`
4. **Parsers**: Add parsing logic in `packages/backend/src/parsers/`

#### API Integration
1. **Commands**: Use `#[tauri::command]` for Rust functions callable from frontend
2. **Events**: Use the event system for real-time updates
3. **Error Handling**: Use `AppResult<T>` for consistent error handling

## 🔒 Security

The application uses Tauri 2's enhanced security features:

- **Capability System**: Limited API access through Tauri 2's capability system
- **Sandboxed Execution**: Application runs in a secure sandbox environment
- **No Dangerous Functions**: No `eval()` or similar dangerous functions
- **Minimal Permissions**: Game process monitoring with minimal required permissions
- **CSP Support**: Content Security Policy configuration available
- **Path Validation**: All file paths are validated before access
- **Secure File Access**: Controlled access to log files and configuration directories

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes following the established patterns:
   - Use TypeScript for frontend code with proper type definitions
   - Use Rust for backend code with proper error handling
   - Follow the modular architecture patterns
   - Add tests for new functionality
4. Test thoroughly:
   - Run `yarn test` for all tests
   - Test with `yarn dev` for full-stack development
   - Test on multiple platforms when possible
5. Ensure code quality:
   - Run `yarn lint` to check for issues
   - Run `yarn format` to format code
6. Submit a pull request with a clear description of changes

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🛣️ Roadmap

### Core Features (In Progress)
- [x] Real-time log monitoring and scene change detection
- [x] Time tracking for zones, acts, and hideouts
- [x] Game process monitoring
- [x] Server status monitoring
- [x] Modern UI with routing system
- [x] Configuration management

### Planned Features
- [ ] Item search and price checking integration
- [ ] Build calculator integration
- [ ] Passive tree overlay
- [ ] Flask timer tracking
- [ ] DPS calculator
- [ ] Map tracking and statistics
- [ ] Advanced filtering and search in activity logs
- [ ] Export functionality for time tracking data
- [ ] Hotkey support for overlay controls
- [ ] Multi-monitor support
- [ ] Plugin system for extensibility
- [ ] Performance metrics and optimization
- [ ] Advanced server monitoring features

## ⚠️ Disclaimer

This is a third-party tool and is not affiliated with or endorsed by Grinding Gear Games. Use at your own risk and in accordance with Path of Exile's Terms of Service.

## 📧 Support

If you encounter issues or have questions:

1. Check the [Issues](https://github.com/yourusername/poe2-overlord/issues) page for existing solutions
2. Create a new issue with detailed information including:
   - Your operating system and version
   - Application version
   - Steps to reproduce the issue
   - Relevant log files or error messages
   - System information (`yarn tauri:info`)
3. For development questions, check the documentation in the respective package README files

## 📚 Documentation

- [Backend Documentation](packages/backend/README.md) - Detailed backend architecture and API documentation
- [Frontend Documentation](packages/frontend/README.md) - Frontend components and routing documentation
- [Test Documentation](packages/backend/tests/README.md) - Testing strategies and test documentation

---

**Happy gaming, Exile!** 🎮
