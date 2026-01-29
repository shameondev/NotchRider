/**
 * Calculate Y position on the road given X position.
 * Road goes: flat at 37px → dips to 109px around notch → back to 37px
 */
export function getRoadY(
  x: number,
  notchX: number,
  notchWidth: number
): number {
  const halfNotch = notchWidth / 2;
  const transitionWidth = 50; // pixels for diagonal transition

  const leftTransitionStart = notchX - halfNotch - transitionWidth;
  const leftTransitionEnd = notchX - halfNotch;
  const rightTransitionStart = notchX + halfNotch;
  const rightTransitionEnd = notchX + halfNotch + transitionWidth;

  const upperY = 37;
  const lowerY = 109;

  if (x < leftTransitionStart) {
    return upperY;
  } else if (x < leftTransitionEnd) {
    // Transitioning down
    const progress = (x - leftTransitionStart) / transitionWidth;
    return upperY + (lowerY - upperY) * progress;
  } else if (x < rightTransitionStart) {
    return lowerY;
  } else if (x < rightTransitionEnd) {
    // Transitioning up
    const progress = (x - rightTransitionStart) / transitionWidth;
    return lowerY - (lowerY - upperY) * progress;
  } else {
    return upperY;
  }
}
