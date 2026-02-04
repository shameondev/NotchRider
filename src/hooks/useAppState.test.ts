import { describe, it, expect } from 'vitest';
import { renderHook, act } from '@testing-library/react';
import { useAppState } from './useAppState';

describe('useAppState', () => {
  it('starts in idle state with no panel', () => {
    const { result } = renderHook(() => useAppState());
    expect(result.current.appState).toBe('idle');
    expect(result.current.panelType).toBe('none');
  });

  it('transitions to recording when startRecording is called', () => {
    const { result } = renderHook(() => useAppState());

    act(() => {
      result.current.startRecording();
    });

    expect(result.current.appState).toBe('recording');
  });

  it('transitions to paused when pauseRecording is called during recording', () => {
    const { result } = renderHook(() => useAppState());

    act(() => {
      result.current.startRecording();
      result.current.pauseRecording();
    });

    expect(result.current.appState).toBe('paused');
  });

  it('transitions to confirming when stopRecording is called', () => {
    const { result } = renderHook(() => useAppState());

    act(() => {
      result.current.startRecording();
      result.current.stopRecording();
    });

    expect(result.current.appState).toBe('confirming');
    expect(result.current.panelType).toBe('confirm-stop');
  });

  it('returns to idle when confirmStop is called', () => {
    const { result } = renderHook(() => useAppState());

    act(() => {
      result.current.startRecording();
      result.current.stopRecording();
      result.current.confirmStop();
    });

    expect(result.current.appState).toBe('idle');
    expect(result.current.panelType).toBe('none');
  });

  it('returns to recording when cancelStop is called', () => {
    const { result } = renderHook(() => useAppState());

    act(() => {
      result.current.startRecording();
      result.current.stopRecording();
      result.current.cancelStop();
    });

    expect(result.current.appState).toBe('recording');
    expect(result.current.panelType).toBe('none');
  });

  it('toggles panel visibility', () => {
    const { result } = renderHook(() => useAppState());

    act(() => {
      result.current.togglePanel('menu');
    });
    expect(result.current.panelType).toBe('menu');

    act(() => {
      result.current.togglePanel('menu');
    });
    expect(result.current.panelType).toBe('none');
  });
});
