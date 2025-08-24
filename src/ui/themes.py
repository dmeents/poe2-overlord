"""
Theme Management for POE2 Master Overlay

Provides theming and appearance customization using GTK4.
"""

import gi
gi.require_version('Gtk', '4.0')
gi.require_version('Gdk', '4.0')

from gi.repository import Gtk, Gdk
from typing import Dict, Any, Optional
import logging

logger = logging.getLogger(__name__)


class ThemeManager:
    """Manages UI themes and appearance using GTK4"""
    
    def __init__(self):
        self.current_theme = "dark"
        self.themes = {
            "dark": {
                "background": "#2E2E2E",
                "foreground": "#FFFFFF",
                "accent": "#4CAF50",
                "secondary": "#424242",
                "text": "#E0E0E0",
                "border": "#555555"
            },
            "light": {
                "background": "#F5F5F5",
                "foreground": "#212121",
                "accent": "#2196F3",
                "secondary": "#E0E0E0",
                "text": "#424242",
                "border": "#BDBDBD"
            }
        }
        
    def get_theme(self, theme_name: str) -> Dict[str, str]:
        """Get a theme by name"""
        return self.themes.get(theme_name, self.themes["dark"])
        
    def apply_theme(self, widget: Gtk.Widget, theme_name: str):
        """Apply a theme to a GTK4 widget"""
        theme = self.get_theme(theme_name)
        
        try:
            # Apply theme colors using CSS classes
            css_provider = Gtk.CssProvider()
            css_data = f"""
            .theme-{theme_name} {{
                background-color: {theme["background"]};
                color: {theme["foreground"]};
                border-color: {theme["border"]};
            }}
            
            .theme-{theme_name} button {{
                background-color: {theme["accent"]};
                color: {theme["foreground"]};
            }}
            
            .theme-{theme_name} entry {{
                background-color: {theme["secondary"]};
                color: {theme["text"]};
                border-color: {theme["border"]};
            }}
            """
            
            css_provider.load_from_data(css_data.encode())
            
            # Apply CSS to the widget
            display = Gdk.Display.get_default()
            if display:
                screen = display.get_default_screen()
                Gtk.StyleContext.add_provider_for_screen(
                    screen,
                    css_provider,
                    Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
                )
            
            # Add theme class to widget
            widget.add_css_class(f"theme-{theme_name}")
            
            # Apply to child widgets recursively
            if hasattr(widget, 'get_first_child'):
                child = widget.get_first_child()
                while child:
                    self.apply_theme(child, theme_name)
                    child = child.get_next_sibling()
                    
        except Exception as e:
            logger.warning(f"Could not apply theme to widget: {e}")
            
    def set_current_theme(self, theme_name: str):
        """Set the current theme"""
        if theme_name in self.themes:
            self.current_theme = theme_name
            logger.info(f"Theme changed to: {theme_name}")
        else:
            logger.warning(f"Unknown theme: {theme_name}")
            
    def get_current_theme(self) -> str:
        """Get the current theme name"""
        return self.current_theme
        
    def get_color(self, color_name: str) -> str:
        """Get a color from the current theme"""
        theme = self.get_theme(self.current_theme)
        return theme.get(color_name, "#000000")
        
    def add_custom_theme(self, name: str, colors: Dict[str, str]):
        """Add a custom theme"""
        self.themes[name] = colors
        logger.info(f"Custom theme added: {name}")
        
    def list_themes(self) -> list:
        """List available themes"""
        return list(self.themes.keys())
        
    def export_theme(self, theme_name: str) -> Dict[str, str]:
        """Export a theme configuration"""
        return self.themes.get(theme_name, {}).copy()
        
    def import_theme(self, name: str, colors: Dict[str, str]):
        """Import a theme configuration"""
        self.themes[name] = colors
        logger.info(f"Theme imported: {name}")
        
    def apply_current_theme_to_widget(self, widget: Gtk.Widget):
        """Apply the current theme to a widget"""
        self.apply_theme(widget, self.current_theme)
        
    def get_css_for_theme(self, theme_name: str) -> str:
        """Get CSS string for a specific theme"""
        theme = self.get_theme(theme_name)
        return f"""
        .theme-{theme_name} {{
            background-color: {theme["background"]};
            color: {theme["foreground"]};
        }}
        
        .theme-{theme_name} button {{
            background-color: {theme["accent"]};
            color: {theme["foreground"]};
        }}
        
        .theme-{theme_name} entry {{
            background-color: {theme["secondary"]};
            color: {theme["text"]};
            border-color: {theme["border"]};
        }}
        """
