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

/// What the decision reads.
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
}

/// The colour decision order, first match no colour, then the depth rule: truecolor for a
/// `COLORTERM` of `truecolor` or `24bit`, 256 colours for a `TERM` holding `256color`, else the
/// 16-colour fallback.
pub(crate) fn decide(facts: &Facts) -> Option<Depth> {
    if facts.json
        || facts.passthrough
        || facts.no_color
        || facts.term.as_deref() == Some("dumb")
        || !facts.stdout_tty
    {
        return None;
    }
    if matches!(facts.colorterm.as_deref(), Some("truecolor" | "24bit")) {
        Some(Depth::Truecolor)
    } else if facts
        .term
        .as_deref()
        .is_some_and(|term| term.contains("256color"))
    {
        Some(Depth::Ansi256)
    } else {
        Some(Depth::Ansi16)
    }
}

/// The decided depth.
pub(crate) struct Colour {
    pub(crate) depth: Option<Depth>,
}

/// Reads `NO_COLOR`, `TERM` and `COLORTERM` (the only variables this reads) and whether stdout is a
/// terminal.
pub(crate) fn read(json: bool, passthrough: bool) -> Colour {
    let text = |name: &str| std::env::var_os(name).map(|v| v.to_string_lossy().into_owned());
    let facts = Facts {
        json,
        passthrough,
        no_color: std::env::var_os("NO_COLOR").is_some(),
        term: text("TERM"),
        colorterm: text("COLORTERM"),
        stdout_tty: io::stdout().is_terminal(),
    };
    Colour {
        depth: decide(&facts),
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
    fn colour_decision_each_rung_switches_colour_off(#[case] facts: Facts) {
        assert_eq!(decide(&facts), None);
    }

    #[test]
    fn colour_decision_a_terminal_with_every_rung_passing_is_coloured() {
        assert_eq!(decide(&colourful()), Some(Depth::Ansi16));
    }

    fn facts(term: Option<&str>, colorterm: Option<&str>) -> Facts {
        Facts {
            term: term.map(str::to_owned),
            colorterm: colorterm.map(str::to_owned),
            ..colourful()
        }
    }

    #[rstest]
    #[case::truecolor(Some("xterm"), Some("truecolor"), Depth::Truecolor)]
    #[case::twenty_four_bit(Some("xterm"), Some("24bit"), Depth::Truecolor)]
    #[case::colorterm_beats_256(Some("xterm-256color"), Some("truecolor"), Depth::Truecolor)]
    #[case::colorterm_without_term(None, Some("24bit"), Depth::Truecolor)]
    #[case::term_256(Some("xterm-256color"), None, Depth::Ansi256)]
    #[case::term_256_inside(Some("screen-256color-bce"), None, Depth::Ansi256)]
    #[case::xterm(Some("xterm"), None, Depth::Ansi16)]
    #[case::term_unset(None, None, Depth::Ansi16)]
    #[case::colorterm_yes(Some("xterm"), Some("yes"), Depth::Ansi16)]
    fn colour_decision_picks_the_depth(
        #[case] term: Option<&str>,
        #[case] colorterm: Option<&str>,
        #[case] depth: Depth,
    ) {
        assert_eq!(decide(&facts(term, colorterm)), Some(depth));
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
}
