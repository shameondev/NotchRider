# NotchRider

A minimalist indoor cycling companion for macOS. Think of it as a lightweight alternative to Zwift that lives in your menu bar.

```
┌──────────────────────────────────────────────────────────────────┐
│  ♥ 142  ⚡ 165W  85rpm  │  ▓▓▓  │  28.5km/h  22.5km  ● 45:23  🟢 │
├──────────────────────────┬───────┬───────────────────────────────┤
│ ═══🚴═══════════════════ │ NOTCH │ ══════════════════════════════│
└──────────────────────────┴───────┴───────────────────────────────┘
```

## What is NotchRider?

NotchRider is a minimalist workout tracking app for indoor cycling. It connects to your ANT+ smart trainer and displays your workout data in a thin strip at the top of your screen — right next to the MacBook notch.

**The key idea:** The app takes almost no screen space. You can watch Netflix, work, or browse the web while keeping an eye on your power, heart rate, and other metrics. No need for a dedicated screen or tablet.

## Features

- **Minimal footprint** — 148px strip that replaces your menu bar
- **ANT+ support** — connects to any FE-C compatible smart trainer
- **Heart rate monitoring** — ANT+ HRM support
- **Workout recording** — track and save your rides
- **Keyboard-driven** — navigate with hotkeys, no mouse needed
- **Terminal aesthetic** — clean monospace design

## Supported Hardware

Works with any ANT+ FE-C compatible trainer:
- Wahoo KICKR, KICKR Core, KICKR Snap
- Tacx Neo, Flux, Flow
- Elite Direto, Suito, Zumo
- CYCPLUS T2
- Saris H3
- And many more...

Requires an ANT+ USB dongle (e.g., Garmin ANT+ Stick, Dynastream ANT USBStick2).

## Roadmap

### Current
- [x] ANT+ trainer connection
- [x] Real-time power, speed, cadence
- [x] Heart rate monitoring
- [x] Keyboard navigation
- [x] Workout recording

### Planned
- [ ] Strava/Garmin Connect export
- [ ] FIT/TCX file export
- [ ] Structured workouts
- [ ] Workout import (ZWO, ERG, MRC)
- [ ] Online multiplayer (ghost riders, group rides)
- [ ] Resistance control for ERG mode
- [ ] Apple Watch integration

## Tech Stack

- **Tauri v2** — native app framework
- **Rust** — backend, ANT+ protocol
- **React + TypeScript** — frontend
- **macOS** — primary platform (Windows support planned)

## Development

```bash
# Prerequisites: Rust, Node.js, pnpm

git clone https://github.com/user/NotchRider.git
cd NotchRider

pnpm install
pnpm tauri dev
```

## About This Project

This entire app is built with AI assistance. I develop it exclusively while training on my indoor bike — no coding happens off the saddle. It's an experiment in building useful software during workout sessions.

## Contributing

Contributions welcome! Feel free to open an issue or PR.

## License

MIT License

---

*Made with ❤️ while pedaling with [Claude Code](https://claude.ai/code) by [@shameondev](https://strava.app.link/yoxVmoFsu0b)*
