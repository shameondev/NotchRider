import { describe, it, expect } from 'vitest';
import { updateStreak } from './useStreak';
import type { StreakState } from './useStreak';

describe('updateStreak', () => {
  it('increments streak when in zone', () => {
    const prev: StreakState = { current: 100, best: 200, isActive: true };
    const result = updateStreak(prev, 'inZone', 10);
    expect(result.current).toBe(110);
    expect(result.isActive).toBe(true);
  });

  it('resets streak when off road', () => {
    const prev: StreakState = { current: 500, best: 200, isActive: true };
    const result = updateStreak(prev, 'offRoad', 0);
    expect(result.current).toBe(0);
    expect(result.isActive).toBe(false);
  });

  it('updates best when current exceeds it', () => {
    const prev: StreakState = { current: 195, best: 200, isActive: true };
    const result = updateStreak(prev, 'inZone', 10);
    expect(result.best).toBe(205);
  });

  it('keeps streak active during warnings', () => {
    const prev: StreakState = { current: 100, best: 200, isActive: true };
    const result = updateStreak(prev, 'tooFast', 5);
    expect(result.current).toBe(105);
    expect(result.isActive).toBe(true);
  });

  it('does not change streak when waiting', () => {
    const prev: StreakState = { current: 100, best: 200, isActive: true };
    const result = updateStreak(prev, 'waiting', 10);
    expect(result).toEqual(prev);
  });
});
