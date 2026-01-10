# AGENTS

## Findings
- LTDC pixel clock was derived from HSI because `DisplayController::new` was passed `None`; the board uses a 25 MHz HSE. This likely prevents the panel from syncing.
- After flashing with HSE enabled, LCD stays white and LED remains off (no visible image).
- After adding LCD reset pulse on PG6, LCD still stays white and LED remains off.
- After switching to RGB565 framebuffer, LCD still stays white and LED remains off.
- After swapping to the HAL screen module and SRAM framebuffer, LCD still stays white and LED remains off.
- After flashing `diag-pins`, no LED blink and no LCD/backlight flicker observed.
- After flashing `diag-bkpt`, debugger did not hit a breakpoint (suggests the image is not executing).

## Next
1) Verify BOOT0/BOOT1 strap, NRST state, power rails, and that the debugger is not holding reset/halt.
2) Confirm the debugger can halt the core (e.g., connect-under-reset) and that the flashed image is the one booted.

## Build/Flash Log
- 2025-09-28: `cargo build` (dev profile) succeeded.
- 2025-09-28: `cargo flash --release --chip STM32F746NGHx` succeeded.
- 2025-09-28: `cargo flash --release --chip STM32F746NGHx` (with LCD reset pulse) succeeded.
- 2025-09-28: `cargo flash --release --chip STM32F746NGHx` (RGB565 framebuffer) succeeded.
- 2025-09-28: `cargo flash --release --chip STM32F746NGHx` (HAL screen module, SRAM framebuffer) succeeded.
- 2025-09-28: `cargo flash --release --chip STM32F746NGHx --features diag-pins` succeeded.
- 2025-09-28: `cargo flash --release --chip STM32F746NGHx --features diag-led` succeeded.
- 2025-09-28: `cargo flash --release --chip STM32F746NGHx --features diag-bkpt` succeeded.
