"""
Default configuration for POE2 Master Overlay

Contains all default configuration values and structure.
"""

from typing import Dict, Any

# Default configuration structure
DEFAULT_CONFIG: Dict[str, Any] = {
    # UI Settings
    "window": {
        "width": 400,
        "height": 300,
        "x_position": None,  # None means auto-position
        "y_position": None,
        "transparency": 0.9,
        "always_on_top": True,
        "auto_show_on_poe2_start": True,
        "auto_hide_on_poe2_exit": False,
        "minimize_to_tray": False,
        "start_minimized": False,
        "always_visible": True  # New setting to keep overlay always visible
    },
    
    # Hotkeys
    "hotkeys": {
        "toggle_overlay": "<ctrl>+<shift>+o",
        "quick_search": "<ctrl>+<shift>+f",
        "hide_overlay": "<escape>",
        "show_settings": "<ctrl>+<shift>+,",
        "refresh_data": "<ctrl>+<shift>+r"
    },
    
    # API Settings
    "api": {
        "rate_limit_requests": 10,
        "rate_limit_window": 60,
        "cache_ttl": 300,
        "request_timeout": 10,
        "user_agent": "POE2-Master-Overlay/1.0 (Linux)",
        "retry_attempts": 3,
        "retry_delay": 1.0
    },
    
    # Search Settings
    "search": {
        "default_league": "Early Access",
        "max_results": 10,
        "price_sort": "asc",  # asc, desc
        "include_offline": False,
        "min_stock": 1,
        "auto_search_delay": 0.5,
        "search_history_size": 50
    },
    
    # Process Detection
    "process": {
        "poe2_executable_names": [
            "PathOfExile_x64Steam.exe",
            "PathOfExile.exe", 
            "PathOfExile_x64.exe",
            "PathOfExileTwo.exe"
        ],
        "check_interval": 2.0,
        "startup_delay": 5.0
    },
    
    # Appearance
    "appearance": {
        "theme": "dark",  # dark, light, auto
        "font_family": "Arial",
        "font_size": 10,
        "results_font_family": "Consolas",
        "results_font_size": 10,
        "accent_color": "#4CAF50",
        "background_color": "#2E2E2E",
        "text_color": "#FFFFFF",
        "border_radius": 8,
        "animation_speed": 0.2
    },
    
    # Debug and Logging
    "debug": {
        "enable_logging": True,
        "log_level": "INFO",  # DEBUG, INFO, WARNING, ERROR
        "log_file": None,  # None means console only
        "log_api_requests": False,
        "show_mock_data_warning": True,
        "enable_profiling": False,
        "log_to_syslog": False,
        "enable_config_watching": False,
        "development_mode": False
    },
    
    # Development Settings
    "development": {
        "hot_reload": True,
        "watch_source_files": True,
        "auto_restart_on_changes": True,
        "restart_cooldown": 2.0,
        "source_directories": ["src"],
        "ignored_patterns": [
            "__pycache__", "*.pyc", "*.pyo", "*.pyd",
            ".git", ".svn", ".hg", ".DS_Store", "Thumbs.db",
            "*.swp", "*.swo", "*~", ".pytest_cache", ".coverage",
            "build", "dist", "*.egg-info"
        ],
        "verbose_logging": False
    },
    
    # Performance
    "performance": {
        "ui_update_interval": 0.1,
        "cache_cleanup_interval": 300,
        "max_cache_size": 1000,
        "enable_animations": True,
        "reduce_animations": False
    },
    
    # Plugins
    "plugins": {
        "auto_load": True,
        "enabled_plugins": [
            "price_checker",
            "build_planner",
            "progression_tracker"
        ],
        "plugin_directory": "plugins",
        "plugin_timeout": 30.0
    },
    
    # Notifications
    "notifications": {
        "enable_desktop_notifications": True,
        "enable_sound_notifications": False,
        "notification_duration": 5.0,
        "show_price_alerts": True,
        "show_process_alerts": True
    },
    
    # Data
    "data": {
        "auto_backup_config": True,
        "backup_interval": 86400,  # 24 hours
        "max_backups": 10,
        "data_directory": "~/.local/share/poe2-master",
        "export_format": "json"  # json, yaml, csv
    }
}

# Configuration schema for validation
CONFIG_SCHEMA = {
    "type": "object",
    "properties": {
        "window": {
            "type": "object",
            "properties": {
                "width": {"type": "integer", "minimum": 200, "maximum": 2000},
                "height": {"type": "integer", "minimum": 150, "maximum": 1500},
                "transparency": {"type": "number", "minimum": 0.1, "maximum": 1.0},
                "always_on_top": {"type": "boolean"},
                "auto_show_on_poe2_start": {"type": "boolean"},
                "auto_hide_on_poe2_exit": {"type": "boolean"},
                "always_visible": {"type": "boolean"}
            },
            "required": ["width", "height", "transparency"]
        },
        "api": {
            "type": "object",
            "properties": {
                "rate_limit_requests": {"type": "integer", "minimum": 1, "maximum": 100},
                "rate_limit_window": {"type": "integer", "minimum": 10, "maximum": 3600},
                "cache_ttl": {"type": "integer", "minimum": 60, "maximum": 86400}
            }
        },
        "search": {
            "type": "object",
            "properties": {
                "max_results": {"type": "integer", "minimum": 1, "maximum": 100}
            }
        },
        "debug": {
            "type": "object",
            "properties": {
                "enable_logging": {"type": "boolean"},
                "log_level": {"type": "string", "enum": ["DEBUG", "INFO", "WARNING", "ERROR"]},
                "development_mode": {"type": "boolean"}
            }
        },
        "development": {
            "type": "object",
            "properties": {
                "hot_reload": {"type": "boolean"},
                "watch_source_files": {"type": "boolean"},
                "auto_restart_on_changes": {"type": "boolean"},
                "restart_cooldown": {"type": "number", "minimum": 0.5, "maximum": 10.0}
            }
        }
    }
}
