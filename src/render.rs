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
        b'-' => [0, 0, 0, 0b111111, 0, 0, 0, 0, 0, 0],
        b'H' => [0b100001, 0b100001, 0b100001, 0b111111, 0b100001, 0b100001, 0b100001, 0, 0, 0],
        b'E' => [0b111111, 0b100000, 0b100000, 0b111110, 0b100000, 0b100000, 0b111111, 0, 0, 0],
        b'L' => [0b100000, 0b100000, 0b100000, 0b100000, 0b100000, 0b100000, 0b111111, 0, 0, 0],
        b'O' => [0b011110, 0b100001, 0b100001, 0b100001, 0b100001, 0b100001, 0b011110, 0, 0, 0],
        b'S' => [0b011111, 0b100000, 0b100000, 0b011110, 0b000001, 0b000001, 0b111110, 0, 0, 0],
        b'T' => [0b111111, 0b001100, 0b001100, 0b001100, 0b001100, 0b001100, 0b001100, 0, 0, 0],
        b'M' => [0b100001, 0b110011, 0b101101, 0b100001, 0b100001, 0b100001, 0b100001, 0, 0, 0],
        b'F' => [0b111111, 0b100000, 0b100000, 0b111110, 0b100000, 0b100000, 0b100000, 0, 0, 0],
        b'0' => [0b011110, 0b100001, 0b100011, 0b101001, 0b110001, 0b100001, 0b011110, 0, 0, 0],
        b'1' => [0b001100, 0b011100, 0b001100, 0b001100, 0b001100, 0b001100, 0b111111, 0, 0, 0],
        b'2' => [0b011110, 0b100001, 0b000001, 0b000110, 0b011000, 0b100000, 0b111111, 0, 0, 0],
        b'3' => [0b111110, 0b000001, 0b000001, 0b011110, 0b000001, 0b000001, 0b111110, 0, 0, 0],
        b'4' => [0b000110, 0b001010, 0b010010, 0b100010, 0b111111, 0b000010, 0b000010, 0, 0, 0],
        b'5' => [0b111111, 0b100000, 0b100000, 0b111110, 0b000001, 0b000001, 0b111110, 0, 0, 0],
        b'6' => [0b011110, 0b100000, 0b100000, 0b111110, 0b100001, 0b100001, 0b011110, 0, 0, 0],
        b'7' => [0b111111, 0b000001, 0b000010, 0b000100, 0b001000, 0b010000, 0b010000, 0, 0, 0],
        b'8' => [0b011110, 0b100001, 0b100001, 0b011110, 0b100001, 0b100001, 0b011110, 0, 0, 0],
        b'9' => [0b011110, 0b100001, 0b100001, 0b011111, 0b000001, 0b000001, 0b011110, 0, 0, 0],
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

pub fn draw_text_6x10_scaled(
    framebuffer: &mut [u16],
    x: i32,
    y: i32,
    s: &str,
    color: u16,
    scale: i32,
) {
    let mut cx = x;
    for &b in s.as_bytes() {
        let g = glyph_6x10(b);
        for (row, bits) in g.iter().enumerate() {
            for col in 0..6 {
                if (bits & (1 << (5 - col))) != 0 {
                    let px = cx + col as i32 * scale;
                    let py = y + row as i32 * scale;
                    for dy in 0..scale {
                        for dx in 0..scale {
                            put_px_rgb565(framebuffer, px + dx, py + dy, color);
                        }
                    }
                }
            }
        }
        cx += 7 * scale;
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

pub fn fill_solid(framebuffer: &mut [u16], color: u16) {
    for pixel in framebuffer.iter_mut() {
        *pixel = color;
    }
}

pub fn fill_rect(framebuffer: &mut [u16], x: i32, y: i32, w: i32, h: i32, color: u16) {
    for dy in 0..h {
        for dx in 0..w {
            put_px_rgb565(framebuffer, x + dx, y + dy, color);
        }
    }
}

pub fn draw_rect_outline(framebuffer: &mut [u16], x: i32, y: i32, w: i32, h: i32, color: u16) {
    for dx in 0..w {
        put_px_rgb565(framebuffer, x + dx, y, color);
        put_px_rgb565(framebuffer, x + dx, y + h - 1, color);
    }
    for dy in 0..h {
        put_px_rgb565(framebuffer, x, y + dy, color);
        put_px_rgb565(framebuffer, x + w - 1, y + dy, color);
    }
}

pub fn draw_triangle_up(framebuffer: &mut [u16], cx: i32, cy: i32, size: i32, color: u16) {
    for row in 0..size {
        let half = row;
        for dx in -half..=half {
            put_px_rgb565(framebuffer, cx + dx, cy - row, color);
        }
    }
}

pub fn draw_triangle_down(framebuffer: &mut [u16], cx: i32, cy: i32, size: i32, color: u16) {
    for row in 0..size {
        let half = row;
        for dx in -half..=half {
            put_px_rgb565(framebuffer, cx + dx, cy + row, color);
        }
    }
}

pub fn draw_text_16x24(framebuffer: &mut [u16], x: i32, y: i32, s: &str, color: u16) {
    let mut cx = x;
    for b in s.as_bytes() {
        if *b == b'-' {
            draw_minus_16x24(framebuffer, cx, y, color);
            cx += 18;
            continue;
        }
        if let Some(digit) = b.checked_sub(b'0') {
            if digit <= 9 {
                draw_digit_16x24(framebuffer, cx, y, digit, color);
                cx += 18;
                continue;
            }
        }
        cx += 18;
    }
}

fn draw_digit_16x24(framebuffer: &mut [u16], x: i32, y: i32, digit: u8, color: u16) {
    let segs = match digit {
        0 => 0b0111111,
        1 => 0b0000110,
        2 => 0b1011011,
        3 => 0b1001111,
        4 => 0b1100110,
        5 => 0b1101101,
        6 => 0b1111101,
        7 => 0b0000111,
        8 => 0b1111111,
        9 => 0b1101111,
        _ => 0,
    };

    if segs & 0b0000001 != 0 {
        seg_h(framebuffer, x, y + 1, color);
    }
    if segs & 0b0000010 != 0 {
        seg_v(framebuffer, x + 12, y + 3, color);
    }
    if segs & 0b0000100 != 0 {
        seg_v(framebuffer, x + 12, y + 13, color);
    }
    if segs & 0b0001000 != 0 {
        seg_h(framebuffer, x, y + 21, color);
    }
    if segs & 0b0010000 != 0 {
        seg_v(framebuffer, x + 1, y + 13, color);
    }
    if segs & 0b0100000 != 0 {
        seg_v(framebuffer, x + 1, y + 3, color);
    }
    if segs & 0b1000000 != 0 {
        seg_h(framebuffer, x, y + 11, color);
    }
}

fn draw_minus_16x24(framebuffer: &mut [u16], x: i32, y: i32, color: u16) {
    seg_h(framebuffer, x, y + 11, color);
}

fn seg_h(framebuffer: &mut [u16], x: i32, y: i32, color: u16) {
    for dy in 0..3 {
        for dx in 2..14 {
            put_px_rgb565(framebuffer, x + dx, y + dy, color);
        }
    }
}

fn seg_v(framebuffer: &mut [u16], x: i32, y: i32, color: u16) {
    for dy in 0..9 {
        for dx in 0..3 {
            put_px_rgb565(framebuffer, x + dx, y + dy, color);
        }
    }
}
