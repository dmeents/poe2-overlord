"""
Settings Dialog for POE2 Master Overlay

Provides a user interface for configuring overlay settings using GTK4.
"""

import gi
gi.require_version('Gtk', '4.0')
gi.require_version('Gdk', '4.0')
gi.require_version('Gio', '2.0')

from gi.repository import Gtk, Gdk, Gio, GLib
from typing import Optional
import logging

from ..config.config_manager import ConfigManager

logger = logging.getLogger(__name__)


class SettingsDialog:
    """Settings configuration dialog using GTK4"""
    
    def __init__(self, parent, config: ConfigManager, on_save_callback=None):
        """
        Initialize the settings dialog
        
        Args:
            parent: Parent window
            config: Configuration manager
            on_save_callback: Optional callback function to call when settings are saved
        """
        self.parent = parent
        self.config = config
        self.on_save_callback = on_save_callback
        self.dialog: Optional[Gtk.Dialog] = None
        
    def show(self):
        """Show the settings dialog"""
        if self.dialog:
            self.dialog.present()
            return
            
        # Create dialog
        self.dialog = Gtk.Dialog(
            title="POE2 Master Overlay - Settings (Ctrl+Shift+S)",
            transient_for=self.parent,
            modal=True
        )
        
        # Set dialog size
        self.dialog.set_default_size(600, 400)
        
        # Add close button
        close_button = self.dialog.add_button("Close", Gtk.ResponseType.CLOSE)
        close_button.connect("clicked", self._on_close)
        
        # Setup UI
        self._setup_ui()
        
        # Handle dialog close
        self.dialog.connect("response", self._on_response)
        
        # Show dialog
        self.dialog.show()
        
        logger.info("Settings dialog opened")
        
    def _setup_ui(self):
        """Setup the dialog interface using GTK4 widgets"""
        # Get content area
        content_area = self.dialog.get_content_area()
        
        # Create notebook for tabbed interface
        notebook = Gtk.Notebook()
        content_area.append(notebook)
        
        # General settings tab
        general_frame = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=10)
        general_frame.set_margin_start(10)
        general_frame.set_margin_end(10)
        general_frame.set_margin_top(10)
        general_frame.set_margin_bottom(10)
        notebook.append_page(general_frame, Gtk.Label(label="General"))
        self._create_general_tab(general_frame)
        
        # Appearance tab
        appearance_frame = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=10)
        appearance_frame.set_margin_start(10)
        appearance_frame.set_margin_end(10)
        appearance_frame.set_margin_top(10)
        appearance_frame.set_margin_bottom(10)
        notebook.append_page(appearance_frame, Gtk.Label(label="Appearance"))
        self._create_appearance_tab(appearance_frame)
        
        # Hotkeys tab
        hotkeys_frame = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=10)
        hotkeys_frame.set_margin_start(10)
        hotkeys_frame.set_margin_end(10)
        hotkeys_frame.set_margin_top(10)
        hotkeys_frame.set_margin_bottom(10)
        notebook.append_page(hotkeys_frame, Gtk.Label(label="Hotkeys"))
        self._create_hotkeys_tab(hotkeys_frame)
        
        # API tab
        api_frame = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=10)
        api_frame.set_margin_start(10)
        api_frame.set_margin_end(10)
        api_frame.set_margin_top(10)
        api_frame.set_margin_bottom(10)
        notebook.append_page(api_frame, Gtk.Label(label="API"))
        self._create_api_tab(api_frame)
        
        # Buttons
        button_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        button_box.set_margin_start(10)
        button_box.set_margin_end(10)
        button_box.set_margin_bottom(10)
        
        # Keyboard shortcut hint
        hint_label = Gtk.Label(label="Press Escape to close")
        hint_label.set_css_classes(["dim-label"])
        button_box.append(hint_label)
        
        # Button container
        button_container = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        button_container.set_hexpand(True)
        button_container.set_halign(Gtk.Align.END)
        
        # Reset to Defaults button
        reset_button = Gtk.Button(label="Reset to Defaults")
        reset_button.connect("clicked", self._reset_defaults)
        button_container.append(reset_button)
        
        # Cancel button
        cancel_button = Gtk.Button(label="Cancel")
        cancel_button.connect("clicked", self._on_close)
        button_container.append(cancel_button)
        
        # Save button
        save_button = Gtk.Button(label="Save")
        save_button.add_css_class("suggested-action")
        save_button.connect("clicked", self._save_settings)
        button_container.append(save_button)
        
        button_box.append(button_container)
        content_area.append(button_box)
        
    def _create_general_tab(self, parent):
        """Create the general settings tab using GTK4 widgets"""
        # Window settings frame
        window_frame = Gtk.Frame()
        window_frame.set_label("Window Settings")
        window_frame.set_margin_start(10)
        window_frame.set_margin_end(10)
        window_frame.set_margin_top(5)
        window_frame.set_margin_bottom(5)
        
        window_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=6)
        window_box.set_margin_start(10)
        window_box.set_margin_end(10)
        window_box.set_margin_top(10)
        window_box.set_margin_bottom(10)
        window_frame.set_child(window_box)
        
        # Width
        width_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        width_label = Gtk.Label(label="Width:")
        width_label.set_halign(Gtk.Align.START)
        width_box.append(width_label)
        
        self.width_var = Gtk.Entry()
        self.width_var.set_text(str(self.config.get('window.width', 400)))
        self.width_var.set_hexpand(True)
        width_box.append(self.width_var)
        window_box.append(width_box)
        
        # Height
        height_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        height_label = Gtk.Label(label="Height:")
        height_label.set_halign(Gtk.Align.START)
        height_box.append(height_label)
        
        self.height_var = Gtk.Entry()
        self.height_var.set_text(str(self.config.get('window.height', 300)))
        self.height_var.set_hexpand(True)
        height_box.append(self.height_var)
        window_box.append(height_box)
        
        # Transparency
        transparency_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        transparency_label = Gtk.Label(label="Transparency:")
        transparency_label.set_halign(Gtk.Align.START)
        transparency_box.append(transparency_label)
        
        self.transparency_var = Gtk.Adjustment(
            value=self.config.get('window.transparency', 0.9),
            lower=0.1,
            upper=1.0,
            step_increment=0.1
        )
        transparency_scale = Gtk.Scale(orientation=Gtk.Orientation.HORIZONTAL, adjustment=self.transparency_var)
        transparency_scale.set_hexpand(True)
        transparency_box.append(transparency_scale)
        window_box.append(transparency_box)
        
        # Auto-show/hide checkboxes
        self.auto_show_var = Gtk.CheckButton(label="Auto-show when POE2 starts")
        self.auto_show_var.set_active(self.config.get('window.auto_show_on_poe2_start', True))
        window_box.append(self.auto_show_var)
        
        self.auto_hide_var = Gtk.CheckButton(label="Auto-hide when POE2 exits")
        self.auto_hide_var.set_active(self.config.get('window.auto_hide_on_poe2_exit', False))
        window_box.append(self.auto_hide_var)
        
        parent.append(window_frame)
        
    def _create_appearance_tab(self, parent):
        """Create the appearance settings tab using GTK4 widgets"""
        appearance_frame = Gtk.Frame()
        appearance_frame.set_label("Appearance")
        appearance_frame.set_margin_start(10)
        appearance_frame.set_margin_end(10)
        appearance_frame.set_margin_top(5)
        appearance_frame.set_margin_bottom(5)
        
        appearance_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=6)
        appearance_box.set_margin_start(10)
        appearance_box.set_margin_end(10)
        appearance_box.set_margin_top(10)
        appearance_box.set_margin_bottom(10)
        appearance_frame.set_child(appearance_box)
        
        # Theme selection (simplified for now)
        theme_label = Gtk.Label(label="Theme:")
        theme_label.set_halign(Gtk.Align.START)
        theme_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        theme_box.append(theme_label)
        
        # TODO: Implement proper theme selection
        # For now, using default values since DropDown handling is complex
        theme_dropdown = Gtk.DropDown()
        theme_dropdown.set_model(Gtk.StringList.new(["Dark", "Light", "Auto"]))
        theme_dropdown.set_selected(0)  # Default to Dark
        theme_box.append(theme_dropdown)
        appearance_box.append(theme_box)
        
        # Font selection (simplified for now)
        font_label = Gtk.Label(label="Font:")
        font_label.set_halign(Gtk.Align.START)
        font_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        font_box.append(font_label)
        
        # TODO: Implement proper font selection
        # For now, using default values since DropDown handling is complex
        font_dropdown = Gtk.DropDown()
        font_dropdown.set_model(Gtk.StringList.new(["System", "Monospace", "Sans"]))
        font_dropdown.set_selected(0)  # Default to System
        font_box.append(font_dropdown)
        appearance_box.append(font_box)
        
        # Font size
        font_size_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        font_size_label = Gtk.Label(label="Font Size:")
        font_size_label.set_halign(Gtk.Align.START)
        font_size_box.append(font_size_label)
        
        self.font_size_var = Gtk.Entry()
        self.font_size_var.set_text(str(self.config.get('appearance.font_size', 10)))
        self.font_size_var.set_hexpand(True)
        font_size_box.append(self.font_size_var)
        appearance_box.append(font_size_box)
        
        parent.append(appearance_frame)
        
    def _create_hotkeys_tab(self, parent):
        """Create the hotkeys settings tab using GTK4 widgets"""
        hotkeys_frame = Gtk.Frame()
        hotkeys_frame.set_label("Hotkeys")
        hotkeys_frame.set_margin_start(10)
        hotkeys_frame.set_margin_end(10)
        hotkeys_frame.set_margin_top(5)
        hotkeys_frame.set_margin_bottom(5)
        
        hotkeys_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=6)
        hotkeys_box.set_margin_start(10)
        hotkeys_box.set_margin_end(10)
        hotkeys_box.set_margin_top(10)
        hotkeys_box.set_margin_bottom(10)
        hotkeys_frame.set_child(hotkeys_box)
        
        # Toggle overlay
        toggle_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        toggle_label = Gtk.Label(label="Toggle Overlay:")
        toggle_label.set_halign(Gtk.Align.START)
        toggle_box.append(toggle_label)
        
        self.toggle_hotkey_var = Gtk.Entry()
        self.toggle_hotkey_var.set_text(self.config.get('hotkeys.toggle_overlay', '<ctrl>+<shift>+o'))
        self.toggle_hotkey_var.set_hexpand(True)
        toggle_box.append(self.toggle_hotkey_var)
        hotkeys_box.append(toggle_box)
        
        # Quick search
        search_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        search_label = Gtk.Label(label="Quick Search:")
        search_label.set_halign(Gtk.Align.START)
        search_box.append(search_label)
        
        self.search_hotkey_var = Gtk.Entry()
        self.search_hotkey_var.set_text(self.config.get('hotkeys.quick_search', '<ctrl>+<shift>+f'))
        self.search_hotkey_var.set_hexpand(True)
        search_box.append(self.search_hotkey_var)
        hotkeys_box.append(search_box)
        
        # Hide overlay
        hide_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        hide_label = Gtk.Label(label="Hide Overlay:")
        hide_label.set_halign(Gtk.Align.START)
        hide_box.append(hide_label)
        
        self.hide_hotkey_var = Gtk.Entry()
        self.hide_hotkey_var.set_text(self.config.get('hotkeys.hide_overlay', '<escape>'))
        self.hide_hotkey_var.set_hexpand(True)
        hide_box.append(self.hide_hotkey_var)
        hotkeys_box.append(hide_box)
        
        parent.append(hotkeys_frame)
        
    def _create_api_tab(self, parent):
        """Create the API settings tab using GTK4 widgets"""
        api_frame = Gtk.Frame()
        api_frame.set_label("API Settings")
        api_frame.set_margin_start(10)
        api_frame.set_margin_end(10)
        api_frame.set_margin_top(5)
        api_frame.set_margin_bottom(5)
        
        api_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=6)
        api_box.set_margin_start(10)
        api_box.set_margin_end(10)
        api_box.set_margin_top(10)
        api_box.set_margin_bottom(10)
        api_frame.set_child(api_box)
        
        # Rate limiting
        max_requests_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        max_requests_label = Gtk.Label(label="Max Requests:")
        max_requests_label.set_halign(Gtk.Align.START)
        max_requests_box.append(max_requests_label)
        
        self.max_requests_var = Gtk.Entry()
        self.max_requests_var.set_text(str(self.config.get('api.rate_limit_requests', 10)))
        self.max_requests_var.set_hexpand(True)
        max_requests_box.append(self.max_requests_var)
        api_box.append(max_requests_box)
        
        time_window_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        time_window_label = Gtk.Label(label="Time Window (seconds):")
        time_window_label.set_halign(Gtk.Align.START)
        time_window_box.append(time_window_label)
        
        self.time_window_var = Gtk.Entry()
        self.time_window_var.set_text(str(self.config.get('api.rate_limit_window', 60)))
        self.time_window_var.set_hexpand(True)
        time_window_box.append(self.time_window_var)
        api_box.append(time_window_box)
        
        # Cache settings
        cache_ttl_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
        cache_ttl_label = Gtk.Label(label="Cache TTL (seconds):")
        cache_ttl_label.set_halign(Gtk.Align.START)
        cache_ttl_box.append(cache_ttl_label)
        
        self.cache_ttl_var = Gtk.Entry()
        self.cache_ttl_var.set_text(str(self.config.get('api.cache_ttl', 300)))
        self.cache_ttl_var.set_hexpand(True)
        cache_ttl_box.append(self.cache_ttl_var)
        api_box.append(cache_ttl_box)
        
        parent.append(api_frame)
        
    def _save_settings(self, widget=None):
        """Save the current settings"""
        try:
            # Save window settings
            self.config.set('window.width', int(self.width_var.get_text()))
            self.config.set('window.height', int(self.height_var.get_text()))
            self.config.set('window.transparency', self.transparency_var.get_value())
            self.config.set('window.auto_show_on_poe2_start', self.auto_show_var.get_active())
            self.config.set('window.auto_hide_on_poe2_exit', self.auto_hide_var.get_active())
            
            # Save appearance settings
            # Note: For now, we'll use default values since DropDown handling is complex
            self.config.set('appearance.theme', 'dark')
            self.config.set('appearance.font_family', 'Arial')
            self.config.set('appearance.font_size', int(self.font_size_var.get_text()))
            
            # Save hotkey settings
            self.config.set('hotkeys.toggle_overlay', self.toggle_hotkey_var.get_text())
            self.config.set('hotkeys.quick_search', self.search_hotkey_var.get_text())
            self.config.set('hotkeys.hide_overlay', self.hide_hotkey_var.get_text())
            
            # Save API settings
            self.config.set('api.rate_limit_requests', int(self.max_requests_var.get_text()))
            self.config.set('api.rate_limit_window', int(self.time_window_var.get_text()))
            self.config.set('api.cache_ttl', int(self.cache_ttl_var.get_text()))
            
            logger.info("Settings saved successfully")
            
            # Show success message
            self._show_success_message("Settings saved successfully!")
            
            # Refresh UI with new values
            self._refresh_ui()
            
            # Call the save callback if provided
            if self.on_save_callback:
                try:
                    self.on_save_callback()
                except Exception as e:
                    logger.error(f"Error in save callback: {e}")
            
            self._on_close()
            
        except ValueError as e:
            logger.error(f"Invalid setting value: {e}")
            # Show error message to user
            self._show_error_message(f"Invalid setting value: {e}")
        except Exception as e:
            logger.error(f"Error saving settings: {e}")
            # Show error message to user
            self._show_error_message(f"Error saving settings: {e}")
            
    def _show_success_message(self, message: str):
        """Show a success message"""
        try:
            # Create info bar for success message
            info_bar = Gtk.InfoBar()
            info_bar.set_message_type(Gtk.MessageType.INFO)
            
            info_label = Gtk.Label(label=message)
            info_bar.add_child(info_label)
            
            # Add close button
            close_button = Gtk.Button(label="Close")
            close_button.connect("clicked", lambda btn: info_bar.hide())
            info_bar.add_action_widget(close_button, Gtk.ResponseType.CLOSE)
            
            # Add to dialog
            self.dialog.get_content_area().prepend(info_bar)
            info_bar.show()
            
            # Auto-hide after 3 seconds
            GLib.timeout_add_seconds(3, lambda: info_bar.hide())
            
        except Exception as e:
            logger.error(f"Error showing success message: {e}")
            
    def _show_error_message(self, message: str):
        """Show an error message"""
        try:
            # Create info bar for error message
            info_bar = Gtk.InfoBar()
            info_bar.set_message_type(Gtk.MessageType.ERROR)
            
            info_label = Gtk.Label(label=message)
            info_bar.add_child(info_label)
            
            # Add close button
            close_button = Gtk.Button(label="Close")
            close_button.connect("clicked", lambda btn: info_bar.hide())
            info_bar.add_action_widget(close_button, Gtk.ResponseType.CLOSE)
            
            # Add to dialog
            self.dialog.get_content_area().prepend(info_bar)
            info_bar.show()
            
            # Auto-hide after 5 seconds
            GLib.timeout_add_seconds(5, lambda: info_bar.hide())
            
        except Exception as e:
            logger.error(f"Error showing error message: {e}")
            
    def _reset_defaults(self, widget=None):
        """Reset settings to defaults"""
        self.config.reset_to_defaults()
        self._refresh_ui()
        logger.info("Settings reset to defaults")
        
    def _refresh_ui(self):
        """Refresh the UI with current config values"""
        # Refresh all variables with current config values
        self.width_var.set_text(str(self.config.get('window.width', 400)))
        self.height_var.set_text(str(self.config.get('window.height', 300)))
        self.transparency_var.set_value(self.config.get('window.transparency', 0.9))
        self.auto_show_var.set_active(self.config.get('window.auto_show_on_poe2_start', True))
        self.auto_hide_var.set_active(self.config.get('window.auto_hide_on_poe2_exit', False))
        
        # Appearance settings
        # Note: DropDown handling is complex, so we'll use defaults for now
        self.font_size_var.set_text(str(self.config.get('appearance.font_size', 10)))
        
        # Hotkey settings
        self.toggle_hotkey_var.set_text(self.config.get('hotkeys.toggle_overlay', '<ctrl>+<shift>+o'))
        self.search_hotkey_var.set_text(self.config.get('hotkeys.quick_search', '<ctrl>+<shift>+f'))
        self.hide_hotkey_var.set_text(self.config.get('hotkeys.hide_overlay', '<escape>'))
        
        # API settings
        self.max_requests_var.set_text(str(self.config.get('api.rate_limit_requests', 10)))
        self.time_window_var.set_text(str(self.config.get('api.rate_limit_window', 60)))
        self.cache_ttl_var.set_text(str(self.config.get('api.cache_ttl', 300)))
        
    def _on_response(self, dialog, response):
        """Handle dialog response"""
        if response == Gtk.ResponseType.CLOSE:
            self._on_close()
            
    def _on_close(self, widget=None):
        """Handle dialog close"""
        if self.dialog:
            self.dialog.destroy()
            self.dialog = None
        logger.info("Settings dialog closed")
