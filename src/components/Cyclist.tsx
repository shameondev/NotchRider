interface CyclistProps {
  x: number;
  y: number;
  isWarning?: boolean;
}

export function Cyclist({ x, y, isWarning = false }: CyclistProps) {
  return (
    <div
      data-testid="cyclist"
      style={{
        position: 'absolute',
        left: `${x}px`,
        top: `${y}px`,
        fontSize: '20px',
        transform: 'translateX(-50%) translateY(-50%)',
        filter: isWarning ? 'drop-shadow(0 0 8px var(--text-warning))' : 'none',
        transition: 'top 0.3s ease-out, filter 0.2s',
      }}
    >
      🚴
    </div>
  );
}
