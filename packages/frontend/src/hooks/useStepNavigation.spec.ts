import { invoke } from '@tauri-apps/api/core';
import { renderHook } from '@testing-library/react';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { createMockWalkthroughProgress } from '../test/mock-data';
import { useStepNavigation } from './useStepNavigation';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('useStepNavigation', () => {
  const characterId = 'test-character-id';
  const progress = createMockWalkthroughProgress({ current_step_id: 'step-1' });

  beforeEach(() => {
    vi.clearAllMocks();
    (invoke as ReturnType<typeof vi.fn>).mockResolvedValue(undefined);
  });

  describe('advanceStep', () => {
    it('invokes update command with correct parameters', async () => {
      const { result } = renderHook(() =>
        useStepNavigation({ characterId, progress }),
      );

      await result.current.advanceStep('step-2');

      expect(invoke).toHaveBeenCalledWith('update_character_walkthrough_progress', {
        characterId,
        progress: expect.objectContaining({
          current_step_id: 'step-2',
          is_completed: false,
        }),
      });
    });

    it('does not invoke when characterId is null', async () => {
      const { result } = renderHook(() =>
        useStepNavigation({ characterId: null, progress }),
      );

      await result.current.advanceStep('step-2');

      expect(invoke).not.toHaveBeenCalled();
    });

    it('does not invoke when progress is null', async () => {
      const { result } = renderHook(() =>
        useStepNavigation({ characterId, progress: null }),
      );

      await result.current.advanceStep('step-2');

      expect(invoke).not.toHaveBeenCalled();
    });

    it('does not invoke when nextStepId is null', async () => {
      const consoleWarnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {});

      const { result } = renderHook(() =>
        useStepNavigation({ characterId, progress }),
      );

      await result.current.advanceStep(null);

      expect(invoke).not.toHaveBeenCalled();
      expect(consoleWarnSpy).toHaveBeenCalledWith(
        'No next step available. Campaign may be completed.',
      );

      consoleWarnSpy.mockRestore();
    });

    it('handles errors gracefully', async () => {
      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      (invoke as ReturnType<typeof vi.fn>).mockRejectedValue(new Error('Failed'));

      const { result } = renderHook(() =>
        useStepNavigation({ characterId, progress }),
      );

      await result.current.advanceStep('step-2');

      expect(consoleErrorSpy).toHaveBeenCalledWith('Failed to advance step:', expect.any(Error));

      consoleErrorSpy.mockRestore();
    });
  });

  describe('goToPreviousStep', () => {
    it('invokes update command with correct parameters', async () => {
      const { result } = renderHook(() =>
        useStepNavigation({ characterId, progress }),
      );

      await result.current.goToPreviousStep('step-0');

      expect(invoke).toHaveBeenCalledWith('update_character_walkthrough_progress', {
        characterId,
        progress: expect.objectContaining({
          current_step_id: 'step-0',
          is_completed: false,
        }),
      });
    });

    it('does not invoke when characterId is null', async () => {
      const { result } = renderHook(() =>
        useStepNavigation({ characterId: null, progress }),
      );

      await result.current.goToPreviousStep('step-0');

      expect(invoke).not.toHaveBeenCalled();
    });

    it('does not invoke when progress is null', async () => {
      const { result } = renderHook(() =>
        useStepNavigation({ characterId, progress: null }),
      );

      await result.current.goToPreviousStep('step-0');

      expect(invoke).not.toHaveBeenCalled();
    });

    it('handles errors gracefully', async () => {
      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      (invoke as ReturnType<typeof vi.fn>).mockRejectedValue(new Error('Failed'));

      const { result } = renderHook(() =>
        useStepNavigation({ characterId, progress }),
      );

      await result.current.goToPreviousStep('step-0');

      expect(consoleErrorSpy).toHaveBeenCalledWith(
        'Failed to go to previous step:',
        expect.any(Error),
      );

      consoleErrorSpy.mockRestore();
    });
  });

  describe('skipToStep', () => {
    it('invokes update command with correct parameters', async () => {
      const { result } = renderHook(() =>
        useStepNavigation({ characterId, progress }),
      );

      await result.current.skipToStep('step-5');

      expect(invoke).toHaveBeenCalledWith('update_character_walkthrough_progress', {
        characterId,
        progress: expect.objectContaining({
          current_step_id: 'step-5',
          is_completed: false,
        }),
      });
    });

    it('does not invoke when characterId is null', async () => {
      const { result } = renderHook(() =>
        useStepNavigation({ characterId: null, progress }),
      );

      await result.current.skipToStep('step-5');

      expect(invoke).not.toHaveBeenCalled();
    });

    it('handles errors gracefully', async () => {
      const consoleErrorSpy = vi.spyOn(console, 'error').mockImplementation(() => {});
      (invoke as ReturnType<typeof vi.fn>).mockRejectedValue(new Error('Failed'));

      const { result } = renderHook(() =>
        useStepNavigation({ characterId, progress }),
      );

      await result.current.skipToStep('step-5');

      expect(consoleErrorSpy).toHaveBeenCalledWith('Failed to skip to step:', expect.any(Error));

      consoleErrorSpy.mockRestore();
    });
  });
});
