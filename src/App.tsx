import { useEffect, useRef, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Road } from './components/Road';
import { Cyclist } from './components/Cyclist';
import { useTrainer } from './hooks/useTrainer';

const APP_HEIGHT = 74;
const NOTCH_WIDTH = 200;
const TOP_HEIGHT = 37;
const ROAD_HEIGHT = 37;
const PIXELS_PER_KMH = 5;
const HOVER_OFFSET = 65; // px to move down on hover

function App() {
  const screenWidth = window.innerWidth;
  const cyclistRef = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const positionRef = useRef(50);
  const lastTimeRef = useRef(0);
  const animationRef = useRef<number>(0);
  const isVisibleRef = useRef(true);
  const isHoveredRef = useRef(false);

  // Real trainer data from ANT+ or simulation
  const { data: trainerData, isConnected, isSimulation } = useTrainer();

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
    let frameSkipCounter = 0;

    const animate = (timestamp: number) => {
      // Skip if not visible (battery saver)
      if (!isVisibleRef.current) {
        animationRef.current = requestAnimationFrame(animate);
        return;
      }

      // Throttle to 30fps when hovered (less important, save CPU)
      if (isHoveredRef.current) {
        frameSkipCounter++;
        if (frameSkipCounter < 2) {
          animationRef.current = requestAnimationFrame(animate);
          return;
        }
        frameSkipCounter = 0;
      }

      if (!lastTimeRef.current) lastTimeRef.current = timestamp;
      const delta = (timestamp - lastTimeRef.current) / 1000;
      lastTimeRef.current = timestamp;

      // Cap delta to avoid jumps after pause
      const cappedDelta = Math.min(delta, 0.1);

      // Update position directly via transform (GPU)
      const speed = trainerData.speed || 0;
      if (speed > 0) {
        positionRef.current += speed * PIXELS_PER_KMH * cappedDelta;
        if (positionRef.current > screenWidth) {
          positionRef.current = 0;
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
        <span>{formatTime(trainerData.elapsedTime)}</span>
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
