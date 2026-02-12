import { useCallback, useEffect, useState } from 'react';

interface UseElapsedTimeOptions {
  /** ISO 8601 timestamp when timer started (e.g., zone entry time) */
  entryTimestamp: string | undefined;
  /** Base duration in seconds already accumulated */
  baseDuration: number;
  /** Whether the timer should be active (defaults to true if entryTimestamp exists) */
  isActive?: boolean;
  /** Update interval in milliseconds (defaults to 1000ms = 1 second) */
  intervalMs?: number;
}

/**
 * Hook to calculate and update elapsed time in real-time.
 *
 * @param options - Configuration for elapsed time calculation
 * @returns Current elapsed time in seconds (baseDuration + time since entryTimestamp)
 *
 * @example
 * const elapsedSeconds = useElapsedTime({
 *   entryTimestamp: zone.entry_timestamp,
 *   baseDuration: zone.duration,
 * });
 */
export function useElapsedTime({
  entryTimestamp,
  baseDuration,
  isActive = true,
  intervalMs = 1000,
}: UseElapsedTimeOptions): number {
  // Calculate elapsed time from entry timestamp
  const calculateElapsed = useCallback((): number => {
    if (!entryTimestamp || !isActive) {
      return baseDuration;
    }

    const entryTime = new Date(entryTimestamp).getTime();
    const now = Date.now();
    const elapsedMs = now - entryTime;
    const elapsedSeconds = Math.floor(elapsedMs / 1000);

    // Prevent negative values (handles future timestamps/clock skew)
    return baseDuration + Math.max(0, elapsedSeconds);
  }, [entryTimestamp, baseDuration, isActive]);

  const [elapsedTime, setElapsedTime] = useState(calculateElapsed);

  useEffect(() => {
    // If no entry timestamp or not active, just show base duration
    if (!entryTimestamp || !isActive) {
      setElapsedTime(baseDuration);
      return;
    }

    // Update immediately on mount/change
    setElapsedTime(calculateElapsed());

    // Set up interval to update every second
    const intervalId = setInterval(() => {
      setElapsedTime(calculateElapsed());
    }, intervalMs);

    // Cleanup interval on unmount or when dependencies change
    return () => {
      clearInterval(intervalId);
    };
  }, [entryTimestamp, baseDuration, isActive, intervalMs, calculateElapsed]);

  return elapsedTime;
}
