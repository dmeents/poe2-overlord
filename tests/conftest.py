"""
Pytest configuration and fixtures for POE2 Master Overlay tests
"""

import pytest
import tempfile
import os
from pathlib import Path
from unittest.mock import Mock, patch

# Add src to Python path for imports
import sys
src_path = Path(__file__).parent.parent / "src"
sys.path.insert(0, str(src_path))

from src.config.config_manager import ConfigManager
from src.core.event_bus import EventBus, EventType


@pytest.fixture
def temp_config_file():
    """Create a temporary configuration file for testing"""
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f:
        f.write('{"test": "value"}')
        temp_file = f.name
    
    yield temp_file
    
    # Cleanup
    try:
        os.unlink(temp_file)
    except OSError:
        pass


@pytest.fixture
def mock_config():
    """Create a mock configuration manager"""
    config = Mock(spec=ConfigManager)
    config.get.return_value = None
    config.set.return_value = True
    return config


@pytest.fixture
def event_bus():
    """Create a fresh event bus for testing"""
    return EventBus()


@pytest.fixture
def sample_event():
    """Create a sample event for testing"""
    return EventType.POE2_STARTED, {"data": "test"}


@pytest.fixture
def temp_dir():
    """Create a temporary directory for testing"""
    with tempfile.TemporaryDirectory() as temp_dir:
        yield temp_dir


@pytest.fixture(autouse=True)
def setup_test_env():
    """Setup test environment variables"""
    # Set test environment
    os.environ['POE2_LOG_LEVEL'] = 'DEBUG'
    os.environ['POE2_LOG_FILE'] = ''
    os.environ['POE2_LOG_SYSLOG'] = 'false'
    
    yield
    
    # Cleanup
    for key in ['POE2_LOG_LEVEL', 'POE2_LOG_FILE', 'POE2_LOG_SYSLOG']:
        os.environ.pop(key, None)


@pytest.fixture
def mock_process():
    """Create a mock process for testing"""
    process = Mock()
    process.info.return_value = {
        'name': 'test_process.exe',
        'cmdline': ['test_process.exe', '--arg']
    }
    return process


@pytest.fixture
def mock_psutil():
    """Create a mock psutil for testing"""
    with patch('psutil.process_iter') as mock_iter:
        mock_iter.return_value = []
        yield mock_iter


@pytest.fixture
def mock_pynput():
    """Create a mock pynput for testing"""
    with patch('pynput.keyboard.GlobalHotKeys') as mock_hotkeys:
        mock_hotkeys.return_value.start.return_value = None
        mock_hotkeys.return_value.stop.return_value = None
        yield mock_hotkeys


@pytest.fixture
def mock_requests():
    """Create a mock requests session for testing"""
    with patch('requests.Session') as mock_session:
        mock_session.return_value.request.return_value.status_code = 200
        mock_session.return_value.request.return_value.json.return_value = {}
        yield mock_session


@pytest.fixture
def mock_tkinter():
    """Create a mock tkinter for testing"""
    with patch('tkinter.Tk') as mock_tk:
        mock_tk.return_value.geometry.return_value = None
        mock_tk.return_value.overrideredirect.return_value = None
        mock_tk.return_value.wm_attributes.return_value = None
        mock_tk.return_value.withdraw.return_value = None
        mock_tk.return_value.deiconify.return_value = None
        mock_tk.return_value.lift.return_value = None
        mock_tk.return_value.mainloop.return_value = None
        yield mock_tk


# Test markers
def pytest_configure(config):
    """Configure pytest with custom markers"""
    config.addinivalue_line(
        "markers", "slow: marks tests as slow (deselect with '-m \"not slow\"')"
    )
    config.addinivalue_line(
        "markers", "integration: marks tests as integration tests"
    )
    config.addinivalue_line(
        "markers", "unit: marks tests as unit tests"
    )
    config.addinivalue_line(
        "markers", "gui: marks tests that require GUI"
    )


# Test collection customization
def pytest_collection_modifyitems(config, items):
    """Modify test collection to add markers"""
    for item in items:
        # Mark tests in specific directories
        if "unit" in str(item.fspath):
            item.add_marker(pytest.mark.unit)
        elif "integration" in str(item.fspath):
            item.add_marker(pytest.mark.integration)
        elif "gui" in str(item.fspath):
            item.add_marker(pytest.mark.gui)
            
        # Mark slow tests based on function name
        if "slow" in item.name.lower():
            item.add_marker(pytest.mark.slow)
