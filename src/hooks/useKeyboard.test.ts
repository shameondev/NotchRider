import { describe, it, expect, vi } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useKeyboard } from './useKeyboard';

describe('useKeyboard', () => {
  it('calls handler when key is pressed', () => {
    const handler = vi.fn();
    renderHook(() => useKeyboard({ Escape: handler }));

    act(() => {
      window.dispatchEvent(new KeyboardEvent('keydown', { key: 'Escape' }));
    });

    expect(handler).toHaveBeenCalledTimes(1);
  });

  it('does not call handler for unregistered keys', () => {
    const handler = vi.fn();
    renderHook(() => useKeyboard({ Escape: handler }));

    act(() => {
      window.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter' }));
    });

    expect(handler).not.toHaveBeenCalled();
  });

  it('handles ? key correctly', () => {
    const handler = vi.fn();
    renderHook(() => useKeyboard({ '?': handler }));

    act(() => {
      window.dispatchEvent(new KeyboardEvent('keydown', { key: '?' }));
    });

    expect(handler).toHaveBeenCalledTimes(1);
  });
});
