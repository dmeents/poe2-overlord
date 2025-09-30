import { invoke } from '@tauri-apps/api/core';
import { useCallback, useEffect, useState } from 'react';
import type { WalkthroughGuide } from '../types/walkthrough';

/**
 * Hook for loading the walkthrough guide data
 *
 * @returns Object containing guide data, loading state, and error state
 */
export function useWalkthroughGuide() {
  const [guide, setGuide] = useState<WalkthroughGuide | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const loadGuide = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

      const guideResponse = await invoke<WalkthroughGuide>(
        'get_walkthrough_guide'
      );
      console.log('Loaded walkthrough guide:', guideResponse);
      console.log(
        'Available step IDs:',
        Object.values(guideResponse.acts)
          .map(act => Object.keys(act.steps))
          .flat()
      );
      setGuide(guideResponse);
    } catch (err) {
      console.error('Failed to load walkthrough guide:', err);
      setError('Failed to load walkthrough guide. Please try again.');
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadGuide();
  }, [loadGuide]);

  return {
    guide,
    loading,
    error,
    refetch: loadGuide,
  };
}
