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

/// The OS temp dir a child of [`child_env`] makes its temp dirs under: its environment names no
/// `TMPDIR`, so `/tmp`, canonical.
pub fn child_temp_root() -> PathBuf {
    fs::canonicalize("/tmp").unwrap_or_else(|_| PathBuf::from("/tmp"))
}

/// Whether `config` (a `.claude.json`, read through `MAX_FRAME`) holds a `projects` key naming a
/// `viola-verify-` dir directly under `temp_root`: `present` · `absent` · `unread`. The bytes are
/// searched and nothing of them is kept.
pub fn verify_record_left(config: &Path, temp_root: &Path) -> &'static str {
    let mut bytes = Vec::new();
    let read = File::open(config).and_then(|f| f.take(MAX_FRAME + 1).read_to_end(&mut bytes));
    if read.is_err() || u64::try_from(bytes.len()).is_ok_and(|n| n > MAX_FRAME) {
        return "unread";
    }
    let Ok(doc) = serde_json::from_slice::<Value>(&bytes) else {
        return "unread";
    };
    let left = doc["projects"].as_object().is_some_and(|projects| {
        projects.keys().any(|key| {
            let key = Path::new(key);
            key.parent() == Some(temp_root)
                && key
                    .file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|n| n.starts_with("viola-verify-"))
        })
    });
    if left { "present" } else { "absent" }
}

/// Every host name the child must not inherit (the pty seam starts from the rig's own env).
pub fn removed_names(env: &[(OsString, OsString)]) -> Vec<OsString> {
    std::env::vars_os()
        .map(|(k, _)| k)
        .filter(|k| !env.iter().any(|(kept, _)| kept == k))
        .collect()
}

/// The sentinel entry's key and its serialised bytes, held in memory only.
#[derive(Debug, Clone)]
pub struct Sentinel {
    pub key: String,
    pub bytes: Vec<u8>,
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

    /// The cwd made a git root by git's own minimal layout written as files — a `.git/` directory
    /// holding `HEAD` and empty `objects/` and `refs/` — with no `git` process spawned: the CLI's
    /// root walk stops at a folder holding a `.git` directory, and reads none of its content.
    pub fn make_git_root(&self) -> io::Result<()> {
        let git = self.cwd.join(".git");
        create_private_dir(&git)?;
        create_private_dir(&git.join("objects"))?;
        create_private_dir(&git.join("refs"))?;
        replace_private(&git.join("HEAD"), b"ref: refs/heads/main\n", FILE_MODE)
            .map_err(|_| io::Error::other("git layout failed"))
    }

    /// `written` when the cwd holds a `.git` directory with a regular `HEAD`, read from the disk;
    /// else `absent`.
    pub fn git_root_word(&self) -> &'static str {
        let git = self.cwd.join(".git");
        let dir = fs::symlink_metadata(&git).is_ok_and(|m| m.is_dir());
        let head = fs::symlink_metadata(git.join("HEAD")).is_ok_and(|m| m.is_file());
        if dir && head { "written" } else { "absent" }
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

    /// The used-config markers merged into the private config `seed_config` wrote, every key
    /// it holds kept; `version` is the running CLI's, read from the child program by path. With
    /// `browser_tools` false the browser-tools key is left out, so the CLI's prompt can stand.
    pub fn seed_used_markers(&self, version: &str, browser_tools: bool) -> io::Result<()> {
        let path = self.config.join(".claude.json");
        let Some(Value::Object(mut seed)) = read_json(&path) else {
            return Err(io::Error::other("seed unread"));
        };
        for (key, marker) in seeded_markers(browser_tools) {
            seed.insert(key.to_owned(), marker.value(version));
        }
        write_private(&path, &Value::Object(seed))
    }

    /// `written` when the private config on the disk holds the markers `seed_used_markers` writes
    /// for `version` and `browser_tools` at their values, and with `browser_tools` false no
    /// browser-tools key at all; else `absent`.
    pub fn seed_markers_word(&self, version: Option<&str>, browser_tools: bool) -> &'static str {
        let Some(version) = version else {
            return "absent";
        };
        let held = read_json(&self.config.join(".claude.json")).is_some_and(|doc| {
            seeded_markers(browser_tools)
                .all(|(key, marker)| doc.get(key) == Some(&marker.value(version)))
                && (browser_tools || doc.get(BROWSER_TOOLS_KEY).is_none())
        });
        if held { "written" } else { "absent" }
    }

    /// `cache-a`: `written` when the key NAME `cachedChromeExtensionInstalled` stands in the
    /// private config, `absent` when it does not, `unread` for a config not read. Presence only:
    /// no value is read out.
    pub fn cache_word(&self) -> &'static str {
        match read_json(&self.config.join(".claude.json")) {
            Some(doc) if doc.get(EXTENSION_CACHE_KEY).is_some() => "written",
            Some(_) => "absent",
            None => "unread",
        }
    }

    /// Whether a `viola verify` run left its trust record in the private config: read before the
    /// teardown removes it.
    pub fn record_left(&self, temp_root: &Path) -> &'static str {
        verify_record_left(&self.config.join(".claude.json"), temp_root)
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

    /// Another folder's entry in the private config, `<root>/sentinel/` made for it: the entry
    /// `{"hasTrustDialogAccepted": true, "allowedTools": []}` merged in, every key kept; its key
    /// and its serialised bytes are held in memory for [`Scratch::entry_words`].
    pub fn seed_sentinel(&self) -> io::Result<Sentinel> {
        let folder = self.root.join("sentinel");
        create_private_dir(&folder)?;
        let key = fs::canonicalize(folder)?.to_string_lossy().into_owned();
        let path = self.config.join(".claude.json");
        let Some(Value::Object(mut seed)) = read_json(&path) else {
            return Err(io::Error::other("seed unread"));
        };
        let entry = json!({"hasTrustDialogAccepted": true, "allowedTools": []});
        let bytes = serde_json::to_vec(&entry).map_err(io::Error::other)?;
        let projects = seed
            .entry("projects")
            .or_insert_with(|| Value::Object(serde_json::Map::new()));
        let Value::Object(projects) = projects else {
            return Err(io::Error::other("seed projects"));
        };
        projects.insert(key.clone(), entry);
        write_private(&path, &Value::Object(seed))?;
        Ok(Sentinel { key, bytes })
    }

    /// A fresh `<root>/run-cwd/`, canonical: outside the cwd, in no repository, trusted by no record.
    pub fn fresh_cwd(&self) -> io::Result<PathBuf> {
        let cwd = self.root.join("run-cwd");
        create_private_dir(&cwd)?;
        fs::canonicalize(cwd)
    }

    /// The private config read again: `cwd`'s `hasTrustDialogAccepted` (`true` · `false` · `absent` ·
    /// `other`) and whether the sentinel's entry serialises to its seeded bytes (`equal` ·
    /// `differs`); both `unread` for a config not read.
    pub fn entry_words(&self, cwd: &Path, sentinel: &Sentinel) -> (&'static str, &'static str) {
        let Some(doc) = read_json(&self.config.join(".claude.json")) else {
            return ("unread", "unread");
        };
        let flag =
            match doc["projects"][cwd.to_string_lossy().as_ref()].get("hasTrustDialogAccepted") {
                Some(Value::Bool(true)) => "true",
                Some(Value::Bool(false)) => "false",
                None => "absent",
                Some(_) => "other",
            };
        let same = serde_json::to_vec(&doc["projects"][sentinel.key.as_str()])
            .is_ok_and(|bytes| bytes == sentinel.bytes);
        (flag, if same { "equal" } else { "differs" })
    }

    /// The last `trust-record` word of `<vhome>/diagnostics/run-<name>.ndjson`, a code word; `absent`
    /// with no such line, `unread` for a log not read.
    pub fn run_trust_word(&self, name: &str) -> String {
        let Some(vhome) = &self.vhome else {
            return "unread".to_owned();
        };
        let log = vhome.join("diagnostics").join(format!("run-{name}.ndjson"));
        let mut bytes = Vec::new();
        let read = File::open(log).and_then(|f| f.take(MAX_FRAME).read_to_end(&mut bytes));
        if read.is_err() {
            return "unread".to_owned();
        }
        String::from_utf8_lossy(&bytes)
            .lines()
            .rev()
            .filter_map(|l| serde_json::from_str::<Value>(l).ok())
            .filter(|l| l["event"] == "trust-record")
            .find_map(|l| l["detail"].as_str().map(str::to_owned))
            .map_or_else(
                || "absent".to_owned(),
                |w| {
                    if viola_core::is_code_word(&w) {
                        w
                    } else {
                        "unread".to_owned()
                    }
                },
            )
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

/// The form a used-config marker takes, as the CLI's bundle writes it.
#[derive(Clone, Copy)]
enum Marker {
    /// `true`.
    Seen,
    /// The running CLI's version.
    Version,
    /// The bundle's own migration constant.
    Migration(u64),
    /// `false`.
    Off,
}

impl Marker {
    fn value(self, version: &str) -> Value {
        match self {
            Self::Seen => Value::Bool(true),
            Self::Off => Value::Bool(false),
            Self::Version => Value::String(version.to_owned()),
            Self::Migration(n) => Value::from(n),
        }
    }
}

/// The used-config markers, read from the claude 2.1.325 bundle, each value the form the bundle
/// writes: `hasSeenAutoDefaultNotice` gates the auto-mode-default notice at an interactive start;
/// `lastReleaseNotesSeen` is read against the running version by the startup update summary;
/// `lastClawdEntranceVersion` stops the entrance animation, which otherwise moves verify's quiet
/// settle; `migrationVersion` at `Ge=15` skips the startup migrations that rewrite the config;
/// `claudeInChromeDefaultEnabled` `false` is what the bundle writes on the browser-tools prompt's
/// «keep browser tools off» answer, and a config holding it closes both that prompt's arming gate
/// and its offer step.
const USED_MARKERS: [(&str, Marker); 5] = [
    ("hasSeenAutoDefaultNotice", Marker::Seen),
    ("lastReleaseNotesSeen", Marker::Version),
    ("lastClawdEntranceVersion", Marker::Version),
    ("migrationVersion", Marker::Migration(15)),
    ("claudeInChromeDefaultEnabled", Marker::Off),
];

/// The browser-tools key among the used markers, left out of the `VP` arm's seed.
const BROWSER_TOOLS_KEY: &str = "claudeInChromeDefaultEnabled";

/// The CLI's own extension-detected cache key, read by name alone (`cache-a`).
const EXTENSION_CACHE_KEY: &str = "cachedChromeExtensionInstalled";

/// The used markers a seed writes: all five, or without the browser-tools key.
fn seeded_markers(browser_tools: bool) -> impl Iterator<Item = (&'static str, Marker)> {
    USED_MARKERS
        .into_iter()
        .filter(move |(key, _)| browser_tools || *key != BROWSER_TOOLS_KEY)
}

/// The run letters of `viola verify`'s interactive runs, in run order.
const RUN_LETTERS: [&str; 6] = ["a", "b", "c", "d", "e", "f"];

impl Scratch {
    /// Where the `V` arm's `viola verify --record` writes: inside the scratch, removed with it.
    pub fn record_dir(&self) -> PathBuf {
        self.root.join("record")
    }

    /// Copies each `FirstScreen.<x>.json` a failed verify wrote under the record dir's version dir
    /// into `kept` (0600, never printed), and names the runs copied: `b,c,…` in run order, `none`
    /// when there is no record dir or no such file, `unread` when the record dir or a file in it
    /// cannot be read, parsed or copied.
    pub fn keep_first_screens(&self, kept: &Path) -> String {
        let record = self.record_dir();
        if !record.exists() {
            return "none".to_owned();
        }
        let Ok(entries) = fs::read_dir(&record) else {
            return "unread".to_owned();
        };
        let mut versions: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
        versions.sort();
        let mut letters = Vec::new();
        for version in versions {
            for letter in RUN_LETTERS {
                let name = format!("FirstScreen.{letter}.json");
                let path = version.join(&name);
                if !path.exists() {
                    continue;
                }
                let copied = read_json(&path).is_some_and(|doc| {
                    create_private_dir(kept).is_ok()
                        && write_private(&kept.join(&name), &doc).is_ok()
                });
                if !copied {
                    return "unread".to_owned();
                }
                letters.push(letter);
            }
        }
        if letters.is_empty() {
            "none".to_owned()
        } else {
            letters.join(",")
        }
    }

    /// Copies every regular file of the record dir's version dirs into `kept/<version>/`, the
    /// bytes as read (0600, never re-serialised, never printed), leaving out the first screens:
    /// the count of files copied, `none` when there is no record dir or no file in it, `exists`
    /// when `kept` already stands (nothing written), `unread` when a read or a write failed.
    pub fn keep_record(&self, kept: &Path) -> String {
        if kept.symlink_metadata().is_ok() {
            return "exists".to_owned();
        }
        let record = self.record_dir();
        if !record.exists() {
            return "none".to_owned();
        }
        let Ok(entries) = fs::read_dir(&record) else {
            return "unread".to_owned();
        };
        let mut versions: Vec<PathBuf> = entries
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.symlink_metadata().is_ok_and(|m| m.is_dir()))
            .collect();
        versions.sort();
        let mut copied = 0usize;
        for version in versions {
            let (Some(name), Ok(files)) = (version.file_name(), fs::read_dir(&version)) else {
                return "unread".to_owned();
            };
            let into = kept.join(name);
            let mut paths: Vec<PathBuf> = files.flatten().map(|e| e.path()).collect();
            paths.sort();
            for path in paths {
                let Some(file) = path.file_name().and_then(|n| n.to_str()) else {
                    return "unread".to_owned();
                };
                if !path.symlink_metadata().is_ok_and(|m| m.is_file())
                    || (file.starts_with("FirstScreen.") && file.ends_with(".json"))
                {
                    continue;
                }
                let Some(bytes) = read_bounded(&path) else {
                    return "unread".to_owned();
                };
                if create_private_dir(&into).is_err()
                    || replace_private(&into.join(file), &bytes, FILE_MODE).is_err()
                {
                    return "unread".to_owned();
                }
                copied += 1;
            }
        }
        if copied == 0 {
            "none".to_owned()
        } else {
            copied.to_string()
        }
    }
}

/// Whether any normal component of `path` is exactly `home`: a recording's scratch must name
/// none, or the recorder refuses its print probe's cwd as a home path (testing.md 2026-10-05).
pub fn names_home_component(path: &Path) -> bool {
    path.components()
        .any(|c| matches!(c, std::path::Component::Normal(n) if n == "home"))
}

/// A file's bytes read through `MAX_FRAME`; `None` when unreadable or over the bound.
fn read_bounded(path: &Path) -> Option<Vec<u8>> {
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

/// A JSON document read through `MAX_FRAME`; `None` when absent, over the bound or malformed.
fn read_json(path: &Path) -> Option<Value> {
    let mut bytes = Vec::new();
    File::open(path)
        .ok()?
        .take(MAX_FRAME + 1)
        .read_to_end(&mut bytes)
        .ok()?;
    if u64::try_from(bytes.len()).is_ok_and(|n| n > MAX_FRAME) {
        return None;
    }
    serde_json::from_slice(&bytes).ok()
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
    fn seed_used_markers_writes_the_bundle_markers_and_keeps_the_seed() {
        let base = tempfile::tempdir().expect("tempdir");
        let scratch = Scratch::create(base.path(), "1-2", true).expect("scratch");
        scratch.seed_config(true).expect("seed");
        assert_eq!(scratch.seed_markers_word(Some("2.1.325"), true), "absent");
        scratch.seed_used_markers("2.1.325", true).expect("markers");
        let config: Value =
            serde_json::from_slice(&fs::read(scratch.config.join(".claude.json")).expect("config"))
                .expect("json");
        let mut keys: Vec<&str> = config
            .as_object()
            .expect("object")
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "claudeInChromeDefaultEnabled",
                "hasCompletedOnboarding",
                "hasSeenAutoDefaultNotice",
                "lastClawdEntranceVersion",
                "lastReleaseNotesSeen",
                "migrationVersion",
                "projects",
            ]
        );
        assert_eq!(config["claudeInChromeDefaultEnabled"], false);
        assert_eq!(config["hasCompletedOnboarding"], true);
        assert_eq!(config["hasSeenAutoDefaultNotice"], true);
        assert_eq!(config.get("hasSeenAutoModeEntryWarning"), None);
        assert_eq!(config.get("lastOnboardingVersion"), None);
        assert_eq!(config["lastReleaseNotesSeen"], "2.1.325");
        assert_eq!(config["lastClawdEntranceVersion"], "2.1.325");
        assert_eq!(config["migrationVersion"], 15);
        let cwd = scratch.cwd.to_string_lossy().into_owned();
        assert_eq!(
            config["projects"],
            json!({ cwd: {"hasTrustDialogAccepted": true} })
        );
        assert_eq!(scratch.seed_markers_word(Some("2.1.325"), true), "written");
        assert_eq!(
            scratch.seed_markers_word(Some("2.1.325"), false),
            "absent",
            "the browser-tools key stands"
        );
        assert_eq!(scratch.seed_markers_word(Some("2.1.326"), true), "absent");
        assert_eq!(scratch.seed_markers_word(None, true), "absent");
    }

    #[test]
    fn seed_used_markers_leaves_the_browser_tools_key_out_for_vp() {
        let base = tempfile::tempdir().expect("tempdir");
        let scratch = Scratch::create(base.path(), "1-2", true).expect("scratch");
        scratch.seed_config(true).expect("seed");
        scratch
            .seed_used_markers("2.1.325", false)
            .expect("markers");
        let config: Value =
            serde_json::from_slice(&fs::read(scratch.config.join(".claude.json")).expect("config"))
                .expect("json");
        let mut keys: Vec<&str> = config
            .as_object()
            .expect("object")
            .keys()
            .map(String::as_str)
            .collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "hasCompletedOnboarding",
                "hasSeenAutoDefaultNotice",
                "lastClawdEntranceVersion",
                "lastReleaseNotesSeen",
                "migrationVersion",
                "projects",
            ]
        );
        assert_eq!(scratch.seed_markers_word(Some("2.1.325"), false), "written");
        assert_eq!(
            scratch.seed_markers_word(Some("2.1.325"), true),
            "absent",
            "the browser-tools key is out"
        );
        assert_eq!(scratch.seed_markers_word(Some("2.1.326"), false), "absent");
    }

    #[test]
    fn cache_word_reads_the_key_name_only() {
        let base = tempfile::tempdir().expect("tempdir");
        let scratch = Scratch::create(base.path(), "1-2", true).expect("scratch");
        assert_eq!(scratch.cache_word(), "unread", "no config yet");
        scratch.seed_config(true).expect("seed");
        assert_eq!(scratch.cache_word(), "absent");
        let path = scratch.config.join(".claude.json");
        for planted in [json!(true), json!(false)] {
            let mut doc: Value =
                serde_json::from_slice(&fs::read(&path).expect("config")).expect("json");
            doc["cachedChromeExtensionInstalled"] = planted.clone();
            write_private(&path, &doc).expect("plant");
            assert_eq!(scratch.cache_word(), "written", "{planted}");
        }
        fs::write(&path, b"not json").expect("garble");
        assert_eq!(scratch.cache_word(), "unread");
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
    fn a_verify_record_left_under_the_temp_root_reads_present() {
        let base = tempfile::tempdir().expect("tempdir");
        let file = base.path().join(".claude.json");
        let root = Path::new("/tmp");
        let cases = [
            (
                "a verify dir's record",
                r#"{"projects":{"/tmp/viola-verify-a1":{"hasTrustDialogAccepted":true}}}"#,
                "present",
            ),
            (
                "the cwd's record alone",
                r#"{"projects":{"/tmp/viola-rig-1/12-34/cwd":{"hasTrustDialogAccepted":true}}}"#,
                "absent",
            ),
            (
                "a verify-named dir elsewhere",
                r#"{"projects":{"/srv/viola-verify-a1":{}}}"#,
                "absent",
            ),
            (
                "no projects",
                r#"{"hasCompletedOnboarding":true}"#,
                "absent",
            ),
            ("not json", "{", "unread"),
        ];
        for (label, body, word) in cases {
            fs::write(&file, body).expect("config");
            assert_eq!(verify_record_left(&file, root), word, "{label}");
        }
        assert_eq!(
            verify_record_left(&base.path().join("gone.json"), root),
            "unread"
        );
        assert!(child_temp_root().is_absolute());
    }

    #[test]
    fn keep_record_copies_the_set_byte_for_byte_and_never_overwrites() {
        use std::os::unix::fs::PermissionsExt as _;
        let base = tempfile::tempdir().expect("tempdir");
        let scratch = Scratch::create(base.path(), "1-2", true).expect("scratch");
        let kept = base.path().join("kept").join("R4").join("record");
        assert_eq!(scratch.keep_record(&kept), "none", "no record dir");
        assert!(!kept.exists(), "none writes nothing");
        let version = scratch.record_dir().join("2.1.325");
        fs::create_dir_all(&version).expect("version dir");
        let spaced: &[u8] = b"{ \"a\" :  1,\n  \"b\": [ 2 ] }\n";
        let other: &[u8] = b"{\"hook_event_name\":\"Stop\"}";
        fs::write(version.join("Stop.default.json"), spaced).expect("set file");
        fs::write(version.join("Screen.ready.json"), other).expect("set file");
        fs::write(version.join("FirstScreen.b.json"), b"{}").expect("first screen");
        assert_eq!(scratch.keep_record(&kept), "2");
        let into = kept.join("2.1.325");
        for (name, bytes) in [("Stop.default.json", spaced), ("Screen.ready.json", other)] {
            let path = into.join(name);
            assert_eq!(fs::read(&path).expect("kept file"), bytes, "{name}");
            let mode = fs::metadata(&path).expect("meta").permissions().mode() & 0o777;
            assert_eq!(mode, 0o600, "{name}");
        }
        assert!(!into.join("FirstScreen.b.json").exists(), "no first screen");
        assert_eq!(scratch.keep_record(&kept), "exists");
        assert_eq!(
            fs::read(into.join("Stop.default.json")).expect("kept file"),
            spaced,
            "left as found"
        );
    }

    #[test]
    fn a_path_names_a_home_component_only_for_the_exact_word() {
        for (label, path, named) in [
            ("the rig's base", "/tmp/viola-rig-1000", false),
            ("an inner home", "/x/home/y", true),
            ("the root home", "/home", true),
            ("a plural", "/x/homes", false),
            ("the vhome", "/x/vhome", false),
        ] {
            assert_eq!(names_home_component(Path::new(path)), named, "{label}");
        }
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
