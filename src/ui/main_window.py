"""
Main Window for POE2 Master Overlay

The primary overlay window that contains all UI components.
"""

import tkinter as tk
from tkinter import ttk
from typing import Optional
import logging

from ..core.event_bus import EventType, event_bus
from ..config.config_manager import ConfigManager
from ..core.overlay_manager import OverlayManager

logger = logging.getLogger(__name__)


class MainWindow:
    """Main overlay window"""
    
    def __init__(self, config: ConfigManager, overlay_manager: OverlayManager):
        """
        Initialize the main window
        
        Args:
            config: Configuration manager
            overlay_manager: Main overlay manager
        """
        self.config = config
        self.overlay_manager = overlay_manager
        self.root: Optional[tk.Tk] = None
        self.is_visible = False
        
        # UI Components
        self.main_frame: Optional[ttk.Frame] = None
        self.status_label: Optional[ttk.Label] = None
        self.search_panel: Optional['SearchPanel'] = None
        self.results_panel: Optional['ResultsPanel'] = None
        
        # Setup UI
        self._setup_ui()
        self._setup_event_handlers()
        
        logger.info("Main window initialized")
        
    def _setup_ui(self):
        """Initialize the overlay UI"""
        try:
            self.root = tk.Tk()
            self.root.title("POE2 Master Overlay")
            
            # Configure overlay window properties
            self._configure_window_properties()
            
            # Create main frame
            self.main_frame = ttk.Frame(self.root, padding="10")
            self.main_frame.pack(fill=tk.BOTH, expand=True)
            
            # Create UI components
            self._create_title()
            self._create_status_display()
            self._create_search_panel()
            self._create_results_panel()
            self._create_control_buttons()
            
            # Ensure overlay is visible and on top
            self.root.deiconify()
            self.root.lift()
            self.root.focus_force()
            self.is_visible = True
            
            # Force update and redraw
            self.root.update()
            self.root.update_idletasks()
            
            logger.info("Main window UI setup completed successfully")
            
        except Exception as e:
            logger.error(f"Failed to setup UI: {e}")
            if hasattr(self, 'root') and self.root:
                self.root.destroy()
            raise
        
    def _configure_window_properties(self):
        """Configure window properties for overlay behavior"""
        try:
            # Remove window decorations
            self.root.overrideredirect(True)
            
            # Always on top
            self.root.wm_attributes("-topmost", True)
            
            # Transparency
            transparency = self.config.get('window.transparency', 0.9)
            self.root.wm_attributes("-alpha", transparency)
            
            # Size and position
            width = self.config.get('window.width', 400)
            height = self.config.get('window.height', 300)
            self.root.geometry(f"{width}x{height}")
            
            # Position overlay in top-right corner with fallback
            try:
                # Wait a moment for screen info to be available
                self.root.update_idletasks()
                
                screen_width = self.root.winfo_screenwidth()
                screen_height = self.root.winfo_screenheight()
                
                # Use default position if screen dimensions are invalid
                if screen_width <= 0 or screen_height <= 0:
                    logger.warning("Invalid screen dimensions, using default position")
                    x_position = 100
                    y_position = 100
                else:
                    x_position = self.config.get('window.x_position', screen_width - width - 20)
                    y_position = self.config.get('window.y_position', 20)
                    
                    # Ensure window is on screen
                    if x_position < 0:
                        x_position = 20
                    if y_position < 0:
                        y_position = 20
                    if x_position + width > screen_width:
                        x_position = screen_width - width - 20
                    if y_position + height > screen_height:
                        y_position = screen_height - height - 20
                
                self.root.geometry(f"{width}x{height}+{x_position}+{y_position}")
                
            except Exception as e:
                logger.warning(f"Could not set window position: {e}, using default")
                # Fallback to center of screen
                self.root.geometry(f"{width}x{height}+100+100")
                x_position, y_position = 100, 100
            
            logger.debug(f"Window configured: {width}x{height} at ({x_position}, {y_position})")
            
        except Exception as e:
            logger.error(f"Error configuring window properties: {e}")
            # Fallback to basic configuration
            self.root.geometry("400x300+100+100")
        
    def _create_title(self):
        """Create the title section"""
        title_label = ttk.Label(
            self.main_frame, 
            text="POE2 Master Overlay",
            font=("Arial", 14, "bold")
        )
        title_label.pack(pady=(0, 10))
        
    def _create_status_display(self):
        """Create the status display section"""
        self.status_label = ttk.Label(
            self.main_frame,
            text="Status: Initializing...",
            foreground="orange"
        )
        self.status_label.pack(pady=(0, 10))
        
    def _create_search_panel(self):
        """Create the search panel"""
        try:
            from .search_panel import SearchPanel
            self.search_panel = SearchPanel(self.main_frame, self.config)
            self.search_panel.pack(fill=tk.X, pady=(0, 10))
        except ImportError:
            # Fallback to basic search if panel not available
            self._create_basic_search()
            
    def _create_basic_search(self):
        """Create a basic search interface as fallback"""
        search_frame = ttk.LabelFrame(self.main_frame, text="Item Search", padding="5")
        search_frame.pack(fill=tk.X, pady=(0, 10))
        
        ttk.Label(search_frame, text="Item Name:").pack(anchor=tk.W)
        self.search_entry = ttk.Entry(search_frame, width=40)
        self.search_entry.pack(fill=tk.X, pady=(2, 5))
        self.search_entry.bind("<Return>", self._on_search)
        
        search_btn = ttk.Button(
            search_frame,
            text="Search Prices",
            command=self._on_search
        )
        search_btn.pack(pady=(0, 5))
        
    def _create_results_panel(self):
        """Create the results panel"""
        try:
            from .results_panel import ResultsPanel
            self.results_panel = ResultsPanel(self.main_frame, self.config)
            self.results_panel.pack(fill=tk.BOTH, expand=True)
        except ImportError:
            # Fallback to basic results if panel not available
            self._create_basic_results()
            
    def _create_basic_results(self):
        """Create a basic results interface as fallback"""
        results_frame = ttk.LabelFrame(self.main_frame, text="Results", padding="5")
        results_frame.pack(fill=tk.BOTH, expand=True)
        
        text_frame = ttk.Frame(results_frame)
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
        
    def _create_control_buttons(self):
        """Create control buttons"""
        control_frame = ttk.Frame(self.main_frame)
        control_frame.pack(fill=tk.X, pady=(10, 0))
        
        # Toggle button - text changes based on always_visible setting
        always_visible = self.config.get('window.always_visible', True)
        toggle_text = "Always Visible" if always_visible else "Hide Overlay"
        toggle_btn = ttk.Button(
            control_frame,
            text=toggle_text,
            command=self._toggle_overlay
        )
        toggle_btn.pack(side=tk.LEFT, padx=(0, 5))
        
        # Settings button
        settings_btn = ttk.Button(
            control_frame,
            text="Settings",
            command=self._show_settings
        )
        settings_btn.pack(side=tk.LEFT, padx=(0, 5))
        
        # Exit button
        exit_btn = ttk.Button(
            control_frame,
            text="Exit",
            command=self._quit_application
        )
        exit_btn.pack(side=tk.RIGHT)
        
    def _setup_event_handlers(self):
        """Setup event handlers"""
        # Subscribe to events
        event_bus.subscribe(EventType.POE2_STARTED, self._on_poe2_started)
        event_bus.subscribe(EventType.POE2_STOPPED, self._on_poe2_stopped)
        event_bus.subscribe(EventType.OVERLAY_SHOW, self._on_overlay_show)
        event_bus.subscribe(EventType.OVERLAY_HIDE, self._on_overlay_hide)
        event_bus.subscribe(EventType.OVERLAY_TOGGLE, self._on_overlay_toggle)
        
    def _on_search(self, event=None):
        """Handle search requests"""
        if hasattr(self, 'search_entry'):
            item_name = self.search_entry.get().strip()
            if item_name:
                logger.info(f"Search requested for: {item_name}")
                # TODO: Implement search functionality
                self._display_search_results(f"Search results for: {item_name}")
                
    def _display_search_results(self, results: str):
        """Display search results"""
        if hasattr(self, 'results_text'):
            self.results_text.delete(1.0, tk.END)
            self.results_text.insert(tk.END, results)
            
    def _toggle_overlay(self):
        """Toggle overlay visibility"""
        # If always_visible is enabled, only allow showing
        if self.config.get('window.always_visible', True):
            if not self.is_visible:
                self.show()
            # Don't allow hiding when always_visible is enabled
            return
            
        if self.is_visible:
            self.hide()
        else:
            self.show()
            
    def _show_settings(self):
        """Show settings dialog"""
        logger.info("Settings dialog requested")
        # TODO: Implement settings dialog
        
    def _quit_application(self):
        """Quit the application"""
        logger.info("Application quit requested")
        if self.root:
            self.root.quit()
            
    def _on_poe2_started(self, event):
        """Handle POE2 started event"""
        self._update_status("POE2 Detected ✓", "green")
        
    def _on_poe2_stopped(self, event):
        """Handle POE2 stopped event"""
        self._update_status("POE2 Not Running", "red")
        # Keep overlay visible even when POE2 stops
        # self.hide()  # Commented out to keep overlay always visible
        
    def _on_overlay_show(self, event):
        """Handle overlay show event"""
        self.show()
        
    def _on_overlay_hide(self, event):
        """Handle overlay hide event"""
        self.hide()
        
    def _on_overlay_toggle(self, event):
        """Handle overlay toggle event"""
        self._toggle_overlay()
        
    def _update_status(self, text: str, color: str):
        """Update the status label"""
        if self.status_label:
            self.status_label.config(text=f"Status: {text}", foreground=color)
            
    def show(self):
        """Show the overlay"""
        try:
            if not self.is_visible and self.root:
                self.root.deiconify()
                self.root.lift()
                self.root.focus_force()
                self.is_visible = True
                
                # Force update and redraw
                self.root.update()
                self.root.update_idletasks()
                
                logger.debug("Overlay shown successfully")
                
            # Ensure overlay stays visible if configured
            if self.config.get('window.always_visible', True):
                self.root.after(100, self._ensure_visible)
                
        except Exception as e:
            logger.error(f"Error showing overlay: {e}")
            
    def force_show(self):
        """Force the overlay to be visible regardless of state"""
        try:
            if self.root:
                # Ensure window exists and is configured
                self.root.deiconify()
                self.root.lift()
                self.root.focus_force()
                self.is_visible = True
                
                # Force update and redraw
                self.root.update()
                self.root.update_idletasks()
                
                # Ensure it's on top
                self.root.wm_attributes("-topmost", True)
                
                logger.info("Overlay forced to show")
                
        except Exception as e:
            logger.error(f"Error forcing overlay to show: {e}")
            
    def _ensure_visible(self):
        """Ensure the overlay stays visible"""
        if self.config.get('window.always_visible', True) and self.root and not self.is_visible:
            self.root.deiconify()
            self.root.lift()
            self.is_visible = True
            logger.debug("Overlay visibility restored")
            
    def hide(self):
        """Hide the overlay"""
        # Don't hide if always_visible is enabled
        if self.config.get('window.always_visible', True):
            logger.debug("Hide request ignored - always_visible is enabled")
            return
            
        if self.is_visible and self.root:
            self.root.withdraw()
            self.is_visible = False
            logger.debug("Overlay hidden")
            
    def is_visible(self) -> bool:
        """Check if overlay is visible"""
        return self.is_visible
        
    def destroy(self):
        """Destroy the window"""
        if self.root:
            self.root.destroy()
            logger.info("Main window destroyed")
            
    def update_config(self):
        """Update UI based on configuration changes"""
        if self.root:
            # Update transparency
            transparency = self.config.get('window.transparency', 0.9)
            self.root.wm_attributes("-alpha", transparency)
            
            # Update size and position
            width = self.config.get('window.width', 400)
            height = self.config.get('window.height', 300)
            self.root.geometry(f"{width}x{height}")
            
            logger.debug("UI configuration updated")
            
    def get_status(self) -> dict:
        """Get current window status"""
        return {
            'visible': self.is_visible,
            'geometry': self.root.geometry() if self.root else None,
            'transparency': self.config.get('window.transparency', 0.9)
        }
