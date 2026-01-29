import type { DriftState } from '../hooks/useZoneDrift';

interface CyclistProps {
  x: number;
  y: number;
  driftOffset: number;
  driftState: DriftState;
}

export function Cyclist({ x, y, driftOffset, driftState }: CyclistProps) {
  const isWarning = driftState === 'tooFast' || driftState === 'tooSlow';
  const isOffRoad = driftState === 'offRoad';

  return (
    <>
      <div
        data-testid="cyclist"
        style={{
          position: 'absolute',
          left: `${x}px`,
          top: `${y + driftOffset}px`,
          fontSize: '20px',
          transform: 'translateX(-50%) translateY(-50%)',
          filter: isWarning
            ? 'drop-shadow(0 0 8px var(--text-warning))'
            : isOffRoad
            ? 'drop-shadow(0 0 12px var(--text-danger))'
            : 'none',
          transition: 'top 0.3s ease-out, filter 0.2s',
        }}
      >
        🚴
      </div>

      {/* Warning indicator */}
      {(isWarning || isOffRoad) && (
        <div
          style={{
            position: 'absolute',
            left: `${x}px`,
            top: `${y + driftOffset + 25}px`,
            transform: 'translateX(-50%)',
            fontSize: '10px',
            color: isOffRoad ? 'var(--text-danger)' : 'var(--text-warning)',
          }}
        >
          ⚠️ {isOffRoad ? 'OFF ROAD!' : 'WARNING'}
        </div>
      )}
    </>
  );
}
