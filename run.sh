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
