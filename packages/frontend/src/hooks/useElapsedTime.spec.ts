import { act, renderHook } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { useElapsedTime } from './useElapsedTime';

describe('useElapsedTime', () => {
  beforeEach(() => {
    vi.useFakeTimers();
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('Active Timer', () => {
    it('returns base duration + elapsed time since entry', () => {
      const now = new Date('2024-01-10T12:00:00Z');
      vi.setSystemTime(now);

      const entryTimestamp = new Date('2024-01-10T11:59:30Z').toISOString(); // 30 seconds ago
      const baseDuration = 100; // 100 seconds already accumulated

      const { result } = renderHook(() => useElapsedTime({ entryTimestamp, baseDuration }));

      // Should be 100 + 30 = 130 seconds
      expect(result.current).toBe(130);
    });

    it('updates elapsed time every second', () => {
      const now = new Date('2024-01-10T12:00:00Z');
      vi.setSystemTime(now);

      const entryTimestamp = now.toISOString();
      const baseDuration = 100;

      const { result } = renderHook(() => useElapsedTime({ entryTimestamp, baseDuration }));

      expect(result.current).toBe(100); // 0 elapsed

      // Advance 1 second
      act(() => {
        vi.advanceTimersByTime(1000);
      });
      expect(result.current).toBe(101);

      // Advance 5 more seconds
      act(() => {
        vi.advanceTimersByTime(5000);
      });
      expect(result.current).toBe(106);
    });

    it('handles custom update interval', () => {
      const now = new Date('2024-01-10T12:00:00Z');
      vi.setSystemTime(now);

      const entryTimestamp = now.toISOString();
      const baseDuration = 0;

      const { result } = renderHook(() =>
        useElapsedTime({ entryTimestamp, baseDuration, intervalMs: 500 }),
      );

      expect(result.current).toBe(0);

      // Advance 500ms (should trigger update)
      act(() => {
        vi.advanceTimersByTime(500);
      });
      expect(result.current).toBe(0); // Still 0 (less than 1 second elapsed)

      // Advance to 1 second total
      act(() => {
        vi.advanceTimersByTime(500);
      });
      expect(result.current).toBe(1);
    });
  });

  describe('Inactive Timer', () => {
    it('returns only base duration when no entry timestamp', () => {
      const { result } = renderHook(() =>
        useElapsedTime({ entryTimestamp: undefined, baseDuration: 100 }),
      );

      expect(result.current).toBe(100);

      // Advance time - should not change
      act(() => {
        vi.advanceTimersByTime(5000);
      });
      expect(result.current).toBe(100);
    });

    it('returns only base duration when isActive is false', () => {
      const now = new Date('2024-01-10T12:00:00Z');
      vi.setSystemTime(now);

      const { result } = renderHook(() =>
        useElapsedTime({
          entryTimestamp: now.toISOString(),
          baseDuration: 100,
          isActive: false,
        }),
      );

      expect(result.current).toBe(100);

      act(() => {
        vi.advanceTimersByTime(5000);
      });
      expect(result.current).toBe(100);
    });
  });

  describe('Timer Reset on Entry Change', () => {
    it('resets timer when entry timestamp changes', () => {
      const now = new Date('2024-01-10T12:00:00Z');
      vi.setSystemTime(now);

      const firstEntry = new Date('2024-01-10T11:59:00Z').toISOString();

      const { result, rerender } = renderHook(
        ({ entryTimestamp, baseDuration }) => useElapsedTime({ entryTimestamp, baseDuration }),
        {
          initialProps: {
            entryTimestamp: firstEntry,
            baseDuration: 100,
          },
        },
      );

      // Should show 100 + 60 = 160
      expect(result.current).toBe(160);

      // Enter new zone (new entry timestamp, new base duration)
      const newNow = new Date('2024-01-10T12:01:00Z');
      vi.setSystemTime(newNow);

      rerender({
        entryTimestamp: newNow.toISOString(),
        baseDuration: 50,
      });

      // Should reset to new base duration (0 elapsed in new zone)
      expect(result.current).toBe(50);

      // Advance 3 seconds in new zone
      act(() => {
        vi.advanceTimersByTime(3000);
      });
      expect(result.current).toBe(53);
    });
  });

  describe('Edge Cases', () => {
    it('handles entry timestamp in the future gracefully', () => {
      const now = new Date('2024-01-10T12:00:00Z');
      vi.setSystemTime(now);

      // Entry timestamp 10 seconds in the future (clock skew)
      const futureEntry = new Date('2024-01-10T12:00:10Z').toISOString();

      const { result } = renderHook(() =>
        useElapsedTime({ entryTimestamp: futureEntry, baseDuration: 100 }),
      );

      // Should use Math.max(0, ...) to prevent negative elapsed time
      expect(result.current).toBe(100); // baseDuration only
    });

    it('cleans up interval on unmount', () => {
      const now = new Date('2024-01-10T12:00:00Z');
      vi.setSystemTime(now);

      const clearIntervalSpy = vi.spyOn(global, 'clearInterval');

      const { unmount } = renderHook(() =>
        useElapsedTime({
          entryTimestamp: now.toISOString(),
          baseDuration: 0,
        }),
      );

      unmount();

      expect(clearIntervalSpy).toHaveBeenCalled();
    });

    it('handles zero base duration correctly', () => {
      const now = new Date('2024-01-10T12:00:00Z');
      vi.setSystemTime(now);

      const entryTimestamp = new Date('2024-01-10T11:59:55Z').toISOString(); // 5 seconds ago

      const { result } = renderHook(() => useElapsedTime({ entryTimestamp, baseDuration: 0 }));

      expect(result.current).toBe(5);
    });

    it('handles very large base duration correctly', () => {
      const now = new Date('2024-01-10T12:00:00Z');
      vi.setSystemTime(now);

      const entryTimestamp = now.toISOString();
      const baseDuration = 86400; // 24 hours in seconds

      const { result } = renderHook(() => useElapsedTime({ entryTimestamp, baseDuration }));

      expect(result.current).toBe(86400);

      act(() => {
        vi.advanceTimersByTime(1000);
      });
      expect(result.current).toBe(86401);
    });
  });
});
