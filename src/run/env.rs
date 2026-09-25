//! Which environment names the user defined persistently, so the R8 strip leaves them in the
//! child. Windows: the value names of the user and machine `Environment` keys. Unix: the
//! `config.json` `claude_env_keep` pass-list. Names only: no value is ever read.

use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use serde_json::Value;
use viola_core::MAX_FRAME;

use crate::obs::ConfigRejection;

const KEEP_KEY: &str = "claude_env_keep";
const KEEP_MAX: usize = 32;

/// The persistent names, and a rejection when `claude_env_keep` is present but unusable (checked
/// on every OS, consulted on Unix only).
pub(crate) fn persistent_names(home: &Path) -> (Vec<String>, Option<ConfigRejection>) {
    let (keep, rejection) = read_env_keep(home);
    #[cfg(windows)]
    let names = {
        let _ = keep;
        registry_names()
    };
    #[cfg(not(windows))]
    let names = keep;
    (names, rejection)
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

#[cfg(windows)]
fn registry_names() -> Vec<String> {
    use windows_sys::Win32::System::Registry::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
    let mut names = key_value_names(HKEY_CURRENT_USER, "Environment");
    names.extend(key_value_names(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment",
    ));
    names
}

/// The value names under one key; an unreadable key reads as none.
#[cfg(windows)]
fn key_value_names(root: windows_sys::Win32::System::Registry::HKEY, subkey: &str) -> Vec<String> {
    use windows_sys::Win32::Foundation::ERROR_SUCCESS;
    use windows_sys::Win32::System::Registry::{
        KEY_READ, RegCloseKey, RegEnumValueW, RegOpenKeyExW,
    };
    let wide: Vec<u16> = subkey.encode_utf16().chain(Some(0)).collect();
    let mut names = Vec::new();
    // SAFETY: the key handle is opened and closed here, and every buffer passed is owned here
    // with its length stated.
    unsafe {
        let mut key = std::ptr::null_mut();
        if RegOpenKeyExW(root, wide.as_ptr(), 0, KEY_READ, &mut key) != ERROR_SUCCESS {
            return names;
        }
        let mut buf = vec![0u16; 16_384];
        for index in 0.. {
            let mut len = u32::try_from(buf.len()).unwrap_or(u32::MAX);
            let status = RegEnumValueW(
                key,
                index,
                buf.as_mut_ptr(),
                &mut len,
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            if status != ERROR_SUCCESS {
                break;
            }
            let len = usize::try_from(len).unwrap_or(0).min(buf.len());
            names.push(String::from_utf16_lossy(&buf[..len]));
        }
        RegCloseKey(key);
    }
    names
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

    #[test]
    fn persistent_names_report_a_bad_pass_list_on_every_os() {
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

    #[cfg(not(windows))]
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

    #[cfg(windows)]
    #[test]
    fn persistent_names_are_the_registry_value_names_on_windows() {
        let tmp = tempfile::tempdir().expect("tmp");
        std::fs::write(
            tmp.path().join("config.json"),
            r#"{"v":1,"claude_env_keep":["CLAUDE_KEEP_ME"]}"#,
        )
        .expect("write");
        let (names, rejection) = persistent_names(tmp.path());
        assert_eq!(rejection, None);
        assert!(!names.iter().any(|n| n == "CLAUDE_KEEP_ME"));
        // The machine Environment key always defines OS (= Windows_NT) and Path.
        assert!(
            names.iter().any(|n| n.eq_ignore_ascii_case("OS")),
            "{names:?}"
        );
        assert!(names.iter().any(|n| n.eq_ignore_ascii_case("Path")));
    }

    #[cfg(windows)]
    #[test]
    fn key_value_names_of_a_missing_key_are_none() {
        use windows_sys::Win32::System::Registry::HKEY_CURRENT_USER;
        assert!(key_value_names(HKEY_CURRENT_USER, r"Software\viola-no-such-key-7c1e").is_empty());
    }
}
