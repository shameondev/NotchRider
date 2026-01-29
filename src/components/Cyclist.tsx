import { forwardRef } from 'react';

interface CyclistProps {
  y: number;
  isMoving?: boolean;
}

export const Cyclist = forwardRef<HTMLDivElement, CyclistProps>(
  function Cyclist({ y, isMoving = false }, ref) {
    return (
      <div
        ref={ref}
        data-testid="cyclist"
        style={{
          position: 'absolute',
          left: 0,
          top: y,
          fontSize: '14px',
          opacity: isMoving ? 1 : 0.6,
          willChange: 'transform',
          transform: 'translateX(0px) translateY(-50%)',
        }}
      >
        🚴
      </div>
    );
  }
);
