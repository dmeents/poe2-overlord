"""
Settings Dialog for POE2 Master Overlay

Provides a user interface for configuring overlay settings.
"""

import tkinter as tk
from tkinter import ttk
from typing import Optional
import logging

from ..config.config_manager import ConfigManager

logger = logging.getLogger(__name__)


class SettingsDialog:
    """Settings configuration dialog"""
    
    def __init__(self, parent, config: ConfigManager):
        """
        Initialize the settings dialog
        
        Args:
            parent: Parent window
            config: Configuration manager
        """
        self.parent = parent
        self.config = config
        self.dialog: Optional[tk.Toplevel] = None
        
    def show(self):
        """Show the settings dialog"""
        if self.dialog:
            self.dialog.lift()
            return
            
        self.dialog = tk.Toplevel(self.parent)
        self.dialog.title("POE2 Master Overlay - Settings")
        self.dialog.geometry("600x400")
        self.dialog.transient(self.parent)
        self.dialog.grab_set()
        
        self._setup_ui()
        self._center_dialog()
        
        # Handle dialog close
        self.dialog.protocol("WM_DELETE_WINDOW", self._on_close)
        
        logger.info("Settings dialog opened")
        
    def _setup_ui(self):
        """Setup the dialog interface"""
        # Create notebook for tabbed interface
        notebook = ttk.Notebook(self.dialog)
        notebook.pack(fill=tk.BOTH, expand=True, padx=10, pady=10)
        
        # General settings tab
        general_frame = ttk.Frame(notebook)
        notebook.add(general_frame, text="General")
        self._create_general_tab(general_frame)
        
        # Appearance tab
        appearance_frame = ttk.Frame(notebook)
        notebook.add(appearance_frame, text="Appearance")
        self._create_appearance_tab(appearance_frame)
        
        # Hotkeys tab
        hotkeys_frame = ttk.Frame(notebook)
        notebook.add(hotkeys_frame, text="Hotkeys")
        self._create_hotkeys_tab(hotkeys_frame)
        
        # API tab
        api_frame = ttk.Frame(notebook)
        notebook.add(api_frame, text="API")
        self._create_api_tab(api_frame)
        
        # Buttons
        button_frame = ttk.Frame(self.dialog)
        button_frame.pack(fill=tk.X, padx=10, pady=(0, 10))
        
        ttk.Button(button_frame, text="Save", command=self._save_settings).pack(side=tk.RIGHT, padx=(5, 0))
        ttk.Button(button_frame, text="Cancel", command=self._on_close).pack(side=tk.RIGHT)
        ttk.Button(button_frame, text="Reset to Defaults", command=self._reset_defaults).pack(side=tk.LEFT)
        
    def _create_general_tab(self, parent):
        """Create the general settings tab"""
        # Window settings
        window_frame = ttk.LabelFrame(parent, text="Window Settings", padding="10")
        window_frame.pack(fill=tk.X, padx=10, pady=5)
        
        # Width
        ttk.Label(window_frame, text="Width:").grid(row=0, column=0, sticky=tk.W, pady=2)
        self.width_var = tk.StringVar(value=str(self.config.get('window.width', 400)))
        ttk.Entry(window_frame, textvariable=self.width_var, width=10).grid(row=0, column=1, padx=(5, 0))
        
        # Height
        ttk.Label(window_frame, text="Height:").grid(row=1, column=0, sticky=tk.W, pady=2)
        self.height_var = tk.StringVar(value=str(self.config.get('window.height', 300)))
        ttk.Entry(window_frame, textvariable=self.height_var, width=10).grid(row=1, column=1, padx=(5, 0))
        
        # Transparency
        ttk.Label(window_frame, text="Transparency:").grid(row=2, column=0, sticky=tk.W, pady=2)
        self.transparency_var = tk.DoubleVar(value=self.config.get('window.transparency', 0.9))
        transparency_scale = ttk.Scale(window_frame, from_=0.1, to=1.0, variable=self.transparency_var, orient=tk.HORIZONTAL)
        transparency_scale.grid(row=2, column=1, padx=(5, 0), sticky=(tk.W, tk.E))
        
        # Auto-show/hide
        self.auto_show_var = tk.BooleanVar(value=self.config.get('window.auto_show_on_poe2_start', True))
        ttk.Checkbutton(window_frame, text="Auto-show when POE2 starts", variable=self.auto_show_var).grid(row=3, column=0, columnspan=2, sticky=tk.W, pady=2)
        
        self.auto_hide_var = tk.BooleanVar(value=self.config.get('window.auto_hide_on_poe2_exit', False))
        ttk.Checkbutton(window_frame, text="Auto-hide when POE2 exits", variable=self.auto_hide_var).grid(row=4, column=0, columnspan=2, sticky=tk.W, pady=2)
        
    def _create_appearance_tab(self, parent):
        """Create the appearance settings tab"""
        appearance_frame = ttk.LabelFrame(parent, text="Appearance", padding="10")
        appearance_frame.pack(fill=tk.X, padx=10, pady=5)
        
        # Theme
        ttk.Label(appearance_frame, text="Theme:").grid(row=0, column=0, sticky=tk.W, pady=2)
        self.theme_var = tk.StringVar(value=self.config.get('appearance.theme', 'dark'))
        theme_combo = ttk.Combobox(appearance_frame, textvariable=self.theme_var, values=['dark', 'light', 'auto'], state='readonly')
        theme_combo.grid(row=0, column=1, padx=(5, 0), sticky=(tk.W, tk.E))
        
        # Font settings
        ttk.Label(appearance_frame, text="Font Family:").grid(row=1, column=0, sticky=tk.W, pady=2)
        self.font_family_var = tk.StringVar(value=self.config.get('appearance.font_family', 'Arial'))
        font_combo = ttk.Combobox(appearance_frame, textvariable=self.font_family_var, values=['Arial', 'Helvetica', 'Times', 'Courier'], state='readonly')
        font_combo.grid(row=1, column=1, padx=(5, 0), sticky=(tk.W, tk.E))
        
        ttk.Label(appearance_frame, text="Font Size:").grid(row=2, column=0, sticky=tk.W, pady=2)
        self.font_size_var = tk.StringVar(value=str(self.config.get('appearance.font_size', 10)))
        ttk.Entry(appearance_frame, textvariable=self.font_size_var, width=10).grid(row=2, column=1, padx=(5, 0))
        
    def _create_hotkeys_tab(self, parent):
        """Create the hotkeys settings tab"""
        hotkeys_frame = ttk.LabelFrame(parent, text="Hotkeys", padding="10")
        hotkeys_frame.pack(fill=tk.X, padx=10, pady=5)
        
        # Toggle overlay
        ttk.Label(hotkeys_frame, text="Toggle Overlay:").grid(row=0, column=0, sticky=tk.W, pady=2)
        self.toggle_hotkey_var = tk.StringVar(value=self.config.get('hotkeys.toggle_overlay', '<ctrl>+<shift>+o'))
        ttk.Entry(hotkeys_frame, textvariable=self.toggle_hotkey_var, width=20).grid(row=0, column=1, padx=(5, 0))
        
        # Quick search
        ttk.Label(hotkeys_frame, text="Quick Search:").grid(row=1, column=0, sticky=tk.W, pady=2)
        self.search_hotkey_var = tk.StringVar(value=self.config.get('hotkeys.quick_search', '<ctrl>+<shift>+f'))
        ttk.Entry(hotkeys_frame, textvariable=self.search_hotkey_var, width=20).grid(row=1, column=1, padx=(5, 0))
        
        # Hide overlay
        ttk.Label(hotkeys_frame, text="Hide Overlay:").grid(row=2, column=0, sticky=tk.W, pady=2)
        self.hide_hotkey_var = tk.StringVar(value=self.config.get('hotkeys.hide_overlay', '<escape>'))
        ttk.Entry(hotkeys_frame, textvariable=self.hide_hotkey_var, width=20).grid(row=2, column=1, padx=(5, 0))
        
    def _create_api_tab(self, parent):
        """Create the API settings tab"""
        api_frame = ttk.LabelFrame(parent, text="API Settings", padding="10")
        api_frame.pack(fill=tk.X, padx=10, pady=5)
        
        # Rate limiting
        ttk.Label(api_frame, text="Max Requests:").grid(row=0, column=0, sticky=tk.W, pady=2)
        self.max_requests_var = tk.StringVar(value=str(self.config.get('api.rate_limit_requests', 10)))
        ttk.Entry(api_frame, textvariable=self.max_requests_var, width=10).grid(row=0, column=1, padx=(5, 0))
        
        ttk.Label(api_frame, text="Time Window (seconds):").grid(row=1, column=0, sticky=tk.W, pady=2)
        self.time_window_var = tk.StringVar(value=str(self.config.get('api.rate_limit_window', 60)))
        ttk.Entry(api_frame, textvariable=self.time_window_var, width=10).grid(row=1, column=1, padx=(5, 0))
        
        # Cache settings
        ttk.Label(api_frame, text="Cache TTL (seconds):").grid(row=2, column=0, sticky=tk.W, pady=2)
        self.cache_ttl_var = tk.StringVar(value=str(self.config.get('api.cache_ttl', 300)))
        ttk.Entry(api_frame, textvariable=self.cache_ttl_var, width=10).grid(row=2, column=1, padx=(5, 0))
        
    def _center_dialog(self):
        """Center the dialog on screen"""
        self.dialog.update_idletasks()
        x = (self.dialog.winfo_screenwidth() // 2) - (self.dialog.winfo_width() // 2)
        y = (self.dialog.winfo_screenheight() // 2) - (self.dialog.winfo_height() // 2)
        self.dialog.geometry(f"+{x}+{y}")
        
    def _save_settings(self):
        """Save the current settings"""
        try:
            # Save window settings
            self.config.set('window.width', int(self.width_var.get()))
            self.config.set('window.height', int(self.height_var.get()))
            self.config.set('window.transparency', self.transparency_var.get())
            self.config.set('window.auto_show_on_poe2_start', self.auto_show_var.get())
            self.config.set('window.auto_hide_on_poe2_exit', self.auto_hide_var.get())
            
            # Save appearance settings
            self.config.set('appearance.theme', self.theme_var.get())
            self.config.set('appearance.font_family', self.font_family_var.get())
            self.config.set('appearance.font_size', int(self.font_size_var.get()))
            
            # Save hotkey settings
            self.config.set('hotkeys.toggle_overlay', self.toggle_hotkey_var.get())
            self.config.set('hotkeys.quick_search', self.search_hotkey_var.get())
            self.config.set('hotkeys.hide_overlay', self.hide_hotkey_var.get())
            
            # Save API settings
            self.config.set('api.rate_limit_requests', int(self.max_requests_var.get()))
            self.config.set('api.rate_limit_window', int(self.time_window_var.get()))
            self.config.set('api.cache_ttl', int(self.cache_ttl_var.get()))
            
            logger.info("Settings saved successfully")
            self._on_close()
            
        except ValueError as e:
            logger.error(f"Invalid setting value: {e}")
            # TODO: Show error dialog to user
            
    def _reset_defaults(self):
        """Reset settings to defaults"""
        self.config.reset_to_defaults()
        self._refresh_ui()
        logger.info("Settings reset to defaults")
        
    def _refresh_ui(self):
        """Refresh the UI with current config values"""
        # Refresh all variables with current config values
        self.width_var.set(str(self.config.get('window.width', 400)))
        self.height_var.set(str(self.config.get('window.height', 300)))
        self.transparency_var.set(self.config.get('window.transparency', 0.9))
        # ... refresh other variables
        
    def _on_close(self):
        """Handle dialog close"""
        if self.dialog:
            self.dialog.destroy()
            self.dialog = None
        logger.info("Settings dialog closed")
