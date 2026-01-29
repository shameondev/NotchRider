import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { StreakDisplay } from './StreakDisplay';
import type { StreakState } from '../hooks/useStreak';

describe('StreakDisplay', () => {
  it('displays current streak in meters when under 1km', () => {
    const streak: StreakState = { current: 500, best: 1000, isActive: true };
    render(<StreakDisplay streak={streak} />);
    expect(screen.getByText(/🔥500m/)).toBeInTheDocument();
  });

  it('displays current streak in km when over 1km', () => {
    const streak: StreakState = { current: 2500, best: 5000, isActive: true };
    render(<StreakDisplay streak={streak} />);
    expect(screen.getByText(/🔥2\.50km/)).toBeInTheDocument();
  });

  it('displays best streak', () => {
    const streak: StreakState = { current: 500, best: 3000, isActive: true };
    render(<StreakDisplay streak={streak} />);
    expect(screen.getByText(/🏆3\.00km/)).toBeInTheDocument();
  });

  it('displays both fire and trophy icons', () => {
    const streak: StreakState = { current: 100, best: 200, isActive: true };
    render(<StreakDisplay streak={streak} />);
    expect(screen.getByText(/🔥/)).toBeInTheDocument();
    expect(screen.getByText(/🏆/)).toBeInTheDocument();
  });
});
