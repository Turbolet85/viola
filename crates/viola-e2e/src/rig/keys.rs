//! The keys the rig may type into its own pty, and only those: the grant comes from argv, each key
//! is checked against it and against the frame it is typed on, and a refused key writes nothing.
//! Ctrl-C is the stop's alone (security-plan · decision/live-round-rig-keys-own-pty).

use std::io::{self, Write};

use super::screen::{Reading, TrustPlace};

/// The most down-arrows the trust step may press.
pub const MAX_DOWN: usize = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Down,
    Enter,
    X,
    Del,
}

impl Key {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Down => "down",
            Self::Enter => "enter",
            Self::X => "x",
            Self::Del => "del",
        }
    }

    const fn bytes(self) -> &'static [u8] {
        match self {
            Self::Down => b"\x1b[B",
            Self::Enter => b"\r",
            Self::X => b"x",
            Self::Del => b"\x7f",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Grant {
    pub trust: bool,
    pub input_box: bool,
    /// A resize of the rig's own pty: no key, so the closed key set stays as it is.
    pub resize: bool,
}

impl Grant {
    /// `trust`, `box`, `resize`, comma-separated; absent is no grant. Any other word is refused.
    pub fn parse(arg: Option<&str>) -> Option<Self> {
        let mut grant = Self::default();
        let Some(arg) = arg else {
            return Some(grant);
        };
        for word in arg.split(',') {
            match word {
                "trust" => grant.trust = true,
                "box" => grant.input_box = true,
                "resize" => grant.resize = true,
                _ => return None,
            }
        }
        Some(grant)
    }
}

/// Why a key was not typed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    NotGranted,
    NotReady,
    Write,
}

impl Refusal {
    pub const fn word(self) -> &'static str {
        match self {
            Self::NotGranted => "key-not-granted",
            Self::NotReady => "key-not-ready",
            Self::Write => "key-write-failed",
        }
    }
}

pub struct Keys {
    grant: Grant,
    typed: Vec<Key>,
}

impl Keys {
    pub fn new(grant: Grant) -> Self {
        Self {
            grant,
            typed: Vec::new(),
        }
    }

    /// Whether the grant holds a resize of the rig's own pty.
    pub const fn may_resize(&self) -> bool {
        self.grant.resize
    }

    /// Whether the grant holds the box keys, `x` and DEL.
    pub const fn may_type_box(&self) -> bool {
        self.grant.input_box
    }

    fn granted(&self, key: Key) -> bool {
        match key {
            Key::Down | Key::Enter => self.grant.trust,
            Key::X | Key::Del => self.grant.input_box,
        }
    }

    fn ready(&self, key: Key, on: &Reading) -> bool {
        match key {
            Key::Down => {
                on.trust_shown()
                    && self.typed.iter().filter(|&&k| k == Key::Down).count() < MAX_DOWN
            }
            Key::Enter => on.trust_shown() && on.selected == "trust-yes",
            Key::X | Key::Del => on.is_box_up(),
        }
    }

    /// Types `key` when the grant holds it and `on`, the frame read just before, admits it.
    pub fn type_key(&mut self, key: Key, on: &Reading, out: &mut dyn Write) -> Result<(), Refusal> {
        if !self.granted(key) {
            return Err(Refusal::NotGranted);
        }
        if !self.ready(key, on) {
            return Err(Refusal::NotReady);
        }
        write_all(out, key.bytes()).map_err(|_| Refusal::Write)?;
        self.typed.push(key);
        Ok(())
    }

    /// `none`, or each key by name with its count, in first-typed order.
    pub fn summary(&self) -> String {
        let mut counts: Vec<(Key, usize)> = Vec::new();
        for &key in &self.typed {
            match counts.iter_mut().find(|(k, _)| *k == key) {
                Some((_, n)) => *n += 1,
                None => counts.push((key, 1)),
            }
        }
        if counts.is_empty() {
            return "none".to_owned();
        }
        counts
            .iter()
            .map(|(k, n)| format!("{} {n}", k.name()))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// The trust-moves word: the down-arrows the dialog needs, or why no key may pass it. An up-arrow
/// and a third move stay outside the grant: they come back to the founder, never typed.
pub fn trust_moves(place: TrustPlace) -> Result<usize, &'static str> {
    match place {
        TrustPlace::Below(k) if k <= MAX_DOWN => Ok(k),
        TrustPlace::Below(_) => Err("needs-third-move"),
        TrustPlace::Above => Err("needs-up"),
        TrustPlace::Unread => Err("unread"),
    }
}

/// What the trust pass reads and writes through: the child's pty.
pub trait TrustPty {
    /// The frame now on screen.
    fn reading(&mut self) -> Reading;
    /// Waits for the frame `key` leads to.
    fn settle_after(&mut self, key: Key);
    fn out(&mut self) -> &mut dyn Write;
}

/// Passes the trust dialog: the moves read from the trust frame first, `k ≤ 2` down-arrows each
/// followed by a settle and a read, then Enter only on a read `trust-yes`. Any other place types
/// nothing at all.
pub fn pass_trust(
    keys: &mut Keys,
    place: TrustPlace,
    pty: &mut dyn TrustPty,
) -> Result<(), &'static str> {
    let moves = trust_moves(place).map_err(|_| "trust-unpassed")?;
    for _ in 0..moves {
        let on = pty.reading();
        keys.type_key(Key::Down, &on, pty.out())
            .map_err(Refusal::word)?;
        pty.settle_after(Key::Down);
    }
    let on = pty.reading();
    if on.selected != "trust-yes" {
        return Err("trust-unpassed");
    }
    keys.type_key(Key::Enter, &on, pty.out())
        .map_err(Refusal::word)?;
    pty.settle_after(Key::Enter);
    Ok(())
}

fn write_all(out: &mut dyn Write, bytes: &[u8]) -> io::Result<()> {
    out.write_all(bytes)?;
    out.flush()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trust(selected: &'static str) -> Reading {
        Reading {
            box_up: "down",
            trust: "shown",
            selected,
            options: 2,
            box_text: "absent",
            modal: "none",
        }
    }

    fn input_box() -> Reading {
        Reading {
            box_up: "up",
            trust: "absent",
            selected: "none",
            options: 0,
            box_text: "empty",
            modal: "none",
        }
    }

    #[test]
    fn the_grant_parses_its_two_words_and_refuses_any_other() {
        assert_eq!(Grant::parse(None), Some(Grant::default()));
        assert_eq!(
            Grant::parse(Some("trust,box")),
            Some(Grant {
                trust: true,
                input_box: true,
                resize: false
            })
        );
        assert_eq!(Grant::parse(Some("box")).map(|g| g.trust), Some(false));
        assert_eq!(Grant::parse(Some("trust,enter")), None);
        assert_eq!(Grant::parse(Some("")), None);
    }

    #[test]
    fn the_resize_word_grants_a_resize_and_no_key() {
        for (label, arg, want) in [
            ("box and resize", Some("box,resize"), Some((true, true))),
            ("resize alone", Some("resize"), Some((false, true))),
            ("box alone", Some("box"), Some((true, false))),
            ("an unknown word", Some("box,resize,paste"), None),
            ("absent", None, Some((false, false))),
        ] {
            let got = Grant::parse(arg).map(|g| {
                let keys = Keys::new(g);
                (keys.may_type_box(), keys.may_resize())
            });
            assert_eq!(got, want, "{label}");
        }
        let mut keys = Keys::new(Grant::parse(Some("resize")).expect("grant"));
        let mut out = Vec::new();
        assert_eq!(
            keys.type_key(Key::X, &input_box(), &mut out),
            Err(Refusal::NotGranted),
            "resize alone grants no x"
        );
        assert!(out.is_empty());
        assert_eq!(keys.summary(), "none");
    }

    #[test]
    fn a_key_outside_the_grant_writes_nothing() {
        let mut keys = Keys::new(Grant::default());
        let mut out = Vec::new();
        assert_eq!(
            keys.type_key(Key::Enter, &trust("trust-yes"), &mut out),
            Err(Refusal::NotGranted)
        );
        assert_eq!(
            keys.type_key(Key::X, &input_box(), &mut out),
            Err(Refusal::NotGranted)
        );
        assert!(out.is_empty());
        assert_eq!(keys.summary(), "none");
    }

    #[test]
    fn enter_types_only_on_the_trust_row_and_down_at_most_twice() {
        let mut keys = Keys::new(Grant::parse(Some("trust")).expect("grant"));
        let mut out = Vec::new();
        assert_eq!(
            keys.type_key(Key::Enter, &trust("other-row"), &mut out),
            Err(Refusal::NotReady)
        );
        for _ in 0..MAX_DOWN {
            keys.type_key(Key::Down, &trust("other-row"), &mut out)
                .expect("down");
        }
        assert_eq!(
            keys.type_key(Key::Down, &trust("other-row"), &mut out),
            Err(Refusal::NotReady)
        );
        keys.type_key(Key::Enter, &trust("trust-yes"), &mut out)
            .expect("enter");
        assert_eq!(out, b"\x1b[B\x1b[B\r");
        assert_eq!(keys.summary(), "down 2, enter 1");
    }

    #[test]
    fn box_keys_type_only_while_the_box_is_up() {
        let mut keys = Keys::new(Grant::parse(Some("box")).expect("grant"));
        let mut out = Vec::new();
        assert_eq!(
            keys.type_key(Key::X, &trust("trust-yes"), &mut out),
            Err(Refusal::NotReady)
        );
        keys.type_key(Key::X, &input_box(), &mut out).expect("x");
        keys.type_key(Key::Del, &input_box(), &mut out)
            .expect("del");
        assert_eq!(out, b"x\x7f");
        assert_eq!(keys.summary(), "x 1, del 1");
    }

    /// A pty over fed frames: each settle moves to the next frame (the last one stays).
    struct Frames {
        frames: Vec<Vec<String>>,
        at: usize,
        out: Vec<u8>,
    }

    impl Frames {
        fn new(frames: &[&[&str]]) -> Self {
            Self {
                frames: frames
                    .iter()
                    .map(|f| f.iter().map(|r| (*r).to_owned()).collect())
                    .collect(),
                at: 0,
                out: Vec::new(),
            }
        }

        fn place(&self) -> TrustPlace {
            super::super::screen::trust_place(&self.frames[0])
        }
    }

    impl TrustPty for Frames {
        fn reading(&mut self) -> Reading {
            super::super::screen::read(&self.frames[self.at])
        }

        fn settle_after(&mut self, _key: Key) {
            self.at = (self.at + 1).min(self.frames.len() - 1);
        }

        fn out(&mut self) -> &mut dyn Write {
            &mut self.out
        }
    }

    fn pass(
        frames: &[&[&str]],
    ) -> (
        Result<usize, &'static str>,
        Result<(), &'static str>,
        Vec<u8>,
    ) {
        let mut pty = Frames::new(frames);
        let place = pty.place();
        let mut keys = Keys::new(Grant::parse(Some("trust")).expect("grant"));
        let passed = pass_trust(&mut keys, place, &mut pty);
        (trust_moves(place), passed, pty.out)
    }

    #[test]
    fn the_fake_trust_form_reads_zero_moves_and_types_enter_alone() {
        let (moves, passed, out) = pass(&[&[" ❯ 1. Yes, I trust this folder", "   2. No, exit"]]);
        assert_eq!((moves, passed), (Ok(0), Ok(())));
        assert_eq!(out, b"\r");
    }

    #[test]
    fn the_unnumbered_form_with_the_selector_on_no_reads_one_move_then_enter_on_trust_yes() {
        let (moves, passed, out) = pass(&[
            &[" ❯ No, exit", "   Yes, I trust this folder"],
            &["   No, exit", " ❯ Yes, I trust this folder"],
        ]);
        assert_eq!((moves, passed), (Ok(1), Ok(())));
        assert_eq!(out, b"\x1b[B\r");
    }

    #[test]
    fn a_down_that_lands_off_the_trust_row_types_no_enter() {
        let (moves, passed, out) = pass(&[
            &[" ❯ No, exit", "   Yes, I trust this folder"],
            &[" ❯ No, exit", "   Yes, I trust this folder"],
        ]);
        assert_eq!((moves, passed), (Ok(1), Err("trust-unpassed")));
        assert_eq!(out, b"\x1b[B");
    }

    #[test]
    fn a_trust_row_above_the_selector_reads_needs_up_and_types_nothing() {
        let (moves, passed, out) = pass(&[&["   Yes, I trust this folder", " ❯ No, exit"]]);
        assert_eq!((moves, passed), (Err("needs-up"), Err("trust-unpassed")));
        assert!(out.is_empty());
    }

    #[test]
    fn three_option_rows_down_read_needs_third_move_and_type_nothing() {
        let (moves, passed, out) = pass(&[&[
            " ❯ No, exit",
            "   Maybe later",
            "   Ask again",
            "   Yes, I trust this folder",
        ]]);
        assert_eq!(
            (moves, passed),
            (Err("needs-third-move"), Err("trust-unpassed"))
        );
        assert!(out.is_empty());
    }
}
