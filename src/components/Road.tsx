interface RoadProps {
  notchWidth: number;
  notchX: number; // center X position of notch
  height?: number;
}

export function Road({ notchWidth, notchX, height = 148 }: RoadProps) {
  const halfNotch = notchWidth / 2;
  const leftEnd = notchX - halfNotch;
  const rightStart = notchX + halfNotch;

  const roadStyle = {
    position: 'absolute' as const,
    height: '4px',
    background: 'var(--road-color, #444)',
    top: '35px', // center of upper 74px row
  };

  return (
    <div data-testid="road" style={{ position: 'relative', width: '100%', height }}>
      {/* Left road segment */}
      <div
        data-testid="road-left"
        style={{
          ...roadStyle,
          left: 0,
          width: leftEnd,
        }}
      />

      {/* Diagonal down to lower road */}
      <svg
        style={{ position: 'absolute', left: leftEnd - 20, top: 0 }}
        width="40"
        height="148"
      >
        <path
          d={`M 20 37 Q 30 37, 35 74 L 35 111 Q 30 111, 20 111`}
          stroke="var(--road-color, #444)"
          strokeWidth="4"
          fill="none"
        />
      </svg>

      {/* Lower road under notch */}
      <div
        data-testid="road-under-notch"
        style={{
          position: 'absolute',
          height: '4px',
          background: 'var(--road-color, #444)',
          top: '109px', // center of lower 74px row
          left: leftEnd,
          width: notchWidth,
        }}
      />

      {/* Diagonal up from lower road */}
      <svg
        style={{ position: 'absolute', left: rightStart - 20, top: 0 }}
        width="40"
        height="148"
      >
        <path
          d={`M 20 111 Q 30 111, 35 74 L 35 37 Q 30 37, 20 37`}
          stroke="var(--road-color, #444)"
          strokeWidth="4"
          fill="none"
        />
      </svg>

      {/* Right road segment */}
      <div
        data-testid="road-right"
        style={{
          ...roadStyle,
          left: rightStart,
          width: `calc(100% - ${rightStart}px)`,
        }}
      />
    </div>
  );
}
