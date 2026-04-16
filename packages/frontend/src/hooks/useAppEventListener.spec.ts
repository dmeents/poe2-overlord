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

  it('does not register listeners for an empty array', async () => {
    renderHook(() => useAppEventListener([]));
    await waitFor(() => {
      expect(eventListener.listenToAppEvent).not.toHaveBeenCalled();
    });
  });

  it('registers event listeners on mount', async () => {
    const listeners = [
      { eventType: 'CharacterUpdated' as const, handler: mockHandler },
      { eventType: 'CharacterDeleted' as const, handler: mockHandler },
    ];

    renderHook(() => useAppEventListener(listeners));

    // Wait for async setup
    await waitFor(() => {
      expect(eventListener.listenToAppEvent).toHaveBeenCalledTimes(2);
    });
  });

  it('registers a single listener on successful setup', async () => {
    const listeners = [{ eventType: 'CharacterUpdated' as const, handler: mockHandler }];

    renderHook(() => useAppEventListener(listeners));

    await waitFor(() => {
      expect(eventListener.listenToAppEvent).toHaveBeenCalledTimes(1);
      expect(eventListener.listenToAppEvent).toHaveBeenCalledWith(
        'CharacterUpdated',
        expect.any(Function),
      );
    });
  });

  it('registers multiple listeners simultaneously', async () => {
    const listeners = [
      { eventType: 'CharacterUpdated' as const, handler: mockHandler },
      { eventType: 'CharacterDeleted' as const, handler: mockHandler },
    ];

    renderHook(() => useAppEventListener(listeners));

    await waitFor(() => {
      expect(eventListener.listenToAppEvent).toHaveBeenCalledTimes(2);
      expect(eventListener.listenToAppEvent).toHaveBeenCalledWith(
        'CharacterUpdated',
        expect.any(Function),
      );
      expect(eventListener.listenToAppEvent).toHaveBeenCalledWith(
        'CharacterDeleted',
        expect.any(Function),
      );
    });
  });

  it('handles empty listeners array without errors', async () => {
    const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
    renderHook(() => useAppEventListener([]));
    await waitFor(() => {
      expect(eventListener.listenToAppEvent).not.toHaveBeenCalled();
    });
    expect(consoleErrorSpy).not.toHaveBeenCalled();
    consoleErrorSpy.mockRestore();
  });

  it('re-registers listeners when deps change', async () => {
    const listeners = [{ eventType: 'CharacterUpdated' as const, handler: mockHandler }];

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

    const listeners = [{ eventType: 'CharacterUpdated' as const, handler: mockHandler }];

    renderHook(() => useAppEventListener(listeners));

    await waitFor(() => {
      expect(consoleErrorSpy).toHaveBeenCalled();
    });

    consoleErrorSpy.mockRestore();
  });
});
