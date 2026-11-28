//! The key decoder and its reader: stdin in reads of at most `READ_CAP` bytes into a fixed buffer,
//! handed whole to the loop, whose `Decoder` turns them into closed key classes; a sequence split
//! across reads decodes as one key. The bytes themselves go nowhere.

use std::io::{self, Read};
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

/// The most one read takes.
pub(crate) const READ_CAP: usize = 64;

/// How long a lone ESC waits for the rest of a sequence before it is the escape key.
pub(crate) const ESC_WAIT: Duration = Duration::from_millis(50);

/// The longest escape sequence read before it is given up as unbound.
const SEQUENCE_CAP: usize = 32;

const ESC: u8 = 0x1b;

/// One decoded key. A printable char is handed as itself: what it means (a verb letter, `q`,
/// `j` / `k`, `y` / `n`, a typed char) is the loop's, by its mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Key {
    Interrupt,
    Up,
    Down,
    Enter,
    Escape,
    Backspace,
    Char(char),
    Unbound,
}

/// The decoder the loop owns: the bytes of a sequence not yet complete, and when it began.
#[derive(Debug, Default)]
pub(crate) struct Decoder {
    pending: Vec<u8>,
    since: Option<Instant>,
}

impl Decoder {
    /// The keys `read` completes at `now`. A 0x03 anywhere in a read is the interrupt alone.
    pub(crate) fn feed(&mut self, read: &[u8], now: Instant) -> Vec<Key> {
        if read.contains(&0x03) {
            self.pending.clear();
            self.since = None;
            return vec![Key::Interrupt];
        }
        let mut keys = Vec::new();
        for &byte in read {
            if self.pending.is_empty() {
                self.since = Some(now);
            }
            self.pending.push(byte);
            self.settle(&mut keys);
        }
        if self.pending.is_empty() {
            self.since = None;
        }
        keys
    }

    /// When the bytes held must be read as they stand, if any are held.
    pub(crate) fn deadline(&self) -> Option<Instant> {
        self.since.map(|since| since + ESC_WAIT)
    }

    /// At or past the deadline, the bytes held as one key: a lone ESC the escape key, anything
    /// else unbound.
    pub(crate) fn flush(&mut self, now: Instant) -> Option<Key> {
        let due = self.deadline().is_some_and(|deadline| now >= deadline);
        if !due || self.pending.is_empty() {
            return None;
        }
        let key = if self.pending == [ESC] {
            Key::Escape
        } else {
            Key::Unbound
        };
        self.pending.clear();
        self.since = None;
        Some(key)
    }

    /// Decodes what is held, as far as it is complete.
    fn settle(&mut self, keys: &mut Vec<Key>) {
        loop {
            match decode(&self.pending) {
                Step::Need => return,
                Step::Key(key, used) => {
                    keys.push(key);
                    self.pending.drain(..used);
                    if self.pending.is_empty() {
                        return;
                    }
                }
            }
        }
    }
}

/// What the held bytes give: more needed, or a key and how many bytes it took.
enum Step {
    Need,
    Key(Key, usize),
}

fn decode(bytes: &[u8]) -> Step {
    let Some(&first) = bytes.first() else {
        return Step::Need;
    };
    match first {
        ESC => escape(bytes),
        b'\r' => Step::Key(Key::Enter, 1),
        0x7f | 0x08 => Step::Key(Key::Backspace, 1),
        0x20..=0x7e => Step::Key(Key::Char(char::from(first)), 1),
        0x80.. => scalar(bytes),
        _ => Step::Key(Key::Unbound, 1),
    }
}

/// A sequence led by ESC: CSI (`ESC [`) read whole to its final byte, SS3 (`ESC O`) one byte
/// after it; ESC then any other byte is the pair unbound, and ESC ESC a lone escape.
fn escape(bytes: &[u8]) -> Step {
    let Some(&second) = bytes.get(1) else {
        return Step::Need;
    };
    match second {
        b'[' => csi(bytes),
        b'O' => match bytes.get(2) {
            None => Step::Need,
            Some(b'A') => Step::Key(Key::Up, 3),
            Some(b'B') => Step::Key(Key::Down, 3),
            Some(_) => Step::Key(Key::Unbound, 3),
        },
        ESC => Step::Key(Key::Escape, 1),
        _ => Step::Key(Key::Unbound, 2),
    }
}

fn csi(bytes: &[u8]) -> Step {
    for (at, &byte) in bytes.iter().enumerate().skip(2) {
        match byte {
            0x20..=0x3f => {}
            0x40..=0x7e => {
                let key = match &bytes[..=at] {
                    b"\x1b[A" => Key::Up,
                    b"\x1b[B" => Key::Down,
                    _ => Key::Unbound,
                };
                return Step::Key(key, at + 1);
            }
            _ => return Step::Key(Key::Unbound, at + 1),
        }
    }
    if bytes.len() >= SEQUENCE_CAP {
        Step::Key(Key::Unbound, bytes.len())
    } else {
        Step::Need
    }
}

/// One whole UTF-8 scalar: a printable one is a char, a control one (C1) or a malformed lead or
/// continuation unbound.
fn scalar(bytes: &[u8]) -> Step {
    let len = match bytes[0] {
        0xc2..=0xdf => 2,
        0xe0..=0xef => 3,
        0xf0..=0xf4 => 4,
        _ => return Step::Key(Key::Unbound, 1),
    };
    for (at, &byte) in bytes.iter().enumerate().skip(1).take(len - 1) {
        if !(0x80..=0xbf).contains(&byte) {
            return Step::Key(Key::Unbound, at);
        }
    }
    if bytes.len() < len {
        return Step::Need;
    }
    match std::str::from_utf8(&bytes[..len])
        .ok()
        .and_then(|s| s.chars().next())
    {
        Some(c) if !c.is_control() => Step::Key(Key::Char(c), len),
        _ => Step::Key(Key::Unbound, len),
    }
}

/// One message from the reader: a read's bytes, the end of input (`None`), or the read's error.
pub(crate) type KeyRead = io::Result<Option<Vec<u8>>>;

/// Reads `input` on a thread of its own, so the loop can poll the size between keys. A panic in
/// the read ends the thread with nothing sent: the loop reads the channel closed and resumes it.
pub(crate) fn spawn<R: Read + Send + 'static>(mut input: R) -> (Receiver<KeyRead>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let mut buf = [0u8; READ_CAP];
        loop {
            let message = match input.read(&mut buf) {
                Ok(0) => Ok(None),
                Ok(n) => Ok(Some(buf.get(..n).unwrap_or(&buf).to_vec())),
                Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                Err(e) => Err(e),
            };
            let last = !matches!(message, Ok(Some(_)));
            if tx.send(message).is_err() || last {
                return;
            }
        }
    });
    (rx, handle)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn decoded(reads: &[&[u8]]) -> Vec<Key> {
        let mut decoder = Decoder::default();
        let now = Instant::now();
        let mut keys: Vec<Key> = reads.iter().flat_map(|r| decoder.feed(r, now)).collect();
        keys.extend(decoder.flush(now + ESC_WAIT));
        keys
    }

    #[rstest]
    #[case::q(vec![b"q".as_slice()], vec![Key::Char('q')])]
    #[case::ctrl_c(vec![b"\x03".as_slice()], vec![Key::Interrupt])]
    #[case::ctrl_c_after_a_key(vec![b"x\x03".as_slice()], vec![Key::Interrupt])]
    #[case::ctrl_c_inside_a_sequence(vec![b"\x1b[".as_slice(), b"\x03".as_slice()], vec![Key::Interrupt])]
    #[case::two_letters(vec![b"qq".as_slice()], vec![Key::Char('q'), Key::Char('q')])]
    #[case::csi_up(vec![b"\x1b[A".as_slice()], vec![Key::Up])]
    #[case::csi_down(vec![b"\x1b[B".as_slice()], vec![Key::Down])]
    #[case::ss3_up(vec![b"\x1bOA".as_slice()], vec![Key::Up])]
    #[case::ss3_down(vec![b"\x1bOB".as_slice()], vec![Key::Down])]
    #[case::up_split_across_reads(vec![b"\x1b".as_slice(), b"[".as_slice(), b"A".as_slice()], vec![Key::Up])]
    #[case::down_split_after_esc(vec![b"\x1b".as_slice(), b"OB".as_slice()], vec![Key::Down])]
    #[case::enter(vec![b"\r".as_slice()], vec![Key::Enter])]
    #[case::del(vec![b"\x7f".as_slice()], vec![Key::Backspace])]
    #[case::backspace(vec![b"\x08".as_slice()], vec![Key::Backspace])]
    #[case::lone_esc(vec![b"\x1b".as_slice()], vec![Key::Escape])]
    #[case::esc_esc(vec![b"\x1b\x1b".as_slice()], vec![Key::Escape, Key::Escape])]
    #[case::csi_read_whole(vec![b"\x1b[1;5C".as_slice()], vec![Key::Unbound])]
    #[case::csi_then_a_letter(vec![b"\x1b[15~j".as_slice()], vec![Key::Unbound, Key::Char('j')])]
    #[case::alt_letter(vec![b"\x1bx".as_slice()], vec![Key::Unbound])]
    #[case::other_c0(vec![b"\x01".as_slice()], vec![Key::Unbound])]
    #[case::utf8_scalar(vec!["é".as_bytes()], vec![Key::Char('é')])]
    #[case::utf8_split_across_reads(vec![b"\xe2\x82".as_slice(), b"\xac".as_slice()], vec![Key::Char('€')])]
    #[case::c1_scalar(vec![b"\xc2\x9b".as_slice()], vec![Key::Unbound])]
    #[case::bad_lead(vec![b"\xff".as_slice()], vec![Key::Unbound])]
    #[case::bad_continuation(vec![b"\xc3a".as_slice()], vec![Key::Unbound, Key::Char('a')])]
    #[case::cut_scalar_at_the_deadline(vec![b"\xe2\x82".as_slice()], vec![Key::Unbound])]
    fn decoder_reads_each_sequence_as_one_closed_key(
        #[case] reads: Vec<&'static [u8]>,
        #[case] keys: Vec<Key>,
    ) {
        assert_eq!(decoded(&reads), keys);
    }

    #[test]
    fn decoder_holds_a_lone_esc_until_its_deadline() {
        let mut decoder = Decoder::default();
        let now = Instant::now();
        assert!(decoder.feed(b"\x1b", now).is_empty());
        assert_eq!(decoder.deadline(), Some(now + ESC_WAIT));
        assert_eq!(
            decoder.flush(now + ESC_WAIT - Duration::from_millis(1)),
            None
        );
        assert_eq!(decoder.flush(now + ESC_WAIT), Some(Key::Escape));
        assert_eq!(decoder.deadline(), None);
        assert_eq!(decoder.flush(now + ESC_WAIT * 2), None);
    }

    #[test]
    fn decoder_holds_no_deadline_once_a_sequence_completes() {
        let mut decoder = Decoder::default();
        let now = Instant::now();
        assert_eq!(decoder.feed(b"\x1b[A", now), [Key::Up]);
        assert_eq!(decoder.deadline(), None);
    }

    #[test]
    fn decoder_gives_up_an_endless_csi_as_unbound() {
        let mut read = b"\x1b[".to_vec();
        read.extend([b'1'; SEQUENCE_CAP - 2]);
        read.push(b'j');
        assert_eq!(decoded(&[&read]), [Key::Unbound, Key::Char('j')]);
    }

    /// One chunk per read, then the end of input.
    struct Chunks(Vec<&'static [u8]>);

    impl Read for Chunks {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            assert!(buf.len() <= READ_CAP, "a read past the cap");
            if self.0.is_empty() {
                return Ok(0);
            }
            let chunk = self.0.remove(0);
            buf[..chunk.len()].copy_from_slice(chunk);
            Ok(chunk.len())
        }
    }

    #[test]
    fn spawn_sends_each_read_then_the_end() {
        let (rx, handle) = spawn(Chunks(vec![b"x", b"\x1b[A"]));
        let got: Vec<Option<Vec<u8>>> = rx.iter().map(|m| m.expect("no error")).collect();
        handle.join().expect("reader");
        assert_eq!(got, [Some(b"x".to_vec()), Some(b"\x1b[A".to_vec()), None]);
    }

    struct Failing;

    impl Read for Failing {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::from(io::ErrorKind::BrokenPipe))
        }
    }

    #[test]
    fn spawn_sends_the_read_error_and_stops() {
        let (rx, handle) = spawn(Failing);
        let got: Vec<KeyRead> = rx.iter().collect();
        handle.join().expect("reader");
        assert_eq!(got.len(), 1);
        assert!(got[0].is_err());
    }
}
