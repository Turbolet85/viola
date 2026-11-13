//! The `agents --json` mode and the live registry row each interactive fake agent keeps
//! beside its executable.

use super::*;

/// One live interactive fake agent's row beside its executable, in the measured `claude agents
/// --json` shape: rewritten whole on each status change and removed at exit. `startedAt` is the
/// process's start time, which the listing compares, so a dead pid or a reused one is left out.
pub(super) struct Registry {
    path: PathBuf,
    row: Mutex<Value>,
}

impl Registry {
    pub(super) fn open(dir: &Path, session_id: &str, cwd: &str) -> Option<Self> {
        let pid = std::process::id();
        let started = process_start_time(pid)?;
        let sessions = dir.join(SESSIONS);
        fs::create_dir_all(&sessions).ok()?;
        let registry = Self {
            path: sessions.join(format!("{pid}.json")),
            row: Mutex::new(json!({
                "cwd": cwd,
                "kind": "interactive",
                "name": SESSION_NAME,
                "pid": pid,
                "sessionId": session_id,
                "startedAt": started.saturating_mul(1000),
                "status": "idle",
            })),
        };
        registry.set_status("idle");
        Some(registry)
    }

    /// The row with `status`, written to a sibling and renamed over the old one: a listing never
    /// reads half a row.
    pub(super) fn set_status(&self, status: &str) {
        let mut row = self.row.lock().unwrap_or_else(PoisonError::into_inner);
        row["status"] = json!(status);
        let next = self.path.with_extension("json.next");
        if fs::write(&next, row.to_string()).is_ok() {
            let _ = fs::rename(&next, &self.path);
        }
    }

    pub(super) fn remove(&self) {
        let _row = self.row.lock().unwrap_or_else(PoisonError::into_inner);
        let _ = fs::remove_file(&self.path);
    }
}

/// The directory holding this executable: where the `agents` mode files and the registry live.
pub(super) fn exe_dir() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()?
        .parent()
        .map(Path::to_path_buf)
}

/// The `agents --json` mode: what [`agents_output`] gives for the mode file beside the executable,
/// on stdout, exit 0.
pub(super) fn agents() -> ExitCode {
    let dir = exe_dir();
    let mode = dir
        .as_deref()
        .and_then(|d| fs::read_to_string(d.join(AGENTS_MODE)).ok());
    let out = agents_output(mode.as_deref().map(str::trim), dir.as_deref());
    let mut stdout = std::io::stdout().lock();
    let _ = stdout.write_all(&out).and_then(|()| stdout.flush());
    ExitCode::SUCCESS
}

/// `oversize`: one byte past the frame cap; `malformed`: a text that is not JSON; anything else:
/// one JSON array of the planted rows, then each live registry row.
pub(super) fn agents_output(mode: Option<&str>, dir: Option<&Path>) -> Vec<u8> {
    match mode {
        Some("oversize") => vec![b' '; usize::try_from(MAX_FRAME).unwrap_or(usize::MAX - 1) + 1],
        Some("malformed") => b"No sessions\n".to_vec(),
        _ => {
            let mut rows = dir.map(planted_rows).unwrap_or_default();
            rows.extend(dir.map(live_sessions).unwrap_or_default());
            let mut text = Value::Array(rows).to_string();
            text.push('\n');
            text.into_bytes()
        }
    }
}

/// The rows of `<dir>/agents-rows.json`; a file that is absent or no JSON array gives none.
pub(super) fn planted_rows(dir: &Path) -> Vec<Value> {
    match fs::read(dir.join(AGENTS_ROWS)).map(|b| serde_json::from_slice::<Value>(&b)) {
        Ok(Ok(Value::Array(rows))) => rows,
        _ => Vec::new(),
    }
}

/// The registry rows under `<dir>/sessions/` whose process is alive with the recorded start time,
/// in file-name order. The process table is read once, whatever the number of rows (rows of killed
/// agents stay behind); no signal is sent.
pub(super) fn live_sessions(dir: &Path) -> Vec<Value> {
    let mut paths: Vec<PathBuf> = fs::read_dir(dir.join(SESSIONS))
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|e| e == "json"))
        .collect();
    paths.sort();
    let table = start_times();
    paths
        .into_iter()
        .filter_map(|path| serde_json::from_slice::<Value>(&fs::read(path).ok()?).ok())
        .filter(|row| {
            let pid = row["pid"].as_u64().and_then(|p| u32::try_from(p).ok());
            let started = row["startedAt"].as_u64().map(|ms| ms / 1000);
            pid.zip(started)
                .is_some_and(|(pid, started)| table.get(&pid) == Some(&started))
        })
        .collect()
}
