#!/bin/bash
# Development script for POE2 Master Overlay with hot reloading

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}POE2 Master Overlay - Development Mode${NC}"
echo "=============================================="

# Check if virtual environment is activated
if [[ "$VIRTUAL_ENV" == "" ]]; then
    echo -e "${YELLOW}Warning: No virtual environment detected${NC}"
    echo "Consider activating your virtual environment first:"
    echo "  source venv/bin/activate  # or your venv path"
    echo ""
fi

# Check if watchdog is installed
if ! python3 -c "import watchdog" 2>/dev/null; then
    echo -e "${YELLOW}Installing development dependencies...${NC}"
    pip install -e ".[dev]"
    echo ""
fi

# Check if development config exists
if [[ ! -f "config.dev.json" ]]; then
    echo -e "${YELLOW}Development config not found, creating...${NC}"
    cat > config.dev.json << 'EOF'
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
  },
  "window": {
    "always_visible": true,
    "transparency": 0.95
  }
}
EOF
    echo -e "${GREEN}Development config created: config.dev.json${NC}"
    echo ""
fi

echo -e "${GREEN}Starting development server with hot reloading...${NC}"
echo "The overlay will automatically restart when you make code changes."
echo "Press Ctrl+C to stop the development server."
echo ""

# Start the development server
python3 src/dev_server.py --verbose
