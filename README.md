# NotchRider 🚴

A minimalist terminal-aesthetic cycling game for macOS that lives in your menu bar and turns the MacBook notch into a game element.

```
┌─────────────────────────────┬─────────┬─────────────────────────────┐
│ ═══🚴═══════════════════╲   │ ▓▓▓▓▓▓▓ │   ╱═════════════════════════ │
│  ♥89                     ╲  │ ▓NOTCH▓ │  ╱                   23:15  │
│                           ╲ │ ▓▓▓▓▓▓▓ │ ╱                           │
├────────────────────────────╲┴─────────┴╱────────────────────────────┤
│  ⚡148W  [▲5%]  🎯150W     ═══🚴═══════              12.4km         │
└──────────────────────────────────────────────────────────────────────┘
```

## Concept

- **148px tall strip** that replaces your macOS menu bar
- Cyclist rides left-to-right, **wrapping around the notch** like an island
- Connect your **ANT+ smart trainer** and ride!
- Keep your power/heart rate in the target zone to stay on the road
- Watch Netflix while you train — the game takes minimal screen space

## Features

- 🖥️ **Notch-aware UI** — the road wraps around the MacBook notch
- 🚴 **ANT+ FE-C support** — works with any compatible smart trainer
- 🎯 **Zone training** — stay in your power/HR zone or drift off the road
- 📊 **Minimal HUD** — power, heart rate, distance, time, grade
- 🏆 **Streak system** — track your longest time in zone

## Supported Hardware

Any ANT+ FE-C compatible smart trainer:
- Wahoo KICKR
- Tacx Neo
- Elite trainers
- CYCPLUS T2
- Saris trainers
- And more...

Requires an ANT+ USB dongle (e.g., ANT USBStick2).

## Tech Stack

- **Tauri v2** — lightweight native app framework
- **Rust** — backend, ANT+ communication
- **React + TypeScript** — frontend UI
- **Terminal aesthetic** — monospace fonts, minimal colors

## Development

```bash
# Prerequisites
# - Rust & Cargo
# - Node.js & pnpm
# - Xcode Command Line Tools (macOS)

# Clone the repo
git clone https://github.com/shameondev/NotchRider.git
cd NotchRider

# Install dependencies
pnpm install

# Run in dev mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## Roadmap

### MVP
- [ ] Tauri app with borderless 148px window
- [ ] ANT+ dongle connection
- [ ] Read trainer data (power, speed, cadence)
- [ ] Road rendering with notch wrap-around
- [ ] Target zone + drift mechanics
- [ ] Basic HUD

### v2
- [ ] Voice commands
- [ ] Multiplayer (ghost riders)
- [ ] Terrain with resistance control
- [ ] Workout recording & export

## License

MIT License — see [LICENSE](LICENSE)

## Contributing

Contributions welcome! Please read [CLAUDE.md](CLAUDE.md) for development guidelines.

---

*Made with ❤️ and 🚴 by [@shameondev](https://github.com/shameondev)*
