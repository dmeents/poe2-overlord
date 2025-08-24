"""
POE2 Master Overlay
A powerful game overlay for Path of Exile 2 on Linux

This package provides item price checking, build planning, campaign progression tracking,
and other utilities while gaming.
"""

__version__ = "0.2.0"
__author__ = "POE2 Master Overlay Developer"
__description__ = "A game overlay for Path of Exile 2 on Linux"

# Core imports for easy access
from .core.overlay_manager import OverlayManager
from .core.process_monitor import ProcessMonitor
from .core.hotkey_manager import HotkeyManager
from .core.event_bus import EventBus
from .utils.logger import get_logger

# Main entry point
def create_overlay():
    """Create and return a new overlay instance"""
    from .core.overlay_manager import OverlayManager
    return OverlayManager()

__all__ = [
    'OverlayManager',
    'ProcessMonitor', 
    'HotkeyManager',
    'EventBus',
    'get_logger',
    'create_overlay',
    '__version__',
    '__author__',
    '__description__'
]
