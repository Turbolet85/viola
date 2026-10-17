//! No setting disables a control (test-plan §5 CLI and env, Vector 6; security-plan §Security
//! Anti-Patterns → Universal): each row sets a variable to a disabling-shaped value and re-runs the
//! controls on its path, which must give the same verdict as without it. The rows cover the
//! hook-path controls, `send`'s paste control (under the hook seam and the `VIOLA_*` variables),
//! `answer` on an unstamped CLI, `send` under a human wheel or during a running turn, the budget
//! gate under a threshold key that is no percentage, the home scan under `VIOLA_DIR`, and the
//! identity strip under the `diagnostics_level` and `claude_env_keep` keys; the completeness case
//! keeps every flag `viola --help` lists, every `VIOLA_*` variable the product reads and every
//! `schemas/config.v1.json` key a row of `SETTINGS`.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

use rstest::rstest;
use serde_json::{Value, json};
use support::home::{StampedHome, TestHome, VIOLA, Wrapper, stamped_home, workspace_path};
use support::outer_pty::{EXIT_WITHIN, OuterPty};
use support::statusline::{feed, usage};
use support::watch::{WITHIN, Watch};

const CANARY: &str = "canary-chain-value-5c1e";

/// A hook-path control and the `hook-decision` detail it must reach.
#[derive(Clone, Copy, Debug)]
enum Control {
    OversizeStdin,
    MalformedJson,
}

impl Control {
    fn stdin(self) -> Vec<u8> {
        match self {
            Self::OversizeStdin => {
                let mut bytes = json!({"transcript_path": CANARY}).to_string().into_bytes();
                bytes.resize((16 << 20) + 1, b' ');
                bytes
            }
            Self::MalformedJson => format!("{{\"prompt\": \"{CANARY}\"").into_bytes(),
        }
    }

    fn detail(self) -> &'static str {
        match self {
            Self::OversizeStdin => "oversize-stdin",
            Self::MalformedJson => "malformed-json",
        }
    }
}

/// Every file under `dir`, recursively.
fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir).expect("read dir") {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            files.extend(files_under(&path));
        } else {
            files.push(path);
        }
    }
    files
}

/// `viola hook stop` on a fresh instance with `setting` over the child's environment: the
/// control's verdict must hold whatever the setting's value.
fn assert_control_holds(control: Control, setting: (&str, &str)) {
    let tmp = TestHome::new();
    let home = tmp.path().to_path_buf();
    let dir = home.join("instances").join("builder");
    let mut child = Command::new(VIOLA)
        .args(["hook", "stop"])
        .env("VIOLA_NAME", "builder")
        .env("VIOLA_DIR", dir.as_os_str())
        .env(setting.0, OsString::from(setting.1))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("viola hook");
    let mut input = child.stdin.take().expect("stdin");
    let stdin = control.stdin();
    let writer = std::thread::spawn(move || {
        let _ = input.write_all(&stdin);
    });
    let out = child.wait_with_output().expect("viola hook exits");
    let _ = writer.join();

    assert_eq!(out.status.code(), Some(0));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());
    let lines = support::ndjson::read_lines(&home.join("diagnostics").join("hook-builder.ndjson"));
    let decision = lines
        .iter()
        .find(|l| l["event"] == "hook-decision")
        .expect("a hook-decision line");
    assert_eq!(decision["detail"], control.detail());
    let logs = files_under(&home)
        .into_iter()
        .filter(|f| f.extension().is_some_and(|e| e == "ndjson"));
    for file in logs {
        let lines = support::ndjson::read_lines(&file);
        assert!(
            !lines.iter().any(|l: &Value| l["event"] == "panic"),
            "a panic line in the home"
        );
    }
}

#[rstest]
#[case::hook_panic_seam_zero(("FAKE_AGENT_HOOK_PANIC", "0"))]
#[case::hook_panic_seam_false(("FAKE_AGENT_HOOK_PANIC", "false"))]
#[case::hook_panic_seam_off(("FAKE_AGENT_HOOK_PANIC", "off"))]
#[case::hook_panic_seam_empty(("FAKE_AGENT_HOOK_PANIC", ""))]
fn setting_does_not_disable_the_hook_path_controls(
    #[case] setting: (&str, &str),
    #[values(Control::OversizeStdin, Control::MalformedJson)] control: Control,
) {
    assert_control_holds(control, setting);
}

/// An ESC-bearing `viola send`, with `setting` over its environment or none: the client refuses it
/// `not-delivered / control-character` (exit 13) before any frame, whatever the setting.
fn assert_send_paste_control_holds(setting: Option<(&str, &str)>) {
    let tmp = TestHome::new();
    let mut command = Command::new(VIOLA);
    command
        .arg("--home")
        .arg(tmp.path())
        .args(["send", "builder"])
        .env_remove("VIOLA_NAME")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some((name, value)) = setting {
        command.env(name, value);
    }
    let mut child = command.spawn().expect("viola send");
    let mut input = child.stdin.take().expect("stdin");
    input
        .write_all(format!("{CANARY}\x1b[201~\rtyped").as_bytes())
        .expect("stdin");
    drop(input);
    let out = child.wait_with_output().expect("viola send exits");

    assert_eq!(out.status.code(), Some(13));
    assert!(out.stdout.is_empty(), "stdout: {} bytes", out.stdout.len());
    assert_eq!(
        String::from_utf8_lossy(&out.stderr),
        "[/ ] unable         builder  not-delivered  control-character\n\
         hint: the text contains a control character (only LF, CR, TAB are allowed)\n"
    );
    let lines =
        support::ndjson::read_lines(&tmp.path().join("diagnostics").join("cli-builder.ndjson"));
    let refused = lines
        .iter()
        .find(|l| l["event"] == "send-refused")
        .expect("a send-refused line");
    assert_eq!(refused["side"], "client");
    assert_eq!(refused["refusal"], "not-delivered");
    assert_eq!(refused["detail"], "control-character");
    assert!(refused.get("corr").is_none());
    assert!(
        !tmp.path()
            .join("instances")
            .join("builder")
            .join("events.ndjson")
            .exists()
    );
}

#[rstest]
#[case::no_setting(None)]
#[case::hook_panic_seam_zero(Some(("FAKE_AGENT_HOOK_PANIC", "0")))]
#[case::hook_panic_seam_false(Some(("FAKE_AGENT_HOOK_PANIC", "false")))]
#[case::hook_panic_seam_off(Some(("FAKE_AGENT_HOOK_PANIC", "off")))]
#[case::hook_panic_seam_empty(Some(("FAKE_AGENT_HOOK_PANIC", "")))]
fn setting_does_not_disable_the_send_paste_control(#[case] setting: Option<(&str, &str)>) {
    assert_send_paste_control_holds(setting);
}

/// `viola answer` against a live wrapper on an unstamped home, with `setting` over its environment
/// or none: refused `unverified-cli` (exit 12), whatever the setting.
fn assert_answer_unstamped_control_holds(wrapper: &Wrapper, setting: Option<(&str, &str)>) {
    let mut command = Command::new(VIOLA);
    command
        .arg("--home")
        .arg(wrapper.home())
        .args(["answer", "builder", "1", "--json"])
        .env_remove("VIOLA_NAME")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some((name, value)) = setting {
        command.env(name, value);
    }
    let mut child = command.spawn().expect("viola answer");
    let mut input = child.stdin.take().expect("stdin");
    input
        .write_all(format!("{{\"behavior\": \"deny\", \"message\": \"{CANARY}\"}}").as_bytes())
        .expect("stdin");
    drop(input);
    let out = child.wait_with_output().expect("viola answer exits");

    assert_eq!(out.status.code(), Some(12), "{setting:?}");
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "{\"v\":1,\"refusal\":\"unverified-cli\",\"detail\":null}\n"
    );
}

#[test]
fn setting_does_not_disable_the_answer_unstamped_control() {
    let fixtures = workspace_path("fixtures/claude");
    let fixtures = fixtures.to_str().expect("utf-8 path");
    let wrapper = Wrapper::boot(
        StampedHome::unstamped(TestHome::new()),
        "builder",
        None,
        &["--fixtures", fixtures],
    );
    for setting in [
        None,
        Some(("FAKE_AGENT_HOOK_PANIC", "0")),
        Some(("FAKE_AGENT_HOOK_PANIC", "false")),
        Some(("FAKE_AGENT_HOOK_PANIC", "off")),
        Some(("FAKE_AGENT_HOOK_PANIC", "")),
    ] {
        assert_answer_unstamped_control_holds(&wrapper, setting);
    }
    wrapper.stop();
}

/// `viola send --json` against a live wrapper, with `setting` over its environment or none: refused
/// with exit `code` and the one document `refused`, whatever the setting.
fn assert_send_control_holds(
    wrapper: &Wrapper,
    setting: Option<(&str, &str)>,
    code: i32,
    refused: &str,
) {
    let mut command = Command::new(VIOLA);
    command
        .arg("--home")
        .arg(wrapper.home())
        .args(["send", "builder", "--json"])
        .env_remove("VIOLA_NAME")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some((name, value)) = setting {
        command.env(name, value);
    }
    let mut child = command.spawn().expect("viola send");
    let mut input = child.stdin.take().expect("stdin");
    input.write_all(CANARY.as_bytes()).expect("stdin");
    drop(input);
    let out = child.wait_with_output().expect("viola send exits");

    assert_eq!(out.status.code(), Some(code), "{setting:?}");
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());
    assert_eq!(String::from_utf8_lossy(&out.stdout), refused);
}

#[test]
fn setting_does_not_disable_the_send_human_wheel_control() {
    let wrapper = Wrapper::boot(
        StampedHome::unstamped(TestHome::new()),
        "builder",
        None,
        &[],
    );
    let paused = Command::new(VIOLA)
        .arg("--home")
        .arg(wrapper.home())
        .args(["pause", "builder", "--json"])
        .env_remove("VIOLA_NAME")
        .stdin(Stdio::null())
        .output()
        .expect("viola pause");
    assert_eq!(paused.status.code(), Some(0));
    for setting in [
        None,
        Some(("FAKE_AGENT_HOOK_PANIC", "0")),
        Some(("FAKE_AGENT_HOOK_PANIC", "false")),
        Some(("FAKE_AGENT_HOOK_PANIC", "off")),
        Some(("FAKE_AGENT_HOOK_PANIC", "")),
    ] {
        assert_send_control_holds(
            &wrapper,
            setting,
            10,
            "{\"v\":1,\"refusal\":\"human-typing\",\"detail\":\"manual-pause\"}\n",
        );
    }
    wrapper.stop();
}

/// `viola send --json` during a running harness turn (no Stop): refused `not-delivered` /
/// `turn-running` (exit 13), whatever the setting.
#[test]
fn setting_does_not_disable_the_send_turn_running_control() {
    let fixtures = workspace_path("fixtures/claude");
    let fixtures = fixtures.to_str().expect("utf-8 path");
    let wrapper = Wrapper::boot(
        StampedHome::unstamped(TestHome::new()),
        "builder",
        None,
        &["--fixtures", fixtures, "--inject-harness-turn"],
    );
    wrapper.release();
    let events = wrapper.instance_dir().join("events.ndjson");
    let watch = Watch::start("events");
    let deadline = Instant::now() + WITHIN;
    while !support::ndjson::read_lines(&events)
        .iter()
        .any(|l: &Value| l["kind"] == "prompt-submitted")
    {
        watch.deadline_check(
            deadline,
            "timed out waiting for the harness prompt-submitted",
        );
        std::thread::yield_now();
    }
    for setting in [
        None,
        Some(("FAKE_AGENT_HOOK_PANIC", "0")),
        Some(("FAKE_AGENT_HOOK_PANIC", "false")),
        Some(("FAKE_AGENT_HOOK_PANIC", "off")),
        Some(("FAKE_AGENT_HOOK_PANIC", "")),
    ] {
        assert_send_control_holds(
            &wrapper,
            setting,
            13,
            "{\"v\":1,\"refusal\":\"not-delivered\",\"detail\":\"turn-running\"}\n",
        );
    }
    wrapper.stop();
}

/// No value of a threshold key, and no `v`, switches the budget gate off (test-plan §5 Vector 6):
/// a threshold that is no whole percentage falls back to its default and a `v` other than 1 drops
/// the whole document to the defaults, the start says so once, and a `send` after a reading of 91
/// in that window is still refused (exit 11). The `v` row's own threshold, 95, would have let 91
/// through had the document been read.
#[rstest]
#[case::over_one_hundred(r#"{"v":1,"five_hour":101}"#, (91, 10), "five-hour")]
#[case::negative(r#"{"v":1,"five_hour":-1}"#, (91, 10), "five-hour")]
#[case::a_string(r#"{"v":1,"five_hour":"90"}"#, (91, 10), "five-hour")]
#[case::seven_day_over_one_hundred(r#"{"v":1,"seven_day":101}"#, (10, 91), "seven-day")]
#[case::v_two(r#"{"v":2,"five_hour":95}"#, (91, 10), "five-hour")]
fn setting_does_not_disable_the_budget_gate(
    stamped_home: StampedHome,
    #[case] config: &str,
    #[case] reading: (u64, u64),
    #[case] window: &str,
) {
    let home = stamped_home.home.path().to_path_buf();
    std::fs::write(home.join("config.json"), config).expect("config.json");
    let wrapper = support::events::boot(stamped_home, None, &[]);
    feed(&wrapper.instance_dir(), usage(reading.0, reading.1));
    assert_send_control_holds(
        &wrapper,
        None,
        11,
        &format!("{{\"v\":1,\"refusal\":\"budget-paused\",\"detail\":\"{window}\"}}\n"),
    );
    let rejected: Vec<Value> =
        support::ndjson::read_lines(&home.join("diagnostics").join("run-builder.ndjson"))
            .into_iter()
            .filter(|l| l["event"] == "parse-rejected")
            .collect();
    assert_eq!(rejected.len(), 1, "{rejected:?}");
    assert_eq!(rejected[0]["parser"], "config-json");
    assert_eq!(rejected[0]["detail"], "malformed");
    wrapper.stop();
}

/// Every setting a run can carry, beside the case that tries it against a control. A literal
/// table: the completeness case below fails on a flag, variable or key it does not name.
const SETTINGS: [(&str, &str); 9] = [
    (
        "--home",
        "cli_list::list_refuses_a_home_another_user_could_write",
    ),
    (
        "VIOLA_NAME",
        "viola_variable_does_not_disable_the_send_paste_control",
    ),
    ("VIOLA_DIR", "viola_dir_does_not_disable_the_home_scan"),
    (
        "VIOLA_BIN",
        "viola_variable_does_not_disable_the_send_paste_control",
    ),
    ("v", "setting_does_not_disable_the_budget_gate"),
    (
        "diagnostics_level",
        "config_key_does_not_disable_the_identity_strip",
    ),
    (
        "claude_env_keep",
        "config_key_does_not_disable_the_identity_strip",
    ),
    ("five_hour", "setting_does_not_disable_the_budget_gate"),
    ("seven_day", "setting_does_not_disable_the_budget_gate"),
];

/// clap's own flags, which carry no setting.
const CLAP_FLAGS: [&str; 2] = ["--help", "--version"];

fn named(setting: &str) -> bool {
    SETTINGS.iter().any(|(name, _)| *name == setting)
}

/// Every `--<word>` that starts a line of `viola --help`'s options.
fn long_flags(help: &str) -> Vec<String> {
    help.lines()
        .skip_while(|line| *line != "Options:")
        .filter_map(|line| {
            let flag = line
                .trim_start()
                .split([' ', ',', '=', '<'])
                .find(|w| w.starts_with("--"))?;
            Some(flag.to_owned())
        })
        .collect()
}

/// Every `"VIOLA_<NAME>"` string literal in the product sources (the root package's `src/` and each
/// library crate's, the test-only harness left out): the variables a build can read.
// andromeda:walks-tree
fn product_variables() -> Vec<String> {
    let mut roots = vec![workspace_path("src")];
    for crate_dir in std::fs::read_dir(workspace_path("crates")).expect("crates") {
        let crate_dir = crate_dir.expect("a crate").path();
        if crate_dir.file_name().is_some_and(|n| n != "viola-e2e") {
            roots.push(crate_dir.join("src"));
        }
    }
    let mut found = Vec::new();
    for file in roots
        .iter()
        .filter(|r| r.is_dir())
        .flat_map(|r| files_under(r))
    {
        if file.extension().is_none_or(|e| e != "rs") {
            continue;
        }
        let text = std::fs::read_to_string(&file).expect("a utf-8 source");
        for (at, _) in text.match_indices("\"VIOLA_") {
            let name: String = text[at + 1..]
                .chars()
                .take_while(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || *c == '_')
                .collect();
            if !found.contains(&name) {
                found.push(name);
            }
        }
    }
    found
}

/// test-plan §5 Cross-module patterns (CLI): no flag, `VIOLA_*` variable or `config.json` key is
/// left out of the table, so none goes untried against a control.
#[test]
fn every_flag_variable_and_config_key_is_a_row_of_the_table() {
    let help = Command::new(VIOLA)
        .arg("--help")
        .env_remove("VIOLA_NAME")
        .output()
        .expect("viola --help");
    assert_eq!(help.status.code(), Some(0));
    let flags = long_flags(&String::from_utf8_lossy(&help.stdout));
    assert!(flags.iter().any(|f| f == "--home"), "{flags:?}");
    for flag in &flags {
        assert!(
            named(flag) || CLAP_FLAGS.contains(&flag.as_str()),
            "{flag} is in no row"
        );
    }
    let variables = product_variables();
    assert!(!variables.is_empty(), "no VIOLA_* literal read");
    for variable in &variables {
        assert!(named(variable), "{variable} is in no row");
    }
    let schema: Value = serde_json::from_str(
        &std::fs::read_to_string(workspace_path("schemas/config.v1.json")).expect("the schema"),
    )
    .expect("json");
    let keys = schema["properties"].as_object().expect("properties");
    assert!(!keys.is_empty());
    for key in keys.keys() {
        assert!(named(key), "{key} is in no row");
    }
}

/// `VIOLA_DIR` choosing the home: `viola list` with no `--home` scans the home the variable names,
/// and a home another user could write refuses the whole board (exit 21), the variable no way
/// around the scan.
#[cfg(unix)]
#[test]
fn viola_dir_does_not_disable_the_home_scan() {
    use std::os::unix::fs::PermissionsExt as _;
    let tmp = TestHome::new();
    let home = tmp.path().to_path_buf();
    let dir = home.join("instances").join("builder");
    std::fs::create_dir_all(&dir).expect("the instance dir");
    for path in [&home, &home.join("instances"), &dir] {
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).expect("chmod");
    }
    std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o770)).expect("chmod");
    let empty = support::fake::empty_bin(tmp.scratch());
    let out = Command::new(VIOLA)
        .args(["list", "--json"])
        .env("VIOLA_NAME", "builder")
        .env("VIOLA_DIR", &dir)
        .env_remove("VIOLA_BIN")
        .env("PATH", &empty)
        .stdin(Stdio::null())
        .output()
        .expect("viola list");
    std::fs::set_permissions(&home, std::fs::Permissions::from_mode(0o700)).expect("chmod back");
    assert_eq!(out.status.code(), Some(21));
    assert!(out.stderr.is_empty(), "stderr: {} bytes", out.stderr.len());
    assert_eq!(
        String::from_utf8_lossy(&out.stdout),
        "{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}\n"
    );
    assert!(
        !home.join("diagnostics").exists(),
        "a refused scan opened a log"
    );
}

#[rstest]
#[case::name_empty(("VIOLA_NAME", ""))]
#[case::name_off(("VIOLA_NAME", "off"))]
#[case::name_zero(("VIOLA_NAME", "0"))]
#[case::bin_empty(("VIOLA_BIN", ""))]
#[case::bin_off(("VIOLA_BIN", "off"))]
#[case::bin_a_shell(("VIOLA_BIN", "/bin/sh"))]
fn viola_variable_does_not_disable_the_send_paste_control(#[case] setting: (&str, &str)) {
    assert_send_paste_control_holds(Some(setting));
}

/// The identity floor's names, one per family: the oracle, written out here.
const FLOOR: [(&str, &str); 2] = [
    ("CLAUDE_CODE_MESSAGING_TOKEN", "canary-token-value-7f3a"),
    ("CLAUDE_CODE_SESSION_ID", "canary-session-value-d4a0"),
];

/// `config.json` holding `config` (with `v` 1), then `viola run builder -- <fake agent>` with a
/// floor name in its environment: the names the child received.
fn child_names_under(config: &str) -> Vec<String> {
    let tmp = TestHome::new();
    let home = tmp.path();
    std::fs::create_dir_all(home).expect("home");
    std::fs::write(home.join("config.json"), format!("{{\"v\":1,{config}}}")).expect("config");
    let receipt = tmp.scratch().join("strip.receipt.ndjson");
    let args: Vec<OsString> = vec![
        "--home".into(),
        home.into(),
        "run".into(),
        "builder".into(),
        "--".into(),
        support::fake::FAKE.into(),
        "--receipt".into(),
        receipt.clone().into(),
    ];
    support::home::seed_conpty(home);
    let mut pty = OuterPty::spawn(Path::new(VIOLA), &args, &FLOOR);
    let lines = support::fake::wait_for(&receipt, "env", |l| {
        !support::fake::of_kind(l, "env").is_empty()
    });
    pty.write(b"\x03");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    support::fake::of_kind(&lines, "env")[0]["names"]
        .as_array()
        .expect("names")
        .iter()
        .map(|n| n.as_str().expect("name").to_owned())
        .collect()
}

/// `diagnostics_level` and `claude_env_keep` at disabling-shaped values: the identity floor is
/// stripped from the child all the same.
#[rstest]
#[case::level_off(r#""diagnostics_level":"off""#)]
#[case::level_a_list(r#""diagnostics_level":"debug,trace""#)]
#[case::keep_an_empty_name(r#""claude_env_keep":[""]"#)]
#[case::keep_a_wildcard(r#""claude_env_keep":["*"]"#)]
#[case::keep_a_floor_name(
    r#""claude_env_keep":["CLAUDE_CODE_MESSAGING_TOKEN","CLAUDE_CODE_SESSION_ID"]"#
)]
fn config_key_does_not_disable_the_identity_strip(#[case] config: &str) {
    let names = child_names_under(config);
    assert!(
        names.iter().any(|n| n.eq_ignore_ascii_case("PATH")),
        "{names:?}"
    );
    for (name, _) in FLOOR {
        assert!(!names.iter().any(|n| n == name), "{name} reached the child");
    }
}
