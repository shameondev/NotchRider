import { useEffect, useState, useCallback } from 'react';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { useListNavigation } from './hooks/useListNavigation';

type PanelView = 'menu' | 'devices' | 'help' | 'trainings' | 'history' | 'settings' | 'about' | 'confirm-stop';

const MENU_ITEMS = [
  { id: 'devices', label: 'Devices' },
  { id: 'trainings', label: 'Trainings' },
  { id: 'history', label: 'History' },
  { id: 'settings', label: 'Settings' },
  { id: 'about', label: 'About' },
] as const;

export function Panel() {
  const [view, setView] = useState<PanelView>('menu');

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
        {view === 'devices' && <DevicesView />}
        {view === 'help' && <HelpView />}
        {view === 'confirm-stop' && <ConfirmStopView />}
      </div>
    </div>
  );
}

function MenuView({ onNavigate }: { onNavigate: (view: string) => void }) {
  const { selectedIndex } = useListNavigation({
    items: MENU_ITEMS,
    onSelect: (item) => onNavigate(item.id as PanelView),
    onCancel: () => invoke('hide_panel'),
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

function DevicesView() {
  return <div>Devices panel (TODO)</div>;
}

function HelpView() {
  return (
    <div>
      <div>[Esc]   Menu</div>
      <div>[D]     Devices</div>
      <div>[R]     Record</div>
      <div>[Space] Pause</div>
      <div>[S]     Stop</div>
      <div>[?]     This help</div>
      <div style={{ marginTop: '16px', opacity: 0.5 }}>
        Press any key to close
      </div>
    </div>
  );
}

function ConfirmStopView() {
  return (
    <div>
      <div>Finish workout?</div>
      <div style={{ marginTop: '12px' }}>
        <div>&gt; [Y]es, finish and save</div>
        <div>  [N]o, continue riding</div>
      </div>
    </div>
  );
}
