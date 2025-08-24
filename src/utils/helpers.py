"""
Helper utilities for POE2 Master Overlay

Provides common utility functions used throughout the application.
"""

import time
import hashlib
import json
from datetime import datetime, timedelta
from pathlib import Path
from typing import Any, Dict, List, Optional, Union
import logging

logger = logging.getLogger(__name__)


def format_currency(amount: Union[int, float], currency: str = "Chaos Orb") -> str:
    """
    Format a currency amount with proper formatting
    
    Args:
        amount: Amount to format
        currency: Currency name
        
    Returns:
        Formatted currency string
    """
    if amount == 0:
        return f"0 {currency}"
        
    if amount < 1 and amount > 0:
        return f"{amount:.3f} {currency}"
        
    if amount < 1000:
        return f"{amount:.1f} {currency}"
    elif amount < 1000000:
        return f"{amount/1000:.1f}k {currency}"
    else:
        return f"{amount/1000000:.1f}M {currency}"


def format_time(seconds: Union[int, float]) -> str:
    """
    Format time duration in a human-readable format
    
    Args:
        seconds: Time in seconds
        
    Returns:
        Formatted time string
    """
    if seconds < 60:
        return f"{seconds:.1f}s"
    elif seconds < 3600:
        minutes = seconds / 60
        return f"{minutes:.1f}m"
    elif seconds < 86400:
        hours = seconds / 3600
        return f"{hours:.1f}h"
    else:
        days = seconds / 86400
        return f"{days:.1f}d"


def format_file_size(bytes_size: int) -> str:
    """
    Format file size in human-readable format
    
    Args:
        bytes_size: Size in bytes
        
    Returns:
        Formatted size string
    """
    if bytes_size < 1024:
        return f"{bytes_size} B"
    elif bytes_size < 1024 * 1024:
        return f"{bytes_size / 1024:.1f} KB"
    elif bytes_size < 1024 * 1024 * 1024:
        return f"{bytes_size / (1024 * 1024):.1f} MB"
    else:
        return f"{bytes_size / (1024 * 1024 * 1024):.1f} GB"


def safe_json_load(file_path: Union[str, Path], default: Any = None) -> Any:
    """
    Safely load JSON from a file
    
    Args:
        file_path: Path to JSON file
        default: Default value if loading fails
        
    Returns:
        Loaded data or default value
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            return json.load(f)
    except Exception as e:
        logger.warning(f"Failed to load JSON from {file_path}: {e}")
        return default


def safe_json_save(data: Any, file_path: Union[str, Path], indent: int = 2) -> bool:
    """
    Safely save data to JSON file
    
    Args:
        data: Data to save
        file_path: Path to save to
        indent: JSON indentation
        
    Returns:
        True if successful, False otherwise
    """
    try:
        file_path = Path(file_path)
        file_path.parent.mkdir(parents=True, exist_ok=True)
        
        with open(file_path, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=indent, ensure_ascii=False)
        return True
    except Exception as e:
        logger.error(f"Failed to save JSON to {file_path}: {e}")
        return False


def generate_cache_key(*args, **kwargs) -> str:
    """
    Generate a cache key from arguments
    
    Args:
        *args: Positional arguments
        **kwargs: Keyword arguments
        
    Returns:
        Cache key string
    """
    # Create a string representation of the arguments
    key_parts = [str(arg) for arg in args]
    key_parts.extend([f"{k}={v}" for k, v in sorted(kwargs.items())])
    
    # Join and hash
    key_string = "|".join(key_parts)
    return hashlib.md5(key_string.encode()).hexdigest()


def retry_operation(operation, max_attempts: int = 3, delay: float = 1.0, 
                   backoff_factor: float = 2.0, exceptions: tuple = (Exception,)):
    """
    Retry an operation with exponential backoff
    
    Args:
        operation: Function to retry
        max_attempts: Maximum number of attempts
        delay: Initial delay between attempts
        backoff_factor: Multiplier for delay on each retry
        exceptions: Tuple of exceptions to catch
        
    Returns:
        Result of operation if successful
        
    Raises:
        Last exception if all attempts fail
    """
    last_exception = None
    current_delay = delay
    
    for attempt in range(max_attempts):
        try:
            return operation()
        except exceptions as e:
            last_exception = e
            if attempt < max_attempts - 1:
                logger.warning(f"Operation failed (attempt {attempt + 1}/{max_attempts}): {e}")
                time.sleep(current_delay)
                current_delay *= backoff_factor
            else:
                logger.error(f"Operation failed after {max_attempts} attempts: {e}")
                
    raise last_exception


def ensure_directory(path: Union[str, Path]) -> bool:
    """
    Ensure a directory exists, creating it if necessary
    
    Args:
        path: Directory path
        
    Returns:
        True if successful, False otherwise
    """
    try:
        Path(path).mkdir(parents=True, exist_ok=True)
        return True
    except Exception as e:
        logger.error(f"Failed to create directory {path}: {e}")
        return False


def get_file_extension(file_path: Union[str, Path]) -> str:
    """
    Get file extension from path
    
    Args:
        file_path: File path
        
    Returns:
        File extension (without dot)
    """
    return Path(file_path).suffix.lstrip('.')


def is_file_recent(file_path: Union[str, Path], max_age_hours: float = 24.0) -> bool:
    """
    Check if a file was modified recently
    
    Args:
        file_path: File path to check
        max_age_hours: Maximum age in hours
        
    Returns:
        True if file is recent, False otherwise
    """
    try:
        file_path = Path(file_path)
        if not file_path.exists():
            return False
            
        file_age = time.time() - file_path.stat().st_mtime
        max_age_seconds = max_age_hours * 3600
        
        return file_age <= max_age_seconds
    except Exception:
        return False


def sanitize_filename(filename: str) -> str:
    """
    Sanitize a filename for safe filesystem use
    
    Args:
        filename: Original filename
        
    Returns:
        Sanitized filename
    """
    import re
    
    # Remove or replace invalid characters
    filename = re.sub(r'[<>:"/\\|?*]', '_', filename)
    
    # Remove leading/trailing spaces and dots
    filename = filename.strip(' .')
    
    # Limit length
    if len(filename) > 255:
        filename = filename[:255]
        
    # Ensure filename is not empty
    if not filename:
        filename = "unnamed_file"
        
    return filename


def merge_dicts(dict1: Dict, dict2: Dict) -> Dict:
    """
    Recursively merge two dictionaries
    
    Args:
        dict1: First dictionary
        dict2: Second dictionary (overrides dict1)
        
    Returns:
        Merged dictionary
    """
    result = dict1.copy()
    
    for key, value in dict2.items():
        if key in result and isinstance(result[key], dict) and isinstance(value, dict):
            result[key] = merge_dicts(result[key], value)
        else:
            result[key] = value
            
    return result


def flatten_dict(d: Dict, parent_key: str = '', sep: str = '.') -> Dict:
    """
    Flatten a nested dictionary
    
    Args:
        d: Dictionary to flatten
        parent_key: Parent key for nested items
        sep: Separator for nested keys
        
    Returns:
        Flattened dictionary
    """
    items = []
    
    for k, v in d.items():
        new_key = f"{parent_key}{sep}{k}" if parent_key else k
        
        if isinstance(v, dict):
            items.extend(flatten_dict(v, new_key, sep=sep).items())
        else:
            items.append((new_key, v))
            
    return dict(items)


def chunk_list(lst: List, chunk_size: int) -> List[List]:
    """
    Split a list into chunks of specified size
    
    Args:
        lst: List to chunk
        chunk_size: Size of each chunk
        
    Returns:
        List of chunks
    """
    return [lst[i:i + chunk_size] for i in range(0, len(lst), chunk_size)]


def safe_int(value: Any, default: int = 0) -> int:
    """
    Safely convert value to integer
    
    Args:
        value: Value to convert
        default: Default value if conversion fails
        
    Returns:
        Integer value or default
    """
    try:
        return int(value)
    except (ValueError, TypeError):
        return default


def safe_float(value: Any, default: float = 0.0) -> float:
    """
    Safely convert value to float
    
    Args:
        value: Value to convert
        default: Default value if conversion fails
        
    Returns:
        Float value or default
    """
    try:
        return float(value)
    except (ValueError, TypeError):
        return default


def truncate_text(text: str, max_length: int, suffix: str = "...") -> str:
    """
    Truncate text to specified length
    
    Args:
        text: Text to truncate
        max_length: Maximum length
        suffix: Suffix to add if truncated
        
    Returns:
        Truncated text
    """
    if len(text) <= max_length:
        return text
        
    return text[:max_length - len(suffix)] + suffix
