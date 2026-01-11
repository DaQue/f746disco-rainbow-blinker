use core::fmt::Write;

use embedded_hal::serial::Read;
use nb::Error as NbError;

use crate::app::AppState;

pub fn poll_serial<RX>(
    rx: &mut RX,
    tx: &mut impl Write,
    app: &mut AppState,
    framebuffer: &mut [u16],
    line_buf: &mut [u8; 64],
    line_len: &mut usize,
) where
    RX: Read<u8>,
{
    loop {
        match rx.read() {
            Ok(byte) => {
                app.on_rx_activity();
                if byte == b'\r' || byte == b'\n' {
                    if *line_len > 0 {
                        if let Ok(line) = core::str::from_utf8(&line_buf[..*line_len]) {
                            app.handle_serial_line(line, tx, framebuffer);
                        } else {
                            let _ = write!(tx, "err\r\n");
                        }
                        *line_len = 0;
                    }
                } else {
                    let _ = write!(tx, "{}", byte as char);
                    if *line_len < line_buf.len() {
                        line_buf[*line_len] = byte;
                        *line_len += 1;
                    } else {
                        *line_len = 0;
                        let _ = write!(tx, "err: overflow\r\n");
                    }
                }
            }
            Err(NbError::WouldBlock) => break,
            Err(_) => {
                let _ = write!(tx, "err\r\n");
            }
        }
    }
}
