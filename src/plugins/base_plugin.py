"""
Base Plugin System for POE2 Master Overlay

Provides the foundation for creating and managing plugins.
"""

import importlib
import inspect
import logging
from abc import ABC, abstractmethod
from pathlib import Path
from typing import Dict, List, Optional, Type, Any
import threading

from ..core.event_bus import EventType, event_bus
from ..config.config_manager import ConfigManager

logger = logging.getLogger(__name__)


class BasePlugin(ABC):
    """Base class for all POE2 Master Overlay plugins"""
    
    def __init__(self, name: str, config: ConfigManager):
        """
        Initialize the plugin
        
        Args:
            name: Plugin name
            config: Configuration manager
        """
        self.name = name
        self.config = config
        self.enabled = False
        self.initialized = False
        self._lock = threading.RLock()
        
    @abstractmethod
    def initialize(self) -> bool:
        """
        Initialize the plugin
        
        Returns:
            True if initialization successful, False otherwise
        """
        pass
        
    @abstractmethod
    def cleanup(self) -> None:
        """Cleanup plugin resources"""
        pass
        
    def enable(self) -> bool:
        """Enable the plugin"""
        with self._lock:
            if not self.initialized:
                if self.initialize():
                    self.enabled = True
                    logger.info(f"Plugin {self.name} enabled")
                    return True
                else:
                    logger.error(f"Failed to initialize plugin {self.name}")
                    return False
            else:
                self.enabled = True
                logger.info(f"Plugin {self.name} enabled")
                return True
                
    def disable(self) -> None:
        """Disable the plugin"""
        with self._lock:
            if self.enabled:
                self.enabled = False
                logger.info(f"Plugin {self.name} disabled")
                
    def get_status(self) -> Dict[str, Any]:
        """Get plugin status information"""
        return {
            'name': self.name,
            'enabled': self.enabled,
            'initialized': self.initialized
        }
        
    def get_config_section(self) -> str:
        """Get the configuration section name for this plugin"""
        return f"plugins.{self.name}"
        
    def get_config(self, key: str, default: Any = None) -> Any:
        """Get plugin-specific configuration value"""
        config_key = f"{self.get_config_section()}.{key}"
        return self.config.get(config_key, default)
        
    def set_config(self, key: str, value: Any) -> bool:
        """Set plugin-specific configuration value"""
        config_key = f"{self.get_config_section()}.{key}"
        return self.config.set(config_key, value)


class PluginManager:
    """Manages plugin loading, initialization, and lifecycle"""
    
    def __init__(self, config: ConfigManager):
        """
        Initialize the plugin manager
        
        Args:
            config: Configuration manager
        """
        self.config = config
        self.plugins: Dict[str, BasePlugin] = {}
        self.plugin_classes: Dict[str, Type[BasePlugin]] = {}
        self.plugin_directories: List[Path] = []
        
        # Add default plugin directories
        self.add_plugin_directory(Path(__file__).parent)
        
    def add_plugin_directory(self, directory: Path) -> None:
        """Add a directory to search for plugins"""
        if directory.exists() and directory.is_dir():
            self.plugin_directories.append(directory)
            logger.info(f"Added plugin directory: {directory}")
            
    def discover_plugins(self) -> List[str]:
        """Discover available plugins in plugin directories"""
        discovered_plugins = []
        
        for plugin_dir in self.plugin_directories:
            try:
                for plugin_file in plugin_dir.glob("*.py"):
                    if plugin_file.name.startswith("_") or plugin_file.name == "base_plugin.py":
                        continue
                        
                    plugin_name = plugin_file.stem
                    if plugin_name not in discovered_plugins:
                        discovered_plugins.append(plugin_name)
                        
            except Exception as e:
                logger.error(f"Error discovering plugins in {plugin_dir}: {e}")
                
        logger.info(f"Discovered {len(discovered_plugins)} plugins: {discovered_plugins}")
        return discovered_plugins
        
    def load_plugin(self, plugin_name: str) -> Optional[BasePlugin]:
        """Load a plugin by name"""
        try:
            # Try to import the plugin module
            module_name = f"poe2_master.plugins.{plugin_name}"
            module = importlib.import_module(module_name)
            
            # Look for plugin class (should be named PluginNamePlugin)
            plugin_class = None
            for name, obj in inspect.getmembers(module):
                if (inspect.isclass(obj) and 
                    issubclass(obj, BasePlugin) and 
                    obj != BasePlugin):
                    plugin_class = obj
                    break
                    
            if plugin_class is None:
                logger.warning(f"No plugin class found in {plugin_name}")
                return None
                
            # Create plugin instance
            plugin = plugin_class(self.config)
            self.plugins[plugin_name] = plugin
            self.plugin_classes[plugin_name] = plugin_class
            
            logger.info(f"Plugin {plugin_name} loaded successfully")
            return plugin
            
        except Exception as e:
            logger.error(f"Failed to load plugin {plugin_name}: {e}")
            return None
            
    def load_all_plugins(self) -> None:
        """Load all discovered plugins"""
        discovered_plugins = self.discover_plugins()
        
        for plugin_name in discovered_plugins:
            self.load_plugin(plugin_name)
            
    def enable_plugin(self, plugin_name: str) -> bool:
        """Enable a specific plugin"""
        if plugin_name not in self.plugins:
            if not self.load_plugin(plugin_name):
                return False
                
        plugin = self.plugins[plugin_name]
        return plugin.enable()
        
    def disable_plugin(self, plugin_name: str) -> None:
        """Disable a specific plugin"""
        if plugin_name in self.plugins:
            self.plugins[plugin_name].disable()
            
    def enable_all_plugins(self) -> None:
        """Enable all loaded plugins"""
        enabled_plugins = self.config.get('plugins.enabled_plugins', [])
        
        for plugin_name in enabled_plugins:
            if plugin_name in self.plugins:
                self.enable_plugin(plugin_name)
            else:
                logger.warning(f"Plugin {plugin_name} not found")
                
    def disable_all_plugins(self) -> None:
        """Disable all plugins"""
        for plugin in self.plugins.values():
            plugin.disable()
            
    def get_plugin(self, plugin_name: str) -> Optional[BasePlugin]:
        """Get a plugin by name"""
        return self.plugins.get(plugin_name)
        
    def get_plugin_status(self) -> Dict[str, Dict[str, Any]]:
        """Get status of all plugins"""
        return {name: plugin.get_status() for name, plugin in self.plugins.items()}
        
    def reload_plugin(self, plugin_name: str) -> bool:
        """Reload a plugin"""
        if plugin_name in self.plugins:
            # Disable and cleanup old plugin
            old_plugin = self.plugins[plugin_name]
            old_plugin.disable()
            old_plugin.cleanup()
            
            # Remove from plugins dict
            del self.plugins[plugin_name]
            
        # Load the plugin again
        return self.load_plugin(plugin_name) is not None
        
    def cleanup(self) -> None:
        """Cleanup all plugins"""
        for plugin in self.plugins.values():
            try:
                plugin.cleanup()
            except Exception as e:
                logger.error(f"Error cleaning up plugin {plugin.name}: {e}")
                
        self.plugins.clear()
        self.plugin_classes.clear()
        
    def get_available_plugins(self) -> List[str]:
        """Get list of available plugin names"""
        return list(self.plugins.keys())
        
    def get_enabled_plugins(self) -> List[str]:
        """Get list of enabled plugin names"""
        return [name for name, plugin in self.plugins.items() if plugin.enabled]
