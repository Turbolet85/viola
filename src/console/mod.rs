//! The terminal fleet console (`viola console`): it owns its terminal, draws the fleet of its home
//! between its title row and its key line, stays whole at the floor size and says so below it,
//! reads its own keys and quits cleanly. The board is read again once a second and the frame drawn
//! again only when what it shows, or the size, changed. Every state change is one codes-only line
//! (`console-frame`, `console-key`); no key's bytes reach a line. It sends no channel frame and
//! calls no compositor.

pub(crate) mod fleet;
mod frame;
mod keys;
mod view;

use std::io::{self, Read, Write};
use std::mem::ManuallyDrop;
use std::sync::mpsc::RecvTimeoutError;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use viola_core::obs::ObsEvent;
use viola_core::obs_event;
use viola_pty::Size;
use viola_state::board::{Board, ScanRefused};

use crate::human;
use keys::Key;
use view::View;

/// How often the size is read between keys, as the pump polls it.
const POLL: Duration = Duration::from_millis(100);

/// How often the board is read again.
pub(crate) const READ_EVERY: Duration = Duration::from_secs(1);

/// The size drawn when none can be read: no frame fits it.
const NO_SIZE: Size = Size { cols: 0, rows: 0 };

/// How the loop ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum End {
    Quit,
    Interrupt,
    /// The input ended (the terminal hung up).
    Closed,
    /// A board read's scan was refused: the home, `instances/` or an instance another user may
    /// read.
    Refused,
}

/// What the loop reads besides its keys: the terminal's size, the board (or its refused scan),
/// the clock the rows' ages are read against, and how often the board is read again.
pub(crate) struct Sources<S, B, C> {
    pub(crate) size: S,
    pub(crate) board: B,
    pub(crate) clock: C,
    pub(crate) read_every: Duration,
}

/// The fields of one `console-frame` line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Logged {
    cols: u16,
    rows: u16,
    fits: bool,
    shown: u64,
    hidden: u64,
}

/// The terminal while the console holds it: the alternate screen and the hidden cursor, with the
/// terminal-mode handle `M` (the raw mode). Its drop writes the restore sequence and only then
/// drops `M`, on every exit path, an error or an unwinding panic included.
struct Screen<W: Write, M> {
    out: W,
    /// Dropped by this guard's body alone, after the restore sequence.
    mode: Option<ManuallyDrop<M>>,
    /// The last `console-frame` line's fields.
    logged: Option<Logged>,
}

impl<W: Write, M> Screen<W, M> {
    /// The guard stands before the first byte, so a failed enter still restores.
    fn enter(out: W, mode: M) -> io::Result<Self> {
        let mut screen = Self {
            out,
            mode: Some(ManuallyDrop::new(mode)),
            logged: None,
        };
        human::write_console_enter(&mut screen.out)?;
        Ok(screen)
    }

    /// The whole frame at `size` holding `view`, or the one floor line below it; then a
    /// `console-frame` line when one of its fields differs from the line last written.
    fn draw(&mut self, size: Size, view: &View) -> io::Result<()> {
        let fits = frame::fits(size);
        if fits {
            human::write_console_frame(&mut self.out, size.rows, view.lines())?;
        } else {
            human::write_console_too_small(
                &mut self.out,
                (frame::FLOOR.cols, frame::FLOOR.rows),
                (size.cols, size.rows),
            )?;
        }
        let now = Logged {
            cols: size.cols,
            rows: size.rows,
            fits,
            shown: view.shown(),
            hidden: view.hidden(),
        };
        if self.logged != Some(now) {
            obs_event!(
                INFO,
                ObsEvent::ConsoleFrame,
                cols = now.cols,
                rows = now.rows,
                fits = now.fits,
                shown = now.shown,
                hidden = now.hidden,
            );
            self.logged = Some(now);
        }
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

/// The view the board gives at `size` as of `now`.
fn view_of(board: &Board, size: Size, now: DateTime<Utc>) -> View {
    View::of(&human::fleet_lines(board, now), size)
}

/// The console's loop over `input` (keys), `out` (the terminal) and `mode` (its raw mode, held
/// until the restore), its size, board and clock read through `sources`. The board is read at the
/// start and then once every `read_every`, checked at each key and each 100 ms poll; after each
/// the view is built and drawn only when it or the size differs from the one drawn. Returns once
/// `q`, 0x03 or the end of input is read, or a board read's scan is refused, the terminal
/// restored; a read error is returned and a panic in the read resumed, each after the restore
/// too.
pub(crate) fn run<R, W, M, S, B, C>(
    input: R,
    out: W,
    mode: M,
    mut sources: Sources<S, B, C>,
) -> io::Result<End>
where
    R: Read + Send + 'static,
    W: Write,
    S: FnMut() -> Option<Size>,
    B: FnMut() -> io::Result<Result<Board, ScanRefused>>,
    C: FnMut() -> DateTime<Utc>,
{
    let mut screen = Screen::enter(out, mode)?;
    let Ok(mut board) = (sources.board)()? else {
        return Ok(End::Refused);
    };
    let mut read_at = Instant::now();
    let (keys, reader) = keys::spawn(input);
    let mut size = (sources.size)().unwrap_or(NO_SIZE);
    let mut drawn = (size, view_of(&board, size, (sources.clock)()));
    screen.draw(drawn.0, &drawn.1)?;
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
        if read_at.elapsed() >= sources.read_every {
            board = match (sources.board)()? {
                Ok(board) => board,
                Err(_) => return Ok(End::Refused),
            };
            read_at = Instant::now();
        }
        if let Some(now) = (sources.size)() {
            size = now;
        }
        let view = view_of(&board, size, (sources.clock)());
        if (size, &view) != (drawn.0, &drawn.1) {
            drawn = (size, view);
            screen.draw(drawn.0, &drawn.1)?;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    use chrono::TimeZone as _;
    use serde_json::Value;
    use tracing_subscriber::layer::{Context, Layer, SubscriberExt as _};
    use viola_state::board::{Item, JoinKeys, RowLiveness, SessionStatus, SkippedTotal};
    use viola_state::snapshot::Wheel;

    const RESTORE: &[u8] = b"\x1b[?25h\x1b[?1049l";
    const ENTER: &[u8] = b"\x1b[?1049h\x1b[?25l";

    fn floor() -> Option<Size> {
        Some(Size::DEFAULT)
    }

    type BoardRead = io::Result<Result<Board, ScanRefused>>;

    /// The lines `f` wrote on this thread, each its fields as one object.
    pub(crate) fn captured<R>(f: impl FnOnce() -> R) -> (R, Vec<Value>) {
        let lines = Lines::default();
        let subscriber = tracing_subscriber::registry().with(lines.clone());
        let result = crate::test_support::with_scoped_default(subscriber, f);
        let seen = lines.0.lock().expect("lines").clone();
        (result, seen)
    }

    #[derive(Clone, Default)]
    struct Lines(Arc<Mutex<Vec<Value>>>);

    impl<S: tracing::Subscriber> Layer<S> for Lines {
        fn on_event(&self, event: &tracing::Event<'_>, _: Context<'_, S>) {
            let mut fields = viola_channel::test_support::JsonFields::default();
            event.record(&mut fields);
            self.0.lock().expect("lines").push(Value::Object(fields.0));
        }
    }

    fn noon() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 11, 28, 12, 0, 0)
            .single()
            .expect("valid date")
    }

    fn fleet_of(names: &[&str]) -> Board {
        Board {
            items: names
                .iter()
                .map(|name| Item {
                    name: (*name).to_owned(),
                    wrapped: true,
                    liveness: RowLiveness::Live,
                    status: SessionStatus::Unknown,
                    wheel: Some(Wheel::Driver),
                    dialog_pending: false,
                    cli_version: None,
                    cli_verified: None,
                    parent: None,
                    role: None,
                    model: None,
                    effort: None,
                    window: None,
                    origin: None,
                    dialog_kind: None,
                    join: JoinKeys::default(),
                    last_activity: Some(noon()),
                })
                .collect(),
            links: Vec::new(),
            skipped: SkippedTotal::default(),
        }
    }

    /// The boards read in turn, the last one standing once they run out.
    fn boards(mut reads: Vec<BoardRead>) -> impl FnMut() -> BoardRead {
        move || {
            if reads.len() > 1 {
                reads.remove(0)
            } else {
                match &reads[0] {
                    Ok(Ok(board)) => Ok(Ok(board.clone())),
                    Ok(Err(refused)) => Ok(Err(refused.clone())),
                    Err(e) => Err(io::Error::new(e.kind(), "read")),
                }
            }
        }
    }

    /// An empty board read at every turn, the clock standing at noon.
    fn at<S: FnMut() -> Option<Size>>(
        size: S,
    ) -> Sources<S, impl FnMut() -> BoardRead, impl FnMut() -> DateTime<Utc>> {
        Sources {
            size,
            board: || Ok(Ok(fleet_of(&[]))),
            clock: noon,
            read_every: Duration::ZERO,
        }
    }

    fn with_boards(
        reads: Vec<BoardRead>,
    ) -> Sources<
        impl FnMut() -> Option<Size>,
        impl FnMut() -> BoardRead,
        impl FnMut() -> DateTime<Utc>,
    > {
        Sources {
            size: floor,
            board: boards(reads),
            clock: noon,
            read_every: Duration::ZERO,
        }
    }

    fn frames(out: &[u8]) -> usize {
        out.windows(b"q quit".len())
            .filter(|w| *w == b"q quit")
            .count()
    }

    #[test]
    fn console_run_redraws_when_the_view_changes() {
        let mut out = Vec::new();
        let sources = with_boards(vec![Ok(Ok(fleet_of(&[]))), Ok(Ok(fleet_of(&["builder"])))]);
        run(Chunks(vec![b"x", b"q"]), &mut out, (), sources).expect("run");
        assert_eq!(frames(&out), 2);
        assert!(holds(&out, b"\x1b[3;1Hno wrapped sessions"));
        assert!(holds(&out, b"\x1b[3;1Hbuilder  live"));
    }

    #[test]
    fn console_run_draws_once_while_size_and_view_stand() {
        let mut out = Vec::new();
        let sources = with_boards(vec![Ok(Ok(fleet_of(&["builder"])))]);
        run(Chunks(vec![b"x", b"y", b"q"]), &mut out, (), sources).expect("run");
        assert_eq!(frames(&out), 1);
    }

    #[test]
    fn console_run_on_a_refused_board_restores_then_ends() {
        for reads in [
            vec![Ok(Err(ScanRefused::Home))],
            vec![Ok(Ok(fleet_of(&["builder"]))), Ok(Err(ScanRefused::Home))],
        ] {
            let mut out = Vec::new();
            let dropped = Rc::new(Cell::new(false));
            let end = run(
                Chunks(vec![b"x", b"q"]),
                &mut out,
                Mode(Rc::clone(&dropped)),
                with_boards(reads),
            )
            .expect("run");
            assert_eq!(end, End::Refused);
            assert!(out.ends_with(RESTORE), "the restore is the last bytes");
            assert!(dropped.get(), "the raw mode was let go");
        }
    }

    #[test]
    fn console_run_on_a_failed_board_read_restores_then_returns_the_error() {
        let mut out = Vec::new();
        let sources = with_boards(vec![Err(io::Error::from(io::ErrorKind::PermissionDenied))]);
        let result = run(Chunks(vec![b"q"]), &mut out, (), sources);
        assert_eq!(
            result.map_err(|e| e.kind()),
            Err(io::ErrorKind::PermissionDenied)
        );
        assert!(out.ends_with(RESTORE));
    }

    #[test]
    fn console_run_logs_a_frame_only_when_its_fields_change() {
        let mut out = Vec::new();
        let mut ticks = 0i64;
        let sources = Sources {
            size: floor,
            board: boards(vec![
                Ok(Ok(fleet_of(&["builder"]))),
                Ok(Ok(fleet_of(&["builder"]))),
                Ok(Ok(fleet_of(&["builder", "tester"]))),
            ]),
            clock: move || {
                ticks += 1;
                noon() + chrono::Duration::seconds(ticks)
            },
            read_every: Duration::ZERO,
        };
        let (end, lines) = captured(|| run(Chunks(vec![b"x", b"y", b"q"]), &mut out, (), sources));
        assert_eq!(end.expect("run"), End::Quit);
        assert_eq!(frames(&out), 3, "the LAST tick redrew");
        let logged: Vec<(u64, u64)> = lines
            .iter()
            .filter(|line| line["event"] == "console-frame")
            .map(|line| {
                (
                    line["shown"].as_u64().expect("shown"),
                    line["hidden"].as_u64().expect("hidden"),
                )
            })
            .collect();
        assert_eq!(logged, [(1, 0), (2, 0)]);
    }

    #[test]
    fn console_run_reads_the_board_again_only_after_its_interval() {
        let mut out = Vec::new();
        let mut reads = 0u32;
        let sources = Sources {
            size: floor,
            board: || {
                reads += 1;
                Ok(Ok(fleet_of(&[])))
            },
            clock: noon,
            read_every: Duration::from_secs(3_600),
        };
        run(Chunks(vec![b"x", b"y", b"q"]), &mut out, (), sources).expect("run");
        assert_eq!(reads, 1);
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
        let result = run(FailingRead, &mut out, Mode(Rc::clone(&dropped)), at(floor));
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
            run(PanickingRead, writer, (), at(floor))
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
            at(floor),
        )
        .expect("run");
        assert_eq!(end, End::Quit);
        assert!(holds(
            &out,
            b"viola console\x1b[2;1HNAME  LIVE   STATUS   WHEEL"
        ));
        assert!(holds(
            &out,
            b"\x1b[3;1Hno wrapped sessions  start one: viola run <name> -- claude\x1b[24;1Hq quit"
        ));
        assert!(out.ends_with(RESTORE));
        assert!(dropped.get());
    }

    #[test]
    fn console_run_on_interrupt_ends_interrupted() {
        let mut out = Vec::new();
        let end = run(Chunks(vec![b"\x03"]), &mut out, (), at(floor)).expect("run");
        assert_eq!(end, End::Interrupt);
    }

    #[test]
    fn console_run_goes_on_past_an_unbound_key_until_the_input_ends() {
        let mut out = Vec::new();
        let end = run(Chunks(vec![b"x", b"y"]), &mut out, (), at(floor)).expect("run");
        assert_eq!(end, End::Closed);
        assert!(out.ends_with(RESTORE));
    }

    #[test]
    fn console_run_below_the_floor_draws_the_one_line() {
        let mut out = Vec::new();
        let small = || Some(Size { cols: 60, rows: 20 });
        run(Chunks(vec![b"q"]), &mut out, (), at(small)).expect("run");
        assert!(holds(
            &out,
            b"viola console needs 80x24; this terminal is 60x20"
        ));
        assert!(!holds(&out, b"q quit"));
    }

    #[test]
    fn console_run_without_a_size_draws_no_frame() {
        let mut out = Vec::new();
        run(Chunks(vec![b"q"]), &mut out, (), at(|| None)).expect("run");
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
        run(Chunks(vec![b"x", b"q"]), &mut out, (), at(size)).expect("run");
        assert!(holds(&out, b"\x1b[24;1Hq quit"));
        assert!(holds(&out, b"\x1b[30;1Hq quit"));
    }

    #[test]
    fn console_run_draws_once_while_the_size_stands() {
        let mut out = Vec::new();
        run(Chunks(vec![b"x", b"y", b"q"]), &mut out, (), at(floor)).expect("run");
        let frames = out
            .windows(b"q quit".len())
            .filter(|w| *w == b"q quit")
            .count();
        assert_eq!(frames, 1);
    }
}
