"""
Tests for SettingsDialog functionality using GTK4
"""

import pytest
from unittest.mock import Mock, patch, MagicMock

from src.ui.settings_dialog import SettingsDialog
from src.config.config_manager import ConfigManager


class TestSettingsDialog:
    """Test settings dialog functionality"""
    
    @pytest.fixture
    def mock_config(self):
        """Create a mock config manager"""
        config = Mock(spec=ConfigManager)
        config.get.return_value = 400  # Default width
        return config
    
    @pytest.fixture
    def mock_parent(self):
        """Create a mock parent window"""
        return Mock()
    
    @pytest.fixture
    def settings_dialog(self, mock_parent, mock_config):
        """Create a SettingsDialog instance for testing"""
        return SettingsDialog(mock_parent, mock_config)
    
    def test_settings_dialog_initialization(self, settings_dialog, mock_parent, mock_config):
        """Test that SettingsDialog initializes correctly"""
        assert settings_dialog.parent == mock_parent
        assert settings_dialog.config == mock_config
        assert settings_dialog.dialog is None
    
    @patch('gi.repository.Gtk.Dialog')
    def test_show_creates_dialog(self, mock_dialog_class, settings_dialog):
        """Test that show() creates a GTK4 dialog"""
        mock_dialog_instance = Mock()
        mock_dialog_class.return_value = mock_dialog_instance
        
        settings_dialog.show()
        
        mock_dialog_class.assert_called_once()
        assert settings_dialog.dialog == mock_dialog_instance
    
    @patch('gi.repository.Gtk.Dialog')
    def test_show_handles_existing_dialog(self, mock_dialog_class, settings_dialog):
        """Test that show() reuses existing dialog"""
        mock_dialog_instance = Mock()
        settings_dialog.dialog = mock_dialog_instance
        
        settings_dialog.show()
        
        # Should call present() on existing dialog, not create new one
        mock_dialog_instance.present.assert_called_once()
        mock_dialog_class.assert_not_called()
    
    def test_save_settings_updates_config(self, settings_dialog):
        """Test that _save_settings updates configuration"""
        # Mock the UI elements
        settings_dialog.width_var = Mock()
        settings_dialog.width_var.get_text.return_value = "500"
        
        settings_dialog.height_var = Mock()
        settings_dialog.height_var.get_text.return_value = "400"
        
        settings_dialog.transparency_var = Mock()
        settings_dialog.transparency_var.get_value.return_value = 0.8
        
        settings_dialog.auto_show_var = Mock()
        settings_dialog.auto_show_var.get_active.return_value = True
        
        settings_dialog.auto_hide_var = Mock()
        settings_dialog.auto_hide_var.get_active.return_value = False
        
        # Mock other required variables
        settings_dialog.font_size_var = Mock()
        settings_dialog.font_size_var.get_text.return_value = "12"
        
        settings_dialog.toggle_hotkey_var = Mock()
        settings_dialog.toggle_hotkey_var.get_text.return_value = "<ctrl>+<shift>+o"
        
        settings_dialog.search_hotkey_var = Mock()
        settings_dialog.search_hotkey_var.get_text.return_value = "<ctrl>+<shift>+f"
        
        settings_dialog.hide_hotkey_var = Mock()
        settings_dialog.hide_hotkey_var.get_text.return_value = "<escape>"
        
        settings_dialog.max_requests_var = Mock()
        settings_dialog.max_requests_var.get_text.return_value = "20"
        
        settings_dialog.time_window_var = Mock()
        settings_dialog.time_window_var.get_text.return_value = "120"
        
        settings_dialog.cache_ttl_var = Mock()
        settings_dialog.cache_ttl_var.get_text.return_value = "600"
        
        # Mock the config methods
        with patch.object(settings_dialog.config, 'set') as mock_set:
            with patch.object(settings_dialog.config, 'save') as mock_save:
                settings_dialog._save_settings()
                
                # Verify config values were set
                mock_set.assert_any_call('window.width', 500)
                mock_set.assert_any_call('window.height', 400)
                mock_set.assert_any_call('window.transparency', 0.8)
                mock_set.assert_any_call('window.auto_show_on_poe2_start', True)
                mock_set.assert_any_call('window.auto_hide_on_poe2_exit', False)
                
                # Verify config was saved
                mock_save.assert_called_once()
    
    def test_reset_defaults_calls_config_reset(self, settings_dialog):
        """Test that _reset_defaults calls config reset"""
        with patch.object(settings_dialog.config, 'reset_to_defaults') as mock_reset:
            with patch.object(settings_dialog, '_refresh_ui') as mock_refresh:
                settings_dialog._reset_defaults()
                
                mock_reset.assert_called_once()
                mock_refresh.assert_called_once()
    
    def test_close_dialog_destroys_dialog(self, settings_dialog):
        """Test that _on_close destroys the dialog"""
        mock_dialog = Mock()
        settings_dialog.dialog = mock_dialog
        
        settings_dialog._on_close()
        
        mock_dialog.destroy.assert_called_once()
        assert settings_dialog.dialog is None
    
    def test_close_dialog_handles_no_dialog(self, settings_dialog):
        """Test that _on_close handles case when no dialog exists"""
        settings_dialog.dialog = None
        
        # Should not raise exception
        settings_dialog._on_close()
