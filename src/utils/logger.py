"""
Logging system for POE2 Master Overlay

Provides centralized logging with configurable levels and outputs.
"""

import logging
import logging.handlers
import sys
from pathlib import Path
from typing import Optional
import os

# Global logger instance
_logger: Optional[logging.Logger] = None


def setup_logging(
    level: str = "INFO",
    log_file: Optional[str] = None,
    log_to_console: bool = True,
    log_to_syslog: bool = False,
    max_bytes: int = 1024 * 1024,  # 1MB
    backup_count: int = 5
) -> logging.Logger:
    """
    Setup the logging system for the overlay
    
    Args:
        level: Logging level (DEBUG, INFO, WARNING, ERROR, CRITICAL)
        log_file: Path to log file (optional)
        log_to_console: Whether to log to console
        log_to_syslog: Whether to log to syslog
        max_bytes: Maximum log file size before rotation
        backup_count: Number of backup log files to keep
        
    Returns:
        Configured logger instance
    """
    global _logger
    
    if _logger is not None:
        return _logger
        
    # Create logger
    _logger = logging.getLogger('poe2_master')
    _logger.setLevel(getattr(logging, level.upper(), logging.INFO))
    
    # Clear any existing handlers
    _logger.handlers.clear()
    
    # Create formatter
    formatter = logging.Formatter(
        '%(asctime)s - %(name)s - %(levelname)s - %(funcName)s:%(lineno)d - %(message)s'
    )
    
    # Console handler
    if log_to_console:
        console_handler = logging.StreamHandler(sys.stdout)
        console_handler.setLevel(logging.INFO)
        console_handler.setFormatter(formatter)
        _logger.addHandler(console_handler)
        
    # File handler
    if log_file:
        try:
            log_path = Path(log_file)
            log_path.parent.mkdir(parents=True, exist_ok=True)
            
            # Use rotating file handler
            file_handler = logging.handlers.RotatingFileHandler(
                log_path,
                maxBytes=max_bytes,
                backupCount=backup_count
            )
            file_handler.setLevel(logging.DEBUG)
            file_handler.setFormatter(formatter)
            _logger.addHandler(file_handler)
            
        except Exception as e:
            _logger.warning(f"Could not setup file logging to {log_file}: {e}")
            
    # Syslog handler
    if log_to_syslog:
        try:
            syslog_handler = logging.handlers.SysLogHandler(
                address='/dev/log',
                facility=logging.handlers.SysLogHandler.LOG_USER
            )
            syslog_handler.setLevel(logging.WARNING)
            syslog_handler.setFormatter(formatter)
            _logger.addHandler(syslog_handler)
            
        except Exception as e:
            _logger.warning(f"Could not setup syslog logging: {e}")
            
    # Prevent propagation to root logger
    _logger.propagate = False
    
    _logger.info(f"Logging system initialized (level: {level})")
    return _logger


def get_logger(name: Optional[str] = None) -> logging.Logger:
    """
    Get a logger instance
    
    Args:
        name: Logger name (optional, defaults to 'poe2_master')
        
    Returns:
        Logger instance
    """
    if _logger is None:
        setup_logging()
        
    if name:
        return logging.getLogger(f'poe2_master.{name}')
    return _logger


def set_log_level(level: str) -> None:
    """
    Set the logging level for all handlers
    
    Args:
        level: Logging level (DEBUG, INFO, WARNING, ERROR, CRITICAL)
    """
    if _logger is None:
        return
        
    try:
        log_level = getattr(logging, level.upper(), logging.INFO)
        _logger.setLevel(log_level)
        
        # Update console handler level
        for handler in _logger.handlers:
            if isinstance(handler, logging.StreamHandler) and handler.stream == sys.stdout:
                handler.setLevel(log_level)
                
        _logger.info(f"Log level changed to {level}")
        
    except Exception as e:
        _logger.error(f"Error setting log level: {e}")


def add_file_handler(log_file: str, level: str = "DEBUG") -> bool:
    """
    Add a file handler to the existing logger
    
    Args:
        log_file: Path to log file
        level: Logging level for this handler
        
    Returns:
        True if successful, False otherwise
    """
    if _logger is None:
        return False
        
    try:
        log_path = Path(log_file)
        log_path.parent.mkdir(parents=True, exist_ok=True)
        
        formatter = logging.Formatter(
            '%(asctime)s - %(name)s - %(levelname)s - %(funcName)s:%(lineno)d - %(message)s'
        )
        
        file_handler = logging.handlers.RotatingFileHandler(
            log_path,
            maxBytes=1024 * 1024,  # 1MB
            backupCount=5
        )
        file_handler.setLevel(getattr(logging, level.upper(), logging.DEBUG))
        file_handler.setFormatter(formatter)
        _logger.addHandler(file_handler)
        
        _logger.info(f"Added file handler: {log_file} (level: {level})")
        return True
        
    except Exception as e:
        _logger.error(f"Error adding file handler: {e}")
        return False


def remove_file_handler(log_file: str) -> bool:
    """
    Remove a file handler from the logger
    
    Args:
        log_file: Path to log file
        
    Returns:
        True if successful, False otherwise
    """
    if _logger is None:
        return False
        
    try:
        for handler in _logger.handlers[:]:
            if (isinstance(handler, logging.handlers.RotatingFileHandler) and 
                handler.baseFilename == str(Path(log_file).resolve())):
                handler.close()
                _logger.removeHandler(handler)
                _logger.info(f"Removed file handler: {log_file}")
                return True
                
        return False
        
    except Exception as e:
        _logger.error(f"Error removing file handler: {e}")
        return False


def get_log_file_handlers() -> list:
    """
    Get list of current file handlers
    
    Returns:
        List of file handler paths
    """
    if _logger is None:
        return []
        
    file_handlers = []
    for handler in _logger.handlers:
        if isinstance(handler, logging.handlers.RotatingFileHandler):
            file_handlers.append(handler.baseFilename)
            
    return file_handlers


def cleanup_logs() -> None:
    """Clean up old log files"""
    if _logger is None:
        return
        
    try:
        for handler in _logger.handlers:
            if isinstance(handler, logging.handlers.RotatingFileHandler):
                handler.doRollover()
                
        _logger.info("Log rotation completed")
        
    except Exception as e:
        _logger.error(f"Error during log cleanup: {e}")


# Convenience functions for common logging operations
def debug(msg: str, *args, **kwargs) -> None:
    """Log debug message"""
    if _logger:
        _logger.debug(msg, *args, **kwargs)


def info(msg: str, *args, **kwargs) -> None:
    """Log info message"""
    if _logger:
        _logger.info(msg, *args, **kwargs)


def warning(msg: str, *args, **kwargs) -> None:
    """Log warning message"""
    if _logger:
        _logger.warning(msg, *args, **kwargs)


def error(msg: str, *args, **kwargs) -> None:
    """Log error message"""
    if _logger:
        _logger.error(msg, *args, **kwargs)


def critical(msg: str, *args, **kwargs) -> None:
    """Log critical message"""
    if _logger:
        _logger.critical(msg, *args, **kwargs)


def exception(msg: str, *args, **kwargs) -> None:
    """Log exception message with traceback"""
    if _logger:
        _logger.exception(msg, *args, **kwargs)
