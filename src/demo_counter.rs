use common::{BLACK_RGB565, GREEN_RGB565, HEIGHT, ORANGE_RGB565, WHITE_RGB565, WIDTH};

use crate::render::{
    draw_crosshair, draw_text_16x24, draw_text_6x10, draw_triangle_down, draw_triangle_up,
    fill_rainbow, fill_round_rect, fill_solid,
};

pub fn render_counter(framebuffer: &mut [u16], value: i16) {
    let mut buf = [0u8; 6];
    let s = format_i16(value, &mut buf);
    let len = s.len() as i32;
    let btn_width = 140;
    let btn_height = 60;
    let btn_gap = 20;
    let left_x = (WIDTH as i32 / 2) - btn_gap - btn_width;
    let right_x = (WIDTH as i32 / 2) + btn_gap;

    let text_width = len * 16 + (len - 1) * 2;
    let text_height = 24;

    let box_pad_y = 12;
    let box_w = right_x + btn_width - left_x;
    let box_h = text_height + box_pad_y * 2;
    let box_x = left_x;
    let box_y = (HEIGHT as i32 - box_h) / 2;
    let text_x = (WIDTH as i32 - text_width) / 2;
    let text_y = box_y + box_pad_y;
    let btn_y = box_y + box_h + 20;

    fill_rainbow(framebuffer);
    fill_round_rect(framebuffer, box_x, box_y, box_w, box_h, 12, BLACK_RGB565);
    draw_text_16x24(framebuffer, text_x, text_y, s, ORANGE_RGB565);

    let btn_radius = 10;
    let btn_frame = 2;
    fill_round_rect(
        framebuffer,
        left_x,
        btn_y,
        btn_width,
        btn_height,
        btn_radius,
        ORANGE_RGB565,
    );
    fill_round_rect(
        framebuffer,
        left_x + btn_frame,
        btn_y + btn_frame,
        btn_width - btn_frame * 2,
        btn_height - btn_frame * 2,
        (btn_radius - btn_frame).max(0),
        BLACK_RGB565,
    );
    fill_round_rect(
        framebuffer,
        right_x,
        btn_y,
        btn_width,
        btn_height,
        btn_radius,
        ORANGE_RGB565,
    );
    fill_round_rect(
        framebuffer,
        right_x + btn_frame,
        btn_y + btn_frame,
        btn_width - btn_frame * 2,
        btn_height - btn_frame * 2,
        (btn_radius - btn_frame).max(0),
        BLACK_RGB565,
    );

    let arrow_size = 12;
    let arrow_offset = (arrow_size - 1) / 2;
    let up_cx = left_x + btn_width / 2;
    let up_cy = btn_y + btn_height / 2 + arrow_offset;
    draw_triangle_up(framebuffer, up_cx, up_cy, arrow_size, ORANGE_RGB565);

    let down_cx = right_x + btn_width / 2;
    let down_cy = btn_y + btn_height / 2 - arrow_offset;
    draw_triangle_down(framebuffer, down_cx, down_cy, arrow_size, ORANGE_RGB565);
}

pub const CAL_POINT_COUNT: usize = 5;
pub const CAL_POINTS: [(i32, i32); CAL_POINT_COUNT] = [
    (20, 20),
    (WIDTH as i32 - 1 - 20, HEIGHT as i32 - 1 - 20),
    (20, HEIGHT as i32 - 1 - 20),
    (WIDTH as i32 - 1 - 20, 20),
    (WIDTH as i32 / 2, HEIGHT as i32 / 2),
];

pub fn render_calibration_screen(
    framebuffer: &mut [u16],
    point_index: usize,
    touch: Option<(u16, u16)>,
) {
    fill_solid(framebuffer, BLACK_RGB565);
    let size = 33;
    let idx = point_index % CAL_POINTS.len();
    let (tx, ty) = CAL_POINTS[idx];
    draw_crosshair(framebuffer, tx, ty, size, WHITE_RGB565);

    if let Some((x, y)) = touch {
        draw_crosshair(framebuffer, x as i32, y as i32, 23, GREEN_RGB565);
    }

    let mut label_buf = [0u8; 12];
    let label = format_label(idx + 1, CAL_POINTS.len(), &mut label_buf);
    draw_text_6x10(framebuffer, 10, 10, label, ORANGE_RGB565);
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

fn format_label(current: usize, total: usize, out: &mut [u8; 12]) -> &str {
    let mut idx = 0usize;
    out[idx] = b'C';
    idx += 1;
    out[idx] = b'A';
    idx += 1;
    out[idx] = b'L';
    idx += 1;
    out[idx] = b' ';
    idx += 1;
    idx += write_usize(current, &mut out[idx..]);
    out[idx] = b'/';
    idx += 1;
    idx += write_usize(total, &mut out[idx..]);
    core::str::from_utf8(&out[..idx]).unwrap_or("CAL")
}

fn write_usize(mut value: usize, out: &mut [u8]) -> usize {
    let mut buf = [0u8; 10];
    let mut len = 0usize;
    if value == 0 {
        buf[0] = 0;
        len = 1;
    } else {
        while value > 0 {
            buf[len] = (value % 10) as u8;
            len += 1;
            value /= 10;
        }
    }
    for i in (0..len).rev() {
        out[len - 1 - i] = b'0' + buf[i];
    }
    len
}
