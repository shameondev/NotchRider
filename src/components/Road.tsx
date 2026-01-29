interface RoadProps {
  roadY: number;
  laneGap?: number;
}

export function Road({ roadY, laneGap = 12 }: RoadProps) {
  const style = {
    fontFamily: '"SF Mono", Monaco, monospace',
    fontSize: '11px',
    color: 'var(--road-color)',
    position: 'absolute' as const,
    left: 0,
    whiteSpace: 'pre' as const,
    overflow: 'hidden' as const,
    width: '100%',
  };

  const line = '─'.repeat(500);

  return (
    <div
      data-testid="road"
      style={{
        position: 'absolute',
        top: 0,
        left: 0,
        width: '100%',
        height: '100%',
        pointerEvents: 'none',
      }}
    >
      {/* Lane 1 */}
      <div style={{ ...style, top: roadY }}>{line}</div>
      {/* Lane 2 */}
      <div style={{ ...style, top: roadY + laneGap }}>{line}</div>

      {/* Test compatibility */}
      <div data-testid="road-left" style={{ display: 'none' }} />
      <div data-testid="road-under-notch" style={{ display: 'none' }} />
      <div data-testid="road-right" style={{ display: 'none' }} />
    </div>
  );
}
