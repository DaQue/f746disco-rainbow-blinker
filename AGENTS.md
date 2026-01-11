# AGENTS

## Status
✅ Working baseline achieved:
- LTDC framebuffer output (RGB565) on STM32F746G-DISCO panel
- Rainbow test pattern visible
- LED heartbeat blinking
- Text rendered on top of framebuffer (shadowed, readable)
- Flashed via probe-rs runner from `cargo run --release`
- Pushed to GitHub as known-good state

Repo: https://github.com/DaQue/f746disco-rainbow-blinker

---

## What actually broke us (real issues)

1) Wrong way to run the firmware  
- `cargo run` initially tried to execute the ELF on the host → shell error.  
- Fix: use `probe-rs run` and set a Cargo runner in `.cargo/config.toml`.

2) Panic/unwind + target config churn  
- Hit: `unwinding panics are not supported without std`  
- Fix: force `panic = "abort"` and ensure target is `thumbv7em-none-eabihf`.

3) ELF not flashable (“No loadable segments”)  
- probe-rs error: `No loadable segments were found in the ELF file.`  
- Root cause: bad build/runner/link configuration at the time (host-style artifact).  
- Fix: correct `.cargo/config.toml`, clean rebuild, ensure embedded link script in use.

4) Borrow checker vs LTDC framebuffer  
- `config_layer(Layer::L1, framebuffer, …)` takes a `'static mut` reference.  
- This prevented later mutable access to the framebuffer for drawing text.  
- Fix: draw into the framebuffer FIRST, then hand it to LTDC.

5) Terminal paste corruption  
- Large heredoc pastes were sometimes mangled, producing broken Rust source.  
- Fix: paste in smaller chunks or edit directly in nano.

---

## What ended up working (final recipe)

### Cargo runner + target
- `.cargo/config.toml`:
  - `[build] target = "thumbv7em-none-eabihf"`
  - `runner = "probe-rs run --chip STM32F746NG"`
  - panic abort enabled (profiles or rustflags)

### Display bring-up (known-good steps)
- Use 25 MHz HSE clock.
- Pulse PG6 for LCD reset.
- Keep PI12 (DISP_ON) low until LTDC is configured.
- Enable backlight on PK3.
- Configure LTDC pins (mostly AF14, PG12 AF9) with VeryHigh speed.
- Use RGB565 framebuffer in SDRAM.
- Configure Layer 1 with framebuffer and reload.
- Fill framebuffer with rainbow bars to validate LTDC path.
- Render text by writing pixels directly into framebuffer.
- Then give framebuffer to LTDC and reload.

---

## Current expected behavior

- 🌈 Rainbow background with centered counter in a black rounded box  
- 🟧 Black rounded buttons with orange outlines and ▲/▼  
- 💚 Green LED blinking continuously  
- 🧭 `cal` mode shows a single target crosshair and prints 5 touch points  

---

## Notes / cleanup

- `src/screen.rs` has an `unused import: prelude::*` warning (harmless).
- If moving back to embedded-graphics fonts later, wrap the framebuffer
  in a custom DrawTarget instead of writing raw pixels.

---

## High-level log

- 2026-01-10: Working graphics pipeline achieved (LTDC + SDRAM + text + LED).
  Pushed to GitHub as known-good baseline.
# AGENTS

## Status
✅ Working baseline achieved:
- LTDC framebuffer output (RGB565) on STM32F746G-DISCO panel
- Rainbow test pattern visible
- LED heartbeat blinking
- Text rendered on top of framebuffer (shadowed, readable)
- Flashed via probe-rs runner from `cargo run --release`
- Pushed to GitHub as known-good state

Repo: https://github.com/DaQue/f746disco-rainbow-blinker

## What actually broke us (real issues)
1) **Wrong way to run the firmware**
   - `cargo run` tried to execute the target ELF on the host -> shell error.
   - Fix: use `probe-rs run` (and set Cargo runner in `.cargo/config.toml`).

2) **Panic/unwind + target config churn**
   - We hit: `unwinding panics are not supported without std`
   - Fix: set panic abort for embedded (either in Cargo profiles or rustflags),
     and ensure build target is `thumbv7em-none-eabihf`.

3) **ELF not flashable ("No loadable segments")**
   - probe-rs said: `No loadable segments were found in the ELF file.`
   - Root cause: the build/runner/link configuration was temporarily wrong
     (host-ish artifact / bad link script situation).
   - Fix: correct `.cargo/config.toml` target+runner, clean rebuild, ensure
     embedded link script (`link.x`) is actually being used in the embedded build.

4) **Borrow checker fight: framebuffer passed as 'static**
   - After LTDC `config_layer(Layer::L1, framebuffer, ...)`, the framebuffer is
     effectively held as a `'static mut` by the controller API.
   - That made it illegal to mutably borrow the framebuffer again for drawing text.
   - Fix: draw into `framebuffer` FIRST, then call `config_layer()`.

5) **Terminal paste corruption (human factor, but real)**
   - Large heredoc pastes sometimes got mangled, producing broken Rust source.
   - Fix: paste in smaller chunks (truncate + cat >>), or otherwise avoid huge pastes.

## What ended up working (final recipe)
### Cargo runner + target
Use a project-local runner so `cargo run` flashes instead of trying to execute on host:

- `.cargo/config.toml` includes:
  - `[build] target = "thumbv7em-none-eabihf"`
  - runner = `probe-rs run --chip STM32F746NG`
  - (panic abort enabled via profiles or rustflags)

### Display bring-up (known-good steps)
- Use 25 MHz HSE for correct clocking.
- Reset pulse on PG6 for LCD reset.
- Enable display gate PI12 (DISP_ON) only after LTDC configured.
- Enable backlight PK3.
- Configure LTDC pins in AF14 (and PG12 AF9) with VeryHigh speed.
- Use RGB565 framebuffer in SDRAM (static buffer).
- Configure Layer1 to point at the framebuffer and reload.
- Fill framebuffer with color bars to prove LTDC path.
- Render text by writing pixels into framebuffer (simple 6x10 glyphs),
  then hand framebuffer to LTDC.

## Current Behavior (expected)
- Rainbow background with centered counter in a black rounded box
- Black rounded buttons with orange outlines and ▲/▼
- Green LED blinking continuously
- `cal` mode steps through five targets and prints touch coords

## Notes / Cleanup (optional)
- `src/screen.rs` has an `unused import: prelude::*` warning; safe to remove later.
- If we later want "real fonts" again, use embedded-graphics with a custom DrawTarget
  wrapper that writes RGB565 pixels into the framebuffer.

## Build/Flash Log (high level)
- 2026-01-10: Working display pipeline achieved (rainbow + text + LED), pushed to GitHub.
