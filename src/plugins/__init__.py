"""
Plugin system for POE2 Master Overlay

This module provides the plugin architecture for extending overlay functionality.
"""

from .base_plugin import BasePlugin, PluginManager

__all__ = [
    'BasePlugin',
    'PluginManager'
]
