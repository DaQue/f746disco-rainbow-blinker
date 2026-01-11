use core::fmt::Write;

use embedded_hal::digital::v2::OutputPin;

use crate::demo_counter::{
    hit_test, render_calibration_screen, render_counter, Button, CAL_POINT_COUNT, CAL_POINTS,
};
use crate::serial_cmd::{handle_serial_command, SerialAction};

pub struct AppState {
    counter_value: i16,
    last_drawn: i16,
    cal_mode: bool,
    cal_index: usize,
    cal_touch_down: bool,
    last_touch: Option<(u16, u16)>,
    cal_points: [Option<(u16, u16)>; CAL_POINT_COUNT],
    cal_tol: i32,
    btn_touch_down: bool,
    held_button: Option<Button>,
    hold_ms: u32,
    repeat_ms: u32,
    last_pressed: Option<Button>,
    led_on: bool,
    led_ms: u32,
    alive_ms: u32,
    rx_idle_ms: u32,
    seconds: u32,
}

impl AppState {
    pub fn new(counter_value: i16) -> Self {
        Self {
            counter_value,
            last_drawn: counter_value,
            cal_mode: false,
            cal_index: 0,
            cal_touch_down: false,
            last_touch: None,
            cal_points: [None; CAL_POINT_COUNT],
            cal_tol: 30,
            btn_touch_down: false,
            held_button: None,
            hold_ms: 0,
            repeat_ms: 0,
            last_pressed: None,
            led_on: false,
            led_ms: 0,
            alive_ms: 0,
            rx_idle_ms: 0,
            seconds: 0,
        }
    }

    pub fn render_initial(&mut self, framebuffer: &mut [u16]) {
        render_counter(framebuffer, self.counter_value, None);
        self.last_drawn = self.counter_value;
    }

    pub fn on_rx_activity(&mut self) {
        self.rx_idle_ms = 0;
    }

    pub fn handle_serial_line(&mut self, line: &str, tx: &mut impl Write, fb: &mut [u16]) {
        if let Some(action) = handle_serial_command(line, &mut self.counter_value, tx) {
            match action {
                SerialAction::ToggleCal => {
                    self.cal_mode = !self.cal_mode;
                    let _ = write!(
                        tx,
                        "cal {}\r\n",
                        if self.cal_mode { "on" } else { "off" }
                    );
                    if self.cal_mode {
                        self.cal_index = 0;
                        self.cal_touch_down = false;
                        self.last_touch = None;
                        self.cal_points = [None; CAL_POINT_COUNT];
                        self.btn_touch_down = false;
                        self.last_pressed = None;
                        render_calibration_screen(fb, self.cal_index, None);
                    } else {
                        self.btn_touch_down = false;
                        self.last_pressed = None;
                        render_counter(fb, self.counter_value, None);
                    }
                    self.last_drawn = self.counter_value;
                }
            }
        }
    }

    pub fn update(
        &mut self,
        touch_now: Option<(u16, u16)>,
        elapsed_ms: u32,
        tx: &mut impl Write,
        fb: &mut [u16],
    ) {
        if self.cal_mode {
            self.update_cal(touch_now, tx, fb);
        } else {
            self.update_buttons(touch_now, elapsed_ms, fb);
        }
    }

    pub fn tick(&mut self, elapsed_ms: u32, led: &mut impl OutputPin, tx: &mut impl Write) {
        self.led_ms = self.led_ms.wrapping_add(elapsed_ms);
        self.rx_idle_ms = self.rx_idle_ms.wrapping_add(elapsed_ms);
        self.alive_ms = self.alive_ms.wrapping_add(elapsed_ms);
        if self.led_ms >= 500 {
            self.led_ms = self.led_ms.wrapping_sub(500);
            if self.led_on {
                let _ = led.set_low();
            } else {
                let _ = led.set_high();
            }
            self.led_on = !self.led_on;
        }
        if !self.cal_mode && self.alive_ms >= 1000 && self.rx_idle_ms >= 3000 {
            self.alive_ms = self.alive_ms.wrapping_sub(1000);
            self.seconds = self.seconds.wrapping_add(1);
            let _ = write!(tx, "alive t={}\r\n", self.seconds);
        }
    }

    fn update_buttons(&mut self, touch_now: Option<(u16, u16)>, elapsed_ms: u32, fb: &mut [u16]) {
        let is_down = touch_now.is_some();
        let just_pressed = is_down && !self.btn_touch_down;
        let current_button = touch_now.and_then(|(x, y)| hit_test(x as i32, y as i32));
        let pressed_now = if is_down { current_button } else { None };
        let mut redraw = pressed_now != self.last_pressed;
        if current_button != self.held_button {
            self.held_button = current_button;
            self.hold_ms = 0;
            self.repeat_ms = 0;
        }
        if just_pressed {
            if let Some(button) = current_button {
                match button {
                    Button::Up => {
                        if self.counter_value > -999 {
                            self.counter_value -= 1;
                        }
                    }
                    Button::Down => {
                        if self.counter_value < 999 {
                            self.counter_value += 1;
                        }
                    }
                }
            }
        }
        if is_down {
            self.hold_ms = self.hold_ms.wrapping_add(elapsed_ms);
            if self.hold_ms >= 1333 {
                self.repeat_ms = self.repeat_ms.wrapping_add(elapsed_ms);
                if self.repeat_ms >= 250 {
                    if let Some(button) = self.held_button {
                        match button {
                            Button::Up => {
                                if self.counter_value > -999 {
                                    self.counter_value -= 1;
                                }
                            }
                            Button::Down => {
                                if self.counter_value < 999 {
                                    self.counter_value += 1;
                                }
                            }
                        }
                    }
                    self.repeat_ms = 0;
                    redraw = true;
                }
            }
        } else {
            self.held_button = None;
            self.hold_ms = 0;
            self.repeat_ms = 0;
        }
        self.btn_touch_down = is_down;
        if self.counter_value != self.last_drawn || redraw {
            render_counter(fb, self.counter_value, pressed_now);
            self.last_drawn = self.counter_value;
        }
        self.last_pressed = pressed_now;
    }

    fn update_cal(&mut self, touch_now: Option<(u16, u16)>, tx: &mut impl Write, fb: &mut [u16]) {
        let is_down = touch_now.is_some();
        let was_down = self.cal_touch_down;
        let mut redraw = false;
        if !is_down && was_down {
            if let Some(pt) = self.last_touch {
                let target = CAL_POINTS[self.cal_index];
                let dx = pt.0 as i32 - target.0;
                let dy = pt.1 as i32 - target.1;
                if dx.abs() <= self.cal_tol && dy.abs() <= self.cal_tol {
                    self.cal_points[self.cal_index] = Some(pt);
                    let _ = write!(tx, "cal {} {} {}\r\n", self.cal_index + 1, pt.0, pt.1);
                    self.cal_index += 1;
                } else {
                    let _ = write!(
                        tx,
                        "cal miss {} {} {} {} {}\r\n",
                        self.cal_index + 1,
                        pt.0,
                        pt.1,
                        target.0,
                        target.1
                    );
                }
            }
            if self.cal_index >= CAL_POINT_COUNT {
                self.cal_mode = false;
                let _ = write!(tx, "cal done\r\n");
                for (i, entry) in self.cal_points.iter().enumerate() {
                    if let Some((x, y)) = entry {
                        let _ = write!(tx, "cal {} {} {}\r\n", i + 1, x, y);
                    }
                }
                render_counter(fb, self.counter_value, None);
                self.last_drawn = self.counter_value;
                self.last_pressed = None;
                self.last_touch = None;
                self.cal_touch_down = false;
                return;
            }
            redraw = true;
        }
        if touch_now != self.last_touch {
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
            render_calibration_screen(fb, self.cal_index, touch_now);
        }
        self.cal_touch_down = is_down;
        self.last_touch = touch_now;
    }
}
