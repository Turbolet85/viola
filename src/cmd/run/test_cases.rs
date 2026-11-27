use super::*;
use std::sync::Mutex;
use tracing::span::{Attributes, Id, Record};
use tracing_subscriber::layer::{Context, Layer, SubscriberExt as _};
use tracing_subscriber::registry::LookupSpan;
use viola_channel::test_support::JsonFields as Fields;
use viola_pty::Pty as _;

/// Every span the calling thread opens, in order: `{name, parent, fields}`, with values
/// recorded after creation merged in.
#[derive(Clone, Default)]
struct Spans(Arc<Mutex<Vec<(u64, Value)>>>);

impl<S: tracing::Subscriber + for<'a> LookupSpan<'a>> Layer<S> for Spans {
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let mut fields = Fields::default();
        attrs.record(&mut fields);
        let parent = ctx
            .span(id)
            .and_then(|s| s.parent())
            .map(|p| p.name().to_owned());
        let span = json!({"name": attrs.metadata().name(), "parent": parent, "fields": fields.0});
        self.0.lock().expect("spans").push((id.into_u64(), span));
    }

    fn on_record(&self, id: &Id, values: &Record<'_>, _: Context<'_, S>) {
        let mut fields = Fields::default();
        values.record(&mut fields);
        let mut spans = self.0.lock().expect("spans");
        if let Some((_, span)) = spans.iter_mut().rev().find(|(i, _)| *i == id.into_u64()) {
            for (key, value) in fields.0 {
                span["fields"][key] = value;
            }
        }
    }
}

impl Spans {
    fn all(&self) -> Vec<Value> {
        let spans = self.0.lock().expect("spans");
        spans.iter().map(|(_, span)| span.clone()).collect()
    }
}

fn capture_spans<R>(f: impl FnOnce() -> R) -> (R, Spans) {
    let spans = Spans::default();
    let subscriber = tracing_subscriber::registry().with(spans.clone());
    (
        crate::test_support::with_scoped_default(subscriber, f),
        spans,
    )
}

fn methods(instance_dir: &Path) -> Methods {
    let name = ViolaName::try_new("builder".to_owned()).expect("valid");
    let wait = Arc::new(WaitFeed::new(SystemClock));
    let wheel = Arc::new(WheelSlot::default());
    let snapshots = Arc::new(Snapshots::new(instance_dir.to_path_buf()));
    let dialogs = DialogSlot::new(
        SystemClock,
        false,
        name.clone(),
        instance_dir.to_path_buf(),
        Arc::clone(&wait),
        Arc::clone(&wheel),
        Arc::clone(&snapshots),
    );
    let send = Arc::new(SendSlot::new(SystemClock, false, Arc::clone(&wheel)));
    let links = Arc::new(LinkSet::new(
        name.clone(),
        instance_dir.to_path_buf(),
        Arc::clone(&snapshots),
    ));
    let switch = Arc::new(SwitchSlot::new(
        name.clone(),
        instance_dir.to_path_buf(),
        Arc::clone(&send),
        Arc::clone(&wheel),
        snapshots,
        ChildPlan {
            program: PathBuf::from("/bin/true"),
            cwd: instance_dir.to_path_buf(),
            pinned: viola_state::pin::Pinned {
                key: "k".to_owned(),
                path: instance_dir.join("viola"),
                path_fwd: "b".to_owned(),
            },
            plugin_dir: instance_dir.join("plugin"),
            path: None,
            strip: StripPlan::default(),
            cli_version: None,
            cli_verified: false,
        },
    ));
    Methods {
        name,
        instance_dir: instance_dir.to_path_buf(),
        send,
        wait,
        dialogs: Arc::new(dialogs),
        wheel,
        links,
        switch,
    }
}

fn events_in(instance_dir: &Path) -> Vec<Value> {
    std::fs::read_to_string(instance_dir.join("events.ndjson"))
        .unwrap_or_default()
        .lines()
        .map(|l| serde_json::from_str(l).expect("one JSON object per line"))
        .collect()
}

#[test]
fn methods_answer_method_not_found_for_every_other_method() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let params = json!({"v": 1});
    assert_eq!(
        methods(tmp.path()).dispatch_call(&call("anything", &params)),
        Err(ProtocolError::MethodNotFound)
    );
    assert!(events_in(tmp.path()).is_empty());
}

fn call<'a>(method: &'a str, params: &'a Value) -> Call<'a> {
    Call {
        method,
        params,
        id: Some(1),
        conn: None,
        srv_conn: Some("srv-1"),
    }
}

/// `link` and `unlink` are answered by the link set: one line each, in that order.
#[test]
fn methods_route_link_and_unlink_to_the_link_set() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let methods = methods(tmp.path());
    let params = json!({"v": 1, "driver": "overseer", "driven": "builder"});
    let linked = methods
        .dispatch_call(&call("link", &params))
        .expect("linked");
    assert_eq!(linked["ok"]["already_linked"], false);
    assert_eq!(
        methods.dispatch_call(&call("unlink", &params)),
        Ok(json!({"ok": {}}))
    );
    let kinds: Vec<Value> = events_in(tmp.path())
        .into_iter()
        .map(|line| line["kind"].clone())
        .collect();
    assert_eq!(kinds, [json!("link"), json!("unlink")]);
}

/// `wait` and `last` are answered by the wait feed, and a `hook.event` line reaches it.
#[test]
fn methods_route_wait_and_last_to_the_wait_feed() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let methods = methods(tmp.path());
    let params = json!({"v": 1, "after": 0, "timeout_ms": 1});
    assert_eq!(
        methods.dispatch_call(&call("wait", &params)),
        Ok(json!({"ok": {"timed_out": true}}))
    );
    let none = json!({"ok": {"last_assistant_message": null, "ts": null}});
    assert_eq!(
        methods.dispatch_call(&call("last", &json!({"v": 1}))),
        Ok(none)
    );
    let event = json!({"v": 1, "event": {"kind": "turn-ended", "data": {"last_assistant_message": "done"}}});
    methods
        .dispatch_call(&call("hook.event", &event))
        .expect("appended");
    let last = methods
        .dispatch_call(&call("last", &json!({"v": 1})))
        .expect("answered");
    assert_eq!(last["ok"]["last_assistant_message"], "done");
    assert_eq!(last["ok"]["ts"], events_in(tmp.path())[0]["ts"]);
    let woken = methods
        .dispatch_call(&call("wait", &json!({"v": 1, "after": 0})))
        .expect("answered");
    assert_eq!(woken["ok"]["event"]["kind"], "turn-ended");
}

/// `hook.dialog` and `answer` are the dialog slot's: on an unverified CLI a dialog is logged and
/// answered `null` at once, and an `answer` is refused `unverified-cli`.
#[test]
fn methods_route_hook_dialog_and_answer_to_the_dialog_slot() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let methods = methods(tmp.path());
    let dialog = json!({"v": 1, "kind": "permission", "hook_event": "permission-request",
        "data": {"tool": "Bash", "input": {}}});
    assert_eq!(
        methods.dispatch_call(&call("hook.dialog", &dialog)),
        Ok(json!({"ok": {"dialog_id": 1, "response": null}}))
    );
    assert_eq!(events_in(tmp.path())[0]["kind"], "permission");
    let answer = json!({"v": 1, "dialog_id": 1, "response": {"behavior": "allow"}});
    assert_eq!(
        methods.dispatch_call(&call("answer", &answer)),
        Ok(json!({"refusal": "unverified-cli", "detail": null}))
    );
    // Neither is a notification: without an id they are not dispatched as one.
    assert_eq!(
        methods.dispatch("hook.dialog", &dialog),
        Err(ProtocolError::MethodNotFound)
    );
}

/// Every line the calling thread emits, its fields as JSON.
#[derive(Clone, Default)]
struct Lines(Arc<Mutex<Vec<Value>>>);

impl<S: tracing::Subscriber> Layer<S> for Lines {
    fn on_event(&self, event: &tracing::Event<'_>, _: Context<'_, S>) {
        let mut fields = Fields::default();
        event.record(&mut fields);
        self.0.lock().expect("lines").push(Value::Object(fields.0));
    }
}

fn capture_lines<R>(f: impl FnOnce() -> R) -> (R, Vec<Value>) {
    let lines = Lines::default();
    let subscriber = tracing_subscriber::registry().with(lines.clone());
    let out = crate::test_support::with_scoped_default(subscriber, f);
    let got = lines.0.lock().expect("lines").clone();
    (out, got)
}

fn cli_call<'a>(method: &'a str, params: &'a Value) -> Call<'a> {
    Call {
        method,
        params,
        id: Some(4),
        conn: Some("cli-1-2-3"),
        srv_conn: Some("srv-1"),
    }
}

/// `pause` takes the wheel for the human (`send` is then `human-typing` / `manual-pause`),
/// a `release` carrying `budget` is refused with the wheel left, and a plain `release` hands it
/// back.
#[test]
fn methods_pause_and_release_move_the_wheel() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let methods = methods(tmp.path());
    let pause = json!({"v": 1, "from": "overseer"});
    assert_eq!(
        methods.dispatch_call(&cli_call("pause", &pause)),
        Ok(json!({"ok": {"wheel": "human"}}))
    );
    let send = json!({"v": 1, "text": "hello"});
    assert_eq!(
        methods.dispatch_call(&cli_call("send", &send)),
        Ok(json!({"refusal": "human-typing", "detail": "manual-pause"}))
    );
    let budget = json!({"v": 1, "budget": true});
    assert_eq!(
        methods.dispatch_call(&cli_call("release", &budget)),
        Err(ProtocolError::InvalidParams)
    );
    assert_eq!(methods.wheel.holder(), Wheel::Human);
    let release = json!({"v": 1, "from": null});
    assert_eq!(
        methods.dispatch_call(&cli_call("release", &release)),
        Ok(json!({"ok": {"wheel": "driver"}}))
    );
    assert_eq!(methods.wheel.holder(), Wheel::Driver);
    assert_eq!(
        methods.dispatch_call(&cli_call("send", &send)),
        Ok(json!({"refusal": "not-delivered", "detail": "input-not-ready"}))
    );
}

#[rstest::rstest]
#[case::pause_from_not_a_name("pause", json!({"v": 1, "from": "../x"}))]
#[case::pause_from_not_string("pause", json!({"v": 1, "from": 3}))]
#[case::release_from_not_string("release", json!({"v": 1, "from": 3}))]
#[case::release_budget_string("release", json!({"v": 1, "budget": "yes"}))]
#[case::release_budget_number("release", json!({"v": 1, "budget": 1}))]
#[case::release_budget_null("release", json!({"v": 1, "budget": null}))]
fn methods_wheel_params_they_cannot_take_are_invalid(#[case] method: &str, #[case] params: Value) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let methods = methods(tmp.path());
    assert_eq!(
        methods.dispatch_call(&cli_call(method, &params)),
        Err(ProtocolError::InvalidParams)
    );
    assert_eq!(methods.wheel.holder(), Wheel::Driver);
}

/// A `release` carrying a string `from` is a driver's: `-32602` `release-from-driver`, the
/// wheel unmoved, and one `release-from-driver` line (its `from` only when it is a name). The
/// refusal comes before `budget` is read: with `true`, and with a value that is no bool.
#[rstest::rstest]
#[case::a_name("overseer", Some("overseer"))]
#[case::not_a_name("Not A Name", None)]
fn methods_release_from_a_driver_is_refused_and_logged(
    #[case] from: &str,
    #[case] logged: Option<&str>,
    #[values(json!(true), json!("ignored"))] budget: Value,
) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let methods = methods(tmp.path());
    let none = json!({"v": 1});
    methods
        .dispatch_call(&cli_call("pause", &none))
        .expect("paused");
    let params = json!({"v": 1, "from": from, "budget": budget});
    let (reply, lines) = capture_lines(|| methods.dispatch_call(&cli_call("release", &params)));
    assert_eq!(reply, Err(ProtocolError::ReleaseFromDriver));
    assert_eq!(methods.wheel.holder(), Wheel::Human);
    let refused: Vec<&Value> = lines
        .iter()
        .filter(|l| l["event"] == "release-from-driver")
        .collect();
    assert_eq!(refused.len(), 1, "{lines:?}");
    let mut want = json!({"event": "release-from-driver", "message": "release-from-driver",
        "corr": 4, "conn": "cli-1-2-3", "from_trust": "self-reported"});
    if let Some(name) = logged {
        want["from"] = json!(name);
    }
    let mut got = refused[0].clone();
    for absent in ["process", "instance"] {
        got.as_object_mut().expect("object").remove(absent);
    }
    assert_eq!(got, want);
}

/// A valid `hook.event` becomes one `source:"hook"` line stamped by the wrapper.
#[test]
fn methods_append_a_hook_event() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let params = json!({"v": 1, "sender": "0.1.0", "ts": "t",
        "event": {"kind": "turn-ended", "data": {"last_assistant_message": null}}});
    assert_eq!(
        methods(tmp.path()).dispatch("hook.event", &params),
        Ok(Value::Null)
    );
    let lines = events_in(tmp.path());
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0]["v"], 1);
    assert_eq!(lines[0]["instance"], "builder");
    assert_eq!(lines[0]["kind"], "turn-ended");
    assert_eq!(lines[0]["source"], "hook");
    assert_eq!(lines[0]["data"], json!({"last_assistant_message": null}));
    let ts = lines[0]["ts"].as_str().expect("ts");
    assert!(ts.ends_with('Z') && ts.len() == 24, "{ts}");
}

#[rstest::rstest]
#[case::session_start(json!({"kind": "session-start", "data": {"cause": "startup", "agent_session_id": null}}))]
#[case::prompt(json!({"kind": "prompt-submitted", "data": {"text": "hi", "origin": "human"}}))]
#[case::harness(json!({"kind": "prompt-submitted", "data": {"text": "", "origin": "harness"}}))]
#[case::session_end(json!({"kind": "session-end", "data": {}}))]
#[case::activity(json!({"kind": "activity", "data": {"tool": "Bash"}}))]
fn methods_take_every_hook_kind(#[case] event: Value) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let kind = event["kind"].clone();
    methods(tmp.path())
        .dispatch("hook.event", &json!({"v": 1, "event": event}))
        .expect("dispatched");
    let lines = events_in(tmp.path());
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0]["kind"], kind);
}

#[rstest::rstest]
#[case::no_event(json!({"v": 1}))]
#[case::wrapper_kind(json!({"v": 1, "event": {"kind": "wheel", "data": {}}}))]
#[case::unknown_kind(json!({"v": 1, "event": {"kind": "question", "data": {}}}))]
#[case::no_data(json!({"v": 1, "event": {"kind": "session-end"}}))]
#[case::data_not_object(json!({"v": 1, "event": {"kind": "session-end", "data": [1]}}))]
#[case::driver_origin(json!({"v": 1, "event": {"kind": "prompt-submitted", "data": {"text": "x", "origin": "driver"}}}))]
#[case::no_origin(json!({"v": 1, "event": {"kind": "prompt-submitted", "data": {"text": "x"}}}))]
#[case::text_not_string(json!({"v": 1, "event": {"kind": "prompt-submitted", "data": {"text": 1, "origin": "human"}}}))]
fn methods_refuse_an_event_that_fails_the_checks(#[case] params: Value) {
    let tmp = tempfile::tempdir().expect("tempdir");
    assert_eq!(
        methods(tmp.path()).dispatch("hook.event", &params),
        Ok(Value::Null)
    );
    assert!(!tmp.path().join("events.ndjson").exists());
}

/// A `turn-ended` is appended only with its `cause` absent or a closed `TurnEndCause` word.
#[rstest::rstest]
#[case::rate_limited(Some(json!("rate-limited")), true)]
#[case::overloaded(Some(json!("overloaded")), true)]
#[case::auth_failed(Some(json!("auth-failed")), true)]
#[case::billing(Some(json!("billing")), true)]
#[case::invalid_request(Some(json!("invalid-request")), true)]
#[case::model_not_found(Some(json!("model-not-found")), true)]
#[case::server_error(Some(json!("server-error")), true)]
#[case::output_limit(Some(json!("output-limit")), true)]
#[case::api_error(Some(json!("api-error")), true)]
#[case::absent(None, true)]
#[case::raw_server_error(Some(json!("server_error")), false)]
#[case::raw_rate_limit(Some(json!("rate_limit")), false)]
#[case::a_number(Some(json!(7)), false)]
#[case::null(Some(Value::Null), false)]
#[case::error_text(Some(json!("API Error: Connection lost mid-response")), false)]
fn run_hook_event_turn_ended_with_a_cause_outside_the_closed_set_is_not_appended(
    #[case] cause: Option<Value>,
    #[case] appended: bool,
) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let mut data = json!({"last_assistant_message": null, "background_tasks": null});
    if let Some(cause) = cause {
        data["cause"] = cause;
    }
    let params = json!({"v": 1, "event": {"kind": "turn-ended", "data": data.clone()}});
    assert_eq!(
        methods(tmp.path()).dispatch("hook.event", &params),
        Ok(Value::Null)
    );
    if appended {
        let lines = events_in(tmp.path());
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["kind"], "turn-ended");
        assert_eq!(lines[0]["data"], data);
    } else {
        assert!(!tmp.path().join("events.ndjson").exists());
    }
}

/// A `transcript` beside the event is taken only for a `session-start`, only as a string passing
/// `transcript_path_ok` against the line's own id; the line is taken either way and its data
/// never holds the path.
#[rstest::rstest]
#[case::its_own_shape("session-start", json!("/t/SID.jsonl"), true)]
#[case::relative("session-start", json!("t/SID.jsonl"), false)]
#[case::a_parent_component("session-start", json!("/t/../SID.jsonl"), false)]
#[case::another_session_s_file("session-start", json!("/t/1b1c2d3e-4f50-6172-8394-a5b6c7d8e9f0.jsonl"), false)]
#[case::not_a_string("session-start", json!(7), false)]
#[case::on_another_kind("session-end", json!("/t/SID.jsonl"), false)]
fn run_hook_event_transcript_is_re_validated(
    #[case] kind: &str,
    #[case] transcript: Value,
    #[case] taken: bool,
) {
    const SID: &str = "0b1c2d3e-4f50-6172-8394-a5b6c7d8e9f0";
    let tmp = tempfile::tempdir().expect("tempdir");
    let transcript = match transcript.as_str() {
        Some(path) => json!(path.replace("SID", SID)),
        None => transcript,
    };
    let data = json!({"cause": "startup", "agent_session_id": SID});
    let params = json!({"v": 1, "event": {"kind": kind, "data": data}, "transcript": transcript});
    let (line, kept) = methods(tmp.path())
        .hook_event_line(&params)
        .expect("the line is taken");
    assert_eq!(line.data, data);
    let want = taken.then(|| PathBuf::from(transcript.as_str().expect("a path")));
    assert_eq!(kept, want);
}

#[test]
fn methods_report_an_append_that_failed_as_internal() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let params = json!({"v": 1, "event": {"kind": "session-end", "data": {}}});
    assert_eq!(
        methods(&tmp.path().join("missing")).dispatch("hook.event", &params),
        Err(ProtocolError::Internal)
    );
}

/// Every event line the calling thread emits, its fields as JSON.
#[derive(Clone, Default)]
struct Events(Arc<Mutex<Vec<Value>>>);

impl<S: tracing::Subscriber> Layer<S> for Events {
    fn on_event(&self, event: &tracing::Event<'_>, _: Context<'_, S>) {
        let mut fields = Fields::default();
        event.record(&mut fields);
        self.0.lock().expect("events").push(Value::Object(fields.0));
    }
}

/// A live wrapper's first snapshot, cut short or re-versioned to `v` 99 from the writer's own
/// bytes, its beat aged as given: the check takes the replay, which records no pid, so the beat
/// alone decides — over a stale beat the name is free, over a fresh one it is `already-live` —
/// and the replay writes its one `state-recovered` line.
#[rstest::rstest]
#[case::cut_short_stale("cut", 10, "snapshot-replayed", true)]
#[case::cut_short_fresh("cut", 0, "snapshot-replayed", false)]
#[case::unsupported_v_stale("v99", 10, "snapshot-unsupported-v", true)]
#[case::unsupported_v_fresh("v99", 0, "snapshot-unsupported-v", false)]
fn collision_check_over_a_replayed_snapshot_reads_the_beat_alone(
    #[case] fault: &str,
    #[case] beat_secs: u64,
    #[case] detail: &str,
    #[case] free: bool,
) {
    let tmp = tempfile::tempdir().expect("tempdir");
    let dir = tmp.path().join("instances").join("builder");
    create_private_dir(&dir).expect("instance dir");
    first_snapshot(&dir, tmp.path());
    let path = dir.join(viola_state::snapshot::SNAPSHOT);
    let bytes = std::fs::read(&path).expect("bytes");
    let faulted = if fault == "cut" {
        bytes[..bytes.len() / 2].to_vec()
    } else {
        String::from_utf8(bytes)
            .expect("utf-8")
            .replacen("\"v\":1", "\"v\":99", 1)
            .into_bytes()
    };
    std::fs::write(&path, faulted).expect("faulted");
    std::fs::OpenOptions::new()
        .append(true)
        .open(dir.join("heartbeat"))
        .expect("the beat")
        .set_modified(SystemTime::now() - std::time::Duration::from_secs(beat_secs))
        .expect("mtime");
    let events = Events::default();
    let subscriber = tracing_subscriber::registry().with(events.clone());
    let name = ViolaName::try_new("builder".to_owned()).expect("valid");
    let taken = crate::test_support::with_scoped_default(subscriber, || {
        let recovered = read_snapshot_or_replay(&dir).ok();
        collision_check(&name, &dir, recovered.as_ref(), true).is_some()
    });
    assert_eq!(taken, !free);
    let lines = events.0.lock().expect("events").clone();
    let recovered: Vec<&Value> = lines
        .iter()
        .filter(|l| l["event"] == "state-recovered")
        .collect();
    assert_eq!(recovered.len(), 1, "{lines:?}");
    assert_eq!(recovered[0]["detail"], detail);
}

/// The first snapshot `start_state` writes for a child spawned in `spawn_dir`, read back from
/// the instance directory.
fn first_snapshot(instance_dir: &Path, spawn_dir: &Path) -> InstanceSnapshot {
    first_snapshot_of(instance_dir, spawn_dir, Lineage::default())
}

fn first_snapshot_of(instance_dir: &Path, spawn_dir: &Path, lineage: Lineage) -> InstanceSnapshot {
    first_snapshot_with(instance_dir, spawn_dir, lineage, run::launch::compiled())
}

fn first_snapshot_with(
    instance_dir: &Path,
    spawn_dir: &Path,
    lineage: Lineage,
    launch: Pair,
) -> InstanceSnapshot {
    first_snapshot_opened(instance_dir, spawn_dir, lineage, launch, None)
}

fn first_snapshot_opened(
    instance_dir: &Path,
    spawn_dir: &Path,
    lineage: Lineage,
    launch: Pair,
    window: Option<Window>,
) -> InstanceSnapshot {
    let name = ViolaName::try_new("builder".to_owned()).expect("valid");
    let pinned = Pinned {
        key: "k".to_owned(),
        path: PathBuf::from("bin").join("k").join("viola"),
        path_fwd: "bin/k/viola".to_owned(),
    };
    let gate = Gate::default();
    let snapshots = Arc::new(Snapshots::new(instance_dir.to_path_buf()));
    let links = LinkSet::new(
        name.clone(),
        instance_dir.to_path_buf(),
        Arc::clone(&snapshots),
    );
    links.restore().expect("the link set");
    let (_beat, returned) = start_state(
        &name,
        instance_dir,
        &snapshots,
        &pinned,
        "endpoint".to_owned(),
        gate,
        Recorded {
            spawn_dir,
            links: links.current(),
            lineage,
            launch,
            window,
        },
    )
    .expect("the start state");
    assert_eq!(
        viola_state::snapshot::read_snapshot(instance_dir),
        Some(returned.clone())
    );
    returned
}

#[derive(clap::Parser)]
struct Line {
    #[command(flatten)]
    args: RunArgs,
}

fn parse(words: &[&str]) -> Result<RunArgs, clap::error::ErrorKind> {
    use clap::Parser as _;
    Line::try_parse_from(std::iter::once("run").chain(words.iter().copied()))
        .map_err(|error| error.kind())
        .and_then(|line| checked(line.args).map_err(|error| error.kind()))
}

fn named(text: &str) -> ViolaName {
    ViolaName::try_new(text.to_owned()).expect("valid")
}

fn field<'a>(spans: &'a [Value], name: &str) -> &'a Value {
    spans
        .iter()
        .find(|s| s["name"] == name)
        .map(|s| &s["fields"])
        .unwrap_or_else(|| panic!("no span {name} in {spans:?}"))
}

/// One in-process start through the spawn (obs-plan §4 Scenario 1): every step's span, in the
/// documented order, under `run.start`, with its required fields; then the seam's own spans.
/// The home is outside `target/e2e-home`: its pinned copy is this test binary, not `viola`.
/// The child is a host program, never this test binary. The wrapper's plugin flag comes first
/// in a child's arguments, libtest refuses it and exits by itself, and under coverage a kill
/// that lands in that exit's profile write leaves a profile `llvm-profdata` refuses. `whoami`
/// refuses the flag too, and writes no profile (test-plan §10 Zero-flakiness budget).
#[test]
fn start_opens_the_scenario_one_spans_under_run_start() {
    let tmp = tempfile::tempdir().expect("tempdir");
    let launch = Launch {
        name: ViolaName::try_new("builder".to_owned()).expect("valid"),
        program: OsString::from("whoami"),
        args: Vec::new(),
        spawn_dir: std::env::current_dir().expect("cwd"),
        json: false,
        lineage: Lineage::default(),
        given: (None, None),
        window: None,
    };
    let exe = std::fs::read(std::env::current_exe().expect("exe")).expect("exe bytes");
    let (started, spans) = capture_spans(|| start(&tmp.path().join("home"), &launch, &[], &exe));
    let Started::Launched(mut launched) = started.expect("started") else {
        panic!("the start was refused");
    };
    let (_, seam) = capture_spans(|| {
        let _ = launched.pty.resize(Size { cols: 90, rows: 30 });
        let _ = launched.pty.kill();
    });
    drop(launched);

    let spans = spans.all();
    let names: Vec<&str> = spans.iter().filter_map(|s| s["name"].as_str()).collect();
    assert_eq!(
        names,
        [
            "run.start",
            "state.snapshot_recover",
            "run.collision_check",
            "run.pin_copy",
            "run.version_gate",
            "channel.bind",
            "state.snapshot_write",
            "state.heartbeat_start",
            "pty.spawn",
            "state.snapshot_write",
        ]
    );
    assert!(spans[0]["parent"].is_null());
    assert!(
        spans[1..].iter().all(|s| s["parent"] == "run.start"),
        "{spans:?}"
    );
    assert_eq!(field(&spans, "run.start")["instance"], "builder");
    assert_eq!(field(&spans, "run.collision_check")["outcome"], "free");
    assert_eq!(field(&spans, "run.pin_copy")["outcome"], "ok");
    assert_eq!(field(&spans, "run.version_gate")["cli_verified"], false);
    assert!(
        field(&spans, "run.version_gate")
            .get("cli_version")
            .is_none()
    );
    assert_eq!(
        field(&spans, "channel.bind")["endpoint_kind"],
        viola_channel::ENDPOINT_KIND
    );
    assert_eq!(field(&spans, "state.snapshot_write")["v"], 1);
    let spawn = field(&spans, "pty.spawn");
    assert_eq!(spawn["pty_backend"], viola_pty::pty_backend());
    assert!(spawn["env_stripped_count"].is_u64(), "{spawn}");

    let seam: Vec<Value> = seam.all();
    let names: Vec<&str> = seam.iter().filter_map(|s| s["name"].as_str()).collect();
    assert_eq!(names, ["pty.resize", "pty.kill"]);
}

#[path = "test_cases_start.rs"]
mod start;
