.PHONY: help install install-dev clean test test-cov lint format type-check docs build package run install-local uninstall

# Default target
help:
	@echo "POE2 Master Overlay - Development Commands"
	@echo ""
	@echo "Installation:"
	@echo "  install          Install the package in development mode"
	@echo "  install-dev      Install with development dependencies"
	@echo "  install-local    Install the package locally"
	@echo ""
	@echo "Development:"
	@echo "  test             Run tests"
	@echo "  test-cov         Run tests with coverage"
	@echo "  lint             Run linting checks"
	@echo "  format           Format code with black"
	@echo "  type-check       Run type checking with mypy"
	@echo "  docs             Build documentation"
	@echo ""
	@echo "Building:"
	@echo "  build            Build the package"
	@echo "  package          Create distribution packages"
	@echo ""
	@echo "Running:"
	@echo "  run              Run the overlay application"
	@echo ""
	@echo "Cleanup:"
	@echo "  clean            Clean build artifacts"
	@echo "  uninstall        Uninstall the package"

# Installation
install:
	pip install -e .

install-dev:
	pip install -e ".[dev]"

install-local:
	pip install .

uninstall:
	pip uninstall poe2-master-overlay -y

# Development
test:
	pytest tests/

test-cov:
	pytest tests/ --cov=poe2_master --cov-report=term-missing --cov-report=html

lint:
	flake8 src/ tests/
	black --check src/ tests/

format:
	black src/ tests/

type-check:
	mypy src/

docs:
	cd docs && make html

# Development mode with hot reloading
dev:
	python3 src/dev_server.py

dev-verbose:
	python3 src/dev_server.py --verbose

dev-watch:
	python3 src/dev_server.py --source-dirs src tests

# Building
build:
	python -m build

package: build
	@echo "Package built in dist/ directory"

# Running
run:
	python -m src

# Cleanup
clean:
	rm -rf build/
	rm -rf dist/
	rm -rf *.egg-info/
	rm -rf .pytest_cache/
	rm -rf .coverage
	rm -rf htmlcov/
	rm -rf .mypy_cache/
	find . -type d -name __pycache__ -exec rm -rf {} +
	find . -type f -name "*.pyc" -delete

# Development environment setup
setup-dev: install-dev
	pre-commit install

# Quick development cycle
dev-cycle: format lint type-check test

# Docker development (if needed)
docker-build:
	docker build -t poe2-master-overlay .

docker-run:
	docker run -it --rm poe2-master-overlay

# System integration
install-system:
	sudo ./scripts/install.sh

uninstall-system:
	sudo ./scripts/uninstall.sh

# Database operations (if applicable)
db-migrate:
	@echo "Database migration not implemented yet"

db-reset:
	@echo "Database reset not implemented yet"

# Performance profiling
profile:
	python -m cProfile -o profile.stats -m poe2_master

profile-view:
	python -c "import pstats; p = pstats.Stats('profile.stats'); p.sort_stats('cumulative').print_stats(20)"

# Security checks
security-check:
	bandit -r src/
	safety check

# Dependency management
update-deps:
	pip install --upgrade pip
	pip install --upgrade -r requirements.txt

freeze-deps:
	pip freeze > requirements.lock

# Git operations
git-hooks:
	pre-commit install

# Release preparation
release-check: clean test lint type-check security-check
	@echo "Release checks completed successfully"

release: release-check package
	@echo "Release package created in dist/ directory"

# Helpers
check-env:
	@echo "Python version: $(shell python --version)"
	@echo "Pip version: $(shell pip --version)"
	@echo "Current directory: $(shell pwd)"
	@echo "Virtual environment: $(shell echo $$VIRTUAL_ENV)"

# Default target
.DEFAULT_GOAL := help
