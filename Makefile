# POE2 Master Overlay - Makefile
# Provides convenient commands for development, testing, and building

.PHONY: help install dev test clean build run install-dev lint format check-deps

# Virtual environment paths
VENV = venv
PYTHON = $(VENV)/bin/python
PIP = $(VENV)/bin/pip

# Default target
help:
	@echo "POE2 Master Overlay - Available Commands:"
	@echo ""
	@echo "Development:"
	@echo "  dev         - Start development server with hot reloading"
	@echo "  run         - Run the overlay application"
	@echo "  run-debug   - Run with debug logging enabled"
	@echo ""
	@echo "Installation:"
	@echo "  install     - Install production dependencies"
	@echo "  install-dev - Install development dependencies"
	@echo "  check-deps  - Check if all dependencies are installed"
	@echo ""
	@echo "Testing:"
	@echo "  test        - Run all tests"
	@echo "  test-unit   - Run unit tests only"
	@echo "  test-cov    - Run tests with coverage report"
	@echo ""
	@echo "Code Quality:"
	@echo "  lint        - Run linting checks"
	@echo "  format      - Format code with black"
	@echo "  check       - Run all quality checks"
	@echo ""
	@echo "Build & Clean:"
	@echo "  build       - Build the package"
	@echo "  clean       - Clean build artifacts and cache"
	@echo "  distclean   - Deep clean (including venv)"
	@echo ""
	@echo "Documentation:"
	@echo "  docs        - Generate documentation"
	@echo "  readme      - Update README with current status"
	@echo ""

# Development commands
dev:
	@echo "🚀 Starting development server with hot reloading..."
	@$(PYTHON) -m src.dev_server

run:
	@echo "🎯 Running POE2 Master Overlay..."
	@$(PYTHON) -m src

run-debug:
	@echo "🐛 Running POE2 Master Overlay with debug logging..."
	@POE2_DEBUG=1 $(PYTHON) -m src

# Installation commands
install:
	@echo "📦 Installing production dependencies..."
	@$(PIP) install -r requirements.txt

install-dev:
	@echo "🔧 Installing development dependencies..."
	@$(PIP) install -r requirements.txt
	@$(PIP) install -e .

check-deps:
	@echo "🔍 Checking dependencies..."
	@$(PYTHON) -c "import gi; gi.require_version('Gtk', '4.0'); print('✅ GTK4 bindings available')"
	@$(PYTHON) -c "import watchdog; print('✅ Watchdog available')"
	@$(PYTHON) -c "import psutil; print('✅ psutil available')"
	@$(PYTHON) -c "import pynput; print('✅ pynput available')"
	@echo "✅ All dependencies are available"

# Testing commands
test:
	@echo "🧪 Running all tests..."
	@$(PYTHON) -m pytest tests/ -v

test-unit:
	@echo "🧪 Running unit tests..."
	@$(PYTHON) -m pytest tests/unit/ -v

test-cov:
	@echo "🧪 Running tests with coverage..."
	@$(PYTHON) -m pytest tests/ --cov=src --cov-report=html --cov-report=term-missing

# Code quality commands
lint:
	@echo "🔍 Running linting checks..."
	@$(PYTHON) -m flake8 src/ tests/ --max-line-length=100 --ignore=E501,W503

format:
	@echo "🎨 Formatting code with black..."
	@$(PYTHON) -m black src/ tests/ --line-length=100

check: lint test
	@echo "✅ All quality checks passed!"

# Build commands
build:
	@echo "🏗️ Building package..."
	@$(PYTHON) -m build

# Clean commands
clean:
	@echo "🧹 Cleaning build artifacts and cache..."
	@find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
	@find . -type f -name "*.pyc" -delete 2>/dev/null || true
	@find . -type f -name "*.pyo" -delete 2>/dev/null || true
	@find . -type d -name "*.egg-info" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name "build" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name "dist" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name ".pytest_cache" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name ".coverage" -exec rm -rf {} + 2>/dev/null || true
	@find . -type d -name "htmlcov" -exec rm -rf {} + 2>/dev/null || true

distclean: clean
	@echo "🧹 Deep cleaning (including virtual environment)..."
	@rm -rf venv/ 2>/dev/null || true
	@rm -rf .venv/ 2>/dev/null || true

# Documentation commands
docs:
	@echo "📚 Generating documentation..."
	@echo "Documentation generation not yet implemented"

readme:
	@echo "📝 Updating README..."
	@echo "README update not yet implemented"

# Quick development setup
setup-dev: install-dev check-deps
	@echo "✅ Development environment setup complete!"
	@echo "Run 'make dev' to start the development server"

# Production setup
setup-prod: install check-deps
	@echo "✅ Production environment setup complete!"
	@echo "Run 'make run' to start the overlay"

# Default development workflow
dev-workflow: clean check run
	@echo "✅ Development workflow complete!"

# Quick test and run
quick: test run
	@echo "✅ Quick test and run complete!"
