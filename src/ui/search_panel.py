"""
Search Panel for POE2 Master Overlay

Handles item search functionality and interface using GTK4.
"""

import gi
gi.require_version('Gtk', '4.0')
gi.require_version('Gdk', '4.0')

from gi.repository import Gtk, Gdk
from typing import Optional, Callable
import logging

from ..config.config_manager import ConfigManager

logger = logging.getLogger(__name__)


class SearchPanel(Gtk.Box):
    """Search panel for item lookup using GTK4"""
    
    def __init__(self, config: ConfigManager, **kwargs):
        super().__init__(orientation=Gtk.Orientation.VERTICAL, spacing=6, **kwargs)
        
        self.config = config
        self.search_callback: Optional[Callable] = None
        
        # Set margins
        self.set_margin_start(10)
        self.set_margin_end(10)
        self.set_margin_top(10)
        self.set_margin_bottom(10)
        
        self._setup_ui()
        
    def _setup_ui(self):
        """Setup the search interface using GTK4 widgets"""
        # Search label
        search_label = Gtk.Label(label="Item Name:")
        search_label.set_halign(Gtk.Align.START)
        self.append(search_label)
        
        # Search entry
        self.search_entry = Gtk.Entry()
        self.search_entry.set_hexpand(True)
        self.search_entry.set_margin_bottom(5)
        self.search_entry.connect("activate", self._on_search)
        self.append(self.search_entry)
        
        # Search button
        self.search_btn = Gtk.Button(label="Search Prices")
        self.search_btn.connect("clicked", self._on_search)
        self.search_btn.set_margin_bottom(5)
        self.append(self.search_btn)
        
    def _on_search(self, widget=None):
        """Handle search requests"""
        item_name = self.search_entry.get_text().strip()
        if item_name:
            logger.info(f"Search requested for: {item_name}")
            if self.search_callback:
                self.search_callback(item_name)
                
    def set_search_callback(self, callback: Callable):
        """Set the search callback function"""
        self.search_callback = callback
        
    def clear_search(self):
        """Clear the search entry"""
        self.search_entry.set_text("")
        
    def get_search_text(self) -> str:
        """Get the current search text"""
        return self.search_entry.get_text().strip()
