# DEMO_TOUCH_COUNTER_PLAN.md
Project: STM32F746G-DISCO graphics platform in Rust
Repo: https://github.com/DaQue/f746disco-rainbow-blinker
Branch: demo-touch-counter
Baseline: LTDC+SDRAM+RGB565 framebuffer working; rainbow test; shadowed text; green LED heartbeat; flashed via probe-rs.

## Prime directive
- DO NOT re-break LCD bring-up.
- Treat this repo as hardware truth.
- Build features strictly on top of the existing working framebuffer pipeline.
- Refactors must be behavior-preserving: no changes to init order or timing unless explicitly required.

## Current state (checkpoint)
- We have a known-working baseline on GitHub that we can revert to.
- We created a working branch:
  - `git checkout -b demo-touch-counter`
- main.rs is ~389 lines. That’s the right time for a *surgical* refactor (extraction only).
- Counter UI now renders: rainbow background, black rounded box with orange digits, black rounded buttons with orange outlines/arrows.

## Demo goal: "Touch Counter"
UI:
- Big centered integer counter: range -999..999, step ±1.
- Two touch buttons below the counter:
  - UP button increments
  - DOWN button decrements
- Integer-only (no float).
- Tap triggers once (edge-detected).
- Hold triggers auto-repeat:
  - starts slow after a short delay
  - ramps up smoothly
  - caps at a reasonable maximum speed (not insane)
- Use existing shadowed text rendering and RGB565 framebuffer writes.

## Implementation approach (layering)
Keep hardware bring-up stable; build UI logic on top.
- Layer A: hardware init (clocks/SDRAM/LTDC/backlight) stays in main.rs for now.
- Layer B: framebuffer drawing helpers (rainbow, shapes, text wrappers)
- Layer C: demo/app logic (state machine + touch → actions)

## Refactor rule (IMPORTANT)
This is NOT a rewrite. It is extraction-only.
- No logic changes.
- No reordering of init steps.
- One chunk per commit, verify on hardware every time.

Target after first refactor pass:
- main.rs ~150–220 lines: just init + loop + calls into modules.

Status:
- DONE: main.rs now ~300 lines with logic split into modules.

## Work plan (in the correct order)

### Chunk 0 — Guardrail setup (done)
- Branch exists: demo-touch-counter
- Baseline known good

Exit: board boots, rainbow/text/LED still work.

### Chunk 1 — Draw counter only (no touch, no buttons)
Goal: show a big centered "0" using the existing text renderer.
Tasks:
- Add `src/demo_counter.rs` (or `src/demo/counter.rs`) with:
  - `CounterDemo { value: i16 }`
  - `render_counter_only(fb, value, center_x, center_y)`
- Hardcode value=0
- Background fill (black)
- Centering uses integer math

Exit:
- Big centered 0 displayed, LED heartbeat still works.

Status:
- DONE.

### Chunk 2 — Increase font size (≈3×)
Goal: counter is clearly larger but still crisp and centered.
Tasks:
- Add a “big font” path:
  - Preferred: second bitmap font table (big glyphs)
  - Acceptable: scale rendering if already supported cleanly
- Verify centering for -999, 0, 999

Exit:
- Large counter renders cleanly; fits screen.

Status:
- DONE (16x24 digits).

### Chunk 3 — Draw buttons (still no touch)
Goal: two buttons below counter, aligned and readable.
Tasks:
- Add `Layout` with fixed rects for 480x272:
  - up_btn rect
  - dn_btn rect
- Draw:
  - button outlines (or rounded)
  - labels: "UP"/"DN" or triangles ▲/▼ (triangles via primitives preferred)

Exit:
- UI looks like a real panel: big number + two buttons.

Status:
- DONE (black-filled buttons with orange outlines/arrows).

### Chunk 4 — Touch input visual debug (no state changes yet)
Goal: confirm touch coordinates match LCD coordinates.
Tasks:
- Implement `touch_read() -> Option<(x,y)>` (screen pixels)
- Draw a small crosshair/dot at touch point
- Add mapping function if needed:
  - swap axes / invert X / invert Y until correct

Exit:
- Touch indicator appears under finger reliably.

Status:
- DONE: `cal` mode steps through five targets and prints coords.

### Chunk 5 — Tap behavior (edge-detected)
Goal: tap UP increments once; tap DOWN decrements once.
Tasks:
- Add edge detection:
  - `just_pressed = touch_now && !touch_was_down`
- On just_pressed:
  - if in up_btn: value = min(value+1, 999)
  - if in dn_btn: value = max(value-1, -999)
- Only redraw when value changes

Exit:
- Tap works reliably; no repeated steps while holding.

Status:
- DONE: touch buttons update the counter once per tap.

### Chunk 6 — Button press visual feedback (polish)
Goal: button looks pressed while finger is down.
Tasks:
- Track held button (if touch down inside rect)
- Render pressed style (fill/invert)

Exit:
- UI responds visually during press.

### Chunk 7 — Hold auto-repeat (slow start)
Goal: holding triggers repeats slowly after delay.
Suggested parameters:
- start delay: ~350ms
- initial repeat: ~6/sec (every ~166ms)

Tasks:
- Track:
  - hold_button (Up/Down/None)
  - hold_ms
  - repeat_accum_ms
- If holding same button:
  - after delay, apply step every repeat interval

Exit:
- Hold changes value at a controlled slow rate.

Status:
- DONE: hold repeats after delay at a steady rate.

### Chunk 8 — Ramp speed up with a cap (still reasonable)
Goal: speed increases smoothly but never becomes insane.
Suggested schedule (integer-only):
- 0–350ms: no repeat
- 350–1000ms: 6/sec (166ms)
- 1–2s: 12/sec (83ms)
- 2–4s: 20/sec (50ms)
- 4s+: cap at 30/sec (33ms)

Tasks:
- `repeat_interval_ms(hold_ms)` via if/match ladder
- Keep clamp at ±999

Exit:
- Feels like a phone spinner: slow → faster → capped.

### Chunk 9 — Cleanup for “showable demo”
Tasks:
- Title text: "Touch Counter"
- Small instructions: "Tap or hold"
- Minor spacing/color polish
- Optional: redraw only dirty areas (counter + buttons) if needed

Exit:
- Looks good on video, code is understandable.

## Planned refactor chunks (safe extraction order)
1) Extract rendering helpers first (lowest risk):
   - Create `src/render.rs`
   - Move rainbow + primitives + text wrapper functions
   - main.rs calls into render functions
2) Extract demo logic:
   - `src/demo/` module (counter demo state + layout)
3) Keep LTDC/SDRAM init in main.rs until platform is stable.

## Verification checklist (run after every chunk)
- Board boots reliably
- LCD is lit and shows expected content
- No regressions in text rendering
- LED heartbeat still runs
- If touch is involved: coordinate sanity confirmed

## Commit discipline
- One chunk per commit
- Commit messages reference chunk number, e.g.:
  - "Chunk1: render centered counter"
  - "Chunk4: touch debug crosshair"
  - "Chunk8: hold ramp repeat"

End.
