"""
Input validation utilities for POE2 Master Overlay

Provides validation functions for user input and configuration data.
"""

import re
from typing import Any, Dict, List, Optional, Union
import logging

logger = logging.getLogger(__name__)


def validate_item_name(item_name: str) -> bool:
    """
    Validate an item name
    
    Args:
        item_name: Item name to validate
        
    Returns:
        True if valid, False otherwise
    """
    if not item_name or not isinstance(item_name, str):
        return False
        
    # Remove leading/trailing whitespace
    item_name = item_name.strip()
    
    # Check minimum length
    if len(item_name) < 2:
        return False
        
    # Check maximum length
    if len(item_name) > 100:
        return False
        
    # Check for valid characters (letters, numbers, spaces, hyphens, apostrophes)
    if not re.match(r'^[a-zA-Z0-9\s\-\']+$', item_name):
        return False
        
    return True


def validate_price(price: Union[str, int, float]) -> bool:
    """
    Validate a price value
    
    Args:
        price: Price to validate
        
    Returns:
        True if valid, False otherwise
    """
    if isinstance(price, (int, float)):
        return price >= 0
        
    if isinstance(price, str):
        # Check if it's a valid number
        try:
            float(price)
            return True
        except ValueError:
            return False
            
    return False


def validate_currency(currency: str) -> bool:
    """
    Validate a currency name
    
    Args:
        currency: Currency name to validate
        
    Returns:
        True if valid, False otherwise
    """
    if not currency or not isinstance(currency, str):
        return False
        
    # Common POE2 currencies
    valid_currencies = [
        'Chaos Orb', 'Divine Orb', 'Exalted Orb', 'Mirror of Kalandra',
        'Alchemy Orb', 'Chromatic Orb', 'Jeweller\'s Orb', 'Fusing Orb',
        'Regal Orb', 'Vaal Orb', 'Scouring Orb', 'Blessed Orb',
        'Regret Orb', 'Gemcutter\'s Prism', 'Glassblower\'s Bauble'
    ]
    
    return currency.strip() in valid_currencies


def validate_league(league: str) -> bool:
    """
    Validate a league name
    
    Args:
        league: League name to validate
        
    Returns:
        True if valid, False otherwise
    """
    if not league or not isinstance(league, str):
        return False
        
    # Common POE2 leagues
    valid_leagues = [
        'Standard', 'Hardcore', 'Early Access', 'Challenge League',
        'Solo Self-Found', 'Hardcore Solo Self-Found'
    ]
    
    return league.strip() in valid_leagues


def validate_config_value(key: str, value: Any, expected_type: type) -> bool:
    """
    Validate a configuration value
    
    Args:
        key: Configuration key
        value: Value to validate
        expected_type: Expected type
        
    Returns:
        True if valid, False otherwise
    """
    if not isinstance(value, expected_type):
        logger.warning(f"Config key '{key}' expected {expected_type.__name__}, got {type(value).__name__}")
        return False
        
    return True


def validate_config_range(key: str, value: Union[int, float], min_val: Union[int, float], max_val: Union[int, float]) -> bool:
    """
    Validate a configuration value within a range
    
    Args:
        key: Configuration key
        value: Value to validate
        min_val: Minimum allowed value
        max_val: Maximum allowed value
        
    Returns:
        True if valid, False otherwise
    """
    if not isinstance(value, (int, float)):
        logger.warning(f"Config key '{key}' expected number, got {type(value).__name__}")
        return False
        
    if value < min_val or value > max_val:
        logger.warning(f"Config key '{key}' value {value} is outside range [{min_val}, {max_val}]")
        return False
        
    return True


def validate_hotkey(hotkey: str) -> bool:
    """
    Validate a hotkey combination
    
    Args:
        hotkey: Hotkey string to validate
        
    Returns:
        True if valid, False otherwise
    """
    if not hotkey or not isinstance(hotkey, str):
        return False
        
    # Basic hotkey pattern: <modifier>+<key> or <modifier>+<modifier>+<key>
    hotkey_pattern = r'^<[^>]+>(?:\+<[^>]+>)*$'
    
    if not re.match(hotkey_pattern, hotkey):
        return False
        
    # Check for valid modifiers
    valid_modifiers = ['ctrl', 'shift', 'alt', 'super', 'meta']
    valid_keys = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
        'f1', 'f2', 'f3', 'f4', 'f5', 'f6', 'f7', 'f8', 'f9', 'f10', 'f11', 'f12',
        'escape', 'tab', 'space', 'enter', 'backspace', 'delete', 'insert',
        'home', 'end', 'pageup', 'pagedown', 'up', 'down', 'left', 'right'
    ]
    
    # Extract parts from hotkey string
    parts = hotkey.strip('<>').split('><')
    
    for part in parts:
        part = part.strip('<>')
        if part not in valid_modifiers and part not in valid_keys:
            logger.warning(f"Invalid hotkey part: {part}")
            return False
            
    return True


def validate_file_path(path: str, must_exist: bool = False) -> bool:
    """
    Validate a file path
    
    Args:
        path: Path to validate
        must_exist: Whether the file must exist
        
    Returns:
        True if valid, False otherwise
    """
    if not path or not isinstance(path, str):
        return False
        
    try:
        from pathlib import Path
        path_obj = Path(path)
        
        if must_exist and not path_obj.exists():
            return False
            
        # Check if path is absolute or can be resolved
        path_obj.resolve()
        return True
        
    except Exception:
        return False


def validate_url(url: str) -> bool:
    """
    Validate a URL
    
    Args:
        url: URL to validate
        
    Returns:
        True if valid, False otherwise
    """
    if not url or not isinstance(url, str):
        return False
        
    # Basic URL pattern
    url_pattern = r'^https?://[^\s/$.?#].[^\s]*$'
    
    return bool(re.match(url_pattern, url))


def sanitize_item_name(item_name: str) -> str:
    """
    Sanitize an item name for safe use
    
    Args:
        item_name: Item name to sanitize
        
    Returns:
        Sanitized item name
    """
    if not item_name:
        return ""
        
    # Remove leading/trailing whitespace
    item_name = item_name.strip()
    
    # Limit length
    if len(item_name) > 100:
        item_name = item_name[:100]
        
    # Remove potentially dangerous characters
    item_name = re.sub(r'[<>"\']', '', item_name)
    
    return item_name


def validate_search_query(query: str) -> Dict[str, Any]:
    """
    Validate and analyze a search query
    
    Args:
        query: Search query to validate
        
    Returns:
        Dictionary with validation results and analysis
    """
    result = {
        'valid': False,
        'errors': [],
        'warnings': [],
        'query': query,
        'length': len(query) if query else 0,
        'has_special_chars': False,
        'suggestions': []
    }
    
    if not query:
        result['errors'].append("Search query cannot be empty")
        return result
        
    if not isinstance(query, str):
        result['errors'].append("Search query must be a string")
        return result
        
    # Check length
    if len(query) < 2:
        result['errors'].append("Search query must be at least 2 characters")
    elif len(query) > 200:
        result['errors'].append("Search query must be less than 200 characters")
        
    # Check for special characters
    if re.search(r'[<>"\']', query):
        result['has_special_chars'] = True
        result['warnings'].append("Query contains special characters that may affect search")
        
    # Check for common POE2 terms
    poe2_terms = ['unique', 'rare', 'magic', 'normal', 'corrupted', 'enchanted']
    found_terms = [term for term in poe2_terms if term.lower() in query.lower()]
    
    if found_terms:
        result['suggestions'].append(f"Found POE2 terms: {', '.join(found_terms)}")
        
    # Determine if valid
    result['valid'] = len(result['errors']) == 0
    
    return result
