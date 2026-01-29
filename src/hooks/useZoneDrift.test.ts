import { describe, it, expect } from 'vitest';
import { calculateDrift, isRideActive } from './useZoneDrift';

describe('calculateDrift', () => {
  const zone = { min: 140, max: 160, metric: 'power' as const };

  it('returns waiting state when ride not started', () => {
    const result = calculateDrift(150, zone, false);
    expect(result.state).toBe('waiting');
    expect(result.offset).toBe(0);
  });

  it('returns no drift when in zone', () => {
    const result = calculateDrift(150, zone, true);
    expect(result.state).toBe('inZone');
    expect(result.offset).toBe(0);
  });

  it('returns positive drift (down) when above zone', () => {
    const result = calculateDrift(170, zone, true);
    expect(result.state).toBe('tooFast');
    expect(result.offset).toBeGreaterThan(0);
  });

  it('returns negative drift (up) when below zone', () => {
    const result = calculateDrift(130, zone, true);
    expect(result.state).toBe('tooSlow');
    expect(result.offset).toBeLessThan(0);
  });

  it('scales drift by distance from zone', () => {
    const slightlyOver = calculateDrift(165, zone, true);
    const wayOver = calculateDrift(200, zone, true);
    expect(wayOver.offset).toBeGreaterThan(slightlyOver.offset);
  });
});

describe('isRideActive', () => {
  it('returns false when power and cadence are zero', () => {
    expect(isRideActive(0, 0)).toBe(false);
  });

  it('returns true when power is above threshold', () => {
    expect(isRideActive(15, 0)).toBe(true);
  });

  it('returns true when cadence is above threshold', () => {
    expect(isRideActive(0, 25)).toBe(true);
  });

  it('returns false when values are below thresholds', () => {
    expect(isRideActive(5, 10)).toBe(false);
  });
});
