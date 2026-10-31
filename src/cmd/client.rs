//! What the frame verbs share as channel clients: the strict-modes check of the instance before
//! anything under the home opens ([`refuse_untrusted`]), the `cli` obs start, the reply shapes, and
//! the outputs every verb writes the same way — one `--json` document, exit 20's wrapper fault and
//! exit 21's unreachable line. Their endpoint read is `viola_state::liveness::live_endpoint`, after
//! that check; their connection is [`connect`], whose server is checked against the recorded
//! wrapper before the first frame.

use std::io::{self, Write as _};
use std::path::{Path, PathBuf};

use serde_json::{Value, json};
use viola_channel::{ChannelError, Client, SERVER_PID_REPORTED, sanitised_fault};
use viola_core::obs::{ObsEvent, ObsProcess};
use viola_core::{RefusalReason, ViolaName, is_code_word, obs_event};
use viola_state::liveness::{LiveEndpoint, ServerVerdict, verify_server};
use viola_state::strict::check_instance;

use crate::{human, obs, run};

/// The strict-modes check of the home and `instances/<name>`, before [`start`] opens the role file:
/// opening it narrows the home to its owner, which would erase the very state this refuses. A
/// refusal prints the instance's state pair, or under `--json` the `instance-unreachable`
/// document, opens nothing under the home and writes no line: `Some(21)`.
pub(crate) fn refuse_untrusted(
    home: &Path,
    name: &ViolaName,
    json: bool,
) -> anyhow::Result<Option<u8>> {
    let name: &str = name.as_ref();
    let absolute = std::path::absolute(home)?;
    if check_instance(&absolute, &absolute.join("instances").join(name)).is_ok() {
        return Ok(None);
    }
    if json {
        refused_document("instance-unreachable");
    } else {
        let (unable, hint) = human::instance_state_pair(name);
        human::refuse(&unable, &hint);
    }
    Ok(Some(21))
}

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
        return sanitised_fault(error).map(Reply::Fault);
    }
    let result = frame.get("result")?;
    if let Some(ok) = result.get("ok") {
        return Some(Reply::Ok(ok.clone()));
    }
    let reason = result.get("refusal")?.as_str()?;
    let reason = if is_code_word(reason) {
        reason.to_owned()
    } else {
        RefusalReason::Unknown.as_str().to_owned()
    };
    let detail = match result.get("detail") {
        None | Some(Value::Null) => None,
        Some(Value::String(detail)) => is_code_word(detail).then(|| detail.clone()),
        Some(_) => return None,
    };
    Some(Reply::Refused { reason, detail })
}

pub(crate) fn fault_document(detail: &Value) -> Value {
    json!({"v": 1, "error": "wrapper-fault", "detail": detail})
}

/// Why exit 21: the instance's wrapper is not there — no live snapshot or no connection
/// (`connect`), or gone while the call was open (`call`) — or the process serving its endpoint is
/// not the wrapper its snapshot records.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Unreachable {
    Dead(&'static str),
    ServerUnverified,
}

impl Unreachable {
    /// The `process-exit` `detail` (obs-plan §6 log-details).
    fn detail(self) -> &'static str {
        match self {
            Self::Dead(_) => "instance-dead",
            Self::ServerUnverified => "server-verify-failed",
        }
    }

    /// The check is part of making the connection, so its failure is `connect`'s.
    fn during(self) -> &'static str {
        match self {
            Self::Dead(during) => during,
            Self::ServerUnverified => "connect",
        }
    }

    /// The `send_hint` cause its human line ends with.
    pub(crate) fn hint_cause(self) -> &'static str {
        match self {
            Self::Dead(_) => "not-running",
            Self::ServerUnverified => "server-verify",
        }
    }
}

/// A call that ended without a reply: the channel's own failure, or a server that failed its check
/// before any frame was written.
#[derive(Debug)]
pub(crate) enum CallError {
    Channel(ChannelError),
    ServerUnverified,
}

impl From<ChannelError> for CallError {
    fn from(error: ChannelError) -> Self {
        Self::Channel(error)
    }
}

/// A connection to `live`'s endpoint whose server is the wrapper the snapshot records — its pid
/// and that pid's start time (security-plan §Authentication & Authorization, IPC client-side
/// server verification) — checked before the caller writes any frame.
pub(crate) fn connect(live: &LiveEndpoint) -> Result<Client, CallError> {
    let client = Client::connect(&live.endpoint, "cli")?;
    let verdict = verify_server(
        live.pid,
        &live.started_at,
        client.server_pid(),
        SERVER_PID_REPORTED,
    );
    match verdict {
        ServerVerdict::Matches => Ok(client),
        ServerVerdict::Mismatch | ServerVerdict::Unverifiable => Err(CallError::ServerUnverified),
    }
}

/// One request's outcome: the reply, or why the instance was unreachable. Any other channel
/// failure, and a reply of no known shape, is an internal error.
pub(crate) fn answer_of(
    sent: Result<Value, CallError>,
) -> anyhow::Result<Result<Reply, Unreachable>> {
    let frame = match sent {
        Ok(frame) => frame,
        Err(CallError::ServerUnverified) => return Ok(Err(Unreachable::ServerUnverified)),
        Err(CallError::Channel(ChannelError::Connect(_))) => {
            return Ok(Err(Unreachable::Dead("connect")));
        }
        Err(CallError::Channel(ChannelError::Closed | ChannelError::Io(_))) => {
            return Ok(Err(Unreachable::Dead("call")));
        }
        Err(CallError::Channel(error)) => return Err(error.into()),
    };
    let reply = reply_of(&frame).ok_or_else(|| anyhow::anyhow!("malformed reply"))?;
    Ok(Ok(reply))
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

/// An exit-1 refusal's `--json` document: its closed code, no detail (architecture §Standard
/// Contracts, CLI `--json` output; `viola revive`'s refusals).
pub(crate) fn refused_document(code: &str) {
    document(&json!({"v": 1, "error": code, "detail": null}));
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

/// Exit 21: the `--json` document, the same for every cause, or the verb's own stderr lines from
/// `human`; its `process-exit` at WARN names the cause (obs-plan §6 Log levels mapping).
pub(crate) fn unreachable(json: bool, cause: Unreachable, human: impl FnOnce()) -> u8 {
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
        detail = cause.detail(),
        during = cause.during(),
        duration_ms = obs::duration_ms(),
    );
    21
}

/// `unable  <name>  instance-unreachable` and its cause's `hint:` line, for `wait`, `last`,
/// `answer`, `pause` and `release`.
pub(crate) fn unable_unreachable(name: &str, cause: Unreachable) {
    let hint = human::send_hint(name, cause.hint_cause());
    let _ = human::write_unable(
        &mut io::stderr().lock(),
        name,
        "instance-unreachable",
        hint.as_deref(),
    );
}

/// Where an unverified server's refusal is made, for each verb's own case: inside the verb's
/// dispatch span, before any `channel.request` span or `channel-request` line, and with no frame
/// written (obs-plan §4, the must-trace spans).
#[cfg(test)]
#[cfg(not(target_os = "macos"))]
pub(crate) mod refused_inside {
    use std::sync::{Arc, Mutex};

    use serde_json::Value;
    use tracing_subscriber::layer::{Context, Layer, SubscriberExt as _};
    use viola_channel::{Dispatch, ProtocolError, Server};
    use viola_state::liveness::LiveEndpoint;

    use super::CallError;

    /// Every span opened and every line written, in order: `span <name>` and
    /// `event <event> in <span>`.
    #[derive(Clone, Default)]
    pub(crate) struct Seen(pub(crate) Arc<Mutex<Vec<String>>>);

    impl<S> Layer<S> for Seen
    where
        S: tracing::Subscriber + for<'a> tracing_subscriber::registry::LookupSpan<'a>,
    {
        fn on_new_span(
            &self,
            attrs: &tracing::span::Attributes<'_>,
            _: &tracing::span::Id,
            _: Context<'_, S>,
        ) {
            let name = attrs.metadata().name();
            self.0.lock().expect("seen").push(format!("span {name}"));
        }

        fn on_event(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>) {
            let mut fields = viola_channel::test_support::JsonFields::default();
            event.record(&mut fields);
            let within = ctx.event_span(event).map_or("none", |span| span.name());
            let name = fields.0.get("event").cloned().unwrap_or_default();
            self.0
                .lock()
                .expect("seen")
                .push(format!("event {name} in {within}"));
        }
    }

    /// Every frame the test server was sent.
    #[derive(Default)]
    struct Frames(Mutex<Vec<String>>);

    impl Dispatch for Frames {
        fn dispatch(&self, method: &str, _: &Value) -> Result<Value, ProtocolError> {
            self.0.lock().expect("frames").push(method.to_owned());
            Ok(serde_json::json!({"ok": {}}))
        }
    }

    /// `request` against a server this process runs while the snapshot's identity records this
    /// process under a start time a second before its own: refused before any frame, and the one
    /// span it opened is `span`.
    pub(crate) fn assert_refused_inside(
        span: &'static str,
        request: impl FnOnce(&LiveEndpoint) -> Result<Value, CallError>,
    ) {
        let dir = tempfile::tempdir().expect("the socket dir");
        let base = format!("viola-test-chan-{}-{span}", std::process::id());
        let endpoint = if cfg!(windows) {
            format!(r"\\.\pipe\{base}")
        } else {
            dir.path()
                .join(format!("{base}.sock"))
                .to_string_lossy()
                .into_owned()
        };
        let frames = Arc::new(Frames::default());
        let _serving = Server::bind(&endpoint)
            .expect("bound")
            .serve(Arc::clone(&frames) as Arc<dyn Dispatch>);
        let me = std::process::id();
        let start = viola_state::liveness::own_start(me).expect("own start");
        let secs = chrono::DateTime::parse_from_rfc3339(&start)
            .expect("rfc3339")
            .timestamp();
        let live = LiveEndpoint {
            endpoint,
            pid: me,
            started_at: crate::obs::timestamp(
                chrono::DateTime::from_timestamp(secs - 1, 0).expect("time"),
            ),
        };
        let seen = Seen::default();
        let subscriber = tracing_subscriber::registry().with(seen.clone());
        let sent = crate::test_support::with_scoped_default(subscriber, || request(&live));
        assert!(matches!(sent, Err(CallError::ServerUnverified)), "{sent:?}");
        assert_eq!(*seen.0.lock().expect("seen"), [format!("span {span}")]);
        assert!(frames.0.lock().expect("frames").is_empty());
    }
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
        Some(Reply::Fault(json!({"code": -32603, "message": "internal error", "data": null})))
    )]
    #[case::fault_holding_free_text(
        json!({"id": 1, "error": {"code": -32603, "message": "/home/x/y", "data": {"p": "canary-newname-value-e2f8"}}}),
        Some(Reply::Fault(json!({"code": -32603, "message": "internal error", "data": null})))
    )]
    #[case::fault_of_a_foreign_code_holding_free_text(
        json!({"id": 1, "error": {"code": -1, "message": "Caused by", "data": {"p": "/home/x/y"}}}),
        Some(Reply::Fault(json!({"code": -1, "message": "wrapper fault", "data": null})))
    )]
    #[case::refusal_of_free_text(
        json!({"id": 1, "result": {"refusal": "/home/x/y", "detail": "turn-running"}}),
        Some(Reply::Refused { reason: "unknown".to_owned(), detail: Some("turn-running".to_owned()) })
    )]
    #[case::detail_of_free_text(
        json!({"id": 1, "result": {"refusal": "not-delivered", "detail": "canary-newname value /home/x/y"}}),
        Some(Reply::Refused { reason: "not-delivered".to_owned(), detail: None })
    )]
    #[case::a_newer_code_word_stays_visible(
        json!({"id": 1, "result": {"refusal": "wheel-held", "detail": "some-new-detail"}}),
        Some(Reply::Refused { reason: "wheel-held".to_owned(), detail: Some("some-new-detail".to_owned()) })
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
        let channel = |error| answer_of(Err(CallError::Channel(error)));
        let reached = answer_of(Ok(json!({"id": 1, "result": {"ok": {}}}))).expect("read");
        assert_eq!(reached, Ok(Reply::Ok(json!({}))));
        let refused = io::Error::from(io::ErrorKind::ConnectionRefused);
        let connect = channel(ChannelError::Connect(refused)).expect("read");
        assert_eq!(connect, Err(Unreachable::Dead("connect")));
        assert_eq!(
            channel(ChannelError::Closed).expect("read"),
            Err(Unreachable::Dead("call"))
        );
        let reset = io::Error::from(io::ErrorKind::ConnectionReset);
        assert_eq!(
            channel(ChannelError::Io(reset)).expect("read"),
            Err(Unreachable::Dead("call"))
        );
        assert_eq!(
            answer_of(Err(CallError::ServerUnverified)).expect("read"),
            Err(Unreachable::ServerUnverified)
        );
        assert!(channel(ChannelError::Oversize).is_err());
        assert!(answer_of(Ok(json!({"id": 1}))).is_err());
    }

    /// Every line the calling thread emits, its fields and its level as JSON.
    #[derive(Clone, Default)]
    struct Lines(std::sync::Arc<std::sync::Mutex<Vec<Value>>>);

    impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for Lines {
        fn on_event(
            &self,
            event: &tracing::Event<'_>,
            _: tracing_subscriber::layer::Context<'_, S>,
        ) {
            let mut fields = viola_channel::test_support::JsonFields::default();
            event.record(&mut fields);
            let mut line = fields.0;
            line.insert("level".to_owned(), event.metadata().level().as_str().into());
            self.0.lock().expect("lines").push(Value::Object(line));
        }
    }

    /// Exit 21's `process-exit` names its cause at WARN, the same document for both, and no pid,
    /// start time, path or endpoint (obs-plan §6 log-details; architecture §Standard Contracts,
    /// CLI `--json` output).
    #[rstest]
    #[case::dead_at_connect(
        Unreachable::Dead("connect"),
        "instance-dead",
        "connect",
        "not-running"
    )]
    #[case::dead_in_the_call(Unreachable::Dead("call"), "instance-dead", "call", "not-running")]
    #[case::server_unverified(
        Unreachable::ServerUnverified,
        "server-verify-failed",
        "connect",
        "server-verify"
    )]
    fn unreachable_logs_its_cause(
        #[case] cause: Unreachable,
        #[case] detail: &str,
        #[case] during: &str,
        #[case] hint: &str,
    ) {
        use tracing_subscriber::layer::SubscriberExt as _;
        let lines = Lines::default();
        let subscriber = tracing_subscriber::registry().with(lines.clone());
        let mut human_ran = false;
        let code = crate::test_support::with_scoped_default(subscriber, || {
            unreachable(false, cause, || human_ran = true)
        });
        assert_eq!((code, human_ran), (21, true));
        assert_eq!(cause.hint_cause(), hint);
        let got = lines.0.lock().expect("lines").clone();
        assert_eq!(got.len(), 1, "{got:?}");
        let line = got[0].as_object().expect("object");
        assert_eq!(line["event"], "process-exit");
        assert_eq!(line["level"], "WARN");
        assert_eq!(line["subject"], "self");
        assert_eq!(line["exit_code"], 21);
        assert_eq!(line["detail"], detail);
        assert_eq!(line["during"], during);
        let mut keys: Vec<&str> = line.keys().map(String::as_str).collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "detail",
                "duration_ms",
                "during",
                "event",
                "exit_code",
                "level",
                "message",
                "subject"
            ]
        );
    }

    #[test]
    fn clock_part_is_the_time_of_day() {
        assert_eq!(clock_part("2026-09-23T19:45:05.912Z"), "19:45:05.912Z");
        assert_eq!(clock_part("no-t"), "no-t");
    }
}
