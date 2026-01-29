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
      display: 'flex',
      gap: '10px',
      fontSize: '9px',
      opacity: 0.9,
    }}>
      <span style={{
        color: streak.isActive ? 'var(--text-primary)' : 'var(--text-danger)'
      }}>
        🔥{formatDistance(streak.current)}
      </span>
      <span style={{ color: 'var(--text-secondary)' }}>
        🏆{formatDistance(streak.best)}
      </span>
    </div>
  );
}
