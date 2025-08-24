"""
Process Monitor for POE2 Master Overlay

Monitors system processes to detect when Path of Exile 2 is running.
"""

import threading
import time
import psutil
from typing import List, Optional, Set
import logging

from .event_bus import EventType, event_bus
from ..config.config_manager import ConfigManager

logger = logging.getLogger(__name__)


class ProcessMonitor:
    """Monitors system processes for POE2 detection"""
    
    def __init__(self, config: ConfigManager):
        self.config = config
        self.monitoring = False
        self.monitor_thread: Optional[threading.Thread] = None
        self.poe2_running = False
        self.last_check_time = 0
        self.check_interval = config.get('process.check_interval', 2.0)
        
        # Subscribe to configuration changes
        event_bus.subscribe(EventType.CONFIG_CHANGED, self._on_config_changed)
        
    def start(self) -> None:
        """Start the process monitoring"""
        if self.monitoring:
            logger.warning("Process monitoring already started")
            return
            
        self.monitoring = True
        self.monitor_thread = threading.Thread(target=self._monitor_loop, daemon=True)
        self.monitor_thread.start()
        logger.info("Process monitoring started")
        
    def stop(self) -> None:
        """Stop the process monitoring"""
        if not self.monitoring:
            return
            
        self.monitoring = False
        if self.monitor_thread and self.monitor_thread.is_alive():
            self.monitor_thread.join(timeout=2.0)
        logger.info("Process monitoring stopped")
        
    def _monitor_loop(self) -> None:
        """Main monitoring loop"""
        while self.monitoring:
            try:
                self._check_poe2_status()
                time.sleep(self.check_interval)
            except Exception as e:
                logger.error(f"Error in process monitoring loop: {e}")
                time.sleep(5.0)  # Longer delay on error
                
    def _check_poe2_status(self) -> None:
        """Check if POE2 is currently running"""
        current_status = self._is_poe2_running()
        
        if current_status != self.poe2_running:
            self.poe2_running = current_status
            self.last_check_time = time.time()
            
            if self.poe2_running:
                logger.info("POE2 process detected")
                event_bus.publish_simple(
                    EventType.POE2_STARTED,
                    source="ProcessMonitor"
                )
            else:
                logger.info("POE2 process not running")
                event_bus.publish_simple(
                    EventType.POE2_STOPPED,
                    source="ProcessMonitor"
                )
                
    def _is_poe2_running(self) -> bool:
        """Check if any POE2 process is currently running"""
        try:
            # Get configured process names
            config_processes = self.config.get('process.poe2_executable_names', [])
            
            # Comprehensive list of possible POE2 process names
            poe2_processes = config_processes + [
                # Windows executables (for Wine/Proton)
                'PathOfExile_x64Steam.exe',
                'PathOfExile.exe', 
                'PathOfExile_x64.exe',
                'PathOfExileTwo.exe',
                # Linux native (hypothetical)
                'pathofexile2',
                'poe2',
                # Steam/Proton processes
                'PathOfExile_x64Steam',
                'PathOfExile_x64',
                'PathOfExile',
            ]
            
            found_processes = []
            
            for proc in psutil.process_iter(['name', 'cmdline']):
                try:
                    proc_name = proc.info.get('name', '')
                    cmdline_list = proc.info.get('cmdline', [])
                    
                    # Safely join cmdline
                    if isinstance(cmdline_list, list):
                        cmdline = ' '.join(str(arg) for arg in cmdline_list if arg is not None)
                    else:
                        cmdline = str(cmdline_list) if cmdline_list else ''
                    
                    # Check process name
                    if proc_name:
                        for poe_proc in poe2_processes:
                            if poe_proc.lower() in proc_name.lower():
                                logger.debug(f"Found POE2 process by name: {proc_name}")
                                return True
                                
                        # Check for "poe" or "pathofexile" in process name
                        if any(keyword in proc_name.lower() for keyword in ['poe', 'pathofexile']):
                            found_processes.append(proc_name)
                            
                    # Check command line for Steam/Proton launches
                    if cmdline and any(keyword in cmdline.lower() for keyword in ['pathofexile', 'poe']):
                        logger.debug(f"Found POE2-related process in cmdline: {proc_name} -> {cmdline[:100]}...")
                        # Be more selective about cmdline matches
                        if any(exe in cmdline.lower() for exe in ['pathofexile_x64steam.exe', 'pathofexile.exe']):
                            return True
                            
                except (psutil.NoSuchProcess, psutil.AccessDenied, psutil.ZombieProcess):
                    continue
                    
            # Debug output for potential matches
            if found_processes:
                logger.debug(f"Potential POE2 processes found (but not matched): {found_processes}")
                
            return False
            
        except Exception as e:
            logger.error(f"Error checking POE2 process status: {e}")
            return False
            
    def _on_config_changed(self, event) -> None:
        """Handle configuration changes"""
        if 'process.check_interval' in event.data:
            self.check_interval = event.data['process.check_interval']
            logger.info(f"Process check interval updated to {self.check_interval}s")
            
    def get_status(self) -> dict:
        """Get current monitoring status"""
        return {
            'monitoring': self.monitoring,
            'poe2_running': self.poe2_running,
            'last_check_time': self.last_check_time,
            'check_interval': self.check_interval
        }
        
    def force_check(self) -> bool:
        """Force an immediate POE2 status check"""
        self._check_poe2_status()
        return self.poe2_running
