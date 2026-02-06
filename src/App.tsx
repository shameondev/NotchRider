import { useEffect, useRef, useCallback, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { emit, listen } from '@tauri-apps/api/event';
import { Road } from './components/Road';
import { Cyclist } from './components/Cyclist';
import { useTrainer } from './hooks/useTrainer';
import { useKeyboard } from './hooks/useKeyboard';
import { useAppState } from './hooks/useAppState';
import { useWorkout } from './hooks/useWorkout';

const APP_HEIGHT = 74;
const NOTCH_WIDTH = 200;
const TOP_HEIGHT = 37;
const ROAD_HEIGHT = 37;
const HOVER_OFFSET = 65; // px to move down on hover
// 1 screen width = 1 kilometer

function App() {
  const screenWidth = window.innerWidth;
  const cyclistRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const positionRef = useRef(50);
  const lastTimeRef = useRef(0);
  const animationRef = useRef<number>(0);
  const isVisibleRef = useRef(true);
  const isHoveredRef = useRef(false);
  const speedRef = useRef(0);

  // Real trainer data from ANT+ or simulation
  const { data: trainerData, isConnected, isSimulation } = useTrainer();

  // App state and panel management
  const {
    appState,
    panelType,
    startRecording,
    pauseRecording,
    resumeRecording,
    stopRecording,
    confirmStop,
    cancelStop,
    togglePanel,
  } = useAppState();

  // Workout recording (FIT file)
  const workout = useWorkout();

  // Wrap state actions with workout backend calls
  const handleStartRecording = useCallback(() => {
    workout.start();
    startRecording();
  }, [workout, startRecording]);

  const handlePauseRecording = useCallback(() => {
    workout.pause();
    pauseRecording();
  }, [workout, pauseRecording]);

  const handleResumeRecording = useCallback(() => {
    workout.resume();
    resumeRecording();
  }, [workout, resumeRecording]);

  const handleStopRecording = useCallback(() => {
    stopRecording();
  }, [stopRecording]);

  // Keyboard bindings
  const keyBindings = useMemo(() => ({
    'Escape': () => togglePanel('menu'),
    'd': () => togglePanel('devices'),
    'D': () => togglePanel('devices'),
    '?': () => togglePanel('help'),
    'r': () => {
      if (appState === 'idle') handleStartRecording();
    },
    'R': () => {
      if (appState === 'idle') handleStartRecording();
    },
    ' ': () => {
      if (appState === 'recording') handlePauseRecording();
      else if (appState === 'paused') handleResumeRecording();
    },
    's': () => {
      if (appState === 'recording' || appState === 'paused') handleStopRecording();
    },
    'S': () => {
      if (appState === 'recording' || appState === 'paused') handleStopRecording();
    },
  }), [appState, togglePanel, handleStartRecording, handlePauseRecording, handleResumeRecording, handleStopRecording]);

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

  // Listen for recording commands from panel window
  useEffect(() => {
    const unsubs = [
      listen('app:start-recording', () => {
        if (appState === 'idle') handleStartRecording();
      }),
      listen('app:pause-recording', () => {
        if (appState === 'recording') handlePauseRecording();
      }),
      listen('app:resume-recording', () => {
        if (appState === 'paused') handleResumeRecording();
      }),
      listen('app:stop-recording', () => {
        if (appState === 'recording' || appState === 'paused') handleStopRecording();
      }),
      listen('workout:confirmed', () => {
        workout.stop();
        confirmStop();
      }),
      listen('workout:cancelled', () => {
        cancelStop();
      }),
    ];
    return () => { unsubs.forEach(p => p.then(fn => fn())); };
  }, [appState, workout, handleStartRecording, handlePauseRecording, handleResumeRecording, handleStopRecording, confirmStop, cancelStop]);

  // Keep speed ref updated for animation loop
  speedRef.current = trainerData.speed;

  const isMoving = trainerData.power > 0 || trainerData.cadence > 0;

  // Smooth window animation
  const windowYRef = useRef(0);
  const targetYRef = useRef(0);
  const windowAnimRef = useRef<number>(0);
  const leaveTimeoutRef = useRef<number>(0);

  useEffect(() => {
    const animateWindow = () => {
      const current = windowYRef.current;
      const target = targetYRef.current;
      const diff = target - current;

      if (Math.abs(diff) > 0.5) {
        // Ease towards target (0.15 = smoothness factor)
        windowYRef.current += diff * 0.15;
        invoke('set_window_y', { y: Math.round(windowYRef.current) }).catch(() => {});
      }

      windowAnimRef.current = requestAnimationFrame(animateWindow);
    };

    windowAnimRef.current = requestAnimationFrame(animateWindow);
    return () => cancelAnimationFrame(windowAnimRef.current);
  }, []);

  // Handle hover - move window down to reveal status bar
  const handleMouseEnter = useCallback(() => {
    // Cancel any pending return
    if (leaveTimeoutRef.current) {
      clearTimeout(leaveTimeoutRef.current);
      leaveTimeoutRef.current = 0;
    }
    isHoveredRef.current = true;
    targetYRef.current = HOVER_OFFSET;
  }, []);

  const handleMouseLeave = useCallback(() => {
    // Delay before returning to prevent flickering
    leaveTimeoutRef.current = window.setTimeout(() => {
      isHoveredRef.current = false;
      targetYRef.current = 0;
    }, 200); // 200ms delay
  }, []);

  // Pause when tab not visible (battery optimization)
  useEffect(() => {
    const handleVisibility = () => {
      isVisibleRef.current = document.visibilityState === 'visible';
      if (isVisibleRef.current) {
        lastTimeRef.current = 0; // Reset to avoid big delta jump
      }
    };
    document.addEventListener('visibilitychange', handleVisibility);
    return () => document.removeEventListener('visibilitychange', handleVisibility);
  }, []);

  // GPU-accelerated animation with requestAnimationFrame
  useEffect(() => {
    const animate = (timestamp: number) => {
      // Skip if not visible (battery saver)
      if (!isVisibleRef.current) {
        animationRef.current = requestAnimationFrame(animate);
        return;
      }

      if (!lastTimeRef.current) lastTimeRef.current = timestamp;
      const delta = (timestamp - lastTimeRef.current) / 1000;
      lastTimeRef.current = timestamp;

      // Cap delta to avoid jumps after pause
      const cappedDelta = Math.min(delta, 0.1);

      // Update position: 1 screen width = 1 km
      // speed is km/h, so pixels/sec = (speed / 3600) * screenWidth
      const speed = speedRef.current || 0;
      if (speed > 0) {
        const pixelsPerSecond = (speed / 3600) * screenWidth;
        positionRef.current += pixelsPerSecond * cappedDelta;
        if (positionRef.current > screenWidth) {
          positionRef.current = positionRef.current % screenWidth;
        }
      }

      if (cyclistRef.current) {
        cyclistRef.current.style.transform =
          `translateX(${positionRef.current}px) translateY(-50%)`;
      }

      animationRef.current = requestAnimationFrame(animate);
    };

    animationRef.current = requestAnimationFrame(animate);
    return () => cancelAnimationFrame(animationRef.current);
  }, [screenWidth]);

  return (
    <div
      ref={containerRef}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      style={{
        width: '100%',
        height: APP_HEIGHT,
        display: 'grid',
        gridTemplateColumns: `1fr ${NOTCH_WIDTH}px 1fr`,
        gridTemplateRows: `${TOP_HEIGHT}px ${ROAD_HEIGHT}px`,
        background: 'var(--bg-primary)',
        fontFamily: '"SF Mono", Monaco, monospace',
        fontSize: '11px',
      }}>
      {/* === TOP LEFT: Data === */}
      <div style={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'flex-end',
        padding: '0 15px',
        gap: '12px',
      }}>
        <span>♥ {trainerData.heartRate || '--'}</span>
        <span>⚡ {trainerData.power}W</span>
        <span style={{ opacity: 0.6 }}>{trainerData.cadence}rpm</span>
      </div>

      {/* === TOP CENTER: Notch area (empty) === */}
      <div />

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
          {appState === 'recording' && '● '}
          {appState === 'paused' && '❚❚ '}
          {formatTime(trainerData.elapsedTime)}
        </span>
        <span style={{ opacity: 0.5 }}>
          {isConnected ? (isSimulation ? '◐' : '●') : '○'}
        </span>
      </div>

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
    </div>
  );
}

function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = Math.floor(seconds % 60);
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}

export default App;
