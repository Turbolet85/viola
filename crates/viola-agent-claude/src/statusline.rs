//! Claude Code's status line, as viola wraps it: the payload the CLI hands a statusline command on
//! stdin, the settings key that names the user's own command, the per-session override that puts
//! `viola hook statusline` in its place, and the shell a statusline command runs through
//! (architecture §Standard Contracts → Hook contract). Pure functions over bytes and strings: no
//! file, clock or environment is read here.

use chrono::{DateTime, Datelike as _, SecondsFormat, Utc};
use serde_json::{Value, json};
use viola_core::{BudgetReading, BudgetWindow, Reading};

use crate::AgentError;

/// The flag that hands the per-session settings override to the `claude` child.
pub const SETTINGS_FLAG: &str = "--settings";

/// The user-scope settings file, relative to the user's home: the one file read for the user's own
/// statusline command. Project, local and managed settings are not read.
pub const USER_SETTINGS: [&str; 2] = [".claude", "settings.json"];

/// Whether a session's status line is wrapped on this OS: no Windows reading of the shell exists.
const WRAPPED_HERE: bool = cfg!(unix);

/// The reading a statusline payload carries: none when it holds no `rate_limits` object (before
/// the first response, or for an account without one). Every other key is ignored.
pub fn reading(stdin: &[u8]) -> Result<Option<BudgetReading>, AgentError> {
    let Ok(Value::Object(payload)) = serde_json::from_slice::<Value>(stdin) else {
        return Err(AgentError::Malformed);
    };
    let Some(Value::Object(limits)) = payload.get("rate_limits") else {
        return Ok(None);
    };
    Ok(Some(BudgetReading {
        five_hour: window(limits.get("five_hour")),
        seven_day: window(limits.get("seven_day")),
    }))
}

fn window(value: Option<&Value>) -> Reading<BudgetWindow> {
    let Some(Value::Object(window)) = value else {
        return Reading::UNKNOWN;
    };
    Reading::Known(BudgetWindow {
        used_percentage: used_percentage(window.get("used_percentage")),
        resets_at: resets_at(window.get("resets_at")),
    })
}

fn used_percentage(value: Option<&Value>) -> Reading<f64> {
    match value.and_then(Value::as_f64) {
        Some(used) if (0.0..=100.0).contains(&used) => Reading::Known(used),
        _ => Reading::UNKNOWN,
    }
}

/// A whole number of Unix epoch seconds, or an RFC 3339 string, as RFC 3339 UTC with milliseconds.
/// A year outside 0 to 9999 has no RFC 3339 form.
fn resets_at(value: Option<&Value>) -> Reading<String> {
    let at = match value {
        Some(Value::Number(seconds)) => seconds
            .as_i64()
            .and_then(|seconds| DateTime::from_timestamp(seconds, 0)),
        Some(Value::String(text)) => DateTime::parse_from_rfc3339(text)
            .ok()
            .map(|at| at.with_timezone(&Utc)),
        _ => None,
    };
    match at {
        Some(at) if (0..=9999).contains(&at.year()) => {
            Reading::Known(at.to_rfc3339_opts(SecondsFormat::Millis, true))
        }
        _ => Reading::UNKNOWN,
    }
}

/// The user's own statusline command in a settings document: `statusLine.command` where
/// `statusLine.type` is `command`. Any other document holds no command; none is an error.
pub fn user_command(settings: &[u8]) -> Option<String> {
    let doc: Value = serde_json::from_slice(settings).ok()?;
    let line = doc.get("statusLine")?;
    if line.get("type")?.as_str()? != "command" {
        return None;
    }
    let command = line.get("command")?.as_str()?;
    (!command.is_empty() && !command.contains('\0')).then(|| command.to_owned())
}

/// The per-session settings override: a `statusLine` whose command is the pinned binary's
/// forward-slash absolute path, one space, `hook statusline`. The CLI runs that string through a
/// shell, and the path is written unquoted, so a path holding anything but ASCII letters, digits
/// and `_ - . / :` gets no override, and neither does any path where the status line is not wrapped.
pub fn override_document(pinned_bin_fwd: &str) -> Option<String> {
    override_on(pinned_bin_fwd, WRAPPED_HERE)
}

fn override_on(pinned_bin_fwd: &str, wrapped: bool) -> Option<String> {
    if !wrapped || !pinned_bin_fwd.chars().all(is_shell_plain) {
        return None;
    }
    let command = format!("{pinned_bin_fwd} hook statusline");
    Some(json!({"statusLine": {"type": "command", "command": command}}).to_string())
}

fn is_shell_plain(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | ':')
}

/// The shell a user's statusline command runs through, with the command as ONE argument: the
/// program by its absolute path, never a lookup. None where the status line is not wrapped.
pub fn shell_argv(command: &str) -> Option<[&str; 3]> {
    shell_argv_on(command, WRAPPED_HERE)
}

fn shell_argv_on(command: &str, wrapped: bool) -> Option<[&str; 3]> {
    wrapped.then_some(["/bin/sh", "-c", command])
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use proptest::test_runner::FileFailurePersistence;
    use rstest::rstest;

    fn known(used_percentage: Reading<f64>, resets_at: Reading<String>) -> Reading<BudgetWindow> {
        Reading::Known(BudgetWindow {
            used_percentage,
            resets_at,
        })
    }

    fn at(text: &str) -> Reading<String> {
        Reading::Known(text.to_owned())
    }

    fn read(payload: &Value) -> Option<BudgetReading> {
        reading(payload.to_string().as_bytes()).expect("one JSON object")
    }

    #[rstest]
    #[case::not_json(b"not json".as_slice())]
    #[case::empty(b"".as_slice())]
    #[case::an_array(br#"[{"rate_limits":{}}]"#.as_slice())]
    #[case::a_string(br#""rate_limits""#.as_slice())]
    #[case::a_number(b"7".as_slice())]
    #[case::null(b"null".as_slice())]
    #[case::cut_short(br#"{"rate_limits":{"five_hour":"#.as_slice())]
    #[case::two_objects(b"{}{}".as_slice())]
    fn statusline_reading_of_a_payload_that_is_not_one_object_is_malformed(#[case] stdin: &[u8]) {
        assert_eq!(reading(stdin), Err(AgentError::Malformed));
    }

    #[rstest]
    #[case::no_key(json!({"model": {"id": "m"}}))]
    #[case::empty_object(json!({}))]
    #[case::null(json!({"rate_limits": null}))]
    #[case::a_string(json!({"rate_limits": "unknown"}))]
    #[case::an_array(json!({"rate_limits": [{"five_hour": {}}]}))]
    #[case::a_number(json!({"rate_limits": 91}))]
    fn statusline_reading_without_a_rate_limits_object_is_none(#[case] payload: Value) {
        assert_eq!(read(&payload), None);
    }

    #[test]
    fn statusline_reading_takes_both_windows_and_ignores_every_other_key() {
        let payload = json!({
            "session_id": "s-1",
            "cost": {"total_cost_usd": 1.5},
            "rate_limits": {
                "five_hour": {"used_percentage": 23.5, "resets_at": 1_738_425_600, "later": 1},
                "seven_day": {"used_percentage": 91, "resets_at": 1_738_857_600},
                "spend_limit": {"used_percentage": 50, "resets_at": 1_738_425_600},
            },
        });
        assert_eq!(
            read(&payload),
            Some(BudgetReading {
                five_hour: known(Reading::Known(23.5), at("2025-02-01T16:00:00.000Z")),
                seven_day: known(Reading::Known(91.0), at("2025-02-06T16:00:00.000Z")),
            })
        );
    }

    #[rstest]
    #[case::absent(json!({}))]
    #[case::null(json!({"five_hour": null}))]
    #[case::a_number(json!({"five_hour": 12}))]
    #[case::a_string(json!({"five_hour": "12"}))]
    #[case::an_array(json!({"five_hour": [{"used_percentage": 12}]}))]
    fn statusline_reading_a_window_that_is_absent_or_no_object_is_unknown(#[case] limits: Value) {
        let got = read(&json!({"rate_limits": limits})).expect("a reading");
        assert_eq!(got.five_hour, Reading::UNKNOWN);
        assert_eq!(got.seven_day, Reading::UNKNOWN);
    }

    #[test]
    fn statusline_reading_each_window_stands_alone() {
        let limits = json!({"seven_day": {"used_percentage": 4, "resets_at": 0}});
        let got = read(&json!({"rate_limits": limits})).expect("a reading");
        assert_eq!(got.five_hour, Reading::UNKNOWN);
        assert_eq!(
            got.seven_day,
            known(Reading::Known(4.0), at("1970-01-01T00:00:00.000Z"))
        );
    }

    #[rstest]
    #[case::zero(json!(0), Some(0.0))]
    #[case::one_hundred(json!(100), Some(100.0))]
    #[case::a_fraction(json!(23.5), Some(23.5))]
    #[case::one_hundred_as_a_float(json!(100.0), Some(100.0))]
    #[case::just_below_zero(json!(-0.001), None)]
    #[case::minus_one(json!(-1), None)]
    #[case::just_past_one_hundred(json!(100.001), None)]
    #[case::one_hundred_and_one(json!(101), None)]
    #[case::a_huge_number(json!(u64::MAX), None)]
    #[case::a_numeric_string(json!("50"), None)]
    #[case::null(json!(null), None)]
    #[case::a_bool(json!(true), None)]
    #[case::an_object(json!({"value": 50}), None)]
    fn statusline_reading_used_percentage_is_kept_only_from_0_to_100(
        #[case] value: Value,
        #[case] kept: Option<f64>,
    ) {
        let limits = json!({"five_hour": {"used_percentage": value, "resets_at": 0}});
        let got = read(&json!({"rate_limits": limits})).expect("a reading");
        let want = kept.map_or(Reading::UNKNOWN, Reading::Known);
        assert_eq!(got.five_hour, known(want, at("1970-01-01T00:00:00.000Z")));
    }

    #[test]
    fn statusline_reading_a_window_without_used_percentage_has_it_unknown() {
        let limits = json!({"five_hour": {"resets_at": 60}});
        let got = read(&json!({"rate_limits": limits})).expect("a reading");
        assert_eq!(
            got.five_hour,
            known(Reading::UNKNOWN, at("1970-01-01T00:01:00.000Z"))
        );
    }

    #[rstest]
    #[case::epoch_seconds(json!(1_738_425_600), Some("2025-02-01T16:00:00.000Z"))]
    #[case::before_the_epoch(json!(-1), Some("1969-12-31T23:59:59.000Z"))]
    #[case::the_last_second_of_year_9999(json!(253_402_300_799_i64), Some("9999-12-31T23:59:59.000Z"))]
    #[case::the_first_second_of_year_10000(json!(253_402_300_800_i64), None)]
    #[case::the_first_second_of_year_0(json!(-62_167_219_200_i64), Some("0000-01-01T00:00:00.000Z"))]
    #[case::the_last_second_before_year_0(json!(-62_167_219_201_i64), None)]
    #[case::seconds_no_date_holds(json!(i64::MAX), None)]
    #[case::seconds_past_i64(json!(u64::MAX), None)]
    #[case::fractional_seconds(json!(1_738_425_600.5), None)]
    #[case::whole_seconds_written_as_a_float(json!(1_738_425_600.0), None)]
    #[case::rfc3339_utc(json!("2026-10-10T12:00:00Z"), Some("2026-10-10T12:00:00.000Z"))]
    #[case::rfc3339_with_an_offset(json!("2026-10-10T14:00:00+02:00"), Some("2026-10-10T12:00:00.000Z"))]
    #[case::rfc3339_with_fractions(json!("2026-10-10T12:00:00.123456Z"), Some("2026-10-10T12:00:00.123Z"))]
    #[case::rfc3339_an_offset_into_year_10000(json!("9999-12-31T23:59:59-01:00"), None)]
    #[case::a_date_alone(json!("2026-10-10"), None)]
    #[case::seconds_as_a_string(json!("1738425600"), None)]
    #[case::the_word_unknown(json!("unknown"), None)]
    #[case::empty(json!(""), None)]
    #[case::null(json!(null), None)]
    #[case::a_bool(json!(true), None)]
    #[case::an_array(json!([1_738_425_600]), None)]
    #[case::an_object(json!({"seconds": 1_738_425_600}), None)]
    fn statusline_reading_resets_at_is_epoch_seconds_or_rfc3339_as_utc_millis(
        #[case] value: Value,
        #[case] written: Option<&str>,
    ) {
        let limits = json!({"seven_day": {"used_percentage": 1, "resets_at": value}});
        let got = read(&json!({"rate_limits": limits})).expect("a reading");
        let want = written.map_or(Reading::UNKNOWN, at);
        assert_eq!(got.seven_day, known(Reading::Known(1.0), want));
    }

    #[test]
    fn statusline_reading_a_window_without_resets_at_has_it_unknown() {
        let limits = json!({"five_hour": {"used_percentage": 7}});
        let got = read(&json!({"rate_limits": limits})).expect("a reading");
        assert_eq!(got.five_hour, known(Reading::Known(7.0), Reading::UNKNOWN));
    }

    #[rstest]
    #[case::a_command(
        json!({"statusLine": {"type": "command", "command": "~/.claude/statusline.sh"}}),
        Some("~/.claude/statusline.sh")
    )]
    #[case::beside_other_keys(
        json!({"model": "m", "statusLine": {"type": "command", "command": "a b 'c d'", "padding": 0}}),
        Some("a b 'c d'")
    )]
    #[case::no_status_line(json!({"model": "m"}), None)]
    #[case::status_line_null(json!({"statusLine": null}), None)]
    #[case::status_line_a_string(json!({"statusLine": "echo x"}), None)]
    #[case::no_type(json!({"statusLine": {"command": "echo x"}}), None)]
    #[case::another_type(json!({"statusLine": {"type": "static", "command": "echo x"}}), None)]
    #[case::type_in_another_case(json!({"statusLine": {"type": "Command", "command": "echo x"}}), None)]
    #[case::type_not_a_string(json!({"statusLine": {"type": 1, "command": "echo x"}}), None)]
    #[case::no_command(json!({"statusLine": {"type": "command"}}), None)]
    #[case::command_not_a_string(json!({"statusLine": {"type": "command", "command": ["echo", "x"]}}), None)]
    #[case::empty_command(json!({"statusLine": {"type": "command", "command": ""}}), None)]
    #[case::a_nul_in_the_command(json!({"statusLine": {"type": "command", "command": "echo\u{0}x"}}), None)]
    #[case::an_array_document(json!([{"statusLine": {"type": "command", "command": "echo x"}}]), None)]
    fn statusline_user_command_is_the_command_of_a_command_status_line(
        #[case] settings: Value,
        #[case] command: Option<&str>,
    ) {
        assert_eq!(
            user_command(settings.to_string().as_bytes()).as_deref(),
            command
        );
    }

    #[rstest]
    #[case::not_json(b"not json".as_slice())]
    #[case::empty(b"".as_slice())]
    #[case::cut_short(br#"{"statusLine":{"type":"command","command":"echo"#.as_slice())]
    fn statusline_user_command_of_bytes_that_are_no_document_is_none(#[case] settings: &[u8]) {
        assert_eq!(user_command(settings), None);
    }

    #[test]
    fn statusline_override_is_the_pinned_path_then_hook_statusline() {
        let path = "/home/u/.viola/bin/0.1.0-0123456789abcdef/viola";
        assert_eq!(
            override_on(path, true).as_deref(),
            Some(
                r#"{"statusLine":{"type":"command","command":"/home/u/.viola/bin/0.1.0-0123456789abcdef/viola hook statusline"}}"#
            )
        );
        let drive = override_on("C:/h/bin/k/viola.exe", true).expect("an override");
        let doc: Value = serde_json::from_str(&drive).expect("json");
        assert_eq!(
            doc["statusLine"]["command"],
            "C:/h/bin/k/viola.exe hook statusline"
        );
    }

    /// The user's own command is read back from the override the way the CLI would read it.
    #[test]
    fn statusline_override_reads_back_as_a_command_status_line() {
        let doc = override_on("/h/bin/k/viola", true).expect("an override");
        assert_eq!(
            user_command(doc.as_bytes()).as_deref(),
            Some("/h/bin/k/viola hook statusline")
        );
    }

    #[rstest]
    #[case::a_space("/home/a user/.viola/bin/k/viola")]
    #[case::a_quote("/home/u'/.viola/bin/k/viola")]
    #[case::a_double_quote("/home/u\"/bin/k/viola")]
    #[case::a_dollar("/home/$USER/bin/k/viola")]
    #[case::a_backslash("C:\\h\\bin\\k\\viola.exe")]
    #[case::a_semicolon("/h/bin;k/viola")]
    #[case::an_ampersand("/h/bin&k/viola")]
    #[case::a_tilde("~/.viola/bin/k/viola")]
    #[case::a_paren("/h/bin(k)/viola")]
    #[case::a_newline("/h/bin\nk/viola")]
    #[case::a_non_ascii_letter("/home/\u{e9}/bin/k/viola")]
    #[case::a_non_ascii_digit("/home/\u{663}/bin/k/viola")]
    fn statusline_override_of_a_path_a_shell_could_misread_is_none(#[case] path: &str) {
        assert_eq!(override_on(path, true), None);
    }

    #[test]
    fn statusline_override_where_the_status_line_is_not_wrapped_is_none() {
        assert_eq!(override_on("/h/bin/k/viola", false), None);
        assert_eq!(override_on("C:/h/bin/k/viola.exe", false), None);
    }

    #[test]
    fn statusline_override_on_this_host_is_built_only_on_unix() {
        assert_eq!(override_document("/h/bin/k/viola").is_some(), cfg!(unix));
        assert_eq!(override_document("/h/bin k/viola"), None);
        assert_eq!(SETTINGS_FLAG, "--settings");
        assert_eq!(USER_SETTINGS, [".claude", "settings.json"]);
    }

    #[test]
    fn statusline_shell_is_bin_sh_with_the_command_as_one_argument() {
        let command = "printf '%s' \"a b\"; exit 3";
        assert_eq!(
            shell_argv_on(command, true),
            Some(["/bin/sh", "-c", command])
        );
        assert_eq!(shell_argv_on(command, false), None);
        assert_eq!(shell_argv(command).is_some(), cfg!(unix));
    }

    fn config() -> ProptestConfig {
        ProptestConfig {
            cases: 512,
            failure_persistence: Some(Box::new(FileFailurePersistence::SourceParallel(
                "proptest-regressions",
            ))),
            ..ProptestConfig::default()
        }
    }

    /// Any JSON value a statusline field could hold, weighted towards the two shapes the reader
    /// takes: seconds around the years an RFC 3339 text can hold (a little before year 0 to a
    /// little past year 9999), and RFC 3339 text.
    fn arbitrary_json() -> impl Strategy<Value = Value> {
        let leaf = prop_oneof![
            Just(Value::Null),
            any::<bool>().prop_map(Value::from),
            any::<i64>().prop_map(Value::from),
            (-70_000_000_000_i64..300_000_000_000_i64).prop_map(Value::from),
            any::<u64>().prop_map(Value::from),
            (-200.0f64..200.0).prop_map(Value::from),
            any::<f64>().prop_map(Value::from),
            ".{0,16}".prop_map(Value::from),
            "[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(Z|[+-][0-9]{2}:[0-9]{2})"
                .prop_map(Value::from),
        ];
        leaf.prop_recursive(2, 8, 3, |inner| {
            prop_oneof![
                prop::collection::vec(inner.clone(), 0..3).prop_map(Value::from),
                prop::collection::vec(("[a-z_]{1,6}", inner), 0..3)
                    .prop_map(|pairs| Value::Object(pairs.into_iter().collect())),
            ]
        })
    }

    fn round_trips(window: &Reading<BudgetWindow>) -> bool {
        let Reading::Known(window) = window else {
            return true;
        };
        let used = match &window.used_percentage {
            Reading::Known(used) => (0.0..=100.0).contains(used),
            Reading::Unknown(_) => true,
        };
        let resets = match &window.resets_at {
            Reading::Known(text) => DateTime::parse_from_rfc3339(text)
                .is_ok_and(|at| at.offset().local_minus_utc() == 0 && text.ends_with('Z')),
            Reading::Unknown(_) => true,
        };
        used && resets
    }

    proptest! {
        #![proptest_config(config())]

        #[test]
        fn statusline_prop_resets_at_never_fails(
            resets_at in arbitrary_json(),
            used_percentage in arbitrary_json(),
        ) {
            let window = json!({"used_percentage": used_percentage, "resets_at": resets_at});
            let payload = json!({"rate_limits": {"five_hour": window.clone(), "seven_day": window}});
            let text = payload.to_string();
            let got = reading(text.as_bytes());
            prop_assert!(matches!(got, Ok(Some(_))));
            let got = got.expect("a reading").expect("with rate limits");
            prop_assert!(round_trips(&got.five_hour));
            prop_assert!(round_trips(&got.seven_day));
            prop_assert_eq!(got.five_hour, got.seven_day);
        }
    }
}
