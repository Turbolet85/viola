//! Stdin as a terminal delivers it: bracketed pastes, keys and the submitting CR.

use super::*;

/// Stdin as a terminal would deliver it: a bracketed paste is prompt content, every other byte is
/// a keystroke, CR submits whatever the prompt holds, `\x03` ends the process.
pub(super) struct Input {
    esc: Vec<u8>,
    in_paste: bool,
    prompt: Vec<u8>,
}

pub(super) enum Action {
    Key(u8),
    Submit(Vec<u8>),
    Exit,
}

impl Input {
    pub(super) fn new() -> Self {
        Self {
            esc: Vec::new(),
            in_paste: false,
            prompt: Vec::new(),
        }
    }

    pub(super) fn feed(&mut self, byte: u8) -> Vec<Action> {
        if self.in_paste {
            self.prompt.push(byte);
            if self.prompt.ends_with(PASTE_END) {
                self.prompt.truncate(self.prompt.len() - PASTE_END.len());
                self.in_paste = false;
            }
            return Vec::new();
        }
        if byte == 0x1b || !self.esc.is_empty() {
            self.esc.push(byte);
            if self.esc == PASTE_START {
                self.esc.clear();
                self.in_paste = true;
                return Vec::new();
            }
            if PASTE_START.starts_with(&self.esc) {
                return Vec::new();
            }
            let mut held = std::mem::take(&mut self.esc);
            // A fresh ESC that broke the match may itself open the next paste.
            if held.last() == Some(&0x1b) {
                held.pop();
                self.esc.push(0x1b);
            }
            return held.into_iter().flat_map(|b| self.plain(b)).collect();
        }
        self.plain(byte)
    }

    fn plain(&mut self, byte: u8) -> Vec<Action> {
        match byte {
            0x03 => vec![Action::Exit],
            b'\r' => {
                let mut actions = vec![Action::Key(byte)];
                if !self.prompt.is_empty() {
                    actions.push(Action::Submit(std::mem::take(&mut self.prompt)));
                }
                actions
            }
            _ => {
                self.prompt.push(byte);
                vec![Action::Key(byte)]
            }
        }
    }
}
