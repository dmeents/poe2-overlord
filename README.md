# POE2 Master Overlay

A powerful, modern game overlay for Path of Exile 2 built with **Tauri** and **React**.

![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Rust](https://img.shields.io/badge/rust-1.89.0+-orange.svg)
![Node](https://img.shields.io/badge/node-24.2.0+-green.svg)

## 🚀 Features

- **Native Performance**: Built with Tauri for optimal performance and small binary size
- **Modern UI**: Beautiful, responsive interface built with React and Tailwind CSS
- **Game Integration**: Automatic Path of Exile 2 process detection and monitoring
- **Always-on-Top Overlay**: Stays visible over your game with transparency support
- **Draggable Interface**: Move the overlay anywhere on your screen
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Dark Theme**: POE2-inspired color scheme optimized for gaming

## 🎯 Current Capabilities

- **Process Monitoring**: Automatically detects when Path of Exile 2 is running
- **Overlay Controls**: Show/hide, minimize, and move the overlay window
- **Real-time Status**: Live updates of game process status
- **Extensible Architecture**: Built for future feature additions

## 🛠️ Technology Stack

### Frontend
- **React 19** with TypeScript
- **Tailwind CSS** for styling
- **Lucide React** for icons
- **Vite** for development and building

### Backend
- **Rust** with Tauri framework
- **sysinfo** for system process monitoring
- **tokio** for async operations

## 📋 Prerequisites

- **Rust**: 1.89.0 or higher
- **Node.js**: 24.2.0 or higher  
- **npm**: Latest version

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
   git clone https://github.com/yourusername/poe2-master.git
   cd poe2-master
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Run in development mode**
   ```bash
   npm run tauri:dev
   ```

4. **Build for production**
   ```bash
   npm run tauri:build
   ```

## 📁 Project Structure

```
poe2-master/
├── src/                    # React frontend source
│   ├── components/         # React components (planned)
│   ├── hooks/             # Custom React hooks (planned)
│   ├── utils/             # Utility functions (planned)
│   ├── App.tsx            # Main application component
│   ├── main.tsx           # React entry point
│   └── index.css          # Global styles
├── src-tauri/             # Rust backend source
│   ├── src/
│   │   ├── main.rs        # Application entry point
│   │   └── lib.rs         # Core application logic
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
├── public/                # Static assets
├── dist/                  # Built frontend (generated)
└── README.md              # This file
```

## 🎮 Usage

1. **Launch the overlay**
   - Run the application using `npm run tauri:dev` or the built executable
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

The overlay behavior can be customized through the `src-tauri/tauri.conf.json` file:

- **Window size**: Adjust `width`, `height`, `minWidth`, `minHeight`
- **Position**: Set default `x`, `y` coordinates
- **Transparency**: Toggle `transparent` property
- **Always on top**: Control `alwaysOnTop` behavior

## 🚧 Development

### Available Scripts

- `npm run dev` - Start Vite dev server
- `npm run build` - Build the frontend
- `npm run tauri:dev` - Run Tauri in development mode
- `npm run tauri:build` - Build the complete application
- `npm run tauri:info` - Show Tauri environment info
- `npm run lint` - Run ESLint
- `npm run clean` - Clean build artifacts

### Adding Features

1. **Frontend**: Add React components in `src/`
2. **Backend**: Add Rust functions in `src-tauri/src/`
3. **API**: Use `#[tauri::command]` for Rust functions callable from frontend

## 🔒 Security

The application uses Tauri's security features:
- No `eval()` or similar dangerous functions
- Limited API access through capability system
- Process monitoring with minimal permissions

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes
4. Test thoroughly
5. Submit a pull request

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

## ⚠️ Disclaimer

This is a third-party tool and is not affiliated with or endorsed by Grinding Gear Games. Use at your own risk and in accordance with Path of Exile's Terms of Service.

## 📧 Support

If you encounter issues or have questions:
1. Check the [Issues](https://github.com/yourusername/poe2-master/issues) page
2. Create a new issue with detailed information
3. Include your system information (`npm run tauri:info`)

---

**Happy gaming, Exile!** 🎮
