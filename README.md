# POE2 Master Overlay

A powerful, modular game overlay for Path of Exile 2 on Linux, providing item price checking, build planning, campaign progression tracking, and other utilities while gaming.

## ✨ Features

- 🎯 **Automatic POE2 Detection**: Automatically shows/hides when POE2 is running
- 💰 **Item Price Checking**: Search for item prices using POE2 trade API (mock data for now)
- ⌨️ **Global Hotkeys**: Control overlay without Alt-tabbing from the game
- 🎨 **Customizable Interface**: Transparent, always-on-top overlay with configurable appearance
- 🔧 **Extensive Configuration**: Fully customizable settings via JSON config with hot-reloading
- 💾 **Intelligent Caching**: Reduces API calls with smart caching system
- 🚀 **Modular Architecture**: Plugin-based system for easy feature extension
- 📊 **Comprehensive Logging**: Detailed logging with configurable levels and outputs
- 🧪 **Full Test Coverage**: Unit and integration tests with pytest
- 🔄 **Event-Driven System**: Loose coupling between components via event bus

## 🏗️ Architecture

The overlay is built with a modern, modular architecture that promotes maintainability and extensibility:

```
src/
├── core/                    # Core system components
│   ├── overlay_manager.py   # Main application coordinator
│   ├── process_monitor.py   # POE2 process detection
│   ├── hotkey_manager.py    # Global hotkey handling
│   └── event_bus.py         # Event system for decoupling
├── ui/                      # User interface components
│   ├── main_window.py       # Main overlay window
│   ├── search_panel.py      # Item search interface
│   ├── results_panel.py     # Results display
│   ├── settings_dialog.py   # Configuration UI
│   └── themes.py            # UI theming system
├── api/                     # API and data layer
│   ├── base_client.py       # Abstract API client
│   ├── poe2_client.py       # POE2-specific client
│   ├── mock_client.py       # Mock data provider
│   ├── rate_limiter.py      # Rate limiting logic
│   └── cache_manager.py     # Caching system
├── services/                # Business logic layer
│   ├── item_service.py      # Item-related business logic
│   ├── build_service.py     # Build planning logic
│   ├── progression_service.py # Campaign tracking
│   └── notification_service.py # User notifications
├── config/                  # Configuration management
│   ├── config_manager.py    # Enhanced config system
│   ├── schema.py            # Configuration validation
│   └── defaults.py          # Default configurations
├── utils/                   # Utility modules
│   ├── logger.py            # Logging system
│   ├── validators.py        # Input validation
│   └── helpers.py           # Common utilities
└── plugins/                 # Plugin system
    ├── base_plugin.py       # Plugin interface
    ├── price_checker.py     # Item price checking
    ├── build_planner.py     # Build planning
    └── progression_tracker.py # Campaign tracking
```

## 🚀 Quick Start

### Prerequisites

- Python 3.8 or higher
- Linux with X11 (Wayland support limited)
- tkinter (usually included with Python)

### Installation

#### Option 1: Development Installation (Recommended)

```bash
# Clone the repository
git clone https://github.com/yourusername/poe2-master.git
cd poe2-master

# Install in development mode with all dependencies
make install-dev

# Or manually:
pip install -e ".[dev]"
```

#### Option 2: System Installation

```bash
# Run the installer script
./scripts/install.sh
```

#### Option 3: Package Installation

```bash
# Install from PyPI (when available)
pip install poe2-master-overlay

# Or build and install locally
make package
pip install dist/*.whl
```

## 🛠️ Development

### Hot Reloading Development Server

The project now includes a development server with hot reloading that automatically restarts the overlay when you make code changes:

```bash
# Start development server with hot reloading
make dev

# Or with verbose logging
make dev-verbose

# Or watch specific directories
make dev-watch

# Or use the development script directly
./dev.sh

# Or run the development server manually
python3 src/dev_server.py --verbose
```

### Development Features

- **🔥 Hot Reloading**: Automatically restarts the overlay when source code changes
- **📁 File Watching**: Monitors `src/` directory for Python file changes
- **⚡ Fast Restarts**: Intelligent restart cooldown prevents rapid restarts
- **🔧 Development Config**: Separate development configuration with enhanced logging
- **📊 Process Management**: Automatic process lifecycle management
- **🚫 Ignored Patterns**: Skips watching build artifacts, cache files, and temporary files

### Development Configuration

Create a `config.dev.json` file for development-specific settings:

```json
{
  "debug": {
    "development_mode": true,
    "enable_config_watching": true,
    "log_level": "DEBUG"
  },
  "development": {
    "hot_reload": true,
    "watch_source_files": true,
    "auto_restart_on_changes": true,
    "restart_cooldown": 1.0,
    "source_directories": ["src"],
    "verbose_logging": true
  }
}
```

### Development Commands

```bash
# Code quality and testing
make dev-cycle          # Format, lint, type-check, and test
make format            # Format code with black
make lint              # Run flake8 linting
make type-check        # Run mypy type checking
make test              # Run tests
make test-cov          # Run tests with coverage

# Development server
make dev               # Start development server
make dev-verbose       # Start with verbose logging
make dev-watch         # Watch src and tests directories
```

## 🚀 Running the Overlay

### Production Mode

```bash
# Run the overlay directly
make run

# Or using Python module
python3 -m src

# Or using the entry point (if installed)
poe2-overlay
```

### Development Mode with Hot Reloading

```bash
# Start development server with hot reloading
make dev

# Or use the development script
./dev.sh

# Or run manually with verbose logging
python3 src/dev_server.py --verbose
```

The development server automatically restarts the overlay when you make code changes, making development much more efficient.

## ⌨️ Controls

| Hotkey         | Action                        |
| -------------- | ----------------------------- |
| `Ctrl+Shift+O` | Toggle overlay visibility     |
| `Ctrl+Shift+F` | Quick search                  |
| `Ctrl+Shift+,` | Show settings                 |
| `Ctrl+Shift+R` | Refresh data                  |
| `Escape`       | Hide overlay (when focused)   |
| `Enter`        | Search for item in search box |

## 🔧 Configuration

The overlay uses a comprehensive configuration system with hot-reloading:

### Configuration File Location

- **User config**: `~/.config/poe2-master/config.json`
- **Default config**: Built into the application
- **Environment variables**: `POE2_LOG_LEVEL`, `POE2_LOG_FILE`, etc.

### Key Configuration Sections

```json
{
  "window": {
    "width": 400,
    "height": 300,
    "transparency": 0.9,
    "auto_show_on_poe2_start": true
  },
  "hotkeys": {
    "toggle_overlay": "<ctrl>+<shift>+o"
  },
  "api": {
    "rate_limit_requests": 10,
    "rate_limit_window": 60,
    "cache_ttl": 300
  },
  "search": {
    "max_results": 10,
    "default_league": "Early Access"
  },
  "appearance": {
    "theme": "dark",
    "font_family": "Arial",
    "font_size": 10
  }
}
```

### Hot-Reloading

Configuration changes are automatically detected and applied without restarting the overlay.

## 🧪 Testing and Quality

### Setting Up Development Environment

```bash
# Install development dependencies
make install-dev

# Setup pre-commit hooks
make setup-dev

# Verify installation
make check-env
```

### Running Tests

```bash
# Run all tests
make test

# Run tests with coverage
make test-cov

# Run specific test categories
pytest tests/unit/          # Unit tests only
pytest tests/integration/   # Integration tests only
pytest -m "not slow"        # Exclude slow tests
```

### Code Quality

```bash
# Format code
make format

# Lint code
make lint

# Type checking
make type-check

# Run all quality checks
make dev-cycle
```

### Building and Packaging

```bash
# Build package
make build

# Create distribution
make package

# Clean build artifacts
make clean
```

## 🔌 Plugin System

The overlay supports a plugin architecture for easy feature extension:

### Creating a Plugin

```python
from src.plugins.base_plugin import BasePlugin

class MyCustomPlugin(BasePlugin):
    def __init__(self):
        super().__init__("my_custom_plugin")

    def initialize(self):
        """Initialize the plugin"""
        pass

    def cleanup(self):
        """Cleanup plugin resources"""
        pass
```

### Plugin Configuration

```json
{
  "plugins": {
    "auto_load": true,
    "enabled_plugins": ["price_checker", "build_planner", "my_custom_plugin"]
  }
}
```

## 📊 Logging

The overlay includes a comprehensive logging system:

### Log Levels

- **DEBUG**: Detailed debugging information
- **INFO**: General information messages
- **WARNING**: Warning messages
- **ERROR**: Error messages
- **CRITICAL**: Critical error messages

### Log Outputs

- **Console**: Real-time logging during development
- **File**: Rotating log files with configurable size limits
- **Syslog**: System logging integration

### Environment Variables

```bash
export POE2_LOG_LEVEL=DEBUG
export POE2_LOG_FILE=/path/to/logfile.log
export POE2_LOG_SYSLOG=true
```

## 🚀 Roadmap

### Phase 1 (Current) ✅

- [x] Modular architecture implementation
- [x] Event-driven component system
- [x] Enhanced configuration management
- [x] Comprehensive logging system
- [x] Plugin system foundation
- [x] Full test coverage setup

### Phase 2 (When POE2 API is Available)

- [ ] Real POE2 trade API integration
- [ ] Currency conversion
- [ ] Advanced search filters
- [ ] Whisper message copying
- [ ] Price alerts

### Phase 3 (Advanced Features)

- [ ] Build planning integration
- [ ] Stash tab organization
- [ ] Atlas progression tracking
- [ ] League mechanics tracking
- [ ] DPS calculator integration

## 🐛 Troubleshooting

### Common Issues

#### Overlay Not Appearing

1. Check if POE2 is detected: Look at status in overlay
2. Try manual toggle with `Ctrl+Shift+O`
3. Check X11 vs Wayland: `echo $XDG_SESSION_TYPE`
4. Verify permissions for global hotkeys

#### Hotkeys Not Working

1. Ensure `pynput` is installed: `pip install pynput`
2. Check for permission issues
3. Verify hotkey configuration in config file

#### Configuration Issues

1. Check configuration file syntax: `~/.config/poe2-master/config.json`
2. Reset to defaults: Use settings dialog or delete config file
3. Verify file permissions

### Debug Mode

```bash
# Enable debug logging
export POE2_LOG_LEVEL=DEBUG

# Run with verbose output
python -m src --verbose
```

### Log Files

Check log files for detailed error information:

- **User logs**: `~/.local/share/poe2-master/logs/`
- **System logs**: `/var/log/syslog` (if syslog enabled)

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Workflow

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass: `make test`
6. Run code quality checks: `make dev-cycle`
7. Commit your changes: `git commit -m 'Add amazing feature'`
8. Push to the branch: `git push origin feature/amazing-feature`
9. Open a Pull Request

### Code Style

We use several tools to maintain code quality:

- **Black**: Code formatting
- **Flake8**: Linting
- **MyPy**: Type checking
- **Pre-commit**: Git hooks

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Path of Exile 2 by Grinding Gear Games
- Python tkinter for GUI framework
- pynput for global hotkey support
- psutil for process monitoring
- All contributors and testers

## 📞 Support

If you encounter issues:

1. Check the troubleshooting section above
2. Look for similar issues in the [GitHub issues](https://github.com/yourusername/poe2-master/issues)
3. Create a new issue with:
   - Your Linux distribution and version
   - Python version
   - Steps to reproduce the problem
   - Any error messages
   - Log files (if available)

## 📚 Documentation

- [API Reference](docs/api.md)
- [Plugin Development](docs/plugins.md)
- [Development Guide](docs/development.md)
- [Deployment Guide](docs/deployment.md)

---

**Note**: This overlay currently uses mock data for testing. Real POE2 API integration will be added once the official API becomes available.

Happy gaming! 🎮
