use common::{BLACK_RGB565, HEIGHT, ORANGE_RGB565, WIDTH};

use crate::render::{
    draw_rect_outline, draw_text_16x24, draw_triangle_down, draw_triangle_up, fill_rainbow,
    fill_rect,
};

pub fn render_counter(framebuffer: &mut [u16], value: i16) {
    let mut buf = [0u8; 6];
    let s = format_i16(value, &mut buf);
    let len = s.len() as i32;
    let text_width = len * 16 + (len - 1) * 2;
    let text_height = 24;

    let bar_height = 80;
    let bar_y = (HEIGHT as i32 - bar_height) / 2;
    let text_x = (WIDTH as i32 - text_width) / 2;
    let text_y = bar_y + (bar_height - text_height) / 2;

    let btn_width = 140;
    let btn_height = 60;
    let btn_gap = 20;
    let btn_y = bar_y + bar_height + 20;
    let left_x = (WIDTH as i32 / 2) - btn_gap - btn_width;
    let right_x = (WIDTH as i32 / 2) + btn_gap;

    fill_rainbow(framebuffer);
    fill_rect(framebuffer, 0, bar_y, WIDTH as i32, bar_height, BLACK_RGB565);
    draw_text_16x24(framebuffer, text_x, text_y, s, ORANGE_RGB565);

    fill_rect(framebuffer, left_x, btn_y, btn_width, btn_height, BLACK_RGB565);
    fill_rect(framebuffer, right_x, btn_y, btn_width, btn_height, BLACK_RGB565);
    draw_rect_outline(framebuffer, left_x, btn_y, btn_width, btn_height, ORANGE_RGB565);
    draw_rect_outline(framebuffer, right_x, btn_y, btn_width, btn_height, ORANGE_RGB565);

    let up_cx = left_x + btn_width / 2;
    let up_cy = btn_y + btn_height / 2 - 4;
    draw_triangle_up(framebuffer, up_cx, up_cy, 12, ORANGE_RGB565);

    let down_cx = right_x + btn_width / 2;
    let down_cy = btn_y + btn_height / 2 + 4;
    draw_triangle_down(framebuffer, down_cx, down_cy, 12, ORANGE_RGB565);
}

fn format_i16(value: i16, out: &mut [u8; 6]) -> &str {
    let mut idx = 0usize;
    let mut val = value;
    if val < 0 {
        out[idx] = b'-';
        idx += 1;
        val = -val;
    }

    let mut digits = [0u8; 5];
    let mut dlen = 0usize;
    if val == 0 {
        digits[0] = 0;
        dlen = 1;
    } else {
        while val > 0 {
            digits[dlen] = (val % 10) as u8;
            dlen += 1;
            val /= 10;
        }
    }

    for i in (0..dlen).rev() {
        out[idx] = b'0' + digits[i];
        idx += 1;
    }

    core::str::from_utf8(&out[..idx]).unwrap_or("?")
}
