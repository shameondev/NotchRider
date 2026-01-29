import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Cyclist } from './Cyclist';

describe('Cyclist', () => {
  it('renders cyclist emoji', () => {
    render(<Cyclist x={100} y={37} />);
    expect(screen.getByText('🚴')).toBeInTheDocument();
  });

  it('positions cyclist at given coordinates', () => {
    render(<Cyclist x={150} y={50} />);
    const cyclist = screen.getByTestId('cyclist');
    expect(cyclist).toHaveStyle({ left: '150px', top: '50px' });
  });
});
