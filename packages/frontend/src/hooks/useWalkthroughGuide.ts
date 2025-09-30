import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect, useState } from 'react';
import type { WalkthroughGuide } from '../types/walkthrough';
import { DEFAULT_ERROR_CONFIG, useErrorHandling } from './useErrorHandling';

/**
 * Hook for loading the walkthrough guide data
 *
 * @returns Object containing guide data, loading state, and error state
 */
export function useWalkthroughGuide() {
  const [guide, setGuide] = useState<WalkthroughGuide | null>(null);
  const [loading, setLoading] = useState(true);
  const { error, clearError, handleAsyncOperation } =
    useErrorHandling(DEFAULT_ERROR_CONFIG);

  const loadGuide = useCallback(async () => {
    setLoading(true);
    clearError();

    const result = await handleAsyncOperation(async () => {
      const guideResponse = await invoke<WalkthroughGuide>(
        'get_walkthrough_guide'
      );
      setGuide(guideResponse);
      return guideResponse;
    }, 'load walkthrough guide');

    setLoading(false);
    return result;
  }, [handleAsyncOperation, clearError]);

  useEffect(() => {
    loadGuide();
  }, [loadGuide]);

  return {
    guide,
    loading,
    error: error?.message || null,
    refetch: loadGuide,
  };
}
