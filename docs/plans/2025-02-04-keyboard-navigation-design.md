# Keyboard Navigation & UI Panels Design

**Date:** 2025-02-04
**Status:** Approved

---

## Overview

Manual workout recording with keyboard-driven navigation, split-view panels, and device management UI.

**Key changes from previous design:**
- ~~Auto-recording~~ → Manual start with `[R]` or menu
- Split-view panels (40% left, 60% game)
- Terminal-style navigation (arrows, space, enter)
- ANT+ status indicator with auto-connect

---

## Section 1: Keyboard Navigation

### Global Hotkeys (always active)

| Key | Action |
|-----|--------|
| **Esc** | Open/close main menu |
| **D** | Open Devices panel |
| **R** | Quick Start recording (Free Ride) |
| **?** | Open Help (keyboard shortcuts) |

### Recording Hotkeys (during recording)

| Key | Action |
|-----|--------|
| **Space** | Pause / Resume (toggle) |
| **S** | Stop (with confirmation) |

### Panel Hotkeys (when panel is open)

| Key | Action |
|-----|--------|
| **↑↓** | Navigate list |
| **Space** | Select/deselect item |
| **Enter** | Confirm action |
| **Esc** | Close panel |
| **Y/N** | Answer dialogs |

### Main Menu (Esc)

- Devices
- Trainings
  - Start Training → Free Ride
  - History
- Settings
- About

---

## Section 2: UI Components

### Layout (split-view)

```
Normal mode (100% game):
┌─────────────────────────────────────────────────────────────────┐
│ TOP ROW: HUD data + status indicator                        │ 37px
├─────────────────────────────────────────────────────────────────┤
│ BOTTOM ROW: Road + Cyclist + [?] hint                       │ 37px
└─────────────────────────────────────────────────────────────────┘

Panel open (40% panel + 60% game):
┌──────────────┬──────────────────────────────────────────────────┐
│  PANEL (40%) │              GAME AREA (60%)                     │
│              │  continues running, compressed                   │
└──────────────┴──────────────────────────────────────────────────┘
```

### ANT+ Status Indicator (top right)

| Status | Icon | Meaning |
|--------|------|---------|
| Disconnected | 🔴 | Dongle not found or no sensors |
| Partial | 🟡 | Connected but missing sensors / issues |
| Connected | 🟢 | All selected devices connected |

### Recording Indicator (next to time)

| State | Display |
|-------|---------|
| Not recording | `45:23` |
| Recording | `🔴 45:23` (blinking) |
| Paused | `⏸ 45:23` |

### HUD Layout

```
┌──────────────────────────────────────────────────────────────────┐
│  ♥ 142  ⚡ 165W  85rpm  │  ▓▓▓  │  28.5km/h  22.5km  🔴 45:23 🟢 │
└──────────────────────────────────────────────────────────────────┘
  └─── sensor data ───┘    notch    └── speed/dist ──┘ └rec┘└ant┘
```

### Help Hint

`[?]` indicator in bottom-left corner as reminder.

---

## Section 3: State Machine

### App States

```
                              ┌─────────────┐
                              │    IDLE     │ ← app start
                              │ (free ride) │
                              └──────┬──────┘
                                     │ [R] or Menu→Start
                                     ▼
                              ┌─────────────┐
                      ┌──────►│  RECORDING  │◄──────┐
                      │       │             │       │
                      │       └──────┬──────┘       │
                      │              │ [Space]      │ [Space]
                      │              ▼              │
                      │       ┌─────────────┐       │
                      │       │   PAUSED    │───────┘
                      │       │             │
                      │       └──────┬──────┘
                      │              │ [S]
                      │              ▼
                      │       ┌─────────────┐
                      │       │ CONFIRMING  │ ← "Finish workout?"
                      │       │   STOP      │
                      │       └──────┬──────┘
                      │         [N]  │  [Y]
                      └──────────────┘   │
                                         ▼
                              ┌─────────────┐
                              │   SAVING    │ → file saved
                              └──────┬──────┘
                                     │
                                     ▼
                              ┌─────────────┐
                              │    IDLE     │
                              └─────────────┘
```

### Panel States

```
┌──────────┐  [Esc]   ┌──────────┐  [Enter]  ┌──────────┐
│  CLOSED  │ ◄──────► │   MENU   │ ────────► │ SUBMENU  │
└──────────┘  [Esc]   └──────────┘           └────┬─────┘
     │                                             │ [Esc]
     │ [D]                                         │
     ▼                                             ▼
┌──────────┐ ◄─────────────────────────────────────┘
│ DEVICES  │
└──────────┘
```

### Pause Notification

```
PAUSED ──(5 min)──► macOS notification: "Workout paused. Don't forget to save!"
                    └── click notification → focus app
```

### ANT+ Connection States

```
┌────────────┐     dongle found      ┌────────────┐
│ DISCONNECTED│ ──────────────────► │  SCANNING  │
│     🔴     │                       │     🟡     │
└────────────┘ ◄──────────────────── └─────┬──────┘
                  dongle removed           │ devices found
                                           ▼
                                    ┌────────────┐
                                    │ CONNECTED  │
                                    │     🟢     │
                                    └────────────┘
```

---

## Section 4: Devices Panel

### UI

```
┌────────────────────────┬──────────────────────────────────────────┐
│  DEVICES          [D]  │  ═══🚴═══════════════════════════════    │
│  ───────────────────── │                                          │
│  ANT+ Dongle      🟢   │  ♥ 142  ⚡ 165W  85rpm    28.5km/h  🟢   │
│  ───────────────────── │                                          │
│  Scanning...      🔄   │  ← auto-scan on panel open               │
│                        │                                          │
│  TRAINERS              │                                          │
│  > [✓] CYCPLUS T2  🟢  │  ← Last Used, auto-connected             │
│    [ ] Wahoo KICKR 🟡  │  ← found during scan                     │
│                        │                                          │
│  HEART RATE            │                                          │
│    [✓] Garmin HRM  🟢  │  ← Last Used                             │
│                        │                                          │
│  ───────────────────── │                                          │
│  [↑↓] [Space] Select   │                                          │
│  [Esc] Close           │                                          │
└────────────────────────┴──────────────────────────────────────────┘
```

### Device Categories

| Category | ANT+ Device Type | Data |
|----------|-----------------|------|
| Trainers | FE-C (17) | Power, Speed, Cadence |
| Heart Rate | HRM (120) | Heart Rate |
| Cadence | Cadence (122) | Cadence only |
| Speed | Speed (123) | Speed only |
| Power Meter | Power (11) | Power, Cadence |

### Device States

| Icon | State |
|------|-------|
| 🔴 | Not connected / not found |
| 🟡 | Found, connecting... |
| 🟢 | Connected, data flowing |
| ⚠️ | Connected but no data >10sec |

### Auto-connect Flow

```
App Start
    │
    ▼
┌─────────────────┐
│ Load config.json│
│ (last used IDs) │
└────────┬────────┘
         │
         ▼
┌─────────────────┐
│ Auto-connect to │ ← background process
│ last used       │
└────────┬────────┘
         │ success
         ▼
    🟢 Ready
```

### Panel Open Behavior

1. Show Last Used devices immediately (with current status)
2. Auto-start scanning
3. New devices appear in list as found
4. Selecting new device → update `lastUsedDevices`

---

## Section 5: Data Structures

### File Structure

```
~/Library/Application Support/com.notchrider.app/
├── config.json              ← app settings
└── workouts/
    ├── 2024-02-04T18-30-00.json
    ├── 2024-02-03T09-15-00.json
    └── ...
```

### config.json

```json
{
  "version": 1,
  "lastUsedDevices": {
    "trainer": { "deviceId": 12345, "name": "CYCPLUS T2", "type": "FE-C" },
    "heartRate": { "deviceId": 67890, "name": "Garmin HRM", "type": "HRM" }
  },
  "autoConnect": true,
  "units": "metric"
}
```

### workout.json

```json
{
  "id": "2024-02-04T18-30-00",
  "version": 1,
  "type": "free_ride",
  "startTime": "2024-02-04T18:30:00Z",
  "endTime": "2024-02-04T19:15:23Z",
  "devices": {
    "trainer": { "name": "CYCPLUS T2", "deviceId": 12345 },
    "heartRate": { "name": "Garmin HRM", "deviceId": 67890 }
  },
  "summary": {
    "duration": 2723,
    "distance": 22.5,
    "avgPower": 165,
    "maxPower": 320,
    "avgHeartRate": 142,
    "maxHeartRate": 175,
    "avgCadence": 85,
    "avgSpeed": 29.7
  },
  "samples": [
    { "t": 0, "power": 0, "hr": 85, "cadence": 0, "speed": 0 },
    { "t": 1, "power": 120, "hr": 88, "cadence": 65, "speed": 18.5 },
    { "t": 2, "power": 145, "hr": 92, "cadence": 78, "speed": 24.2 }
  ]
}
```

### TypeScript Types (Frontend)

```typescript
interface WorkoutSummary {
  duration: number;      // seconds
  distance: number;      // km
  avgPower: number;      // W
  maxPower: number;
  avgHeartRate: number;  // BPM
  maxHeartRate: number;
  avgCadence: number;    // RPM
  avgSpeed: number;      // km/h
}

interface WorkoutSample {
  t: number;        // second from start
  power: number;
  hr: number;
  cadence: number;
  speed: number;
}

interface Workout {
  id: string;
  type: 'free_ride' | 'structured';
  startTime: string;
  endTime: string;
  summary: WorkoutSummary;
  samples: WorkoutSample[];
}

type AppState = 'idle' | 'recording' | 'paused' | 'confirming' | 'saving';
type PanelState = 'closed' | 'menu' | 'devices' | 'trainings' | 'history' | 'settings' | 'help';
type AntStatus = 'disconnected' | 'scanning' | 'connected';
```

### Rust Structs (Backend)

```rust
#[derive(Serialize, Deserialize)]
pub struct WorkoutSample {
    pub t: u32,
    pub power: u16,
    pub hr: u8,
    pub cadence: u8,
    pub speed: f32,
}

#[derive(Serialize, Deserialize)]
pub struct WorkoutSummary {
    pub duration: u32,
    pub distance: f32,
    pub avg_power: u16,
    pub max_power: u16,
    pub avg_heart_rate: u8,
    pub max_heart_rate: u8,
    pub avg_cadence: u8,
    pub avg_speed: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Workout {
    pub id: String,
    pub workout_type: String,
    pub start_time: String,
    pub end_time: String,
    pub summary: WorkoutSummary,
    pub samples: Vec<WorkoutSample>,
}
```

### Sample Recording

- Frequency: 1 sample per second
- During pause: no samples recorded
- Max size: ~1 hour = 3600 samples ≈ 150KB JSON

---

## Section 6: Implementation Plan

### Phase 1: Keyboard Foundation
1. **Keyboard event system** — global key listener in React
2. **App state machine** — `idle | recording | paused | confirming`
3. **Help panel** — `[?]` shows hotkeys

### Phase 2: Panel System
4. **Panel component** — base left panel component (40%)
5. **Split-view layout** — game compression when panel open
6. **Menu panel** — Esc → section list
7. **Panel navigation** — ↑↓, Enter, Esc inside panels

### Phase 3: Devices
8. **ANT+ device scanning** — device discovery (backend)
9. **Devices panel UI** — device list, selection
10. **Auto-connect** — connect to last used on startup
11. **Status indicator** — 🔴🟡🟢 in HUD
12. **Config persistence** — save/load config.json

### Phase 4: Workout Recording
13. **useWorkout hook** — recording state machine, samples
14. **Recording indicator** — 🔴 in HUD
15. **Pause/Resume** — Space toggle
16. **Stop confirmation** — S → panel "Finish workout?"
17. **Save workout** — write JSON file (backend)
18. **Pause notification** — macOS notification after 5 min

### Phase 5: History & Menu
19. **Trainings menu** — Start Training → Free Ride
20. **History panel** — workout list
21. **Workout details** — view summary
22. **Settings panel** — placeholder for future

### Dependencies

```
Phase 1 ──► Phase 2 ──► Phase 3
                │           │
                ▼           ▼
            Phase 5 ◄── Phase 4
```

---

## Future Enhancements

- **Strava Integration** — Share button in History details
- **FIT/TCX Export** — Convert JSON to standard formats
- **Structured Workouts** — Interval training support
- **Garmin Connect** — Direct upload
