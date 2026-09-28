mod cmd;
mod human;
mod obs;
mod run;

use std::ffi::OsString;
use std::fs::File;
use std::io::Write as _;
use std::panic::PanicHookInfo;
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;
use std::sync::{Arc, OnceLock};

use clap::Parser as _;
use serde_json::{Map, Value};
use viola_core::ViolaName;
use viola_core::obs::{ObsEvent, ObsProcess};

/// Where the panic hook writes: the role file of the process, opened before any work runs, and
/// the home its instance detail file lives under.
struct PanicSink {
    file: Arc<File>,
    process: ObsProcess,
    instance: Option<ViolaName>,
    home: PathBuf,
}

static PANIC_SINK: OnceLock<PanicSink> = OnceLock::new();

pub(crate) fn set_panic_sink(
    file: Arc<File>,
    process: ObsProcess,
    instance: Option<ViolaName>,
    home: PathBuf,
) {
    let _ = PANIC_SINK.set(PanicSink {
        file,
        process,
        instance,
        home,
    });
}

fn main() -> ExitCode {
    std::panic::set_hook(Box::new(viola_panic_hook));
    let role = role_of(std::env::args_os());
    let outcome = std::panic::catch_unwind(|| {
        let cli = match cmd::Cli::try_parse() {
            Ok(cli) => cli,
            Err(_) if role == Role::Hook => return Outcome::Unparsed,
            Err(usage) => usage.exit(),
        };
        match cmd::dispatch(cli) {
            Ok(code) => Outcome::Done(code),
            Err(failure) => {
                obs::report_internal_error(&failure.error, failure.sink.as_ref());
                Outcome::Failed
            }
        }
    })
    .unwrap_or_else(|_| {
        obs::internal_error_exit_line();
        Outcome::Panicked
    });
    if prints_internal_error(role, outcome) {
        human::internal_error();
    }
    exit_code(role, outcome)
}

/// The process role, read from argv before clap runs (obs-plan §7 main-thread catch site).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Role {
    Hook,
    /// A human verb whose failure prints `error: internal error` (`verify`).
    Cli,
    Other,
}

/// The first argument that is neither the global `--home` nor its value names the role, so
/// `viola --home <dir> hook stop` is still a `hook`.
fn role_of(args: impl IntoIterator<Item = OsString>) -> Role {
    let mut args = args.into_iter().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--home" {
            args.next();
        } else if !arg.to_str().is_some_and(|a| a.starts_with("--home=")) {
            return if arg == "hook" {
                Role::Hook
            } else if arg == "verify" {
                Role::Cli
            } else {
                Role::Other
            };
        }
    }
    Role::Other
}

/// How the catch site's work ended.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
    Done(ExitCode),
    /// A clap error, `--help` and `--version` included, for a role that prints nothing.
    Unparsed,
    Failed,
    Panicked,
}

/// `hook` exits 0 whatever happened (architecture [Hook Contract]); every other role keeps its
/// own code and exits 1 on a dispatch error or a caught panic.
fn exit_code(role: Role, outcome: Outcome) -> ExitCode {
    match (role, outcome) {
        (Role::Hook, _) => ExitCode::SUCCESS,
        (Role::Cli | Role::Other, Outcome::Done(code)) => code,
        (Role::Cli | Role::Other, Outcome::Unparsed | Outcome::Failed | Outcome::Panicked) => {
            ExitCode::from(1)
        }
    }
}

/// Only the `cli` role owns a stderr line at the catch site (obs-plan §7 Per-role behaviour); its
/// chain stays in the detail file.
fn prints_internal_error(role: Role, outcome: Outcome) -> bool {
    role == Role::Cli && matches!(outcome, Outcome::Failed | Outcome::Panicked)
}

/// Never calls the default hook and never writes stderr: one payload-free JSON line, one
/// `write_all`, into the role file; payload and backtrace go only to the instance detail file
/// (obs-plan §7).
fn viola_panic_hook(info: &PanicHookInfo<'_>) {
    let Some(sink) = PANIC_SINK.get() else {
        return;
    };
    let location = info
        .location()
        .map(|l| format!("{}:{}", panic_location(Path::new(l.file())), l.line()))
        .unwrap_or_default();
    let thread = std::thread::current()
        .name()
        .unwrap_or("<unnamed>")
        .to_owned();
    write_panic_lines(
        sink,
        &location,
        &thread,
        info.payload_as_str().unwrap_or("non-string payload"),
    );
}

/// The panic's two records for `sink`: the codes-only role line, then the payload and backtrace in
/// the instance detail file when the process has an instance.
fn write_panic_lines(sink: &PanicSink, location: &str, thread: &str, payload: &str) {
    let timestamp = obs::timestamp(chrono::Utc::now());
    let line = panic_line(
        &timestamp,
        sink.process.as_str(),
        sink.instance.as_ref().map(AsRef::as_ref),
        location,
        thread,
    );
    let _ = (&*sink.file).write_all(line.as_bytes());
    if let Some(instance) = &sink.instance {
        let detail = panic_detail_line(
            &timestamp,
            sink.process,
            instance,
            location,
            thread,
            payload,
        );
        obs::write_detail(&sink.home, instance, sink.process, &detail);
    }
}

fn panic_detail_line(
    timestamp: &str,
    process: ObsProcess,
    instance: &ViolaName,
    location: &str,
    thread: &str,
    payload: &str,
) -> String {
    // MEASUREMENT ONLY (Windows 1.0 s spine bound): the capture and render times ride as the last
    // backtrace string; removed once read.
    let since_init = obs::duration_ms();
    let started = std::time::Instant::now();
    let captured = std::backtrace::Backtrace::force_capture();
    let capture = started.elapsed();
    let rendered = captured.to_string();
    let render = started.elapsed();
    let mut backtrace: Vec<Value> = rendered
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(Value::from)
        .collect();
    backtrace.push(Value::from(format!(
        "measure: since_init_ms={since_init} capture={capture:?} capture_and_render={render:?} frames={}",
        backtrace.len()
    )));
    let mut fields = Map::new();
    fields.insert("panic_location".to_owned(), location.into());
    fields.insert("thread".to_owned(), thread.into());
    fields.insert("panic_payload".to_owned(), payload.into());
    fields.insert("backtrace".to_owned(), Value::Array(backtrace));
    obs::detail_line(
        timestamp,
        "ERROR",
        "viola::panic",
        ObsEvent::Panic,
        process,
        instance,
        fields,
    )
}

fn panic_line(
    timestamp: &str,
    process: &str,
    instance: Option<&str>,
    location: &str,
    thread: &str,
) -> String {
    let mut record = serde_json::json!({
        "timestamp": timestamp,
        "level": "ERROR",
        "target": "viola::panic",
        "message": "panic",
        "event": "panic",
        "process": process,
        "panic_location": location,
        "thread": thread,
    });
    if let Some(instance) = instance {
        record["instance"] = serde_json::Value::from(instance);
    }
    let mut line = record.to_string();
    line.push('\n');
    line
}

/// Workspace-relative for our own code, `<crate>-<ver>/src/...` for a dependency: an
/// absolute path would put the builder's home directory into the log.
fn panic_location(file: &Path) -> String {
    let parts: Vec<String> = file
        .components()
        .filter_map(|c| match c {
            Component::Normal(p) => Some(p.to_string_lossy().into_owned()),
            _ => None,
        })
        .collect();
    if !file.is_absolute() {
        return parts.join("/");
    }
    match parts.iter().position(|p| p == "src") {
        Some(src) if src > 0 => parts[src - 1..].join("/"),
        _ => parts.last().cloned().unwrap_or_default(),
    }
}

/// Helpers shared by this binary's test modules.
#[cfg(test)]
pub(crate) mod test_support {
    use serde_json::Value;

    /// One complete JSON line: a single `\n`, at the end.
    pub(crate) fn one_line(line: &str) -> Value {
        assert!(line.ends_with('\n'));
        assert_eq!(line.matches('\n').count(), 1);
        serde_json::from_str(line).expect("json")
    }

    /// The home-level keys of a diagnostics line; `message` is the event name.
    pub(crate) fn assert_home_level(
        v: &Value,
        timestamp: &str,
        level: &str,
        target: &str,
        event: &str,
        process: &str,
        instance: &str,
    ) {
        assert_eq!(v["timestamp"], timestamp);
        assert_eq!(v["level"], level);
        assert_eq!(v["target"], target);
        assert_eq!(v["message"], event);
        assert_eq!(v["event"], event);
        assert_eq!(v["process"], process);
        assert_eq!(v["instance"], instance);
    }

    pub(crate) fn diag_detail_validator() -> jsonschema::Validator {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/schemas/diag-detail.v1.json");
        let text = std::fs::read_to_string(path).expect("schema");
        let schema: Value = serde_json::from_str(&text).expect("schema JSON");
        jsonschema::validator_for(&schema).expect("valid schema")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::{assert_home_level, diag_detail_validator, one_line};

    use rstest::rstest;

    fn argv(args: &[&str]) -> Vec<OsString> {
        std::iter::once("viola")
            .chain(args.iter().copied())
            .map(OsString::from)
            .collect()
    }

    #[rstest]
    #[case::bare(&["hook", "stop"], Role::Hook)]
    #[case::home_first(&["--home", "C:/h", "hook", "stop"], Role::Hook)]
    #[case::home_equals(&["--home=C:/h", "hook", "stop"], Role::Hook)]
    #[case::home_named_hook(&["--home", "hook", "run", "x"], Role::Other)]
    #[case::run(&["run", "builder", "--", "claude"], Role::Other)]
    #[case::run_after_home(&["--home", "C:/h", "run", "b"], Role::Other)]
    #[case::help(&["--help"], Role::Other)]
    #[case::nothing(&[], Role::Other)]
    #[case::home_only(&["--home", "C:/h"], Role::Other)]
    #[case::later_hook(&["run", "hook"], Role::Other)]
    #[case::verify(&["verify", "--", "claude"], Role::Cli)]
    #[case::verify_after_home(&["--home=C:/h", "verify"], Role::Cli)]
    #[case::later_verify(&["run", "verify"], Role::Other)]
    fn role_of_skips_the_home_flag_and_its_value(#[case] args: &[&str], #[case] role: Role) {
        assert_eq!(role_of(argv(args)), role);
    }

    #[rstest]
    #[case::hook_done(Role::Hook, Outcome::Done(ExitCode::from(3)), ExitCode::SUCCESS)]
    #[case::hook_unparsed(Role::Hook, Outcome::Unparsed, ExitCode::SUCCESS)]
    #[case::hook_failed(Role::Hook, Outcome::Failed, ExitCode::SUCCESS)]
    #[case::hook_panicked(Role::Hook, Outcome::Panicked, ExitCode::SUCCESS)]
    #[case::run_done(Role::Other, Outcome::Done(ExitCode::from(3)), ExitCode::from(3))]
    #[case::run_unparsed(Role::Other, Outcome::Unparsed, ExitCode::from(1))]
    #[case::run_failed(Role::Other, Outcome::Failed, ExitCode::from(1))]
    #[case::run_panicked(Role::Other, Outcome::Panicked, ExitCode::from(1))]
    #[case::cli_done(Role::Cli, Outcome::Done(ExitCode::from(3)), ExitCode::from(3))]
    #[case::cli_unparsed(Role::Cli, Outcome::Unparsed, ExitCode::from(1))]
    #[case::cli_failed(Role::Cli, Outcome::Failed, ExitCode::from(1))]
    #[case::cli_panicked(Role::Cli, Outcome::Panicked, ExitCode::from(1))]
    fn exit_code_maps_role_and_outcome(
        #[case] role: Role,
        #[case] outcome: Outcome,
        #[case] code: ExitCode,
    ) {
        assert_eq!(exit_code(role, outcome), code);
    }

    #[rstest]
    #[case::cli_failed(Role::Cli, Outcome::Failed, true)]
    #[case::cli_panicked(Role::Cli, Outcome::Panicked, true)]
    #[case::cli_done(Role::Cli, Outcome::Done(ExitCode::from(1)), false)]
    #[case::cli_unparsed(Role::Cli, Outcome::Unparsed, false)]
    #[case::run_failed(Role::Other, Outcome::Failed, false)]
    #[case::run_panicked(Role::Other, Outcome::Panicked, false)]
    #[case::hook_failed(Role::Hook, Outcome::Failed, false)]
    #[case::hook_panicked(Role::Hook, Outcome::Panicked, false)]
    fn prints_internal_error_only_for_a_cli_failure(
        #[case] role: Role,
        #[case] outcome: Outcome,
        #[case] prints: bool,
    ) {
        assert_eq!(prints_internal_error(role, outcome), prints);
    }

    /// The hook role's panic records, driven through the writer the panic hook calls (no second
    /// test sets the process-global sink): one payload-free role line, and the payload with its
    /// backtrace only in `detail-hook.ndjson`.
    #[test]
    fn panic_lines_for_a_hook_split_the_payload_into_the_detail_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let home = dir.path().join("home");
        std::fs::create_dir(&home).expect("home");
        let path = dir.path().join("hook-builder.ndjson");
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .expect("open");
        let builder = ViolaName::try_new("builder".to_owned()).expect("valid");
        let sink = PanicSink {
            file: Arc::new(file),
            process: ObsProcess::Hook,
            instance: Some(builder.clone()),
            home: home.clone(),
        };
        write_panic_lines(&sink, "src/cmd/hook.rs:9", "main", "hook-payload-text");

        let text = std::fs::read_to_string(&path).expect("read");
        let v = one_line(&text);
        assert_eq!(v["event"], "panic");
        assert_eq!(v["process"], "hook");
        assert_eq!(v["instance"], "builder");
        assert_eq!(v["panic_location"], "src/cmd/hook.rs:9");
        assert!(!text.contains("hook-payload-text"));
        assert!(v.get("backtrace").is_none());

        let detail_path = obs::detail_path(&home, &builder, ObsProcess::Hook);
        assert!(detail_path.ends_with("detail-hook.ndjson"));
        let d = one_line(&std::fs::read_to_string(&detail_path).expect("detail file"));
        assert_eq!(d["event"], "panic");
        assert_eq!(d["process"], "hook");
        assert_eq!(d["panic_payload"], "hook-payload-text");
        assert_eq!(d["thread"], "main");
        assert!(d["backtrace"].as_array().is_some_and(|b| !b.is_empty()));
        assert!(diag_detail_validator().is_valid(&d));
    }

    #[test]
    fn panic_lines_without_an_instance_write_only_the_role_line() {
        let dir = tempfile::tempdir().expect("tempdir");
        let home = dir.path().join("home");
        let path = dir.path().join("role.ndjson");
        let file = std::fs::File::create(&path).expect("create");
        let sink = PanicSink {
            file: Arc::new(file),
            process: ObsProcess::Hook,
            instance: None,
            home: home.clone(),
        };
        write_panic_lines(&sink, "l", "t", "p");
        let v = one_line(&std::fs::read_to_string(&path).expect("read"));
        assert!(v.get("instance").is_none());
        assert!(!home.exists(), "no detail file without an instance");
    }

    #[test]
    fn panic_location_relative_path_kept_with_forward_slashes() {
        let p = Path::new("src").join("run").join("mod.rs");
        assert_eq!(panic_location(&p), "src/run/mod.rs");
    }

    #[test]
    fn panic_location_dependency_path_trimmed_to_crate_dir() {
        let base = std::env::temp_dir();
        let p = base
            .join("registry")
            .join("serde-1.0.229")
            .join("src")
            .join("de.rs");
        assert_eq!(panic_location(&p), "serde-1.0.229/src/de.rs");
    }

    #[test]
    fn panic_location_src_at_root_keeps_file_name_only() {
        let root = std::env::temp_dir()
            .ancestors()
            .last()
            .expect("a root")
            .to_path_buf();
        let p = root.join("src").join("x.rs");
        assert_eq!(panic_location(&p), "x.rs");
    }

    #[test]
    fn panic_location_absolute_without_src_keeps_file_name_only() {
        let p = std::env::temp_dir().join("elsewhere").join("x.rs");
        assert_eq!(panic_location(&p), "x.rs");
    }

    #[test]
    fn panic_line_has_the_binding_fields_and_no_corr() {
        let line = panic_line(
            "2026-09-24T06:00:00.000Z",
            "run",
            Some("builder"),
            "src/x.rs:3",
            "main",
        );
        let v = one_line(&line);
        assert_home_level(
            &v,
            "2026-09-24T06:00:00.000Z",
            "ERROR",
            "viola::panic",
            "panic",
            "run",
            "builder",
        );
        assert_eq!(v["panic_location"], "src/x.rs:3");
        assert_eq!(v["thread"], "main");
        assert!(v.get("corr").is_none());
    }

    #[test]
    fn panic_line_without_instance_omits_the_key() {
        let line = panic_line("t", "run", None, "l", "main");
        let v: serde_json::Value = serde_json::from_str(&line).expect("json");
        assert!(v.get("instance").is_none());
    }

    /// The role file keeps one payload-free line; payload and backtrace land only in the
    /// instance detail file, which validates against its committed schema.
    #[test]
    fn panic_hook_writes_detail_line_with_payload_and_backtrace() {
        let dir = tempfile::tempdir().expect("tempdir");
        let home = dir.path().join("home");
        std::fs::create_dir(&home).expect("home");
        let path = dir.path().join("run-builder.ndjson");
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .expect("open");
        let builder = ViolaName::try_new("builder".to_owned()).expect("valid");
        set_panic_sink(
            Arc::new(file),
            ObsProcess::Run,
            Some(builder.clone()),
            home.clone(),
        );
        std::panic::set_hook(Box::new(viola_panic_hook));
        let caught = std::panic::catch_unwind(|| panic!("secret-payload-text"));
        let _ = std::panic::take_hook();
        assert!(caught.is_err());

        let text = std::fs::read_to_string(&path).expect("read");
        assert_eq!(text.lines().count(), 1);
        assert!(!text.contains("secret-payload-text"));
        let v: Value = serde_json::from_str(text.trim_end()).expect("json");
        assert_eq!(v["event"], "panic");
        assert_eq!(v["instance"], "builder");
        assert!(v.get("backtrace").is_none());
        assert!(
            v["panic_location"]
                .as_str()
                .is_some_and(|l| l.starts_with("src/main.rs:"))
        );

        let detail_path = obs::detail_path(&home, &builder, ObsProcess::Run);
        let detail_text = std::fs::read_to_string(&detail_path).expect("detail file");
        assert_eq!(detail_text.lines().count(), 1);
        let d: Value = serde_json::from_str(detail_text.trim_end()).expect("json");
        assert_eq!(d["event"], "panic");
        assert_eq!(d["level"], "ERROR");
        assert_eq!(d["target"], "viola::panic");
        assert_eq!(d["instance"], "builder");
        assert_eq!(d["panic_payload"], "secret-payload-text");
        assert_eq!(d["panic_location"], v["panic_location"]);
        assert_eq!(d["thread"], v["thread"]);
        assert!(
            d["backtrace"]
                .as_array()
                .is_some_and(|frames| !frames.is_empty() && frames.iter().all(Value::is_string))
        );
        assert!(diag_detail_validator().is_valid(&d));
    }
}
