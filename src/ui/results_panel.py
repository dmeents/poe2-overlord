"""
Results Panel for POE2 Master Overlay

Displays search results and other information using GTK4.
"""

import gi
gi.require_version('Gtk', '4.0')
gi.require_version('Gdk', '4.0')
gi.require_version('Pango', '1.0')

from gi.repository import Gtk, Gdk, Pango
from typing import Dict, Any, List
import logging

from ..config.config_manager import ConfigManager

logger = logging.getLogger(__name__)


class ResultsPanel(Gtk.Box):
    """Results display panel using GTK4"""
    
    def __init__(self, config: ConfigManager, **kwargs):
        super().__init__(orientation=Gtk.Orientation.VERTICAL, spacing=6, **kwargs)
        
        self.config = config
        
        # Set margins
        self.set_margin_start(10)
        self.set_margin_end(10)
        self.set_margin_top(10)
        self.set_margin_bottom(10)
        
        self._setup_ui()
        
    def _setup_ui(self):
        """Setup the results interface using GTK4 widgets"""
        # Results label
        results_label = Gtk.Label(label="Results:")
        results_label.set_halign(Gtk.Align.START)
        self.append(results_label)
        
        # Create scrolled window for results
        scrolled_window = Gtk.ScrolledWindow()
        scrolled_window.set_vexpand(True)
        scrolled_window.set_min_content_height(200)
        
        # Create text view for results
        self.results_text = Gtk.TextView()
        self.results_text.set_editable(False)
        self.results_text.set_wrap_mode(Gtk.WrapMode.WORD_CHAR)
        
        # Set monospace font for better readability
        font_desc = Pango.FontDescription.from_string("Monospace 10")
        self.results_text.override_font(font_desc)
        
        # Add text view to scrolled window
        scrolled_window.set_child(self.results_text)
        
        # Add scrolled window to main container
        self.append(scrolled_window)
        
    def display_results(self, results: Dict[str, Any]):
        """Display search results"""
        # Get text buffer
        text_buffer = self.results_text.get_buffer()
        
        # Clear existing text
        text_buffer.set_text("")
        
        if 'error' in results:
            text_buffer.insert_at_cursor(f"❌ {results['error']}\n")
            return
            
        if not results.get('listings'):
            text_buffer.insert_at_cursor("No results found.\n")
            return
            
        # Display results
        text_buffer.insert_at_cursor(f"Found {len(results['listings'])} listings:\n\n")
        
        max_results = self.config.get('search.max_results', 10)
        for i, listing in enumerate(results['listings'][:max_results], 1):
            price = listing.get('price', 'N/A')
            currency = listing.get('currency', '')
            account = listing.get('account', 'Unknown')
            
            result_line = f"{i}. {price} {currency} - {account}\n"
            text_buffer.insert_at_cursor(result_line)
            
    def clear_results(self):
        """Clear the results display"""
        text_buffer = self.results_text.get_buffer()
        text_buffer.set_text("")
        
    def add_result(self, text: str):
        """Add a single result line"""
        text_buffer = self.results_text.get_buffer()
        text_buffer.insert_at_cursor(text + "\n")
        
    def get_results_text(self) -> str:
        """Get the current results text"""
        text_buffer = self.results_text.get_buffer()
        start_iter = text_buffer.get_start_iter()
        end_iter = text_buffer.get_end_iter()
        return text_buffer.get_text(start_iter, end_iter, True)
