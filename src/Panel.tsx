import { useEffect, useState, useCallback } from 'react';
import { listen, emit } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { useListNavigation } from './hooks/useListNavigation';
import { useKeyboard } from './hooks/useKeyboard';

type PanelView = 'menu' | 'devices' | 'help' | 'trainings' | 'settings' | 'about' | 'confirm-stop' | 'summary';

interface WorkoutSummary {
  duration_secs: number;
  distance_km: number;
  avg_power: number;
  max_power: number;
  avg_hr: number;
  max_hr: number;
  avg_cadence: number;
  sample_count: number;
  file_path: string;
}

const MENU_ITEMS = [
  { id: 'trainings', label: 'Trainings' },
  { id: 'devices', label: 'Devices' },
  { id: 'settings', label: 'Settings' },
  { id: 'about', label: 'About' },
] as const;

const closePanel = () => invoke('hide_panel');

export function Panel() {
  const [view, setView] = useState<PanelView>('menu');
  const [workoutSummary, setWorkoutSummary] = useState<WorkoutSummary | null>(null);

  useEffect(() => {
    const unlisten = listen<PanelView>('panel:set-view', (event) => {
      setView(event.payload);
    });

    return () => {
      unlisten.then(fn => fn());
    };
  }, []);

  const handleNavigate = useCallback((newView: string) => {
    setView(newView as PanelView);
  }, []);

  const handleWorkoutSaved = useCallback((summary: WorkoutSummary) => {
    setWorkoutSummary(summary);
    setView('summary');
  }, []);

  // Forward global recording shortcuts to main window
  useEffect(() => {
    const handleKey = (e: KeyboardEvent) => {
      // Don't forward when in views that handle their own keys
      if (view === 'confirm-stop' || view === 'summary') return;

      if (e.key === 'r' || e.key === 'R') {
        emit('app:start-recording');
        closePanel();
      } else if (e.key === ' ') {
        e.preventDefault();
        // Main window checks actual state and picks pause or resume
        emit('app:pause-recording');
        emit('app:resume-recording');
      } else if (e.key === 's' || e.key === 'S') {
        emit('app:stop-recording');
      }
    };
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  }, [view]);

  return (
    <div style={{
      width: '100%',
      height: '100%',
      background: 'var(--bg-primary)',
      fontFamily: '"SF Mono", Monaco, monospace',
      fontSize: '12px',
      color: 'var(--text-primary)',
      padding: '16px',
      boxSizing: 'border-box',
    }}>
      <div style={{
        borderBottom: '1px solid var(--text-secondary)',
        paddingBottom: '8px',
        marginBottom: '12px',
        display: 'flex',
        justifyContent: 'space-between',
      }}>
        <span style={{ textTransform: 'uppercase' }}>{view}</span>
        <span style={{ opacity: 0.5 }}>[Esc]</span>
      </div>

      <div>
        {view === 'menu' && <MenuView onNavigate={handleNavigate} />}
        {view === 'trainings' && <TrainingsView onBack={() => setView('menu')} />}
        {view === 'devices' && <DevicesView onBack={() => setView('menu')} />}
        {view === 'settings' && <PlaceholderView name="Settings" onBack={() => setView('menu')} />}
        {view === 'about' && <AboutView onBack={() => setView('menu')} />}
        {view === 'help' && <HelpView onBack={() => setView('menu')} />}
        {view === 'confirm-stop' && <ConfirmStopView onBack={() => setView('menu')} onSaved={handleWorkoutSaved} />}
        {view === 'summary' && workoutSummary && <SummaryView summary={workoutSummary} onClose={() => { setWorkoutSummary(null); closePanel(); }} />}
      </div>
    </div>
  );
}

function MenuView({ onNavigate }: { onNavigate: (view: string) => void }) {
  const { selectedIndex } = useListNavigation({
    items: MENU_ITEMS,
    onSelect: (item) => onNavigate(item.id as PanelView),
    onCancel: closePanel,
  });

  return (
    <div>
      {MENU_ITEMS.map((item, index) => (
        <div key={item.id} style={{ opacity: index === selectedIndex ? 1 : 0.5 }}>
          {index === selectedIndex ? '> ' : '  '}{item.label}
        </div>
      ))}
      <div style={{ marginTop: '16px', opacity: 0.5 }}>
        [↑↓] Navigate  [Enter] Select  [Esc] Close
      </div>
    </div>
  );
}

const TRAINING_ITEMS = [
  { id: 'start', label: 'Start Training [R]' },
  { id: 'history', label: 'History' },
] as const;

function TrainingsView({ onBack }: { onBack: () => void }) {
  const { selectedIndex } = useListNavigation({
    items: TRAINING_ITEMS,
    onSelect: (item) => {
      if (item.id === 'start') {
        emit('app:start-recording');
        closePanel();
      }
      // history: coming soon, no action yet
    },
    onCancel: onBack,
  });

  return (
    <div>
      {TRAINING_ITEMS.map((item, index) => (
        <div key={item.id} style={{ opacity: index === selectedIndex ? 1 : 0.5 }}>
          {index === selectedIndex ? '> ' : '  '}{item.label}
        </div>
      ))}
      <div style={{ marginTop: '16px', opacity: 0.5 }}>
        [↑↓] Navigate  [Enter] Select  [Esc] Back
      </div>
    </div>
  );
}

type DeviceStatus = 'idle' | 'scanning' | 'found' | 'connecting' | 'connected' | 'error';

function DevicesView({ onBack }: { onBack: () => void }) {
  const [status, setStatus] = useState<DeviceStatus>('idle');
  const [error, setError] = useState<string | null>(null);
  const [usbDevices, setUsbDevices] = useState<string[]>([]);

  // Check initial connection state
  useEffect(() => {
    invoke<boolean>('is_connected').then(connected => {
      if (connected) setStatus('connected');
    });
  }, []);

  const scan = useCallback(async () => {
    setStatus('scanning');
    setError(null);
    try {
      const devices = await invoke<string[]>('list_usb_devices');
      setUsbDevices(devices);
      const found = await invoke<boolean>('find_ant_device');
      setStatus(found ? 'found' : 'idle');
      if (!found) setError('ANT+ dongle not found');
    } catch (e) {
      setStatus('error');
      setError(String(e));
    }
  }, []);

  const connect = useCallback(async () => {
    setStatus('connecting');
    setError(null);
    try {
      await invoke<boolean>('connect_ant_device');
      setStatus('connected');
    } catch (e) {
      setStatus('error');
      setError(String(e));
    }
  }, []);

  const disconnect = useCallback(async () => {
    try {
      await invoke('disconnect_ant_device');
      setStatus('idle');
    } catch (e) {
      setError(String(e));
    }
  }, []);

  const DEVICE_ACTIONS = (() => {
    switch (status) {
      case 'idle':
      case 'error':
        return [{ id: 'scan', label: 'Scan for ANT+ dongle' }];
      case 'found':
        return [
          { id: 'connect', label: 'Connect' },
          { id: 'scan', label: 'Rescan' },
        ];
      case 'connected':
        return [{ id: 'disconnect', label: 'Disconnect' }];
      default:
        return [];
    }
  })();

  const { selectedIndex } = useListNavigation({
    items: DEVICE_ACTIONS,
    onSelect: (item) => {
      if (item.id === 'scan') scan();
      else if (item.id === 'connect') connect();
      else if (item.id === 'disconnect') disconnect();
    },
    onCancel: onBack,
  });

  const statusLine = (() => {
    switch (status) {
      case 'idle': return 'No device';
      case 'scanning': return 'Scanning...';
      case 'found': return 'ANT+ dongle found';
      case 'connecting': return 'Connecting...';
      case 'connected': return 'Connected';
      case 'error': return 'Error';
    }
  })();

  const statusColor = status === 'connected' ? 'var(--text-primary)' :
                       status === 'error' ? '#ff4444' :
                       'var(--text-secondary)';

  return (
    <div>
      <div style={{ marginBottom: '12px' }}>
        <span style={{ opacity: 0.5 }}>STATUS: </span>
        <span style={{ color: statusColor }}>{statusLine}</span>
      </div>

      {error && (
        <div style={{ color: '#ff4444', marginBottom: '8px', fontSize: '11px' }}>
          {error}
        </div>
      )}

      {usbDevices.length > 0 && status !== 'connected' && (
        <div style={{ marginBottom: '12px', opacity: 0.5, fontSize: '11px' }}>
          USB: {usbDevices.length} device(s)
        </div>
      )}

      {status === 'connected' && (
        <div style={{ marginBottom: '12px', opacity: 0.7, fontSize: '11px' }}>
          <div>CH0: FE-C (trainer)</div>
          <div>CH1: HRM (heart rate)</div>
        </div>
      )}

      {(status === 'scanning' || status === 'connecting') ? (
        <div style={{ opacity: 0.5 }}>
          {'> '}{statusLine}
        </div>
      ) : (
        DEVICE_ACTIONS.map((item, index) => (
          <div key={item.id} style={{ opacity: index === selectedIndex ? 1 : 0.5 }}>
            {index === selectedIndex ? '> ' : '  '}{item.label}
          </div>
        ))
      )}

      <div style={{ marginTop: '16px', opacity: 0.5 }}>
        {DEVICE_ACTIONS.length > 0 && '[Enter] Select  '}[Esc] Back
      </div>
    </div>
  );
}

function PlaceholderView({ name, onBack }: { name: string; onBack: () => void }) {
  useKeyboard({ 'Escape': onBack });

  return (
    <div>
      <div style={{ opacity: 0.5 }}>{name} — coming soon</div>
      <div style={{ marginTop: '16px', opacity: 0.5 }}>
        [Esc] Back
      </div>
    </div>
  );
}

function AboutView({ onBack }: { onBack: () => void }) {
  useKeyboard({ 'Escape': onBack });

  return (
    <div>
      <div style={{ marginBottom: '12px' }}>NotchRider</div>
      <div style={{ opacity: 0.7 }}>
        <div>Minimalist cycling game for macOS.</div>
        <div>Turns your MacBook notch into a</div>
        <div>game element during indoor rides.</div>
      </div>
      <div style={{ marginTop: '12px', opacity: 0.5, fontSize: '11px' }}>
        <div>ANT+ FE-C / HRM</div>
        <div>Tauri + React + Rust</div>
      </div>
      <div style={{ marginTop: '16px', opacity: 0.5 }}>
        [Esc] Back
      </div>
    </div>
  );
}

function HelpView({ onBack }: { onBack: () => void }) {
  useKeyboard({ 'Escape': onBack });

  return (
    <div>
      <div>[Esc]   Menu</div>
      <div>[D]     Devices</div>
      <div>[R]     Record</div>
      <div>[Space] Pause</div>
      <div>[S]     Stop</div>
      <div>[?]     This help</div>
      <div style={{ marginTop: '16px', opacity: 0.5 }}>
        [Esc] Back
      </div>
    </div>
  );
}

function ConfirmStopView({ onBack, onSaved }: { onBack: () => void; onSaved: (summary: WorkoutSummary) => void }) {
  const [saving, setSaving] = useState(false);

  const handleConfirm = useCallback(async () => {
    if (saving) return;
    setSaving(true);
    try {
      const summary = await invoke<WorkoutSummary>('stop_workout');
      emit('workout:confirmed');
      onSaved(summary);
    } catch (e) {
      console.error('Failed to save workout:', e);
      setSaving(false);
    }
  }, [saving, onSaved]);

  const handleCancel = useCallback(() => {
    emit('workout:cancelled');
    onBack();
  }, [onBack]);

  useKeyboard({
    'y': handleConfirm,
    'Y': handleConfirm,
    'n': handleCancel,
    'N': handleCancel,
    'Escape': handleCancel,
  });

  return (
    <div>
      <div>Finish workout?</div>
      <div style={{ marginTop: '12px' }}>
        {saving ? (
          <div>Saving...</div>
        ) : (
          <>
            <div>&gt; [Y]es, finish and save</div>
            <div>  [N]o, continue riding</div>
          </>
        )}
      </div>
    </div>
  );
}

function SummaryView({ summary, onClose }: { summary: WorkoutSummary; onClose: () => void }) {
  useKeyboard({
    'Escape': onClose,
    'Enter': onClose,
  });

  const mins = Math.floor(summary.duration_secs / 60);
  const secs = summary.duration_secs % 60;

  return (
    <div>
      <div style={{ marginBottom: '12px' }}>Workout saved</div>
      <div style={{ opacity: 0.8 }}>
        <div>Duration  {mins}:{secs.toString().padStart(2, '0')}</div>
        <div>Distance  {summary.distance_km.toFixed(2)} km</div>
        <div>Avg Power {summary.avg_power}W (max {summary.max_power}W)</div>
        {summary.avg_hr > 0 && <div>Avg HR    {summary.avg_hr} bpm (max {summary.max_hr})</div>}
        <div>Cadence   {summary.avg_cadence} rpm</div>
        <div>Samples   {summary.sample_count}</div>
      </div>
      <div style={{ marginTop: '12px', opacity: 0.5, fontSize: '11px' }}>
        {summary.file_path.split('/').pop()}
      </div>
      <div style={{ marginTop: '16px', opacity: 0.5 }}>
        [Enter/Esc] Close
      </div>
    </div>
  );
}
