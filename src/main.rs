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
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::{Rgb565, RgbColor},
    prelude::*,
    text::Text,
};
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
#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Clocks
    let rcc = dp.RCC.constrain();
    let hse = HSEClock::new(25_000_000.Hz(), HSEClockMode::Oscillator);
    let clocks = rcc
        .cfgr
        .hse(hse)
        .sysclk(216.MHz())
        .hclk(216.MHz())
        .freeze();

    let mut delay = cp.SYST.delay(&clocks);

    // IO
    let gpioe = dp.GPIOE.split();
    let gpiog = dp.GPIOG.split();
    let gpioh = dp.GPIOH.split();
    let gpioi = dp.GPIOI.split();
    let gpioj = dp.GPIOJ.split();
    let gpiok = dp.GPIOK.split();
    let mut led = gpioi.pi1.into_push_pull_output();
    for _ in 0..10 {
        led.set_high();
        busy_delay(216_000_000 / 8);
        led.set_low();
        busy_delay(216_000_000 / 8);
    }

    // LCD reset: PG6 -> LCD_RST
    let mut lcd_reset = gpiog.pg6.into_push_pull_output();
    lcd_reset.set_low();
    delay.delay_ms(20u16);
    lcd_reset.set_high();
    delay.delay_ms(20u16);

    // LTDC pins
    gpioe.pe4.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_B0
    gpiog.pg12.into_alternate::<9>().set_speed(Speed::VeryHigh); // LTCD_B4

    gpioi.pi9.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_VSYNC
    gpioi.pi10.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_HSYNC
    gpioi.pi13.into_alternate::<14>().set_speed(Speed::VeryHigh);
    gpioi.pi14.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_CLK
    gpioi.pi15.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R0

    gpioj.pj0.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R1
    gpioj.pj1.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R2
    gpioj.pj2.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R3
    gpioj.pj3.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R4
    gpioj.pj4.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R5
    gpioj.pj5.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R6
    gpioj.pj6.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_R7
    gpioj.pj7.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G0
    gpioj.pj8.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G1
    gpioj.pj9.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G2
    gpioj.pj10.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G3
    gpioj.pj11.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G4
    gpioj.pj13.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_B1
    gpioj.pj14.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_B2
    gpioj.pj15.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_B3

    gpiok.pk0.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G5
    gpiok.pk1.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G6
    gpiok.pk2.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_G7
    gpiok.pk4.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_B5
    gpiok.pk5.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_B6
    gpiok.pk6.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_B7
    gpiok.pk7.into_alternate::<14>().set_speed(Speed::VeryHigh); // LTCD_DE

    // HSE osc out in High Z
    gpioh.ph1.into_floating_input();

    // LCD enable: set it low first to avoid LCD bleed while setting up timings
    let mut disp_on = gpioi.pi12.into_push_pull_output();
    disp_on.set_low();

    // LCD backlight enable
    let mut backlight = gpiok.pk3.into_push_pull_output();
    backlight.set_high();

    let mut display = Stm32F7DiscoDisplay::new(dp.LTDC, dp.DMA2D, &hse);

    let framebuffer = unsafe { &mut *core::ptr::addr_of_mut!(FB_LAYER1) };
    let bars: [u16; 8] = [
        WHITE_RGB565,
        YELLOW_RGB565,
        CYAN_RGB565,
        GREEN_RGB565,
        MAGENTA_RGB565,
        RED_RGB565,
        BLUE_RGB565,
        BLACK_RGB565,
    ];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let bar = (x * bars.len()) / WIDTH;
            framebuffer[x + WIDTH * y] = bars[bar];
        }
    }

    display
        .controller
        .config_layer(Layer::L1, framebuffer, PixelFormat::RGB565);
    display.controller.enable_layer(Layer::L1);
    display.controller.reload();

    // LCD enable: activate LCD !
    disp_on.set_high();

    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE);
    Text::new("Hello from SRAM!", Point::new(20, 40), style)
        .draw(&mut display)
        .ok();

    let mut led_on = false;
    loop {
        if led_on {
            led.set_low();
        } else {
            led.set_high();
        }
        led_on = !led_on;
        busy_delay(216_000_000 / 4);
    }
}
