import { describe, it, expect } from 'vitest';
import { render, screen } from '@testing-library/react';
import { Cyclist } from './Cyclist';

describe('Cyclist', () => {
  it('renders cyclist emoji', () => {
    render(<Cyclist y={32} />);
    expect(screen.getByText('🚴')).toBeInTheDocument();
  });

  it('renders at specified Y position', () => {
    render(<Cyclist y={40} />);
    const cyclist = screen.getByTestId('cyclist');
    expect(cyclist).toHaveStyle({ top: '40px' });
  });

  it('shows reduced opacity when not moving', () => {
    render(<Cyclist y={32} isMoving={false} />);
    const cyclist = screen.getByTestId('cyclist');
    expect(cyclist).toHaveStyle({ opacity: '0.6' });
  });

  it('shows full opacity when moving', () => {
    render(<Cyclist y={32} isMoving={true} />);
    const cyclist = screen.getByTestId('cyclist');
    expect(cyclist).toHaveStyle({ opacity: '1' });
  });
});
