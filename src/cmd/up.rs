//! `viola up [<name>…] [--dry-run] [--json]`: the fleet re-opened after a reboot. Every instance of
//! the home whose wrapper is gone, whose heartbeat is older than this boot and whose log holds no
//! `session-end` after its newest `session-start` is resumed in place by the pinned `viola revive`,
//! each in a `foot` terminal of its own, parents before the instances under them; a named instance
//! is taken whatever its log's end reads. A live or stale instance is skipped. One line per instance
//! names its outcome in a closed word. `up` writes no snapshot, event line or stamp of its own: each
//! re-opened instance's records are its revive's.

use std::collections::BTreeMap;
use std::ffi::{OsStr, OsString};
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::SystemTime;

use serde_json::{Value, json};
use tracing::instrument;
use viola_agent_claude::is_session_id;
use viola_core::{UnreachableCause, ViolaName};
use viola_state::board::{self, ScanRefused};
use viola_state::heartbeat::beat_age;
use viola_state::liveness::{
    Liveness, STALE_AFTER, beat_before, boot_time, classify, same_process,
};
use viola_state::pin::{PinError, pin_bytes, pin_key};
use viola_state::replay::{
    Recovered, ended_after_last_start, read_snapshot_or_replay, session_chain,
};
use viola_state::strict::check_home_tree;

use super::client::{self, own_name};
use super::open::{TERMINAL, came_up, recorded_window, refused, spawn_terminal};
use super::run::refuse_tampered_pin;
use crate::{human, run};

#[derive(clap::Args)]
pub(crate) struct UpArgs {
    /// Instances to re-open whatever their log's end reads; else every one the reboot left
    #[arg(value_name = "NAME", value_parser = super::send::parse_name)]
    names: Vec<ViolaName>,
    /// Print what would be opened, in order, and open nothing
    #[arg(long)]
    dry_run: bool,
    /// Print one JSON document on stdout instead of the lines
    #[arg(long)]
    json: bool,
}

/// One instance's outcome: its closed word on the line and in the document.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Outcome {
    /// Opened and live, with the placement word its new window records.
    Live(String),
    AlreadyLive,
    StaleHeartbeat,
    WouldOpen,
    NoSession,
    CwdMissing,
    SessionLive,
    NotLive,
    NotTaken,
}

impl Outcome {
    const fn word(&self) -> &'static str {
        match self {
            Self::Live(_) => "live",
            Self::AlreadyLive => "already-live",
            Self::StaleHeartbeat => "stale-heartbeat",
            Self::WouldOpen => "would-open",
            Self::NoSession => "no-session",
            Self::CwdMissing => "cwd-missing",
            Self::SessionLive => "session-live",
            Self::NotLive => "not-live",
            Self::NotTaken => "not-taken",
        }
    }

    fn placement(&self) -> Option<&str> {
        match self {
            Self::Live(placement) => Some(placement),
            _ => None,
        }
    }

    /// Whether this outcome leaves the exit at 0.
    const fn fine(&self) -> bool {
        matches!(
            self,
            Self::Live(_) | Self::AlreadyLive | Self::StaleHeartbeat | Self::WouldOpen
        )
    }
}

/// What one scanned instance read, for the selection.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Reading {
    liveness: Liveness,
    /// The heartbeat was last touched before this boot.
    beat_before: bool,
    /// The log holds a `session-end` after its newest `session-start`.
    ended: bool,
}

/// What the selection makes of one instance.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Pick {
    Skip(Outcome),
    Candidate,
    /// Named, and not taken: a line of its own.
    NotTaken,
    /// Not named, and not taken: no line.
    Out,
}

/// The selection rule: a live or stale instance is skipped before anything else is read; a gone
/// one is taken when its heartbeat is older than this boot and, unless it was named, its log does
/// not end in a `session-end` after its newest `session-start`.
fn pick(reading: &Reading, named: bool) -> Pick {
    match reading.liveness {
        Liveness::Live => return Pick::Skip(Outcome::AlreadyLive),
        Liveness::Stale => return Pick::Skip(Outcome::StaleHeartbeat),
        Liveness::Gone => {}
    }
    let taken = if named {
        reading.beat_before
    } else {
        reading.beat_before && !reading.ended
    };
    match (taken, named) {
        (true, _) => Pick::Candidate,
        (false, true) => Pick::NotTaken,
        (false, false) => Pick::Out,
    }
}

/// The candidates' indices, parents first: a candidate whose parent names no other candidate is a
/// root, roots and siblings in the order given (name order), each followed by its children depth
/// first; a candidate a parent cycle keeps from every root is taken once, from its first name.
fn parents_first(parents: &[Option<&str>], names: &[&str]) -> Vec<usize> {
    let by_name: BTreeMap<&str, usize> = names.iter().enumerate().map(|(at, n)| (*n, at)).collect();
    let parent_of = |at: usize| {
        parents[at]
            .and_then(|parent| by_name.get(parent).copied())
            .filter(|&parent| parent != at)
    };
    let mut children: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for at in 0..names.len() {
        if let Some(parent) = parent_of(at) {
            children.entry(parent).or_default().push(at);
        }
    }
    let mut taken = vec![false; names.len()];
    let mut order = Vec::with_capacity(names.len());
    let roots = (0..names.len()).filter(|&at| parent_of(at).is_none());
    for root in roots.chain(0..names.len()) {
        let mut stack = vec![root];
        while let Some(at) = stack.pop() {
            if taken[at] {
                continue;
            }
            taken[at] = true;
            order.push(at);
            if let Some(kids) = children.get(&at) {
                stack.extend(kids.iter().rev().copied());
            }
        }
    }
    order
}

/// One instance the selection took, with what its terminal needs.
struct Candidate {
    name: ViolaName,
    dir: PathBuf,
    parent: Option<String>,
    cwd: Option<PathBuf>,
    id: Option<String>,
}

pub(crate) fn up(home: &Path, args: &UpArgs) -> anyhow::Result<ExitCode> {
    let home = std::path::absolute(home)?;
    // Before the log opens: opening it narrows the home to its owner, which would erase the very
    // state this check refuses. A refused scan opens nothing under the home and writes no line.
    let scan = match board::scan(&home)? {
        Ok(scan) => scan,
        Err(refused) => {
            refuse_board(args.json, &refused);
            return Ok(ExitCode::from(21));
        }
    };
    if let Some(name) = own_name() {
        client::start(&home, &name)?;
    }
    // The log's open may have created the home: from here on its one canonical spelling.
    let home = super::canonical_home(&home);
    upped(&home, &scan, args)
}

/// Exit 21's output: the one `--json` document, or the pair for what failed.
fn refuse_board(json: bool, refused: &ScanRefused) {
    if json {
        client::unreachable_document(UnreachableCause::StrictModes);
        return;
    }
    match refused {
        ScanRefused::Home => {
            let (unable, hint) = human::home_state_pair();
            human::refuse(unable, hint);
        }
        ScanRefused::Instance(name) => {
            let (unable, hint) = human::instance_state_pair(name.as_ref());
            human::refuse(&unable, &hint);
        }
    }
}

#[instrument(skip_all, name = "cli.up", fields(dry_run = args.dry_run))]
fn upped(home: &Path, scan: &board::Scan, args: &UpArgs) -> anyhow::Result<ExitCode> {
    let Some(boot) = boot_time() else {
        return Ok(refused(args.json, "boot-time-unread", || {
            let (unable, hint) = human::boot_time_unread_pair();
            human::refuse(unable, hint);
        }));
    };
    let (mut lines, candidates, order) = plan(home, scan, args, boot);
    if args.dry_run || order.is_empty() {
        for at in order {
            lines.push((candidates[at].name.as_ref().to_owned(), Outcome::WouldOpen));
        }
        return Ok(finish(&lines, args.json));
    }
    act(home, args, &candidates, order, lines)
}

/// The board read: each scanned instance's pick, then the named instances the scan did not hold.
fn select(
    scan: &board::Scan,
    args: &UpArgs,
    boot: SystemTime,
) -> (Vec<(String, Outcome)>, Vec<Candidate>) {
    let named = |name: &ViolaName| args.names.contains(name);
    let mut lines: Vec<(String, Outcome)> = Vec::new();
    let mut candidates: Vec<Candidate> = Vec::new();
    for (name, dir) in scan.instances() {
        if !args.names.is_empty() && !named(name) {
            continue;
        }
        let recovered = read_snapshot_or_replay(dir).ok();
        let reading = Reading {
            liveness: liveness_of(dir, recovered.as_ref()),
            beat_before: beat_before(dir, boot),
            ended: ended_after_last_start(dir).unwrap_or(false),
        };
        match pick(&reading, named(name)) {
            Pick::Skip(outcome) => lines.push((name.as_ref().to_owned(), outcome)),
            Pick::NotTaken => lines.push((name.as_ref().to_owned(), Outcome::NotTaken)),
            Pick::Out => {}
            Pick::Candidate => candidates.push(Candidate {
                name: name.clone(),
                dir: dir.clone(),
                parent: recorded_parent(recovered.as_ref()),
                cwd: recorded_cwd(recovered.as_ref()),
                id: newest_id(dir),
            }),
        }
    }
    let scanned: Vec<&ViolaName> = scan.instances().iter().map(|(name, _)| name).collect();
    for name in &args.names {
        if !scanned.contains(&name) && !lines.iter().any(|(n, _)| n == name.as_ref() as &str) {
            lines.push((name.as_ref().to_owned(), Outcome::NotTaken));
        }
    }
    (lines, candidates)
}

/// The session reads and the order: the candidates with no session, no directory or a session
/// live elsewhere leave with their line, then the lines sort and the rest go parents first.
fn plan(
    home: &Path,
    scan: &board::Scan,
    args: &UpArgs,
    boot: SystemTime,
) -> (Vec<(String, Outcome)>, Vec<Candidate>, Vec<usize>) {
    let (mut lines, mut candidates) = select(scan, args, boot);
    candidates.retain(|candidate| match (&candidate.id, &candidate.cwd) {
        (None, _) => {
            lines.push((candidate.name.as_ref().to_owned(), Outcome::NoSession));
            false
        }
        (Some(_), None) => {
            lines.push((candidate.name.as_ref().to_owned(), Outcome::CwdMissing));
            false
        }
        (Some(_), Some(_)) => true,
    });
    if !candidates.is_empty() {
        let listed = listed_ids(home);
        candidates.retain(|candidate| {
            let live = candidate
                .id
                .as_ref()
                .is_some_and(|id| listed.iter().any(|row| row == id));
            if live {
                lines.push((candidate.name.as_ref().to_owned(), Outcome::SessionLive));
            }
            !live
        });
    }
    lines.sort_by(|(a, _), (b, _)| a.cmp(b));
    let names: Vec<&str> = candidates.iter().map(|c| c.name.as_ref() as &str).collect();
    let parents: Vec<Option<&str>> = candidates.iter().map(|c| c.parent.as_deref()).collect();
    let order = parents_first(&parents, &names);
    (lines, candidates, order)
}

/// The spawns: the home tree checked, the binary pinned and the terminal resolved, then one
/// pinned `viola revive` per candidate in order, each line its outcome.
fn act(
    home: &Path,
    args: &UpArgs,
    candidates: &[Candidate],
    order: Vec<usize>,
    mut lines: Vec<(String, Outcome)>,
) -> anyhow::Result<ExitCode> {
    let exe = std::fs::read(std::env::current_exe()?)?;
    if check_home_tree(home, None, Some(&pin_key(&exe))).is_err() {
        return Ok(refused(args.json, "strict-modes-failed", || {
            let (unable, hint) = human::home_state_pair();
            human::refuse(unable, hint);
        }));
    }
    let pinned = match pin_bytes(home, &exe) {
        Ok(pinned) => pinned,
        Err(PinError::HashMismatch) => {
            return Ok(refused(
                args.json,
                "pinned-hash-mismatch",
                refuse_tampered_pin,
            ));
        }
        Err(error) => return Err(error.into()),
    };
    let here = std::env::current_dir()?;
    let Ok(foot) = run::resolve_program(OsStr::new(TERMINAL), &here) else {
        return Ok(refused(args.json, "terminal-not-found", || {
            let (unable, hint) = human::terminal_not_found_pair();
            human::refuse(unable, hint);
        }));
    };
    let origin = if own_name().is_some() { "agent" } else { "cli" };
    for at in order {
        let candidate = &candidates[at];
        let cwd = candidate.cwd.as_deref().unwrap_or(&here);
        let argv = up_argv(&candidate.name, cwd, &pinned.path, home, origin);
        // A failed start is that instance's `not-live` and the next one still starts.
        let live =
            spawn_terminal(&foot, &argv).is_ok_and(|terminal| came_up(&candidate.dir, terminal));
        let outcome = if live {
            Outcome::Live(
                recorded_window(&candidate.dir)
                    .map_or_else(|| "unknown".to_owned(), |window| window.placement),
            )
        } else {
            Outcome::NotLive
        };
        lines.push((candidate.name.as_ref().to_owned(), outcome));
    }
    Ok(finish(&lines, args.json))
}

/// The lines, or the one document, then the hint and the exit.
fn finish(lines: &[(String, Outcome)], json: bool) -> ExitCode {
    if json {
        let instances: Vec<Value> = lines
            .iter()
            .map(|(name, outcome)| {
                json!({"name": name, "outcome": outcome.word(), "placement": outcome.placement()})
            })
            .collect();
        client::document(&json!({"v": 1, "ok": {"instances": instances}}));
    } else {
        let mut out = io::stdout().lock();
        for (name, outcome) in lines {
            let _ = human::write_up_line(&mut out, name, outcome.word(), outcome.placement());
        }
    }
    let not_live = lines.iter().any(|(_, o)| *o == Outcome::NotLive);
    if not_live && !json {
        let _ = human::write_hint(&mut io::stderr().lock(), human::UP_NOT_LIVE_HINT);
    }
    if lines.iter().all(|(_, outcome)| outcome.fine()) {
        run::log_self_exit(0, None);
        ExitCode::SUCCESS
    } else {
        run::log_self_exit(1, not_live.then_some("not-live"));
        ExitCode::from(1)
    }
}

/// The instance's liveness from its one snapshot read: a real snapshot by pid, start time and beat;
/// a replayed one by its beat alone, `live` within `STALE_AFTER` and else gone; an absent one gone.
fn liveness_of(dir: &Path, recovered: Option<&Recovered>) -> Liveness {
    let age = || beat_age(dir, SystemTime::now());
    match recovered {
        Some(Recovered::Snapshot(snapshot)) => classify(age(), same_process(snapshot)),
        Some(Recovered::Replayed { .. }) if age().is_some_and(|age| age <= STALE_AFTER) => {
            Liveness::Live
        }
        _ => Liveness::Gone,
    }
}

/// The parent the snapshot or the replay records, read only to order the re-opens.
fn recorded_parent(recovered: Option<&Recovered>) -> Option<String> {
    let lineage = match recovered? {
        Recovered::Snapshot(snapshot) => snapshot.lineage.as_ref(),
        Recovered::Replayed { state, .. } => state.lineage.as_ref(),
        Recovered::Absent => None,
    };
    lineage?.parent.clone()
}

/// The directory a real snapshot records, while it is still a directory.
fn recorded_cwd(recovered: Option<&Recovered>) -> Option<PathBuf> {
    match recovered? {
        Recovered::Snapshot(snapshot) => snapshot
            .cwd
            .as_deref()
            .map(PathBuf::from)
            .filter(|dir| dir.is_dir()),
        Recovered::Absent | Recovered::Replayed { .. } => None,
    }
}

/// The newest logged session id of the session-id shape.
fn newest_id(dir: &Path) -> Option<String> {
    session_chain(dir)
        .ok()?
        .links
        .into_iter()
        .map(|link| link.agent_session_id)
        .rfind(|id| is_session_id(id))
}

/// The session ids one `claude agents --json` read lists; a failed read lists none.
fn listed_ids(home: &Path) -> Vec<String> {
    std::env::current_dir()
        .ok()
        .and_then(|cwd| run::agents::read_here(home, &cwd).ok())
        .map(|list| list.rows.into_iter().map(|row| row.session_id).collect())
        .unwrap_or_default()
}

/// `foot`'s argv, closed: its three options, then the pinned `viola revive` of the one instance
/// under `--origin`, each value one element.
fn up_argv(
    name: &ViolaName,
    cwd: &Path,
    pinned: &Path,
    home: &Path,
    origin: &str,
) -> Vec<OsString> {
    let name: &str = name.as_ref();
    let mut working = OsString::from("--working-directory=");
    working.push(cwd.as_os_str());
    vec![
        format!("--app-id=viola.{name}").into(),
        format!("--title={name}").into(),
        working,
        pinned.into(),
        "--home".into(),
        home.into(),
        "revive".into(),
        name.into(),
        "--origin".into(),
        origin.into(),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn reading(liveness: Liveness, beat_before: bool, ended: bool) -> Reading {
        Reading {
            liveness,
            beat_before,
            ended,
        }
    }

    #[rstest]
    #[case::live_with_an_old_beat(Liveness::Live, true, false, Outcome::AlreadyLive)]
    #[case::live(Liveness::Live, false, false, Outcome::AlreadyLive)]
    #[case::live_and_ended(Liveness::Live, true, true, Outcome::AlreadyLive)]
    #[case::stale_with_an_old_beat(Liveness::Stale, true, false, Outcome::StaleHeartbeat)]
    #[case::stale(Liveness::Stale, false, true, Outcome::StaleHeartbeat)]
    fn up_plan_skips_a_live_or_stale_instance(
        #[case] liveness: Liveness,
        #[case] beat_before: bool,
        #[case] ended: bool,
        #[case] outcome: Outcome,
    ) {
        let read = reading(liveness, beat_before, ended);
        assert_eq!(pick(&read, false), Pick::Skip(outcome.clone()));
        assert_eq!(pick(&read, true), Pick::Skip(outcome));
    }

    #[rstest]
    #[case::unnamed_before_the_boot(false, true, Pick::Candidate)]
    #[case::unnamed_this_boot(false, false, Pick::Out)]
    #[case::named_before_the_boot(true, true, Pick::Candidate)]
    #[case::named_this_boot(true, false, Pick::NotTaken)]
    fn up_plan_takes_only_a_beat_older_than_the_boot(
        #[case] named: bool,
        #[case] beat_before: bool,
        #[case] want: Pick,
    ) {
        assert_eq!(
            pick(&reading(Liveness::Gone, beat_before, false), named),
            want
        );
    }

    #[rstest]
    #[case::unnamed_ended(false, true, Pick::Out)]
    #[case::unnamed_not_ended(false, false, Pick::Candidate)]
    #[case::named_ended(true, true, Pick::Candidate)]
    #[case::named_not_ended(true, false, Pick::Candidate)]
    fn up_plan_leaves_out_an_instance_ended_after_its_last_start(
        #[case] named: bool,
        #[case] ended: bool,
        #[case] want: Pick,
    ) {
        assert_eq!(pick(&reading(Liveness::Gone, true, ended), named), want);
    }

    fn ordered(rows: &[(&'static str, Option<&'static str>)]) -> Vec<&'static str> {
        let names: Vec<&str> = rows.iter().map(|(name, _)| *name).collect();
        let parents: Vec<Option<&str>> = rows.iter().map(|(_, parent)| *parent).collect();
        parents_first(&parents, &names)
            .into_iter()
            .map(|at| names[at])
            .collect()
    }

    #[rstest]
    #[case::a_child_sorted_first(&[("a-child", Some("b-parent")), ("b-parent", None)], &["b-parent", "a-child"])]
    #[case::three_deep(
        &[("a", Some("b")), ("b", Some("c")), ("c", None)],
        &["c", "b", "a"]
    )]
    #[case::siblings_in_name_order(
        &[("a", Some("p")), ("b", Some("p")), ("p", None), ("q", None)],
        &["p", "a", "b", "q"]
    )]
    #[case::a_missing_parent_is_a_root(&[("a", Some("gone")), ("b", None)], &["a", "b"])]
    #[case::a_self_parent_is_a_root(&[("a", Some("a")), ("b", Some("a"))], &["a", "b"])]
    #[case::a_cycle_once_from_its_first_name(
        &[("a", Some("b")), ("b", Some("a")), ("c", None)],
        &["c", "a", "b"]
    )]
    #[case::none(&[], &[])]
    fn up_plan_orders_parents_before_children(
        #[case] rows: &[(&'static str, Option<&'static str>)],
        #[case] want: &[&str],
    ) {
        assert_eq!(ordered(rows), want);
    }

    #[test]
    fn up_plan_words_are_the_closed_set() {
        let all = [
            (Outcome::Live("placed".to_owned()), "live", true),
            (Outcome::AlreadyLive, "already-live", true),
            (Outcome::StaleHeartbeat, "stale-heartbeat", true),
            (Outcome::WouldOpen, "would-open", true),
            (Outcome::NoSession, "no-session", false),
            (Outcome::CwdMissing, "cwd-missing", false),
            (Outcome::SessionLive, "session-live", false),
            (Outcome::NotLive, "not-live", false),
            (Outcome::NotTaken, "not-taken", false),
        ];
        for (outcome, word, fine) in all {
            assert_eq!((outcome.word(), outcome.fine()), (word, fine), "{word}");
            assert!(viola_core::is_code_word(word), "{word}");
        }
        assert_eq!(
            Outcome::Live("placed".to_owned()).placement(),
            Some("placed")
        );
        assert_eq!(Outcome::NotLive.placement(), None);
    }

    #[test]
    fn up_plan_argv_is_the_closed_argv_one_element_each() {
        let name = ViolaName::try_new("child".to_owned()).expect("valid");
        let argv = up_argv(
            &name,
            Path::new("/w/a dir 'q'"),
            Path::new("/h/bin/k/viola"),
            Path::new("/h ome"),
            "cli",
        );
        let want: Vec<OsString> = [
            "--app-id=viola.child",
            "--title=child",
            "--working-directory=/w/a dir 'q'",
            "/h/bin/k/viola",
            "--home",
            "/h ome",
            "revive",
            "child",
            "--origin",
            "cli",
        ]
        .map(OsString::from)
        .into();
        assert_eq!(argv, want);
    }

    #[derive(clap::Parser)]
    struct Line {
        #[command(flatten)]
        args: UpArgs,
    }

    fn parse(words: &[&str]) -> Result<UpArgs, clap::error::ErrorKind> {
        use clap::Parser as _;
        Line::try_parse_from(std::iter::once("up").chain(words.iter().copied()))
            .map(|line| line.args)
            .map_err(|error| error.kind())
    }

    #[test]
    fn up_plan_args_take_names_dry_run_and_json() {
        let args = parse(&["a", "b-2", "--dry-run", "--json"]).expect("args");
        let names: Vec<&str> = args.names.iter().map(|n| n.as_ref() as &str).collect();
        assert_eq!(names, ["a", "b-2"]);
        assert!(args.dry_run && args.json);
        let bare = parse(&[]).expect("bare");
        assert!(bare.names.is_empty() && !bare.dry_run && !bare.json);
        assert_eq!(
            parse(&["Not-A-Name"]).err(),
            Some(clap::error::ErrorKind::ValueValidation)
        );
        assert_eq!(
            parse(&["--origin", "cli"]).err(),
            Some(clap::error::ErrorKind::UnknownArgument)
        );
    }
}
