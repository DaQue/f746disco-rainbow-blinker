#![no_std]
#![no_main]

use panic_halt as _;

#[cfg(feature = "diag-led")]
use cortex_m::asm::delay as busy_delay;
#[cfg(feature = "diag-led")]
use cortex_m_rt::entry;
#[cfg(feature = "diag-led")]
use stm32f7xx_hal as hal;
#[cfg(feature = "diag-led")]
use hal::{pac, prelude::*};

#[cfg(feature = "diag-bkpt")]
use cortex_m::asm::{bkpt, delay as busy_delay};
#[cfg(feature = "diag-bkpt")]
use cortex_m_rt::entry;
#[cfg(feature = "diag-bkpt")]
use stm32f7xx_hal as hal;
#[cfg(feature = "diag-bkpt")]
use hal::{pac, prelude::*};

#[cfg(feature = "diag-pins")]
use cortex_m::asm::delay as busy_delay;
#[cfg(feature = "diag-pins")]
use cortex_m_rt::entry;
#[cfg(feature = "diag-pins")]
use stm32f7xx_hal as hal;
#[cfg(feature = "diag-pins")]
use hal::{pac, prelude::*};

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
use core::fmt::Write;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
use nb::Error as NbError;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
use cortex_m::asm::delay as busy_delay;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
use cortex_m_rt::entry;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
use stm32f7xx_hal as hal;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
use hal::{
    gpio::Speed,
    ltdc::{Layer, PixelFormat},
    pac,
    prelude::*,
    rcc::{HSEClock, HSEClockMode},
    serial::{Config, Serial},
};

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
use crate::screen::Stm32F7DiscoDisplay;

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
mod screen;

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
const WIDTH: usize = 480;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
const HEIGHT: usize = 272;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
const FB_SIZE: usize = WIDTH * HEIGHT;

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
const WHITE_RGB565: u16 = 0xFFFF;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
const YELLOW_RGB565: u16 = 0xFFE0;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
const CYAN_RGB565: u16 = 0x07FF;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
const GREEN_RGB565: u16 = 0x07E0;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
const MAGENTA_RGB565: u16 = 0xF81F;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
const RED_RGB565: u16 = 0xF800;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
const BLUE_RGB565: u16 = 0x001F;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
const BLACK_RGB565: u16 = 0x0000;

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
static mut FB_LAYER1: [u16; FB_SIZE] = [0; FB_SIZE];

#[cfg(feature = "diag-led")]
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.freeze();

    let gpioi = dp.GPIOI.split();
    let mut led = gpioi.pi1.into_push_pull_output();

    loop {
        led.set_high();
        busy_delay(8_000_000);
        led.set_low();
        busy_delay(8_000_000);
    }
}

#[cfg(feature = "diag-bkpt")]
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.sysclk(216.MHz()).freeze();

    loop {
        bkpt();
        busy_delay(216_000_000 / 4);
    }
}

#[cfg(feature = "diag-pins")]
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.sysclk(216.MHz()).freeze();

    let gpiog = dp.GPIOG.split();
    let gpioi = dp.GPIOI.split();
    let gpiok = dp.GPIOK.split();

    let mut led = gpioi.pi1.into_push_pull_output();
    let mut disp_on = gpioi.pi12.into_push_pull_output();
    let mut backlight = gpiok.pk3.into_push_pull_output();
    let mut lcd_reset = gpiog.pg6.into_push_pull_output();

    loop {
        led.set_high();
        disp_on.set_low();
        backlight.set_high();
        lcd_reset.set_low();
        busy_delay(216_000_000 / 8);

        led.set_low();
        disp_on.set_high();
        backlight.set_low();
        lcd_reset.set_high();
        busy_delay(216_000_000 / 8);
    }
}

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
fn put_px_rgb565(framebuffer: &mut [u16], x: i32, y: i32, color: u16) {
    if x < 0 || y < 0 {
        return;
    }
    let (x, y) = (x as usize, y as usize);
    if x >= WIDTH || y >= HEIGHT {
        return;
    }
    framebuffer[x + WIDTH * y] = color;
}

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
fn glyph_6x10(c: u8) -> [u8; 10] {
    match c {
        b' ' => [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        b'H' => [0b100001, 0b100001, 0b100001, 0b111111, 0b100001, 0b100001, 0b100001, 0, 0, 0],
        b'E' => [0b111111, 0b100000, 0b100000, 0b111110, 0b100000, 0b100000, 0b111111, 0, 0, 0],
        b'L' => [0b100000, 0b100000, 0b100000, 0b100000, 0b100000, 0b100000, 0b111111, 0, 0, 0],
        b'O' => [0b011110, 0b100001, 0b100001, 0b100001, 0b100001, 0b100001, 0b011110, 0, 0, 0],
        b'S' => [0b011111, 0b100000, 0b100000, 0b011110, 0b000001, 0b000001, 0b111110, 0, 0, 0],
        b'T' => [0b111111, 0b001100, 0b001100, 0b001100, 0b001100, 0b001100, 0b001100, 0, 0, 0],
        b'M' => [0b100001, 0b110011, 0b101101, 0b100001, 0b100001, 0b100001, 0b100001, 0, 0, 0],
        b'F' => [0b111111, 0b100000, 0b100000, 0b111110, 0b100000, 0b100000, 0b100000, 0, 0, 0],
        b'2' => [0b011110, 0b100001, 0b000001, 0b000110, 0b011000, 0b100000, 0b111111, 0, 0, 0],
        b'3' => [0b111110, 0b000001, 0b000001, 0b011110, 0b000001, 0b000001, 0b111110, 0, 0, 0],
        b'4' => [0b000110, 0b001010, 0b010010, 0b100010, 0b111111, 0b000010, 0b000010, 0, 0, 0],
        b'6' => [0b011110, 0b100000, 0b100000, 0b111110, 0b100001, 0b100001, 0b011110, 0, 0, 0],
        b'7' => [0b111111, 0b000001, 0b000010, 0b000100, 0b001000, 0b010000, 0b010000, 0, 0, 0],
        _ => [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    }
}

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
fn draw_text_6x10(framebuffer: &mut [u16], x: i32, y: i32, s: &str, color: u16) {
    let mut cx = x;
    for &b in s.as_bytes() {
        let g = glyph_6x10(b);
        for (row, bits) in g.iter().enumerate() {
            for col in 0..6 {
                if (bits & (1 << (5 - col))) != 0 {
                    put_px_rgb565(framebuffer, cx + col as i32, y + row as i32, color);
                }
            }
        }
        cx += 7; // 6px glyph + 1px spacing
    }
}

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
fn busy_delay_ms(cpu_hz: u32, ms: u32) {
    let cycles = (cpu_hz as u64 * ms as u64) / 1000;
    let cycles = cycles.max(1) as u32;
    busy_delay(cycles);
}

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
fn clamp_counter(value: i16) -> i16 {
    if value > 999 {
        999
    } else if value < -999 {
        -999
    } else {
        value
    }
}

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
fn handle_serial_command(line: &str, value: &mut i16, tx: &mut impl core::fmt::Write) {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return;
    }

    if trimmed.eq_ignore_ascii_case("get") {
        let _ = write!(tx, "{}\r\n", *value);
    } else if trimmed.eq_ignore_ascii_case("inc") {
        *value = clamp_counter(value.wrapping_add(1));
        let _ = write!(tx, "{}\r\n", *value);
    } else if trimmed.eq_ignore_ascii_case("dec") {
        *value = clamp_counter(value.wrapping_sub(1));
        let _ = write!(tx, "{}\r\n", *value);
    } else if trimmed.eq_ignore_ascii_case("help") {
        let _ = write!(
            tx,
            "get\r\nset N\r\ninc\r\ndec\r\nhelp\r\n"
        );
    } else if trimmed.len() >= 3
        && trimmed.as_bytes()[..3].eq_ignore_ascii_case(b"set")
    {
        let arg = trimmed.get(3..).unwrap_or("").trim();
        match arg.parse::<i16>() {
            Ok(parsed) => {
                *value = clamp_counter(parsed);
                let _ = write!(tx, "{}\r\n", *value);
            }
            Err(_) => {
                let _ = write!(tx, "err\r\n");
            }
        }
    } else {
        let _ = write!(tx, "err\r\n");
    }
}

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let hse = HSEClock::new(25_000_000.Hz(), HSEClockMode::Oscillator);
    let clocks = rcc.cfgr.hse(hse).sysclk(216.MHz()).hclk(216.MHz()).freeze();
    let cpu_hz = clocks.sysclk().raw();

    let mut delay = cp.SYST.delay(&clocks);

    let gpioa = dp.GPIOA.split();
    let gpiob = dp.GPIOB.split();
    let gpioe = dp.GPIOE.split();
    let gpiog = dp.GPIOG.split();
    let gpioh = dp.GPIOH.split();
    let gpioi = dp.GPIOI.split();
    let gpioj = dp.GPIOJ.split();
    let gpiok = dp.GPIOK.split();

    let mut led = gpioi.pi1.into_push_pull_output();

    let tx = gpioa.pa9.into_alternate::<7>();
    let rx = gpiob.pb7.into_alternate::<7>();
    let serial = Serial::new(dp.USART1, (tx, rx), &clocks, Config::default());
    let (mut tx, mut rx) = serial.split();
    let _ = write!(tx, "boot ok\r\n");

    for _ in 0..10 {
        led.set_high();
        busy_delay_ms(cpu_hz, 125);
        led.set_low();
        busy_delay_ms(cpu_hz, 125);
    }

    let mut lcd_reset = gpiog.pg6.into_push_pull_output();
    lcd_reset.set_low();
    delay.delay_ms(20u16);
    lcd_reset.set_high();
    delay.delay_ms(20u16);

    gpioe.pe4.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiog.pg12.into_alternate::<9>().set_speed(Speed::VeryHigh);
    gpioi.pi9.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioi.pi10.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioi.pi13.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioi.pi14.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioi.pi15.into_alternate::<14>().set_speed(Speed::VeryHigh);

    gpioj.pj0.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj1.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj2.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj3.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj4.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj5.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj6.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj7.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj8.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj9.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj10.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj11.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj13.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj14.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioj.pj15.into_alternate::<14>().set_speed(Speed::VeryHigh);

    gpiok.pk0.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiok.pk1.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiok.pk2.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiok.pk4.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiok.pk5.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiok.pk6.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpiok.pk7.into_alternate::<14>().set_speed(Speed::VeryHigh);

    gpioh.ph1.into_floating_input();

    let mut disp_on = gpioi.pi12.into_push_pull_output();
    disp_on.set_low();

    let mut backlight = gpiok.pk3.into_push_pull_output();
    backlight.set_high();

    let mut display = Stm32F7DiscoDisplay::new(dp.LTDC, dp.DMA2D, &hse);
    let framebuffer = unsafe { &mut *core::ptr::addr_of_mut!(FB_LAYER1) };

    let bars: [u16; 8] = [WHITE_RGB565, YELLOW_RGB565, CYAN_RGB565, GREEN_RGB565, MAGENTA_RGB565, RED_RGB565, BLUE_RGB565, BLACK_RGB565];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let bar = (x * bars.len()) / WIDTH;
            framebuffer[x + WIDTH * y] = bars[bar];
        }
    }

    
disp_on.set_high();

draw_text_6x10(framebuffer, 21, 41, "HELLO", BLACK_RGB565);
draw_text_6x10(framebuffer, 21, 61, "STM32F746", BLACK_RGB565);
draw_text_6x10(framebuffer, 20, 40, "HELLO", MAGENTA_RGB565);
draw_text_6x10(framebuffer, 20, 60, "STM32F746", MAGENTA_RGB565);

display.controller.config_layer(Layer::L1, framebuffer, PixelFormat::RGB565);
display.controller.enable_layer(Layer::L1);
display.controller.reload();

display.controller.reload();
 
    let mut led_on = false;
    let mut half_sec_ticks: u32 = 0;
    let mut seconds: u32 = 0;
    let mut counter_value: i16 = 0;
    let mut line_buf = [0u8; 64];
    let mut line_len: usize = 0;
    let mut rx_idle_ticks: u32 = 0;
    loop {
        loop {
            match rx.read() {
                Ok(byte) => {
                    rx_idle_ticks = 0;
                    if byte == b'\r' || byte == b'\n' {
                        if line_len > 0 {
                            if let Ok(line) = core::str::from_utf8(&line_buf[..line_len]) {
                                handle_serial_command(line, &mut counter_value, &mut tx);
                            } else {
                                let _ = write!(tx, "err\r\n");
                            }
                            line_len = 0;
                        }
                    } else {
                        let _ = write!(tx, "{}", byte as char);
                        if line_len < line_buf.len() {
                            line_buf[line_len] = byte;
                            line_len += 1;
                        } else {
                            line_len = 0;
                            let _ = write!(tx, "err: overflow\r\n");
                        }
                    }
                }
                Err(NbError::WouldBlock) => break,
                Err(_) => {
                    let _ = write!(tx, "err\r\n");
                }
            }
        }

        busy_delay_ms(cpu_hz, 1);
        half_sec_ticks = half_sec_ticks.wrapping_add(1);
        rx_idle_ticks = rx_idle_ticks.wrapping_add(1);
        if half_sec_ticks % 500 == 0 {
            if led_on {
                led.set_low();
            } else {
                led.set_high();
            }
            led_on = !led_on;
        }
        if half_sec_ticks % 1000 == 0 && rx_idle_ticks >= 1000 && seconds < 60 {
            seconds = seconds.wrapping_add(1);
            let _ = write!(tx, "alive t={}\r\n", seconds);
        }
    }
}
