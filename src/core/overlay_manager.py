"""
Main Overlay Manager for POE2 Master Overlay

Coordinates all components and manages the overlay lifecycle.
"""

import threading
import time
from typing import Optional, Dict, Any
import logging

from .event_bus import EventType, event_bus
from .process_monitor import ProcessMonitor
from .hotkey_manager import HotkeyManager
from ..config.config_manager import ConfigManager
from ..utils.logger import setup_logging, get_logger

logger = get_logger(__name__)


class OverlayManager:
    """Main coordinator for the POE2 Master Overlay"""
    
    def __init__(self, config_file: Optional[str] = None):
        """
        Initialize the overlay manager
        
        Args:
            config_file: Optional path to configuration file
        """
        # Setup logging first
        self._setup_logging()
        
        # Initialize configuration
        self.config = ConfigManager(config_file)
        
        # Initialize core components
        self.process_monitor = ProcessMonitor(self.config)
        self.hotkey_manager = HotkeyManager(self.config)
        
        # State management
        self.running = False
        self.ui_ready = False
        self.shutdown_event = threading.Event()
        
        # Subscribe to events
        self._setup_event_handlers()
        
        logger.info("Overlay manager initialized")
        
    def _setup_logging(self) -> None:
        """Setup the logging system"""
        try:
            # Get logging configuration from environment or use defaults
            log_level = os.getenv('POE2_LOG_LEVEL', 'INFO')
            log_file = os.getenv('POE2_LOG_FILE')
            log_to_syslog = os.getenv('POE2_LOG_SYSLOG', 'false').lower() == 'true'
            
            setup_logging(
                level=log_level,
                log_file=log_file,
                log_to_console=True,
                log_to_syslog=log_to_syslog
            )
            
        except Exception as e:
            print(f"Warning: Could not setup logging: {e}")
            # Fallback to basic logging
            logging.basicConfig(level=logging.INFO)
            
    def _setup_event_handlers(self) -> None:
        """Setup event handlers for system events"""
        # Process events
        event_bus.subscribe(EventType.POE2_STARTED, self._on_poe2_started)
        event_bus.subscribe(EventType.POE2_STOPPED, self._on_poe2_stopped)
        
        # Hotkey events
        event_bus.subscribe(EventType.HOTKEY_TRIGGERED, self._on_hotkey_triggered)
        
        # Configuration events
        event_bus.subscribe(EventType.CONFIG_CHANGED, self._on_config_changed)
        
        # UI events
        event_bus.subscribe(EventType.OVERLAY_SHOW, self._on_overlay_show)
        event_bus.subscribe(EventType.OVERLAY_HIDE, self._on_overlay_hide)
        event_bus.subscribe(EventType.OVERLAY_TOGGLE, self._on_overlay_toggle)
        
        logger.debug("Event handlers configured")
        
    def start(self) -> bool:
        """Start the overlay system"""
        if self.running:
            logger.warning("Overlay manager already running")
            return True
            
        try:
            logger.info("Starting POE2 Master Overlay...")
            
            # Start process monitoring
            self.process_monitor.start()
            logger.info("Process monitoring started")
            
            # Start hotkey manager
            if self.hotkey_manager.start():
                logger.info("Hotkey manager started")
            else:
                logger.warning("Hotkey manager failed to start")
                
            # Start UI (this will be implemented in the UI module)
            self._start_ui()
            
            self.running = True
            logger.info("Overlay manager started successfully")
            
            # Main event loop
            self._main_loop()
            
            return True
            
        except Exception as e:
            logger.error(f"Failed to start overlay manager: {e}")
            self.stop()
            return False
            
    def stop(self) -> None:
        """Stop the overlay system"""
        if not self.running:
            return
            
        logger.info("Stopping POE2 Master Overlay...")
        
        # Signal shutdown
        self.shutdown_event.set()
        
        # Stop components
        try:
            self.process_monitor.stop()
            self.hotkey_manager.stop()
            self._stop_ui()
        except Exception as e:
            logger.error(f"Error stopping components: {e}")
            
        self.running = False
        logger.info("Overlay manager stopped")
        
    def _main_loop(self) -> None:
        """Main event loop for the overlay manager"""
        try:
            while not self.shutdown_event.is_set():
                # Process events (if any)
                time.sleep(0.1)
                
        except KeyboardInterrupt:
            logger.info("Received keyboard interrupt")
        except Exception as e:
            logger.error(f"Error in main loop: {e}")
        finally:
            self.stop()
            
    def _start_ui(self) -> None:
        """Start the user interface"""
        try:
            # Import UI components here to avoid circular imports
            from ..ui.main_window import MainWindow
            
            self.main_window = MainWindow(self.config, self)
            self.main_window.show()
            self.ui_ready = True
            
            logger.info("User interface started")
            
        except Exception as e:
            logger.error(f"Failed to start UI: {e}")
            self.ui_ready = False
            
    def _stop_ui(self) -> None:
        """Stop the user interface"""
        if hasattr(self, 'main_window') and self.main_window:
            try:
                self.main_window.destroy()
                logger.info("User interface stopped")
            except Exception as e:
                logger.error(f"Error stopping UI: {e}")
                
        self.ui_ready = False
        
    def _on_poe2_started(self, event) -> None:
        """Handle POE2 process started event"""
        logger.info("POE2 process detected")
        
        # Auto-show overlay if configured
        if self.config.get('window.auto_show_on_poe2_start', True):
            event_bus.publish_simple(EventType.OVERLAY_SHOW, source="OverlayManager")
            
    def _on_poe2_stopped(self, event) -> None:
        """Handle POE2 process stopped event"""
        logger.info("POE2 process stopped")
        
        # Auto-hide overlay if configured
        if self.config.get('window.auto_hide_on_poe2_exit', False):
            event_bus.publish_simple(EventType.OVERLAY_HIDE, source="OverlayManager")
            
    def _on_hotkey_triggered(self, event) -> None:
        """Handle hotkey events"""
        action = event.data.get('action', '')
        logger.debug(f"Hotkey triggered: {action}")
        
        if action == 'toggle_overlay':
            event_bus.publish_simple(EventType.OVERLAY_TOGGLE, source="OverlayManager")
        elif action == 'quick_search':
            # TODO: Implement quick search
            logger.info("Quick search hotkey pressed")
        elif action == 'hide_overlay':
            event_bus.publish_simple(EventType.OVERLAY_HIDE, source="OverlayManager")
        elif action == 'show_settings':
            # TODO: Show settings dialog
            logger.info("Settings hotkey pressed")
        elif action == 'refresh_data':
            # TODO: Refresh data
            logger.info("Refresh data hotkey pressed")
            
    def _on_config_changed(self, event) -> None:
        """Handle configuration changes"""
        logger.debug(f"Configuration changed: {event.data}")
        
        # Update component configurations
        # This will be handled by individual components subscribing to config changes
        
    def _on_overlay_show(self, event) -> None:
        """Handle overlay show event"""
        if self.ui_ready and hasattr(self, 'main_window'):
            self.main_window.show()
            
    def _on_overlay_hide(self, event) -> None:
        """Handle overlay hide event"""
        if self.ui_ready and hasattr(self, 'main_window'):
            self.main_window.hide()
            
    def _on_overlay_toggle(self, event) -> None:
        """Handle overlay toggle event"""
        if self.ui_ready and hasattr(self, 'main_window'):
            if self.main_window.is_visible():
                self.main_window.hide()
            else:
                self.main_window.show()
                
    def get_status(self) -> Dict[str, Any]:
        """Get current system status"""
        return {
            'running': self.running,
            'ui_ready': self.ui_ready,
            'poe2_running': self.process_monitor.poe2_running,
            'hotkeys_enabled': self.hotkey_manager.enabled,
            'config_file': self.config.get_config_file_path(),
            'process_monitor_status': self.process_monitor.get_status(),
            'hotkey_status': self.hotkey_manager.get_status()
        }
        
    def restart(self) -> bool:
        """Restart the overlay system"""
        logger.info("Restarting overlay system...")
        
        try:
            self.stop()
            time.sleep(1)  # Brief pause
            return self.start()
        except Exception as e:
            logger.error(f"Failed to restart overlay: {e}")
            return False
            
    def reload_config(self) -> None:
        """Reload configuration from file"""
        logger.info("Reloading configuration...")
        self.config.reload()
        
    def get_config(self) -> ConfigManager:
        """Get the configuration manager"""
        return self.config
        
    def get_process_monitor(self) -> ProcessMonitor:
        """Get the process monitor"""
        return self.process_monitor
        
    def get_hotkey_manager(self) -> HotkeyManager:
        """Get the hotkey manager"""
        return self.hotkey_manager


# Import os for environment variable access
import os
