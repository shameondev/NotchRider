import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Road } from './Road';

describe('Road', () => {
  it('renders road with notch gap', () => {
    render(<Road notchWidth={200} notchX={860} />);

    const road = screen.getByTestId('road');
    expect(road).toBeInTheDocument();
  });

  it('renders left and right road segments', () => {
    render(<Road notchWidth={200} notchX={860} />);

    expect(screen.getByTestId('road-left')).toBeInTheDocument();
    expect(screen.getByTestId('road-right')).toBeInTheDocument();
  });

  it('renders under-notch road segment', () => {
    render(<Road notchWidth={200} notchX={860} />);
    expect(screen.getByTestId('road-under-notch')).toBeInTheDocument();
  });
});
