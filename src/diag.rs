#[cfg(feature = "diag-led")]
use stm32f7xx_hal as hal;
#[cfg(feature = "diag-led")]
use hal::{pac, prelude::*};
#[cfg(feature = "diag-led")]
use cortex_m::asm::delay as busy_delay;

#[cfg(feature = "diag-bkpt")]
use stm32f7xx_hal as hal;
#[cfg(feature = "diag-bkpt")]
use hal::{pac, prelude::*};
#[cfg(feature = "diag-bkpt")]
use cortex_m::asm::{bkpt, delay as busy_delay};

#[cfg(feature = "diag-pins")]
use stm32f7xx_hal as hal;
#[cfg(feature = "diag-pins")]
use hal::{pac, prelude::*};
#[cfg(feature = "diag-pins")]
use cortex_m::asm::delay as busy_delay;

#[cfg(feature = "diag-led")]
pub fn run_led() -> ! {
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
pub fn run_bkpt() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.sysclk(216.MHz()).freeze();

    loop {
        bkpt();
        busy_delay(216_000_000 / 4);
    }
}

#[cfg(feature = "diag-pins")]
pub fn run_pins() -> ! {
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
