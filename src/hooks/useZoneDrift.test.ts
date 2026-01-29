import { describe, it, expect } from 'vitest';
import { calculateDrift, DriftState } from './useZoneDrift';

describe('calculateDrift', () => {
  const zone = { min: 140, max: 160, metric: 'power' as const };

  it('returns no drift when in zone', () => {
    const result = calculateDrift(150, zone);
    expect(result.state).toBe('inZone');
    expect(result.offset).toBe(0);
  });

  it('returns positive drift (down) when above zone', () => {
    const result = calculateDrift(170, zone);
    expect(result.state).toBe('tooFast');
    expect(result.offset).toBeGreaterThan(0);
  });

  it('returns negative drift (up) when below zone', () => {
    const result = calculateDrift(130, zone);
    expect(result.state).toBe('tooSlow');
    expect(result.offset).toBeLessThan(0);
  });

  it('scales drift by distance from zone', () => {
    const slightlyOver = calculateDrift(165, zone);
    const wayOver = calculateDrift(200, zone);
    expect(wayOver.offset).toBeGreaterThan(slightlyOver.offset);
  });
});
