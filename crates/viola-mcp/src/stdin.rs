//! `viola mcp`'s stdin bound (security-plan §Input Validation, Constants): rmcp reads each line with
//! an unbounded `read_until`, so the bytes it is handed pass through this reader first. A line of at
//! most the cap, its `\n` counted, passes whole; a longer one passes no byte and the stream ends
//! there, as the wrapper channel ends a connection on an over-long frame.

use std::io;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};

use tokio::io::{AsyncRead, ReadBuf};

/// How much of the inner source one read takes.
const CHUNK: usize = 8 * 1024;

/// An async byte source cut into lines of at most `cap` bytes.
pub(crate) struct CappedLines<R> {
    inner: R,
    cap: u64,
    /// The current line, its `\n` not yet read; never more than `cap` bytes.
    held: Vec<u8>,
    /// Whole lines released and not yet read out, from `pos` on.
    ready: Vec<u8>,
    pos: usize,
    /// The inner source ended or a line was cut: nothing more is read from it.
    done: bool,
    cut: Arc<AtomicBool>,
}

impl<R> CappedLines<R> {
    pub(crate) fn new(inner: R, cap: u64) -> Self {
        Self {
            inner,
            cap,
            held: Vec::new(),
            ready: Vec::new(),
            pos: 0,
            done: false,
            cut: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Reads `true` once a line has been cut; it stays readable after the reader is moved away.
    pub(crate) fn cut_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.cut)
    }

    fn fits(&self, len: usize) -> bool {
        u64::try_from(len).is_ok_and(|len| len <= self.cap)
    }

    /// Sorts `bytes` into whole lines and the held one; a line past the cap ends the stream.
    fn take_in(&mut self, mut bytes: &[u8]) {
        while !bytes.is_empty() {
            let (segment, rest, ends_line) = match bytes.iter().position(|&b| b == b'\n') {
                Some(at) => (&bytes[..=at], &bytes[at + 1..], true),
                None => (bytes, &[][..], false),
            };
            if !self.fits(self.held.len() + segment.len()) {
                self.held.clear();
                self.done = true;
                self.cut.store(true, Ordering::Relaxed);
                return;
            }
            self.held.extend_from_slice(segment);
            if ends_line {
                self.ready.append(&mut self.held);
            }
            bytes = rest;
        }
    }
}

impl<R: AsyncRead + Unpin> AsyncRead for CappedLines<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let this = self.get_mut();
        loop {
            if this.pos < this.ready.len() {
                let n = buf.remaining().min(this.ready.len() - this.pos);
                buf.put_slice(&this.ready[this.pos..this.pos + n]);
                this.pos += n;
                if this.pos == this.ready.len() {
                    this.ready.clear();
                    this.pos = 0;
                }
                return Poll::Ready(Ok(()));
            }
            if this.done {
                return Poll::Ready(Ok(()));
            }
            let mut chunk = [0u8; CHUNK];
            let mut read = ReadBuf::new(&mut chunk);
            match Pin::new(&mut this.inner).poll_read(cx, &mut read) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
                Poll::Ready(Ok(())) if read.filled().is_empty() => {
                    this.ready.append(&mut this.held);
                    this.done = true;
                }
                Poll::Ready(Ok(())) => this.take_in(read.filled()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use rstest::rstest;
    use tokio::io::{AsyncBufReadExt as _, BufReader};

    use super::*;

    /// Gives its pieces one per read, each read first answered `Pending` once, then ends.
    struct Pieces {
        pieces: VecDeque<Vec<u8>>,
        at: usize,
        stalled: bool,
    }

    impl AsyncRead for Pieces {
        fn poll_read(
            mut self: Pin<&mut Self>,
            cx: &mut Context<'_>,
            buf: &mut ReadBuf<'_>,
        ) -> Poll<io::Result<()>> {
            if !self.stalled {
                self.stalled = true;
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
            self.stalled = false;
            let at = self.at;
            let Some(piece) = self.pieces.front() else {
                return Poll::Ready(Ok(()));
            };
            let n = buf.remaining().min(piece.len() - at);
            buf.put_slice(&piece[at..at + n]);
            if at + n == piece.len() {
                self.pieces.pop_front();
                self.at = 0;
            } else {
                self.at = at + n;
            }
            Poll::Ready(Ok(()))
        }
    }

    /// Every line rmcp's `read_until` takes from the reader, then whether it cut.
    fn lines_through(pieces: Vec<Vec<u8>>, cap: u64) -> (Vec<Vec<u8>>, bool) {
        let source = Pieces {
            pieces: pieces.into(),
            at: 0,
            stalled: false,
        };
        let capped = CappedLines::new(source, cap);
        let cut = capped.cut_flag();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("runtime");
        let lines = runtime.block_on(async move {
            let mut reader = BufReader::new(capped);
            let mut lines = Vec::new();
            loop {
                let mut line = Vec::new();
                let n = reader.read_until(b'\n', &mut line).await.expect("read");
                if n == 0 {
                    break lines;
                }
                lines.push(line);
            }
        });
        (lines, cut.load(Ordering::Relaxed))
    }

    fn line(len: usize) -> Vec<u8> {
        let mut line = vec![b'a'; len - 1];
        line.push(b'\n');
        line
    }

    #[rstest]
    #[case::one_under_the_cap(line(7), vec![line(7)], false)]
    #[case::exactly_the_cap(line(8), vec![line(8)], false)]
    #[case::one_past_the_cap(line(9), vec![], true)]
    #[case::past_the_cap_with_no_newline(vec![b'a'; 9], vec![], true)]
    #[case::unterminated_within_the_cap(vec![b'a'; 8], vec![vec![b'a'; 8]], false)]
    fn capped_lines_passes_a_line_within_the_cap_whole_and_cuts_a_longer_one(
        #[case] input: Vec<u8>,
        #[case] expected: Vec<Vec<u8>>,
        #[case] cut: bool,
    ) {
        assert_eq!(lines_through(vec![input], 8), (expected, cut));
    }

    #[test]
    fn capped_lines_yields_a_fitting_line_then_ends_at_an_over_long_one() {
        let input = [line(4), line(9), line(3)].concat();
        assert_eq!(lines_through(vec![input], 8), (vec![line(4)], true));
    }

    #[test]
    fn capped_lines_restarts_the_count_after_each_newline() {
        let input = [line(8), line(8), line(8)].concat();
        assert_eq!(
            lines_through(vec![input], 8),
            (vec![line(8), line(8), line(8)], false)
        );
    }

    #[test]
    fn capped_lines_cuts_an_over_long_line_split_across_reads() {
        let pieces = vec![line(2), b"aaa".to_vec(), b"aaa".to_vec(), b"aaa\n".to_vec()];
        assert_eq!(lines_through(pieces, 8), (vec![line(2)], true));
    }

    #[test]
    fn capped_lines_joins_a_fitting_line_split_across_reads() {
        let pieces = vec![b"aaa".to_vec(), b"aaaa".to_vec(), b"\n".to_vec()];
        assert_eq!(lines_through(pieces, 8), (vec![line(8)], false));
    }

    #[test]
    fn capped_lines_at_max_frame_passes_the_cap_and_cuts_one_byte_more() {
        assert_eq!(viola_core::MAX_FRAME, 16 * 1024 * 1024);
        let cap = 16 * 1024 * 1024;
        let (lines, cut) = lines_through(vec![line(cap)], viola_core::MAX_FRAME);
        assert!(!cut);
        assert_eq!(lines.iter().map(Vec::len).collect::<Vec<_>>(), [cap]);
        assert_eq!(
            lines_through(vec![line(cap + 1)], viola_core::MAX_FRAME),
            (vec![], true)
        );
    }
}
