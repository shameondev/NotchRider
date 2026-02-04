# NotchRider Development Progress

> Разработка ведётся только во время тренировок на велостанке (2 раза в неделю).
> Этот файл помогает не терять контекст между сессиями.

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
