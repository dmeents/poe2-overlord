"""
Main entry point for POE2 Master Overlay

This module is executed when the package is run as a module.
"""

import sys
import os
from pathlib import Path

# Add the src directory to the Python path
src_path = Path(__file__).parent.parent
sys.path.insert(0, str(src_path))

from . import create_overlay, get_logger

logger = get_logger(__name__)


def main():
    """Main entry point for the overlay application"""
    try:
        logger.info("Starting POE2 Master Overlay...")
        
        # Create and start the overlay
        overlay = create_overlay()
        overlay.start()
        
        return 0
        
    except KeyboardInterrupt:
        logger.info("Received keyboard interrupt, shutting down...")
        return 0
    except Exception as e:
        logger.error(f"Fatal error: {e}")
        return 1


if __name__ == "__main__":
    exit(main())
