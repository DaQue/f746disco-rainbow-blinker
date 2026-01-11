pub fn handle_serial_command(line: &str, value: &mut i16, tx: &mut impl core::fmt::Write) {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return;
    }

    if trimmed.eq_ignore_ascii_case("get") {
        let _ = write!(tx, "{}\r\n", *value);
    } else if trimmed.eq_ignore_ascii_case("inc") {
        *value = clamp_counter(value.wrapping_add(1));
        let _ = write!(tx, "{}\r\n", *value);
    } else if trimmed.eq_ignore_ascii_case("dec") {
        *value = clamp_counter(value.wrapping_sub(1));
        let _ = write!(tx, "{}\r\n", *value);
    } else if trimmed.eq_ignore_ascii_case("help") {
        let _ = write!(
            tx,
            "get\r\nset N\r\ninc\r\ndec\r\nhelp\r\n"
        );
    } else if trimmed.len() >= 3
        && trimmed.as_bytes()[..3].eq_ignore_ascii_case(b"set")
    {
        let arg = trimmed.get(3..).unwrap_or("").trim();
        match arg.parse::<i16>() {
            Ok(parsed) => {
                *value = clamp_counter(parsed);
                let _ = write!(tx, "{}\r\n", *value);
            }
            Err(_) => {
                let _ = write!(tx, "err\r\n");
            }
        }
    } else {
        let _ = write!(tx, "err\r\n");
    }
}

fn clamp_counter(value: i16) -> i16 {
    if value > 999 {
        999
    } else if value < -999 {
        -999
    } else {
        value
    }
}
