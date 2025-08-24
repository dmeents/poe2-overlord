#!/usr/bin/env python3
"""
Main Window for POE2 Master Overlay using GTK4

This module provides the main overlay window with proper Wayland support,
draggable functionality, and modern GTK4 UI components.
"""

import gi
gi.require_version('Gtk', '4.0')
gi.require_version('Gdk', '4.0')
gi.require_version('Gio', '2.0')

from gi.repository import Gtk, Gdk, Gio, GLib
from typing import Optional
import logging
import os

from ..core.event_bus import EventType, event_bus


class MainWindow(Gtk.ApplicationWindow):
    """Main overlay window using GTK4 for proper Wayland support"""
    
    def __init__(self, config, *args, **kwargs):
        super().__init__(*args, **kwargs)
        
        self.config = config
        self.logger = logging.getLogger(__name__)
        
        # Window state
        self.is_dragging = False
        self.drag_start_x = 0
        self.drag_start_y = 0
        self.drag_start_window_x = 0
        self.drag_start_window_y = 0
        
        # Setup the window
        self._setup_window()
        self._setup_ui()
        self._setup_drag_events()
        self._setup_keyboard_shortcuts()
        self._restore_window_position()
        
        # Connect to event bus
        event_bus.subscribe(EventType.OVERLAY_TOGGLE, self._on_overlay_toggle)
        
        self.logger.info("GTK4 Main window initialized successfully")
        
    def _setup_window(self):
        """Configure window properties for overlay behavior"""
        try:
            # Detect display server
            wayland_display = os.environ.get('WAYLAND_DISPLAY')
            xdg_desktop = os.environ.get('XDG_CURRENT_DESKTOP', '')
            
            self.logger.debug(f"Display server: {'Wayland' if wayland_display else 'X11'}")
            self.logger.debug(f"Desktop environment: {xdg_desktop}")
            
            # Set window properties
            self.set_title("POE2 Master Overlay")
            self.set_default_size(400, 300)
            
            # Configure for overlay behavior
            if wayland_display:
                self.logger.debug("Wayland detected - using GTK4 overlay configuration")
                self._configure_wayland_window()
            else:
                self.logger.debug("X11 detected - using GTK4 overlay configuration")
                self._configure_x11_window()
                
            # Always on top (GTK4 method)
            self.set_keep_above(True)
            
            # Skip taskbar (GTK4 method)
            self.set_skip_taskbar_hint(True)
            
            # Set window type hint for overlay (GTK4 method)
            self.set_type_hint(Gdk.WindowTypeHint.UTILITY)
            
        except Exception as e:
            self.logger.error(f"Error setting up GTK4 window: {e}")
            
    def _configure_wayland_window(self):
        """Configure window specifically for Wayland"""
        try:
            # Wayland-friendly settings
            self.set_decorated(False)  # No decorations for overlay
            self.set_resizable(False)  # Fixed size overlay
            
            # Set window class for Wayland (GTK4 method)
            self.set_wmclass("poe2-overlay", "POE2 Master Overlay")
            
        except Exception as e:
            self.logger.error(f"Error configuring for Wayland: {e}")
            
    def _configure_x11_window(self):
        """Configure window specifically for X11"""
        try:
            # X11-friendly settings
            self.set_decorated(False)
            self.set_resizable(False)
            
        except Exception as e:
            self.logger.error(f"Error configuring for X11: {e}")
            
    def _setup_ui(self):
        """Create the main UI components"""
        try:
            # Main container
            self.main_box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=6)
            self.main_box.set_margin_start(10)
            self.main_box.set_margin_end(10)
            self.main_box.set_margin_top(10)
            self.main_box.set_margin_bottom(10)
            
            # Title section
            self._create_title()
            
            # Control buttons
            self._create_control_buttons()
            
            # Search panel
            self._create_search_panel()
            
            # Results panel
            self._create_results_panel()
            
            # Status bar
            self._create_status_bar()
            
            # Set the main container
            self.set_child(self.main_box)
            
        except Exception as e:
            self.logger.error(f"Error setting up GTK4 UI: {e}")
            
    def _create_title(self):
        """Create the title section with drag indicator"""
        try:
            # Title box
            title_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
            
            # Main title
            title_label = Gtk.Label(label="POE2 Master Overlay")
            title_label.set_hexpand(True)
            title_label.set_halign(Gtk.Align.START)
            
            # Drag indicator
            drag_label = Gtk.Label(label="⋮⋮")
            drag_label.set_tooltip_text("Drag to move overlay")
            drag_label.set_css_classes(["drag-indicator"])
            
            # Add to title box
            title_box.append(title_label)
            title_box.append(drag_label)
            
            # Add to main box
            self.main_box.append(title_box)
            
            # Store references
            self.title_label = title_label
            self.drag_label = drag_label
            
        except Exception as e:
            self.logger.error(f"Error creating title: {e}")
            
    def _create_control_buttons(self):
        """Create control buttons"""
        try:
            button_box = Gtk.Box(orientation=Gtk.Orientation.HORIZONTAL, spacing=6)
            
            # Settings button
            settings_button = Gtk.Button(label="Settings")
            settings_button.connect("clicked", self._on_settings_clicked)
            settings_button.set_css_classes(["settings-button"])
            button_box.append(settings_button)
            
            # Reset position button
            reset_button = Gtk.Button(label="Reset Position")
            reset_button.connect("clicked", self._on_reset_position_clicked)
            reset_button.set_css_classes(["reset-button"])
            button_box.append(reset_button)
            
            # Add to main box
            self.main_box.append(button_box)
            
        except Exception as e:
            self.logger.error(f"Error creating control buttons: {e}")
            
    def _create_search_panel(self):
        """Create search panel using GTK4 components"""
        try:
            from .search_panel import SearchPanel
            self.search_panel = SearchPanel(self.config)
            self.main_box.append(self.search_panel)
        except Exception as e:
            self.logger.error(f"Error creating search panel: {e}")
            # Fallback to label
            search_label = Gtk.Label(label="Search Panel")
            search_label.set_css_classes(["search-panel"])
            self.main_box.append(search_label)
            
    def _create_results_panel(self):
        """Create results panel using GTK4 components"""
        try:
            from .results_panel import ResultsPanel
            self.results_panel = ResultsPanel(self.config)
            self.main_box.append(self.results_panel)
            
        except Exception as e:
            self.logger.error(f"Error creating results panel: {e}")
            # Fallback to label
            results_label = Gtk.Label(label="📊 Results Panel (GTK4)")
            results_label.set_css_classes(["results-panel"])
            self.main_box.append(results_label)
            
    def _create_status_bar(self):
        """Create status bar"""
        try:
            self.status_label = Gtk.Label(label="Status: Ready")
            self.status_label.set_css_classes(["status-bar"])
            self.main_box.append(self.status_label)
            
        except Exception as e:
            self.logger.error(f"Error creating status bar: {e}")
            
    def _setup_drag_events(self):
        """Setup mouse drag events for moving the window"""
        try:
            # Create event controllers for GTK4
            click_controller = Gtk.GestureClick()
            click_controller.connect("pressed", self._on_button_press)
            click_controller.connect("released", self._on_button_release)
            
            motion_controller = Gtk.EventControllerMotion()
            motion_controller.connect("motion", self._on_motion)
            
            # Add controllers to the window
            self.add_controller(click_controller)
            self.add_controller(motion_controller)
            
        except Exception as e:
            self.logger.error(f"Error setting up drag events: {e}")
            
    def _on_button_press(self, gesture, n_press, x, y):
        """Handle mouse button press for dragging"""
        try:
            # Get the button number (1 = left, 2 = middle, 3 = right)
            button = gesture.get_current_button()
            
            if button == 1:  # Left mouse button
                self.is_dragging = True
                
                # Get current window position using GTK4 API
                current_x, current_y = self._get_window_position()
                self.drag_start_window_x = current_x
                self.drag_start_window_y = current_y
                
                # Get pointer position relative to window
                self.drag_start_x = x
                self.drag_start_y = y
                
                # Change cursor to indicate dragging
                self.get_native().get_surface().set_cursor(Gdk.Cursor.new_from_name("move"))
                
        except Exception as e:
            self.logger.error(f"Error in button press: {e}")
            
    def _on_motion(self, controller, x, y):
        """Handle mouse motion for dragging"""
        try:
            if self.is_dragging:
                # Calculate new position
                delta_x = x - self.drag_start_x
                delta_y = y - self.drag_start_y
                
                new_x = self.drag_start_window_x + delta_x
                new_y = self.drag_start_window_y + delta_y
                
                # Move window using GTK4 method
                self._set_window_position(new_x, new_y)
                
        except Exception as e:
            self.logger.error(f"Error in motion: {e}")
            
    def _on_button_release(self, gesture, n_press, x, y):
        """Handle mouse button release"""
        try:
            if self.is_dragging:
                self.is_dragging = False
                
                # Reset cursor
                self.get_native().get_surface().set_cursor(None)
                
                # Save new position
                self._save_window_position()
                
        except Exception as e:
            self.logger.error(f"Error in button release: {e}")
            
    def _get_window_position(self):
        """Get current window position using GTK4 API"""
        try:
            # Method 1: Try to get position from native window
            native = self.get_native()
            if native:
                surface = native.get_surface()
                if surface:
                    try:
                        # This might not work on Wayland
                        position = surface.get_position()
                        return position
                    except Exception as e:
                        self.logger.debug(f"Surface.get_position() failed: {e}")
                else:
                    self.logger.debug("No surface available")
            else:
                self.logger.debug("No native window available")
                
            # Method 2: Try alternative approach
            try:
                # Get allocation (position relative to parent)
                allocation = self.get_allocation()
                return allocation.x, allocation.y
            except Exception as e:
                self.logger.debug(f"get_allocation() failed: {e}")
                
            # Fallback to default position
            self.logger.debug("Using fallback position (100, 100)")
            return 100, 100
            
        except Exception as e:
            self.logger.error(f"Error getting window position: {e}")
            return 100, 100
            
    def _set_window_position(self, x, y):
        """Set window position using GTK4 API"""
        try:
            # Method 1: Try to move the native window surface
            native = self.get_native()
            if native:
                surface = native.get_surface()
                if surface:
                    try:
                        surface.move(int(x), int(y))
                        return
                    except Exception as e:
                        self.logger.debug(f"Surface.move() failed: {e}")
                else:
                    self.logger.debug("No surface available")
            else:
                self.logger.debug("No native window available")
                
            # Method 2: Try using window manager hints (may not work on Wayland)
            try:
                self.set_default_size(400, 300)  # Ensure size is set
                self.logger.debug("Window positioning may not work on Wayland due to security restrictions")
            except Exception as e:
                self.logger.debug(f"Window manager approach failed: {e}")
                
        except Exception as e:
            self.logger.error(f"Error setting window position: {e}")
            
    def _setup_keyboard_shortcuts(self):
        """Setup keyboard shortcuts for window movement"""
        try:
            # Create keyboard controller
            key_controller = Gtk.EventControllerKey()
            key_controller.connect("key-pressed", self._on_key_pressed)
            self.add_controller(key_controller)
            
        except Exception as e:
            self.logger.error(f"Error setting up keyboard shortcuts: {e}")
            
    def _on_key_pressed(self, controller, keyval, keycode, state):
        """Handle keyboard shortcuts"""
        try:
            # Check for Ctrl+Shift combinations
            if state & Gdk.ModifierType.CONTROL_MASK and state & Gdk.ModifierType.SHIFT_MASK:
                if keyval == Gdk.KEY_Left:
                    self._move_window(-10, 0)
                elif keyval == Gdk.KEY_Right:
                    self._move_window(10, 0)
                elif keyval == Gdk.KEY_Up:
                    self._move_window(0, -10)
                elif keyval == Gdk.KEY_Down:
                    self._move_window(0, 10)
                elif keyval == Gdk.KEY_Home:
                    self._reset_window_position()
                    
        except Exception as e:
            self.logger.error(f"Error in key press: {e}")
            
    def _move_window(self, delta_x, delta_y):
        """Move window by specified delta"""
        try:
            current_x, current_y = self._get_window_position()
            new_x = current_x + delta_x
            new_y = current_y + delta_y
            
            # Use GTK4 move method
            self._set_window_position(new_x, new_y)
            self._save_window_position()
            
        except Exception as e:
            self.logger.error(f"Error moving window: {e}")
            
    def _reset_window_position(self):
        """Reset window to default position"""
        try:
            # Get screen dimensions using GTK4 API
            display = Gdk.Display.get_default()
            if display:
                monitor = display.get_monitor_at_surface(self.get_native().get_surface())
                if monitor:
                    geometry = monitor.get_geometry()
                    
                    # Calculate default position (top-right)
                    window_width, window_height = self.get_default_size()
                    default_x = geometry.x + geometry.width - window_width - 20
                    default_y = geometry.y + 20
                    
                    # Use GTK4 move method
                    self._set_window_position(default_x, default_y)
                    self._save_window_position()
                    
                else:
                    self.logger.debug("Could not get monitor geometry, using fallback position")
                    self._set_window_position(100, 100)
            else:
                self.logger.debug("Could not get display, using fallback position")
                self._set_window_position(100, 100)
                
        except Exception as e:
            self.logger.error(f"Error resetting position: {e}")
            # Fallback to safe position
            try:
                self._set_window_position(100, 100)
            except:
                pass
            
    def _save_window_position(self):
        """Save current window position to config"""
        try:
            x, y = self._get_window_position()
            self.config.set('window.x_position', x)
            self.config.set('window.y_position', y)
            self.config.save()
            
        except Exception as e:
            self.logger.error(f"Error saving position: {e}")
            
    def _restore_window_position(self):
        """Restore window position from config"""
        try:
            x = self.config.get('window.x_position')
            y = self.config.get('window.y_position')
            
            if x is not None and y is not None:
                # Use GTK4 move method
                self._set_window_position(x, y)
            else:
                self._reset_window_position()
                
        except Exception as e:
            self.logger.error(f"Error restoring position: {e}")
            self._reset_window_position()
            
    def _on_settings_clicked(self, button):
        """Handle settings button click"""
        self.logger.info("GTK4 settings button clicked")
        
        try:
            from .settings_dialog import SettingsDialog
            settings_dialog = SettingsDialog(self, self.config)
            settings_dialog.show()
        except Exception as e:
            self.logger.error(f"Error showing settings dialog: {e}")
        
        if hasattr(self, 'status_label'):
            self.status_label.set_text("Settings Clicked! ✓")
            GLib.timeout_add_seconds(2, lambda: self.status_label.set_text("Status: Ready"))
            
    def _show_settings(self):
        """Show the settings dialog"""
        try:
            from .settings_dialog import SettingsDialog
            settings_dialog = SettingsDialog(self, self.config)
            settings_dialog.show()
        except Exception as e:
            self.logger.error(f"Error showing settings dialog: {e}")
            
    def _on_reset_position_clicked(self, button):
        """Handle reset position button click"""
        self.logger.info("GTK4 reset position button clicked")
        
        self._reset_window_position()
        
        if hasattr(self, 'status_label'):
            self.status_label.set_text("Position Reset! ✓")
            GLib.timeout_add_seconds(2, lambda: self.status_label.set_text("Status: Ready"))
            
    def _on_overlay_toggle(self, data):
        """Handle overlay toggle event"""
        try:
            if data.get('visible', False):
                self.show()
            else:
                self.hide()
                
        except Exception as e:
            self.logger.error(f"Error in overlay toggle: {e}")
            
    def show_overlay(self):
        """Show the overlay window"""
        try:
            self.show()
            self.present()
            
        except Exception as e:
            self.logger.error(f"Error showing overlay: {e}")
            
    def hide_overlay(self):
        """Hide the overlay window"""
        try:
            self.hide()
            
        except Exception as e:
            self.logger.error(f"Error hiding overlay: {e}")
