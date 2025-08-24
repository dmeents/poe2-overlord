#!/usr/bin/env python3
"""
Development Server with Hot Reloading for POE2 Master Overlay

This module provides a development server that automatically restarts the overlay
when source code changes are detected.
"""

import os
import sys
import time
import signal
import subprocess
import threading
from pathlib import Path
from typing import Optional, List, Set
import logging

from watchdog.observers import Observer
from watchdog.events import FileSystemEventHandler, FileModifiedEvent, FileCreatedEvent, FileDeletedEvent

# Setup basic logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)


class OverlayProcessManager:
    """Manages the overlay process lifecycle during development"""
    
    def __init__(self):
        self.process: Optional[subprocess.Popen] = None
        self.restart_pending = False
        self.shutdown_requested = False
        
        # Setup signal handlers
        signal.signal(signal.SIGINT, self._signal_handler)
        signal.signal(signal.SIGTERM, self._signal_handler)
        
    def start_overlay(self) -> bool:
        """Start the overlay application"""
        if self.process and self.process.poll() is None:
            logger.info("Overlay already running")
            return True
            
        try:
            logger.info("Starting POE2 Master Overlay...")
            
            # Start the overlay as a subprocess
            self.process = subprocess.Popen(
                [sys.executable, "-m", "src"],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                bufsize=1,
                universal_newlines=True
            )
            
            logger.info(f"Overlay started with PID: {self.process.pid}")
            return True
            
        except Exception as e:
            logger.error(f"Failed to start overlay: {e}")
            return False
            
    def stop_overlay(self) -> bool:
        """Stop the overlay application"""
        if not self.process:
            return True
            
        try:
            logger.info("Stopping overlay...")
            
            # Send SIGTERM first
            self.process.terminate()
            
            # Wait for graceful shutdown
            try:
                self.process.wait(timeout=5)
                logger.info("Overlay stopped gracefully")
            except subprocess.TimeoutExpired:
                logger.warning("Overlay didn't stop gracefully, forcing...")
                self.process.kill()
                self.process.wait()
                
            self.process = None
            return True
            
        except Exception as e:
            logger.error(f"Error stopping overlay: {e}")
            return False
            
    def restart_overlay(self) -> bool:
        """Restart the overlay application"""
        logger.info("Restarting overlay...")
        
        if self.stop_overlay():
            time.sleep(1)  # Brief pause between stop/start
            return self.start_overlay()
        return False
        
    def is_running(self) -> bool:
        """Check if overlay is currently running"""
        return self.process is not None and self.process.poll() is None
        
    def _signal_handler(self, signum, frame):
        """Handle shutdown signals"""
        logger.info(f"Received signal {signum}, shutting down...")
        self.shutdown_requested = True
        self.stop_overlay()


class SourceCodeWatcher(FileSystemEventHandler):
    """Watches source code files for changes"""
    
    def __init__(self, process_manager: OverlayProcessManager, source_dirs: List[Path]):
        self.process_manager = process_manager
        self.source_dirs = source_dirs
        self.ignored_patterns = {
            '__pycache__', '.pyc', '.pyo', '.pyd',
            '.git', '.svn', '.hg',
            '.DS_Store', 'Thumbs.db',
            '*.swp', '*.swo', '*~',
            '.pytest_cache', '.coverage',
            'build', 'dist', '*.egg-info'
        }
        self.last_restart_time = 0
        self.restart_cooldown = 2.0  # Minimum seconds between restarts
        
    def on_modified(self, event):
        """Handle file modification events"""
        if not event.is_directory and self._should_watch_file(event.src_path):
            self._handle_file_change(event.src_path, "modified")
            
    def on_created(self, event):
        """Handle file creation events"""
        if not event.is_directory and self._should_watch_file(event.src_path):
            self._handle_file_change(event.src_path, "created")
            
    def on_deleted(self, event):
        """Handle file deletion events"""
        if not event.is_directory and self._should_watch_file(event.src_path):
            self._handle_file_change(event.src_path, "deleted")
            
    def _should_watch_file(self, file_path: str) -> bool:
        """Check if a file should be watched for changes"""
        path = Path(file_path)
        
        # Check if it's a Python file
        if path.suffix != '.py':
            return False
            
        # Check if it's in a source directory
        if not any(path.is_relative_to(src_dir) for src_dir in self.source_dirs):
            return False
            
        # Check if it should be ignored
        for pattern in self.ignored_patterns:
            if pattern in str(path):
                return False
                
        return True
        
    def _handle_file_change(self, file_path: str, change_type: str):
        """Handle file change events"""
        current_time = time.time()
        
        # Prevent rapid restarts
        if current_time - self.last_restart_time < self.restart_cooldown:
            return
            
        logger.info(f"Source file {change_type}: {file_path}")
        
        # Mark restart as pending
        self.process_manager.restart_pending = True
        self.last_restart_time = current_time
        
        # Schedule restart after a brief delay to catch multiple rapid changes
        threading.Timer(0.5, self._perform_restart).start()
        
    def _perform_restart(self):
        """Perform the actual restart"""
        if self.process_manager.restart_pending:
            self.process_manager.restart_pending = False
            self.process_manager.restart_overlay()


class DevServer:
    """Development server with hot reloading"""
    
    def __init__(self, source_dirs: Optional[List[str]] = None):
        self.source_dirs = [Path(d) for d in (source_dirs or ['src'])]
        self.process_manager = OverlayProcessManager()
        self.observer = Observer()
        self.watcher = SourceCodeWatcher(self.process_manager, self.source_dirs)
        
    def start(self):
        """Start the development server"""
        try:
            logger.info("Starting POE2 Master Overlay Development Server...")
            logger.info(f"Watching directories: {[str(d) for d in self.source_dirs]}")
            
            # Start the overlay
            if not self.process_manager.start_overlay():
                logger.error("Failed to start overlay, exiting")
                return False
                
            # Setup file watching
            for source_dir in self.source_dirs:
                if source_dir.exists():
                    self.observer.schedule(self.watcher, str(source_dir), recursive=True)
                    logger.info(f"Watching: {source_dir}")
                else:
                    logger.warning(f"Source directory not found: {source_dir}")
                    
            # Start file watching
            self.observer.start()
            logger.info("File watching started")
            
            # Main loop
            self._main_loop()
            
            return True
            
        except KeyboardInterrupt:
            logger.info("Received keyboard interrupt")
        except Exception as e:
            logger.error(f"Development server error: {e}")
        finally:
            self.stop()
            
        return False
        
    def stop(self):
        """Stop the development server"""
        logger.info("Stopping development server...")
        
        # Stop file watching
        self.observer.stop()
        self.observer.join()
        
        # Stop overlay
        self.process_manager.stop_overlay()
        
        logger.info("Development server stopped")
        
    def _main_loop(self):
        """Main server loop"""
        try:
            while not self.process_manager.shutdown_requested:
                # Check if overlay process is still running
                if not self.process_manager.is_running():
                    logger.warning("Overlay process stopped unexpectedly")
                    if not self.process_manager.start_overlay():
                        logger.error("Failed to restart overlay")
                        break
                        
                time.sleep(1)
                
        except KeyboardInterrupt:
            logger.info("Shutdown requested")


def main():
    """Main entry point for development server"""
    import argparse
    
    parser = argparse.ArgumentParser(description="POE2 Master Overlay Development Server")
    parser.add_argument(
        "--source-dirs", 
        nargs="+", 
        default=["src"],
        help="Source directories to watch (default: src)"
    )
    parser.add_argument(
        "--verbose", "-v",
        action="store_true",
        help="Enable verbose logging"
    )
    
    args = parser.parse_args()
    
    if args.verbose:
        logging.getLogger().setLevel(logging.DEBUG)
        
    # Create and start development server
    server = DevServer(args.source_dirs)
    
    try:
        server.start()
    except KeyboardInterrupt:
        logger.info("Shutdown requested by user")
    finally:
        server.stop()


if __name__ == "__main__":
    main()
