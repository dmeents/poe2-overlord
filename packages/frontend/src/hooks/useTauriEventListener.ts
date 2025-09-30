import { listen } from '@tauri-apps/api/event';
import { useCallback, useEffect, useRef, useState } from 'react';

/**
 * Configuration for a Tauri event listener
 * @template T - The type of the event payload
 */
export interface EventListenerConfig<T = unknown> {
  /** The event name to listen for */
  eventName: string;
  /** Function to handle the event payload */
  handler: (payload: T) => void;
  /** Optional function to get initial data */
  getInitialData?: () => Promise<T | null>;
  /** Whether to enable the listener (defaults to true) */
  enabled?: boolean;
}

/**
 * Configuration for multiple event listeners
 * @template T - The type of the event payload
 */
export interface MultiEventListenerConfig<T = unknown> {
  /** Array of event listener configurations */
  listeners: EventListenerConfig<T>[];
  /** Optional function to get initial data for all listeners */
  getInitialData?: () => Promise<T | null>;
  /** Whether to enable all listeners (defaults to true) */
  enabled?: boolean;
}

/**
 * Generic hook for listening to Tauri events
 * 
 * This hook provides a reusable pattern for listening to Tauri events with
 * automatic cleanup, error handling, and optional initial data loading.
 * 
 * @template T - The type of the event payload
 * @param config - Configuration object for the event listener
 * @returns Object containing listening state and data
 * 
 * @example
 * ```typescript
 * interface MyEventPayload {
 *   data: string;
 *   timestamp: string;
 * }
 * 
 * const config: EventListenerConfig<MyEventPayload> = {
 *   eventName: 'my-event',
 *   handler: (payload) => {
 *     console.log('Received event:', payload);
 *   },
 *   getInitialData: async () => {
 *     const { invoke } = await import('@tauri-apps/api/core');
 *     return await invoke<MyEventPayload>('get_initial_data');
 *   }
 * };
 * 
 * const { isListening, error } = useTauriEventListener(config);
 * ```
 */
export function useTauriEventListener<T = unknown>(
  config: EventListenerConfig<T>
) {
  const { eventName, handler, getInitialData, enabled = true } = config;
  
  const [isListening, setIsListening] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const listenerRef = useRef<(() => void) | null>(null);
  const isListeningRef = useRef(false);

  const setupListener = useCallback(async () => {
    // Clean up existing listener
    if (listenerRef.current) {
      listenerRef.current();
      listenerRef.current = null;
    }

    // Prevent multiple listeners
    if (isListeningRef.current || !enabled) {
      return;
    }

    isListeningRef.current = true;
    setError(null);

    try {
      // Set up event listener
      const unlisten = await listen<T>(eventName, (event) => {
        try {
          handler(event.payload);
        } catch (err) {
          console.error(`Error handling event ${eventName}:`, err);
          setError(`Error handling event: ${err instanceof Error ? err.message : 'Unknown error'}`);
        }
      });

      listenerRef.current = unlisten;
      setIsListening(true);

      // Load initial data if provided
      if (getInitialData) {
        try {
          const initialData = await getInitialData();
          if (initialData !== null) {
            handler(initialData);
          }
        } catch (err) {
          console.warn(`Failed to load initial data for ${eventName}:`, err);
          // Don't set error for initial data failure - events will still work
        }
      }
    } catch (err) {
      console.error(`Failed to set up listener for ${eventName}:`, err);
      setError(`Failed to set up listener: ${err instanceof Error ? err.message : 'Unknown error'}`);
      isListeningRef.current = false;
    }
  }, [eventName, handler, getInitialData, enabled]);

  useEffect(() => {
    setupListener();

    // Cleanup on unmount or dependency change
    return () => {
      if (listenerRef.current) {
        listenerRef.current();
        listenerRef.current = null;
      }
      isListeningRef.current = false;
      setIsListening(false);
    };
  }, [setupListener]);

  return {
    isListening,
    error,
  };
}

/**
 * Generic hook for listening to multiple Tauri events
 * 
 * This hook provides a reusable pattern for listening to multiple Tauri events
 * with automatic cleanup and error handling.
 * 
 * @template T - The type of the event payload
 * @param config - Configuration object for multiple event listeners
 * @returns Object containing listening state and errors
 * 
 * @example
 * ```typescript
 * const config: MultiEventListenerConfig = {
 *   listeners: [
 *     {
 *       eventName: 'event1',
 *       handler: (payload) => console.log('Event 1:', payload),
 *     },
 *     {
 *       eventName: 'event2',
 *       handler: (payload) => console.log('Event 2:', payload),
 *     },
 *   ],
 *   enabled: true
 * };
 * 
 * const { isListening, errors } = useMultiTauriEventListener(config);
 * ```
 */
export function useMultiTauriEventListener<T = unknown>(
  config: MultiEventListenerConfig<T>
) {
  const { listeners, getInitialData, enabled = true } = config;
  
  const [isListening, setIsListening] = useState(false);
  const [errors, setErrors] = useState<Record<string, string>>({});
  const listenerRefs = useRef<Record<string, (() => void) | null>>({});
  const isListeningRef = useRef(false);

  const setupListeners = useCallback(async () => {
    // Clean up existing listeners
    Object.values(listenerRefs.current).forEach(unlisten => {
      if (unlisten) unlisten();
    });
    listenerRefs.current = {};

    // Prevent multiple listeners
    if (isListeningRef.current || !enabled) {
      return;
    }

    isListeningRef.current = true;
    setErrors({});

    try {
      const unlistenFns: (() => void)[] = [];
      const newErrors: Record<string, string> = {};

      // Set up each event listener
      for (const listenerConfig of listeners) {
        const { eventName, handler } = listenerConfig;
        
        try {
          const unlisten = await listen<T>(eventName, (event) => {
            try {
              handler(event.payload);
            } catch (err) {
              console.error(`Error handling event ${eventName}:`, err);
              newErrors[eventName] = `Error handling event: ${err instanceof Error ? err.message : 'Unknown error'}`;
              setErrors(prev => ({ ...prev, ...newErrors }));
            }
          });

          listenerRefs.current[eventName] = unlisten;
          unlistenFns.push(unlisten);
        } catch (err) {
          console.error(`Failed to set up listener for ${eventName}:`, err);
          newErrors[eventName] = `Failed to set up listener: ${err instanceof Error ? err.message : 'Unknown error'}`;
        }
      }

      setErrors(newErrors);
      setIsListening(true);

      // Load initial data if provided
      if (getInitialData) {
        try {
          const initialData = await getInitialData();
          if (initialData !== null) {
            // Call the first listener's handler with initial data
            if (listeners.length > 0) {
              listeners[0].handler(initialData);
            }
          }
        } catch (err) {
          console.warn('Failed to load initial data:', err);
          // Don't set error for initial data failure - events will still work
        }
      }
    } catch (err) {
      console.error('Failed to set up listeners:', err);
      isListeningRef.current = false;
    }
  }, [listeners, getInitialData, enabled]);

  useEffect(() => {
    setupListeners();

    // Cleanup on unmount or dependency change
    return () => {
      Object.values(listenerRefs.current).forEach(unlisten => {
        if (unlisten) unlisten();
      });
      listenerRefs.current = {};
      isListeningRef.current = false;
      setIsListening(false);
    };
  }, [setupListeners]);

  return {
    isListening,
    errors,
  };
}

/**
 * Helper function to create event listener configurations
 * @template T - The type of the event payload
 * @param eventName - The event name to listen for
 * @param handler - Function to handle the event payload
 * @param options - Optional configuration
 * @returns Event listener configuration object
 */
export function createEventListenerConfig<T = unknown>(
  eventName: string,
  handler: (payload: T) => void,
  options?: {
    getInitialData?: () => Promise<T | null>;
    enabled?: boolean;
  }
): EventListenerConfig<T> {
  return {
    eventName,
    handler,
    getInitialData: options?.getInitialData,
    enabled: options?.enabled,
  };
}

/**
 * Helper function to create multi-event listener configurations
 * @template T - The type of the event payload
 * @param listeners - Array of event listener configurations
 * @param options - Optional configuration
 * @returns Multi-event listener configuration object
 */
export function createMultiEventListenerConfig<T = unknown>(
  listeners: EventListenerConfig<T>[],
  options?: {
    getInitialData?: () => Promise<T | null>;
    enabled?: boolean;
  }
): MultiEventListenerConfig<T> {
  return {
    listeners,
    getInitialData: options?.getInitialData,
    enabled: options?.enabled,
  };
}
