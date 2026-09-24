//! `viola::obs`: the codes-only role file per process, the owner-only per-instance detail files
//! and the `diagnostics_level` read (obs-plan §3). Nothing here writes stdout or stderr.

use std::fs::{self, File, OpenOptions};
use std::io::{self, Read as _, Write as _};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use std::time::Instant;

use chrono::{DateTime, SecondsFormat, Utc};
use serde_json::{Map, Value};
use tracing::Level;
use tracing_subscriber::filter::{LevelFilter, Targets};
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;
use tracing_subscriber::layer::SubscriberExt as _;
use viola_core::obs::{ObsEvent, ObsProcess, ProcessCtx};
use viola_core::{MAX_FRAME, ViolaName, obs_event};

/// Every other target (rmcp, axum, notify, …) is OFF: its records lack the required fields.
const VIOLA_TARGETS: [&str; 8] = [
    "viola",
    "viola_core",
    "viola_pty",
    "viola_channel",
    "viola_state",
    "viola_agent_claude",
    "viola_mcp",
    "viola_ui",
];

const CONFIG_KEYS: [&str; 2] = ["v", "diagnostics_level"];

static STARTED: OnceLock<Instant> = OnceLock::new();

/// Creates `dir` (and missing parents) owner-only; an existing one is narrowed to 0700 on Unix.
pub(crate) fn ensure_private_dir(dir: &Path) -> io::Result<()> {
    let mut builder = fs::DirBuilder::new();
    builder.recursive(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::{DirBuilderExt as _, PermissionsExt as _};
        builder.mode(0o700).create(dir)?;
        fs::set_permissions(dir, fs::Permissions::from_mode(0o700))
    }
    #[cfg(not(unix))]
    builder.create(dir)
}

/// Append-only, 0600 on Unix.
fn open_private_append(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.create(true).append(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _};
        options.mode(0o600);
        let file = options.open(path)?;
        file.set_permissions(fs::Permissions::from_mode(0o600))?;
        Ok(file)
    }
    #[cfg(not(unix))]
    options.open(path)
}

/// The home-level role file's basename; `None` when the role's naming input is missing.
pub(crate) fn role_file_name(
    process: ObsProcess,
    instance: Option<&ViolaName>,
    port: Option<u16>,
) -> Option<String> {
    let stem = match (process, instance, port) {
        (ObsProcess::Mcp, _, _) => "mcp".to_owned(),
        (ObsProcess::Ui, _, Some(port)) => format!("ui-{port}"),
        (ObsProcess::Run | ObsProcess::Hook | ObsProcess::Cli, Some(name), _) => {
            format!("{}-{}", process.as_str(), name_str(name))
        }
        _ => return None,
    };
    Some(format!("{stem}.ndjson"))
}

fn name_str(name: &ViolaName) -> &str {
    name.as_ref()
}

/// `<home>/diagnostics/<role file>`, the dirs 0700 and the file 0600.
pub(crate) fn open_role_file(
    home: &Path,
    process: ObsProcess,
    instance: Option<&ViolaName>,
    port: Option<u16>,
) -> io::Result<Arc<File>> {
    let name = role_file_name(process, instance, port)
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?;
    ensure_private_dir(home)?;
    let dir = home.join("diagnostics");
    ensure_private_dir(&dir)?;
    Ok(Arc::new(open_private_append(&dir.join(name))?))
}

pub(crate) fn timestamp(at: DateTime<Utc>) -> String {
    at.to_rfc3339_opts(SecondsFormat::Millis, true)
}

struct MillisUtc;

impl FormatTime for MillisUtc {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        w.write_str(&timestamp(Utc::now()))
    }
}

fn targets(level: Level) -> Targets {
    VIOLA_TARGETS
        .iter()
        .fold(Targets::new().with_default(LevelFilter::OFF), |t, name| {
            t.with_target(*name, level)
        })
}

/// One JSON object per line, one `write` per line, into `writer` only (obs-plan §3 Logging stack).
fn subscriber<W>(writer: W, level: Level) -> impl tracing::Subscriber + Send + Sync
where
    W: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let layer = tracing_subscriber::fmt::layer()
        .json()
        .flatten_event(true)
        .with_current_span(false)
        .with_span_list(false)
        .with_ansi(false)
        .log_internal_errors(false)
        .with_timer(MillisUtc)
        .with_writer(writer);
    tracing_subscriber::registry()
        .with(targets(level))
        .with(layer)
}

/// Opens the role file, then sets the process context, the panic sink and the global subscriber.
pub(crate) fn viola_obs_init(
    home: &Path,
    process: ObsProcess,
    instance: Option<ViolaName>,
    level: Level,
) -> io::Result<()> {
    let file = open_role_file(home, process, instance.as_ref(), None)?;
    let _ = STARTED.set(Instant::now());
    crate::set_panic_sink(file.clone(), process, instance.clone(), home.to_path_buf());
    viola_core::obs::set_ctx(ProcessCtx { process, instance });
    let _ = tracing::subscriber::set_global_default(subscriber(file, level));
    Ok(())
}

/// Milliseconds since `viola_obs_init`; 0 before it.
pub(crate) fn duration_ms() -> u64 {
    STARTED.get().map_or(0, |started| {
        u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX)
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConfigRejection {
    Unreadable,
    Malformed,
    UnknownKeys(usize),
}

impl ConfigRejection {
    fn detail(self) -> &'static str {
        match self {
            Self::Unreadable => "unreadable",
            Self::Malformed => "malformed",
            Self::UnknownKeys(_) => "unknown-keys",
        }
    }

    fn count(self) -> usize {
        match self {
            Self::UnknownKeys(n) => n,
            Self::Unreadable | Self::Malformed => 1,
        }
    }
}

/// `<home>/config.json` `diagnostics_level` is the only level knob (never `RUST_LOG`); anything
/// unusable falls back to INFO and is reported, never fatal.
pub(crate) fn read_diagnostics_level(home: &Path) -> (Level, Option<ConfigRejection>) {
    let mut bytes = Vec::new();
    match File::open(home.join("config.json")) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => (Level::INFO, None),
        Err(_) => (Level::INFO, Some(ConfigRejection::Unreadable)),
        Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
            Ok(_) => parse_diagnostics_level(&bytes),
            Err(_) => (Level::INFO, Some(ConfigRejection::Unreadable)),
        },
    }
}

fn parse_diagnostics_level(bytes: &[u8]) -> (Level, Option<ConfigRejection>) {
    let Ok(Value::Object(map)) = serde_json::from_slice::<Value>(bytes) else {
        return (Level::INFO, Some(ConfigRejection::Malformed));
    };
    if map.get("v").and_then(Value::as_u64) != Some(1) {
        return (Level::INFO, Some(ConfigRejection::Malformed));
    }
    let (level, bad_value) = match map.get("diagnostics_level").map(Value::as_str) {
        None | Some(Some("info")) => (Level::INFO, false),
        Some(Some("debug")) => (Level::DEBUG, false),
        Some(_) => (Level::INFO, true),
    };
    let unknown = map
        .keys()
        .filter(|k| !CONFIG_KEYS.contains(&k.as_str()))
        .count();
    let rejection = if bad_value {
        Some(ConfigRejection::Malformed)
    } else if unknown > 0 {
        Some(ConfigRejection::UnknownKeys(unknown))
    } else {
        None
    };
    (level, rejection)
}

pub(crate) fn log_config_rejection(rejection: ConfigRejection) {
    obs_event!(
        WARN,
        ObsEvent::ParseRejected,
        parser = "config-json",
        detail = rejection.detail(),
        count = rejection.count(),
    );
}

/// The catch site's role line after a caught panic or a dispatch error: only `run` owns one
/// (obs-plan §7).
pub(crate) fn internal_error_exit_line() {
    if viola_core::obs::ctx().is_some_and(|c| c.process == ObsProcess::Run) {
        obs_event!(
            ERROR,
            ObsEvent::ProcessExit,
            subject = "self",
            exit_code = 1u8,
            detail = "internal-error",
            duration_ms = duration_ms(),
        );
    }
}

/// Where a dispatch error's chain may be recorded, known once the home and the instance resolved.
pub(crate) struct DetailSink {
    pub(crate) home: PathBuf,
    pub(crate) instance: ViolaName,
    pub(crate) process: ObsProcess,
}

/// The catch site's records for a dispatch error: the codes-only role line, and the chain only in
/// the instance detail file (obs-plan §7); nothing reaches stdout or stderr.
pub(crate) fn report_internal_error(error: &anyhow::Error, sink: Option<&DetailSink>) {
    internal_error_exit_line();
    if let Some(sink) = sink {
        let line = chain_detail_line(&timestamp(Utc::now()), sink.process, &sink.instance, error);
        write_detail(&sink.home, &sink.instance, sink.process, &line);
    }
}

/// `chain` holds every cause's text, outermost first.
fn chain_detail_line(
    timestamp: &str,
    process: ObsProcess,
    instance: &ViolaName,
    error: &anyhow::Error,
) -> String {
    let chain: Vec<Value> = error
        .chain()
        .map(|cause| Value::from(cause.to_string()))
        .collect();
    let mut fields = Map::new();
    fields.insert("chain".to_owned(), Value::Array(chain));
    detail_line(
        timestamp,
        "ERROR",
        "viola::obs",
        ObsEvent::ProcessExit,
        process,
        instance,
        fields,
    )
}

pub(crate) fn detail_path(home: &Path, instance: &ViolaName, process: ObsProcess) -> PathBuf {
    home.join("instances")
        .join(name_str(instance))
        .join("diagnostics")
        .join(format!("detail-{}.ndjson", process.as_str()))
}

/// A content-bearing line for an instance detail file, formatted here and never through the
/// subscriber; `fields` follow the home-level keys.
pub(crate) fn detail_line(
    timestamp: &str,
    level: &str,
    target: &str,
    event: ObsEvent,
    process: ObsProcess,
    instance: &ViolaName,
    fields: Map<String, Value>,
) -> String {
    let mut record = Map::new();
    record.insert("timestamp".to_owned(), timestamp.into());
    record.insert("level".to_owned(), level.into());
    record.insert("target".to_owned(), target.into());
    record.insert("message".to_owned(), event.as_str().into());
    record.insert("event".to_owned(), event.as_str().into());
    record.insert("process".to_owned(), process.as_str().into());
    record.insert("instance".to_owned(), name_str(instance).into());
    record.extend(fields);
    let mut line = Value::Object(record).to_string();
    line.push('\n');
    line
}

/// One `write_all` into `<home>/instances/<name>/diagnostics/detail-<process>.ndjson`, opened at
/// first use; every failure drops the line and nothing reaches stderr.
pub(crate) fn write_detail(home: &Path, instance: &ViolaName, process: ObsProcess, line: &str) {
    let path = detail_path(home, instance, process);
    let Some(dir) = path.parent() else {
        return;
    };
    if ensure_private_dir(dir).is_err() {
        return;
    }
    if let Ok(mut file) = open_private_append(&path) {
        let _ = file.write_all(line.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone as _;
    use rstest::rstest;
    use std::sync::Mutex;

    fn name(raw: &str) -> ViolaName {
        ViolaName::try_new(raw.to_owned()).expect("valid")
    }

    #[test]
    fn timestamp_is_rfc3339_utc_millis_with_z() {
        let at = Utc
            .with_ymd_and_hms(2026, 9, 24, 6, 7, 8)
            .single()
            .expect("valid date")
            + chrono::Duration::milliseconds(9);
        assert_eq!(timestamp(at), "2026-09-24T06:07:08.009Z");
    }

    #[test]
    fn millis_utc_writes_the_current_timestamp() {
        let mut out = String::new();
        MillisUtc
            .format_time(&mut Writer::new(&mut out))
            .expect("format");
        assert_eq!(out.len(), "2026-09-24T06:07:08.009Z".len());
        assert!(out.ends_with('Z'));
        assert!(out.starts_with("20"));
    }

    #[test]
    fn ensure_private_dir_creates_missing_parents() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let dir = tmp.path().join("a").join("b");
        ensure_private_dir(&dir).expect("create");
        assert!(dir.is_dir());
        ensure_private_dir(&dir).expect("idempotent");
    }

    #[rstest]
    #[case::run(ObsProcess::Run, Some("builder"), None, Some("run-builder.ndjson"))]
    #[case::hook(ObsProcess::Hook, Some("builder"), None, Some("hook-builder.ndjson"))]
    #[case::cli(ObsProcess::Cli, Some("overseer"), None, Some("cli-overseer.ndjson"))]
    #[case::mcp(ObsProcess::Mcp, None, None, Some("mcp.ndjson"))]
    #[case::ui(ObsProcess::Ui, None, Some(47319), Some("ui-47319.ndjson"))]
    #[case::ui_without_port(ObsProcess::Ui, Some("builder"), None, None)]
    #[case::run_without_name(ObsProcess::Run, None, Some(1), None)]
    fn role_file_name_maps_every_process(
        #[case] process: ObsProcess,
        #[case] instance: Option<&str>,
        #[case] port: Option<u16>,
        #[case] expected: Option<&str>,
    ) {
        let instance = instance.map(name);
        assert_eq!(
            role_file_name(process, instance.as_ref(), port).as_deref(),
            expected
        );
    }

    #[test]
    fn open_role_file_without_a_name_creates_nothing() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        assert!(open_role_file(&home, ObsProcess::Run, None, None).is_err());
        assert!(!home.exists());
    }

    #[test]
    fn targets_enable_viola_crates_at_the_level_and_nothing_else() {
        let info = targets(Level::INFO);
        assert!(info.would_enable("viola::run", &Level::INFO));
        assert!(info.would_enable("viola_ui::sse", &Level::WARN));
        assert!(!info.would_enable("viola::run", &Level::DEBUG));
        assert!(!info.would_enable("rmcp::service", &Level::ERROR));
        let debug = targets(Level::DEBUG);
        assert!(debug.would_enable("viola_channel::server", &Level::DEBUG));
        assert!(!debug.would_enable("viola::run", &Level::TRACE));
    }

    fn level_of(config: Option<&str>) -> (Level, Option<ConfigRejection>) {
        let tmp = tempfile::tempdir().expect("tempdir");
        if let Some(config) = config {
            fs::write(tmp.path().join("config.json"), config).expect("write");
        }
        read_diagnostics_level(tmp.path())
    }

    #[rstest]
    #[case::absent_file(None, Level::INFO, None)]
    #[case::no_version(Some("{}"), Level::INFO, Some(ConfigRejection::Malformed))]
    #[case::version_only(Some(r#"{"v":1}"#), Level::INFO, None)]
    #[case::info(Some(r#"{"v":1,"diagnostics_level":"info"}"#), Level::INFO, None)]
    #[case::debug(Some(r#"{"v":1,"diagnostics_level":"debug"}"#), Level::DEBUG, None)]
    #[case::trace(
        Some(r#"{"v":1,"diagnostics_level":"trace"}"#),
        Level::INFO,
        Some(ConfigRejection::Malformed)
    )]
    #[case::not_a_string(
        Some(r#"{"v":1,"diagnostics_level":2}"#),
        Level::INFO,
        Some(ConfigRejection::Malformed)
    )]
    #[case::newer_version(
        Some(r#"{"v":2,"diagnostics_level":"debug"}"#),
        Level::INFO,
        Some(ConfigRejection::Malformed)
    )]
    #[case::not_an_object(Some("[1]"), Level::INFO, Some(ConfigRejection::Malformed))]
    #[case::not_json(Some("not json"), Level::INFO, Some(ConfigRejection::Malformed))]
    #[case::unknown_keys(
        Some(r#"{"v":1,"diagnostics_level":"debug","port":1,"x":true}"#),
        Level::DEBUG,
        Some(ConfigRejection::UnknownKeys(2))
    )]
    fn read_diagnostics_level_cases(
        #[case] config: Option<&str>,
        #[case] level: Level,
        #[case] rejection: Option<ConfigRejection>,
    ) {
        assert_eq!(level_of(config), (level, rejection));
    }

    #[test]
    fn read_diagnostics_level_unreadable_config_is_reported() {
        let tmp = tempfile::tempdir().expect("tempdir");
        fs::create_dir(tmp.path().join("config.json")).expect("a dir where the file should be");
        assert_eq!(
            read_diagnostics_level(tmp.path()),
            (Level::INFO, Some(ConfigRejection::Unreadable))
        );
    }

    /// Opening `<file>/config.json` fails at open with ENOTDIR (`NotADirectory`, not `NotFound`),
    /// so only the open-error arm yields `Unreadable`; the directory case above opens fine on Unix
    /// and fails at read instead.
    #[cfg(unix)]
    #[test]
    fn read_diagnostics_level_file_as_home_is_unreadable() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        fs::write(&home, "not a dir").expect("a file where the home should be");
        assert_eq!(
            read_diagnostics_level(&home),
            (Level::INFO, Some(ConfigRejection::Unreadable))
        );
    }

    #[rstest]
    #[case::unreadable(ConfigRejection::Unreadable, "unreadable", 1)]
    #[case::malformed(ConfigRejection::Malformed, "malformed", 1)]
    #[case::unknown_keys(ConfigRejection::UnknownKeys(3), "unknown-keys", 3)]
    fn config_rejection_detail_and_count(
        #[case] rejection: ConfigRejection,
        #[case] detail: &str,
        #[case] count: usize,
    ) {
        assert_eq!(rejection.detail(), detail);
        assert_eq!(rejection.count(), count);
    }

    #[test]
    fn detail_line_carries_the_home_level_keys_then_the_fields() {
        let mut fields = Map::new();
        fields.insert("panic_payload".to_owned(), "boom".into());
        let line = detail_line(
            "2026-09-24T06:00:00.000Z",
            "ERROR",
            "viola::panic",
            ObsEvent::Panic,
            ObsProcess::Run,
            &name("builder"),
            fields,
        );
        assert!(line.ends_with('\n'));
        assert_eq!(line.matches('\n').count(), 1);
        let v: Value = serde_json::from_str(&line).expect("json");
        assert_eq!(v["timestamp"], "2026-09-24T06:00:00.000Z");
        assert_eq!(v["level"], "ERROR");
        assert_eq!(v["target"], "viola::panic");
        assert_eq!(v["message"], "panic");
        assert_eq!(v["event"], "panic");
        assert_eq!(v["process"], "run");
        assert_eq!(v["instance"], "builder");
        assert_eq!(v["panic_payload"], "boom");
    }

    #[test]
    fn write_detail_creates_owner_only_dirs_and_one_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        fs::create_dir(&home).expect("home");
        let builder = name("builder");
        write_detail(&home, &builder, ObsProcess::Run, "{\"n\":1}\n");
        write_detail(&home, &builder, ObsProcess::Run, "{\"n\":2}\n");
        let path = detail_path(&home, &builder, ObsProcess::Run);
        assert!(
            path.ends_with(
                Path::new("instances")
                    .join("builder")
                    .join("diagnostics")
                    .join("detail-run.ndjson")
            )
        );
        assert_eq!(
            fs::read_to_string(&path).expect("detail"),
            "{\"n\":1}\n{\"n\":2}\n"
        );
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let mode = |p: &Path| fs::metadata(p).expect("meta").permissions().mode() & 0o777;
            assert_eq!(mode(&home.join("instances")), 0o700);
            assert_eq!(mode(&home.join("instances").join("builder")), 0o700);
            assert_eq!(mode(path.parent().expect("dir")), 0o700);
            assert_eq!(mode(&path), 0o600);
        }
    }

    #[test]
    fn write_detail_under_a_file_drops_the_line() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        fs::write(&home, "not a dir").expect("file");
        write_detail(&home, &name("builder"), ObsProcess::Run, "{}\n");
        assert_eq!(fs::read_to_string(&home).expect("file"), "not a dir");
    }

    #[derive(Clone, Default)]
    struct Buffer(Arc<Mutex<Vec<u8>>>);

    impl io::Write for Buffer {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0.lock().expect("lock").extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Buffer {
        fn lines(&self) -> Vec<Value> {
            let bytes = self.0.lock().expect("lock").clone();
            String::from_utf8(bytes)
                .expect("utf-8")
                .lines()
                .map(|l| serde_json::from_str(l).expect("one JSON object per line"))
                .collect()
        }
    }

    fn captured(level: Level, emit: impl FnOnce()) -> Vec<Value> {
        let buffer = Buffer::default();
        let writer = buffer.clone();
        tracing::subscriber::with_default(subscriber(move || writer.clone(), level), emit);
        buffer.lines()
    }

    fn canary_error() -> anyhow::Error {
        anyhow::anyhow!("parse failed near canary-chain-value-5c1e").context("dispatch failed")
    }

    /// The one test in this binary that sets `ProcessCtx` (nextest runs each test in its own
    /// process).
    #[test]
    fn internal_error_exit_line_writes_run_internal_error() {
        assert!(captured(Level::INFO, internal_error_exit_line).is_empty());
        viola_core::obs::set_ctx(ProcessCtx {
            process: ObsProcess::Run,
            instance: Some(name("builder")),
        });
        let lines = captured(Level::INFO, internal_error_exit_line);
        assert_eq!(lines.len(), 1);
        let line = &lines[0];
        assert_eq!(line["event"], "process-exit");
        assert_eq!(line["message"], "process-exit");
        assert_eq!(line["level"], "ERROR");
        assert_eq!(line["process"], "run");
        assert_eq!(line["instance"], "builder");
        assert_eq!(line["subject"], "self");
        assert_eq!(line["exit_code"], 1);
        assert_eq!(line["detail"], "internal-error");
        assert!(line["duration_ms"].as_u64().is_some());
        assert!(line.get("corr").is_none());

        let error = canary_error();
        let reported = captured(Level::INFO, || report_internal_error(&error, None));
        assert_eq!(reported.len(), 1);
        assert_eq!(reported[0]["detail"], "internal-error");
        assert!(reported[0].get("chain").is_none());
        assert!(!reported[0].to_string().contains("canary-chain-value-5c1e"));
    }

    #[test]
    fn internal_error_detail_line_carries_the_chain() {
        let error = canary_error();
        let line = chain_detail_line(
            "2026-09-24T06:00:00.000Z",
            ObsProcess::Run,
            &name("builder"),
            &error,
        );
        assert!(line.ends_with('\n'));
        assert_eq!(line.matches('\n').count(), 1);
        let v: Value = serde_json::from_str(&line).expect("json");
        assert_eq!(v["event"], "process-exit");
        assert_eq!(v["level"], "ERROR");
        assert_eq!(v["process"], "run");
        assert_eq!(v["instance"], "builder");
        assert_eq!(
            v["chain"],
            serde_json::json!([
                "dispatch failed",
                "parse failed near canary-chain-value-5c1e"
            ])
        );
        let schema: Value = serde_json::from_str(
            &fs::read_to_string(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/schemas/diag-detail.v1.json"
            ))
            .expect("schema"),
        )
        .expect("schema JSON");
        let validator = jsonschema::validator_for(&schema).expect("valid schema");
        assert!(validator.is_valid(&v));

        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        fs::create_dir(&home).expect("home");
        let sink = DetailSink {
            home: home.clone(),
            instance: name("builder"),
            process: ObsProcess::Run,
        };
        report_internal_error(&error, Some(&sink));
        let detail = fs::read_to_string(detail_path(&home, &sink.instance, ObsProcess::Run))
            .expect("detail file");
        assert_eq!(detail.lines().count(), 1);
        let d: Value = serde_json::from_str(detail.trim_end()).expect("json");
        assert_eq!(d["chain"], v["chain"]);
        assert!(!home.join("diagnostics").exists());
    }

    #[test]
    fn log_config_rejection_writes_one_warn_line() {
        let lines = captured(Level::INFO, || {
            log_config_rejection(ConfigRejection::UnknownKeys(2));
        });
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["event"], "parse-rejected");
        assert_eq!(lines[0]["level"], "WARN");
        assert_eq!(lines[0]["parser"], "config-json");
        assert_eq!(lines[0]["detail"], "unknown-keys");
        assert_eq!(lines[0]["count"], 2);
        assert!(lines[0].get("instance").is_none());
    }

    #[test]
    fn subscriber_level_gates_debug_lines_only() {
        let emit = || obs_event!(DEBUG, ObsEvent::StateRecovered, detail = "torn-line-healed");
        assert!(captured(Level::INFO, emit).is_empty());
        let lines = captured(Level::DEBUG, emit);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["level"], "DEBUG");
        assert!(
            lines[0]["timestamp"]
                .as_str()
                .is_some_and(|t| t.ends_with('Z') && t.len() == 24)
        );
    }
}
