"""
Tests for MainWindow settings functionality using GTK4
"""

import pytest
from unittest.mock import Mock, patch, MagicMock

from src.ui.main_window import MainWindow
from src.config.config_manager import ConfigManager


class TestMainWindowSettings:
    """Test settings functionality in MainWindow"""
    
    @pytest.fixture
    def mock_config(self):
        """Create a mock config manager"""
        config = Mock(spec=ConfigManager)
        config.get.return_value = 400  # Default width
        return config
    
    @pytest.fixture
    def main_window(self, mock_config):
        """Create a MainWindow instance for testing"""
        with patch('gi.repository.Gtk.ApplicationWindow.__init__'):
            with patch('gi.repository.Gtk.ApplicationWindow._setup_window'):
                with patch('gi.repository.Gtk.ApplicationWindow._setup_ui'):
                    with patch('gi.repository.Gtk.ApplicationWindow._setup_drag_events'):
                        with patch('gi.repository.Gtk.ApplicationWindow._setup_keyboard_shortcuts'):
                            with patch('gi.repository.Gtk.ApplicationWindow._restore_window_position'):
                                window = MainWindow(mock_config)
                                # Don't actually create the UI, just return the instance
                                return window
    
    def test_show_settings_method_exists(self, main_window):
        """Test that _show_settings method exists"""
        assert hasattr(main_window, '_show_settings')
        assert callable(main_window._show_settings)
    
    @patch('src.ui.main_window.SettingsDialog')
    def test_show_settings_creates_dialog(self, mock_settings_dialog, main_window):
        """Test that _show_settings creates a SettingsDialog"""
        mock_dialog_instance = Mock()
        mock_settings_dialog.return_value = mock_dialog_instance
        
        main_window._show_settings()
        
        mock_settings_dialog.assert_called_once_with(
            main_window, 
            main_window.config
        )
        mock_dialog_instance.show.assert_called_once()
    
    @patch('src.ui.main_window.SettingsDialog')
    def test_show_settings_handles_errors(self, mock_settings_dialog, main_window):
        """Test that _show_settings handles errors gracefully"""
        mock_settings_dialog.side_effect = Exception("Test error")
        
        # Should not raise exception and should log error
        with patch.object(main_window.logger, 'error') as mock_logger:
            main_window._show_settings()
            mock_logger.assert_called_once()
    
    def test_on_settings_clicked_creates_dialog(self, main_window):
        """Test that _on_settings_clicked creates a SettingsDialog"""
        with patch('src.ui.main_window.SettingsDialog') as mock_settings_dialog:
            mock_dialog_instance = Mock()
            mock_settings_dialog.return_value = mock_dialog_instance
            
            # Create a mock button
            mock_button = Mock()
            
            main_window._on_settings_clicked(mock_button)
            
            mock_settings_dialog.assert_called_once_with(
                main_window, 
                main_window.config
            )
            mock_dialog_instance.show.assert_called_once()
    
    def test_on_settings_clicked_handles_errors(self, main_window):
        """Test that _on_settings_clicked handles errors gracefully"""
        with patch('src.ui.main_window.SettingsDialog') as mock_settings_dialog:
            mock_settings_dialog.side_effect = Exception("Test error")
            
            # Create a mock button
            mock_button = Mock()
            
            # Should not raise exception and should log error
            with patch.object(main_window.logger, 'error') as mock_logger:
                main_window._on_settings_clicked(mock_button)
                mock_logger.assert_called_once()
    
    def test_main_window_initialization(self, main_window):
        """Test that MainWindow initializes correctly"""
        assert main_window.config is not None
        assert hasattr(main_window, 'logger')
        assert hasattr(main_window, 'is_dragging')
        assert main_window.is_dragging is False
    
    def test_window_properties(self, main_window):
        """Test that window properties are set correctly"""
        # Test that window state variables are initialized
        assert hasattr(main_window, 'drag_start_x')
        assert hasattr(main_window, 'drag_start_y')
        assert hasattr(main_window, 'drag_start_window_x')
        assert hasattr(main_window, 'drag_start_window_y')
