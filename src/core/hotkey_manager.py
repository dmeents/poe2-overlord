"""
Hotkey Manager for POE2 Master Overlay

Handles global hotkeys for controlling the overlay from anywhere in the system.
"""

import threading
from typing import Dict, Optional, Callable
import logging

try:
    from pynput import keyboard
    HOTKEYS_AVAILABLE = True
except ImportError:
    HOTKEYS_AVAILABLE = False

from .event_bus import EventType, event_bus
from ..config.config_manager import ConfigManager

logger = logging.getLogger(__name__)


class HotkeyManager:
    """Manages global hotkeys for the overlay"""
    
    def __init__(self, config: ConfigManager):
        self.config = config
        self.hotkey_listener: Optional[keyboard.GlobalHotKeys] = None
        self.hotkey_callbacks: Dict[str, Callable] = {}
        self.enabled = False
        
        # Subscribe to configuration changes
        event_bus.subscribe(EventType.CONFIG_CHANGED, self._on_config_changed)
        
        # Setup default hotkeys
        self._setup_default_hotkeys()
        
    def start(self) -> bool:
        """Start the hotkey manager"""
        if not HOTKEYS_AVAILABLE:
            logger.warning("pynput not available, hotkeys disabled")
            return False
            
        if self.enabled:
            logger.warning("Hotkey manager already started")
            return True
            
        try:
            self._setup_hotkeys()
            self.enabled = True
            logger.info("Hotkey manager started")
            return True
        except Exception as e:
            logger.error(f"Failed to start hotkey manager: {e}")
            return False
            
    def stop(self) -> None:
        """Stop the hotkey manager"""
        if not self.enabled:
            return
            
        if self.hotkey_listener:
            self.hotkey_listener.stop()
            self.hotkey_listener = None
            
        self.enabled = False
        logger.info("Hotkey manager stopped")
        
    def _setup_default_hotkeys(self) -> None:
        """Setup default hotkey bindings"""
        default_hotkeys = {
            'toggle_overlay': '<ctrl>+<shift>+o',
            'quick_search': '<ctrl>+<shift>+f',
            'hide_overlay': '<escape>'
        }
        
        for action, hotkey in default_hotkeys.items():
            self.hotkey_callbacks[action] = self._get_default_callback(action)
            
    def _setup_hotkeys(self) -> None:
        """Setup the actual hotkey bindings"""
        if not HOTKEYS_AVAILABLE:
            return
            
        # Get hotkey configuration
        hotkey_config = self.config.get('hotkeys', {})
        
        # Build hotkey mapping
        hotkey_mapping = {}
        for action, hotkey in hotkey_config.items():
            if action in self.hotkey_callbacks:
                hotkey_mapping[hotkey] = self.hotkey_callbacks[action]
                
        if not hotkey_mapping:
            logger.warning("No valid hotkeys configured")
            return
            
        try:
            self.hotkey_listener = keyboard.GlobalHotKeys(hotkey_mapping)
            self.hotkey_listener.start()
            logger.info(f"Hotkeys registered: {list(hotkey_mapping.keys())}")
        except Exception as e:
            logger.error(f"Failed to setup hotkeys: {e}")
            raise
            
    def _get_default_callback(self, action: str) -> Callable:
        """Get default callback for a hotkey action"""
        def callback():
            logger.debug(f"Hotkey triggered: {action}")
            event_bus.publish_simple(
                EventType.HOTKEY_TRIGGERED,
                data={'action': action},
                source="HotkeyManager"
            )
        return callback
        
    def register_hotkey(self, action: str, hotkey: str, callback: Callable) -> bool:
        """Register a custom hotkey"""
        if not HOTKEYS_AVAILABLE:
            logger.warning("Cannot register hotkey: pynput not available")
            return False
            
        try:
            # Stop current listener
            if self.hotkey_listener:
                self.hotkey_listener.stop()
                
            # Update callbacks
            self.hotkey_callbacks[action] = callback
            
            # Restart with new configuration
            self._setup_hotkeys()
            logger.info(f"Registered hotkey: {action} -> {hotkey}")
            return True
            
        except Exception as e:
            logger.error(f"Failed to register hotkey {action}: {e}")
            return False
            
    def unregister_hotkey(self, action: str) -> bool:
        """Unregister a hotkey"""
        if action not in self.hotkey_callbacks:
            return False
            
        try:
            # Stop current listener
            if self.hotkey_listener:
                self.hotkey_listener.stop()
                
            # Remove callback
            del self.hotkey_callbacks[action]
            
            # Restart with updated configuration
            self._setup_hotkeys()
            logger.info(f"Unregistered hotkey: {action}")
            return True
            
        except Exception as e:
            logger.error(f"Failed to unregister hotkey {action}: {e}")
            return False
            
    def _on_config_changed(self, event) -> None:
        """Handle configuration changes"""
        if 'hotkeys' in event.data:
            logger.info("Hotkey configuration changed, restarting hotkey manager")
            if self.enabled:
                self.stop()
                self.start()
                
    def get_status(self) -> dict:
        """Get current hotkey manager status"""
        return {
            'enabled': self.enabled,
            'hotkeys_available': HOTKEYS_AVAILABLE,
            'registered_actions': list(self.hotkey_callbacks.keys()),
            'active_listener': self.hotkey_listener is not None
        }
        
    def get_hotkey_info(self) -> dict:
        """Get information about configured hotkeys"""
        hotkey_config = self.config.get('hotkeys', {})
        return {
            'configured_hotkeys': hotkey_config,
            'available_actions': list(self.hotkey_callbacks.keys()),
            'hotkeys_available': HOTKEYS_AVAILABLE
        }
