//! viola's one seam to the Hyprland compositor (architecture §Established Decisions →
//! window-features-target-hyprland): the relied-on requests as a closed set, one bounded read of the
//! `hyprctl` program spawned directly, tolerant parsers into closed viola types and closed outcome
//! words. Every reply is upstream output: its text is dropped at the parse and reaches no line,
//! error or refusal.

use std::ffi::OsString;

use nutype::nutype;

pub mod read;
pub mod reply;

pub use read::{HYPRCTL_DEADLINE, Read, read};
pub use reply::{HyprlandVersion, Reply, Window, Workspace};

/// The program viola spawns; the caller resolves it on `PATH` and passes the absolute path.
pub const PROGRAM: &str = "hyprctl";

/// A window's address as Hyprland prints it: `0x` and 1 to 16 lower-case hex digits. Any other
/// byte, a control character included, is refused, never stripped.
#[nutype(
    validate(predicate = is_address),
    derive(Debug, Clone, PartialEq, Eq, AsRef)
)]
pub struct Address(String);

/// A workspace's numeric id.
#[nutype(derive(Debug, Clone, Copy, PartialEq, Eq))]
pub struct WorkspaceId(i32);

/// Whether `text` is an address's closed form.
pub fn is_address(text: &str) -> bool {
    let Some(digits) = text.strip_prefix("0x") else {
        return false;
    };
    (1..=16).contains(&digits.len())
        && digits
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// The relied-on requests, closed: four queries and two dispatches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Request {
    Clients,
    ActiveWindow,
    Workspaces,
    Version,
    FocusWindow(Address),
    Workspace(WorkspaceId),
}

impl Request {
    /// The argv after the program, one element each; no element is built from a reply's text. A
    /// dispatch is one Lua dispatcher expression: 0.56.2 runs `dispatch <args>` as
    /// `hl.dispatch(<args>)` (measured live), and only a closed `Address` or an integer enters it.
    pub fn args(&self) -> Vec<OsString> {
        let words: Vec<String> = match self {
            Self::Clients => vec!["-j".into(), "clients".into()],
            Self::ActiveWindow => vec!["-j".into(), "activewindow".into()],
            Self::Workspaces => vec!["-j".into(), "workspaces".into()],
            Self::Version => vec!["-j".into(), "version".into()],
            Self::FocusWindow(address) => vec![
                "dispatch".into(),
                format!(
                    "hl.dsp.focus({{ window = \"address:{}\" }})",
                    address.as_ref()
                ),
            ],
            Self::Workspace(id) => vec![
                "dispatch".into(),
                format!("hl.dsp.focus({{ workspace = {} }})", id.into_inner()),
            ],
        };
        words.into_iter().map(OsString::from).collect()
    }

    /// The closed kebab-case word a fixture is named for.
    pub fn word(&self) -> &'static str {
        match self {
            Self::Clients => "clients",
            Self::ActiveWindow => "active-window",
            Self::Workspaces => "workspaces",
            Self::Version => "version",
            Self::FocusWindow(_) => "focus-window",
            Self::Workspace(_) => "workspace",
        }
    }

    /// Whether this request changes the compositor's state rather than reading it.
    pub fn is_dispatch(&self) -> bool {
        matches!(self, Self::FocusWindow(_) | Self::Workspace(_))
    }
}

/// Every request word, in declaration order.
pub const WORDS: [&str; 6] = [
    "clients",
    "active-window",
    "workspaces",
    "version",
    "focus-window",
    "workspace",
];

/// Why a read gave no reply; each word is closed and each message fixed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum HyprlandError {
    #[error("hyprctl not found")]
    Unsupported,
    #[error("hyprctl did not answer")]
    Unreachable,
    #[error("hyprctl reply over the frame cap")]
    Oversize,
    #[error("hyprctl reply not understood")]
    Malformed,
    #[error("hyprctl refused the dispatch")]
    Refused,
}

impl HyprlandError {
    pub const ALL: [Self; 5] = [
        Self::Unsupported,
        Self::Unreachable,
        Self::Oversize,
        Self::Malformed,
        Self::Refused,
    ];

    pub fn code(self) -> &'static str {
        match self {
            Self::Unsupported => "unsupported",
            Self::Unreachable => "unreachable",
            Self::Oversize => "oversize",
            Self::Malformed => "malformed",
            Self::Refused => "refused",
        }
    }
}

/// Line capture for this crate's unit tests: the fields of every line a closure wrote (plus
/// `level`) as JSON, on the calling thread only.
#[cfg(test)]
mod test_capture {
    use std::sync::{Arc, Mutex};

    use serde_json::{Map, Value};
    use tracing::field::{Field, Visit};
    use tracing::{Event, Subscriber};
    use tracing_subscriber::layer::{Context, Layer, SubscriberExt as _};

    /// Numbers and bools as themselves, a `Debug`-only value as its `Debug` text.
    #[derive(Default)]
    struct Fields(Map<String, Value>);

    impl Visit for Fields {
        fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
            self.0
                .insert(field.name().to_owned(), format!("{value:?}").into());
        }
        fn record_str(&mut self, field: &Field, value: &str) {
            self.0.insert(field.name().to_owned(), value.into());
        }
        fn record_u64(&mut self, field: &Field, value: u64) {
            self.0.insert(field.name().to_owned(), value.into());
        }
        fn record_i64(&mut self, field: &Field, value: i64) {
            self.0.insert(field.name().to_owned(), value.into());
        }
        fn record_bool(&mut self, field: &Field, value: bool) {
            self.0.insert(field.name().to_owned(), value.into());
        }
    }

    #[derive(Clone, Default)]
    struct Captured(Arc<Mutex<Vec<Value>>>);

    impl<S: Subscriber> Layer<S> for Captured {
        fn on_event(&self, event: &Event<'_>, _: Context<'_, S>) {
            let mut fields = Fields::default();
            event.record(&mut fields);
            fields.0.insert(
                "level".to_owned(),
                event.metadata().level().to_string().into(),
            );
            self.0.lock().expect("lines").push(Value::Object(fields.0));
        }
    }

    /// What `f` returned, and the lines it wrote.
    pub(crate) fn capture<R>(f: impl FnOnce() -> R) -> (R, Vec<Value>) {
        let captured = Captured::default();
        let subscriber = tracing_subscriber::registry().with(captured.clone());
        let result = tracing::subscriber::with_default(subscriber, f);
        let lines = captured.0.lock().expect("lines").clone();
        (result, lines)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn address(text: &str) -> Address {
        Address::try_new(text.to_owned()).expect("an address")
    }

    #[rstest]
    #[case::one_digit("0x0")]
    #[case::measured_width("0x55d4c1a2b3c0")]
    #[case::sixteen_digits("0xffffffffffffffff")]
    fn address_takes_the_closed_form(#[case] text: &str) {
        assert!(Address::try_new(text.to_owned()).is_ok());
        assert!(is_address(text));
    }

    #[rstest]
    #[case::empty("")]
    #[case::no_digits("0x")]
    #[case::no_prefix("55d4c1a2")]
    #[case::upper_case_digit("0x55D4")]
    #[case::upper_case_prefix("0X55d4")]
    #[case::seventeen_digits("0x1ffffffffffffffff")]
    #[case::not_hex("0x55g4")]
    #[case::a_space("0x55 d4")]
    #[case::a_newline("0x55d4\n")]
    #[case::a_nul("0x55\u{0}d4")]
    #[case::an_escape("0x55\u{1b}d4")]
    #[case::a_c1("0x55\u{85}d4")]
    #[case::del("0x55\u{7f}")]
    fn address_refuses_every_other_text(#[case] text: &str) {
        assert!(Address::try_new(text.to_owned()).is_err());
        assert!(!is_address(text));
    }

    #[test]
    fn request_args_are_the_closed_argv_one_element_each() {
        let argv = |request: Request| {
            request
                .args()
                .into_iter()
                .map(|a| a.into_string().expect("utf-8"))
                .collect::<Vec<_>>()
        };
        assert_eq!(argv(Request::Clients), ["-j", "clients"]);
        assert_eq!(argv(Request::ActiveWindow), ["-j", "activewindow"]);
        assert_eq!(argv(Request::Workspaces), ["-j", "workspaces"]);
        assert_eq!(argv(Request::Version), ["-j", "version"]);
        assert_eq!(
            argv(Request::FocusWindow(address("0x55d4c1a2b3c0"))),
            [
                "dispatch",
                r#"hl.dsp.focus({ window = "address:0x55d4c1a2b3c0" })"#
            ]
        );
        assert_eq!(
            argv(Request::Workspace(WorkspaceId::new(-98))),
            ["dispatch", "hl.dsp.focus({ workspace = -98 })"]
        );
    }

    #[test]
    fn request_words_are_closed_code_words_in_order() {
        let requests = [
            Request::Clients,
            Request::ActiveWindow,
            Request::Workspaces,
            Request::Version,
            Request::FocusWindow(address("0x1")),
            Request::Workspace(WorkspaceId::new(1)),
        ];
        let words: Vec<&str> = requests.iter().map(Request::word).collect();
        assert_eq!(words, WORDS);
        for word in WORDS {
            assert!(viola_core::is_code_word(word), "{word}");
        }
        let dispatches: Vec<bool> = requests.iter().map(Request::is_dispatch).collect();
        assert_eq!(dispatches, [false, false, false, false, true, true]);
    }

    #[test]
    fn hyprland_error_words_and_messages_are_fixed() {
        let pinned: Vec<(&str, String)> = HyprlandError::ALL
            .iter()
            .map(|e| (e.code(), e.to_string()))
            .collect();
        assert_eq!(
            pinned,
            [
                ("unsupported", "hyprctl not found".to_owned()),
                ("unreachable", "hyprctl did not answer".to_owned()),
                ("oversize", "hyprctl reply over the frame cap".to_owned()),
                ("malformed", "hyprctl reply not understood".to_owned()),
                ("refused", "hyprctl refused the dispatch".to_owned()),
            ]
        );
        for error in HyprlandError::ALL {
            assert!(viola_core::is_code_word(error.code()));
        }
    }
}
