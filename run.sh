#!/bin/bash
# POE2 Master Overlay - Quick Run Script

# Check if Python is available
if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 is not installed or not in PATH"
    exit 1
fi

# Check if GTK4 bindings are available
if ! python3 -c "import gi; gi.require_version('Gtk', '4.0')" 2>/dev/null; then
    echo "❌ GTK4 bindings not available. Please install python3-gi and gir1.2-gtk-4.0"
    exit 1
fi

# Run the overlay
echo "🚀 Starting POE2 Master Overlay..."
python3 -m src
