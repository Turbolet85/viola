//! The two escapers: message mode and row mode.

/// Message mode (design-system cli Component Patterns 3; security-plan §Input Validation): every
/// control character but `\n` and `\t` — C0, DEL, C1 — shows as `\xHH` text. Nothing is stripped.
pub(crate) fn escape_message(text: &str) -> String {
    let mut shown = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_control() && c != '\n' && c != '\t' {
            shown.push_str(&format!("\\x{:02X}", u32::from(c)));
        } else {
            shown.push(c);
        }
    }
    shown
}

/// Row mode (design-system cli Component Patterns 3): every control character, `\n` and `\t`
/// included, DEL and C1 included, shows as `\xHH` text, so a field stays on its row. Nothing is
/// stripped.
pub(crate) fn escape_row(text: &str) -> String {
    let mut shown = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_control() {
            shown.push_str(&format!("\\x{:02X}", u32::from(c)));
        } else {
            shown.push(c);
        }
    }
    shown
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::esc("\u{1b}[31m", "\\x1B[31m")]
    #[case::cr("a\rb", "a\\x0Db")]
    #[case::del("\u{7f}", "\\x7F")]
    #[case::csi_c1("\u{9b}2J", "\\x9B2J")]
    #[case::bel_nul("\u{7}\u{0}", "\\x07\\x00")]
    #[case::c1_first_and_last("\u{80}\u{9f}", "\\x80\\x9F")]
    #[case::newline_and_tab_kept("one\n\ttwo", "one\n\ttwo")]
    #[case::plain_and_wide("41 passed, é 中 🙂", "41 passed, é 中 🙂")]
    #[case::nbsp_is_not_control("\u{a0}", "\u{a0}")]
    #[case::backslash_x_text_kept("\\x1B", "\\x1B")]
    fn escape_message_shows_controls_as_hex_text(#[case] text: &str, #[case] shown: &str) {
        assert_eq!(escape_message(text), shown);
    }

    #[rstest]
    #[case::esc("\u{1b}[31m", "\\x1B[31m")]
    #[case::c1("\u{9b}2J", "\\x9B2J")]
    #[case::del("\u{7f}", "\\x7F")]
    #[case::lf("2.1\n287", "2.1\\x0A287")]
    #[case::tab("2.1\t287", "2.1\\x09287")]
    #[case::cr_nul("\r\u{0}", "\\x0D\\x00")]
    #[case::plain("2.1.287", "2.1.287")]
    #[case::wide_and_nbsp("é 中\u{a0}", "é 中\u{a0}")]
    fn escape_row_shows_every_control_character_as_hex_text(
        #[case] text: &str,
        #[case] shown: &str,
    ) {
        assert_eq!(escape_row(text), shown);
    }
}
