"""
Theme Management for POE2 Master Overlay

Provides theming and appearance customization.
"""

import tkinter as tk
from typing import Dict, Any, Optional
import logging

logger = logging.getLogger(__name__)


class ThemeManager:
    """Manages UI themes and appearance"""
    
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
        
    def apply_theme(self, widget: tk.Widget, theme_name: str):
        """Apply a theme to a widget"""
        theme = self.get_theme(theme_name)
        
        try:
            # Apply theme colors
            widget.configure(
                background=theme["background"],
                foreground=theme["foreground"]
            )
            
            # Apply to child widgets recursively
            for child in widget.winfo_children():
                self.apply_theme(child, theme_name)
                
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
