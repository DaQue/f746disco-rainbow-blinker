#![no_std]
#![no_main]

use panic_halt as _;

#[cfg(feature = "diag-led")]
use cortex_m_rt::entry;
#[cfg(feature = "diag-bkpt")]
use cortex_m_rt::entry;
#[cfg(feature = "diag-pins")]
use cortex_m_rt::entry;

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
use cortex_m::peripheral::DWT;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
use hal::{
    gpio::Speed,
    i2c::{BlockingI2c, Mode},
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
use crate::{app::AppState, serial_io::poll_serial, time::busy_delay_ms};
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
use crate::touch::Touch;

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
mod serial_io;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
mod app;
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
mod touch;
#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
mod demo_counter;
#[cfg(any(feature = "diag-led", feature = "diag-bkpt", feature = "diag-pins"))]
mod diag;

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
    diag::run_led()
}

#[cfg(feature = "diag-bkpt")]
#[entry]
fn main() -> ! {
    diag::run_bkpt()
}

#[cfg(feature = "diag-pins")]
#[entry]
fn main() -> ! {
    diag::run_pins()
}

#[cfg(all(
    not(feature = "diag-led"),
    not(feature = "diag-pins"),
    not(feature = "diag-bkpt")
))]
#[entry]
fn main() -> ! {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let hse = HSEClock::new(25_000_000.Hz(), HSEClockMode::Oscillator);
    let clocks = rcc.cfgr.hse(hse).sysclk(216.MHz()).hclk(216.MHz()).freeze();
    let mut apb1 = rcc.apb1;

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
    let sws = unsafe { &*pac::RCC::ptr() }.cfgr.read().sws().bits();
    let cpu_hz = match sws {
        0 => 16_000_000,
        1 => 25_000_000,
        2 => clocks.sysclk().raw(),
        _ => clocks.sysclk().raw(),
    };
    let _ = write!(tx, "clk sws={} hz={}\r\n", sws, cpu_hz);
    cp.DCB.enable_trace();
    DWT::unlock();
    cp.DWT.enable_cycle_counter();
    let cycles_per_ms = cpu_hz / 1000;
    let mut last_cycle = DWT::cycle_count();
    let mut cycle_remainder: u32 = 0;
    let _ = write!(tx, "boot ok\r\n");

    for _ in 0..10 {
        led.set_high();
        busy_delay_ms(cpu_hz, 125);
        led.set_low();
        busy_delay_ms(cpu_hz, 125);
    }

    let i2c_scl = gpioh.ph7.into_alternate::<4>().set_open_drain();
    let i2c_sda = gpioh.ph8.into_alternate::<4>().set_open_drain();
    let i2c = BlockingI2c::i2c3(
        dp.I2C3,
        (i2c_scl, i2c_sda),
        Mode::Standard {
            frequency: 100.kHz(),
        },
        &clocks,
        &mut apb1,
        1000,
    );
    let mut touch = Touch::new(i2c);

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
    let fb_ptr = core::ptr::addr_of_mut!(FB_LAYER1);

    let mut app = AppState::new(0);
    unsafe {
        app.render_initial(&mut *fb_ptr);
    }
    disp_on.set_high();
    display
        .controller
        .config_layer(Layer::L1, unsafe { &mut *fb_ptr }, PixelFormat::RGB565);
    display.controller.enable_layer(Layer::L1);
    display.controller.reload();
    display.controller.reload();
    let mut line_buf = [0u8; 64];
    let mut line_len: usize = 0;
    loop {
        let fb = unsafe { &mut *fb_ptr };
        poll_serial(&mut rx, &mut tx, &mut app, fb, &mut line_buf, &mut line_len);

        let now = DWT::cycle_count();
        let delta = now.wrapping_sub(last_cycle);
        last_cycle = now;
        let total = cycle_remainder.wrapping_add(delta);
        let elapsed_ms = total / cycles_per_ms;
        cycle_remainder = total % cycles_per_ms;
        busy_delay_ms(cpu_hz, 1);
        let fb = unsafe { &mut *fb_ptr };
        app.update(touch.read_touch(), elapsed_ms, &mut tx, fb);
        app.tick(elapsed_ms, &mut led, &mut tx);
    }
}
