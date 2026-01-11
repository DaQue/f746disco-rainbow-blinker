use embedded_hal::blocking::i2c::WriteRead;
use common::{HEIGHT, WIDTH};

const FT5336_ADDR: u8 = 0x38;
const REG_TD_STATUS: u8 = 0x02;
const SWAP_XY: bool = true;
const INVERT_X: bool = false;
const INVERT_Y: bool = false;

pub struct Touch<I2C> {
    i2c: I2C,
}

impl<I2C> Touch<I2C>
where
    I2C: WriteRead,
{
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn read_touch(&mut self) -> Option<(u16, u16)> {
        let mut buf = [0u8; 5];
        if self
            .i2c
            .write_read(FT5336_ADDR, &[REG_TD_STATUS], &mut buf)
            .is_err()
        {
            return None;
        }

        let touches = buf[0] & 0x0F;
        if touches == 0 {
            return None;
        }

        let mut x = (((buf[1] & 0x0F) as u16) << 8) | buf[2] as u16;
        let mut y = (((buf[3] & 0x0F) as u16) << 8) | buf[4] as u16;
        if x == 0x0FFF && y == 0x0FFF {
            return None;
        }

        if SWAP_XY {
            core::mem::swap(&mut x, &mut y);
        }
        let mut xi = x as i32;
        let mut yi = y as i32;
        if INVERT_X {
            xi = WIDTH as i32 - 1 - xi;
        }
        if INVERT_Y {
            yi = HEIGHT as i32 - 1 - yi;
        }
        xi = xi.clamp(0, WIDTH as i32 - 1);
        yi = yi.clamp(0, HEIGHT as i32 - 1);
        Some((xi as u16, yi as u16))
    }
}
