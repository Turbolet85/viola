//! `viola run`'s version gate (architecture §Established Decisions → [CLI Version Compatibility]):
//! the resolved child answers `--version` once, and only a version whose ledger stamp reads every
//! row `pass` makes the instance `cli_verified`. Every other outcome degrades silently to
//! transport-only; `run` prints nothing about it.

use std::ffi::OsString;
use std::io::Read;
use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use tracing::field::Empty;
use tracing::instrument;
use viola_agent_claude::StripPlan;
use viola_agent_claude::ledger::{parse_version, verified};
use viola_core::obs::ObsEvent;
use viola_core::{MAX_FRAME, obs_event};
use viola_state::stamps::{StampsReadError, read_stamps_strict};

/// The bound on a `--version` answer, `verify`'s and the gate's.
pub(crate) const VERSION_DEADLINE: Duration = Duration::from_secs(5);
const POLL: Duration = Duration::from_millis(10);
/// How long the output may take to arrive after the child ended: a grandchild still holding the
/// pipe is not waited for.
const DRAIN_GRACE: Duration = Duration::from_millis(500);

/// A bounded child's exit (`None` when the deadline killed it) and its stdout.
#[derive(Default)]
pub(crate) struct Bounded {
    pub(crate) status: Option<ExitStatus>,
    pub(crate) stdout: Vec<u8>,
}

/// `program args`, spawned directly (never through a shell) in `cwd` with stdin null and the R8
/// strip applied; stdout and stderr drained through the frame cap, stdout kept; killed at
/// `deadline`. Upstream output is content: it is returned, never printed.
pub(crate) fn run_bounded(
    program: &Path,
    args: &[OsString],
    cwd: &Path,
    strip: &StripPlan,
    deadline: Duration,
) -> std::io::Result<Bounded> {
    let mut command = Command::new(program);
    command
        .args(args)
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for name in &strip.remove {
        command.env_remove(name);
    }
    let mut child = command.spawn()?;
    let stdout = drain(child.stdout.take());
    let _stderr = drain(child.stderr.take());
    let until = Instant::now() + deadline;
    let status = loop {
        if let Some(status) = child.try_wait()? {
            break Some(status);
        }
        if Instant::now() >= until {
            let _ = child.kill();
            let _ = child.wait();
            break None;
        }
        std::thread::sleep(POLL);
    };
    let stdout = stdout.bytes(DRAIN_GRACE);
    Ok(Bounded { status, stdout })
}

/// A pipe being read to its end on its own thread; `Default` is a pipe that yields nothing.
#[derive(Default)]
struct Drained(Option<mpsc::Receiver<Vec<u8>>>);

impl Drained {
    /// The bytes read, waiting at most `grace` for the reader to finish.
    fn bytes(self, grace: Duration) -> Vec<u8> {
        self.0
            .and_then(|rx| rx.recv_timeout(grace).ok())
            .unwrap_or_default()
    }
}

fn drain(pipe: Option<impl Read + Send + 'static>) -> Drained {
    Drained(pipe.map(|pipe| {
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let mut bytes = Vec::new();
            let _ = pipe.take(MAX_FRAME).read_to_end(&mut bytes);
            let _ = tx.send(bytes);
        });
        rx
    }))
}

/// What the gate measured for the snapshot and the `claude-child` start line.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub(crate) struct Gate {
    pub(crate) cli_version: Option<String>,
    pub(crate) cli_verified: bool,
}

/// `program --version` (no user arguments), then the stamps read for the version it names.
#[instrument(
    skip_all,
    name = "run.version_gate",
    fields(cli_version = Empty, cli_verified = Empty)
)]
pub(crate) fn version_gate(home: &Path, program: &Path, cwd: &Path, strip: &StripPlan) -> Gate {
    let started = Instant::now();
    obs_event!(INFO, ObsEvent::ProcessStart, subject = "version-probe");
    let answer = run_bounded(
        program,
        &[OsString::from("--version")],
        cwd,
        strip,
        VERSION_DEADLINE,
    );
    let status = answer
        .as_ref()
        .ok()
        .and_then(|a| a.status)
        .and_then(|s| s.code());
    obs_event!(
        INFO,
        ObsEvent::ProcessExit,
        subject = "version-probe",
        child_exit_status = status,
        duration_ms = u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
    );
    let cli_version = answer.ok().and_then(|a| parse_version(&a.stdout));
    let cli_verified = cli_version.as_deref().is_some_and(|version| {
        let (verified, detail) = stamps_verdict(read_stamps_strict(home), version);
        if let Some(detail) = detail {
            obs_event!(
                WARN,
                ObsEvent::ParseRejected,
                parser = "ledger-stamps",
                detail = detail,
                count = 1u64,
            );
        }
        verified
    });
    let span = tracing::Span::current();
    if let Some(version) = &cli_version {
        span.record("cli_version", version.as_str());
    }
    span.record("cli_verified", cli_verified);
    Gate {
        cli_version,
        cli_verified,
    }
}

/// The stamps read's verdict for `version`, with the `parse-rejected` detail code when the file
/// could not be used: absent is simply unverified, and a ledger another user could write is never
/// read at all (it fails closed, never open).
pub(crate) fn stamps_verdict(
    read: Result<Option<Vec<u8>>, StampsReadError>,
    version: &str,
) -> (bool, Option<&'static str>) {
    match read {
        Ok(None) => (false, None),
        Err(StampsReadError::StrictModes(_)) => (false, Some("strict-modes-failed")),
        Err(StampsReadError::State(_)) => (false, Some("unreadable")),
        Ok(Some(bytes)) => match verified(&bytes, version) {
            Ok(verified) => (verified, None),
            Err(_) => (false, Some("malformed")),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use viola_agent_claude::ledger::{LedgerRow, TypedRun, merge_stamp};

    fn stamp(version: &str, pass: bool) -> Vec<u8> {
        let results: Vec<(LedgerRow, bool)> = LedgerRow::ALL.iter().map(|r| (*r, pass)).collect();
        merge_stamp(
            None,
            version,
            &results,
            &[],
            &TypedRun::default(),
            None,
            "2026-09-28T10:00:00.000Z",
        )
    }

    #[test]
    fn stamps_verdict_reads_every_arm() {
        assert_eq!(stamps_verdict(Ok(None), "2.1.0"), (false, None));
        let io = viola_state::StateError::Io(std::io::Error::other("C:/secret"));
        assert_eq!(
            stamps_verdict(Err(StampsReadError::State(io)), "2.1.0"),
            (false, Some("unreadable"))
        );
        let refused = StampsReadError::StrictModes(viola_state::strict::Refused::Writable);
        assert_eq!(
            stamps_verdict(Err(refused), "2.1.0"),
            (false, Some("strict-modes-failed"))
        );
        assert_eq!(
            stamps_verdict(Ok(Some(b"{not json".to_vec())), "2.1.0"),
            (false, Some("malformed"))
        );
        assert_eq!(
            stamps_verdict(Ok(Some(stamp("2.1.0", true))), "2.1.0"),
            (true, None)
        );
        assert_eq!(
            stamps_verdict(Ok(Some(stamp("3.0.0", true))), "2.1.0"),
            (false, None)
        );
        assert_eq!(
            stamps_verdict(Ok(Some(stamp("2.1.0", false))), "2.1.0"),
            (false, None)
        );
    }

    fn stamp_of(results: &[(LedgerRow, bool)]) -> Vec<u8> {
        merge_stamp(
            None,
            "2.1.0",
            results,
            &[],
            &TypedRun::default(),
            None,
            "2026-10-05T12:00:00.000Z",
        )
    }

    /// The verdict needs the dialog rows (R2 closed) and the framing rows: a stamp written before
    /// either landed (the ten spine and screen rows, or those and the four dialog rows) and one
    /// whose dialog or framing row failed all leave the version unverified; all seventeen passing
    /// verifies it.
    #[test]
    fn stamps_verdict_needs_the_dialog_rows() {
        let ten = [
            LedgerRow::ShimResolution,
            LedgerRow::SpineHooks,
            LedgerRow::SessionStartFields,
            LedgerRow::PromptVerbatim,
            LedgerRow::StopMessage,
            LedgerRow::LargestHookPayload,
            LedgerRow::ModalSignature,
            LedgerRow::InputBoxSignature,
            LedgerRow::QuietPeriod,
            LedgerRow::ConfirmWindow,
        ];
        let spine: Vec<(LedgerRow, bool)> = ten.iter().map(|r| (*r, true)).collect();
        assert_eq!(
            stamps_verdict(Ok(Some(stamp_of(&spine))), "2.1.0"),
            (false, None)
        );
        let dialogs = [
            LedgerRow::QuestionAnswer,
            LedgerRow::PlanApproveRevise,
            LedgerRow::QuestionNotes,
            LedgerRow::DialogConcurrency,
        ];
        let framing = [
            LedgerRow::LongPasteWrapper,
            LedgerRow::TagEscaping,
            LedgerRow::LocalCommandClear,
        ];
        let mut all = spine.clone();
        all.extend(dialogs.iter().map(|r| (*r, true)));
        assert_eq!(
            stamps_verdict(Ok(Some(stamp_of(&all))), "2.1.0"),
            (false, None),
            "the fourteen rows of an older stamp"
        );
        all.extend(framing.iter().map(|r| (*r, true)));
        assert_eq!(
            stamps_verdict(Ok(Some(stamp_of(&all))), "2.1.0"),
            (true, None)
        );
        for failing in dialogs.into_iter().chain(framing) {
            let rows: Vec<(LedgerRow, bool)> =
                all.iter().map(|(r, p)| (*r, *p && *r != failing)).collect();
            assert_eq!(
                stamps_verdict(Ok(Some(stamp_of(&rows))), "2.1.0"),
                (false, None),
                "{}",
                failing.id()
            );
        }
    }

    /// The test binary lists its tests and exits: its stdout comes back whole.
    #[test]
    fn run_bounded_returns_the_exit_and_stdout() {
        let exe = std::env::current_exe().expect("test exe");
        let cwd = std::env::current_dir().expect("cwd");
        let got = run_bounded(
            &exe,
            &[OsString::from("--list")],
            &cwd,
            &StripPlan::default(),
            VERSION_DEADLINE,
        )
        .expect("spawned");
        assert!(got.status.is_some_and(|s| s.success()));
        let listed = String::from_utf8_lossy(&got.stdout);
        assert!(
            listed.contains("run_bounded_returns_the_exit_and_stdout"),
            "{listed}"
        );
    }

    #[test]
    fn run_bounded_reports_a_program_that_cannot_start() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let missing = tmp.path().join("absent-program");
        let got = run_bounded(
            &missing,
            &[],
            tmp.path(),
            &StripPlan::default(),
            VERSION_DEADLINE,
        );
        assert!(got.is_err());
    }

    /// A child that does not answer `X.Y.Z (Claude Code)` leaves the instance unversioned and
    /// unverified, and the stamps are never read.
    #[test]
    fn version_gate_degrades_for_a_child_that_is_not_the_cli() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let exe = std::env::current_exe().expect("test exe");
        let gate = version_gate(tmp.path(), &exe, tmp.path(), &StripPlan::default());
        assert_eq!(
            gate,
            Gate {
                cli_version: None,
                cli_verified: false
            }
        );
        assert!(!tmp.path().join("ledger").exists());
    }
}
