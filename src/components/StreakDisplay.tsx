import type { StreakState } from '../hooks/useStreak';

interface StreakDisplayProps {
  streak: StreakState;
}

function formatDistance(meters: number): string {
  if (meters < 1000) {
    return `${Math.floor(meters)}m`;
  }
  return `${(meters / 1000).toFixed(2)}km`;
}

export function StreakDisplay({ streak }: StreakDisplayProps) {
  return (
    <div style={{
      position: 'absolute',
      top: '5px',
      left: '50%',
      transform: 'translateX(-50%)',
      fontSize: '12px',
      display: 'flex',
      gap: '15px',
      opacity: 0.8,
    }}>
      <span style={{
        color: streak.isActive ? 'var(--text-primary)' : 'var(--text-danger)'
      }}>
        🔥 {formatDistance(streak.current)}
      </span>
      <span style={{ color: 'var(--text-secondary)' }}>
        🏆 {formatDistance(streak.best)}
      </span>
    </div>
  );
}
