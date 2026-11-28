//! The fleet console's own surface (`viola console`): the alternate screen, the frame and the
//! restore, plain ASCII with no colour. Each piece is one write.

use std::io::{self, Write};

/// The alternate screen entered, then the cursor hidden.
pub(crate) const CONSOLE_ENTER: &[u8] = b"\x1b[?1049h\x1b[?25l";

/// The cursor shown again, then the alternate screen left: the terminal as the console found it.
pub(crate) const CONSOLE_RESTORE: &[u8] = b"\x1b[?25h\x1b[?1049l";

/// The frame's first line.
pub(crate) const CONSOLE_TITLE: &str = "viola console";

/// The frame's last line.
pub(crate) const CONSOLE_KEYS: &str = "q quit";

pub(crate) fn write_console_enter(out: &mut impl Write) -> io::Result<()> {
    out.write_all(CONSOLE_ENTER)?;
    out.flush()
}

pub(crate) fn write_console_restore(out: &mut impl Write) -> io::Result<()> {
    out.write_all(CONSOLE_RESTORE)?;
    out.flush()
}

/// The whole frame from the top-left of a cleared screen: the title on the first row and the key
/// line on row `rows`.
pub(crate) fn write_console_frame(out: &mut impl Write, rows: u16) -> io::Result<()> {
    out.write_all(format!("\x1b[H\x1b[2J{CONSOLE_TITLE}\x1b[{rows};1H{CONSOLE_KEYS}").as_bytes())?;
    out.flush()
}

/// The one line a terminal below the floor shows, and nothing else.
pub(crate) fn write_console_too_small(
    out: &mut impl Write,
    floor: (u16, u16),
    size: (u16, u16),
) -> io::Result<()> {
    let ((floor_cols, floor_rows), (cols, rows)) = (floor, size);
    out.write_all(
        format!(
            "\x1b[H\x1b[2Jviola console needs {floor_cols}x{floor_rows}; this terminal is {cols}x{rows}"
        )
        .as_bytes(),
    )?;
    out.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn console_frame_holds_the_title_then_the_key_line_on_the_last_row() {
        let mut out = Vec::new();
        write_console_frame(&mut out, 24).expect("write");
        assert_eq!(out, b"\x1b[H\x1b[2Jviola console\x1b[24;1Hq quit");
    }

    #[test]
    fn console_too_small_is_one_line_naming_both_sizes() {
        let mut out = Vec::new();
        write_console_too_small(&mut out, (80, 24), (60, 20)).expect("write");
        assert_eq!(
            out,
            b"\x1b[H\x1b[2Jviola console needs 80x24; this terminal is 60x20"
        );
    }

    #[test]
    fn console_enter_and_restore_are_the_alternate_screen_pair() {
        let mut out = Vec::new();
        write_console_enter(&mut out).expect("enter");
        write_console_restore(&mut out).expect("restore");
        assert_eq!(out, b"\x1b[?1049h\x1b[?25l\x1b[?25h\x1b[?1049l");
    }
}
