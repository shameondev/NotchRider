function App() {
  return (
    <div style={{
      width: '100%',
      height: '148px',
      background: 'var(--bg-primary)',
      display: 'flex',
      flexDirection: 'column',
    }}>
      {/* Upper row: 74px - road with notch */}
      <div style={{
        height: '74px',
        background: 'var(--bg-secondary)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        color: 'var(--text-primary)',
      }}>
        Upper Road (74px)
      </div>

      {/* Lower row: 74px - road under notch + HUD */}
      <div style={{
        height: '74px',
        background: 'var(--bg-primary)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        color: 'var(--text-secondary)',
      }}>
        Lower Road + HUD (74px)
      </div>
    </div>
  );
}

export default App;
