import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useTrainer } from './useTrainer';

// Mock Tauri's invoke function
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

const mockInvoke = invoke as ReturnType<typeof vi.fn>;

describe('useTrainer', () => {
  beforeEach(() => {
    vi.useFakeTimers();
    mockInvoke.mockReset();
    // Default mock for find_ant_device
    mockInvoke.mockResolvedValue(false);
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it('starts with default values', async () => {
    const { result } = renderHook(() => useTrainer());

    expect(result.current.data.power).toBe(0);
    expect(result.current.data.speed).toBe(0);
    expect(result.current.data.cadence).toBe(0);
    expect(result.current.data.heartRate).toBe(0);

    // Cleanup: let auto-connect finish
    await act(async () => {
      await vi.runAllTimersAsync();
    });
  });

  it('auto-connects on mount and enters simulation mode', async () => {
    const { result } = renderHook(() => useTrainer());

    // Run all pending promises and timers
    await act(async () => {
      await vi.runAllTimersAsync();
    });

    expect(mockInvoke).toHaveBeenCalledWith('find_ant_device');
    expect(result.current.isSimulation).toBe(true);
    expect(result.current.isConnected).toBe(true);
  });

  it('connects to real device when available', async () => {
    mockInvoke
      .mockResolvedValueOnce(true) // find_ant_device
      .mockResolvedValueOnce(true) // connect_ant_device
      .mockResolvedValue(undefined); // disconnect_ant_device

    const { result } = renderHook(() => useTrainer());

    await act(async () => {
      await vi.runAllTimersAsync();
    });

    expect(result.current.isConnected).toBe(true);
    expect(result.current.isSimulation).toBe(false);
  });

  it('accumulates elapsed time in simulation mode', async () => {
    const { result, unmount } = renderHook(() => useTrainer());

    // Wait for connection to complete
    await act(async () => {
      await Promise.resolve(); // flush microtasks
    });

    expect(result.current.isSimulation).toBe(true);

    // Advance timer by exactly one interval (100ms)
    await act(async () => {
      vi.advanceTimersByTime(100);
    });

    // elapsedTime should have increased by 0.1
    expect(result.current.data.elapsedTime).toBeGreaterThan(0);

    // Cleanup
    unmount();
  });

  it('returns error as null by default', async () => {
    const { result } = renderHook(() => useTrainer());
    expect(result.current.error).toBeNull();

    // Cleanup
    await act(async () => {
      await vi.runAllTimersAsync();
    });
  });

  it('provides connect and disconnect functions', async () => {
    const { result } = renderHook(() => useTrainer());

    expect(typeof result.current.connect).toBe('function');
    expect(typeof result.current.disconnect).toBe('function');

    // Cleanup
    await act(async () => {
      await vi.runAllTimersAsync();
    });
  });
});
