#!/bin/bash
# POE2 Master Overlay - Quick Run Script

# Virtual environment paths
VENV="venv"
PYTHON="$VENV/bin/python"

# Check if virtual environment exists
if [ ! -d "$VENV" ]; then
    echo "❌ Virtual environment not found. Please run 'make install-dev' first."
    exit 1
fi

# Check if Python is available in virtual environment
if [ ! -f "$PYTHON" ]; then
    echo "❌ Python not found in virtual environment"
    exit 1
fi

# Check if GTK4 bindings are available
if ! $PYTHON -c "import gi; gi.require_version('Gtk', '4.0')" 2>/dev/null; then
    echo "❌ GTK4 bindings not available. Please install python3-gi and gir1.2-gtk-4.0"
    exit 1
fi

# Run the overlay
echo "🚀 Starting POE2 Master Overlay..."
$PYTHON -m src
