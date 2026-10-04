//! What `send`, `wait` and `last` share as channel clients: the `cli` obs start, the liveness-only
//! endpoint (security-plan §Authentication & Authorization: no server identity and no strict-modes
//! until Epoch 6, a dated gap), the reply shapes, and the outputs every verb writes the same way —
//! one `--json` document, exit 20's wrapper fault and exit 21's unreachable line.

use std::io::{self, Write as _};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde_json::{Value, json};
use viola_channel::ChannelError;
use viola_core::obs::{ObsEvent, ObsProcess};
use viola_core::{ViolaName, obs_event};
use viola_state::heartbeat::beat_age;
use viola_state::liveness::{Liveness, classify, same_process};
use viola_state::snapshot::read_snapshot;

use crate::{human, obs, run};

/// The absolute home, with this process's `cli` role file open and its start logged.
pub(crate) fn start(home: &Path, name: &ViolaName) -> anyhow::Result<PathBuf> {
    let home = std::path::absolute(home)?;
    let (level, rejection) = obs::read_diagnostics_level(&home);
    obs::viola_obs_init(&home, ObsProcess::Cli, Some(name.clone()), level)?;
    run::log_self_start();
    if let Some(rejection) = rejection {
        obs::log_config_rejection(rejection);
    }
    Ok(home)
}

/// What the wrapper answered.
#[derive(Debug, PartialEq)]
pub(crate) enum Reply {
    Ok(Value),
    Refused {
        reason: String,
        detail: Option<String>,
    },
    /// The contract's `detail`: `{code, message, data}` (architecture §Standard Contracts, CLI
    /// `--json` output).
    Fault(Value),
}

pub(crate) fn reply_of(frame: &Value) -> Option<Reply> {
    if let Some(error) = frame.get("error") {
        return fault_detail(error).map(Reply::Fault);
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

/// A JSON-RPC `error` member as the wrapper-fault `detail`; `None` without an integer `code`.
fn fault_detail(error: &Value) -> Option<Value> {
    let code = error["code"].as_i64()?;
    let message = error["message"].as_str().unwrap_or_default();
    let data = match &error["data"] {
        Value::Object(data) => Value::Object(data.clone()),
        _ => Value::Null,
    };
    Some(json!({"code": code, "message": message, "data": data}))
}

pub(crate) fn fault_document(detail: &Value) -> Value {
    json!({"v": 1, "error": "wrapper-fault", "detail": detail})
}

/// One request's outcome: the reply, or `Err(during)` when the instance could not be reached
/// (`connect`) or vanished while the call was open (`call`). Any other channel failure, and a
/// reply of no known shape, is an internal error.
pub(crate) fn answer_of(
    sent: Result<Value, ChannelError>,
) -> anyhow::Result<Result<Reply, &'static str>> {
    let frame = match sent {
        Ok(frame) => frame,
        Err(ChannelError::Connect(_)) => return Ok(Err("connect")),
        Err(ChannelError::Closed | ChannelError::Io(_)) => return Ok(Err("call")),
        Err(error) => return Err(error.into()),
    };
    let reply = reply_of(&frame).ok_or_else(|| anyhow::anyhow!("malformed reply"))?;
    Ok(Ok(reply))
}

/// The endpoint of a live instance: its snapshot names one, and its recorded pid + start time and
/// heartbeat read `live`. Strict-modes and the server's identity are not checked here.
pub(crate) fn live_endpoint(instance_dir: &Path) -> Option<String> {
    let snapshot = read_snapshot(instance_dir)?;
    let age = beat_age(instance_dir, SystemTime::now());
    if classify(age, same_process(&snapshot)) != Liveness::Live {
        return None;
    }
    snapshot.endpoint
}

/// The sender's own instance name, self-reported.
pub(crate) fn own_name() -> Option<ViolaName> {
    let raw = std::env::var_os("VIOLA_NAME")?.into_string().ok()?;
    ViolaName::try_new(raw).ok()
}

/// `HH:MM:SS.mmmZ` out of an RFC 3339 instant.
pub(crate) fn clock_part(ts: &str) -> &str {
    ts.split_once('T').map_or(ts, |(_, time)| time)
}

/// One `--json` document on stdout, as one write.
pub(crate) fn document(doc: &Value) {
    let _ = io::stdout().lock().write_all(format!("{doc}\n").as_bytes());
}

/// Exit 20: the wrapper-fault document, or `error: wrapper fault  <code>` with no hint.
pub(crate) fn fault(json: bool, detail: &Value) -> u8 {
    if json {
        document(&fault_document(detail));
    } else {
        let code = detail["code"].as_i64().unwrap_or_default();
        let _ = human::write_wrapper_fault(&mut io::stderr().lock(), code);
    }
    run::log_self_exit(20, Some("wrapper-fault"));
    20
}

/// Exit 21: the `--json` document, or the verb's own stderr lines from `human`; its
/// `process-exit` at WARN (obs-plan §6 Log levels mapping).
pub(crate) fn unreachable(json: bool, during: &'static str, human: impl FnOnce()) -> u8 {
    if json {
        document(&json!({"v": 1, "error": "instance-unreachable", "detail": null}));
    } else {
        human();
    }
    obs_event!(
        WARN,
        ObsEvent::ProcessExit,
        subject = "self",
        exit_code = 21u8,
        detail = "instance-dead",
        during = during,
        duration_ms = obs::duration_ms(),
    );
    21
}

/// `unable  <name>  instance-unreachable` and its `hint:` line, for `wait` and `last`.
pub(crate) fn unable_unreachable(name: &str) {
    let hint = human::send_hint(name, "not-running");
    let _ = human::write_unable(
        &mut io::stderr().lock(),
        name,
        "instance-unreachable",
        hint.as_deref(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

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
    #[case::fault(
        json!({"id": 1, "error": {"code": -32603, "message": "internal error", "data": null}}),
        Some(Reply::Fault(json!({"code": -32603, "message": "internal error", "data": null})))
    )]
    #[case::fault_with_data(
        json!({"id": 1, "error": {"code": -32602, "message": "unsupported protocol version", "data": {"supported": 1, "wrapper": "0.1.0"}}}),
        Some(Reply::Fault(json!({"code": -32602, "message": "unsupported protocol version", "data": {"supported": 1, "wrapper": "0.1.0"}})))
    )]
    #[case::fault_without_message_or_object_data(
        json!({"id": 1, "error": {"code": -32603, "data": [1]}}),
        Some(Reply::Fault(json!({"code": -32603, "message": "", "data": null})))
    )]
    #[case::fault_without_code(json!({"id": 1, "error": {}}), None)]
    #[case::no_result(json!({"id": 1}), None)]
    #[case::refusal_not_string(json!({"id": 1, "result": {"refusal": 1}}), None)]
    #[case::detail_not_string(json!({"id": 1, "result": {"refusal": "x", "detail": 2}}), None)]
    fn reply_of_reads_the_three_shapes(#[case] frame: Value, #[case] want: Option<Reply>) {
        assert_eq!(reply_of(&frame), want);
    }

    /// architecture §Standard Contracts, CLI `--json` output: exit 20's one document.
    #[test]
    fn fault_document_is_the_contract_shape() {
        let detail = json!({"code": -32603, "message": "internal error", "data": null});
        assert_eq!(
            fault_document(&detail).to_string(),
            r#"{"v":1,"error":"wrapper-fault","detail":{"code":-32603,"message":"internal error","data":null}}"#
        );
    }

    #[test]
    fn answer_of_splits_unreachable_from_a_fault() {
        let reached = answer_of(Ok(json!({"id": 1, "result": {"ok": {}}}))).expect("read");
        assert_eq!(reached, Ok(Reply::Ok(json!({}))));
        let refused = io::Error::from(io::ErrorKind::ConnectionRefused);
        let connect = answer_of(Err(ChannelError::Connect(refused))).expect("read");
        assert_eq!(connect, Err("connect"));
        assert_eq!(
            answer_of(Err(ChannelError::Closed)).expect("read"),
            Err("call")
        );
        let reset = io::Error::from(io::ErrorKind::ConnectionReset);
        assert_eq!(
            answer_of(Err(ChannelError::Io(reset))).expect("read"),
            Err("call")
        );
        assert!(answer_of(Err(ChannelError::Oversize)).is_err());
        assert!(answer_of(Ok(json!({"id": 1}))).is_err());
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
}
