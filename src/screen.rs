use embedded_graphics::{
    pixelcolor::{Rgb565, RgbColor},
    prelude::*,
};

use stm32f7xx_hal::{
    ltdc::{DisplayConfig, DisplayController, Layer, PixelFormat},
    pac::{DMA2D, LTDC},
    prelude::*,
    rcc::HSEClock,
};

pub const DISCO_SCREEN_CONFIG: DisplayConfig = DisplayConfig {
    active_width: 480,
    active_height: 272,
    h_back_porch: 13,
    h_front_porch: 30,
    h_sync: 41,
    v_back_porch: 2,
    v_front_porch: 2,
    v_sync: 10,
    frame_rate: 60,
    h_sync_pol: false,
    v_sync_pol: false,
    no_data_enable_pol: false,
    pixel_clock_pol: false,
};

pub struct Stm32F7DiscoDisplay {
    pub controller: DisplayController<u16>,
}

impl Stm32F7DiscoDisplay {
    pub fn new(ltdc: LTDC, dma2d: DMA2D, hse: &HSEClock) -> Self {
        let controller = DisplayController::new(
            ltdc,
            dma2d,
            PixelFormat::RGB565,
            DISCO_SCREEN_CONFIG,
            Some(hse),
        );

        Self { controller }
    }
}

impl DrawTarget for Stm32F7DiscoDisplay {
    type Color = Rgb565;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coord, color) in pixels {
            if coord.x < 0 || coord.y < 0 {
                continue;
            }
            let x = coord.x as usize;
            let y = coord.y as usize;
            if x >= DISCO_SCREEN_CONFIG.active_width as usize
                || y >= DISCO_SCREEN_CONFIG.active_height as usize
            {
                continue;
            }

            let value: u16 = ((color.r() as u16) & 0x1F) << 11
                | ((color.g() as u16) & 0x3F) << 5
                | (color.b() as u16 & 0x1F);
            self.controller.draw_pixel(Layer::L1, x, y, value);
        }

        Ok(())
    }
}

impl OriginDimensions for Stm32F7DiscoDisplay {
    fn size(&self) -> Size {
        Size::new(
            DISCO_SCREEN_CONFIG.active_width as u32,
            DISCO_SCREEN_CONFIG.active_height as u32,
        )
    }
}
