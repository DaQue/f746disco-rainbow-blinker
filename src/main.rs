#![no_std]
#![no_main]

use panic_halt as _;

#[cfg(feature = "diag-led")]
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
use crate::{
    demo_counter::{
        hit_test, render_calibration_screen, render_counter, Button, CAL_POINT_COUNT, CAL_POINTS,
    },
    serial_cmd::{handle_serial_command, SerialAction},
    time::busy_delay_ms,
};
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
    let framebuffer = unsafe { &mut *core::ptr::addr_of_mut!(FB_LAYER1) };

    let mut counter_value: i16 = 0;
    let mut last_drawn: i16 = counter_value;
    let mut cal_mode = false;
    let mut cal_index: usize = 0;
    let mut cal_touch_down = false;
    let mut last_touch: Option<(u16, u16)> = None;
    let mut cal_points: [Option<(u16, u16)>; CAL_POINT_COUNT] = [None; CAL_POINT_COUNT];
    let cal_tol: i32 = 30;
    let mut btn_touch_down = false;
    let mut held_button: Option<Button> = None;
    let mut hold_ms: u32 = 0;
    let mut repeat_ms: u32 = 0;
    render_counter(framebuffer, counter_value);

    
disp_on.set_high();

display.controller.config_layer(Layer::L1, framebuffer, PixelFormat::RGB565);
display.controller.enable_layer(Layer::L1);
display.controller.reload();

display.controller.reload();
 
    let mut led_on = false;
    let mut led_ms: u32 = 0;
    let mut alive_ms: u32 = 0;
    let mut seconds: u32 = 0;
    let mut line_buf = [0u8; 64];
    let mut line_len: usize = 0;
    let mut rx_idle_ms: u32 = 0;
    loop {
        loop {
            match rx.read() {
                Ok(byte) => {
                    rx_idle_ms = 0;
                    if byte == b'\r' || byte == b'\n' {
                        if line_len > 0 {
                            if let Ok(line) = core::str::from_utf8(&line_buf[..line_len]) {
                                if let Some(action) =
                                    handle_serial_command(line, &mut counter_value, &mut tx)
                                {
                                    match action {
                                        SerialAction::ToggleCal => {
                                            cal_mode = !cal_mode;
                                            let _ = write!(
                                                tx,
                                                "cal {}\r\n",
                                                if cal_mode { "on" } else { "off" }
                                            );
                                            let fb =
                                                unsafe { &mut *core::ptr::addr_of_mut!(FB_LAYER1) };
                                            if cal_mode {
                                                cal_index = 0;
                                                cal_touch_down = false;
                                                last_touch = None;
                                                cal_points = [None; CAL_POINT_COUNT];
                                                btn_touch_down = false;
                                                render_calibration_screen(fb, cal_index, None);
                                            } else {
                                                btn_touch_down = false;
                                                render_counter(fb, counter_value);
                                            }
                                            last_drawn = counter_value;
                                        }
                                    }
                                }
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

        let now = DWT::cycle_count();
        let delta = now.wrapping_sub(last_cycle);
        last_cycle = now;
        let total = cycle_remainder.wrapping_add(delta);
        let elapsed_ms = total / cycles_per_ms;
        cycle_remainder = total % cycles_per_ms;
        busy_delay_ms(cpu_hz, 1);
        if !cal_mode {
            let touch_now = touch.read_touch();
            let is_down = touch_now.is_some();
            let just_pressed = is_down && !btn_touch_down;
            let current_button = touch_now.and_then(|(x, y)| hit_test(x as i32, y as i32));
            if current_button != held_button {
                held_button = current_button;
                hold_ms = 0;
                repeat_ms = 0;
            }
            if just_pressed {
                if let Some(button) = current_button {
                    match button {
                        Button::Up => {
                            if counter_value < 999 {
                                counter_value += 1;
                            }
                        }
                        Button::Down => {
                            if counter_value > -999 {
                                counter_value -= 1;
                            }
                        }
                    }
                }
            }
            if is_down {
                hold_ms = hold_ms.wrapping_add(elapsed_ms);
                if hold_ms >= 1333 {
                    repeat_ms = repeat_ms.wrapping_add(elapsed_ms);
                    if repeat_ms >= 250 {
                        if let Some(button) = held_button {
                            match button {
                                Button::Up => {
                                    if counter_value < 999 {
                                        counter_value += 1;
                                    }
                                }
                                Button::Down => {
                                    if counter_value > -999 {
                                        counter_value -= 1;
                                    }
                                }
                            }
                        }
                        repeat_ms = 0;
                    }
                }
            } else {
                held_button = None;
                hold_ms = 0;
                repeat_ms = 0;
            }
            btn_touch_down = is_down;
            if counter_value != last_drawn {
                let fb = unsafe { &mut *core::ptr::addr_of_mut!(FB_LAYER1) };
                render_counter(fb, counter_value);
                last_drawn = counter_value;
            }
        }
        if cal_mode {
            let touch_now = touch.read_touch();
            let is_down = touch_now.is_some();
            let was_down = cal_touch_down;
            let mut redraw = false;
            if !is_down && was_down {
                if let Some(pt) = last_touch {
                    let target = CAL_POINTS[cal_index];
                    let dx = pt.0 as i32 - target.0;
                    let dy = pt.1 as i32 - target.1;
                    if dx.abs() <= cal_tol && dy.abs() <= cal_tol {
                        cal_points[cal_index] = Some(pt);
                        let _ = write!(
                            tx,
                            "cal {} {} {}\r\n",
                            cal_index + 1,
                            pt.0,
                            pt.1
                        );
                        cal_index += 1;
                    } else {
                        let _ = write!(
                            tx,
                            "cal miss {} {} {} {} {}\r\n",
                            cal_index + 1,
                            pt.0,
                            pt.1,
                            target.0,
                            target.1
                        );
                    }
                }
                if cal_index >= CAL_POINT_COUNT {
                    cal_mode = false;
                    let _ = write!(tx, "cal done\r\n");
                    for (i, entry) in cal_points.iter().enumerate() {
                        if let Some((x, y)) = entry {
                            let _ = write!(tx, "cal {} {} {}\r\n", i + 1, x, y);
                        }
                    }
                    let fb = unsafe { &mut *core::ptr::addr_of_mut!(FB_LAYER1) };
                    render_counter(fb, counter_value);
                    last_drawn = counter_value;
                    last_touch = None;
                    cal_touch_down = false;
                    continue;
                }
                redraw = true;
            }
            if touch_now != last_touch {
                match touch_now {
                    Some((x, y)) => {
                        let _ = write!(tx, "touch {} {}\r\n", x, y);
                    }
                    None => {
                        let _ = write!(tx, "touch up\r\n");
                    }
                }
                redraw = true;
            }
            if redraw {
                let fb = unsafe { &mut *core::ptr::addr_of_mut!(FB_LAYER1) };
                render_calibration_screen(fb, cal_index, touch_now);
            }
            cal_touch_down = is_down;
            last_touch = touch_now;
        }
        led_ms = led_ms.wrapping_add(elapsed_ms);
        rx_idle_ms = rx_idle_ms.wrapping_add(elapsed_ms);
        alive_ms = alive_ms.wrapping_add(elapsed_ms);
        if led_ms >= 500 {
            led_ms = led_ms.wrapping_sub(500);
            if led_on {
                led.set_low();
            } else {
                led.set_high();
            }
            led_on = !led_on;
        }
        if !cal_mode && alive_ms >= 1000 && rx_idle_ms >= 3000 {
            alive_ms = alive_ms.wrapping_sub(1000);
            seconds = seconds.wrapping_add(1);
            let _ = write!(tx, "alive t={}\r\n", seconds);
        }
    }
}
