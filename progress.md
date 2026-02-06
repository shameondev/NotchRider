# NotchRider Development Progress

> Разработка ведётся только во время тренировок на велостанке (2 раза в неделю).
> Этот файл помогает не терять контекст между сессиями.

---

## 2026-02-06 (~45 min ride)

### What we did
- Implemented workout recording with FIT file export
- Created custom FIT encoder from scratch (~500 lines, zero new dependencies):
  - `src-tauri/src/fit/crc.rs` — CRC16 with half-byte lookup table
  - `src-tauri/src/fit/types.rs` — FIT constants, base types, epoch conversion
  - `src-tauri/src/fit/encoder.rs` — FitEncoder: header, definition/data messages, CRC
  - `src-tauri/src/fit/messages.rs` — Builders for FileId, Event, Record, Lap, Session, Activity
  - `src-tauri/src/fit/mod.rs` — Module re-exports
- Created `src-tauri/src/workout.rs` — WorkoutRecorder with sample recording, stats, FIT encoding, file save
- Added 4 Tauri commands: `start_workout`, `add_workout_sample`, `pause_workout`, `stop_workout`
- Created `src/hooks/useWorkout.ts` — Frontend hook for 1/sec sampling
- Updated `App.tsx` — Wired useWorkout to useAppState, cross-window event listeners
- Updated `Panel.tsx`:
  - Replaced "Trainings — coming soon" with TrainingsView submenu (Start Training [R], History)
  - ConfirmStopView now calls stop_workout and shows SummaryView
  - Added global R/Space/S key forwarding from panel to main window via Tauri events
- Fixed pre-existing test bug in ant/channel.rs (FEC_RF_FREQUENCY → ANT_PLUS_RF_FREQUENCY)
- Fixed: R key now works from both main window and panel (cross-window event forwarding)
- All 28 Rust tests pass, TypeScript compiles cleanly

### Files created
- `src-tauri/src/fit/crc.rs`
- `src-tauri/src/fit/types.rs`
- `src-tauri/src/fit/encoder.rs`
- `src-tauri/src/fit/messages.rs`
- `src-tauri/src/fit/mod.rs`
- `src-tauri/src/workout.rs`
- `src/hooks/useWorkout.ts`

### Files modified
- `src-tauri/src/lib.rs` — Added mod fit/workout, AppState.workout, 4 commands
- `src/App.tsx` — Added useWorkout, cross-window event listeners for recording control
- `src/Panel.tsx` — TrainingsView, ConfirmStopView, SummaryView, key forwarding
- `src-tauri/src/ant/channel.rs` — Fixed test bug

### NOT tested yet
- **Workout recording is NOT tested on real trainer!** Code compiles, tests pass, but no live run yet
- Need to verify: R → record → S → Y → .fit file saved
- Need to upload .fit to Strava to verify format correctness
- Need to check pause/resume handling
- Need to verify file at `~/Library/Application Support/com.notchrider.app/workouts/`

### Next session priorities
1. **Test workout recording live on trainer** — this is the #1 priority
2. Upload .fit file to Strava to verify format
3. Fix any FIT format issues discovered during upload
4. Consider adding NP (Normalized Power) and TSS to summary

---

## 2025-02-04 (47 min 32 sec ride)

### What we did
- Обсудили концепцию keyboard navigation (настройки, hotkeys)
- Создали дизайн-документ `docs/plans/2025-02-04-keyboard-navigation-design.md`
- Реализовали все 6 задач keyboard navigation:
  1. `useKeyboard` hook — глобальный обработчик клавиш
  2. `useAppState` hook — state machine (idle/recording/paused/confirming)
  3. Panel window backend — Rust команды show/hide/toggle
  4. Panel window router — React Router + views (Menu, Devices, Help, ConfirmStop)
  5. Интеграция keyboard с App — все hotkeys работают
  6. `useListNavigation` hook — навигация стрелками в меню
- Исправили баги:
  - Белый фон панели → тёмный (CSS fix)
  - Потеря фокуса после закрытия панели → возврат фокуса на main window

### Keyboard shortcuts implemented
- `Esc` — Menu panel
- `D` — Devices panel
- `?` — Help panel
- `R` — Start recording
- `Space` — Pause/Resume
- `S` — Stop
- `↑↓` — Navigate in menus
- `Enter` — Select

### What's NOT done
- **Тестирование keyboard navigation** — функционал не протестирован вживую!

### Next session priorities
1. Протестировать keyboard navigation на велостанке
2. Убедиться, что все hotkeys работают корректно
3. Проверить panel window (появляется/скрывается, фокус)

### Side discussions
- Брейншторм по pixel art анимации велосипедиста
- Записали концепцию в `docs/plans/2025-02-04-cyclist-animation-design.md`
- Добавили в backlog

---
