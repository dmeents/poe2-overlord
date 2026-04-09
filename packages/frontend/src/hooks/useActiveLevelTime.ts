import { useEffect, useState } from 'react';

/**
 * Tracks accumulated "active" time at the current level.
 *
 * The backend's `activeSecondsAtLevel` already encodes all pause/resume logic
 * (town/hideout filtering, level-up resets, persistence). This hook adds a
 * live segment on top so the display updates in real-time between backend events.
 *
 * The live segment resets whenever `activeSecondsAtLevel` changes (to avoid
 * double-counting time already captured by the backend).
 */
export function useActiveLevelTime({
  lastLevelTimestamp,
  isActive,
  activeSecondsAtLevel,
}: {
  lastLevelTimestamp: string | undefined;
  isActive: boolean;
  activeSecondsAtLevel: number;
}): number {
  const [segmentStart, setSegmentStart] = useState<number | null>(
    isActive && lastLevelTimestamp ? Date.now() : null,
  );
  const [, setTick] = useState(0);

  // Reset live segment whenever the backend base, activity state, or level changes
  useEffect(() => {
    setSegmentStart(isActive && lastLevelTimestamp ? Date.now() : null);
  }, [activeSecondsAtLevel, isActive, lastLevelTimestamp]);

  // Live 1-second ticker — only runs while actively counting
  useEffect(() => {
    if (!isActive || !lastLevelTimestamp) return;
    const id = setInterval(() => setTick(t => t + 1), 1000);
    return () => clearInterval(id);
  }, [isActive, lastLevelTimestamp, activeSecondsAtLevel]);

  if (!lastLevelTimestamp) return 0;

  const currentSegment = segmentStart !== null ? Math.floor((Date.now() - segmentStart) / 1000) : 0;

  return activeSecondsAtLevel + currentSegment;
}
