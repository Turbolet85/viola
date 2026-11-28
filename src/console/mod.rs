//! The terminal fleet console's scaffold (`viola console`): it owns its terminal, draws one frame
//! that stays whole at the floor size and says so below it, reads its own keys and quits cleanly.
//! Every state change is one codes-only line (`console-frame`, `console-key`); no key's bytes reach
//! a line. It reads no board, sends no channel frame and calls no compositor.

mod frame;
mod keys;

use std::io::{self, Read, Write};
use std::mem::ManuallyDrop;
use std::sync::mpsc::RecvTimeoutError;
use std::time::Duration;

use viola_core::obs::ObsEvent;
use viola_core::obs_event;
use viola_pty::Size;

use crate::human;
use keys::Key;

/// How often the size is read between keys, as the pump polls it.
const POLL: Duration = Duration::from_millis(100);

/// The size drawn when none can be read: no frame fits it.
const NO_SIZE: Size = Size { cols: 0, rows: 0 };

/// How the loop ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum End {
    Quit,
    Interrupt,
    /// The input ended (the terminal hung up).
    Closed,
}

/// The terminal while the console holds it: the alternate screen and the hidden cursor, with the
/// terminal-mode handle `M` (the raw mode). Its drop writes the restore sequence and only then
/// drops `M`, on every exit path, an error or an unwinding panic included.
struct Screen<W: Write, M> {
    out: W,
    /// Dropped by this guard's body alone, after the restore sequence.
    mode: Option<ManuallyDrop<M>>,
}

impl<W: Write, M> Screen<W, M> {
    /// The guard stands before the first byte, so a failed enter still restores.
    fn enter(out: W, mode: M) -> io::Result<Self> {
        let mut screen = Self {
            out,
            mode: Some(ManuallyDrop::new(mode)),
        };
        human::write_console_enter(&mut screen.out)?;
        Ok(screen)
    }

    /// The whole frame at `size`, or the one floor line below it; then its `console-frame` line.
    fn draw(&mut self, size: Size) -> io::Result<()> {
        let fits = frame::fits(size);
        if fits {
            human::write_console_frame(&mut self.out, size.rows)?;
        } else {
            human::write_console_too_small(
                &mut self.out,
                (frame::FLOOR.cols, frame::FLOOR.rows),
                (size.cols, size.rows),
            )?;
        }
        obs_event!(
            INFO,
            ObsEvent::ConsoleFrame,
            cols = size.cols,
            rows = size.rows,
            fits = fits,
        );
        Ok(())
    }
}

impl<W: Write, M> Drop for Screen<W, M> {
    fn drop(&mut self) {
        let _ = human::write_console_restore(&mut self.out);
        if let Some(mode) = self.mode.take() {
            drop(ManuallyDrop::into_inner(mode));
        }
    }
}

/// The console's loop over `input` (keys), `out` (the terminal) and `mode` (its raw mode, held
/// until the restore), its size read through `size`. Returns once `q`, 0x03 or the end of input
/// is read, the terminal restored; a read error is returned and a panic in the read resumed, each
/// after the restore too.
pub(crate) fn run<R, W, M>(
    input: R,
    out: W,
    mode: M,
    mut size: impl FnMut() -> Option<Size>,
) -> io::Result<End>
where
    R: Read + Send + 'static,
    W: Write,
{
    let mut screen = Screen::enter(out, mode)?;
    let (keys, reader) = keys::spawn(input);
    let mut shown = size().unwrap_or(NO_SIZE);
    screen.draw(shown)?;
    loop {
        match keys.recv_timeout(POLL) {
            Ok(Ok(Some(key))) => {
                obs_event!(INFO, ObsEvent::ConsoleKey, key = key.word());
                match key {
                    Key::Quit => return Ok(End::Quit),
                    Key::Interrupt => return Ok(End::Interrupt),
                    Key::Unbound => {}
                }
            }
            Ok(Ok(None)) => return Ok(End::Closed),
            Ok(Err(error)) => return Err(error),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => {
                if let Err(panic) = reader.join() {
                    std::panic::resume_unwind(panic);
                }
                return Ok(End::Closed);
            }
        }
        if let Some(now) = size().filter(|now| *now != shown) {
            shown = now;
            screen.draw(shown)?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    const RESTORE: &[u8] = b"\x1b[?25h\x1b[?1049l";
    const ENTER: &[u8] = b"\x1b[?1049h\x1b[?25l";

    fn floor() -> Option<Size> {
        Some(Size::DEFAULT)
    }

    /// One chunk per read, then the end of input.
    struct Chunks(Vec<&'static [u8]>);

    impl Read for Chunks {
        fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if self.0.is_empty() {
                return Ok(0);
            }
            let chunk = self.0.remove(0);
            buf[..chunk.len()].copy_from_slice(chunk);
            Ok(chunk.len())
        }
    }

    struct FailingRead;

    impl Read for FailingRead {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            Err(io::Error::from(io::ErrorKind::BrokenPipe))
        }
    }

    struct PanickingRead;

    impl Read for PanickingRead {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            panic!("the read panicked");
        }
    }

    /// A terminal-mode double: records its own drop.
    struct Mode(Rc<Cell<bool>>);

    impl Drop for Mode {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }

    #[derive(Clone, Default)]
    struct Shared(Arc<Mutex<Vec<u8>>>);

    impl Write for Shared {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.lock().expect("out").extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn holds(haystack: &[u8], needle: &[u8]) -> bool {
        haystack.windows(needle.len()).any(|w| w == needle)
    }

    #[test]
    fn console_run_on_a_failing_read_writes_the_restore_before_it_returns_the_error() {
        let mut out = Vec::new();
        let dropped = Rc::new(Cell::new(false));
        let result = run(FailingRead, &mut out, Mode(Rc::clone(&dropped)), floor);
        assert_eq!(result.map_err(|e| e.kind()), Err(io::ErrorKind::BrokenPipe));
        assert!(out.starts_with(ENTER));
        assert!(out.ends_with(RESTORE), "the restore is the last bytes");
        assert!(dropped.get(), "the raw mode was let go");
    }

    #[test]
    fn console_run_panicking_in_the_loop_writes_the_restore_during_the_unwind() {
        let out = Shared::default();
        let writer = out.clone();
        let unwound = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            run(PanickingRead, writer, (), floor)
        }));
        assert!(unwound.is_err(), "the panic reached the caller");
        let bytes = out.0.lock().expect("out").clone();
        assert!(bytes.ends_with(RESTORE), "the restore is the last bytes");
    }

    #[test]
    fn console_run_on_q_draws_the_frame_then_quits_restored() {
        let mut out = Vec::new();
        let dropped = Rc::new(Cell::new(false));
        let end = run(
            Chunks(vec![b"q"]),
            &mut out,
            Mode(Rc::clone(&dropped)),
            floor,
        )
        .expect("run");
        assert_eq!(end, End::Quit);
        assert!(holds(&out, b"viola console\x1b[24;1Hq quit"));
        assert!(out.ends_with(RESTORE));
        assert!(dropped.get());
    }

    #[test]
    fn console_run_on_interrupt_ends_interrupted() {
        let mut out = Vec::new();
        let end = run(Chunks(vec![b"\x03"]), &mut out, (), floor).expect("run");
        assert_eq!(end, End::Interrupt);
    }

    #[test]
    fn console_run_goes_on_past_an_unbound_key_until_the_input_ends() {
        let mut out = Vec::new();
        let end = run(Chunks(vec![b"x", b"y"]), &mut out, (), floor).expect("run");
        assert_eq!(end, End::Closed);
        assert!(out.ends_with(RESTORE));
    }

    #[test]
    fn console_run_below_the_floor_draws_the_one_line() {
        let mut out = Vec::new();
        let small = || Some(Size { cols: 60, rows: 20 });
        run(Chunks(vec![b"q"]), &mut out, (), small).expect("run");
        assert!(holds(
            &out,
            b"viola console needs 80x24; this terminal is 60x20"
        ));
        assert!(!holds(&out, b"q quit"));
    }

    #[test]
    fn console_run_without_a_size_draws_no_frame() {
        let mut out = Vec::new();
        run(Chunks(vec![b"q"]), &mut out, (), || None).expect("run");
        assert!(holds(&out, b"this terminal is 0x0"));
    }

    #[test]
    fn console_run_redraws_at_a_changed_size() {
        let mut out = Vec::new();
        let mut calls = 0u32;
        let size = move || {
            calls += 1;
            Some(if calls == 1 {
                Size::DEFAULT
            } else {
                Size {
                    cols: 100,
                    rows: 30,
                }
            })
        };
        run(Chunks(vec![b"x", b"q"]), &mut out, (), size).expect("run");
        assert!(holds(&out, b"\x1b[24;1Hq quit"));
        assert!(holds(&out, b"\x1b[30;1Hq quit"));
    }

    #[test]
    fn console_run_draws_once_while_the_size_stands() {
        let mut out = Vec::new();
        run(Chunks(vec![b"x", b"y", b"q"]), &mut out, (), floor).expect("run");
        let frames = out
            .windows(b"q quit".len())
            .filter(|w| *w == b"q quit")
            .count();
        assert_eq!(frames, 1);
    }
}
