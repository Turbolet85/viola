//! The colour layer of human output (design-system §Tokens (platform-specific)): the SGR tokens at
//! each depth, the decision whether and how deep to colour, and the reader of the facts it decides
//! on. The facts choose only whether SGR bytes are written: they never reach a log line, an error
//! or a refusal, and change no escaping and no check (security-plan §Security Anti-Patterns).

use std::io::{self, IsTerminal as _};

/// How many colours the terminal takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Depth {
    Truecolor,
    Ansi256,
    Ansi16,
}

/// The four SGR tokens of one depth. No background and no blue token exists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Tokens {
    /// The `DIALOG` word only.
    pub(crate) attention: &'static str,
    /// A whole `stale` row.
    pub(crate) stale: &'static str,
    /// NAME column values.
    pub(crate) callsign: &'static str,
    /// After every styled span.
    pub(crate) reset: &'static str,
}

impl Depth {
    pub(crate) const fn tokens(self) -> Tokens {
        let attention = match self {
            Self::Truecolor => "\x1b[38;2;217;119;6m",
            Self::Ansi256 => "\x1b[38;5;172m",
            Self::Ansi16 => "\x1b[33m",
        };
        Tokens {
            attention,
            stale: "\x1b[2m",
            callsign: "\x1b[1m",
            reset: "\x1b[0m",
        }
    }
}

/// What the decision reads. `vt` is the Windows VT enable's outcome, `None` elsewhere and before
/// it is made.
#[derive(Clone, Debug, Default)]
pub(crate) struct Facts {
    pub(crate) json: bool,
    /// `viola run`, a passed `viola revive`, `viola hook` or `viola mcp`.
    pub(crate) passthrough: bool,
    /// `NO_COLOR` present, an empty value included.
    pub(crate) no_color: bool,
    pub(crate) term: Option<String>,
    pub(crate) colorterm: Option<String>,
    pub(crate) stdout_tty: bool,
    pub(crate) vt: Option<bool>,
}

/// The colour decision order, first match no colour, then the depth rule: truecolor for a
/// `COLORTERM` of `truecolor` or `24bit`, 256 colours for a `TERM` holding `256color` or a VT enable
/// made, else the 16-colour fallback.
pub(crate) fn decide(facts: &Facts) -> Option<Depth> {
    if facts.json
        || facts.passthrough
        || facts.no_color
        || facts.term.as_deref() == Some("dumb")
        || !facts.stdout_tty
        || facts.vt == Some(false)
    {
        return None;
    }
    if matches!(facts.colorterm.as_deref(), Some("truecolor" | "24bit")) {
        Some(Depth::Truecolor)
    } else if facts
        .term
        .as_deref()
        .is_some_and(|term| term.contains("256color"))
        || facts.vt == Some(true)
    {
        Some(Depth::Ansi256)
    } else {
        Some(Depth::Ansi16)
    }
}

/// ENABLE_VIRTUAL_TERMINAL_PROCESSING alone.
#[cfg(any(windows, test))]
const VT_PROCESSING: u32 = 0x0004;

/// The console output mode with VT processing on, and nothing else added.
#[cfg(any(windows, test))]
const fn vt_mode(mode: u32) -> u32 {
    mode | VT_PROCESSING
}

/// The decided depth, and on Windows the VT enable it rests on, undone when this is dropped.
pub(crate) struct Colour {
    pub(crate) depth: Option<Depth>,
    #[cfg(windows)]
    _vt: Option<vt::Restore>,
}

/// Reads `NO_COLOR`, `TERM` and `COLORTERM` (the only variables this reads) and whether stdout is a
/// terminal, then, on Windows and only when nothing earlier switched colour off, makes the VT
/// enable.
pub(crate) fn read(json: bool, passthrough: bool) -> Colour {
    let text = |name: &str| std::env::var_os(name).map(|v| v.to_string_lossy().into_owned());
    let facts = Facts {
        json,
        passthrough,
        no_color: std::env::var_os("NO_COLOR").is_some(),
        term: text("TERM"),
        colorterm: text("COLORTERM"),
        stdout_tty: io::stdout().is_terminal(),
        vt: None,
    };
    #[cfg(windows)]
    let colour = {
        let restore = decide(&facts).and_then(|_| vt::enable());
        let facts = Facts {
            vt: Some(restore.is_some()),
            ..facts
        };
        Colour {
            depth: decide(&facts),
            _vt: restore,
        }
    };
    #[cfg(not(windows))]
    let colour = Colour {
        depth: decide(&facts),
    };
    colour
}

/// The Windows VT enable on stdout's console, its saved mode restored on drop.
#[cfg(windows)]
mod vt {
    use windows_sys::Win32::Foundation::HANDLE;
    use windows_sys::Win32::System::Console::{
        GetConsoleMode, GetStdHandle, STD_OUTPUT_HANDLE, SetConsoleMode,
    };

    pub(super) struct Restore {
        handle: HANDLE,
        mode: u32,
    }

    impl Drop for Restore {
        fn drop(&mut self) {
            // SAFETY: the handle is stdout's console, read when the mode was saved.
            let _ = unsafe { SetConsoleMode(self.handle, self.mode) };
        }
    }

    /// `None` when the mode cannot be read or set: no colour, and no mode left changed.
    pub(super) fn enable() -> Option<Restore> {
        // SAFETY: plain console calls on stdout's handle, the mode read into a local.
        unsafe {
            let handle = GetStdHandle(STD_OUTPUT_HANDLE);
            let mut mode = 0u32;
            if GetConsoleMode(handle, &mut mode) == 0 {
                return None;
            }
            if SetConsoleMode(handle, super::vt_mode(mode)) == 0 {
                return None;
            }
            Some(Restore { handle, mode })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    /// A terminal that takes colour: every rung passes.
    fn colourful() -> Facts {
        Facts {
            stdout_tty: true,
            term: Some("xterm".to_owned()),
            ..Facts::default()
        }
    }

    /// Each rung alone switches colour off, every earlier rung passing.
    #[rstest]
    #[case::json(Facts { json: true, ..colourful() })]
    #[case::passthrough(Facts { passthrough: true, ..colourful() })]
    #[case::no_color(Facts { no_color: true, ..colourful() })]
    #[case::dumb(Facts { term: Some("dumb".to_owned()), ..colourful() })]
    #[case::not_a_terminal(Facts { stdout_tty: false, ..colourful() })]
    #[case::vt_enable_failed(Facts { vt: Some(false), ..colourful() })]
    fn colour_decision_each_rung_switches_colour_off(#[case] facts: Facts) {
        assert_eq!(decide(&facts), None);
    }

    #[test]
    fn colour_decision_a_terminal_with_every_rung_passing_is_coloured() {
        assert_eq!(decide(&colourful()), Some(Depth::Ansi16));
        let windows = Facts {
            vt: Some(true),
            ..colourful()
        };
        assert!(decide(&windows).is_some());
    }

    fn facts(term: Option<&str>, colorterm: Option<&str>, vt: Option<bool>) -> Facts {
        Facts {
            term: term.map(str::to_owned),
            colorterm: colorterm.map(str::to_owned),
            vt,
            ..colourful()
        }
    }

    #[rstest]
    #[case::truecolor(Some("xterm"), Some("truecolor"), None, Depth::Truecolor)]
    #[case::twenty_four_bit(Some("xterm"), Some("24bit"), None, Depth::Truecolor)]
    #[case::colorterm_beats_256(Some("xterm-256color"), Some("truecolor"), None, Depth::Truecolor)]
    #[case::colorterm_on_windows(None, Some("24bit"), Some(true), Depth::Truecolor)]
    #[case::term_256(Some("xterm-256color"), None, None, Depth::Ansi256)]
    #[case::term_256_inside(Some("screen-256color-bce"), None, None, Depth::Ansi256)]
    #[case::windows_vt(None, None, Some(true), Depth::Ansi256)]
    #[case::windows_vt_colorterm_yes(None, Some("yes"), Some(true), Depth::Ansi256)]
    #[case::xterm(Some("xterm"), None, None, Depth::Ansi16)]
    #[case::term_unset(None, None, None, Depth::Ansi16)]
    #[case::colorterm_yes(Some("xterm"), Some("yes"), None, Depth::Ansi16)]
    fn colour_decision_picks_the_depth(
        #[case] term: Option<&str>,
        #[case] colorterm: Option<&str>,
        #[case] vt: Option<bool>,
        #[case] depth: Depth,
    ) {
        assert_eq!(decide(&facts(term, colorterm, vt)), Some(depth));
    }

    #[rstest]
    #[case::truecolor(Depth::Truecolor, "\x1b[38;2;217;119;6m")]
    #[case::ansi_256(Depth::Ansi256, "\x1b[38;5;172m")]
    #[case::ansi_16(Depth::Ansi16, "\x1b[33m")]
    fn colour_tokens_are_the_design_bytes(#[case] depth: Depth, #[case] attention: &'static str) {
        let want = Tokens {
            attention,
            stale: "\x1b[2m",
            callsign: "\x1b[1m",
            reset: "\x1b[0m",
        };
        assert_eq!(depth.tokens(), want);
    }

    /// ENABLE_VIRTUAL_TERMINAL_PROCESSING alone, never viola-pty's 0x000c, which adds
    /// DISABLE_NEWLINE_AUTO_RETURN.
    #[rstest]
    #[case::from_zero(0x0000, 0x0004)]
    #[case::kept_bits(0x0003, 0x0007)]
    #[case::already_on(0x0007, 0x0007)]
    #[case::newline_bit_untouched(0x0001, 0x0005)]
    fn colour_vt_mode_adds_only_the_processing_bit(#[case] mode: u32, #[case] want: u32) {
        assert_eq!(vt_mode(mode), want);
    }
}
