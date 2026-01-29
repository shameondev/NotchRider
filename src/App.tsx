import { useState, useEffect } from 'react';
import { Road } from './components/Road';
import { Cyclist } from './components/Cyclist';
import { HUD } from './components/HUD';
import { getRoadY } from './hooks/useRoadPosition';
import { calculateDrift } from './hooks/useZoneDrift';
import type { TrainerData, TargetZone } from './types/trainer';

function App() {
  const screenWidth = window.innerWidth;
  const notchWidth = 200;
  const notchX = screenWidth / 2;

  const [cyclistX, setCyclistX] = useState(100);

  const [trainerData, setTrainerData] = useState<TrainerData>({
    power: 148,
    speed: 32.5,
    cadence: 90,
    heartRate: 142,
    distance: 0,
    elapsedTime: 0,
    grade: 0,
  });

  const targetZone: TargetZone = {
    min: 140,
    max: 160,
    metric: 'power',
  };

  useEffect(() => {
    const startTime = Date.now();
    const interval = setInterval(() => {
      const elapsed = Math.floor((Date.now() - startTime) / 1000);

      setCyclistX(x => {
        const newX = x + 2;
        return newX > screenWidth ? 0 : newX;
      });

      setTrainerData(d => ({
        ...d,
        distance: d.distance + 0.5,
        elapsedTime: elapsed,
        // Simulate power fluctuation (sometimes outside zone)
        power: 130 + Math.floor(Math.random() * 50),
      }));
    }, 1000 / 60);

    return () => clearInterval(interval);
  }, [screenWidth]);

  const cyclistY = getRoadY(cyclistX, notchX, notchWidth);
  const currentValue = targetZone.metric === 'power'
    ? trainerData.power
    : trainerData.heartRate;
  const drift = calculateDrift(currentValue, targetZone);

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
      <HUD data={trainerData} targetZone={targetZone} />
    </div>
  );
}

export default App;
