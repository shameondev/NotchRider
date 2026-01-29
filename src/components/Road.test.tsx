import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Road } from './Road';

describe('Road', () => {
  it('renders road', () => {
    render(<Road roadY={32} />);
    const road = screen.getByTestId('road');
    expect(road).toBeInTheDocument();
  });

  it('renders two lanes', () => {
    render(<Road roadY={32} laneGap={10} />);
    const road = screen.getByTestId('road');
    // Road contains the lane lines
    expect(road.textContent).toContain('─');
  });
});
