//! The terminal fleet console (`viola console`): it owns its terminal, draws the fleet of its home
//! between its title row and its key line, stays whole at the floor size and says so below it,
//! reads its own keys and quits cleanly. The board is read again once a second and the frame drawn
//! again only when what it shows, or the size, changed. A selection names one wrapped row, and an
//! action runs a CLI verb on it as a child of the console's own exe (`action`), one at a time, off
//! the key loop; the console itself sends no channel frame and calls no compositor. Every state
//! change is one codes-only line (`console-frame`, `console-key`, `console-action`); no key's
//! bytes, name or child's output reach a line.

pub(crate) mod action;
pub(crate) mod fleet;
mod frame;
mod keys;
mod view;

use std::io::{self, Read, Write};
use std::mem::ManuallyDrop;
use std::sync::mpsc::{Receiver, RecvTimeoutError, TryRecvError};
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use viola_core::MAX_FRAME;
use viola_core::obs::ObsEvent;
use viola_core::obs_event;
use viola_pty::Size;
use viola_state::board::{Board, ScanRefused};

use crate::human::{self, Pending};
use action::{Action, Behavior, Outcome, Verb};
use keys::{Decoder, Key};
use view::{Selection, View, wrapped_names};

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
/// the clock the rows' ages are read against, how often the board is read again, and how an
/// action's child is started (its outcome on the channel returned).
pub(crate) struct Sources<S, B, C, A> {
    pub(crate) size: S,
    pub(crate) board: B,
    pub(crate) clock: C,
    pub(crate) read_every: Duration,
    pub(crate) act: A,
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
            human::write_console_frame(&mut self.out, size.rows, view.lines(), view.keys())?;
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

/// What the key line's row holds: the list's bindings, a confirm's prompt for its action, the
/// editing line of a send, switch or open, or the answer prompt of a pending permission or plan.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Mode {
    List,
    Confirm(Action),
    Editing {
        verb: Verb,
        name: String,
        text: String,
    },
    Answer {
        name: String,
        dialog: Pending,
    },
}

/// An editing read's keys folded by class: the scalars appended, those removed, the keys dropped.
#[derive(Debug, Default)]
struct Fold {
    text: u64,
    backspace: u64,
    unbound: u64,
}

impl Fold {
    /// One `console-key` line a class whose count is at least 1, then the counts cleared.
    fn write(&mut self) {
        for (word, count) in [
            ("text", self.text),
            ("backspace", self.backspace),
            ("unbound", self.unbound),
        ] {
            if count > 0 {
                obs_event!(INFO, ObsEvent::ConsoleKey, key = word, count = count);
            }
        }
        *self = Self::default();
    }
}

/// What the body's lower rows show of an action.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Shown {
    Nothing,
    Running(Verb, String),
    Done(Vec<String>),
}

/// The loop's own state beside the board: the mode, the wrapped rows' names the last view drew,
/// the selection, the action shown and the child running, if one is.
struct State {
    mode: Mode,
    names: Vec<String>,
    /// The wrapped rows' pending dialogs, by name, as the last view drew them.
    dialogs: Vec<(String, Pending)>,
    selection: Option<Selection>,
    shown: Shown,
    running: Option<Receiver<Outcome>>,
}

impl State {
    fn new() -> Self {
        Self {
            mode: Mode::List,
            names: Vec::new(),
            dialogs: Vec::new(),
            selection: None,
            shown: Shown::Nothing,
            running: None,
        }
    }

    /// The view the board gives at `size` as of `now`; the selection follows the board first.
    fn view(&mut self, board: &Board, size: Size, now: DateTime<Utc>) -> View {
        let lines = human::fleet_lines(board, now);
        let names = wrapped_names(&lines);
        self.selection = Selection::follow(self.selection.as_ref(), &names);
        self.names = names.into_iter().map(str::to_owned).collect();
        self.dialogs = lines
            .iter()
            .filter_map(|line| Some((line.name.clone()?, line.dialog.clone()?)))
            .collect();
        let keys = match &self.mode {
            Mode::List | Mode::Editing { .. } => human::CONSOLE_KEYS.to_owned(),
            Mode::Confirm(action) => {
                human::console_confirm_prompt(action.verb.label(), &action.name)
            }
            Mode::Answer { name, dialog } => {
                human::console_answer_prompt(name, &dialog.kind).unwrap_or_default()
            }
        };
        let result = match &self.shown {
            Shown::Nothing => Vec::new(),
            Shown::Running(verb, name) => vec![human::console_running_line(verb.label(), name)],
            Shown::Done(lines) => lines.clone(),
        };
        let selected = self.selection.as_ref().map(Selection::name);
        let view = View::of(&lines, size, selected, &result, &keys);
        match &self.mode {
            Mode::Editing { verb, name, text } => {
                let prompt = human::console_editing_prompt(verb.label(), name);
                view.with_fitted_keys(human::console_editing_line(
                    &prompt,
                    text,
                    usize::from(size.cols),
                ))
            }
            _ => view,
        }
    }

    /// One read's keys in turn. In the list, confirm and answer modes each key writes its own
    /// line; on the editing line the read's keys fold by class into at most three lines, then
    /// one for the enter, escape or interrupt that ends the read (the rest of it dropped).
    fn press_read<A>(&mut self, keys: Vec<Key>, act: &mut A) -> Option<End>
    where
        A: FnMut(Action) -> Receiver<Outcome>,
    {
        let mut fold = Fold::default();
        for key in keys {
            let Mode::Editing { text, .. } = &mut self.mode else {
                if let Some(end) = self.press(key, act) {
                    return Some(end);
                }
                continue;
            };
            match key {
                Key::Char(c)
                    if u64::try_from(text.len() + c.len_utf8()).is_ok_and(|n| n <= MAX_FRAME) =>
                {
                    text.push(c);
                    fold.text += 1;
                }
                Key::Backspace if text.pop().is_some() => fold.backspace += 1,
                Key::Enter | Key::Escape | Key::Interrupt => {
                    fold.write();
                    return self.press(key, act);
                }
                _ => fold.unbound += 1,
            }
        }
        fold.write();
        None
    }

    /// One key in the current mode: the `console-key` line of its meaning, then what it does;
    /// `Some` when it ends the console.
    fn press<A>(&mut self, key: Key, act: &mut A) -> Option<End>
    where
        A: FnMut(Action) -> Receiver<Outcome>,
    {
        let (word, end) = match std::mem::replace(&mut self.mode, Mode::List) {
            Mode::List => self.list_key(key, act),
            Mode::Confirm(action) => self.confirm_key(key, action, act),
            Mode::Editing { verb, name, text } => self.editing_end(key, verb, &name, &text, act),
            Mode::Answer { name, dialog } => self.answer_key(key, name, dialog, act),
        };
        obs_event!(INFO, ObsEvent::ConsoleKey, key = word);
        end
    }

    /// The key that ends the editing line: enter runs its verb on the text, escape cancels.
    fn editing_end<A>(
        &mut self,
        key: Key,
        verb: Verb,
        name: &str,
        text: &str,
        act: &mut A,
    ) -> (&'static str, Option<End>)
    where
        A: FnMut(Action) -> Receiver<Outcome>,
    {
        match key {
            Key::Interrupt => ("interrupt", Some(End::Interrupt)),
            Key::Enter => {
                let action = match verb {
                    Verb::Switch => Action::switch(name, text),
                    Verb::Open => Action::open(name, text),
                    _ => Action::send(name, text),
                };
                self.start(action, act);
                ("enter", None)
            }
            _ => ("escape", None),
        }
    }

    fn answer_key<A>(
        &mut self,
        key: Key,
        name: String,
        dialog: Pending,
        act: &mut A,
    ) -> (&'static str, Option<End>)
    where
        A: FnMut(Action) -> Receiver<Outcome>,
    {
        let plan = dialog.kind == "plan";
        let behavior = match key {
            Key::Interrupt => return ("interrupt", Some(End::Interrupt)),
            Key::Escape => return ("escape", None),
            Key::Char('y') if plan => Behavior::Approve,
            Key::Char('y') => Behavior::Allow,
            Key::Char('n') if plan => Behavior::Revise,
            Key::Char('n') => Behavior::Deny,
            _ => {
                self.mode = Mode::Answer { name, dialog };
                return ("unbound", None);
            }
        };
        if self.running.is_some() {
            return ("busy", None);
        }
        self.start(Action::answer(&name, dialog.id, behavior), act);
        (if key == Key::Char('y') { "yes" } else { "no" }, None)
    }

    fn list_key<A>(&mut self, key: Key, act: &mut A) -> (&'static str, Option<End>)
    where
        A: FnMut(Action) -> Receiver<Outcome>,
    {
        let verb = match key {
            Key::Interrupt => return ("interrupt", Some(End::Interrupt)),
            Key::Char('q') => return ("quit", Some(End::Quit)),
            Key::Up | Key::Char('k') => return (self.moved(true), None),
            Key::Down | Key::Char('j') => return (self.moved(false), None),
            Key::Escape => {
                if matches!(self.shown, Shown::Done(_)) {
                    self.shown = Shown::Nothing;
                }
                return ("escape", None);
            }
            Key::Char('l') => Verb::Last,
            Key::Char('p') => Verb::Pause,
            Key::Char('f') => Verb::Focus,
            Key::Char('r') => Verb::Release,
            Key::Char('v') => Verb::Up,
            Key::Char('s') => Verb::Send,
            Key::Char('m') => Verb::Switch,
            Key::Char('o') => Verb::Open,
            Key::Char('a') => Verb::Answer,
            _ => return ("unbound", None),
        };
        let Some(name) = self.selection.as_ref().map(|s| s.name().to_owned()) else {
            return ("unbound", None);
        };
        if verb == Verb::Answer {
            return self.answer_opened(name);
        }
        if self.running.is_some() {
            return ("busy", None);
        }
        match verb {
            Verb::Release | Verb::Up => self.mode = Mode::Confirm(Action::on(verb, &name)),
            Verb::Send | Verb::Switch | Verb::Open => {
                self.mode = Mode::Editing {
                    verb,
                    name,
                    text: String::new(),
                };
            }
            _ => self.start(Action::on(verb, &name), act),
        }
        ("action", None)
    }

    /// `a` on the selected row: a permission or a plan opens its answer prompt, a question draws
    /// its one line and starts nothing, no dialog (or a kind none of the three) is unbound. Only
    /// the board's kind word and id are read, never a dialog's text.
    fn answer_opened(&mut self, name: String) -> (&'static str, Option<End>) {
        let Some(dialog) = self
            .dialogs
            .iter()
            .find(|(held, _)| *held == name)
            .map(|(_, dialog)| dialog.clone())
        else {
            return ("unbound", None);
        };
        match dialog.kind.as_str() {
            "question" => {
                if self.running.is_none() {
                    self.shown = Shown::Done(vec![human::console_question_line(&name)]);
                }
                ("question", None)
            }
            "permission" | "plan" if self.running.is_some() => ("busy", None),
            "permission" | "plan" => {
                self.mode = Mode::Answer { name, dialog };
                ("action", None)
            }
            _ => ("unbound", None),
        }
    }

    fn confirm_key<A>(
        &mut self,
        key: Key,
        action: Action,
        act: &mut A,
    ) -> (&'static str, Option<End>)
    where
        A: FnMut(Action) -> Receiver<Outcome>,
    {
        match key {
            Key::Interrupt => ("interrupt", Some(End::Interrupt)),
            Key::Char('y') if self.running.is_some() => ("busy", None),
            Key::Char('y') => {
                self.start(action, act);
                ("yes", None)
            }
            Key::Char('n') => ("no", None),
            Key::Escape => ("escape", None),
            _ => {
                self.mode = Mode::Confirm(action);
                ("unbound", None)
            }
        }
    }

    /// The selection one row up or down over the wrapped rows the last view drew.
    fn moved(&mut self, up: bool) -> &'static str {
        let names: Vec<&str> = self.names.iter().map(String::as_str).collect();
        self.selection = match &self.selection {
            Some(selection) => selection.moved(&names, up),
            None => Selection::follow(None, &names),
        };
        if up { "up" } else { "down" }
    }

    fn start<A>(&mut self, action: Action, act: &mut A)
    where
        A: FnMut(Action) -> Receiver<Outcome>,
    {
        self.shown = Shown::Running(action.verb, action.name.clone());
        self.running = Some(act(action));
    }

    /// A finished child: one `console-action` line and its lines shown.
    fn reap(&mut self) {
        let Some(running) = &self.running else {
            return;
        };
        match running.try_recv() {
            Ok(outcome) => {
                obs_event!(
                    INFO,
                    ObsEvent::ConsoleAction,
                    verb = outcome.verb.word(),
                    exit_code = outcome.exit_code,
                );
                self.shown = Shown::Done(outcome.lines);
                self.running = None;
            }
            Err(TryRecvError::Disconnected) => {
                self.shown = Shown::Nothing;
                self.running = None;
            }
            Err(TryRecvError::Empty) => {}
        }
    }
}

/// The console's loop over `input` (keys), `out` (the terminal) and `mode` (its raw mode, held
/// until the restore), its size, board and clock read through `sources`. The board is read at the
/// start and then once every `read_every`, checked at each key and each 100 ms poll (sooner when a
/// lone ESC's wait ends); after each the view is built and drawn only when it or the size differs
/// from the one drawn. Returns once `q` (in the list), 0x03 (in every mode) or the end of input
/// is read, or a board read's scan is refused, the terminal restored; a read error is returned and
/// a panic in the read resumed, each after the restore too. A child still running at the end runs
/// to its own end.
pub(crate) fn run<R, W, M, S, B, C, A>(
    input: R,
    out: W,
    mode: M,
    mut sources: Sources<S, B, C, A>,
) -> io::Result<End>
where
    R: Read + Send + 'static,
    W: Write,
    S: FnMut() -> Option<Size>,
    B: FnMut() -> io::Result<Result<Board, ScanRefused>>,
    C: FnMut() -> DateTime<Utc>,
    A: FnMut(Action) -> Receiver<Outcome>,
{
    let mut screen = Screen::enter(out, mode)?;
    let Ok(mut board) = (sources.board)()? else {
        return Ok(End::Refused);
    };
    let mut read_at = Instant::now();
    let (keys, reader) = keys::spawn(input);
    let mut decoder = Decoder::default();
    let mut state = State::new();
    let mut size = (sources.size)().unwrap_or(NO_SIZE);
    let mut drawn = (size, state.view(&board, size, (sources.clock)()));
    screen.draw(drawn.0, &drawn.1)?;
    loop {
        let wait = decoder.deadline().map_or(POLL, |at| {
            at.saturating_duration_since(Instant::now()).min(POLL)
        });
        let mut pressed = Vec::new();
        match keys.recv_timeout(wait) {
            Ok(Ok(Some(bytes))) => pressed = decoder.feed(&bytes, Instant::now()),
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
        pressed.extend(decoder.flush(Instant::now()));
        if let Some(end) = state.press_read(pressed, &mut sources.act) {
            return Ok(end);
        }
        state.reap();
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
        let view = state.view(&board, size, (sources.clock)());
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
    use std::sync::mpsc::{self, Sender};
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

    /// An action starter whose child never ends, and nothing is run.
    fn idle(_: Action) -> Receiver<Outcome> {
        mpsc::channel().1
    }

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
                    dialog_id: None,
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
    ) -> Sources<
        S,
        impl FnMut() -> BoardRead,
        impl FnMut() -> DateTime<Utc>,
        impl FnMut(Action) -> Receiver<Outcome>,
    > {
        Sources {
            size,
            board: || Ok(Ok(fleet_of(&[]))),
            clock: noon,
            read_every: Duration::ZERO,
            act: idle,
        }
    }

    type Boards = Box<dyn FnMut() -> BoardRead>;
    type Starter = Box<dyn FnMut(Action) -> Receiver<Outcome>>;
    type Fixed = Sources<fn() -> Option<Size>, Boards, fn() -> DateTime<Utc>, Starter>;
    type Held = Rc<std::cell::RefCell<Vec<(Action, Sender<Outcome>)>>>;

    fn with_boards(reads: Vec<BoardRead>) -> Fixed {
        Sources {
            size: floor,
            board: Box::new(boards(reads)),
            clock: noon,
            read_every: Duration::ZERO,
            act: Box::new(idle),
        }
    }

    /// The actions started, each child held running until its sender is used.
    #[derive(Clone, Default)]
    struct Started(Held);

    impl Started {
        fn starter(&self) -> impl FnMut(Action) -> Receiver<Outcome> + use<> {
            let held = self.clone();
            move |action| {
                let (tx, rx) = mpsc::channel();
                held.0.borrow_mut().push((action, tx));
                rx
            }
        }

        fn actions(&self) -> Vec<Action> {
            self.0.borrow().iter().map(|(a, _)| a.clone()).collect()
        }
    }

    fn acting(names: &[&str], started: &Started) -> Fixed {
        Sources {
            act: Box::new(started.starter()),
            ..with_boards(vec![Ok(Ok(fleet_of(names)))])
        }
    }

    fn key_words(lines: &[Value]) -> Vec<String> {
        lines
            .iter()
            .filter(|l| l["event"] == "console-key")
            .map(|l| l["key"].as_str().expect("a key word").to_owned())
            .collect()
    }

    #[test]
    fn console_run_takes_one_action_at_a_time() {
        let started = Started::default();
        let mut out = Vec::new();
        let (end, lines) = captured(|| {
            run(
                Chunks(vec![b"l", b"p", b"r", b"q"]),
                &mut out,
                (),
                acting(&["builder"], &started),
            )
        });
        assert_eq!(end.expect("run"), End::Quit);
        assert_eq!(started.actions(), [Action::on(Verb::Last, "builder")]);
        assert_eq!(key_words(&lines), ["action", "busy", "busy", "quit"]);
    }

    #[test]
    fn console_run_acts_on_the_selected_row() {
        let started = Started::default();
        let mut out = Vec::new();
        let (_, lines) = captured(|| {
            run(
                Chunks(vec![b"j", b"\x1b[B", b"k", b"f", b"q"]),
                &mut out,
                (),
                acting(&["aaa", "bbb", "ccc"], &started),
            )
        });
        assert_eq!(started.actions(), [Action::on(Verb::Focus, "bbb")]);
        assert_eq!(key_words(&lines), ["down", "down", "up", "action", "quit"]);
        assert!(holds(&out, b"focus  bbb  running"));
    }

    #[test]
    fn console_run_release_runs_only_on_yes() {
        let started = Started::default();
        let mut out = Vec::new();
        let (_, lines) = captured(|| {
            run(
                Chunks(vec![b"r", b"x", b"n", b"r", b"n", b"v", b"y", b"q"]),
                &mut out,
                (),
                acting(&["builder"], &started),
            )
        });
        assert_eq!(started.actions(), [Action::on(Verb::Up, "builder")]);
        assert_eq!(
            key_words(&lines),
            [
                "action", "unbound", "no", "action", "no", "action", "yes", "quit"
            ]
        );
        assert!(holds(&out, b"\x1b[24;1Hrelease builder? y/n"));
        assert!(holds(&out, b"\x1b[24;1Hrevive builder? y/n"));
    }

    /// A lone ESC is read only at its deadline, so the confirm's escape is pressed on the state.
    #[test]
    fn console_state_escape_leaves_the_confirm_and_clears_a_result() {
        let started = Started::default();
        let mut act = started.starter();
        let mut state = State::new();
        state.names = vec!["builder".to_owned()];
        state.selection = Selection::follow(None, &["builder"]);
        let (_, lines) = captured(|| {
            assert_eq!(state.press(Key::Char('r'), &mut act), None);
            assert!(matches!(state.mode, super::Mode::Confirm(_)));
            assert_eq!(state.press(Key::Escape, &mut act), None);
            assert_eq!(state.mode, super::Mode::List);
            state.shown = Shown::Done(vec!["out".to_owned()]);
            assert_eq!(state.press(Key::Escape, &mut act), None);
            assert_eq!(state.shown, Shown::Nothing);
        });
        assert!(started.actions().is_empty());
        assert_eq!(key_words(&lines), ["action", "escape", "escape"]);
    }

    #[test]
    fn console_run_ctrl_c_ends_from_the_confirm() {
        let started = Started::default();
        let mut out = Vec::new();
        let end = run(
            Chunks(vec![b"r", b"\x03"]),
            &mut out,
            (),
            acting(&["builder"], &started),
        )
        .expect("run");
        assert_eq!(end, End::Interrupt);
        assert!(started.actions().is_empty());
        assert!(out.ends_with(RESTORE));
    }

    #[test]
    fn console_run_with_no_selection_runs_nothing() {
        let started = Started::default();
        let mut out = Vec::new();
        let (_, lines) = captured(|| {
            run(
                Chunks(vec![b"l", b"r", b"q"]),
                &mut out,
                (),
                acting(&[], &started),
            )
        });
        assert!(started.actions().is_empty());
        assert_eq!(key_words(&lines), ["unbound", "unbound", "quit"]);
    }

    #[test]
    fn console_run_logs_a_finished_child_and_draws_its_lines() {
        let mut out = Vec::new();
        let sources = Sources {
            size: floor,
            board: boards(vec![Ok(Ok(fleet_of(&["builder"])))]),
            clock: noon,
            read_every: Duration::ZERO,
            act: |action: Action| {
                let (tx, rx) = mpsc::channel();
                tx.send(Outcome {
                    verb: action.verb,
                    name: action.name,
                    exit_code: 21,
                    lines: vec!["unable  builder  instance-unreachable".to_owned()],
                })
                .expect("send");
                rx
            },
        };
        let (_, lines) = captured(|| run(Chunks(vec![b"p", b"q"]), &mut out, (), sources));
        let actions: Vec<(&str, i64)> = lines
            .iter()
            .filter(|l| l["event"] == "console-action")
            .map(|l| {
                (
                    l["verb"].as_str().expect("verb"),
                    l["exit_code"].as_i64().expect("exit_code"),
                )
            })
            .collect();
        assert_eq!(actions, [("pause", 21)]);
        assert!(holds(&out, b"unable  builder  instance-unreachable"));
    }

    /// Each `console-key` line as its word and its count, if it carries one.
    fn key_counts(lines: &[Value]) -> Vec<(String, Option<u64>)> {
        lines
            .iter()
            .filter(|l| l["event"] == "console-key")
            .map(|l| {
                (
                    l["key"].as_str().expect("a key word").to_owned(),
                    l["count"].as_u64(),
                )
            })
            .collect()
    }

    fn word(w: &str, count: Option<u64>) -> (String, Option<u64>) {
        (w.to_owned(), count)
    }

    #[test]
    fn editing_a_read_of_text_writes_one_text_line() {
        let started = Started::default();
        let mut out = Vec::new();
        let paste: &'static [u8] = &[b'x'; 64];
        let (_, lines) = captured(|| {
            run(
                Chunks(vec![b"s", paste, b"\r", b"q"]),
                &mut out,
                (),
                acting(&["builder"], &started),
            )
        });
        assert_eq!(
            key_counts(&lines),
            [
                word("action", None),
                word("text", Some(64)),
                word("enter", None),
                word("quit", None),
            ]
        );
        assert_eq!(
            started.actions(),
            [Action::send("builder", &"x".repeat(64))]
        );
    }

    #[test]
    fn editing_letters_are_text_and_backspace_removes_one_scalar() {
        let started = Started::default();
        let mut out = Vec::new();
        let (_, lines) = captured(|| {
            run(
                Chunks(vec![b"s", "qjkпé\x7f\x09".as_bytes(), b"\r", b"q"]),
                &mut out,
                (),
                acting(&["builder"], &started),
            )
        });
        assert_eq!(started.actions(), [Action::send("builder", "qjkп")]);
        assert_eq!(
            key_counts(&lines),
            [
                word("action", None),
                word("text", Some(5)),
                word("backspace", Some(1)),
                word("unbound", Some(1)),
                word("enter", None),
                word("quit", None),
            ]
        );
        for line in &lines {
            assert!(!line.to_string().contains("qjk"), "no typed text in a line");
        }
    }

    #[test]
    fn editing_a_cr_ends_the_read_and_drops_the_rest() {
        let started = Started::default();
        let mut out = Vec::new();
        let (_, lines) = captured(|| {
            run(
                Chunks(vec![b"s", b"ab\rcdq", b"q"]),
                &mut out,
                (),
                acting(&["builder"], &started),
            )
        });
        assert_eq!(started.actions(), [Action::send("builder", "ab")]);
        assert_eq!(
            key_counts(&lines),
            [
                word("action", None),
                word("text", Some(2)),
                word("enter", None),
                word("quit", None),
            ]
        );
    }

    #[test]
    fn editing_opens_in_the_same_read_as_its_letter_and_the_letter_keeps_its_line() {
        let started = Started::default();
        let mut out = Vec::new();
        let (_, lines) = captured(|| {
            run(
                Chunks(vec![b"jmopus high\r", b"q"]),
                &mut out,
                (),
                acting(&["builder"], &started),
            )
        });
        assert_eq!(started.actions(), [Action::switch("builder", "opus high")]);
        assert_eq!(
            key_counts(&lines),
            [
                word("down", None),
                word("action", None),
                word("text", Some(9)),
                word("enter", None),
                word("quit", None),
            ]
        );
    }

    #[test]
    fn editing_escape_cancels_and_runs_nothing() {
        let started = Started::default();
        let mut act = started.starter();
        let mut state = State::new();
        state.names = vec!["builder".to_owned()];
        state.selection = Selection::follow(None, &["builder"]);
        let (_, lines) = captured(|| {
            assert_eq!(
                state.press_read(vec![Key::Char('o'), Key::Char('c')], &mut act),
                None
            );
            assert!(matches!(state.mode, super::Mode::Editing { .. }));
            assert_eq!(state.press_read(vec![Key::Escape], &mut act), None);
            assert_eq!(state.mode, super::Mode::List);
        });
        assert!(started.actions().is_empty());
        assert_eq!(
            key_counts(&lines),
            [
                word("action", None),
                word("text", Some(1)),
                word("escape", None)
            ]
        );
    }

    #[test]
    fn editing_ctrl_c_ends_the_console() {
        let started = Started::default();
        let mut out = Vec::new();
        let end = run(
            Chunks(vec![b"s", b"ab", b"\x03"]),
            &mut out,
            (),
            acting(&["builder"], &started),
        )
        .expect("run");
        assert_eq!(end, End::Interrupt);
        assert!(started.actions().is_empty());
        assert!(out.ends_with(RESTORE));
    }

    #[test]
    fn editing_text_past_the_bound_appends_nothing() {
        let started = Started::default();
        let mut act = started.starter();
        let mut state = State::new();
        let full = "x".repeat(usize::try_from(MAX_FRAME).expect("fits"));
        state.mode = super::Mode::Editing {
            verb: Verb::Send,
            name: "builder".to_owned(),
            text: full.clone(),
        };
        let (_, lines) = captured(|| state.press_read(vec![Key::Char('y')], &mut act));
        let super::Mode::Editing { text, .. } = &state.mode else {
            panic!("still editing");
        };
        assert_eq!(text.len(), full.len());
        assert_eq!(key_counts(&lines), [word("unbound", Some(1))]);
    }

    #[test]
    fn editing_line_is_drawn_in_the_key_line_as_typed() {
        let started = Started::default();
        let mut out = Vec::new();
        run(
            Chunks(vec![b"s", "привет".as_bytes(), b"\x1b[A", b"q"]),
            &mut out,
            (),
            acting(&["builder"], &started),
        )
        .expect("run");
        assert!(holds(&out, "\x1b[24;1Hsend builder: привет".as_bytes()));
        assert!(holds(&out, "send builder: приветq".as_bytes()));
    }

    fn with_dialog(name: &str, kind: &str, id: u64) -> Board {
        let mut board = fleet_of(&[name]);
        board.items[0].dialog_pending = true;
        board.items[0].dialog_kind = Some(kind.to_owned());
        board.items[0].dialog_id = Some(id);
        board
    }

    fn answering(board: Board, started: &Started) -> Fixed {
        Sources {
            act: Box::new(started.starter()),
            ..with_boards(vec![Ok(Ok(board))])
        }
    }

    #[test]
    fn console_run_answer_permission_by_y_and_plan_by_n() {
        for (kind, keys, behavior, prompt) in [
            (
                "permission",
                b"y",
                Behavior::Allow,
                "answer builder permission? y allow  n deny",
            ),
            (
                "plan",
                b"n",
                Behavior::Revise,
                "answer builder plan? y approve  n revise",
            ),
        ] {
            let started = Started::default();
            let mut out = Vec::new();
            let keys: &'static [u8] = keys;
            let (_, lines) = captured(|| {
                run(
                    Chunks(vec![b"a", b"x", keys, b"q"]),
                    &mut out,
                    (),
                    answering(with_dialog("builder", kind, 9), &started),
                )
            });
            assert_eq!(started.actions(), [Action::answer("builder", 9, behavior)]);
            let answered = if behavior == Behavior::Allow {
                "yes"
            } else {
                "no"
            };
            assert_eq!(key_words(&lines), ["action", "unbound", answered, "quit"]);
            assert!(holds(&out, format!("\x1b[24;1H{prompt}").as_bytes()));
        }
    }

    #[test]
    fn console_run_answer_on_a_question_draws_one_line_and_runs_nothing() {
        let started = Started::default();
        let mut out = Vec::new();
        let (_, lines) = captured(|| {
            run(
                Chunks(vec![b"a", b"q"]),
                &mut out,
                (),
                answering(with_dialog("builder", "question", 4), &started),
            )
        });
        assert!(started.actions().is_empty());
        assert_eq!(key_words(&lines), ["question", "quit"]);
        assert!(holds(
            &out,
            b"builder  question - answer it in the session (f)"
        ));
        assert!(!lines.iter().any(|l| l["event"] == "console-action"));
    }

    #[test]
    fn console_run_answer_with_nothing_pending_is_unbound() {
        let started = Started::default();
        let mut out = Vec::new();
        let (_, lines) = captured(|| {
            run(
                Chunks(vec![b"a", b"q"]),
                &mut out,
                (),
                acting(&["builder"], &started),
            )
        });
        assert!(started.actions().is_empty());
        assert_eq!(key_words(&lines), ["unbound", "quit"]);
    }

    #[test]
    fn console_state_escape_leaves_the_answer_prompt() {
        let started = Started::default();
        let mut act = started.starter();
        let mut state = State::new();
        state.names = vec!["builder".to_owned()];
        state.dialogs = vec![(
            "builder".to_owned(),
            Pending {
                kind: "permission".to_owned(),
                id: 2,
            },
        )];
        state.selection = Selection::follow(None, &["builder"]);
        let (_, lines) = captured(|| {
            assert_eq!(state.press(Key::Char('a'), &mut act), None);
            assert!(matches!(state.mode, super::Mode::Answer { .. }));
            assert_eq!(state.press(Key::Escape, &mut act), None);
            assert_eq!(state.mode, super::Mode::List);
        });
        assert!(started.actions().is_empty());
        assert_eq!(key_words(&lines), ["action", "escape"]);
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
        assert!(holds(&out, b"\x1b[3;1H  no wrapped sessions"));
        assert!(holds(&out, b"\x1b[3;1H> builder  live"));
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
        // Wide enough that the gutter cuts no row's LAST cell, whose tick is the redraw read here.
        let sources = Sources {
            size: || {
                Some(Size {
                    cols: 100,
                    rows: 24,
                })
            },
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
            act: idle,
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
            act: idle,
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
    fn console_run_on_q_draws_the_whole_key_line_at_80_columns() {
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
            b"viola console\x1b[2;1H  NAME  LIVE   STATUS   WHEEL"
        ));
        assert!(holds(
            &out,
            b"\x1b[3;1H  no wrapped sessions  start one: viola run <name> -- claude\x1b[24;1Hq quit  jk  l last  p pause  r rel  f focus  v rev  s send  m sw  a ans  o open"
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
