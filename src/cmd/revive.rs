//! `viola revive <name>`: a dead instance restarted in the terminal the verb is typed in
//! (architecture §Established Decisions [Session Liveness]). It replays no launch: the program is
//! looked up again by name, the environment is this terminal's, and the child resumes the newest
//! session the instance's log holds, in the directory the dead wrapper recorded. A session a live
//! `claude` already holds is not resumed a second time, unless the revive forks it.

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use serde_json::{Value, json};
use viola_agent_claude::{PROGRAM, RESUME_FLAG, is_session_id, resume_args};
use viola_core::ViolaName;
use viola_state::replay::{Recovered, SessionLink, read_snapshot_or_replay, session_chain};
use viola_state::strict::check_instance;

use super::client;
use super::run::{
    Launch, Started, checked_exe, collision_check, open_wrapper_log, pump_child, refuse_tree,
    refused, start,
};
use crate::human;

#[derive(clap::Args)]
pub(crate) struct ReviveArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = super::send::parse_name)]
    pub(super) name: ViolaName,
    /// Resume this logged session instead of the newest one
    #[arg(long, value_name = "ID", value_parser = parse_session_id)]
    id: Option<String>,
    /// Continue the resumed session under a new session id
    #[arg(long)]
    fork: bool,
    /// Print the instance's logged sessions, oldest first, and start nothing
    #[arg(long, conflicts_with_all = ["id", "fork", "extra"])]
    pub(super) list: bool,
    /// Print one JSON document on stdout instead of the rows or a refusal's lines
    #[arg(long)]
    json: bool,
    /// Extra arguments for the child, after `--`
    #[arg(last = true)]
    extra: Vec<OsString>,
}

fn parse_session_id(raw: &str) -> Result<String, String> {
    if is_session_id(raw) {
        Ok(raw.to_owned())
    } else {
        Err("invalid session id".to_owned())
    }
}

/// A refusal revive words itself. A name held by a live or stale wrapper is `run`'s own refusal,
/// written by its collision check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Refusal {
    StrictModes,
    NoSession,
    UnknownId,
    SessionLive,
    CwdMissing,
}

impl Refusal {
    /// The closed `process-exit` detail (obs-plan §6). A refused strict-modes check is written
    /// with no line, so no line carries its detail.
    const fn detail(self) -> &'static str {
        match self {
            Self::StrictModes => "strict-modes-failed",
            Self::NoSession | Self::UnknownId => "no-session",
            Self::SessionLive => "session-live",
            Self::CwdMissing => "cwd-missing",
        }
    }

    /// The `unable:` and `hint:` texts: fixed words and the instance's name, never the recorded
    /// directory, a pid or an id read from the log.
    fn pair(self, name: &str) -> (String, String) {
        match self {
            Self::StrictModes => human::instance_state_pair(name),
            Self::NoSession => (
                format!("{name} has no logged session to resume"),
                format!("start one: viola run {name} -- {PROGRAM}"),
            ),
            Self::UnknownId => (
                format!("{name} has no logged session with that id"),
                format!("viola revive {name} --list"),
            ),
            Self::SessionLive => (
                format!("{name}'s session is open in another {PROGRAM}"),
                format!("close it there first, or start a copy: viola revive {name} --fork"),
            ),
            Self::CwdMissing => (
                format!("{name}'s recorded directory is missing"),
                format!(
                    "resume it by hand from a directory you choose: viola run {name} -- {PROGRAM} \
                     {RESUME_FLAG} <id>, ids from viola revive {name} --list"
                ),
            ),
        }
    }

    /// The stderr pair, or with `json` the one document naming the closed code.
    fn write(self, name: &ViolaName, json: bool) {
        if json {
            client::refused_document(self.detail());
        } else {
            let (unable, hint) = self.pair(name.as_ref());
            human::refuse(&unable, &hint);
        }
    }
}

pub(crate) fn revive(home: &Path, args: ReviveArgs) -> anyhow::Result<ExitCode> {
    if args.list {
        return list(home, &args.name, args.json);
    }
    let json = args.json;
    // The error still reaches the catch site, for its chain and its line. A `pump_child` error
    // prints no document: the child has held the terminal, and viola writes nothing past it.
    let started = started(home, args).inspect_err(|_| {
        if json {
            client::refused_document("internal-error");
        }
    })?;
    match started {
        Started::Launched(launched) => pump_child(*launched),
        Started::Refused(code) => Ok(code),
    }
}

/// Everything ahead of the child: the checks, the log, the preflight and `run`'s start.
fn started(home: &Path, args: ReviveArgs) -> anyhow::Result<Started> {
    // Before the log opens: opening it narrows the home to its owner, which would erase the very
    // state this check refuses. A refused check opens nothing under the home and writes no line.
    if !trusted(home, &args.name)? {
        Refusal::StrictModes.write(&args.name, args.json);
        return Ok(Started::Refused(ExitCode::from(1)));
    }
    let exe = match checked_exe(home, &args.name)? {
        Ok(exe) => exe,
        Err(_) if args.json => {
            Refusal::StrictModes.write(&args.name, true);
            return Ok(Started::Refused(ExitCode::from(1)));
        }
        Err(refused) => {
            refuse_tree(&args.name, refused);
            return Ok(Started::Refused(ExitCode::from(1)));
        }
    };
    let persistent = open_wrapper_log(home, &args.name)?;
    match preflight(home, args)? {
        Ok(launch) => start(home, &launch, &persistent, &exe),
        Err(refusal) => Ok(refusal),
    }
}

/// Whether the instance's home and files pass the strict-modes check.
fn trusted(home: &Path, name: &ViolaName) -> anyhow::Result<bool> {
    let home = std::path::absolute(home)?;
    let instance_dir = home.join("instances").join(AsRef::<str>::as_ref(name));
    Ok(check_instance(&home, &instance_dir).is_ok())
}

/// The four readings in their fixed order, taken once the strict-modes check has passed: the
/// name's holder, the logged session, whether a live `claude` holds that session, the recorded
/// directory. Each is taken only when every one before it passed, so the first refusal wins and a
/// refused instance is read no further.
fn preflight_order<R, S, D>(
    holder: impl FnOnce() -> Option<R>,
    session: impl FnOnce() -> anyhow::Result<Result<S, R>>,
    session_live: impl FnOnce(&S) -> Option<R>,
    recorded_dir: impl FnOnce(&S) -> Result<D, R>,
) -> anyhow::Result<Result<(S, D), R>> {
    if let Some(refusal) = holder() {
        return Ok(Err(refusal));
    }
    let session = match session()? {
        Ok(session) => session,
        Err(refusal) => return Ok(Err(refusal)),
    };
    if let Some(refusal) = session_live(&session) {
        return Ok(Err(refusal));
    }
    Ok(recorded_dir(&session).map(|dir| (session, dir)))
}

/// Whether a `claude agents --json` read run from `cwd` lists `id`. A read that fails is no
/// refusal: the revive goes on (architecture §Standard Contracts, Session liveness).
fn live_elsewhere(home: &Path, cwd: &Path, id: &str) -> bool {
    crate::run::agents::read_here(home, cwd)
        .is_ok_and(|list| list.rows.iter().any(|row| row.session_id == id))
}

/// The launch a passed preflight yields, or its refusal, already written and logged.
fn preflight(home: &Path, args: ReviveArgs) -> anyhow::Result<Result<Launch, Started>> {
    let name = &args.name;
    let home = std::path::absolute(home)?;
    let instance_dir = home.join("instances").join(AsRef::<str>::as_ref(name));
    let json = args.json;
    let refuse = |refusal: Refusal| {
        refusal.write(name, json);
        refused(refusal.detail())
    };
    let fork = args.fork;
    let cwd = std::env::current_dir()?;
    // One read of the snapshot, through the replay, for the holder and the recorded directory
    // alike: one replay writes one `state-recovered` line.
    let read = read_snapshot_or_replay(&instance_dir);
    let passed = preflight_order(
        || collision_check(name, &instance_dir, read.as_ref().ok(), json),
        || {
            let recovered = match &read {
                Ok(recovered) => recovered.clone(),
                Err(_) => anyhow::bail!("the snapshot could not be read"),
            };
            let chain = session_chain(&instance_dir)?;
            Ok(pick_session(&chain.links, args.id.as_deref())
                .map(|id| (id, recovered))
                .map_err(&refuse))
        },
        |(id, _)| (!fork && live_elsewhere(&home, &cwd, id)).then(|| refuse(Refusal::SessionLive)),
        |(_, recovered)| recorded_dir(recovered).ok_or_else(|| refuse(Refusal::CwdMissing)),
    )?;
    let ((id, _), spawn_dir) = match passed {
        Ok(passed) => passed,
        Err(refusal) => return Ok(Err(refusal)),
    };
    let mut child_args = resume_args(&id, args.fork);
    child_args.extend(args.extra);
    Ok(Ok(Launch {
        name: args.name,
        program: OsString::from(PROGRAM),
        args: child_args,
        spawn_dir,
        json,
    }))
}

/// The session a revive resumes: the asked id when the log holds it, else the newest logged one.
/// Only an id of the session-id shape counts as logged.
fn pick_session(links: &[SessionLink], asked: Option<&str>) -> Result<String, Refusal> {
    let mut ids = links
        .iter()
        .map(|link| link.agent_session_id.as_str())
        .filter(|id| is_session_id(id));
    match asked {
        Some(asked) => ids.find(|id| *id == asked).ok_or(Refusal::UnknownId),
        None => ids.next_back().ok_or(Refusal::NoSession),
    }
    .map(str::to_owned)
}

/// The directory the dead wrapper recorded, while it is still a directory. Only a snapshot that
/// reads holds one: no event carries it, so a replayed or an absent snapshot yields none.
fn recorded_dir(recovered: &Recovered) -> Option<PathBuf> {
    match recovered {
        Recovered::Snapshot(snapshot) => snapshot
            .cwd
            .as_deref()
            .map(PathBuf::from)
            .filter(|dir| dir.is_dir()),
        Recovered::Absent | Recovered::Replayed { .. } => None,
    }
}

/// `--list`: one stdout line per logged session, oldest first, or with `json` one document of
/// them. It starts nothing. Once the strict-modes check has passed, and under a `VIOLA_NAME`, it
/// logs its start and exit as a `cli` verb; a refused check opens nothing and writes no line.
fn list(home: &Path, name: &ViolaName, json: bool) -> anyhow::Result<ExitCode> {
    if !trusted(home, name)? {
        Refusal::StrictModes.write(name, json);
        return Ok(ExitCode::from(1));
    }
    let home = std::path::absolute(home)?;
    if let Some(own) = client::own_name() {
        client::start(&home, &own)?;
    }
    let instance_dir = home.join("instances").join(AsRef::<str>::as_ref(name));
    let rows = list_rows(&session_chain(&instance_dir)?.links);
    if rows.is_empty() {
        Refusal::NoSession.write(name, json);
        crate::run::log_self_exit(1, Some(Refusal::NoSession.detail()));
        return Ok(ExitCode::from(1));
    }
    if json {
        let sessions: Vec<Value> = rows
            .iter()
            .map(|row| json!({"ts": row.ts, "cause": row.cause, "id": row.id}))
            .collect();
        client::document(&json!({"v": 1, "ok": {"sessions": sessions}}));
    } else {
        for row in &rows {
            human::result(&row.line());
        }
    }
    crate::run::log_self_exit(0, None);
    Ok(ExitCode::SUCCESS)
}

/// One `--list` row: the escaped timestamp, the cause word, the session-shaped id.
#[derive(Debug, PartialEq)]
struct ListRow {
    ts: String,
    cause: &'static str,
    id: String,
}

impl ListRow {
    /// `<ts>  <cause>  <id>`, two spaces apart.
    fn line(&self) -> String {
        format!("{}  {}  {}", self.ts, self.cause, self.id)
    }
}

/// A row per link whose id has the session-id shape, in log order.
fn list_rows(links: &[SessionLink]) -> Vec<ListRow> {
    links
        .iter()
        .filter(|link| is_session_id(&link.agent_session_id))
        .map(|link| ListRow {
            ts: human::escape_message(&link.ts),
            cause: cause_word(&link.cause),
            id: link.agent_session_id.clone(),
        })
        .collect()
}

/// A logged cause is printed only when it is one of the four the hook map names.
fn cause_word(cause: &str) -> &'static str {
    match cause {
        "startup" => "startup",
        "clear" => "clear",
        "resume" => "resume",
        "compact" => "compact",
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    use clap::Parser as _;
    use rstest::rstest;
    use viola_state::replay::{ReplayCause, Replayed};
    use viola_state::snapshot::{InstanceSnapshot, Wheel};

    const FIRST: &str = "c0f0cc23-690b-45dd-bbfb-06d6cfd44942";
    const SECOND: &str = "d5d38bc2-fcb5-44d3-b336-6a62b90b7c39";

    /// Each reading answers what the case gives it and writes its own name down when it is taken.
    #[rstest]
    #[case::the_pass([None, None, None, None], None, "holder session live dir")]
    #[case::holder_alone([Some("holder"), None, None, None], Some("holder"), "holder")]
    #[case::session_alone([None, Some("session"), None, None], Some("session"), "holder session")]
    #[case::live_alone([None, None, Some("live"), None], Some("live"), "holder session live")]
    #[case::dir_alone([None, None, None, Some("dir")], Some("dir"), "holder session live dir")]
    #[case::holder_before_session(
        [Some("holder"), Some("session"), None, None],
        Some("holder"),
        "holder"
    )]
    #[case::holder_before_live([Some("holder"), None, Some("live"), None], Some("holder"), "holder")]
    #[case::holder_before_dir([Some("holder"), None, None, Some("dir")], Some("holder"), "holder")]
    #[case::session_before_live(
        [None, Some("session"), Some("live"), None],
        Some("session"),
        "holder session"
    )]
    #[case::session_before_dir(
        [None, Some("session"), None, Some("dir")],
        Some("session"),
        "holder session"
    )]
    #[case::live_before_dir(
        [None, None, Some("live"), Some("dir")],
        Some("live"),
        "holder session live"
    )]
    #[case::all_four(
        [Some("holder"), Some("session"), Some("live"), Some("dir")],
        Some("holder"),
        "holder"
    )]
    fn preflight_order_the_first_refusal_wins_and_no_later_reading_is_taken(
        #[case] readings: [Option<&'static str>; 4],
        #[case] refusal: Option<&'static str>,
        #[case] taken: &str,
    ) {
        let log = RefCell::new(Vec::new());
        let read = |name: &'static str, answer: Option<&'static str>| {
            log.borrow_mut().push(name);
            answer
        };
        let [holder, session, live, dir] = readings;
        let got = preflight_order(
            || read("holder", holder),
            || Ok(read("session", session).map_or(Ok("the session"), Err)),
            |session: &&str| {
                assert_eq!(*session, "the session");
                read("live", live)
            },
            |session: &&str| {
                assert_eq!(*session, "the session");
                read("dir", dir).map_or(Ok("the dir"), Err)
            },
        )
        .expect("no reading failed");
        assert_eq!(got, refusal.map_or(Ok(("the session", "the dir")), Err));
        assert_eq!(log.borrow().join(" "), taken);
    }

    /// A log that cannot be read is no refusal: the error goes up, and the directory is not read.
    #[test]
    fn preflight_order_a_session_reading_that_fails_is_an_error_and_ends_the_readings() {
        let later_taken = RefCell::new(false);
        let got: anyhow::Result<Result<((), ()), &str>> = preflight_order(
            || None,
            || Err(anyhow::anyhow!("unreadable")),
            |(): &()| {
                *later_taken.borrow_mut() = true;
                None
            },
            |(): &()| {
                *later_taken.borrow_mut() = true;
                Ok(())
            },
        );
        assert!(got.is_err());
        assert!(!*later_taken.borrow());
    }

    /// A refused `session-live` reading takes no directory reading.
    #[test]
    fn preflight_order_a_refused_session_live_takes_no_directory_reading() {
        let dir_taken = RefCell::new(false);
        let got = preflight_order(
            || None,
            || Ok(Ok("the session")),
            |_: &&str| Some(Refusal::SessionLive),
            |_: &&str| {
                *dir_taken.borrow_mut() = true;
                Err::<(), _>(Refusal::CwdMissing)
            },
        )
        .expect("no reading failed");
        assert_eq!(got, Err(Refusal::SessionLive));
        assert!(!*dir_taken.borrow());
    }

    fn builder() -> ViolaName {
        ViolaName::try_new("builder".to_owned()).expect("valid")
    }

    #[test]
    fn preflight_check_passes_a_home_viola_laid_down_and_one_not_there_yet() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("vhome");
        assert!(trusted(&home, &builder()).expect("read"));
        viola_state::fs::create_private_dir(&home.join("instances").join("builder"))
            .expect("the instance dir");
        assert!(trusted(&home, &builder()).expect("read"));
    }

    /// A home, or an instance directory, another user could write is refused, and the reading
    /// leaves its mode as found.
    #[cfg(unix)]
    #[rstest]
    #[case::the_home("")]
    #[case::the_instance_directory("instances/builder")]
    fn preflight_check_refuses_a_directory_another_user_could_write(#[case] widened: &str) {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("vhome");
        viola_state::fs::create_private_dir(&home.join("instances").join("builder"))
            .expect("the instance dir");
        let dir = home.join(widened);
        std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o770)).expect("chmod");
        assert!(!trusted(&home, &builder()).expect("read"));
        let mode = std::fs::metadata(&dir).expect("meta").permissions().mode() & 0o777;
        assert_eq!(mode, 0o770);
    }

    fn link(cause: &str, id: &str) -> SessionLink {
        SessionLink {
            ts: "2026-10-10T01:02:03.000Z".to_owned(),
            cause: cause.to_owned(),
            agent_session_id: id.to_owned(),
        }
    }

    #[rstest]
    #[case::the_newest_of_two(&[("startup", FIRST), ("clear", SECOND)], None, Ok(SECOND))]
    #[case::the_one(&[("startup", FIRST)], None, Ok(FIRST))]
    #[case::the_asked_older_one(&[("startup", FIRST), ("clear", SECOND)], Some(FIRST), Ok(FIRST))]
    #[case::the_asked_newest_one(&[("startup", FIRST), ("clear", SECOND)], Some(SECOND), Ok(SECOND))]
    #[case::an_empty_log(&[], None, Err(Refusal::NoSession))]
    #[case::an_empty_log_and_an_asked_id(&[], Some(FIRST), Err(Refusal::UnknownId))]
    #[case::an_id_the_log_does_not_hold(&[("startup", FIRST)], Some(SECOND), Err(Refusal::UnknownId))]
    #[case::only_an_id_of_another_shape(&[("startup", "s-1")], None, Err(Refusal::NoSession))]
    #[case::a_newest_id_of_another_shape_is_stepped_over(
        &[("startup", FIRST), ("clear", "s-2")],
        None,
        Ok(FIRST)
    )]
    #[case::an_asked_id_in_other_case(
        &[("startup", FIRST)],
        Some("C0F0CC23-690B-45DD-BBFB-06D6CFD44942"),
        Err(Refusal::UnknownId)
    )]
    fn preflight_order_third_reading_picks_the_asked_or_the_newest_logged_session(
        #[case] logged: &[(&str, &str)],
        #[case] asked: Option<&str>,
        #[case] picked: Result<&str, Refusal>,
    ) {
        let links: Vec<SessionLink> = logged.iter().map(|(c, id)| link(c, id)).collect();
        assert_eq!(
            pick_session(&links, asked),
            picked.map(str::to_owned),
            "{logged:?}"
        );
    }

    fn snapshot(cwd: Option<&Path>) -> Recovered {
        Recovered::Snapshot(Box::new(InstanceSnapshot {
            endpoint: None,
            pid: 1,
            started_at: "s".to_owned(),
            pinned_bin: "b".to_owned(),
            cli_verified: true,
            cli_version: None,
            wheel: Wheel::Driver,
            budget_paused: false,
            budget_override_until: None,
            links: Vec::new(),
            child_pid: None,
            pending_dialog: None,
            cwd: cwd.map(|dir| dir.to_str().expect("utf-8").to_owned()),
            statusline_command: None,
            agent_session_id: None,
        }))
    }

    #[test]
    fn preflight_order_fourth_reading_takes_only_a_snapshot_whose_cwd_is_a_directory() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let file = tmp.path().join("a-file");
        std::fs::write(&file, b"x").expect("file");
        assert_eq!(
            recorded_dir(&snapshot(Some(tmp.path()))),
            Some(tmp.path().to_path_buf())
        );
        assert_eq!(
            recorded_dir(&snapshot(Some(&tmp.path().join("gone")))),
            None
        );
        assert_eq!(recorded_dir(&snapshot(Some(&file))), None);
        assert_eq!(recorded_dir(&snapshot(None)), None);
        assert_eq!(recorded_dir(&Recovered::Absent), None);
        let replayed = Recovered::Replayed {
            cause: ReplayCause::Unreadable,
            state: Replayed::default(),
        };
        assert_eq!(recorded_dir(&replayed), None);
    }

    #[rstest]
    #[case::strict_modes(
        Refusal::StrictModes,
        "strict-modes-failed",
        "builder's state files can be written by another user",
        "viola will not read them; make the viola home and its files owner-only"
    )]
    #[case::no_session(
        Refusal::NoSession,
        "no-session",
        "builder has no logged session to resume",
        "start one: viola run builder -- claude"
    )]
    #[case::unknown_id(
        Refusal::UnknownId,
        "no-session",
        "builder has no logged session with that id",
        "viola revive builder --list"
    )]
    #[case::session_live(
        Refusal::SessionLive,
        "session-live",
        "builder's session is open in another claude",
        "close it there first, or start a copy: viola revive builder --fork"
    )]
    #[case::cwd_missing(
        Refusal::CwdMissing,
        "cwd-missing",
        "builder's recorded directory is missing",
        "resume it by hand from a directory you choose: viola run builder -- claude --resume <id>, ids from viola revive builder --list"
    )]
    fn preflight_order_refusals_have_their_fixed_pair_and_closed_detail(
        #[case] refusal: Refusal,
        #[case] detail: &str,
        #[case] unable: &str,
        #[case] hint: &str,
    ) {
        assert_eq!(refusal.detail(), detail);
        assert_eq!(
            refusal.pair("builder"),
            (unable.to_owned(), hint.to_owned())
        );
        assert!(!hint.contains("release"));
    }

    #[test]
    fn list_rows_are_ts_cause_and_id_two_spaces_apart_in_log_order() {
        let mut odd = link("later-cause", SECOND);
        odd.ts = "t\u{1b}[31m".to_owned();
        let rows = list_rows(&[
            link("startup", FIRST),
            link("clear", "s-2"),
            odd,
            link("resume", FIRST),
            link("compact", SECOND),
        ]);
        assert_eq!(
            rows[1],
            ListRow {
                ts: "t\\x1B[31m".to_owned(),
                cause: "unknown",
                id: SECOND.to_owned(),
            }
        );
        let lines: Vec<String> = rows.iter().map(ListRow::line).collect();
        assert_eq!(
            lines,
            [
                format!("2026-10-10T01:02:03.000Z  startup  {FIRST}"),
                format!("t\\x1B[31m  unknown  {SECOND}"),
                format!("2026-10-10T01:02:03.000Z  resume  {FIRST}"),
                format!("2026-10-10T01:02:03.000Z  compact  {SECOND}"),
            ]
        );
        assert_eq!(cause_word("clear"), "clear");
        assert!(list_rows(&[]).is_empty());
    }

    #[derive(clap::Parser)]
    struct Line {
        #[command(flatten)]
        args: ReviveArgs,
    }

    fn parse(words: &[&str]) -> Result<ReviveArgs, clap::error::ErrorKind> {
        Line::try_parse_from(std::iter::once("revive").chain(words.iter().copied()))
            .map(|line| line.args)
            .map_err(|error| error.kind())
    }

    #[test]
    fn revive_args_take_a_name_an_id_the_fork_flag_and_the_words_after_the_dashes() {
        let args =
            parse(&["builder", "--id", FIRST, "--fork", "--", "--model", "x"]).expect("parsed");
        assert_eq!(AsRef::<str>::as_ref(&args.name), "builder");
        assert_eq!(args.id.as_deref(), Some(FIRST));
        assert!(args.fork && !args.list);
        assert_eq!(args.extra, [OsString::from("--model"), OsString::from("x")]);
        let bare = parse(&["builder"]).expect("parsed");
        assert!(bare.id.is_none() && !bare.fork && !bare.list && bare.extra.is_empty());
        assert!(parse(&["builder", "--list"]).expect("parsed").list);
    }

    #[rstest]
    #[case::an_id_of_another_shape(&["builder", "--id", "s-1"], clap::error::ErrorKind::ValueValidation)]
    #[case::an_id_that_is_an_option_word(
        &["builder", "--id=--dangerously-skip-permissions-00000"],
        clap::error::ErrorKind::ValueValidation
    )]
    #[case::a_name_that_is_no_name(&["Not-A-Name"], clap::error::ErrorKind::ValueValidation)]
    #[case::list_beside_id(&["builder", "--list", "--id", FIRST], clap::error::ErrorKind::ArgumentConflict)]
    #[case::list_beside_fork(&["builder", "--list", "--fork"], clap::error::ErrorKind::ArgumentConflict)]
    #[case::list_beside_child_words(
        &["builder", "--list", "--", "x"],
        clap::error::ErrorKind::ArgumentConflict
    )]
    #[case::no_name(&[], clap::error::ErrorKind::MissingRequiredArgument)]
    fn revive_args_refuse_as_usage(#[case] words: &[&str], #[case] kind: clap::error::ErrorKind) {
        assert_eq!(parse(words).err(), Some(kind));
    }
}
