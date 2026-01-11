# f746disco-lcd

Known-good LTDC + SDRAM framebuffer bring-up for the STM32F746G-DISCO, plus
serial control over ST-LINK VCP.

## Status
- Rainbow background with counter UI (black rounded box, orange digits, black rounded buttons with orange arrows).
- LED heartbeat blinking.
- Serial (ST-LINK VCP) TX/RX working with simple commands.

## Build + flash
- `cargo run --release`

## Serial (ST-LINK VCP)
USART1 is used for VCP on this board. Current pin mapping in code:
- TX: PA9 (AF7)
- RX: PB7 (AF7)

Baud: 115200 8N1.

Commands (case-insensitive):
- `get`
- `set N`
- `inc`
- `dec`
- `help`

Alive output:
- `alive t=...` prints ~1Hz when idle, pauses for ~3s after RX activity.

## Notes
- LCD bring-up order is sensitive; see `ROADMAP.md` for guardrails.
