mod cmd;
mod run;

use std::fs::File;
use std::io::Write as _;
use std::panic::PanicHookInfo;
use std::path::{Component, Path};
use std::process::ExitCode;
use std::sync::{Arc, OnceLock};

use clap::Parser as _;

/// Where the panic hook writes: the role file of the process, opened before any work runs.
struct PanicSink {
    file: Arc<File>,
    process: &'static str,
    instance: Option<String>,
}

static PANIC_SINK: OnceLock<PanicSink> = OnceLock::new();

pub(crate) fn set_panic_sink(file: Arc<File>, process: &'static str, instance: Option<String>) {
    let _ = PANIC_SINK.set(PanicSink {
        file,
        process,
        instance,
    });
}

fn main() -> ExitCode {
    std::panic::set_hook(Box::new(viola_panic_hook));
    std::panic::catch_unwind(|| match cmd::dispatch(cmd::Cli::parse()) {
        Ok(code) => code,
        Err(_) => ExitCode::from(1),
    })
    .unwrap_or(ExitCode::from(1))
}

/// Never calls the default hook and never writes stderr: one JSON line, one `write_all`,
/// into the role file (obs-plan §7). The payload is content and stays out of this line.
fn viola_panic_hook(info: &PanicHookInfo<'_>) {
    let Some(sink) = PANIC_SINK.get() else {
        return;
    };
    let location = info
        .location()
        .map(|l| format!("{}:{}", panic_location(Path::new(l.file())), l.line()))
        .unwrap_or_default();
    let line = panic_line(
        &run::timestamp(chrono::Utc::now()),
        sink.process,
        sink.instance.as_deref(),
        &location,
        std::thread::current().name().unwrap_or("<unnamed>"),
    );
    let _ = (&*sink.file).write_all(line.as_bytes());
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(line.ends_with('\n'));
        assert_eq!(line.matches('\n').count(), 1);
        let v: serde_json::Value = serde_json::from_str(&line).expect("json");
        assert_eq!(v["level"], "ERROR");
        assert_eq!(v["target"], "viola::panic");
        assert_eq!(v["message"], "panic");
        assert_eq!(v["event"], "panic");
        assert_eq!(v["process"], "run");
        assert_eq!(v["instance"], "builder");
        assert_eq!(v["panic_location"], "src/x.rs:3");
        assert_eq!(v["thread"], "main");
        assert_eq!(v["timestamp"], "2026-09-24T06:00:00.000Z");
        assert!(v.get("corr").is_none());
    }

    #[test]
    fn panic_line_without_instance_omits_the_key() {
        let line = panic_line("t", "run", None, "l", "main");
        let v: serde_json::Value = serde_json::from_str(&line).expect("json");
        assert!(v.get("instance").is_none());
    }

    #[test]
    fn panic_hook_writes_exactly_one_line_without_payload() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("run-builder.ndjson");
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .expect("open");
        set_panic_sink(Arc::new(file), "run", Some("builder".to_owned()));
        std::panic::set_hook(Box::new(viola_panic_hook));
        let caught = std::panic::catch_unwind(|| panic!("secret-payload-text"));
        let _ = std::panic::take_hook();
        assert!(caught.is_err());
        let text = std::fs::read_to_string(&path).expect("read");
        assert_eq!(text.lines().count(), 1);
        assert!(!text.contains("secret-payload-text"));
        let v: serde_json::Value = serde_json::from_str(text.trim_end()).expect("json");
        assert_eq!(v["event"], "panic");
        assert_eq!(v["instance"], "builder");
        assert!(
            v["panic_location"]
                .as_str()
                .is_some_and(|l| l.starts_with("src/main.rs:"))
        );
    }
}
