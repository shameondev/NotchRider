import { Road } from './components/Road';

function App() {
  // MacBook Pro 14" notch is approximately 200px wide, centered
  const screenWidth = window.innerWidth;
  const notchWidth = 200;
  const notchX = screenWidth / 2;

  return (
    <div style={{
      width: '100%',
      height: '148px',
      background: 'var(--bg-primary)',
      position: 'relative',
    }}>
      <Road notchWidth={notchWidth} notchX={notchX} />
    </div>
  );
}

export default App;
