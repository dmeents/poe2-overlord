"""
Event Bus System for POE2 Master Overlay

Provides a centralized event system for loose coupling between components.
"""

import threading
from typing import Any, Callable, Dict, List, Optional, Set
from dataclasses import dataclass
from enum import Enum
import logging

logger = logging.getLogger(__name__)


class EventType(Enum):
    """Standard event types for the overlay system"""
    # Process events
    POE2_STARTED = "poe2_started"
    POE2_STOPPED = "poe2_stopped"
    
    # UI events
    OVERLAY_SHOW = "overlay_show"
    OVERLAY_HIDE = "overlay_hide"
    OVERLAY_TOGGLE = "overlay_toggle"
    
    # Search events
    SEARCH_STARTED = "search_started"
    SEARCH_COMPLETED = "search_completed"
    SEARCH_FAILED = "search_failed"
    
    # Configuration events
    CONFIG_CHANGED = "config_changed"
    CONFIG_LOADED = "config_loaded"
    
    # Hotkey events
    HOTKEY_TRIGGERED = "hotkey_triggered"
    
    # Plugin events
    PLUGIN_LOADED = "plugin_loaded"
    PLUGIN_UNLOADED = "plugin_unloaded"


@dataclass
class Event:
    """Represents an event in the system"""
    type: EventType
    data: Optional[Dict[str, Any]] = None
    source: Optional[str] = None
    timestamp: Optional[float] = None
    
    def __post_init__(self):
        if self.timestamp is None:
            import time
            self.timestamp = time.time()


class EventBus:
    """Centralized event bus for component communication"""
    
    def __init__(self):
        self._listeners: Dict[EventType, Set[Callable[[Event], None]]] = {}
        self._global_listeners: List[Callable[[Event], None]] = []
        self._lock = threading.RLock()
        self._enabled = True
        
    def subscribe(self, event_type: EventType, callback: Callable[[Event], None]) -> None:
        """Subscribe to a specific event type"""
        with self._lock:
            if event_type not in self._listeners:
                self._listeners[event_type] = set()
            self._listeners[event_type].add(callback)
            logger.debug(f"Subscribed to event: {event_type.value}")
            
    def subscribe_all(self, callback: Callable[[Event], None]) -> None:
        """Subscribe to all events"""
        with self._lock:
            self._global_listeners.append(callback)
            logger.debug("Subscribed to all events")
            
    def unsubscribe(self, event_type: EventType, callback: Callable[[Event], None]) -> None:
        """Unsubscribe from a specific event type"""
        with self._lock:
            if event_type in self._listeners:
                self._listeners[event_type].discard(callback)
                if not self._listeners[event_type]:
                    del self._listeners[event_type]
                logger.debug(f"Unsubscribed from event: {event_type.value}")
                
    def unsubscribe_all(self, callback: Callable[[Event], None]) -> None:
        """Unsubscribe from all events"""
        with self._lock:
            self._global_listeners = [cb for cb in self._global_listeners if cb != callback]
            for event_type in list(self._listeners.keys()):
                self._listeners[event_type].discard(callback)
                if not self._listeners[event_type]:
                    del self._listeners[event_type]
            logger.debug("Unsubscribed from all events")
            
    def publish(self, event: Event) -> None:
        """Publish an event to all subscribers"""
        if not self._enabled:
            return
            
        with self._lock:
            # Notify specific event listeners
            if event.type in self._listeners:
                for callback in self._listeners[event.type].copy():
                    try:
                        callback(event)
                    except Exception as e:
                        logger.error(f"Error in event callback for {event.type.value}: {e}")
                        
            # Notify global listeners
            for callback in self._global_listeners.copy():
                try:
                    callback(event)
                except Exception as e:
                    logger.error(f"Error in global event callback: {e}")
                    
        logger.debug(f"Published event: {event.type.value} from {event.source or 'unknown'}")
        
    def publish_simple(self, event_type: EventType, data: Optional[Dict[str, Any]] = None, 
                      source: Optional[str] = None) -> None:
        """Publish a simple event with just type and optional data"""
        event = Event(type=event_type, data=data, source=source)
        self.publish(event)
        
    def disable(self) -> None:
        """Disable the event bus"""
        self._enabled = False
        logger.info("Event bus disabled")
        
    def enable(self) -> None:
        """Enable the event bus"""
        self._enabled = True
        logger.info("Event bus enabled")
        
    def clear(self) -> None:
        """Clear all listeners"""
        with self._lock:
            self._listeners.clear()
            self._global_listeners.clear()
            logger.info("Event bus cleared")
            
    def get_listener_count(self, event_type: Optional[EventType] = None) -> int:
        """Get the number of listeners for an event type or total"""
        with self._lock:
            if event_type:
                return len(self._listeners.get(event_type, set()))
            else:
                total = len(self._global_listeners)
                for listeners in self._listeners.values():
                    total += len(listeners)
                return total


# Global event bus instance
event_bus = EventBus()
