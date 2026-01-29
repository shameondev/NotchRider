import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { HUD } from './HUD';
import type { TrainerData, TargetZone } from '../types/trainer';

const mockData: TrainerData = {
  power: 150,
  speed: 32.5,
  cadence: 90,
  heartRate: 145,
  distance: 5000,
  elapsedTime: 900,
  grade: 3,
};

const mockZone: TargetZone = {
  min: 140,
  max: 160,
  metric: 'power',
};

describe('HUD', () => {
  it('displays power value', () => {
    render(<HUD data={mockData} targetZone={mockZone} />);
    expect(screen.getByText(/150W/)).toBeInTheDocument();
  });

  it('displays heart rate', () => {
    render(<HUD data={mockData} targetZone={mockZone} />);
    expect(screen.getByText(/145/)).toBeInTheDocument();
  });

  it('displays distance in km', () => {
    render(<HUD data={mockData} targetZone={mockZone} />);
    expect(screen.getByText(/5\.0/)).toBeInTheDocument();
  });

  it('displays elapsed time formatted', () => {
    render(<HUD data={mockData} targetZone={mockZone} />);
    expect(screen.getByText(/15:00/)).toBeInTheDocument();
  });

  it('displays grade indicator', () => {
    render(<HUD data={mockData} targetZone={mockZone} />);
    expect(screen.getByText(/▲.*3%/)).toBeInTheDocument();
  });
});
