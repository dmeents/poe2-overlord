"""
Search Panel for POE2 Master Overlay

Handles item search functionality and interface.
"""

import tkinter as tk
from tkinter import ttk
from typing import Optional, Callable
import logging

from ..config.config_manager import ConfigManager

logger = logging.getLogger(__name__)


class SearchPanel(ttk.LabelFrame):
    """Search panel for item lookup"""
    
    def __init__(self, parent, config: ConfigManager, **kwargs):
        super().__init__(parent, text="Item Search", padding="5", **kwargs)
        
        self.config = config
        self.search_callback: Optional[Callable] = None
        
        self._setup_ui()
        
    def _setup_ui(self):
        """Setup the search interface"""
        # Search entry
        ttk.Label(self, text="Item Name:").pack(anchor=tk.W)
        self.search_entry = ttk.Entry(self, width=40)
        self.search_entry.pack(fill=tk.X, pady=(2, 5))
        self.search_entry.bind("<Return>", self._on_search)
        
        # Search button
        self.search_btn = ttk.Button(
            self,
            text="Search Prices",
            command=self._on_search
        )
        self.search_btn.pack(pady=(0, 5))
        
    def _on_search(self, event=None):
        """Handle search requests"""
        item_name = self.search_entry.get().strip()
        if item_name:
            logger.info(f"Search requested for: {item_name}")
            if self.search_callback:
                self.search_callback(item_name)
                
    def set_search_callback(self, callback: Callable):
        """Set the search callback function"""
        self.search_callback = callback
        
    def clear_search(self):
        """Clear the search entry"""
        self.search_entry.delete(0, tk.END)
        
    def get_search_text(self) -> str:
        """Get the current search text"""
        return self.search_entry.get().strip()
