# POE2 Master Overlay

A powerful game overlay for Path of Exile 2 on Linux, built with **GTK4** for modern Wayland support and optimal performance. This overlay provides item price checking, build planning, campaign progression tracking, and other utilities while gaming.

## 🚀 Features

- **Search Functionality** - Look up item prices and information with real-time API integration
- **Configurable Settings** - Customize appearance, behavior, and hotkeys through built-in settings
- **Modern GTK4 Interface** - Native Linux desktop integration with CSS-based theming
- **Wayland Support** - Full compatibility with modern display servers and X11 fallback
- **Game Process Detection** - Automatically detects when POE2 is running and shows/hides overlay
- **Global Hotkeys** - Toggle overlay with customizable keyboard shortcuts
- **Draggable Interface** - Move overlay anywhere on screen with position memory
- **Development Mode** - Hot reloading for rapid development and testing

## 🛠️ Requirements

- **Linux** (Ubuntu 22.04+, Fedora 36+, Arch Linux, etc.)
- **Python 3.8+**
- **GTK4** with Python bindings (PyGObject)
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

# Using the Makefile
make run          # Run the overlay
make dev          # Start development server
make run-debug    # Run with debug logging
```

### Controls

- **Ctrl+Shift+O** - Toggle overlay visibility
- **Ctrl+Shift+F** - Quick search
- **Escape** - Hide overlay
- **Ctrl+Shift+S** - Show settings dialog
- **Ctrl+Shift+R** - Refresh data
- **Mouse drag** - Move overlay around screen
- **Ctrl+Shift+Arrow keys** - Fine-tune overlay position

## 🔧 Configuration

The overlay automatically creates a configuration file at `~/.config/poe2-master/config.json`. You can modify settings through the built-in settings dialog or edit the file directly.

### Key Settings

- **Window positioning** - Save and restore overlay location
- **Transparency** - Adjust overlay opacity (0.1 - 1.0)
- **Auto-show/hide** - Automatically show when POE2 starts
- **Hotkeys** - Customize keyboard shortcuts
- **API settings** - Configure rate limiting, caching, and timeouts
- **Appearance** - Theme, fonts, colors, and animations
- **Process detection** - POE2 executable names and check intervals

### Configuration Structure

```json
{
  "window": {
    "width": 400,
    "height": 300,
    "transparency": 0.9,
    "always_on_top": true,
    "draggable": true
  },
  "hotkeys": {
    "toggle_overlay": "<ctrl>+<shift>+o",
    "quick_search": "<ctrl>+<shift>+f"
  },
  "api": {
    "rate_limit_requests": 10,
    "cache_ttl": 300
  },
  "search": {
    "default_league": "Early Access",
    "max_results": 10
  }
}
```

## 🏗️ Architecture

The project uses a modular architecture with clear separation of concerns:

- **Core** - Event bus, process monitoring, hotkey management, overlay coordination
- **UI** - GTK4-based interface components (main window, search panel, results panel, settings)
- **Config** - Configuration management, validation, and defaults
- **Utils** - Logging, helpers, and validation utilities

### Core Components

- **OverlayManager** - Main coordinator for the overlay lifecycle
- **ProcessMonitor** - Detects POE2 process and manages overlay visibility
- **HotkeyManager** - Handles global keyboard shortcuts
- **EventBus** - Inter-component communication system
- **ConfigManager** - Configuration loading, validation, and persistence

## 🧪 Development

### Quick Start

```bash
# Setup development environment
make setup-dev

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
│   │   ├── overlay_manager.py    # Main overlay coordinator
│   │   ├── process_monitor.py    # POE2 process detection
│   │   ├── hotkey_manager.py     # Global hotkey handling
│   │   └── event_bus.py          # Inter-component events
│   ├── ui/                # GTK4 UI components
│   │   ├── main_window.py        # Main overlay window
│   │   ├── search_panel.py       # Search interface
│   │   ├── results_panel.py      # Results display
│   │   ├── settings_dialog.py    # Settings configuration
│   │   └── themes.py             # CSS theming system
│   ├── config/            # Configuration management
│   │   ├── config_manager.py     # Config loading/validation
│   │   └── defaults.py           # Default configuration
│   └── utils/             # Utility functions
│       ├── logger.py              # Logging setup
│       ├── helpers.py             # Helper functions
│       └── validators.py          # Data validation
├── tests/                 # Test suite
│   └── unit/             # Unit tests
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

- ✅ **Main Window** - Fully converted to GTK4 with proper overlay behavior
- ✅ **Search Panel** - Converted to GTK4 widgets with modern search interface
- ✅ **Results Panel** - Converted to GTK4 widgets with responsive layout
- ✅ **Settings Dialog** - Converted to GTK4 widgets with form validation
- ✅ **Theme Manager** - Updated for GTK4 CSS theming system
- ✅ **Overlay Manager** - Updated for GTK4 Application lifecycle
- ✅ **Dependencies** - Updated requirements and installation scripts

## 🧪 Testing

The project includes a comprehensive test suite:

- **Unit Tests** - Core functionality testing
- **Integration Tests** - Component interaction testing
- **Test Coverage** - Coverage reporting with pytest-cov
- **Test Configuration** - pytest.ini with custom markers

### Test Categories

- **Unit Tests** - Individual component testing
- **Integration Tests** - Component interaction testing
- **Slow Tests** - Performance and stress testing

## 📚 Dependencies

### Core Dependencies

- **requests** - HTTP library for API calls
- **psutil** - System and process utilities
- **pynput** - Global hotkey support
- **PyGObject** - GTK4 Python bindings

### Development Dependencies

- **pytest** - Testing framework
- **pytest-cov** - Coverage reporting
- **black** - Code formatter
- **flake8** - Code linter
- **mypy** - Type checking
- **watchdog** - File watching for hot reloading

### System Dependencies

- **GTK4** - Modern GUI toolkit
- **Python 3.8+** - Python runtime
- **Linux** - Operating system

## 🚀 Performance Features

- **Hardware Acceleration** - GTK4 rendering with GPU support
- **Efficient Caching** - API response caching with configurable TTL
- **Rate Limiting** - Configurable API request throttling
- **Background Processing** - Non-blocking UI operations
- **Memory Management** - Efficient resource usage and cleanup

## 🔒 Security Features

- **Input Validation** - Comprehensive data validation
- **Rate Limiting** - API abuse prevention
- **Secure Configuration** - Safe configuration file handling
- **Process Isolation** - Secure process monitoring

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Install development dependencies (`make install-dev`)
4. Make your changes and add tests
5. Run the test suite (`make test`)
6. Format and lint your code (`make format && make lint`)
7. Commit your changes (`git commit -m 'Add amazing feature'`)
8. Push to the branch (`git push origin feature/amazing-feature`)
9. Open a Pull Request

### Development Guidelines

- Follow PEP 8 style guidelines
- Add tests for new functionality
- Update documentation as needed
- Use type hints for function parameters
- Handle errors gracefully with proper logging

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **GTK4** team for the excellent toolkit
- **Python** community for PyGObject bindings
- **Path of Exile** community for inspiration and feedback

## 📞 Support

- **Issues** - Report bugs and request features on GitHub
- **Discussions** - Join community discussions
- **Wiki** - Check the project wiki for detailed documentation

## 🔮 Roadmap

- [ ] **API Integration** - Real-time item price data
- [ ] **Build Planner** - Character build planning tools
- [ ] **Campaign Tracker** - Story progression tracking
- [ ] **Plugin System** - Extensible overlay functionality
- [ ] **Mobile Companion** - Mobile app integration
- [ ] **Cloud Sync** - Configuration and data synchronization

---

**Happy gaming! 🎮⚔️**

_Built with ❤️ for the Path of Exile 2 community_
