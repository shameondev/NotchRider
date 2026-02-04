export type AppState = 'idle' | 'recording' | 'paused' | 'confirming';
export type PanelType = 'none' | 'menu' | 'devices' | 'help' | 'trainings' | 'history' | 'settings' | 'confirm-stop';

export interface AppContext {
  appState: AppState;
  panelType: PanelType;
  // Actions
  startRecording: () => void;
  pauseRecording: () => void;
  resumeRecording: () => void;
  stopRecording: () => void;
  confirmStop: () => void;
  cancelStop: () => void;
  openPanel: (panel: PanelType) => void;
  closePanel: () => void;
  togglePanel: (panel: PanelType) => void;
}
