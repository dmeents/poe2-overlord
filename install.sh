#!/bin/bash

# POE2 Master Overlay Installation Script
# This script installs the required dependencies for the GTK4-based overlay

set -e

echo "🚀 Installing POE2 Master Overlay dependencies..."

# Detect package manager
if command -v apt-get &> /dev/null; then
    PKG_MANAGER="apt-get"
    UPDATE_CMD="apt-get update"
    INSTALL_CMD="apt-get install -y"
elif command -v dnf &> /dev/null; then
    PKG_MANAGER="dnf"
    UPDATE_CMD="dnf update -y"
    INSTALL_CMD="dnf install -y"
elif command -v pacman &> /dev/null; then
    PKG_MANAGER="pacman"
    UPDATE_CMD="pacman -Sy"
    INSTALL_CMD="pacman -S --noconfirm"
elif command -v zypper &> /dev/null; then
    PKG_MANAGER="zypper"
    UPDATE_CMD="zypper refresh"
    INSTALL_CMD="zypper install -y"
else
    echo "❌ Unsupported package manager. Please install dependencies manually."
    exit 1
fi

echo "📦 Using package manager: $PKG_MANAGER"

# Update package lists
echo "🔄 Updating package lists..."
sudo $UPDATE_CMD

# Install system dependencies
echo "📥 Installing system dependencies..."

# GTK4 and Python bindings
if [ "$PKG_MANAGER" = "apt-get" ]; then
    sudo $INSTALL_CMD python3-gi gir1.2-gtk-4.0 python3-dev
elif [ "$PKG_MANAGER" = "dnf" ]; then
    sudo $INSTALL_CMD python3-gobject gtk4-devel python3-devel
elif [ "$PKG_MANAGER" = "pacman" ]; then
    sudo $INSTALL_CMD python-gobject gtk4 python
elif [ "$PKG_MANAGER" = "zypper" ]; then
    sudo $INSTALL_CMD python3-gobject gtk4-devel python3-devel
fi

# Install Python dependencies
echo "🐍 Installing Python dependencies..."
pip3 install -r requirements.txt

# Install the package in development mode
echo "🔧 Installing POE2 Master Overlay in development mode..."
pip3 install -e .

echo "✅ Installation completed successfully!"
echo ""
echo "🎮 To run the overlay:"
echo "   python3 -m src"
echo ""
echo "🔧 To run in development mode:"
echo "   python3 -m src.dev_server"
echo ""
echo "📚 For more information, see README.md"
