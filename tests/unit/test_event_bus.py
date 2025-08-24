"""
Unit tests for the Event Bus system
"""

import pytest
import time
from unittest.mock import Mock, patch

from src.core.event_bus import EventBus, EventType, Event


class TestEvent:
    """Test the Event class"""
    
    def test_event_creation(self):
        """Test basic event creation"""
        event = Event(EventType.POE2_STARTED, {"data": "test"})
        assert event.type == EventType.POE2_STARTED
        assert event.data == {"data": "test"}
        assert event.timestamp is not None
        assert event.source is None
        
    def test_event_with_source(self):
        """Test event creation with source"""
        event = Event(EventType.POE2_STARTED, source="TestSource")
        assert event.source == "TestSource"
        
    def test_event_timestamp(self):
        """Test event timestamp handling"""
        timestamp = time.time()
        event = Event(EventType.POE2_STARTED, timestamp=timestamp)
        assert event.timestamp == timestamp


class TestEventBus:
    """Test the EventBus class"""
    
    def test_event_bus_initialization(self):
        """Test event bus initialization"""
        bus = EventBus()
        assert bus._enabled is True
        assert len(bus._listeners) == 0
        assert len(bus._global_listeners) == 0
        
    def test_subscribe_to_specific_event(self):
        """Test subscribing to a specific event type"""
        bus = EventBus()
        callback = Mock()
        
        bus.subscribe(EventType.POE2_STARTED, callback)
        
        assert EventType.POE2_STARTED in bus._listeners
        assert callback in bus._listeners[EventType.POE2_STARTED]
        
    def test_subscribe_to_all_events(self):
        """Test subscribing to all events"""
        bus = EventBus()
        callback = Mock()
        
        bus.subscribe_all(callback)
        
        assert callback in bus._global_listeners
        
    def test_unsubscribe_from_specific_event(self):
        """Test unsubscribing from a specific event type"""
        bus = EventBus()
        callback = Mock()
        
        bus.subscribe(EventType.POE2_STARTED, callback)
        bus.unsubscribe(EventType.POE2_STARTED, callback)
        
        assert EventType.POE2_STARTED not in bus._listeners
        
    def test_unsubscribe_from_all_events(self):
        """Test unsubscribing from all events"""
        bus = EventBus()
        callback = Mock()
        
        bus.subscribe_all(callback)
        bus.subscribe(EventType.POE2_STARTED, callback)
        bus.unsubscribe_all(callback)
        
        assert callback not in bus._global_listeners
        assert EventType.POE2_STARTED not in bus._listeners
        
    def test_publish_event(self):
        """Test publishing an event"""
        bus = EventBus()
        callback = Mock()
        
        bus.subscribe(EventType.POE2_STARTED, callback)
        
        event = Event(EventType.POE2_STARTED, {"data": "test"})
        bus.publish(event)
        
        callback.assert_called_once_with(event)
        
    def test_publish_simple_event(self):
        """Test publishing a simple event"""
        bus = EventBus()
        callback = Mock()
        
        bus.subscribe(EventType.POE2_STARTED, callback)
        bus.publish_simple(EventType.POE2_STARTED, {"data": "test"})
        
        callback.assert_called_once()
        call_args = callback.call_args[0][0]
        assert call_args.type == EventType.POE2_STARTED
        assert call_args.data == {"data": "test"}
        
    def test_publish_to_global_listeners(self):
        """Test publishing to global listeners"""
        bus = EventBus()
        callback = Mock()
        
        bus.subscribe_all(callback)
        
        event = Event(EventType.POE2_STARTED, {"data": "test"})
        bus.publish(event)
        
        callback.assert_called_once_with(event)
        
    def test_publish_disabled_bus(self):
        """Test publishing to disabled event bus"""
        bus = EventBus()
        callback = Mock()
        
        bus.subscribe(EventType.POE2_STARTED, callback)
        bus.disable()
        
        event = Event(EventType.POE2_STARTED, {"data": "test"})
        bus.publish(event)
        
        callback.assert_not_called()
        
    def test_enable_disable_bus(self):
        """Test enabling and disabling the event bus"""
        bus = EventBus()
        
        bus.disable()
        assert bus._enabled is False
        
        bus.enable()
        assert bus._enabled is True
        
    def test_clear_bus(self):
        """Test clearing all listeners"""
        bus = EventBus()
        callback = Mock()
        
        bus.subscribe(EventType.POE2_STARTED, callback)
        bus.subscribe_all(callback)
        
        bus.clear()
        
        assert len(bus._listeners) == 0
        assert len(bus._global_listeners) == 0
        
    def test_get_listener_count(self):
        """Test getting listener counts"""
        bus = EventBus()
        callback1 = Mock()
        callback2 = Mock()
        
        bus.subscribe(EventType.POE2_STARTED, callback1)
        bus.subscribe(EventType.POE2_STOPPED, callback2)
        bus.subscribe_all(callback1)
        
        assert bus.get_listener_count() == 3
        assert bus.get_listener_count(EventType.POE2_STARTED) == 1
        assert bus.get_listener_count(EventType.POE2_STOPPED) == 1
        
    def test_publish_with_callback_error(self):
        """Test publishing when a callback raises an exception"""
        bus = EventBus()
        
        def error_callback(event):
            raise ValueError("Test error")
            
        def good_callback(event):
            pass
            
        bus.subscribe(EventType.POE2_STARTED, error_callback)
        bus.subscribe(EventType.POE2_STARTED, good_callback)
        
        event = Event(EventType.POE2_STARTED, {"data": "test"})
        
        # Should not raise an exception
        bus.publish(event)
        
    def test_multiple_subscribers(self):
        """Test multiple subscribers to the same event"""
        bus = EventBus()
        callback1 = Mock()
        callback2 = Mock()
        
        bus.subscribe(EventType.POE2_STARTED, callback1)
        bus.subscribe(EventType.POE2_STARTED, callback2)
        
        event = Event(EventType.POE2_STARTED, {"data": "test"})
        bus.publish(event)
        
        callback1.assert_called_once_with(event)
        callback2.assert_called_once_with(event)
        
    def test_thread_safety(self):
        """Test thread safety of the event bus"""
        import threading
        
        bus = EventBus()
        results = []
        
        def subscriber(event):
            results.append(event.data['value'])
            
        bus.subscribe(EventType.POE2_STARTED, subscriber)
        
        def publish_events():
            for i in range(100):
                bus.publish_simple(EventType.POE2_STARTED, {"value": i})
                
        threads = [threading.Thread(target=publish_events) for _ in range(5)]
        
        for thread in threads:
            thread.start()
            
        for thread in threads:
            thread.join()
            
        # All events should be processed
        assert len(results) == 500
        assert set(results) == set(range(100))
