# NotchRider — Design Document

**Date:** 2025-01-29
**Status:** Draft

---

## Concept

A minimalist terminal-aesthetic cycling game for macOS that lives in the menu bar and transforms the MacBook notch into a game element. The cyclist rides along a road from left to right, wrapping around the notch like it's an island.

**Key Feature:** The app replaces the system menu bar, and the notch becomes part of the game world — an obstacle to ride around.

---

## Tech Stack

| Component | Technology |
|-----------|------------|
| Framework | Tauri v2 |
| Backend | Rust |
| Frontend | React + TypeScript |
| ANT+ | openant / ant-rs |
| Style | Monospace, terminal aesthetic |

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         macOS                                │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────────────┐    │
│  │              Tauri App (borderless)                 │    │
│  │  ┌───────────────────┐  ┌───────────────────────┐   │    │
│  │  │   Rust Backend    │  │   React Frontend      │   │    │
│  │  │                   │  │                       │   │    │
│  │  │  • ANT+ driver    │◄─┤  • Road renderer      │   │    │
│  │  │  • Trainer comms  │  │  • HUD (stats)        │   │    │
│  │  │  • Game state     │─►│  • Animations         │   │    │
│  │  │  • Future: WS     │  │  • Target zones       │   │    │
│  │  └───────────────────┘  └───────────────────────┘   │    │
│  └─────────────────────────────────────────────────────┘    │
│                              │                               │
│                              ▼                               │
│                    ┌─────────────────┐                      │
│                    │  ANT USBStick2  │                      │
│                    └────────┬────────┘                      │
│                             │ ANT+ FE-C                     │
│                             ▼                               │
│                    ┌─────────────────┐                      │
│                    │  Smart Trainer  │                      │
│                    └─────────────────┘                      │
└─────────────────────────────────────────────────────────────┘
```

### Rust Backend

- Connect to ANT+ dongle via openant or ant-rs
- Receive data: power (W), speed (km/h), cadence (RPM), heart rate
- Send commands to trainer: resistance changes
- Manage game state: position, streak, zone status

### React Frontend

- Render road with notch wrap-around
- Display HUD with statistics
- Animate cyclist and other riders
- Visual effects (warning, glow)

### Rust ↔ React Communication

- Tauri commands (call Rust from JS)
- Tauri events (push data ~10-20 fps)

---

## UI/UX

### Window Size & Position

- **Height:** 148px (74px × 2)
- **Width:** Full screen width
- **Position:** Replaces macOS system menu bar
- **Notch:** 74px height, ~200px width

### Tauri Window Properties

```rust
decorations: false,    // no close/minimize buttons
always_on_top: true,   // above other windows
transparent: true,     // transparent background
resizable: false,      // fixed height
```

### Layout

```
┌─────────────────────────────┬─────────┬─────────────────────────────┐
│ ═══🚴═══════════════════╲   │ ▓▓▓▓▓▓▓ │   ╱═════════════════════════ │
│  data                    ╲  │ ▓NOTCH▓ │  ╱                    data  │  74px
│                           ╲ │ ▓▓▓▓▓▓▓ │ ╱                           │
├────────────────────────────╲┴─────────┴╱────────────────────────────┤
│  ♥89  ⚡148W  [▲5%]        ═══🚴═══════        12.4km   23:15  🎯150W │  74px
└──────────────────────────────────────────────────────────────────────┘
```

### Cyclist Path

1. Rides along upper road (menu bar level) →
2. Approaches notch, turns downward ↘
3. Rides along lower road, passing under notch →
4. Turns back upward ↗
5. Continues along upper road →

**Cyclist is always visible** — notch acts like an island in the middle of the road.

### Interactions

- Right-click → context menu (settings, exit)
- Mouse hover on top edge → system menu bar appears
- Voice control (future)

---

## Game Mechanics

### Core Gameplay — "Stay in Zone"

Player must maintain power/heart rate within target zone. Zone is calculated from user's FTP.

```
Target zone:  [====🎯 140-160W ====]

Current:              ⚡ 148W ✓  (in zone — ride straight)
                      ⚡ 172W ⚠  (above zone — drift down)
                      ⚡ 118W ⚠  (below zone — drift up)
```

### Drift Visualization

```
Normal (in zone):      ═══════🚴═══════

Too fast:              ═══════════════
                              🚴 ↓ (drifts down)
                              ⚠️ WARNING

Too slow:                     🚴 ↑ (drifts up)
                       ═══════════════
                       ⚠️ WARNING
```

### Tracked Metrics

- **Power (W)** — for strength training
- **Heart Rate (BPM)** — for cardio zone (requires HRM)
- **Cadence (RPM)** — for pedaling technique

### Streak System

- Stay in zone → streak distance accumulates
- Drift off road → streak resets
- Records: best streak, total session distance

### Difficulty Levels

- 🟢 Easy: ±20W range
- 🟡 Medium: ±10W range
- 🔴 Hard: ±5W range

### Terrain Visualization

- **Grade indicator in HUD:** `[▲ 8%]` or `[▼ 3%]`
- **Animation speed:** uphill = slower, downhill = faster
- Trainer adjusts resistance to match terrain

---

## Hardware

### Supported Protocols

- **ANT+ FE-C** — standard for smart trainers
- Via ANT USBStick2 or compatible dongle

### Compatible Trainers

Any ANT+ FE-C compatible:
- CYCPLUS T2 ✓
- Wahoo KICKR ✓
- Tacx Neo ✓
- Elite ✓
- Saris ✓
- And more...

### Data Received

| Metric | Source |
|--------|--------|
| Power (W) | Trainer |
| Speed (km/h) | Trainer |
| Cadence (RPM) | Trainer |
| Heart Rate (BPM) | HRM sensor (optional) |

### Commands Sent

- Resistance changes (for terrain simulation)
- Target power (ERG mode)

---

## Multiplayer (v2)

### Phase 1 — Async Ghosts

- Friends record their rides
- You see their "ghosts" on the road
- No real-time synchronization required

```
═══🚴you═══════🚴‍♂️ghostA═══════🚴ghostB═══════════════════
```

### Phase 2 — Live Multiplayer

- WebSocket server
- Position sync ~10 times/sec
- See who's passing you, who you're passing

**Protocol:**
```json
{
  "user_id": "abc123",
  "speed": 32.5,
  "power": 185,
  "position": 12450,
  "timestamp": 1706540000000
}
```

---

## MVP Scope

### Included in MVP

- [ ] Tauri app with borderless 148px window
- [ ] ANT+ dongle connection
- [ ] Read trainer data (power, speed, cadence)
- [ ] Road rendering with notch wrap-around
- [ ] Cyclist animation
- [ ] Target zone + drift visualization
- [ ] Basic HUD (power, HR, distance, time, grade)
- [ ] Streak system with records

### Deferred to v2+

- Voice control
- Multiplayer (ghosts and live)
- Terrain with trainer resistance control
- FTP/zone settings UI
- Support for Macs without notch
- Workout recording and export

---

## Open Questions

1. **Name:** NotchRider — final or working title?
2. **Sound:** Do we need sound effects?
3. **Themes:** Light/dark theme or dark only?
4. **Onboarding:** How to help users set up ANT+ dongle?

---

## Resources & Links

**ANT+ Libraries:**
- [openant](https://github.com/Tigge/openant) — Python
- [ant-rs](https://crates.io/crates/ant) — Rust

**Tauri:**
- [Tauri v2 docs](https://v2.tauri.app/)
- [Borderless windows](https://v2.tauri.app/reference/config/#windowconfig)

**Inspiration:**
- Zwift — virtual cycling
- Pole Position — retro racing
- Chase HQ — arcade chase games

**Protocols:**
- [ANT+ FE-C](https://www.thisisant.com/developer/ant-plus/device-profiles/#521_tab)
- [Bluetooth FTMS](https://www.bluetooth.com/specifications/specs/fitness-machine-service-1-0/)
