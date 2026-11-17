//! The per-project trust record the `claude` CLI reads at a start: which config file a child reads,
//! the key the CLI files a folder under, and the record's form inside that file. Pure: the file
//! checks, the lock, the replace and the read-back are the root bin's (`src/trust_record.rs`).
//! Every outcome is one closed word, never a path, key or value.

use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value};

/// The config file's name, inside the config dir or the user home.
pub const CONFIG_FILE: &str = ".claude.json";
/// The variable naming the config dir, read only when the strip kept it for the child.
pub const CONFIG_DIR_VAR: &str = "CLAUDE_CONFIG_DIR";
/// The object holding one entry per project folder.
pub const PROJECTS_KEY: &str = "projects";
/// The one field of a record viola writes.
pub const ACCEPTED_KEY: &str = "hasTrustDialogAccepted";
/// The CLI's own lock beside the config file: `<file>.lock`, a directory.
pub const LOCK_SUFFIX: &str = ".lock";

/// What one write or removal of the record did, a closed code word each.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrustWord {
    Written,
    Removed,
    ConfigAbsent,
    ConfigUnreadable,
    ConfigLink,
    ConfigNotRegular,
    ConfigForeign,
    ConfigOversize,
    ConfigMalformed,
    ConfigDirUnusable,
    HomeUnknown,
    KeyPresent,
    LockHeld,
    Lost,
}

impl TrustWord {
    pub const ALL: [Self; 14] = [
        Self::Written,
        Self::Removed,
        Self::ConfigAbsent,
        Self::ConfigUnreadable,
        Self::ConfigLink,
        Self::ConfigNotRegular,
        Self::ConfigForeign,
        Self::ConfigOversize,
        Self::ConfigMalformed,
        Self::ConfigDirUnusable,
        Self::HomeUnknown,
        Self::KeyPresent,
        Self::LockHeld,
        Self::Lost,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Written => "written",
            Self::Removed => "removed",
            Self::ConfigAbsent => "config-absent",
            Self::ConfigUnreadable => "config-unreadable",
            Self::ConfigLink => "config-link",
            Self::ConfigNotRegular => "config-not-regular",
            Self::ConfigForeign => "config-foreign",
            Self::ConfigOversize => "config-oversize",
            Self::ConfigMalformed => "config-malformed",
            Self::ConfigDirUnusable => "config-dir-unusable",
            Self::HomeUnknown => "home-unknown",
            Self::KeyPresent => "key-present",
            Self::LockHeld => "lock-held",
            Self::Lost => "lost",
        }
    }
}

/// The config file a child reads: `<CLAUDE_CONFIG_DIR>/.claude.json` when the strip kept that name
/// (`kept_config_dir` is its value, passed only then), else `<user home>/.claude.json`. A kept value
/// that is empty or not absolute is `config-dir-unusable`; no home is `home-unknown`.
pub fn config_file(
    kept_config_dir: Option<&OsStr>,
    user_home: Option<&Path>,
) -> Result<PathBuf, TrustWord> {
    match kept_config_dir {
        Some(dir) => {
            let dir = Path::new(dir);
            if dir.as_os_str().is_empty() || !dir.is_absolute() {
                Err(TrustWord::ConfigDirUnusable)
            } else {
                Ok(dir.join(CONFIG_FILE))
            }
        }
        None => user_home
            .filter(|home| !home.as_os_str().is_empty())
            .map(|home| home.join(CONFIG_FILE))
            .ok_or(TrustWord::HomeUnknown),
    }
}

/// The key the CLI files a folder under: its git root inside a repository, else the folder itself
/// (both canonical; the caller canonicalises).
pub fn record_key(canonical_cwd: &Path, git_root: Option<&Path>) -> String {
    git_root
        .unwrap_or(canonical_cwd)
        .to_string_lossy()
        .into_owned()
}

/// The document as an object with `projects` absent or an object; any other shape is malformed.
fn document(bytes: &[u8]) -> Result<Map<String, Value>, TrustWord> {
    match serde_json::from_slice::<Value>(bytes) {
        Ok(Value::Object(doc)) if doc.get(PROJECTS_KEY).is_none_or(Value::is_object) => Ok(doc),
        _ => Err(TrustWord::ConfigMalformed),
    }
}

/// The document as the CLI writes it: two-space indent.
fn written(doc: Map<String, Value>) -> Vec<u8> {
    serde_json::to_vec_pretty(&Value::Object(doc)).unwrap_or_default()
}

/// `bytes` with `projects[key] = {"hasTrustDialogAccepted": true}` added, every other key and entry
/// kept in its order; an entry already at `key` is `key-present` and nothing is changed.
pub fn with_record(bytes: &[u8], key: &str) -> Result<Vec<u8>, TrustWord> {
    let mut doc = document(bytes)?;
    let projects = doc
        .entry(PROJECTS_KEY)
        .or_insert_with(|| Value::Object(Map::new()));
    let Value::Object(projects) = projects else {
        return Err(TrustWord::ConfigMalformed);
    };
    if projects.contains_key(key) {
        return Err(TrustWord::KeyPresent);
    }
    let mut record = Map::new();
    record.insert(ACCEPTED_KEY.to_owned(), Value::Bool(true));
    projects.insert(key.to_owned(), Value::Object(record));
    Ok(written(doc))
}

/// `bytes` with the whole entry at `projects[key]` removed, whatever it holds; a `projects` object
/// the removal leaves empty is removed too, so a record [`with_record`] wrote into a document with
/// no `projects` leaves it as it was. Nothing else changed.
pub fn without_record(bytes: &[u8], key: &str) -> Result<Vec<u8>, TrustWord> {
    let mut doc = document(bytes)?;
    let emptied = match doc.get_mut(PROJECTS_KEY) {
        Some(Value::Object(projects)) => {
            projects.shift_remove(key).is_some() && projects.is_empty()
        }
        _ => false,
    };
    if emptied {
        doc.shift_remove(PROJECTS_KEY);
    }
    Ok(written(doc))
}

/// Whether `projects[key].hasTrustDialogAccepted` is `true` in `bytes`.
pub fn holds_record(bytes: &[u8], key: &str) -> bool {
    serde_json::from_slice::<Value>(bytes)
        .is_ok_and(|doc| doc[PROJECTS_KEY][key][ACCEPTED_KEY] == Value::Bool(true))
}

/// Whether `bytes` holds any entry at `projects[key]`.
pub fn holds_entry(bytes: &[u8], key: &str) -> bool {
    serde_json::from_slice::<Value>(bytes).is_ok_and(|doc| doc[PROJECTS_KEY].get(key).is_some())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn trust_words_are_each_their_own_code_word() {
        let words: Vec<&str> = TrustWord::ALL.iter().map(|w| w.as_str()).collect();
        assert_eq!(
            words,
            [
                "written",
                "removed",
                "config-absent",
                "config-unreadable",
                "config-link",
                "config-not-regular",
                "config-foreign",
                "config-oversize",
                "config-malformed",
                "config-dir-unusable",
                "home-unknown",
                "key-present",
                "lock-held",
                "lost",
            ]
        );
        for word in TrustWord::ALL {
            assert!(viola_core::is_code_word(word.as_str()), "{word:?}");
        }
    }

    /// A label, the kept config dir, the user home and the file chosen.
    type FileCase<'a> = (
        &'a str,
        Option<&'a str>,
        Option<&'a Path>,
        Result<PathBuf, TrustWord>,
    );

    #[test]
    fn config_file_takes_the_kept_dir_else_the_user_home() {
        let home = Path::new("/u/me");
        let cases: [FileCase<'_>; 6] = [
            (
                "the kept dir",
                Some("/c/dir"),
                Some(home),
                Ok(PathBuf::from("/c/dir/.claude.json")),
            ),
            (
                "the kept dir with no home",
                Some("/c/dir"),
                None,
                Ok(PathBuf::from("/c/dir/.claude.json")),
            ),
            (
                "a relative kept dir",
                Some("c/dir"),
                Some(home),
                Err(TrustWord::ConfigDirUnusable),
            ),
            (
                "an empty kept dir",
                Some(""),
                Some(home),
                Err(TrustWord::ConfigDirUnusable),
            ),
            (
                "the user home",
                None,
                Some(home),
                Ok(PathBuf::from("/u/me/.claude.json")),
            ),
            ("no home", None, None, Err(TrustWord::HomeUnknown)),
        ];
        for (label, kept, user_home, expected) in cases {
            assert_eq!(
                config_file(kept.map(OsStr::new), user_home),
                expected,
                "{label}"
            );
        }
        assert_eq!(
            config_file(None, Some(Path::new(""))),
            Err(TrustWord::HomeUnknown)
        );
    }

    #[test]
    fn record_key_is_the_git_root_else_the_folder() {
        let cwd = Path::new("/r/repo/sub");
        assert_eq!(record_key(cwd, Some(Path::new("/r/repo"))), "/r/repo");
        assert_eq!(record_key(cwd, None), "/r/repo/sub");
    }

    #[test]
    fn with_record_adds_only_its_entry_and_keeps_every_other_in_order() {
        let before =
            br#"{"z":1,"projects":{"/a":{"hasTrustDialogAccepted":true,"x":[1]}},"b":"k"}"#;
        let after = with_record(before, "/t/v").expect("written");
        let text = String::from_utf8(after.clone()).expect("utf-8");
        assert!(text.starts_with("{\n  \"z\": 1,"), "{text}");
        let doc: Value = serde_json::from_slice(&after).expect("json");
        let keys: Vec<&String> = doc.as_object().expect("object").keys().collect();
        assert_eq!(keys, ["z", "projects", "b"]);
        let projects: Vec<&String> = doc["projects"]
            .as_object()
            .expect("projects")
            .keys()
            .collect();
        assert_eq!(projects, ["/a", "/t/v"]);
        assert_eq!(
            doc["projects"]["/t/v"],
            json!({"hasTrustDialogAccepted": true})
        );
        assert_eq!(
            doc["projects"]["/a"],
            json!({"hasTrustDialogAccepted": true, "x": [1]})
        );
        assert!(holds_record(&after, "/t/v"));
        assert!(!holds_record(before, "/t/v"));
    }

    #[test]
    fn with_record_makes_projects_when_absent() {
        let after = with_record(b"{}", "/t/v").expect("written");
        assert_eq!(
            serde_json::from_slice::<Value>(&after).expect("json"),
            json!({"projects": {"/t/v": {"hasTrustDialogAccepted": true}}})
        );
    }

    #[test]
    fn with_record_refuses_a_present_key_and_every_other_shape() {
        assert_eq!(
            with_record(br#"{"projects":{"/t/v":{}}}"#, "/t/v"),
            Err(TrustWord::KeyPresent)
        );
        for (label, bytes) in [
            ("an array", &b"[]"[..]),
            ("a string", br#""x""#),
            ("projects an array", br#"{"projects":[]}"#),
            ("projects null", br#"{"projects":null}"#),
            ("not json", b"{"),
            ("empty", b""),
        ] {
            assert_eq!(
                with_record(bytes, "/t/v"),
                Err(TrustWord::ConfigMalformed),
                "{label}"
            );
            assert_eq!(
                without_record(bytes, "/t/v"),
                Err(TrustWord::ConfigMalformed),
                "{label}"
            );
        }
    }

    #[test]
    fn without_record_removes_the_whole_entry_and_nothing_else() {
        let before = br#"{"projects":{"/a":{"k":1},"/t/v":{"hasTrustDialogAccepted":true,"added":2}},"b":0}"#;
        let after = without_record(before, "/t/v").expect("removed");
        assert_eq!(
            serde_json::from_slice::<Value>(&after).expect("json"),
            json!({"projects": {"/a": {"k": 1}}, "b": 0})
        );
        assert!(!holds_entry(&after, "/t/v"));
        assert!(holds_entry(before, "/t/v"));
        let none = without_record(br#"{"b":0}"#, "/t/v").expect("nothing to remove");
        assert_eq!(
            serde_json::from_slice::<Value>(&none).expect("json"),
            json!({"b": 0})
        );
        let empty = without_record(br#"{"b":0,"projects":{}}"#, "/t/v").expect("not there");
        assert_eq!(
            serde_json::from_slice::<Value>(&empty).expect("json"),
            json!({"b": 0, "projects": {}}),
            "an empty projects the removal did not empty stays"
        );
    }

    #[test]
    fn with_then_without_record_restores_a_document_with_no_projects() {
        let before = br#"{"b":1,"z":[2]}"#;
        let written = with_record(before, "/t/v").expect("written");
        let restored = without_record(&written, "/t/v").expect("removed");
        assert_eq!(
            serde_json::from_slice::<Value>(&restored).expect("json"),
            json!({"b": 1, "z": [2]})
        );
    }

    #[test]
    fn holds_record_reads_only_a_true_accepted_flag() {
        assert!(holds_record(
            br#"{"projects":{"/k":{"hasTrustDialogAccepted":true}}}"#,
            "/k"
        ));
        for bytes in [
            &br#"{"projects":{"/k":{"hasTrustDialogAccepted":false}}}"#[..],
            br#"{"projects":{"/k":{"hasTrustDialogAccepted":"true"}}}"#,
            br#"{"projects":{"/other":{"hasTrustDialogAccepted":true}}}"#,
            br#"{"projects":{"/k":{}}}"#,
            b"not json",
        ] {
            assert!(!holds_record(bytes, "/k"));
        }
    }
}
