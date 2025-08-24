"""
Configuration management for POE2 Master Overlay

This module handles all configuration-related functionality including loading,
validation, and hot-reloading of settings.
"""

from .config_manager import ConfigManager
from .defaults import DEFAULT_CONFIG

__all__ = [
    'ConfigManager',
    'DEFAULT_CONFIG'
]
