# ROADMAP

## Status
- Working baseline achieved: LTDC framebuffer (RGB565), rainbow test, shadowed text, LED heartbeat.
- Flashed via probe-rs runner from `cargo run --release`.
- Known-good state pushed to GitHub: https://github.com/DaQue/f746disco-rainbow-blinker

## Guardrails
- Do not re-break LCD bring-up.
- Treat this repo as hardware truth.
- Build new features strictly on top of the existing working framebuffer pipeline.
- Refactors must be extraction-only and behavior-preserving: no init order/timing changes unless required.

## What actually broke us (real issues)
1) Wrong way to run firmware
   - `cargo run` tried to execute the ELF on host.
   - Fix: use `probe-rs run` and set a Cargo runner in `.cargo/config.toml`.
2) Panic/unwind + target config churn
   - Error: `unwinding panics are not supported without std`.
   - Fix: `panic = "abort"`, target `thumbv7em-none-eabihf`.
3) ELF not flashable ("No loadable segments")
   - Root cause: bad build/runner/link configuration (host-style artifact).
   - Fix: correct `.cargo/config.toml`, clean rebuild, ensure `link.x` is used.
4) Borrow checker vs LTDC framebuffer
   - `config_layer(Layer::L1, framebuffer, ...)` takes `'static mut`.
   - Fix: draw into framebuffer first, then hand to LTDC.
5) Terminal paste corruption
   - Large heredoc pastes sometimes mangled Rust source.
   - Fix: paste smaller chunks or edit directly in editor.

## Current expected behavior
- Rainbow color bars across screen.
- Shadowed magenta text ("HELLO / STM32F746").
- Green LED blinking continuously.

## Notes / cleanup
- `src/screen.rs` has an unused `prelude::*` import (harmless).
- If returning to embedded-graphics fonts later, wrap framebuffer in a DrawTarget.

## Updated plan (serial first)

### Step 1 — Serial Phase 1: write only
Goal: prove you can print reliably (no blocking, no LCD changes).

Do:
- Initialize USART/VCP printing (ST-LINK VCP).
- Print once at boot: `boot ok`.
- Optional: heartbeat every 1s: `alive t=...`.

Exit:
- `boot ok` appears on every reset/flash in minicom/screen.
- Main loop continues (LED still blinks, LCD still works).

Status:
- DONE: `boot ok` and `alive t=...` verified in minicom (ST-LINK VCP / USART1).

### Step 2 — Serial Phase 2: read + write
Goal: accept simple commands and echo responses (still no UI dependence).

Commands:
- `get` -> prints current value.
- `set N` -> sets value (clamped -999..999) and prints it.
- `inc` / `dec` -> changes by ±1 and prints it.
- `help` -> prints command list.

Rules:
- Non-blocking RX (never stall the loop).
- Fixed buffer, safe overflow behavior.
- Echo + `\r\n` newlines.

Exit:
- Commands update internal state and print responses reliably.

Status:
- DONE: commands verified on ST-LINK VCP (case-insensitive, echo off in minicom).

### Step 3 — Draw counter only
Goal: big centered number using existing framebuffer + shadowed text path.

Do:
- Render current value (same one serial modifies).
- Redraw only when value changes.

Exit:
- Screen shows centered value.
- Serial `inc/dec/set` changes the on-screen number.

### Step 4 — Draw buttons (still no touch)
Goal: two buttons below counter with ▲ / ▼ (or UP/DN), looks presentable.

Exit:
- UI layout is correct and readable.

### Step 5 — Touch visual debug
Goal: draw a dot/crosshair under your finger; fix axis/inversion mapping.

Exit:
- Touch coordinates match the screen.

### Step 6 — Tap buttons (edge-detected)
Goal: tap UP increments once; tap DOWN decrements once (no repeats yet).

Exit:
- Reliable single-step per tap.

### Step 7 — Hold auto-repeat + ramp
Goal: press-and-hold repeats slowly, then ramps to a capped speed.

Exit:
- Feels good, not insane, clamps at ±999.

## Planned refactor chunks (safe extraction order)
1) Extract rendering helpers first (lowest risk): `src/render.rs`.
2) Extract demo logic: `src/demo/` (counter + layout).
3) Keep LTDC/SDRAM init in `src/main.rs` until platform is stable.

## Verification checklist (run after every chunk)
- Board boots reliably.
- LCD is lit and shows expected content.
- No regressions in text rendering.
- LED heartbeat still runs.
- If touch is involved: coordinate sanity confirmed.

## Commit discipline
- One chunk per commit.
- Commit messages reference chunk number, e.g.:
  - "Step1: serial boot ok"
  - "Step3: render centered counter"
  - "Step7: hold ramp repeat"
