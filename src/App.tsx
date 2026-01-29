import { useState, useEffect } from 'react';
import { Road } from './components/Road';
import { Cyclist } from './components/Cyclist';
import { getRoadY } from './hooks/useRoadPosition';

function App() {
  const screenWidth = window.innerWidth;
  const notchWidth = 200;
  const notchX = screenWidth / 2;

  const [cyclistX, setCyclistX] = useState(100);
  const speed = 2; // pixels per frame

  useEffect(() => {
    const interval = setInterval(() => {
      setCyclistX(x => {
        const newX = x + speed;
        // Loop back to start when reaching end
        return newX > screenWidth ? 0 : newX;
      });
    }, 1000 / 60); // 60fps

    return () => clearInterval(interval);
  }, [screenWidth]);

  const cyclistY = getRoadY(cyclistX, notchX, notchWidth);

  return (
    <div style={{
      width: '100%',
      height: '148px',
      background: 'var(--bg-primary)',
      position: 'relative',
    }}>
      <Road notchWidth={notchWidth} notchX={notchX} />
      <Cyclist x={cyclistX} y={cyclistY} />
    </div>
  );
}

export default App;
