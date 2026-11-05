//! Which environment names the user defined persistently, so the R8 strip leaves them in the
//! child: the `config.json` `claude_env_keep` pass-list. Names only: no value is ever read.

use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use serde_json::Value;
use viola_core::MAX_FRAME;

use crate::obs::ConfigRejection;

const KEEP_KEY: &str = "claude_env_keep";
const KEEP_MAX: usize = 32;

/// The persistent names, and a rejection when `claude_env_keep` is present but unusable.
pub(crate) fn persistent_names(home: &Path) -> (Vec<String>, Option<ConfigRejection>) {
    read_env_keep(home)
}

fn read_env_keep(home: &Path) -> (Vec<String>, Option<ConfigRejection>) {
    let mut bytes = Vec::new();
    // An absent, unreadable or malformed document is `read_diagnostics_level`'s to report, once.
    match File::open(home.join("config.json")) {
        Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
            Ok(_) => parse_env_keep(&bytes),
            Err(_) => (Vec::new(), None),
        },
        Err(_) => (Vec::new(), None),
    }
}

fn parse_env_keep(bytes: &[u8]) -> (Vec<String>, Option<ConfigRejection>) {
    let Ok(Value::Object(map)) = serde_json::from_slice::<Value>(bytes) else {
        return (Vec::new(), None);
    };
    if map.get("v").and_then(Value::as_u64) != Some(1) {
        return (Vec::new(), None);
    }
    let Some(value) = map.get(KEEP_KEY) else {
        return (Vec::new(), None);
    };
    let names: Option<Vec<String>> = value.as_array().and_then(|items| {
        (items.len() <= KEEP_MAX)
            .then(|| {
                items
                    .iter()
                    .map(|v| v.as_str().filter(|s| is_claude_name(s)).map(str::to_owned))
                    .collect()
            })
            .flatten()
    });
    match names {
        Some(names) => (names, None),
        None => (Vec::new(), Some(ConfigRejection::Malformed)),
    }
}

fn is_claude_name(name: &str) -> bool {
    name.strip_prefix("CLAUDE").is_some_and(|rest| {
        rest.bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(text: &str) -> (Vec<String>, Option<ConfigRejection>) {
        parse_env_keep(text.as_bytes())
    }

    #[test]
    fn env_keep_reads_the_listed_claude_names() {
        let (names, rejection) =
            parse(r#"{"v":1,"claude_env_keep":["CLAUDE_CONFIG_DIR","CLAUDE_CODE_X1"]}"#);
        assert_eq!(names, ["CLAUDE_CONFIG_DIR", "CLAUDE_CODE_X1"]);
        assert_eq!(rejection, None);
    }

    #[test]
    fn env_keep_absent_or_foreign_documents_read_as_none_without_a_rejection() {
        for text in [
            r#"{"v":1}"#,
            r#"{"v":2,"claude_env_keep":["CLAUDE_X"]}"#,
            r#"{"claude_env_keep":["CLAUDE_X"]}"#,
            "[1,2]",
            "not json",
        ] {
            assert_eq!(parse(text), (Vec::new(), None), "{text}");
        }
    }

    #[test]
    fn env_keep_rejects_the_whole_list_on_any_bad_entry() {
        let too_many: Vec<String> = (0..=KEEP_MAX).map(|i| format!("\"CLAUDE_{i}\"")).collect();
        let too_many = format!(r#"{{"v":1,"claude_env_keep":[{}]}}"#, too_many.join(","));
        for text in [
            r#"{"v":1,"claude_env_keep":"CLAUDE_X"}"#,
            r#"{"v":1,"claude_env_keep":["CLAUDE_X","PATH"]}"#,
            r#"{"v":1,"claude_env_keep":["claude_x"]}"#,
            r#"{"v":1,"claude_env_keep":["CLAUDE-X"]}"#,
            r#"{"v":1,"claude_env_keep":[7]}"#,
            too_many.as_str(),
        ] {
            assert_eq!(
                parse(text),
                (Vec::new(), Some(ConfigRejection::Malformed)),
                "{text}"
            );
        }
    }

    #[test]
    fn env_keep_takes_exactly_the_maximum() {
        let names: Vec<String> = (0..KEEP_MAX).map(|i| format!("\"CLAUDE_{i}\"")).collect();
        let text = format!(r#"{{"v":1,"claude_env_keep":[{}]}}"#, names.join(","));
        assert_eq!(parse(&text).0.len(), KEEP_MAX);
    }

    #[test]
    fn claude_names_are_upper_snake_after_the_prefix() {
        assert!(is_claude_name("CLAUDE"));
        assert!(is_claude_name("CLAUDE_CODE_9"));
        assert!(!is_claude_name("CLAUDE_code"));
        assert!(!is_claude_name("XCLAUDE"));
        assert!(!is_claude_name("CLAUDE X"));
    }

    #[test]
    fn read_env_keep_reads_the_home_config_file() {
        let tmp = tempfile::tempdir().expect("tmp");
        assert_eq!(read_env_keep(tmp.path()), (Vec::new(), None));
        std::fs::write(
            tmp.path().join("config.json"),
            r#"{"v":1,"claude_env_keep":["CLAUDE_KEEP_ME"]}"#,
        )
        .expect("write");
        assert_eq!(read_env_keep(tmp.path()).0, ["CLAUDE_KEEP_ME"]);
        std::fs::write(
            tmp.path().join("config.json"),
            r#"{"v":1,"claude_env_keep":[1]}"#,
        )
        .expect("write");
        assert_eq!(
            read_env_keep(tmp.path()).1,
            Some(ConfigRejection::Malformed)
        );
    }

    /// The read stops at `MAX_FRAME`: a document one byte longer loses its closing brace, so it reads
    /// as no document, with no rejection, and the same document at the cap reads the value.
    #[test]
    fn env_keep_keeps_no_key_past_the_frame_cap() {
        let tmp = tempfile::tempdir().expect("tmp");
        let home = tmp.path().join("home");
        viola_state::fs::create_private_dir(&home).expect("home");
        let (head, tail) = (
            r#"{"v":1,"pad":""#,
            r#"","claude_env_keep":["CLAUDE_KEEP_ME"]}"#,
        );
        let document = |len: usize| {
            let pad = "x".repeat(len - head.len() - tail.len());
            format!("{head}{pad}{tail}")
        };
        let cap = usize::try_from(MAX_FRAME).expect("fits");
        std::fs::write(home.join("config.json"), document(cap + 1)).expect("write");
        assert_eq!(read_env_keep(&home), (Vec::new(), None));
        std::fs::write(home.join("config.json"), document(cap)).expect("write");
        assert_eq!(
            read_env_keep(&home),
            (vec!["CLAUDE_KEEP_ME".to_owned()], None)
        );
    }

    #[test]
    fn persistent_names_report_a_bad_pass_list() {
        let tmp = tempfile::tempdir().expect("tmp");
        std::fs::write(
            tmp.path().join("config.json"),
            r#"{"v":1,"claude_env_keep":[1]}"#,
        )
        .expect("write");
        assert_eq!(
            persistent_names(tmp.path()).1,
            Some(ConfigRejection::Malformed)
        );
    }

    #[test]
    fn persistent_names_are_the_pass_list_on_unix() {
        let tmp = tempfile::tempdir().expect("tmp");
        std::fs::write(
            tmp.path().join("config.json"),
            r#"{"v":1,"claude_env_keep":["CLAUDE_KEEP_ME"]}"#,
        )
        .expect("write");
        assert_eq!(
            persistent_names(tmp.path()),
            (vec!["CLAUDE_KEEP_ME".to_owned()], None)
        );
    }
}
