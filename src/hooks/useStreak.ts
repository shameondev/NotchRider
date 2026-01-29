import { useState, useCallback } from 'react';
import type { DriftState } from './useZoneDrift';

export interface StreakState {
  current: number;  // meters
  best: number;     // meters
  isActive: boolean;
}

export function updateStreak(
  prev: StreakState,
  driftState: DriftState,
  distanceDelta: number
): StreakState {
  if (driftState === 'offRoad') {
    return {
      current: 0,
      best: prev.best,
      isActive: false,
    };
  }

  const newCurrent = prev.current + distanceDelta;
  const newBest = Math.max(prev.best, newCurrent);

  return {
    current: newCurrent,
    best: newBest,
    isActive: true,
  };
}

export function useStreak() {
  const [streak, setStreak] = useState<StreakState>({
    current: 0,
    best: 0,
    isActive: true,
  });

  const update = useCallback((driftState: DriftState, distanceDelta: number) => {
    setStreak(prev => updateStreak(prev, driftState, distanceDelta));
  }, []);

  const reset = useCallback(() => {
    setStreak(prev => ({ ...prev, current: 0, isActive: true }));
  }, []);

  return { streak, update, reset };
}
