# Keyboard Navigation Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add keyboard navigation with floating panel window for menu, devices, and workout recording.

**Architecture:** Two-window Tauri app. Main window (74px top bar) handles game display. Panel window (floating, ~640x270px) handles all menus and settings. Windows communicate via Tauri events.

**Tech Stack:** Tauri v2, Rust, React 18, TypeScript, Vite

---

## Task 1: Keyboard Event Hook

**Files:**
- Create: `src/hooks/useKeyboard.ts`
- Create: `src/hooks/useKeyboard.test.ts`

**Step 1: Write the failing test**

Create `src/hooks/useKeyboard.test.ts`:

```typescript
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
```

**Step 2: Run test to verify it fails**

Run: `pnpm vitest run src/hooks/useKeyboard.test.ts`
Expected: FAIL with "Cannot find module './useKeyboard'"

**Step 3: Write minimal implementation**

Create `src/hooks/useKeyboard.ts`:

```typescript
import { useEffect } from 'react';

type KeyHandler = () => void;
type KeyBindings = Record<string, KeyHandler>;

export function useKeyboard(bindings: KeyBindings): void {
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      const handler = bindings[event.key];
      if (handler) {
        event.preventDefault();
        handler();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [bindings]);
}
```

**Step 4: Run test to verify it passes**

Run: `pnpm vitest run src/hooks/useKeyboard.test.ts`
Expected: PASS

**Step 5: Commit**

```bash
git add src/hooks/useKeyboard.ts src/hooks/useKeyboard.test.ts
git commit -m "feat: add useKeyboard hook for global hotkeys"
```

---

## Task 2: App State Machine

**Files:**
- Create: `src/hooks/useAppState.ts`
- Create: `src/hooks/useAppState.test.ts`
- Create: `src/types/app.ts`

**Step 1: Create types**

Create `src/types/app.ts`:

```typescript
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
```

**Step 2: Write the failing test**

Create `src/hooks/useAppState.test.ts`:

```typescript
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
```

**Step 3: Run test to verify it fails**

Run: `pnpm vitest run src/hooks/useAppState.test.ts`
Expected: FAIL

**Step 4: Write implementation**

Create `src/hooks/useAppState.ts`:

```typescript
import { useState, useCallback } from 'react';
import type { AppState, PanelType, AppContext } from '../types/app';

export function useAppState(): AppContext {
  const [appState, setAppState] = useState<AppState>('idle');
  const [panelType, setPanelType] = useState<PanelType>('none');

  const startRecording = useCallback(() => {
    setAppState('recording');
    setPanelType('none');
  }, []);

  const pauseRecording = useCallback(() => {
    if (appState === 'recording') {
      setAppState('paused');
    }
  }, [appState]);

  const resumeRecording = useCallback(() => {
    if (appState === 'paused') {
      setAppState('recording');
    }
  }, [appState]);

  const stopRecording = useCallback(() => {
    if (appState === 'recording' || appState === 'paused') {
      setAppState('confirming');
      setPanelType('confirm-stop');
    }
  }, [appState]);

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
```

**Step 5: Run test to verify it passes**

Run: `pnpm vitest run src/hooks/useAppState.test.ts`
Expected: PASS

**Step 6: Commit**

```bash
git add src/types/app.ts src/hooks/useAppState.ts src/hooks/useAppState.test.ts
git commit -m "feat: add app state machine for recording and panels"
```

---

## Task 3: Panel Window Backend (Rust)

**Files:**
- Modify: `src-tauri/src/lib.rs`
- Modify: `src-tauri/tauri.conf.json`

**Step 1: Add panel window to Tauri config**

Edit `src-tauri/tauri.conf.json`, add second window to `windows` array:

```json
{
  "windows": [
    {
      "title": "NotchRider",
      "label": "main",
      "width": 1920,
      "height": 74,
      "x": 0,
      "y": 0,
      "decorations": false,
      "transparent": true,
      "alwaysOnTop": true,
      "resizable": false,
      "skipTaskbar": true,
      "visible": true
    },
    {
      "title": "NotchRider Panel",
      "label": "panel",
      "width": 640,
      "height": 270,
      "x": 0,
      "y": 74,
      "decorations": false,
      "transparent": true,
      "alwaysOnTop": true,
      "resizable": false,
      "skipTaskbar": true,
      "visible": false,
      "url": "index.html#/panel"
    }
  ]
}
```

**Step 2: Add Tauri commands for panel control**

Edit `src-tauri/src/lib.rs`, add commands:

```rust
#[tauri::command]
fn show_panel(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("panel") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn hide_panel(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("panel") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn toggle_panel(app: AppHandle) -> Result<bool, String> {
    if let Some(window) = app.get_webview_window("panel") {
        if window.is_visible().unwrap_or(false) {
            window.hide().map_err(|e| e.to_string())?;
            Ok(false)
        } else {
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
            Ok(true)
        }
    } else {
        Err("Panel window not found".to_string())
    }
}
```

**Step 3: Register commands in invoke_handler**

Add to invoke_handler:

```rust
.invoke_handler(tauri::generate_handler![
    // ... existing commands ...
    show_panel,
    hide_panel,
    toggle_panel,
])
```

**Step 4: Build and verify**

Run: `pnpm tauri dev`
Expected: App compiles, panel window exists but hidden

**Step 5: Commit**

```bash
git add src-tauri/src/lib.rs src-tauri/tauri.conf.json
git commit -m "feat: add panel window backend commands"
```

---

## Task 4: Panel Window Router

**Files:**
- Modify: `src/main.tsx`
- Create: `src/Panel.tsx`
- Modify: `package.json` (add react-router)

**Step 1: Install react-router**

Run: `pnpm add react-router-dom`

**Step 2: Create Panel component**

Create `src/Panel.tsx`:

```typescript
import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';

type PanelView = 'menu' | 'devices' | 'help' | 'trainings' | 'history' | 'settings' | 'confirm-stop';

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
        {view === 'menu' && <MenuView />}
        {view === 'devices' && <DevicesView />}
        {view === 'help' && <HelpView />}
        {view === 'confirm-stop' && <ConfirmStopView />}
        {/* Other views to be implemented */}
      </div>
    </div>
  );
}

function MenuView() {
  return (
    <div>
      <div>&gt; Devices</div>
      <div>  Trainings</div>
      <div>  History</div>
      <div>  Settings</div>
      <div>  About</div>
      <div style={{ marginTop: '16px', opacity: 0.5 }}>
        [↑↓] Navigate  [Enter] Select
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
```

**Step 3: Update main.tsx with router**

Edit `src/main.tsx`:

```typescript
import React from "react";
import ReactDOM from "react-dom/client";
import { HashRouter, Routes, Route } from "react-router-dom";
import App from "./App";
import { Panel } from "./Panel";
import "./styles/global.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <HashRouter>
      <Routes>
        <Route path="/" element={<App />} />
        <Route path="/panel" element={<Panel />} />
      </Routes>
    </HashRouter>
  </React.StrictMode>
);
```

**Step 4: Build and verify**

Run: `pnpm tauri dev`
Expected: Main app works, panel window shows Panel component when visible

**Step 5: Commit**

```bash
git add src/main.tsx src/Panel.tsx package.json pnpm-lock.yaml
git commit -m "feat: add panel window with router and basic views"
```

---

## Task 5: Integrate Keyboard with App

**Files:**
- Modify: `src/App.tsx`

**Step 1: Add keyboard handling to App**

Edit `src/App.tsx`, add imports and keyboard integration:

```typescript
import { useEffect, useRef, useCallback, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { emit } from '@tauri-apps/api/event';
import { Road } from './components/Road';
import { Cyclist } from './components/Cyclist';
import { useTrainer } from './hooks/useTrainer';
import { useKeyboard } from './hooks/useKeyboard';
import { useAppState } from './hooks/useAppState';
```

Add after useTrainer hook:

```typescript
const {
  appState,
  panelType,
  startRecording,
  pauseRecording,
  resumeRecording,
  stopRecording,
  togglePanel,
  closePanel,
} = useAppState();

// Keyboard bindings
const keyBindings = useMemo(() => ({
  'Escape': () => togglePanel('menu'),
  'd': () => togglePanel('devices'),
  'D': () => togglePanel('devices'),
  '?': () => togglePanel('help'),
  'r': () => {
    if (appState === 'idle') startRecording();
  },
  'R': () => {
    if (appState === 'idle') startRecording();
  },
  ' ': () => {
    if (appState === 'recording') pauseRecording();
    else if (appState === 'paused') resumeRecording();
  },
  's': () => {
    if (appState === 'recording' || appState === 'paused') stopRecording();
  },
  'S': () => {
    if (appState === 'recording' || appState === 'paused') stopRecording();
  },
}), [appState, togglePanel, startRecording, pauseRecording, resumeRecording, stopRecording]);

useKeyboard(keyBindings);

// Sync panel visibility with backend
useEffect(() => {
  if (panelType === 'none') {
    invoke('hide_panel').catch(console.error);
  } else {
    invoke('show_panel').catch(console.error);
    emit('panel:set-view', panelType).catch(console.error);
  }
}, [panelType]);
```

**Step 2: Add recording indicator to HUD**

Update the TOP RIGHT section in the return JSX:

```typescript
{/* === TOP RIGHT: Data === */}
<div style={{
  display: 'flex',
  alignItems: 'center',
  padding: '0 15px',
  gap: '12px',
}}>
  <span>{trainerData.speed.toFixed(1)}km/h</span>
  <span>{trainerData.distance.toFixed(2)}km</span>
  <span>
    {appState === 'recording' && '🔴 '}
    {appState === 'paused' && '⏸ '}
    {formatTime(trainerData.elapsedTime)}
  </span>
  <span style={{ opacity: 0.5 }}>
    {isConnected ? (isSimulation ? '◐' : '●') : '○'}
  </span>
</div>
```

**Step 3: Add help hint to bottom**

Add to BOTTOM section:

```typescript
{/* === BOTTOM: Road (spans all 3 columns) === */}
<div style={{
  gridColumn: '1 / -1',
  position: 'relative',
}}>
  <Road roadY={10} laneGap={10} />
  <Cyclist ref={cyclistRef} y={15} isMoving={isMoving} />
  <span style={{
    position: 'absolute',
    bottom: '4px',
    left: '8px',
    opacity: 0.3,
    fontSize: '10px',
  }}>
    [?]
  </span>
</div>
```

**Step 4: Verify everything works**

Run: `pnpm tauri dev`
Test:
- Press `?` → Help panel appears
- Press `Esc` → Menu panel appears
- Press `D` → Devices panel appears
- Press `R` → Recording starts (🔴 appears)
- Press `Space` → Pause (⏸ appears)
- Press `S` → Confirm dialog appears

**Step 5: Commit**

```bash
git add src/App.tsx
git commit -m "feat: integrate keyboard navigation with app"
```

---

## Task 6: Panel Navigation (Arrow Keys)

**Files:**
- Modify: `src/Panel.tsx`
- Create: `src/hooks/useListNavigation.ts`

**Step 1: Create list navigation hook**

Create `src/hooks/useListNavigation.ts`:

```typescript
import { useState, useCallback } from 'react';
import { useKeyboard } from './useKeyboard';

interface UseListNavigationOptions<T> {
  items: T[];
  onSelect: (item: T, index: number) => void;
  onCancel?: () => void;
}

export function useListNavigation<T>({ items, onSelect, onCancel }: UseListNavigationOptions<T>) {
  const [selectedIndex, setSelectedIndex] = useState(0);

  const moveUp = useCallback(() => {
    setSelectedIndex(i => Math.max(0, i - 1));
  }, []);

  const moveDown = useCallback(() => {
    setSelectedIndex(i => Math.min(items.length - 1, i + 1));
  }, [items.length]);

  const select = useCallback(() => {
    if (items[selectedIndex]) {
      onSelect(items[selectedIndex], selectedIndex);
    }
  }, [items, selectedIndex, onSelect]);

  useKeyboard({
    'ArrowUp': moveUp,
    'ArrowDown': moveDown,
    'Enter': select,
    'Escape': onCancel ?? (() => {}),
  });

  return { selectedIndex, setSelectedIndex };
}
```

**Step 2: Update MenuView with navigation**

Update `src/Panel.tsx` MenuView:

```typescript
import { emit } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/core';
import { useListNavigation } from './hooks/useListNavigation';

const MENU_ITEMS = [
  { id: 'devices', label: 'Devices' },
  { id: 'trainings', label: 'Trainings' },
  { id: 'history', label: 'History' },
  { id: 'settings', label: 'Settings' },
  { id: 'about', label: 'About' },
] as const;

function MenuView({ onNavigate }: { onNavigate: (view: string) => void }) {
  const { selectedIndex } = useListNavigation({
    items: MENU_ITEMS,
    onSelect: (item) => onNavigate(item.id),
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
```

**Step 3: Update Panel to pass onNavigate**

```typescript
export function Panel() {
  const [view, setView] = useState<PanelView>('menu');

  // ... listen effect ...

  const handleNavigate = useCallback((newView: string) => {
    setView(newView as PanelView);
  }, []);

  return (
    <div style={{ /* ... */ }}>
      {/* ... header ... */}
      <div>
        {view === 'menu' && <MenuView onNavigate={handleNavigate} />}
        {/* ... other views ... */}
      </div>
    </div>
  );
}
```

**Step 4: Verify**

Run: `pnpm tauri dev`
Test: Press Esc → Menu → Arrow keys → Enter to select

**Step 5: Commit**

```bash
git add src/hooks/useListNavigation.ts src/Panel.tsx
git commit -m "feat: add arrow key navigation in panel menus"
```

---

## Summary

After completing all tasks:

**Phase 1 Complete:**
- ✅ Keyboard event system (`useKeyboard`)
- ✅ App state machine (`useAppState`)
- ✅ Help hint `[?]`

**Phase 2 Complete:**
- ✅ Panel window (Rust backend)
- ✅ Panel component with views
- ✅ Window show/hide
- ✅ List navigation

**Next phases (separate plan):**
- Phase 3: Devices panel with ANT+ scanning
- Phase 4: Workout recording with save
- Phase 5: History and trainings menu
