//! The key decoder and its reader: stdin in reads of at most `READ_CAP` bytes into a fixed buffer,
//! each read decoded to one closed key class; the bytes themselves go nowhere.

use std::io::{self, Read};
use std::sync::mpsc::{self, Receiver};
use std::thread::{self, JoinHandle};

/// The most one read takes.
pub(crate) const READ_CAP: usize = 64;

/// What one read of the terminal means to the scaffold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Key {
    Quit,
    Interrupt,
    Unbound,
}

impl Key {
    /// One read: a byte 0x03 anywhere is the interrupt, a lone `q` is quit, anything else unbound.
    pub(crate) fn of(read: &[u8]) -> Self {
        if read.contains(&0x03) {
            Self::Interrupt
        } else if read == b"q" {
            Self::Quit
        } else {
            Self::Unbound
        }
    }

    /// The `console-key` line's `key` word.
    pub(crate) const fn word(self) -> &'static str {
        match self {
            Self::Quit => "quit",
            Self::Interrupt => "interrupt",
            Self::Unbound => "unbound",
        }
    }
}

/// One message from the reader: a key, the end of input (`None`), or the read's error.
pub(crate) type KeyRead = io::Result<Option<Key>>;

/// Reads `input` on a thread of its own, so the loop can poll the size between keys. A panic in
/// the read ends the thread with nothing sent: the loop reads the channel closed and resumes it.
pub(crate) fn spawn<R: Read + Send + 'static>(mut input: R) -> (Receiver<KeyRead>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let mut buf = [0u8; READ_CAP];
        loop {
            let message = match input.read(&mut buf) {
                Ok(0) => Ok(None),
                Ok(n) => Ok(Some(Key::of(buf.get(..n).unwrap_or(&buf)))),
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

    #[rstest]
    #[case::q(b"q", Key::Quit)]
    #[case::ctrl_c(b"\x03", Key::Interrupt)]
    #[case::ctrl_c_after_a_key(b"x\x03", Key::Interrupt)]
    #[case::other_letter(b"x", Key::Unbound)]
    #[case::capital_q(b"Q", Key::Unbound)]
    #[case::q_among_others(b"qq", Key::Unbound)]
    #[case::an_arrow(b"\x1b[A", Key::Unbound)]
    fn key_of_reads_one_read_as_one_class(#[case] read: &[u8], #[case] key: Key) {
        assert_eq!(Key::of(read), key);
    }

    #[test]
    fn key_words_are_the_closed_three() {
        let words = [Key::Quit, Key::Interrupt, Key::Unbound].map(Key::word);
        assert_eq!(words, ["quit", "interrupt", "unbound"]);
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
    fn spawn_sends_one_key_per_read_then_the_end() {
        let (rx, handle) = spawn(Chunks(vec![b"x", b"q"]));
        let got: Vec<Option<Key>> = rx.iter().map(|m| m.expect("no error")).collect();
        handle.join().expect("reader");
        assert_eq!(got, [Some(Key::Unbound), Some(Key::Quit), None]);
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
