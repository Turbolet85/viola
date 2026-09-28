//! `viola::human`: the root bin's human-facing text (design-system §Streams). Refusals and their
//! `hint:` line go to stderr, the hint last, as does the `cli` catch site's `error:` line; result
//! lines go to stdout. Later human verbs (send, wait/last, list) add their own writers here when
//! they land.

use std::io::{self, Write};

/// A refusal's `unable:` / `hint:` pair as ONE write, so another writer's bytes on a shared stream
/// cannot land between the two lines.
pub(crate) fn write_refusal(out: &mut impl Write, unable: &str, hint: &str) -> io::Result<()> {
    out.write_all(format!("unable: {unable}\nhint: {hint}\n").as_bytes())
}

/// A refusal's two fixed lines on stderr (design-system cli pattern 2): no path, no pid. A failed
/// write (a closed pipe) is dropped, never a panic (obs-plan §10).
pub(crate) fn refuse(unable: &str, hint: &str) {
    let _ = write_refusal(&mut io::stderr().lock(), unable, hint);
}

/// The `cli` catch site's one line (obs-plan §7; design-system cli Exit-code phraseology): fixed,
/// uncoloured, no hint, no path.
pub(crate) fn write_internal_error(out: &mut impl Write) -> io::Result<()> {
    out.write_all(b"error: internal error\n")
}

/// `error: internal error` on stderr; a failed write is dropped.
pub(crate) fn internal_error() {
    let _ = write_internal_error(&mut io::stderr().lock());
}

/// One result line on stdout, as one write.
pub(crate) fn write_result(out: &mut impl Write, line: &str) -> io::Result<()> {
    out.write_all(format!("{line}\n").as_bytes())
}

/// A result line on stdout (design-system §Streams: stdout = results); a failed write is dropped.
pub(crate) fn result(line: &str) {
    let _ = write_result(&mut io::stdout().lock(), line);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    /// Counts every `write` / `write_all` call and keeps the bytes.
    #[derive(Default)]
    struct Recorder {
        calls: usize,
        bytes: Vec<u8>,
    }

    impl Write for Recorder {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.calls += 1;
            self.bytes.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
            self.calls += 1;
            self.bytes.extend_from_slice(buf);
            Ok(())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    struct Broken;

    impl Write for Broken {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::from(io::ErrorKind::BrokenPipe))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn write_refusal_writes_the_pair_in_one_write() {
        let mut out = Recorder::default();
        write_refusal(&mut out, "builder is already live", "viola list").expect("write");
        assert_eq!(out.calls, 1);
        assert_eq!(
            out.bytes,
            b"unable: builder is already live\nhint: viola list\n"
        );
    }

    #[rstest]
    #[case::live("builder is already live", "viola list")]
    #[case::tampered(
        "the pinned viola copy failed its integrity check",
        "the pinned copy was changed after it was written, so viola will not run it"
    )]
    fn write_refusal_puts_the_hint_last(#[case] unable: &str, #[case] hint: &str) {
        let mut out = Recorder::default();
        write_refusal(&mut out, unable, hint).expect("write");
        let text = String::from_utf8(out.bytes).expect("utf-8");
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(
            lines.first().copied(),
            Some(format!("unable: {unable}").as_str())
        );
        assert_eq!(
            lines.last().copied(),
            Some(format!("hint: {hint}").as_str())
        );
        assert!(text.ends_with(&format!("hint: {hint}\n")));
    }

    #[test]
    fn write_refusal_returns_the_writer_error() {
        let err = write_refusal(&mut Broken, "builder is already live", "viola list")
            .expect_err("a broken writer fails");
        assert_eq!(err.kind(), io::ErrorKind::BrokenPipe);
    }

    #[test]
    fn write_internal_error_is_the_fixed_line_in_one_write() {
        let mut out = Recorder::default();
        write_internal_error(&mut out).expect("write");
        assert_eq!(out.calls, 1);
        assert_eq!(out.bytes, b"error: internal error\n");
        let err = write_internal_error(&mut Broken).expect_err("a broken writer fails");
        assert_eq!(err.kind(), io::ErrorKind::BrokenPipe);
    }

    #[test]
    fn write_result_is_one_line_in_one_write() {
        let mut out = Recorder::default();
        write_result(&mut out, "stamped 2.1.0  6 pass  0 fail").expect("write");
        assert_eq!(out.calls, 1);
        assert_eq!(out.bytes, b"stamped 2.1.0  6 pass  0 fail\n");
        let err = write_result(&mut Broken, "x").expect_err("a broken writer fails");
        assert_eq!(err.kind(), io::ErrorKind::BrokenPipe);
    }
}
