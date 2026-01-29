import { useState, useEffect, useRef } from 'react';
import { Road } from './components/Road';
import { Cyclist } from './components/Cyclist';
import { HUD } from './components/HUD';
import { StreakDisplay } from './components/StreakDisplay';
import { getRoadY } from './hooks/useRoadPosition';
import { calculateDrift } from './hooks/useZoneDrift';
import { useTrainer } from './hooks/useTrainer';
import { useStreak } from './hooks/useStreak';
import type { TrainerData, TargetZone } from './types/trainer';

function App() {
  const screenWidth = window.innerWidth;
  const notchWidth = 200;
  const notchX = screenWidth / 2;

  const [cyclistX, setCyclistX] = useState(100);
  const [elapsedTime, setElapsedTime] = useState(0);
  const [distance, setDistance] = useState(0);

  const { data: trainerData, isConnected, findDevice } = useTrainer();
  const prevDistanceRef = useRef(0);
  const { streak, update: updateStreak } = useStreak();

  const targetZone: TargetZone = {
    min: 140,
    max: 160,
    metric: 'power',
  };

  // Try to find ANT+ device on mount
  useEffect(() => {
    findDevice();
  }, [findDevice]);

  // Simulated power for when not connected
  const [simulatedPower, setSimulatedPower] = useState(150);
  useEffect(() => {
    if (!isConnected) {
      const interval = setInterval(() => {
        setSimulatedPower(130 + Math.floor(Math.random() * 50));
      }, 500);
      return () => clearInterval(interval);
    }
  }, [isConnected]);

  const currentPower = isConnected ? trainerData.power : simulatedPower;
  const drift = calculateDrift(currentPower, targetZone);

  // Animation loop
  useEffect(() => {
    const startTime = Date.now();
    const interval = setInterval(() => {
      const elapsed = Math.floor((Date.now() - startTime) / 1000);
      setElapsedTime(elapsed);

      // Move cyclist based on speed (or simulated if not connected)
      const speed = isConnected ? trainerData.speed : 30;
      setCyclistX(x => {
        const pixelsPerSecond = speed * 2; // scale for visibility
        const newX = x + pixelsPerSecond / 60;
        return newX > screenWidth ? 0 : newX;
      });

      // Accumulate distance
      setDistance(d => d + (speed / 3.6) / 60); // m/s / 60fps
    }, 1000 / 60);

    return () => clearInterval(interval);
  }, [screenWidth, isConnected, trainerData.speed]);

  // Update streak based on drift state
  useEffect(() => {
    const distanceDelta = distance - prevDistanceRef.current;
    if (distanceDelta > 0) {
      updateStreak(drift.state, distanceDelta);
    }
    prevDistanceRef.current = distance;
  }, [distance, drift.state, updateStreak]);

  const displayData: TrainerData = {
    ...trainerData,
    power: currentPower,
    distance,
    elapsedTime,
    grade: 0,
  };

  const cyclistY = getRoadY(cyclistX, notchX, notchWidth);

  return (
    <div style={{
      width: '100%',
      height: '148px',
      background: 'var(--bg-primary)',
      position: 'relative',
    }}>
      <Road notchWidth={notchWidth} notchX={notchX} />
      <Cyclist
        x={cyclistX}
        y={cyclistY}
        driftOffset={drift.offset}
        driftState={drift.state}
      />
      <HUD data={displayData} targetZone={targetZone} />
      <StreakDisplay streak={streak} />

      {/* Connection status */}
      <div style={{
        position: 'absolute',
        top: '5px',
        right: '10px',
        fontSize: '10px',
        opacity: 0.5,
      }}>
        {isConnected ? '🟢 ANT+' : '🔴 Sim'}
      </div>
    </div>
  );
}

export default App;
