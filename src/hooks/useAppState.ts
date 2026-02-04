import { useState, useCallback, useRef, useEffect } from 'react';
import type { AppState, PanelType, AppContext } from '../types/app';

export function useAppState(): AppContext {
  const [appState, setAppState] = useState<AppState>('idle');
  const [panelType, setPanelType] = useState<PanelType>('none');

  // Use ref to track current state for callbacks to avoid stale closures
  const appStateRef = useRef(appState);
  useEffect(() => {
    appStateRef.current = appState;
  }, [appState]);

  const startRecording = useCallback(() => {
    setAppState('recording');
    appStateRef.current = 'recording';
    setPanelType('none');
  }, []);

  const pauseRecording = useCallback(() => {
    if (appStateRef.current === 'recording') {
      setAppState('paused');
      appStateRef.current = 'paused';
    }
  }, []);

  const resumeRecording = useCallback(() => {
    if (appStateRef.current === 'paused') {
      setAppState('recording');
      appStateRef.current = 'recording';
    }
  }, []);

  const stopRecording = useCallback(() => {
    if (appStateRef.current === 'recording' || appStateRef.current === 'paused') {
      setAppState('confirming');
      appStateRef.current = 'confirming';
      setPanelType('confirm-stop');
    }
  }, []);

  const confirmStop = useCallback(() => {
    setAppState('idle');
    setPanelType('none');
  }, []);

  const cancelStop = useCallback(() => {
    setAppState('recording');
    setPanelType('none');
  }, []);

  const openPanel = useCallback((panel: PanelType) => {
    setPanelType(panel);
  }, []);

  const closePanel = useCallback(() => {
    setPanelType('none');
  }, []);

  const togglePanel = useCallback((panel: PanelType) => {
    setPanelType(current => current === panel ? 'none' : panel);
  }, []);

  return {
    appState,
    panelType,
    startRecording,
    pauseRecording,
    resumeRecording,
    stopRecording,
    confirmStop,
    cancelStop,
    openPanel,
    closePanel,
    togglePanel,
  };
}
