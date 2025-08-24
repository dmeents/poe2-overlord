"""
Results Panel for POE2 Master Overlay

Displays search results and other information.
"""

import tkinter as tk
from tkinter import ttk
from typing import Dict, Any, List
import logging

from ..config.config_manager import ConfigManager

logger = logging.getLogger(__name__)


class ResultsPanel(ttk.LabelFrame):
    """Results display panel"""
    
    def __init__(self, parent, config: ConfigManager, **kwargs):
        super().__init__(parent, text="Results", padding="5", **kwargs)
        
        self.config = config
        self._setup_ui()
        
    def _setup_ui(self):
        """Setup the results interface"""
        # Results text widget with scrollbar
        text_frame = ttk.Frame(self)
        text_frame.pack(fill=tk.BOTH, expand=True)
        
        self.results_text = tk.Text(
            text_frame,
            height=8,
            wrap=tk.WORD,
            font=("Consolas", 10)
        )
        
        scrollbar = ttk.Scrollbar(text_frame, orient=tk.VERTICAL, command=self.results_text.yview)
        self.results_text.configure(yscrollcommand=scrollbar.set)
        
        self.results_text.pack(side=tk.LEFT, fill=tk.BOTH, expand=True)
        scrollbar.pack(side=tk.RIGHT, fill=tk.Y)
        
    def display_results(self, results: Dict[str, Any]):
        """Display search results"""
        self.results_text.delete(1.0, tk.END)
        
        if 'error' in results:
            self.results_text.insert(tk.END, f"❌ {results['error']}\n")
            return
            
        if not results.get('listings'):
            self.results_text.insert(tk.END, "No results found.\n")
            return
            
        # Display results
        self.results_text.insert(tk.END, f"Found {len(results['listings'])} listings:\n\n")
        
        max_results = self.config.get('search.max_results', 10)
        for i, listing in enumerate(results['listings'][:max_results], 1):
            price = listing.get('price', 'N/A')
            currency = listing.get('currency', '')
            account = listing.get('account', 'Unknown')
            
            result_line = f"{i}. {price} {currency} - {account}\n"
            self.results_text.insert(tk.END, result_line)
            
    def clear_results(self):
        """Clear the results display"""
        self.results_text.delete(1.0, tk.END)
        
    def add_result(self, text: str):
        """Add a single result line"""
        self.results_text.insert(tk.END, text + "\n")
        
    def get_results_text(self) -> str:
        """Get the current results text"""
        return self.results_text.get(1.0, tk.END)
