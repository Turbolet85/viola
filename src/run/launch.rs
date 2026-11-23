//! The model and effort a start hands its child: the pair given at the start, each missing half
//! from `<home>/config.json`'s `default_model` / `default_effort`, else from the compiled
//! `DEFAULT_PAIR`.

use std::ffi::OsString;
use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use serde_json::{Map, Value};
use viola_agent_claude::{DEFAULT_PAIR, EFFORT_FLAG, MODEL_FLAG, is_effort_level};
use viola_core::{MAX_FRAME, is_model_word};

pub(crate) use viola_state::snapshot::Launch as Pair;

use crate::obs::ConfigRejection;

const MODEL_KEY: &str = "default_model";
const EFFORT_KEY: &str = "default_effort";

/// The compiled default pair.
pub(crate) fn compiled() -> Pair {
    Pair {
        model: DEFAULT_PAIR.0.to_owned(),
        effort: DEFAULT_PAIR.1.to_owned(),
    }
}

/// The configured default pair, and one rejection per key whose value does not parse: that half is
/// then the compiled default's. An absent key takes the compiled half with no rejection.
pub(crate) fn read_default(home: &Path) -> (Pair, Vec<ConfigRejection>) {
    let mut bytes = Vec::new();
    // An absent, unreadable or malformed document is `read_diagnostics_level`'s to report, once.
    match File::open(home.join("config.json")) {
        Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {
            Ok(_) => parse_default(&bytes),
            Err(_) => (compiled(), Vec::new()),
        },
        Err(_) => (compiled(), Vec::new()),
    }
}

fn parse_default(bytes: &[u8]) -> (Pair, Vec<ConfigRejection>) {
    let Ok(Value::Object(map)) = serde_json::from_slice::<Value>(bytes) else {
        return (compiled(), Vec::new());
    };
    if map.get("v").and_then(Value::as_u64) != Some(1) {
        return (compiled(), Vec::new());
    }
    let mut rejections = Vec::new();
    let model = half(
        &map,
        MODEL_KEY,
        is_model_word,
        DEFAULT_PAIR.0,
        &mut rejections,
    );
    let effort = half(
        &map,
        EFFORT_KEY,
        is_effort_level,
        DEFAULT_PAIR.1,
        &mut rejections,
    );
    (Pair { model, effort }, rejections)
}

fn half(
    map: &Map<String, Value>,
    key: &str,
    parses: fn(&str) -> bool,
    compiled: &str,
    rejections: &mut Vec<ConfigRejection>,
) -> String {
    match map.get(key) {
        None => compiled.to_owned(),
        Some(value) => match value.as_str().filter(|word| parses(word)) {
            Some(word) => word.to_owned(),
            None => {
                rejections.push(ConfigRejection::Malformed);
                compiled.to_owned()
            }
        },
    }
}

/// The pair handed: each half given wins, a missing half is the default's.
pub(crate) fn resolve(model: Option<String>, effort: Option<String>, default: Pair) -> Pair {
    Pair {
        model: model.unwrap_or(default.model),
        effort: effort.unwrap_or(default.effort),
    }
}

/// Whether the caller's words after `--` name a model or an effort themselves, in the spaced or
/// the `=` form: the pair is given to viola, never past it.
pub(crate) fn names_pair(words: &[OsString]) -> bool {
    words.iter().any(|word| {
        let word = word.as_encoded_bytes();
        [MODEL_FLAG, EFFORT_FLAG].iter().any(|flag| {
            let flag = flag.as_bytes();
            word == flag || (word.starts_with(flag) && word.get(flag.len()) == Some(&b'='))
        })
    })
}

/// The one usage line of a pair typed after `--`: it names viola's own flags and echoes none of
/// the caller's words.
pub(crate) const PAIR_AFTER_DASHES: &str = "give the model and the effort to viola as '--model <MODEL>' and '--effort <EFFORT>' before '--', not to the program after it\n";

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    fn default_of(config: Option<&str>) -> (Pair, Vec<ConfigRejection>) {
        let tmp = tempfile::tempdir().expect("tempdir");
        if let Some(config) = config {
            std::fs::write(tmp.path().join("config.json"), config).expect("write");
        }
        read_default(tmp.path())
    }

    fn pair(model: &str, effort: &str) -> Pair {
        Pair {
            model: model.to_owned(),
            effort: effort.to_owned(),
        }
    }

    #[rstest]
    #[case::absent_document(None, None, None, 0)]
    #[case::no_keys(Some(r#"{"v":1}"#), None, None, 0)]
    #[case::both_good(
        Some(r#"{"v":1,"default_model":"haiku","default_effort":"low"}"#),
        Some("haiku"),
        Some("low"),
        0
    )]
    #[case::a_bad_model(
        Some(r#"{"v":1,"default_model":"Haiku","default_effort":"low"}"#),
        None,
        Some("low"),
        1
    )]
    #[case::a_bad_effort(
        Some(r#"{"v":1,"default_model":"haiku","default_effort":"medium-ish"}"#),
        Some("haiku"),
        None,
        1
    )]
    #[case::both_bad(
        Some(r#"{"v":1,"default_model":"a b","default_effort":"none"}"#),
        None,
        None,
        2
    )]
    #[case::another_version(
        Some(r#"{"v":2,"default_model":"haiku","default_effort":"low"}"#),
        None,
        None,
        0
    )]
    #[case::a_non_string_value(
        Some(r#"{"v":1,"default_model":7,"default_effort":["low"]}"#),
        None,
        None,
        2
    )]
    fn read_default_pair_cases(
        #[case] config: Option<&str>,
        #[case] model: Option<&str>,
        #[case] effort: Option<&str>,
        #[case] rejected: usize,
    ) {
        let (got, rejections) = default_of(config);
        assert_eq!(
            got,
            pair(
                model.unwrap_or(DEFAULT_PAIR.0),
                effort.unwrap_or(DEFAULT_PAIR.1)
            )
        );
        assert_eq!(rejections, vec![ConfigRejection::Malformed; rejected]);
    }

    /// The read stops at `MAX_FRAME`: a document one byte longer loses its closing brace, so it reads
    /// as no document, with no rejection, and the same document at the cap reads the pair.
    #[test]
    fn read_default_pair_keeps_no_key_past_the_frame_cap() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let home = tmp.path().join("home");
        viola_state::fs::create_private_dir(&home).expect("home");
        let (head, tail) = (
            r#"{"v":1,"pad":""#,
            r#"","default_model":"haiku","default_effort":"low"}"#,
        );
        let document = |len: usize| {
            let pad = "x".repeat(len - head.len() - tail.len());
            format!("{head}{pad}{tail}")
        };
        let cap = usize::try_from(MAX_FRAME).expect("fits");
        std::fs::write(home.join("config.json"), document(cap + 1)).expect("config");
        assert_eq!(read_default(&home), (compiled(), Vec::new()));
        std::fs::write(home.join("config.json"), document(cap)).expect("config");
        assert_eq!(read_default(&home), (pair("haiku", "low"), Vec::new()));
    }

    #[test]
    fn resolve_pair_takes_each_missing_half_from_the_default() {
        let default = pair("sonnet", "high");
        assert_eq!(
            resolve(Some("haiku".into()), Some("low".into()), default.clone()),
            pair("haiku", "low")
        );
        assert_eq!(
            resolve(Some("haiku".into()), None, default.clone()),
            pair("haiku", "high")
        );
        assert_eq!(
            resolve(None, Some("low".into()), default.clone()),
            pair("sonnet", "low")
        );
        assert_eq!(resolve(None, None, default.clone()), default);
    }

    #[test]
    fn names_pair_finds_the_flag_and_its_equals_form() {
        let words = |list: &[&str]| list.iter().map(OsString::from).collect::<Vec<_>>();
        for named in [
            &["--model", "x"][..],
            &["--effort=low"],
            &["--model=x"],
            &["--resume", "id", "--effort", "high"],
        ] {
            assert!(names_pair(&words(named)), "{named:?}");
        }
        for clean in [
            &[][..],
            &["--modelx"],
            &["--efforts"],
            &["x", "-model"],
            &["model", "effort"],
        ] {
            assert!(!names_pair(&words(clean)), "{clean:?}");
        }
    }

    #[test]
    fn pair_after_dashes_names_the_two_flags() {
        assert_eq!(
            PAIR_AFTER_DASHES,
            "give the model and the effort to viola as '--model <MODEL>' and '--effort <EFFORT>' \
             before '--', not to the program after it\n"
        );
    }
}
