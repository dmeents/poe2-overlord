"""
Unit tests for the Configuration Manager
"""

import pytest
import tempfile
import os
from pathlib import Path
from unittest.mock import Mock, patch

from src.config.config_manager import ConfigManager


class TestConfigManager:
    """Test the Configuration Manager"""
    
    def test_config_manager_initialization(self):
        """Test basic initialization"""
        config = ConfigManager()
        assert config.config is not None
        assert 'window' in config.config
        assert 'api' in config.config
        
    def test_get_config_value(self):
        """Test getting configuration values"""
        config = ConfigManager()
        
        # Test basic get
        width = config.get('window.width')
        assert width == 400  # Default value
        
        # Test with default
        unknown_value = config.get('unknown.key', 'default')
        assert unknown_value == 'default'
        
    def test_set_config_value(self):
        """Test setting configuration values"""
        config = ConfigManager()
        
        # Test setting a value
        success = config.set('window.width', 500)
        assert success is True
        
        # Verify the value was set
        width = config.get('window.width')
        assert width == 500
        
    def test_config_validation(self):
        """Test configuration validation"""
        config = ConfigManager()
        
        # Test valid values
        config.set('window.width', 300)
        config.set('window.height', 200)
        config.set('window.transparency', 0.8)
        
        # Test invalid values (should still work but log warnings)
        config.set('window.width', 100)  # Below minimum
        config.set('window.height', 2000)  # Above maximum
        
        # Get validation issues
        issues = config.validate_config()
        assert len(issues) > 0  # Should have validation warnings
        
    def test_config_file_operations(self, temp_config_file):
        """Test configuration file operations"""
        config = ConfigManager(temp_config_file)
        
        # Test export
        export_path = temp_config_file + ".export"
        success = config.export_config(export_path)
        assert success is True
        
        # Test import
        success = config.import_config(export_path)
        assert success is True
        
        # Cleanup
        os.unlink(export_path)
        
    def test_config_watchers(self):
        """Test configuration watchers"""
        config = ConfigManager()
        
        # Mock callback
        callback_called = False
        def test_callback(config_data):
            nonlocal callback_called
            callback_called = True
            
        # Add watcher
        config.add_watcher(test_callback)
        
        # Change config
        config.set('window.width', 600)
        
        # Watcher should have been called
        assert callback_called is True
        
        # Remove watcher
        config.remove_watcher(test_callback)
        
    def test_config_sections(self):
        """Test configuration section operations"""
        config = ConfigManager()
        
        # Get section
        window_section = config.get_section('window')
        assert isinstance(window_section, dict)
        assert 'width' in window_section
        
        # Set section
        new_api_config = {
            'rate_limit_requests': 20,
            'rate_limit_window': 120
        }
        success = config.set_section('api', new_api_config)
        assert success is True
        
        # Verify section was set
        api_section = config.get_section('api')
        assert api_section['rate_limit_requests'] == 20
        
    def test_config_reset(self):
        """Test configuration reset to defaults"""
        config = ConfigManager()
        
        # Change some values
        config.set('window.width', 800)
        config.set('window.height', 600)
        
        # Reset to defaults
        config.reset_to_defaults()
        
        # Verify values are back to defaults
        assert config.get('window.width') == 400
        assert config.get('window.height') == 300
        
    def test_config_has_key(self):
        """Test checking if configuration keys exist"""
        config = ConfigManager()
        
        # Test existing keys
        assert config.has_key('window.width') is True
        assert config.has_key('api.rate_limit_requests') is True
        
        # Test non-existing keys
        assert config.has_key('unknown.key') is False
        assert config.has_key('window.unknown') is False
        
    def test_config_reload(self):
        """Test configuration reloading"""
        config = ConfigManager()
        
        # Change a value
        original_width = config.get('window.width')
        config.set('window.width', 900)
        
        # Reload should restore original value
        config.reload()
        assert config.get('window.width') == original_width
