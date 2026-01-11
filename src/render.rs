use common::{
    BLACK_RGB565, BLUE_RGB565, CYAN_RGB565, GREEN_RGB565, HEIGHT, MAGENTA_RGB565, RED_RGB565,
    WHITE_RGB565, WIDTH, YELLOW_RGB565,
};

pub fn put_px_rgb565(framebuffer: &mut [u16], x: i32, y: i32, color: u16) {
    if x < 0 || y < 0 {
        return;
    }
    let (x, y) = (x as usize, y as usize);
    if x >= WIDTH || y >= HEIGHT {
        return;
    }
    framebuffer[x + WIDTH * y] = color;
}

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

pub fn draw_text_6x10(framebuffer: &mut [u16], x: i32, y: i32, s: &str, color: u16) {
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

pub fn fill_rainbow(framebuffer: &mut [u16]) {
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
}
