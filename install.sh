#!/bin/bash
# Installation script for POE2 Master Overlay on Linux

set -e

echo "=== POE2 Master Overlay Installation Script ==="
echo "This script will install POE2 Master Overlay and its dependencies."
echo ""

# Check if running on Linux
if [[ "$OSTYPE" != "linux-gnu"* ]]; then
    echo "❌ This installer is designed for Linux systems only."
    exit 1
fi

# Check Python version
echo "🐍 Checking Python version..."
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 is not installed. Please install Python 3.8 or higher."
    exit 1
fi

PYTHON_VERSION=$(python3 -c 'import sys; print(".".join(map(str, sys.version_info[:2])))')
REQUIRED_VERSION="3.8"

if ! python3 -c "import sys; exit(0 if sys.version_info >= (3, 8) else 1)"; then
    echo "❌ Python ${PYTHON_VERSION} detected. Python 3.8 or higher is required."
    exit 1
fi

echo "✅ Python ${PYTHON_VERSION} detected."

# Check if pip is available
echo "📦 Checking pip..."
if ! command -v pip3 &> /dev/null; then
    echo "❌ pip3 is not installed. Please install pip for Python 3."
    exit 1
fi
echo "✅ pip3 is available."

# Install system dependencies
echo ""
echo "🔧 Installing system dependencies..."

# Detect package manager
if command -v apt &> /dev/null; then
    PKG_MANAGER="apt"
    INSTALL_CMD="sudo apt install -y"
elif command -v dnf &> /dev/null; then
    PKG_MANAGER="dnf"
    INSTALL_CMD="sudo dnf install -y"
elif command -v pacman &> /dev/null; then
    PKG_MANAGER="pacman"
    INSTALL_CMD="sudo pacman -S --noconfirm"
elif command -v zypper &> /dev/null; then
    PKG_MANAGER="zypper"
    INSTALL_CMD="sudo zypper install -y"
else
    echo "⚠️  Could not detect package manager. Please manually install:"
    echo "   - python3-tk (tkinter)"
    echo "   - python3-dev (development headers)"
    echo "   - xorg-dev (X11 development libraries)"
    PKG_MANAGER="manual"
fi

if [ "$PKG_MANAGER" != "manual" ]; then
    echo "📋 Detected package manager: $PKG_MANAGER"
    
    case $PKG_MANAGER in
        "apt")
            $INSTALL_CMD python3-tk python3-dev libx11-dev libxtst-dev
            ;;
        "dnf")
            $INSTALL_CMD tkinter python3-devel libX11-devel libXtst-devel
            ;;
        "pacman")
            $INSTALL_CMD tk python libx11 libxtst
            ;;
        "zypper")
            $INSTALL_CMD python3-tk python3-devel libX11-devel libXtst-devel
            ;;
    esac
    
    if [ $? -eq 0 ]; then
        echo "✅ System dependencies installed successfully."
    else
        echo "❌ Failed to install system dependencies. Please install manually."
        exit 1
    fi
fi

# Create virtual environment
echo ""
echo "🏠 Setting up Python virtual environment..."
if [ -d "venv" ]; then
    echo "⚠️  Virtual environment already exists. Removing old one..."
    rm -rf venv
fi

python3 -m venv venv
source venv/bin/activate

echo "✅ Virtual environment created."

# Upgrade pip
echo "⬆️  Upgrading pip..."
pip install --upgrade pip

# Install Python dependencies
echo ""
echo "📚 Installing Python dependencies..."
pip install -r requirements.txt

if [ $? -eq 0 ]; then
    echo "✅ Python dependencies installed successfully."
else
    echo "❌ Failed to install Python dependencies."
    exit 1
fi

# Install the package in development mode
echo ""
echo "🔧 Installing POE2 Master Overlay in development mode..."
pip install -e .

if [ $? -eq 0 ]; then
    echo "✅ POE2 Master Overlay installed successfully."
else
    echo "❌ Failed to install POE2 Master Overlay."
    exit 1
fi

# Create desktop entry (optional)
echo ""
read -p "🖥️  Create desktop entry? (y/N): " CREATE_DESKTOP
if [[ $CREATE_DESKTOP =~ ^[Yy]$ ]]; then
    DESKTOP_FILE="$HOME/.local/share/applications/poe2-master-overlay.desktop"
    CURRENT_DIR=$(pwd)
    
    cat > "$DESKTOP_FILE" << EOF
[Desktop Entry]
Name=POE2 Master Overlay
Comment=Game overlay for Path of Exile 2
Exec=$CURRENT_DIR/venv/bin/python -m src
Icon=applications-games
Terminal=false
Type=Application
Categories=Game;Utility;
StartupNotify=true
EOF
    
    chmod +x "$DESKTOP_FILE"
    echo "✅ Desktop entry created at $DESKTOP_FILE"
fi

# Update launch script
echo ""
echo "🚀 Updating launch script..."
cat > run.sh << 'EOF'
#!/bin/bash
# Launch script for POE2 Master Overlay

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Check if virtual environment exists
if [ -d "venv" ]; then
    echo "Activating virtual environment..."
    source venv/bin/activate
fi

# Check if package is installed
if python -c "import poe2_master" 2>/dev/null; then
    echo "Running POE2 Master Overlay (installed package)..."
    python -m src "$@"
else
    echo "Running POE2 Master Overlay (development mode)..."
    # Add src to Python path and run
    PYTHONPATH="${SCRIPT_DIR}/src:${PYTHONPATH}" python -m src "$@"
fi
EOF

chmod +x run.sh
echo "✅ Launch script updated: ./run.sh"

# Installation complete
echo ""
echo "🎉 Installation completed successfully!"
echo ""
echo "📋 Quick Start:"
echo "   1. Run: ./run.sh"
echo "   2. Or use: python -m src"
echo "   3. Use Ctrl+Shift+O to toggle overlay"
echo "   4. Configuration saved to: ~/.config/poe2-master/config.json"
echo ""
echo "📖 Features:"
echo "   • Automatic POE2 process detection"
echo "   • Item price checking (mock data for now)"
echo "   • Global hotkeys"
echo "   • Configurable transparency and positioning"
echo "   • Modular architecture with plugin support"
echo "   • Comprehensive logging and configuration"
echo ""
echo "🛠️  Development:"
echo "   • Run tests: make test"
echo "   • Code quality: make dev-cycle"
echo "   • Build package: make package"
echo ""
echo "⚠️  Note: This overlay uses mock data until POE2 API becomes available."
echo ""
echo "Happy gaming! 🎮"
