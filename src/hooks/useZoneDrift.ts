import type { TargetZone } from '../types/trainer';

export type DriftState = 'waiting' | 'inZone' | 'tooFast' | 'tooSlow' | 'offRoad';

export interface DriftResult {
  state: DriftState;
  offset: number; // pixels to offset from road center (-30 to +30)
}

const MAX_DRIFT = 30; // max pixels off road before "off road"

// Minimum values to consider the ride "started"
const MIN_POWER_TO_START = 10; // watts
const MIN_CADENCE_TO_START = 20; // rpm

export function calculateDrift(
  currentValue: number,
  zone: TargetZone,
  isRideStarted: boolean = true
): DriftResult {
  // If ride hasn't started, return waiting state
  if (!isRideStarted) {
    return { state: 'waiting', offset: 0 };
  }

  if (currentValue >= zone.min && currentValue <= zone.max) {
    return { state: 'inZone', offset: 0 };
  }

  const zoneRange = zone.max - zone.min;

  if (currentValue > zone.max) {
    // Too fast → drift down (positive offset)
    const excess = currentValue - zone.max;
    const normalized = Math.min(excess / zoneRange, 1);
    const offset = normalized * MAX_DRIFT;
    return {
      state: offset >= MAX_DRIFT ? 'offRoad' : 'tooFast',
      offset,
    };
  } else {
    // Too slow → drift up (negative offset)
    const deficit = zone.min - currentValue;
    const normalized = Math.min(deficit / zoneRange, 1);
    const offset = -normalized * MAX_DRIFT;
    return {
      state: offset <= -MAX_DRIFT ? 'offRoad' : 'tooSlow',
      offset,
    };
  }
}

// Helper to check if ride has started based on trainer data
export function isRideActive(power: number, cadence: number): boolean {
  return power >= MIN_POWER_TO_START || cadence >= MIN_CADENCE_TO_START;
}
