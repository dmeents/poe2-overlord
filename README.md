# POE2 Master Overlay

A powerful game overlay for Path of Exile 2 on Linux, built with **GTK4** for modern Wayland support and optimal performance.

## 🚀 Features

- **Search Functionality** - Look up item prices and information
- **Configurable Settings** - Customize appearance and behavior
- **Modern GTK4 Interface** - Native Linux desktop integration
- **Wayland Support** - Full compatibility with modern display servers
- **Game Process Detection** - Automatically detects when POE2 is running
- **Global Hotkeys** - Toggle overlay with keyboard shortcuts
- **Draggable Interface** - Move overlay anywhere on screen

## 🛠️ Requirements

- **Linux** (Ubuntu 22.04+, Fedora 36+, Arch Linux, etc.)
- **Python 3.8+**
- **GTK4** with Python bindings
- **Wayland** or **X11** display server

## 📦 Installation

### Quick Install

```bash
# Clone the repository
git clone https://github.com/yourusername/poe2-master.git
cd poe2-master

# Run the installation script
chmod +x install.sh
./install.sh
```

### Manual Installation

1. **Install system dependencies:**

   **Ubuntu/Debian:**

   ```bash
   sudo apt update
   sudo apt install python3-gi gir1.2-gtk-4.0 python3-dev python3-pip
   ```

   **Fedora:**

   ```bash
   sudo dnf install python3-gobject gtk4-devel python3-devel python3-pip
   ```

   **Arch Linux:**

   ```bash
   sudo pacman -S python-gobject gtk4 python python-pip
   ```

2. **Install Python dependencies:**
   ```bash
   pip3 install -r requirements.txt
   pip3 install -e .
   ```

## 🎮 Usage

### Running the Overlay

```bash
# Run the main overlay
python3 -m src

# Run in development mode with hot reloading
python3 -m src.dev_server
```

### Controls

- **Ctrl+Shift+O** - Toggle overlay visibility
- **Ctrl+Shift+F** - Quick search
- **Escape** - Hide overlay
- **Mouse drag** - Move overlay around screen
- **Ctrl+Shift+Arrow keys** - Fine-tune overlay position

## 🔧 Configuration

The overlay automatically creates a configuration file at `~/.config/poe2-master/config.json`. You can modify settings through the built-in settings dialog or edit the file directly.

### Key Settings

- **Window positioning** - Save and restore overlay location
- **Transparency** - Adjust overlay opacity
- **Auto-show/hide** - Automatically show when POE2 starts
- **Hotkeys** - Customize keyboard shortcuts
- **API settings** - Configure rate limiting and caching

## 🏗️ Architecture

The project uses a modular architecture with clear separation of concerns:

- **Core** - Event bus, process monitoring, hotkey management
- **UI** - GTK4-based interface components
- **Config** - Configuration management and validation
- **Services** - Business logic and API integration

## 🧪 Development

### Quick Start

```bash
# Install development dependencies
make install-dev

# Start development server with hot reloading
make dev

# Run the application
make run

# Run tests
make test
```

### Development Tools

#### Makefile Commands

```bash
# Development
make dev         # Start development server with hot reloading
make run         # Run the overlay application
make run-debug   # Run with debug logging enabled

# Installation
make install     # Install production dependencies
make install-dev # Install development dependencies
make check-deps  # Check if all dependencies are installed

# Testing
make test        # Run all tests
make test-unit   # Run unit tests only
make test-cov    # Run tests with coverage report

# Code Quality
make lint        # Run linting checks
make format      # Format code with black
make check       # Run all quality checks

# Build & Clean
make build       # Build the package
make clean       # Clean build artifacts and cache
make distclean   # Deep clean (including venv)

# Quick workflows
make setup-dev   # Setup development environment
make dev-workflow # Complete development cycle
```

#### Development Scripts

```bash
# Development server with hot reloading
./dev.sh dev

# Run the application
./dev.sh run

# Run tests
./dev.sh test

# Install dependencies
./dev.sh install

# Code quality checks
./dev.sh check

# Clean build artifacts
./dev.sh clean
```

#### Quick Run Script

```bash
# Simple run script
./run.sh
```

### Running Tests

```bash
# Run all tests
make test

# Run specific test categories
pytest tests/unit/
pytest tests/integration/

# Run with coverage
make test-cov
```

### Code Quality

```bash
# Format code
make format

# Lint code
make lint

# Run all checks
make check
```

### Development Cycle

```bash
# Complete development cycle (clean, check, run)
make dev-workflow

# Quick test and run
make quick
```

## 📁 Project Structure

```
poe2-master/
├── src/                    # Source code
│   ├── core/              # Core functionality
│   ├── ui/                # GTK4 UI components
│   ├── config/            # Configuration management
│   └── services/          # Business logic
├── tests/                 # Test suite
├── examples/              # Example configurations
├── requirements.txt       # Python dependencies
├── pyproject.toml         # Project configuration
├── install.sh             # Installation script
├── Makefile               # Development and build commands
├── dev.sh                 # Development script with hot reloading
├── run.sh                 # Quick run script
└── config.dev.json        # Development configuration
```

## 🔄 GTK4 Migration

This project has been fully migrated from **tkinter** to **GTK4** for:

- **Better Wayland support** - Native integration with modern Linux desktops
- **Improved performance** - Hardware acceleration and modern rendering
- **Better theming** - CSS-based styling and system theme integration
- **Modern widgets** - Rich set of UI components and layouts
- **Accessibility** - Better screen reader and keyboard navigation support

### Migration Status

- ✅ **Main Window** - Fully converted to GTK4
- ✅ **Search Panel** - Converted to GTK4 widgets
- ✅ **Results Panel** - Converted to GTK4 widgets
- ✅ **Settings Dialog** - Converted to GTK4 widgets
- ✅ **Theme Manager** - Updated for GTK4 CSS theming
- ✅ **Overlay Manager** - Updated for GTK4 Application
- ✅ **Dependencies** - Updated requirements and installation

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **GTK4** team for the excellent toolkit
- **Python** community for PyGObject bindings
- **Path of Exile** community for inspiration

## 📞 Support

- **Issues** - Report bugs and request features on GitHub
- **Discussions** - Join community discussions
- **Wiki** - Check the project wiki for detailed documentation

---

**Happy gaming! 🎮⚔️**
