# CLAUDE.md

## Session Start — READ THIS FIRST

**At the start of every session, read `progress.md`** to restore context.

The developer works on this project only during indoor cycling trainer sessions (~2 per week). Context is lost between sessions. The progress file tracks:
- What was done last session
- What needs testing or finishing
- Current priorities

---

## Repository Overview

NotchRider is a minimalist terminal-aesthetic cycling game for macOS that lives in the menu bar and turns the MacBook notch into a game element. The cyclist rides along a road that wraps around the notch like an island.

## Important: Public Repository

**This is a PUBLIC repository.** Before committing:

- NEVER commit API keys, secrets, or credentials
- NEVER commit `.env` files with real values
- Use `.env.example` with placeholder values
- Check `git diff` before committing to ensure no secrets leak
- All documentation and comments must be in **English**

## Tech Stack

- **Framework:** Tauri v2
- **Backend:** Rust
- **Frontend:** React + TypeScript
- **ANT+ Protocol:** openant / ant-rs
- **Style:** Monospace, terminal aesthetic

## Project Structure

```
NotchRider/
├── src-tauri/          # Rust backend
│   ├── src/
│   │   ├── main.rs     # Tauri entry point
│   │   ├── ant/        # ANT+ communication
│   │   └── game/       # Game state logic
│   └── Cargo.toml
├── src/                # React frontend
│   ├── components/     # UI components
│   ├── hooks/          # Custom hooks
│   └── App.tsx
├── docs/               # Documentation
│   └── design.md       # Design document
└── README.md
```

## Key Concepts

### Window Configuration
- Height: 148px (74px × 2)
- Position: Replaces macOS menu bar
- Properties: borderless, always-on-top, transparent

### Game Mechanics
- Player must maintain target power/heart rate zone
- Going above zone → cyclist drifts down
- Going below zone → cyclist drifts up
- Drifting off road → streak resets

### ANT+ Integration
- Protocol: ANT+ FE-C (Fitness Equipment Control)
- Hardware: ANT USBStick2 or compatible dongle
- Supports any ANT+ FE-C compatible smart trainer

## Development Commands

```bash
# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build for production
pnpm tauri build
```

## Code Style

- Use English for all code, comments, and documentation
- Follow Rust conventions for backend code
- Follow React/TypeScript conventions for frontend
- Keep components small and focused
- Prefer composition over inheritance

## Security Checklist

Before each commit, verify:
- [ ] No hardcoded secrets or API keys
- [ ] No `.env` files with real credentials
- [ ] No private paths or usernames in code
- [ ] All new dependencies are from trusted sources
