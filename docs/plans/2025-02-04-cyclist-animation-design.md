# Cyclist Animation Design

**Date:** 2025-02-04
**Status:** Concept / Brainstorm

---

## Problem

Current cyclist is a static emoji 🚴 that:
- Doesn't pedal
- Moves backwards (no direction)
- No animation, no life
- Can't scale for intro sequence

---

## Vision

A pixel art cyclist with soul:
- Pedals rotate synced to real cadence
- Intro sequence: close-up → zoom out as user starts pedaling
- Retro 80s game aesthetic, but with grace
- Parallax backgrounds (mountains passing by)

---

## Chosen Approach: Pixel Art

### Why Pixel Art

| Criteria | Pixel Art Advantage |
|----------|---------------------|
| Window size (74px height) | Natural fit for 16-32px sprites |
| Animation | Simple frame-by-frame, 4-8 frames = alive |
| Scaling | CSS `scale()` keeps pixels crisp |
| Independence | No artist dependency, iterate freely |
| Learning curve | Manageable for non-artists |
| Aesthetic match | "80s games with soul" = pixel art |

### Alternatives Considered

- **SVG/Vector**: Overkill for 16px, complex animation rigging
- **AI Generation**: Inconsistent style, can't animate coherently
- **Pre-made assets**: Limits creative control, dependency

---

## Animation Concepts

### 1. Pedaling Cycle

```
4-8 frames cycling through leg positions
Speed tied to cadence: frameIndex = cadence % totalFrames
Fast cadence = fast animation, slow = slow
```

### 2. Start Sequence (Intro)

Triggered by cumulative pedal rotations, not time:

```
[0 rotations]    Rider stands, looks at bike (close-up, 32×32 or larger)
[1-2 rotations]  Puts on helmet
[3-4 rotations]  Gets on bike
[5+ rotations]   Camera zooms out, riding begins (16×16)
```

User controls animation speed through pedaling speed.

### 3. Scaling System

```
Intro (close-up):  32×32 sprite → displayed at 2x = 64×64
Normal riding:     16×16 sprite → displayed at 1x = 16×16
```

---

## Sprite Requirements

### Minimum Viable Set

1. **Riding cycle** (side view, right direction)
   - 4-6 frames of pedaling
   - Size: 16×16 or 16×24

2. **Standing pose** (side view)
   - 1 frame, larger: 32×32
   - For intro close-up

3. **Helmet animation** (optional for v1)
   - 3-4 frames
   - Same scale as standing

4. **Mounting bike** (optional for v1)
   - 3-4 frames
   - Transition from standing to riding

### Future Additions

- Different rider skins/colors
- Tired animation (low power)
- Sprint animation (high power)
- Celebration (streak milestone)

---

## Technical Implementation Notes

### Sprite Sheet Format

```
cyclist-sprites.png (single file with all frames)

Layout example:
┌────┬────┬────┬────┬────┬────┐
│ F1 │ F2 │ F3 │ F4 │ F5 │ F6 │  ← Pedaling cycle
├────┴────┴────┴────┴────┴────┤
│         Standing            │  ← Larger frame
└─────────────────────────────┘
```

### Animation in React

```tsx
// Concept - not final code
const frameIndex = Math.floor((cadence / 60) * Date.now() / 100) % frameCount;
const spriteOffset = frameIndex * frameWidth;

style={{
  backgroundImage: 'url(cyclist-sprites.png)',
  backgroundPosition: `-${spriteOffset}px 0`,
  width: frameWidth,
  height: frameHeight,
}}
```

---

## Tools

### Aseprite ($19.99 one-time)

- Industry standard for pixel art animation
- Onion skinning (see previous/next frames while drawing)
- Export to sprite sheets
- Timeline for animation
- https://www.aseprite.org/

### Free Alternatives

- **Piskel** (web-based): https://www.piskelapp.com/
- **Libresprite** (Aseprite fork): https://libresprite.github.io/

---

## Open Questions

1. Exact sprite sizes (16×16 vs 16×24 for riding?)
2. Color palette (match terminal aesthetic?)
3. Background parallax layers (mountains, trees?)
4. How to handle notch wrap-around visually?

---

## Next Steps (When Ready)

1. [ ] Install Aseprite or Piskel
2. [ ] Find pixel art cyclist references
3. [ ] Draw first test sprite (16×16 side view)
4. [ ] Create 4-frame pedaling cycle
5. [ ] Implement sprite animation system in React
6. [ ] Test cadence-to-animation sync
