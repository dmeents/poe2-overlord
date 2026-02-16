import { renderHook, waitFor } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import * as eventListener from '@/utils/events/listener';
import { useAppEventListener } from './useAppEventListener';

// Mock the event listener module
vi.mock('@/utils/events/listener', () => ({
  listenToAppEvent: vi.fn(),
}));

describe('useAppEventListener', () => {
  const mockUnlisten = vi.fn();
  const mockHandler = vi.fn();

  beforeEach(() => {
    vi.clearAllMocks();
    (eventListener.listenToAppEvent as ReturnType<typeof vi.fn>).mockResolvedValue(mockUnlisten);
  });

  afterEach(() => {
    vi.clearAllMocks();
  });

  it('sets isListening to false initially', async () => {
    const { result } = renderHook(() => useAppEventListener([]));
    // Wait for any async state updates to complete
    await waitFor(() => {
      expect(result.current.isListening).toBe(false);
    });
  });

  it('registers event listeners on mount', async () => {
    const listeners = [
      { eventType: 'character:created' as const, handler: mockHandler },
      { eventType: 'character:updated' as const, handler: mockHandler },
    ];

    renderHook(() => useAppEventListener(listeners));

    // Wait for async setup
    await waitFor(() => {
      expect(eventListener.listenToAppEvent).toHaveBeenCalledTimes(2);
    });
  });

  it('sets isListening to true after successful setup', async () => {
    const listeners = [{ eventType: 'character:created' as const, handler: mockHandler }];

    const { result } = renderHook(() => useAppEventListener(listeners));

    await waitFor(() => {
      expect(result.current.isListening).toBe(true);
    });
  });

  it('registers multiple listeners simultaneously', async () => {
    const listeners = [
      { eventType: 'character:created' as const, handler: mockHandler },
      { eventType: 'character:updated' as const, handler: mockHandler },
    ];

    renderHook(() => useAppEventListener(listeners));

    await waitFor(() => {
      expect(eventListener.listenToAppEvent).toHaveBeenCalledTimes(2);
      expect(eventListener.listenToAppEvent).toHaveBeenCalledWith(
        'character:created',
        expect.any(Function),
      );
      expect(eventListener.listenToAppEvent).toHaveBeenCalledWith(
        'character:updated',
        expect.any(Function),
      );
    });
  });

  it('handles empty listeners array', async () => {
    const { result } = renderHook(() => useAppEventListener([]));
    await waitFor(() => {
      expect(result.current.isListening).toBe(false);
    });
  });

  it('re-registers listeners when deps change', async () => {
    const listeners = [{ eventType: 'character:created' as const, handler: mockHandler }];

    const { rerender } = renderHook(({ deps }) => useAppEventListener(listeners, deps), {
      initialProps: { deps: [1] },
    });

    await waitFor(() => {
      expect(eventListener.listenToAppEvent).toHaveBeenCalledTimes(1);
    });

    // Change deps should trigger re-registration
    rerender({ deps: [2] });

    await waitFor(() => {
      expect(eventListener.listenToAppEvent).toHaveBeenCalledTimes(2);
    });
  });

  it('handles setup errors gracefully', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    (eventListener.listenToAppEvent as ReturnType<typeof vi.fn>).mockRejectedValue(
      new Error('Setup failed'),
    );

    const listeners = [{ eventType: 'character:created' as const, handler: mockHandler }];

    const { result } = renderHook(() => useAppEventListener(listeners));

    await waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalled();
    });

    expect(result.current.isListening).toBe(false);

    consoleErrorSpy.mockRestore();
  });
});
