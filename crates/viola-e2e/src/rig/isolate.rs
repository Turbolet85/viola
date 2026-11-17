//! The rig's isolation from the user's own Claude config and login: a 0700 scratch root outside
//! every repository holding the start's cwd, a private config dir and (wrapped) a viola home; a
//! credentials copy placed only inside a token window and removed on every exit path; the user's
//! `~/.claude.json` stamped before and after; a child environment built from nothing.

use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{self, Read as _};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};
use viola_core::MAX_FRAME;
use viola_state::fs::{FILE_MODE, create_private_dir, place_private_new, replace_private};
use viola_state::pin::content_key;

use super::procs::{self, MARKER, Scope};

/// The least token time a start needs left, so no refresh falls inside it.
pub const TOKEN_FLOOR: Duration = Duration::from_secs(20 * 60);

/// The host names a child keeps; every other name is absent from its environment.
const KEPT: [&str; 7] = [
    "HOME",
    "USER",
    "LOGNAME",
    "SHELL",
    "PATH",
    "LANG",
    "XDG_RUNTIME_DIR",
];

/// The default scratch base: under `/tmp`, outside every repository, one per user.
pub fn default_scratch_base() -> PathBuf {
    // SAFETY: geteuid reads the caller's id and cannot fail.
    let uid = unsafe { libc::geteuid() };
    PathBuf::from(format!("/tmp/viola-rig-{uid}"))
}

/// The credentials file read through `MAX_FRAME`.
pub fn read_credentials(path: &Path) -> io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    File::open(path)?.take(MAX_FRAME).read_to_end(&mut bytes)?;
    Ok(bytes)
}

/// Whether the credentials hold at least [`TOKEN_FLOOR`] of token time after `now_ms`: only
/// `claudeAiOauth.expiresAt` is read, and an absent or non-numeric value holds none.
pub fn token_window_open(bytes: &[u8], now_ms: u128) -> bool {
    let Ok(doc) = serde_json::from_slice::<Value>(bytes) else {
        return false;
    };
    let Some(expires) = doc["claudeAiOauth"]["expiresAt"].as_u64() else {
        return false;
    };
    u128::from(expires) >= now_ms + TOKEN_FLOOR.as_millis()
}

pub fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_millis())
}

/// The user config's stamp: its sha256 prefix and mtime, the bytes never kept.
pub type Stamp = Option<(String, SystemTime)>;

pub fn stamp(path: &Path) -> Stamp {
    let mut bytes = Vec::new();
    let file = File::open(path).ok()?;
    let mtime = file.metadata().and_then(|m| m.modified()).ok()?;
    file.take(MAX_FRAME).read_to_end(&mut bytes).ok()?;
    Some((content_key(&bytes), mtime))
}

/// `unchanged` · `changed` · `absent` (absent before and after).
pub fn compare(before: &Stamp, after: &Stamp) -> &'static str {
    match (before, after) {
        (None, None) => "absent",
        (a, b) if a == b => "unchanged",
        _ => "changed",
    }
}

/// The child's environment, built from nothing: the kept host names, `TERM`, the private config
/// dir and the run marker; `cli_dir` first on `PATH`.
pub fn child_env(cli_dir: Option<&Path>, config: &Path, run: &str) -> Vec<(OsString, OsString)> {
    let mut env: Vec<(OsString, OsString)> = KEPT
        .iter()
        .filter_map(|&name| std::env::var_os(name).map(|v| (OsString::from(name), v)))
        .collect();
    if let Some(dir) = cli_dir {
        let host = std::env::var_os("PATH").unwrap_or_default();
        let mut path = dir.as_os_str().to_owned();
        if !host.is_empty() {
            path.push(":");
            path.push(host);
        }
        env.retain(|(k, _)| k != "PATH");
        env.push(("PATH".into(), path));
    }
    env.push(("TERM".into(), "xterm-256color".into()));
    env.push(("CLAUDE_CONFIG_DIR".into(), config.into()));
    env.push((MARKER.into(), run.into()));
    env
}

/// Every host name the child must not inherit (the pty seam starts from the rig's own env).
pub fn removed_names(env: &[(OsString, OsString)]) -> Vec<OsString> {
    std::env::vars_os()
        .map(|(k, _)| k)
        .filter(|k| !env.iter().any(|(kept, _)| kept == k))
        .collect()
}

/// What the teardown read, each a closed word.
#[derive(Debug, Clone)]
pub struct Teardown {
    pub census: String,
    pub credentials: &'static str,
    pub scratch: &'static str,
}

/// One start's scratch root. Its teardown runs on every exit path, a panic's included (`Drop`).
pub struct Scratch {
    pub root: PathBuf,
    pub cwd: PathBuf,
    pub config: PathBuf,
    pub vhome: Option<PathBuf>,
    run: String,
    credentials: Option<PathBuf>,
    torn_down: Option<Teardown>,
}

impl Scratch {
    pub fn create(base: &Path, run: &str, wrapped: bool) -> io::Result<Self> {
        let root = base.join(run);
        create_private_dir(&root)?;
        let mut scratch = Self {
            cwd: root.join("cwd"),
            config: root.join("config"),
            vhome: wrapped.then(|| root.join("vhome")),
            root,
            run: run.to_owned(),
            credentials: None,
            torn_down: None,
        };
        create_private_dir(&scratch.cwd)?;
        create_private_dir(&scratch.config)?;
        if let Some(vhome) = scratch.vhome.clone() {
            create_private_dir(&vhome)?;
            // `viola run` strips every CLAUDE* name from its child unless the home keeps it.
            let config = json!({"v": 1, "claude_env_keep": ["CLAUDE_CONFIG_DIR"]});
            write_private(&vhome.join("config.json"), &config)?;
        }
        scratch.cwd = fs::canonicalize(&scratch.cwd)?;
        scratch.config = fs::canonicalize(&scratch.config)?;
        if let Some(vhome) = scratch.vhome.take() {
            scratch.vhome = Some(fs::canonicalize(vhome)?);
        }
        Ok(scratch)
    }

    /// The credentials bytes placed 0600, mode first; never read again.
    pub fn place_credentials(&mut self, bytes: &[u8]) -> io::Result<()> {
        let path = self.config.join(".credentials.json");
        self.credentials = Some(path.clone());
        place_private_new(&path, bytes, FILE_MODE)
            .map(|_| ())
            .map_err(|_| io::Error::other("credentials copy failed"))
    }

    /// The private config: onboarding complete, and the start's cwd trusted when asked.
    pub fn seed_config(&self, trusted: bool) -> io::Result<()> {
        let mut seed = json!({"hasCompletedOnboarding": true});
        if trusted {
            let mut projects = serde_json::Map::new();
            projects.insert(
                self.cwd.to_string_lossy().into_owned(),
                json!({"hasTrustDialogAccepted": true}),
            );
            seed["projects"] = Value::Object(projects);
        }
        write_private(&self.config.join(".claude.json"), &seed)
    }

    /// Census → the credentials copy removed → the scratch root removed → read absent.
    pub fn teardown(&mut self) -> Teardown {
        if let Some(done) = &self.torn_down {
            return done.clone();
        }
        let (killed, left) = procs::sweep(Scope::Run {
            run: &self.run,
            root: &self.root,
        });
        let census = match (killed, left) {
            (0, 0) => "0".to_owned(),
            (k, 0) => format!("{k} killed"),
            (k, l) => format!("{k} killed, {l} left"),
        };
        let credentials = match &self.credentials {
            None => "none",
            Some(path) => {
                let _ = fs::remove_file(path);
                if path.symlink_metadata().is_ok() {
                    "PRESENT"
                } else {
                    "removed"
                }
            }
        };
        let _ = fs::remove_dir_all(&self.root);
        let scratch = if self.root.symlink_metadata().is_ok() {
            "PRESENT"
        } else {
            "absent"
        };
        let done = Teardown {
            census,
            credentials,
            scratch,
        };
        self.torn_down = Some(done.clone());
        done
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        self.teardown();
    }
}

fn write_private(path: &Path, doc: &Value) -> io::Result<()> {
    replace_private(path, doc.to_string().as_bytes(), FILE_MODE)
        .map_err(|_| io::Error::other("private write failed"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn creds(expires_ms: u128) -> Vec<u8> {
        format!(r#"{{"claudeAiOauth":{{"accessToken":"t","expiresAt":{expires_ms}}}}}"#)
            .into_bytes()
    }

    #[test]
    fn the_token_window_holds_at_twenty_minutes_and_one_second_and_not_below() {
        let now = 1_000_000_000_000;
        let floor = TOKEN_FLOOR.as_millis();
        assert!(token_window_open(&creds(now + floor + 1000), now));
        assert!(token_window_open(&creds(now + floor), now));
        assert!(!token_window_open(&creds(now + floor - 1000), now));
    }

    #[test]
    fn absent_or_non_numeric_expiry_holds_no_window() {
        assert!(!token_window_open(b"{}", 0));
        assert!(!token_window_open(
            br#"{"claudeAiOauth":{"expiresAt":"soon"}}"#,
            0
        ));
        assert!(!token_window_open(b"not json", 0));
    }

    #[test]
    fn the_child_env_keeps_only_the_named_hosts_names_and_adds_its_own() {
        let env = child_env(Some(Path::new("/cli")), Path::new("/cfg"), "1-2");
        let names: Vec<String> = env
            .iter()
            .map(|(k, _)| k.to_string_lossy().into_owned())
            .collect();
        for name in &names {
            assert!(
                KEPT.contains(&name.as_str())
                    || ["TERM", "CLAUDE_CONFIG_DIR", MARKER].contains(&name.as_str()),
                "{name} kept"
            );
        }
        let get = |n: &str| env.iter().find(|(k, _)| k == n).map(|(_, v)| v.clone());
        assert_eq!(get("TERM"), Some("xterm-256color".into()));
        assert_eq!(get("CLAUDE_CONFIG_DIR"), Some("/cfg".into()));
        assert_eq!(get(MARKER), Some("1-2".into()));
        assert!(
            get("PATH").is_some_and(|p| p.to_string_lossy().starts_with("/cli")),
            "the CLI dir first"
        );
        assert!(!names.iter().any(|n| n.starts_with("VIOLA_")));
    }

    #[test]
    fn the_stamps_compare_to_their_three_words() {
        let t = SystemTime::UNIX_EPOCH;
        let a = Some(("a".to_owned(), t));
        let b = Some(("b".to_owned(), t));
        assert_eq!(compare(&None, &None), "absent");
        assert_eq!(compare(&a, &a.clone()), "unchanged");
        assert_eq!(compare(&a, &b), "changed");
        assert_eq!(compare(&a, &None), "changed");
    }
}
