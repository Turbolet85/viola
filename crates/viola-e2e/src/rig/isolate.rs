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

/// The user config's word: `unchanged` / `absent` while the stamp held; past a moved stamp,
/// `unread` with no bytes read, `changed-by-rig` when the read holds one of the rig's own needles,
/// else `changed-by-others`. The bytes are searched and nothing of them is kept.
pub fn witness(
    before: &Stamp,
    after: &Stamp,
    read: Option<&[u8]>,
    needles: &[&[u8]],
) -> &'static str {
    let held = compare(before, after);
    if held != "changed" {
        return held;
    }
    let Some(bytes) = read else {
        return "unread";
    };
    if needles.iter().any(|needle| holds(bytes, needle)) {
        "changed-by-rig"
    } else {
        "changed-by-others"
    }
}

fn holds(bytes: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty() && bytes.windows(needle.len()).any(|w| w == needle)
}

/// The user config read once after a moved stamp, through `MAX_FRAME`: `None` when absent,
/// unreadable, over the bound, or a symbolic link whose target lies outside `home`.
pub fn read_moved(path: &Path, home: &Path) -> Option<Vec<u8>> {
    if fs::symlink_metadata(path).ok()?.file_type().is_symlink() {
        let target = fs::canonicalize(path).ok()?;
        if !target.starts_with(fs::canonicalize(home).ok()?) {
            return None;
        }
    }
    let mut bytes = Vec::new();
    File::open(path)
        .ok()?
        .take(MAX_FRAME + 1)
        .read_to_end(&mut bytes)
        .ok()?;
    u64::try_from(bytes.len())
        .is_ok_and(|n| n <= MAX_FRAME)
        .then_some(bytes)
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

    /// The capture file a wake start's hook appends to.
    pub fn capture_file(&self) -> PathBuf {
        self.root.join("capture.ndjson")
    }

    /// The private config's `settings.json`: one UserPromptSubmit hook running `rig capture --out
    /// <root>/capture.ndjson`. Both paths are absolute and hold no shell metacharacter, else nothing
    /// is written and the seed fails.
    pub fn seed_capture(&self, rig: &Path) -> io::Result<()> {
        let out = self.capture_file();
        if !shell_plain(rig) || !shell_plain(&out) {
            return Err(io::Error::other("capture hook path refused"));
        }
        let command = format!("{} capture --out {}", rig.display(), out.display());
        let settings = json!({
            "hooks": {
                "UserPromptSubmit": [
                    {"hooks": [{"type": "command", "command": command}]}
                ]
            }
        });
        write_private(&self.config.join("settings.json"), &settings)
    }

    /// The imports seed: a file under `<root>/outside/` (outside the cwd, inside the scratch root)
    /// and the cwd's `CLAUDE.md` whose one line imports it.
    pub fn seed_imports(&self) -> io::Result<()> {
        let outside = self.root.join("outside");
        create_private_dir(&outside)?;
        let outside = fs::canonicalize(outside)?;
        let imported = outside.join("imported.md");
        replace_private(&imported, b"Reply tersely.\n", FILE_MODE)
            .map_err(|_| io::Error::other("imports seed failed"))?;
        let line = format!("@{}\n", imported.display());
        replace_private(&self.cwd.join("CLAUDE.md"), line.as_bytes(), FILE_MODE)
            .map_err(|_| io::Error::other("imports seed failed"))
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

/// An absolute path a hook command may name unquoted: `[A-Za-z0-9/._-]` only.
fn shell_plain(path: &Path) -> bool {
    path.is_absolute()
        && path.to_str().is_some_and(|s| {
            s.bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'/' | b'.' | b'_' | b'-'))
        })
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
    fn a_hook_path_is_plain_only_when_absolute_and_metacharacter_free() {
        assert!(shell_plain(Path::new(
            "/tmp/viola-rig-1000/12-34/capture.ndjson"
        )));
        assert!(!shell_plain(Path::new("rel/capture.ndjson")));
        assert!(!shell_plain(Path::new("/tmp/a b")));
        assert!(!shell_plain(Path::new("/tmp/a;rm")));
        assert!(!shell_plain(Path::new("/tmp/$HOME")));
    }

    #[test]
    fn the_capture_seed_registers_one_prompt_hook_in_the_private_config() {
        let base = tempfile::tempdir().expect("tempdir");
        let scratch = Scratch::create(base.path(), "1-2", true).expect("scratch");
        scratch
            .seed_capture(Path::new("/opt/rig/viola-live-rig"))
            .expect("seed");
        let settings: Value = serde_json::from_slice(
            &fs::read(scratch.config.join("settings.json")).expect("settings"),
        )
        .expect("json");
        let hooks = &settings["hooks"]["UserPromptSubmit"];
        assert_eq!(hooks.as_array().map(Vec::len), Some(1));
        assert_eq!(
            hooks[0]["hooks"][0]["command"],
            format!(
                "/opt/rig/viola-live-rig capture --out {}",
                scratch.capture_file().display()
            )
        );
        assert!(scratch.seed_capture(Path::new("/opt/a b/rig")).is_err());
    }

    #[test]
    fn the_imports_seed_imports_a_file_outside_the_cwd() {
        let base = tempfile::tempdir().expect("tempdir");
        let scratch = Scratch::create(base.path(), "1-2", false).expect("scratch");
        scratch.seed_imports().expect("seed");
        let line = fs::read_to_string(scratch.cwd.join("CLAUDE.md")).expect("CLAUDE.md");
        let imported = PathBuf::from(line.trim().trim_start_matches('@'));
        assert!(imported.is_file());
        assert!(!imported.starts_with(&scratch.cwd));
        assert!(imported.starts_with(fs::canonicalize(&scratch.root).expect("root")));
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

    const ROOT: &[u8] = b"/tmp/viola-rig-1000/12-34";
    const ID: &[u8] = b"0b4e8f1c-2d3a-4b5c-9d6e-7f8091a2b3c4";

    fn moved() -> (Stamp, Stamp) {
        let t = SystemTime::UNIX_EPOCH;
        (Some(("a".to_owned(), t)), Some(("b".to_owned(), t)))
    }

    #[test]
    fn a_held_stamp_reads_unchanged_whatever_the_file_holds() {
        let t = SystemTime::UNIX_EPOCH;
        let a = Some(("a".to_owned(), t));
        let read: &[u8] = br#"{"projects":{"/tmp/viola-rig-1000/12-34/cwd":{}}}"#;
        assert_eq!(witness(&a, &a.clone(), Some(read), &[ROOT]), "unchanged");
        assert_eq!(witness(&None, &None, None, &[ROOT]), "absent");
    }

    #[test]
    fn a_moved_stamp_with_no_mark_reads_changed_by_others() {
        let (before, after) = moved();
        let read: &[u8] = br#"{"numStartups":7}"#;
        assert_eq!(
            witness(&before, &after, Some(read), &[ROOT, ID]),
            "changed-by-others"
        );
    }

    #[test]
    fn a_moved_stamp_with_the_path_mark_reads_changed_by_rig() {
        let (before, after) = moved();
        let read: &[u8] = br#"{"projects":{"/tmp/viola-rig-1000/12-34/cwd":{"x":1}}}"#;
        assert_eq!(
            witness(&before, &after, Some(read), &[ROOT]),
            "changed-by-rig"
        );
    }

    #[test]
    fn a_moved_stamp_with_the_id_mark_reads_changed_by_rig() {
        let (before, after) = moved();
        let read: &[u8] = br#"{"lastSessionId":"0b4e8f1c-2d3a-4b5c-9d6e-7f8091a2b3c4"}"#;
        assert_eq!(
            witness(&before, &after, Some(read), &[ROOT, ID]),
            "changed-by-rig"
        );
    }

    #[test]
    fn an_id_shaped_text_with_no_id_needle_reads_changed_by_others() {
        let (before, after) = moved();
        let read: &[u8] = br#"{"lastSessionId":"0b4e8f1c-2d3a-4b5c-9d6e-7f8091a2b3c4"}"#;
        assert_eq!(
            witness(&before, &after, Some(read), &[ROOT]),
            "changed-by-others"
        );
    }

    #[test]
    fn a_moved_stamp_over_an_absent_or_unreadable_file_reads_unread() {
        let (before, after) = moved();
        assert_eq!(witness(&before, &after, None, &[ROOT, ID]), "unread");
        let base = tempfile::tempdir().expect("tempdir");
        assert_eq!(
            read_moved(&base.path().join("gone.json"), base.path()),
            None
        );
        assert_eq!(read_moved(base.path(), base.path()), None, "a directory");
    }

    #[test]
    fn a_link_out_of_the_home_is_not_read() {
        let home = tempfile::tempdir().expect("home");
        let away = tempfile::tempdir().expect("away");
        let outside = away.path().join("cfg.json");
        fs::write(&outside, b"{}").expect("outside");
        let inside = home.path().join("inner.json");
        fs::write(&inside, b"{\"in\":1}").expect("inside");
        let out_link = home.path().join("out.json");
        let in_link = home.path().join("in.json");
        std::os::unix::fs::symlink(&outside, &out_link).expect("link out");
        std::os::unix::fs::symlink(&inside, &in_link).expect("link in");
        assert_eq!(read_moved(&out_link, home.path()), None);
        assert_eq!(
            read_moved(&in_link, home.path()).as_deref(),
            Some(&b"{\"in\":1}"[..])
        );
        assert_eq!(
            read_moved(&outside, home.path()).as_deref(),
            Some(&b"{}"[..]),
            "a regular file is read where it stands"
        );
    }
}
