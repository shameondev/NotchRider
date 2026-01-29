import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Cyclist } from './Cyclist';

describe('Cyclist', () => {
  it('renders cyclist emoji', () => {
    render(<Cyclist x={100} y={37} driftOffset={0} driftState="inZone" />);
    expect(screen.getByText('🚴')).toBeInTheDocument();
  });

  it('shows warning when tooFast', () => {
    render(<Cyclist x={100} y={37} driftOffset={15} driftState="tooFast" />);
    expect(screen.getByText(/WARNING/)).toBeInTheDocument();
  });

  it('shows OFF ROAD when offRoad', () => {
    render(<Cyclist x={100} y={37} driftOffset={30} driftState="offRoad" />);
    expect(screen.getByText(/OFF ROAD/)).toBeInTheDocument();
  });
});
