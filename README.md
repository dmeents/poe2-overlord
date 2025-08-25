# POE2 Overlord

A powerful, modern game overlay for Path of Exile 2 built with **Tauri 2** and **React 19**.

![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Rust](https://img.shields.io/badge/rust-1.77.2+-orange.svg)
![Node](https://img.shields.io/badge/node-18.0+-green.svg)
![Tauri](https://img.shields.io/badge/tauri-2.8.3-purple.svg)

## 🚀 Features

- **Native Performance**: Built with Tauri 2 for optimal performance and small binary size
- **Modern UI**: Beautiful, responsive interface built with React 19 and Tailwind CSS 4
- **Game Integration**: Automatic Path of Exile 2 process detection and monitoring
- **Always-on-Top Overlay**: Stays visible over your game with transparency support
- **Draggable Interface**: Move the overlay anywhere on your screen
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Dark Theme**: POE2-inspired color scheme optimized for gaming
- **Monorepo Architecture**: Clean separation between frontend and backend

## 🎯 Current Capabilities

- **Process Monitoring**: Automatically detects when Path of Exile 2 is running
- **Overlay Controls**: Show/hide, minimize, and move the overlay window
- **Real-time Status**: Live updates of game process status with refresh capability
- **Window Management**: Full window control including minimize, close, and positioning
- **Extensible Architecture**: Built for future feature additions

## 🛠️ Technology Stack

### Frontend

- **React 19** with TypeScript 5.8
- **Tailwind CSS 4** for styling
- **Lucide React** for icons
- **Vite 7** for development and building
- **ESLint 9** for code quality

### Backend

- **Rust 1.77.2** with Tauri 2.8.3 framework
- **sysinfo** for system process monitoring
- **tokio** for async operations
- **serde** for serialization

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
│   ├── frontend/           # React frontend application
│   │   ├── src/
│   │   │   ├── components/ # Reusable UI components
│   │   │   ├── hooks/      # Custom React hooks
│   │   │   ├── types/      # TypeScript type definitions
│   │   │   ├── utils/      # Utility functions
│   │   │   ├── App.tsx     # Main application component
│   │   │   └── main.tsx    # React entry point
│   │   ├── package.json    # Frontend dependencies
│   │   └── vite.config.ts  # Vite configuration
│   └── backend/            # Rust backend application
│       ├── src/
│       │   ├── commands/   # Tauri command handlers
│       │   ├── handlers/   # Application setup
│       │   ├── models/     # Data structures
│       │   ├── services/   # Business logic
│       │   ├── lib.rs      # Core application logic
│       │   └── main.rs     # Application entry point
│       ├── Cargo.toml      # Rust dependencies
│       └── tauri.conf.json # Tauri configuration
├── package.json             # Root workspace configuration
└── README.md               # This file
```

## 🎮 Usage

1. **Launch the overlay**

   - Run the application using `yarn tauri:dev` or the built executable
   - The overlay will appear as a small window on your screen

2. **Game Detection**

   - Start Path of Exile 2
   - The overlay will automatically detect the game process
   - Status indicator will change from red (offline) to green (online)

3. **Overlay Controls**
   - **Drag**: Click and drag the title bar to move the overlay
   - **Minimize/Expand**: Use the eye icon to collapse/expand content
   - **Minimize to Taskbar**: Use the minimize button
   - **Close**: Use the X button to exit the application

## 🔧 Configuration

The overlay behavior can be customized through the `packages/backend/tauri.conf.json` file:

- **Window size**: Adjust `width`, `height`, `minWidth`, `minHeight`
- **Position**: Set default `x`, `y` coordinates
- **Transparency**: Toggle `transparent` property
- **Always on top**: Control `alwaysOnTop` behavior

## 🚧 Development

### Available Scripts

- `yarn dev` - Start Vite dev server (frontend only)
- `yarn build` - Build the frontend
- `yarn tauri:dev` - Run Tauri in development mode
- `yarn tauri:build` - Build the complete application
- `yarn tauri:info` - Show Tauri environment info
- `yarn lint` - Run ESLint
- `yarn clean` - Clean build artifacts
- `yarn format:all` - Format both frontend and backend code
- `yarn build:all` - Build both frontend and backend

### Adding Features

1. **Frontend**: Add React components in `packages/frontend/src/`
2. **Backend**: Add Rust functions in `packages/backend/src/`
3. **API**: Use `#[tauri::command]` for Rust functions callable from frontend

### Development Workflow

1. **Frontend Development**: Use `yarn dev` for hot reloading
2. **Backend Development**: Use `yarn tauri:dev` for full-stack development
3. **Code Formatting**: Use `yarn format:all` to format both frontend and backend
4. **Building**: Use `yarn build:all` for production builds

## 🔒 Security

The application uses Tauri 2's enhanced security features:

- No `eval()` or similar dangerous functions
- Limited API access through capability system
- Process monitoring with minimal permissions
- CSP (Content Security Policy) support

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Test thoroughly with `yarn tauri:dev`
5. Format code with `yarn format:all`
6. Submit a pull request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🛣️ Roadmap

- [ ] Item search and price checking
- [ ] Build calculator integration
- [ ] Passive tree overlay
- [ ] Flask timer tracking
- [ ] DPS calculator
- [ ] Map tracking features
- [ ] Configuration UI
- [ ] Hotkey support
- [ ] Multi-monitor support
- [ ] Plugin system for extensibility

## ⚠️ Disclaimer

This is a third-party tool and is not affiliated with or endorsed by Grinding Gear Games. Use at your own risk and in accordance with Path of Exile's Terms of Service.

## 📧 Support

If you encounter issues or have questions:

1. Check the [Issues](https://github.com/yourusername/poe2-overlord/issues) page
2. Create a new issue with detailed information
3. Include your system information (`yarn tauri:info`)

---

**Happy gaming, Exile!** 🎮
