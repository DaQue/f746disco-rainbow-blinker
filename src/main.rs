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
use crate::{
    demo_counter::render_counter,
    serial_cmd::handle_serial_command,
    time::busy_delay_ms,
};

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
mod render;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
mod serial_cmd;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
mod time;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
mod demo_counter;

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
use common::FB_SIZE;

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

    let mut counter_value: i16 = 0;
    let mut last_drawn: i16 = i16::MIN;
    render_counter(framebuffer, counter_value);
    last_drawn = counter_value;

    
disp_on.set_high();

display.controller.config_layer(Layer::L1, framebuffer, PixelFormat::RGB565);
display.controller.enable_layer(Layer::L1);
display.controller.reload();

display.controller.reload();
 
    let mut led_on = false;
    let mut half_sec_ticks: u32 = 0;
    let mut seconds: u32 = 0;
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
        if counter_value != last_drawn {
            let fb = unsafe { &mut *core::ptr::addr_of_mut!(FB_LAYER1) };
            render_counter(fb, counter_value);
            last_drawn = counter_value;
        }
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
        if half_sec_ticks % 1000 == 0 && rx_idle_ticks >= 3000 {
            seconds = seconds.wrapping_add(1);
            let _ = write!(tx, "alive t={}\r\n", seconds);
        }
    }
}
