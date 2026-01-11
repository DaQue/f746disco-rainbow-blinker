#![no_std]

pub const WIDTH: usize = 480;
pub const HEIGHT: usize = 272;
pub const FB_SIZE: usize = WIDTH * HEIGHT;
pub const WHITE_RGB565: u16 = 0xFFFF;
pub const YELLOW_RGB565: u16 = 0xFFE0;
pub const CYAN_RGB565: u16 = 0x07FF;
pub const GREEN_RGB565: u16 = 0x07E0;
pub const MAGENTA_RGB565: u16 = 0xF81F;
pub const RED_RGB565: u16 = 0xF800;
pub const BLUE_RGB565: u16 = 0x001F;
pub const BLACK_RGB565: u16 = 0x0000;
pub const ORANGE_RGB565: u16 = 0xFD20;
