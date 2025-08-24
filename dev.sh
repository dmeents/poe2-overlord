#!/bin/bash
# POE2 Master Overlay - Development Script
# Quick access to common development tasks

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show help
show_help() {
    echo "POE2 Master Overlay - Development Script"
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  dev         - Start development server with hot reloading"
    echo "  run         - Run the overlay application"
    echo "  test        - Run tests"
    echo "  install     - Install dependencies"
    echo "  clean       - Clean build artifacts"
    echo "  check       - Run code quality checks"
    echo "  help        - Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 dev      - Start development server"
    echo "  $0 test     - Run all tests"
    echo "  $0 install  - Install dependencies"
}

# Function to check if Python is available
check_python() {
    if ! command -v python3 &> /dev/null; then
        print_error "Python 3 is not installed or not in PATH"
        exit 1
    fi
    
    python_version=$(python3 -c "import sys; print(f'{sys.version_info.major}.{sys.version_info.minor}')")
    print_status "Using Python $python_version"
}

# Function to check dependencies
check_dependencies() {
    print_status "Checking dependencies..."
    
    # Check GTK4
    if ! python3 -c "import gi; gi.require_version('Gtk', '4.0')" 2>/dev/null; then
        print_error "GTK4 bindings not available. Please install python3-gi and gir1.2-gtk-4.0"
        exit 1
    fi
    
    # Check other dependencies
    for dep in watchdog psutil pynput; do
        if ! python3 -c "import $dep" 2>/dev/null; then
            print_warning "$dep not available. Run '$0 install' to install dependencies"
        fi
    done
    
    print_success "Dependencies check completed"
}

# Function to install dependencies
install_dependencies() {
    print_status "Installing dependencies..."
    
    if [ -f "requirements.txt" ]; then
        pip3 install -r requirements.txt
        print_success "Dependencies installed from requirements.txt"
    else
        print_error "requirements.txt not found"
        exit 1
    fi
    
    # Install in development mode
    pip3 install -e .
    print_success "Package installed in development mode"
}

# Function to start development server
start_dev_server() {
    print_status "Starting development server with hot reloading..."
    python3 -m src.dev_server
}

# Function to run the application
run_application() {
    print_status "Running POE2 Master Overlay..."
    python3 -m src
}

# Function to run tests
run_tests() {
    print_status "Running tests..."
    
    if command -v pytest &> /dev/null; then
        python3 -m pytest tests/ -v
    else
        print_warning "pytest not available. Installing development dependencies..."
        install_dependencies
        python3 -m pytest tests/ -v
    fi
}

# Function to run code quality checks
run_checks() {
    print_status "Running code quality checks..."
    
    # Check if tools are available
    if ! command -v flake8 &> /dev/null; then
        print_warning "flake8 not available. Installing development dependencies..."
        install_dependencies
    fi
    
    if ! command -v black &> /dev/null; then
        print_warning "black not available. Installing development dependencies..."
        install_dependencies
    fi
    
    # Run checks
    print_status "Running flake8..."
    python3 -m flake8 src/ tests/ --max-line-length=100 --ignore=E501,W503
    
    print_status "Running black check..."
    python3 -m black --check src/ tests/ --line-length=100
    
    print_success "All code quality checks passed!"
}

# Function to clean build artifacts
clean_artifacts() {
    print_status "Cleaning build artifacts..."
    
    # Remove Python cache
    find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true
    find . -type f -name "*.pyc" -delete 2>/dev/null || true
    find . -type f -name "*.pyo" -delete 2>/dev/null || true
    
    # Remove build artifacts
    find . -type d -name "*.egg-info" -exec rm -rf {} + 2>/dev/null || true
    find . -type d -name "build" -exec rm -rf {} + 2>/dev/null || true
    find . -type d -name "dist" -exec rm -rf {} + 2>/dev/null || true
    
    # Remove test artifacts
    find . -type d -name ".pytest_cache" -exec rm -rf {} + 2>/dev/null || true
    find . -type d -name ".coverage" -exec rm -rf {} + 2>/dev/null || true
    find . -type d -name "htmlcov" -exec rm -rf {} + 2>/dev/null || true
    
    print_success "Cleanup completed"
}

# Main script logic
main() {
    # Check Python availability
    check_python
    
    # Parse command line arguments
    case "${1:-help}" in
        dev)
            check_dependencies
            start_dev_server
            ;;
        run)
            check_dependencies
            run_application
            ;;
        test)
            run_tests
            ;;
        install)
            install_dependencies
            ;;
        clean)
            clean_artifacts
            ;;
        check)
            run_checks
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            print_error "Unknown command: $1"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

# Run main function with all arguments
main "$@"
