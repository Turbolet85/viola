//! `viola send <name> [--file <PATH>] [--json]`: the driver's text, from stdin or `--file`, typed
//! into the named instance as one bracketed paste and confirmed by the wrapper after the fact
//! (architecture [Delivery Confirmation]). The text is checked here first (client checks are
//! advisory; the wrapper re-runs them), the instance must be live by its snapshot pid + start time
//! and heartbeat, then one `send` request; the reply is the readback mirror or one `--json`
//! document, and a typed exit code.

use std::fs::File;
use std::io::{self, IsTerminal as _, Read, Write as _};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::SystemTime;

use chrono::Utc;
use serde_json::{Map, Value, json};
use tracing::instrument;
use viola_channel::{ChannelError, Client};
use viola_core::obs::{ObsEvent, ObsProcess};
use viola_core::{MAX_FRAME, NotDelivered, RefusalReason, ViolaName, obs_event};
use viola_state::heartbeat::beat_age;
use viola_state::liveness::{Liveness, classify, same_process};
use viola_state::snapshot::read_snapshot;

use crate::{human, obs, run};

#[derive(clap::Args)]
pub(crate) struct SendArgs {
    /// Instance name: [a-z0-9-], 1-32 characters, starting with a letter
    #[arg(value_parser = parse_name)]
    pub(super) name: ViolaName,
    /// Read the text from this file instead of stdin
    #[arg(long, value_name = "PATH")]
    file: Option<PathBuf>,
    /// Print one JSON document on stdout instead of the readback lines
    #[arg(long)]
    json: bool,
    /// Never set: the text is not an argument, so any second word is a usage error.
    #[allow(dead_code)]
    #[arg(hide = true, value_parser = refuse_text_argument)]
    text: Option<String>,
}

fn parse_name(raw: &str) -> Result<ViolaName, String> {
    warn_if_rewritten(raw);
    ViolaName::try_new(raw.to_owned()).map_err(|_| "invalid instance name".to_owned())
}

fn refuse_text_argument(raw: &str) -> Result<String, String> {
    warn_if_rewritten(raw);
    Err("the text is read from stdin or --file, never from an argument".to_owned())
}

/// Git for Windows rewrites a leading-slash argument into a path under its install (`/clear` →
/// `C:/Program Files/Git/clear`): say so before clap's usage error (design-system Platform-Specific
/// Notes).
fn warn_if_rewritten(raw: &str) {
    if looks_rewritten(raw) {
        let _ = io::stderr()
            .lock()
            .write_all(b"warning: argument looks like a Git Bash rewritten path\n");
    }
}

fn looks_rewritten(raw: &str) -> bool {
    let bytes = raw.as_bytes();
    bytes.len() > 3
        && bytes[0].is_ascii_alphabetic()
        && &bytes[1..3] == b":/"
        && raw[3..]
            .to_ascii_lowercase()
            .starts_with("program files/git/")
}

/// Exit 2 with one fixed line: the text could not be taken.
const UNUSABLE_TEXT: &str = "error: the text must be readable UTF-8 of at most 16 MiB\n";

/// At most `MAX_FRAME` bytes of UTF-8; `None` for anything else.
fn read_text(reader: impl Read) -> Option<String> {
    let mut bytes = Vec::new();
    reader.take(MAX_FRAME + 1).read_to_end(&mut bytes).ok()?;
    if bytes.len() as u64 > MAX_FRAME {
        return None;
    }
    String::from_utf8(bytes).ok()
}

/// What the wrapper answered.
#[derive(Debug, PartialEq)]
enum Reply {
    Ok(Value),
    Refused {
        reason: String,
        detail: Option<String>,
    },
    Fault(i64),
}

fn reply_of(frame: &Value) -> Option<Reply> {
    if let Some(error) = frame.get("error") {
        return error["code"].as_i64().map(Reply::Fault);
    }
    let result = frame.get("result")?;
    if let Some(ok) = result.get("ok") {
        return Some(Reply::Ok(ok.clone()));
    }
    let reason = result.get("refusal")?.as_str()?.to_owned();
    let detail = match result.get("detail") {
        None | Some(Value::Null) => None,
        Some(Value::String(detail)) => Some(detail.clone()),
        Some(_) => return None,
    };
    Some(Reply::Refused { reason, detail })
}

/// The closed refusal → exit table (architecture §Conventions exit codes).
fn exit_of(reason: RefusalReason) -> u8 {
    match reason {
        RefusalReason::HumanTyping => 10,
        RefusalReason::BudgetPaused => 11,
        RefusalReason::UnverifiedCli => 12,
        RefusalReason::NotDelivered => 13,
        RefusalReason::Unknown => 14,
    }
}

fn reason_of(reason: &str) -> RefusalReason {
    serde_json::from_value(Value::from(reason)).unwrap_or(RefusalReason::Unknown)
}

/// `HH:MM:SS.mmmZ` out of an RFC 3339 instant.
fn clock_part(ts: &str) -> &str {
    ts.split_once('T').map_or(ts, |(_, time)| time)
}

/// The verb's outputs: one `--json` document, or the mirror lines.
struct Out<'a> {
    name: &'a str,
    json: bool,
}

impl Out<'_> {
    fn document(&self, doc: &Value) {
        let _ = io::stdout().lock().write_all(format!("{doc}\n").as_bytes());
    }

    fn open(&self) {
        if !self.json && io::stdout().is_terminal() {
            let issued = Utc::now().format("%H:%M:%S%.3fZ").to_string();
            let _ = human::write_send_open(&mut io::stderr().lock(), self.name, &issued);
        }
    }

    fn read_back(&self, ok: &Value) {
        if self.json {
            return self.document(&json!({"v": 1, "ok": ok}));
        }
        let submitted = ok["submitted_at"].as_str().unwrap_or_default();
        let cursor = ok["cursor"].as_u64().unwrap_or_default();
        let _ = human::write_read_back(
            &mut io::stdout().lock(),
            self.name,
            clock_part(submitted),
            cursor,
        );
    }

    fn unable(&self, reason: &str, detail: Option<&str>) {
        if self.json {
            return self.document(&json!({"v": 1, "refusal": reason, "detail": detail}));
        }
        let shown = match detail {
            Some(detail) => format!("{reason}  {detail}"),
            None => reason.to_owned(),
        };
        let hint = (reason == RefusalReason::NotDelivered.as_str())
            .then_some(detail)
            .flatten()
            .and_then(|detail| human::send_hint(self.name, detail));
        let _ =
            human::write_send_unable(&mut io::stderr().lock(), self.name, &shown, hint.as_deref());
    }

    fn unreachable(&self) {
        if self.json {
            return self
                .document(&json!({"v": 1, "error": "instance-unreachable", "detail": null}));
        }
        let hint = human::send_hint(self.name, "not-running");
        let _ = human::write_send_unable(
            &mut io::stderr().lock(),
            self.name,
            "instance-unreachable",
            hint.as_deref(),
        );
    }

    fn fault(&self, code: i64) {
        if self.json {
            return self.document(&json!({"v": 1, "error": "wrapper-fault", "code": code}));
        }
        let _ = human::write_wrapper_fault(&mut io::stderr().lock(), code);
    }
}

pub(crate) fn send(home: &Path, args: &SendArgs) -> anyhow::Result<ExitCode> {
    let text = match &args.file {
        Some(path) => File::open(path).ok().and_then(read_text),
        None => read_text(io::stdin().lock()),
    };
    let Some(text) = text else {
        let _ = io::stderr().lock().write_all(UNUSABLE_TEXT.as_bytes());
        return Ok(ExitCode::from(2));
    };
    let home = std::path::absolute(home)?;
    let (level, rejection) = obs::read_diagnostics_level(&home);
    obs::viola_obs_init(&home, ObsProcess::Cli, Some(args.name.clone()), level)?;
    run::log_self_start();
    if let Some(rejection) = rejection {
        obs::log_config_rejection(rejection);
    }
    let out = Out {
        name: args.name.as_ref(),
        json: args.json,
    };
    let code = deliver(&home, &text, &out)?;
    Ok(ExitCode::from(code))
}

fn deliver(home: &Path, text: &str, out: &Out<'_>) -> anyhow::Result<u8> {
    if let Err(detail) = viola_core::validate_paste_text(text) {
        refused_client_side(detail);
        out.unable(RefusalReason::NotDelivered.as_str(), Some(detail.as_str()));
        run::log_self_exit(13, None);
        return Ok(13);
    }
    let instance_dir = home.join("instances").join(out.name);
    let Some(endpoint) = live_endpoint(&instance_dir) else {
        return Ok(unreachable(out, "connect"));
    };
    let mut params = Map::new();
    params.insert("text".to_owned(), text.into());
    if let Some(from) = own_name() {
        params.insert("from".to_owned(), from.as_ref().into());
    }
    out.open();
    let frame = match request(&endpoint, params) {
        Ok(frame) => frame,
        Err(ChannelError::Connect(_)) => return Ok(unreachable(out, "connect")),
        Err(ChannelError::Closed | ChannelError::Io(_)) => return Ok(unreachable(out, "call")),
        Err(error) => return Err(error.into()),
    };
    let reply = reply_of(&frame).ok_or_else(|| anyhow::anyhow!("malformed send reply"))?;
    let code = match reply {
        Reply::Ok(ok) => {
            out.read_back(&ok);
            0
        }
        Reply::Refused { reason, detail } => {
            out.unable(&reason, detail.as_deref());
            exit_of(reason_of(&reason))
        }
        Reply::Fault(code) => {
            out.fault(code);
            run::log_self_exit(20, Some("wrapper-fault"));
            return Ok(20);
        }
    };
    run::log_self_exit(code, None);
    Ok(code)
}

/// The endpoint of a live instance: its snapshot names one, and its recorded pid + start time and
/// heartbeat read `live`. Strict-modes and the server's identity are not checked here.
fn live_endpoint(instance_dir: &Path) -> Option<String> {
    let snapshot = read_snapshot(instance_dir)?;
    let age = beat_age(instance_dir, SystemTime::now());
    if classify(age, same_process(&snapshot)) != Liveness::Live {
        return None;
    }
    snapshot.endpoint
}

/// The sender's own instance name, self-reported.
fn own_name() -> Option<ViolaName> {
    let raw = std::env::var_os("VIOLA_NAME")?.into_string().ok()?;
    ViolaName::try_new(raw).ok()
}

#[instrument(skip_all, name = "send.client")]
fn request(endpoint: &str, params: Map<String, Value>) -> Result<Value, ChannelError> {
    Client::connect(endpoint, "cli")?.request("send", params)
}

fn refused_client_side(detail: NotDelivered) {
    obs_event!(
        INFO,
        ObsEvent::SendRefused,
        side = "client",
        refusal = RefusalReason::NotDelivered.as_str(),
        detail = detail.as_str(),
    );
}

fn unreachable(out: &Out<'_>, during: &'static str) -> u8 {
    out.unreachable();
    obs_event!(
        ERROR,
        ObsEvent::ProcessExit,
        subject = "self",
        exit_code = 21u8,
        detail = "instance-dead",
        during = during,
        duration_ms = obs::duration_ms(),
    );
    21
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case::clear("C:/Program Files/Git/clear", true)]
    #[case::lower_drive("d:/program files/git/skill", true)]
    #[case::slash_clear("/clear", false)]
    #[case::name("builder", false)]
    #[case::other_drive_path("C:/Users/me/clear", false)]
    #[case::too_short("C:/", false)]
    #[case::no_colon("CX/Program Files/Git/x", false)]
    #[case::digit_drive("1:/Program Files/Git/x", false)]
    fn looks_rewritten_matches_only_the_git_bash_shape(#[case] raw: &str, #[case] want: bool) {
        assert_eq!(looks_rewritten(raw), want);
    }

    #[test]
    fn read_text_takes_utf8_up_to_max_frame() {
        assert_eq!(
            read_text(&b"line one\nline two"[..]).as_deref(),
            Some("line one\nline two")
        );
        assert_eq!(read_text(&b"\xff\xfe"[..]), None);
        let max = usize::try_from(MAX_FRAME).expect("fits");
        assert_eq!(
            read_text(io::repeat(b'a').take(MAX_FRAME)).map(|t| t.len()),
            Some(max)
        );
        assert_eq!(read_text(io::repeat(b'a').take(MAX_FRAME + 1)), None);
    }

    #[rstest]
    #[case::human_typing("human-typing", 10)]
    #[case::budget_paused("budget-paused", 11)]
    #[case::unverified_cli("unverified-cli", 12)]
    #[case::not_delivered("not-delivered", 13)]
    #[case::unknown("unknown", 14)]
    #[case::unlisted("wheel-held", 14)]
    fn exit_of_is_the_closed_table(#[case] reason: &str, #[case] exit: u8) {
        assert_eq!(exit_of(reason_of(reason)), exit);
    }

    #[rstest]
    #[case::ok(
        json!({"id": 1, "result": {"ok": {"submitted_at": "t", "cursor": 4}}}),
        Some(Reply::Ok(json!({"submitted_at": "t", "cursor": 4})))
    )]
    #[case::refusal(
        json!({"id": 1, "result": {"refusal": "not-delivered", "detail": "turn-running"}}),
        Some(Reply::Refused { reason: "not-delivered".to_owned(), detail: Some("turn-running".to_owned()) })
    )]
    #[case::refusal_null_detail(
        json!({"id": 1, "result": {"refusal": "human-typing", "detail": null}}),
        Some(Reply::Refused { reason: "human-typing".to_owned(), detail: None })
    )]
    #[case::fault(json!({"id": 1, "error": {"code": -32603}}), Some(Reply::Fault(-32603)))]
    #[case::fault_without_code(json!({"id": 1, "error": {}}), None)]
    #[case::no_result(json!({"id": 1}), None)]
    #[case::refusal_not_string(json!({"id": 1, "result": {"refusal": 1}}), None)]
    #[case::detail_not_string(json!({"id": 1, "result": {"refusal": "x", "detail": 2}}), None)]
    fn reply_of_reads_the_three_shapes(#[case] frame: Value, #[case] want: Option<Reply>) {
        assert_eq!(reply_of(&frame), want);
    }

    #[test]
    fn clock_part_is_the_time_of_day() {
        assert_eq!(clock_part("2026-09-23T19:45:05.912Z"), "19:45:05.912Z");
        assert_eq!(clock_part("no-t"), "no-t");
    }

    #[test]
    fn live_endpoint_without_a_snapshot_is_none() {
        let tmp = tempfile::tempdir().expect("tempdir");
        assert_eq!(live_endpoint(tmp.path()), None);
    }

    #[test]
    fn text_argument_is_always_refused() {
        assert!(refuse_text_argument("hello").is_err());
        assert!(parse_name("builder").is_ok());
        assert!(parse_name("/clear").is_err());
    }
}
