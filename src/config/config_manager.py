"""
Enhanced Configuration Manager for POE2 Master Overlay

Handles loading, saving, validation, and hot-reloading of configuration settings.
"""

import json
import os
import threading
import time
from pathlib import Path
from typing import Any, Dict, Optional, List, Callable
import logging

from .defaults import DEFAULT_CONFIG, CONFIG_SCHEMA
from ..core.event_bus import EventType, event_bus

logger = logging.getLogger(__name__)


class ConfigManager:
    """Enhanced configuration manager with validation and hot-reloading"""
    
    def __init__(self, config_file: Optional[str] = None):
        """
        Initialize configuration manager
        
        Args:
            config_file: Path to config file, defaults to ~/.config/poe2-master/config.json
        """
        if config_file is None:
            config_dir = Path.home() / '.config' / 'poe2-master'
            config_dir.mkdir(parents=True, exist_ok=True)
            self.config_file = config_dir / 'config.json'
        else:
            self.config_file = Path(config_file)
            
        self.config = DEFAULT_CONFIG.copy()
        self._watchers: List[Callable] = []
        self._file_watcher_thread: Optional[threading.Thread] = None
        self._watching = False
        self._last_modified = 0
        
        # Load configuration
        self._load_config()
        
        # Start file watching if enabled
        if self.get('debug.enable_config_watching', False):
            self._start_file_watcher()
            
    def _load_config(self) -> None:
        """Load configuration from file"""
        try:
            if self.config_file.exists():
                with open(self.config_file, 'r') as f:
                    user_config = json.load(f)
                    self._merge_config(user_config)
                    self._last_modified = self.config_file.stat().st_mtime
                    logger.info(f"Configuration loaded from {self.config_file}")
            else:
                logger.info(f"No config file found at {self.config_file}, using defaults")
                self.save()  # Create default config file
        except Exception as e:
            logger.error(f"Error loading config: {e}")
            logger.info("Using default configuration")
            
    def _merge_config(self, user_config: Dict[str, Any]) -> None:
        """Recursively merge user config with defaults"""
        def merge_dict(default: Dict, user: Dict) -> Dict:
            result = default.copy()
            for key, value in user.items():
                if key in result and isinstance(result[key], dict) and isinstance(value, dict):
                    result[key] = merge_dict(result[key], value)
                else:
                    result[key] = value
            return result
            
        self.config = merge_dict(self.config, user_config)
        
        # Validate configuration
        validation_errors = self.validate_config()
        if validation_errors:
            logger.warning(f"Configuration validation warnings: {validation_errors}")
            
    def save(self) -> None:
        """Save current configuration to file"""
        try:
            self.config_file.parent.mkdir(parents=True, exist_ok=True)
            with open(self.config_file, 'w') as f:
                json.dump(self.config, f, indent=2)
            self._last_modified = self.config_file.stat().st_mtime
            logger.info(f"Configuration saved to {self.config_file}")
            
            # Notify watchers
            self._notify_watchers()
            
        except Exception as e:
            logger.error(f"Error saving config: {e}")
            
    def get(self, key: str, default: Any = None) -> Any:
        """
        Get configuration value using dot notation
        
        Args:
            key: Configuration key (e.g., 'window.width' or 'api.rate_limit_requests')
            default: Default value if key not found
            
        Returns:
            Configuration value or default
        """
        try:
            keys = key.split('.')
            value = self.config
            
            for k in keys:
                if isinstance(value, dict) and k in value:
                    value = value[k]
                else:
                    return default
                    
            return value
        except Exception:
            return default
            
    def set(self, key: str, value: Any) -> bool:
        """
        Set configuration value using dot notation
        
        Args:
            key: Configuration key (e.g., 'window.width')
            value: Value to set
            
        Returns:
            True if successful, False otherwise
        """
        try:
            keys = key.split('.')
            config = self.config
            
            # Navigate to the parent dictionary
            for k in keys[:-1]:
                if k not in config:
                    config[k] = {}
                config = config[k]
                
            # Set the value
            config[keys[-1]] = value
            
            # Validate the change
            validation_errors = self.validate_config()
            if validation_errors:
                logger.warning(f"Configuration validation warnings after setting {key}: {validation_errors}")
                
            # Notify watchers
            self._notify_watchers()
            
            # Publish configuration change event
            event_bus.publish_simple(
                EventType.CONFIG_CHANGED,
                data={key: value},
                source="ConfigManager"
            )
            
            return True
            
        except Exception as e:
            logger.error(f"Error setting config key '{key}': {e}")
            return False
            
    def reset_to_defaults(self) -> None:
        """Reset configuration to defaults"""
        self.config = DEFAULT_CONFIG.copy()
        self.save()
        logger.info("Configuration reset to defaults")
        
    def validate_config(self) -> List[str]:
        """
        Validate current configuration and return list of issues
        
        Returns:
            List of validation error messages
        """
        issues = []
        
        # Validate window settings
        width = self.get('window.width')
        if not isinstance(width, int) or width < 200 or width > 2000:
            issues.append("Window width must be between 200 and 2000 pixels")
            
        height = self.get('window.height')
        if not isinstance(height, int) or height < 150 or height > 1500:
            issues.append("Window height must be between 150 and 1500 pixels")
            
        transparency = self.get('window.transparency')
        if not isinstance(transparency, (int, float)) or transparency < 0.1 or transparency > 1.0:
            issues.append("Window transparency must be between 0.1 and 1.0")
            
        # Validate API settings
        rate_limit = self.get('api.rate_limit_requests')
        if not isinstance(rate_limit, int) or rate_limit < 1 or rate_limit > 100:
            issues.append("API rate limit requests must be between 1 and 100")
            
        # Validate search settings
        max_results = self.get('search.max_results')
        if not isinstance(max_results, int) or max_results < 1 or max_results > 100:
            issues.append("Max search results must be between 1 and 100")
            
        return issues
        
    def add_watcher(self, callback: Callable) -> None:
        """Add a configuration change watcher"""
        self._watchers.append(callback)
        
    def remove_watcher(self, callback: Callable) -> None:
        """Remove a configuration change watcher"""
        if callback in self._watchers:
            self._watchers.remove(callback)
            
    def _notify_watchers(self) -> None:
        """Notify all configuration watchers"""
        for watcher in self._watchers:
            try:
                watcher(self.config)
            except Exception as e:
                logger.error(f"Error in config watcher: {e}")
                
    def _start_file_watcher(self) -> None:
        """Start watching the configuration file for changes"""
        if self._watching:
            return
            
        self._watching = True
        self._file_watcher_thread = threading.Thread(target=self._file_watcher_loop, daemon=True)
        self._file_watcher_thread.start()
        logger.info("Configuration file watcher started")
        
    def _file_watcher_loop(self) -> None:
        """File watching loop"""
        while self._watching:
            try:
                if self.config_file.exists():
                    current_mtime = self.config_file.stat().st_mtime
                    if current_mtime > self._last_modified:
                        logger.info("Configuration file changed, reloading...")
                        self._load_config()
                        self._notify_watchers()
                        
                time.sleep(1)  # Check every second
            except Exception as e:
                logger.error(f"Error in file watcher: {e}")
                time.sleep(5)
                
    def stop_file_watcher(self) -> None:
        """Stop watching the configuration file"""
        self._watching = False
        if self._file_watcher_thread and self._file_watcher_thread.is_alive():
            self._file_watcher_thread.join(timeout=2.0)
        logger.info("Configuration file watcher stopped")
        
    def export_config(self, file_path: str) -> bool:
        """Export current configuration to a file"""
        try:
            with open(file_path, 'w') as f:
                json.dump(self.config, f, indent=2)
            logger.info(f"Configuration exported to {file_path}")
            return True
        except Exception as e:
            logger.error(f"Error exporting config: {e}")
            return False
            
    def import_config(self, file_path: str) -> bool:
        """Import configuration from a file"""
        try:
            with open(file_path, 'r') as f:
                imported_config = json.load(f)
                self._merge_config(imported_config)
                self.save()
            logger.info(f"Configuration imported from {file_path}")
            return True
        except Exception as e:
            logger.error(f"Error importing config: {e}")
            return False
            
    def get_config_file_path(self) -> str:
        """Get the path to the configuration file"""
        return str(self.config_file)
        
    def reload(self) -> None:
        """Reload configuration from file"""
        self._load_config()
        self._notify_watchers()
        logger.info("Configuration reloaded")
        
    def get_section(self, section: str) -> Dict[str, Any]:
        """Get an entire configuration section"""
        return self.get(section, {})
        
    def set_section(self, section: str, values: Dict[str, Any]) -> bool:
        """Set an entire configuration section"""
        try:
            self.config[section] = values
            self.save()
            return True
        except Exception as e:
            logger.error(f"Error setting config section '{section}': {e}")
            return False
            
    def has_key(self, key: str) -> bool:
        """Check if a configuration key exists"""
        try:
            keys = key.split('.')
            config = self.config
            
            for k in keys:
                if isinstance(config, dict) and k in config:
                    config = config[k]
                else:
                    return False
            return True
        except Exception:
            return False
            
    def __str__(self) -> str:
        """String representation of configuration"""
        return json.dumps(self.config, indent=2)
        
    def __del__(self):
        """Cleanup when the object is destroyed"""
        self.stop_file_watcher()
