import type { TrainerData, TargetZone } from '../types/trainer';

interface HUDProps {
  data: TrainerData;
  targetZone: TargetZone;
}

function formatTime(seconds: number): string {
  const mins = Math.floor(seconds / 60);
  const secs = seconds % 60;
  return `${mins}:${secs.toString().padStart(2, '0')}`;
}

function formatDistance(meters: number): string {
  return (meters / 1000).toFixed(1);
}

export function HUD({ data, targetZone }: HUDProps) {
  const gradeSymbol = data.grade >= 0 ? '▲' : '▼';
  const gradeValue = Math.abs(data.grade);

  const currentValue = targetZone.metric === 'power' ? data.power : data.heartRate;
  const inZone = currentValue >= targetZone.min && currentValue <= targetZone.max;

  return (
    <div
      data-testid="hud"
      style={{
        position: 'absolute',
        bottom: '10px',
        left: '20px',
        right: '20px',
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'center',
        fontSize: '14px',
        fontFamily: 'inherit',
      }}
    >
      {/* Left: Heart rate and power */}
      <div style={{ display: 'flex', gap: '20px' }}>
        <span>♥ {data.heartRate}</span>
        <span style={{ color: inZone ? 'var(--text-primary)' : 'var(--text-warning)' }}>
          ⚡ {data.power}W
        </span>
        <span style={{ opacity: 0.7 }}>
          [{gradeSymbol} {gradeValue}%]
        </span>
      </div>

      {/* Center: Target zone */}
      <div>
        🎯 {targetZone.min}-{targetZone.max}W
      </div>

      {/* Right: Distance and time */}
      <div style={{ display: 'flex', gap: '20px' }}>
        <span>{formatDistance(data.distance)}km</span>
        <span>{formatTime(data.elapsedTime)}</span>
      </div>
    </div>
  );
}
