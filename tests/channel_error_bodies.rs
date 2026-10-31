//! The channel's error bodies (security-plan §Error Handling; test-plan §6 Security control
//! negatives): frames written straight to a real wrapper's endpoint, past every CLI check, each
//! carrying the home's path, an upstream text and a planted stripped value, are answered by literal
//! bodies holding none of them, and the served `channel-response` lines tell the three `-32602`
//! causes apart in closed words (obs-plan §5, §6). `-32603` stays at the unit layer, so no counted
//! panic line lands in a scanned home.

#[allow(dead_code)]
mod support;

use std::ffi::OsString;
use std::io::{BufRead as _, BufReader, Write as _};
use std::path::Path;
use std::time::Instant;

use interprocess::local_socket::traits::Stream as _;
use interprocess::local_socket::{GenericFilePath, Stream, ToFsName as _};
use rstest::rstest;
use serde_json::{Value, json};
use support::events::{assert_error_body_clean, wait_events};
use support::fake;
use support::home::{
    StampedHome, UNWRAPPED, VIOLA, seed_conpty, snapshot_data, stamped_home, wait_endpoint_gone,
    workspace_path,
};
use support::ndjson::read_lines;
use support::outer_pty::{EXIT_WITHIN, OuterPty};
use support::watch::{WITHIN, Watch};
use viola_pty::Size;

/// A stripped name no reading keeps, and its value in the secret scan's table.
const STRIPPED_NAME: &str = "CLAUDE_VIOLA_TEST_NEW_X";
const STRIPPED_VALUE: &str = "canary-newname-value-e2f8";
const UPSTREAM: &str = "canary-chain-value-5c1e upstream text";

const INVALID_PARAMS: &str = r#""error":{"code":-32602,"message":"invalid params","data":null}"#;

/// `viola run builder -- <fake agent>` under an outer PTY with the stripped value on that child, its
/// SessionStart record landed.
fn booted_with_the_value(stamped: &StampedHome) -> OuterPty {
    let home = stamped.home.path();
    let cwd = std::env::current_dir().expect("cwd");
    let mut args: Vec<OsString> = vec!["--home".into(), home.into()];
    args.extend(["run", "builder", "--"].map(OsString::from));
    args.push(stamped.fake.clone().into());
    args.push("--control".into());
    args.push(fake::control_path(home, "builder").into());
    args.push("--receipt".into());
    args.push(fake::receipt_path(home, "builder").into());
    args.extend(["--cli-version", fake::RECORDED_CLI_VERSION, "--screens"].map(OsString::from));
    args.push("--trusted-root".into());
    args.push(cwd.clone().into());
    args.push("--fixtures".into());
    args.push(workspace_path("fixtures/claude").into());
    seed_conpty(home);
    let pty = OuterPty::spawn_without(
        Path::new(VIOLA),
        &args,
        &[(STRIPPED_NAME, STRIPPED_VALUE)],
        &UNWRAPPED,
        Size::DEFAULT,
        &cwd,
    );
    wait_events(
        &home.join("instances").join("builder"),
        "the session-start record",
        |lines| lines.iter().any(|line| line["kind"] == "session-start"),
    );
    pty
}

fn frame(id: u64, method: &str, params: &Value) -> String {
    json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params}).to_string() + "\n"
}

/// The role file's lines once every `(corr, detail)` of `wanted` is on it.
fn wait_details(home: &Path, wanted: &[(u64, &str)]) -> Vec<Value> {
    let role = home.join("diagnostics").join("run-builder.ndjson");
    let watch = Watch::start("details");
    let deadline = Instant::now() + WITHIN;
    loop {
        let lines = read_lines(&role);
        let landed = wanted.iter().all(|(corr, detail)| {
            lines.iter().any(|line| {
                line["event"] == "channel-response"
                    && line["corr"] == *corr
                    && line["detail"] == *detail
            })
        });
        if landed {
            return lines;
        }
        watch.note(&format!("lines {}", lines.len()));
        watch.deadline_check(deadline, "the channel-response details");
        std::thread::yield_now();
    }
}

#[rstest]
fn channel_error_bodies_hold_no_path_or_upstream_text(stamped_home: StampedHome) {
    let mut pty = booted_with_the_value(&stamped_home);
    let home = stamped_home.home.path().to_path_buf();
    let path = home.to_str().expect("a utf-8 home");
    let planted = [STRIPPED_VALUE, UPSTREAM];
    let data = snapshot_data(&home.join("instances").join("builder")).expect("the snapshot");
    let endpoint = data["endpoint"].as_str().expect("the endpoint").to_owned();
    let name = endpoint
        .as_str()
        .to_fs_name::<GenericFilePath>()
        .expect("an endpoint name");
    let mut stream = BufReader::new(Stream::connect(name).expect("connect"));

    let note = format!("{path} {UPSTREAM} {STRIPPED_VALUE}");
    let newer = r#""error":{"code":-32602,"message":"unsupported protocol version","data":{"supported":1,"wrapper":"0.1.0"}}"#;
    let driver = r#""error":{"code":-32602,"message":"invalid params","data":{"reason":"release-from-driver"}}"#;
    let rows: [(&str, String, u64, &str); 7] = [
        (
            "an unknown method named by the path",
            frame(1, path, &json!({"v": 1, "sender": path, "note": note})),
            1,
            r#""error":{"code":-32601,"message":"method not found","data":null}"#,
        ),
        (
            "a line that is no JSON",
            format!("{{not json {note}\n"),
            0,
            r#""error":{"code":-32700,"message":"parse error","data":null}"#,
        ),
        (
            "send from the path",
            frame(
                3,
                "send",
                &json!({"v": 1, "sender": path, "from": path, "text": note}),
            ),
            3,
            INVALID_PARAMS,
        ),
        (
            "send of a text that is a number",
            frame(
                4,
                "send",
                &json!({"v": 1, "sender": path, "text": 7, "note": note}),
            ),
            4,
            INVALID_PARAMS,
        ),
        (
            "a newer params.v",
            frame(
                5,
                "send",
                &json!({"v": 2, "sender": path, "text": note, "from": path}),
            ),
            5,
            newer,
        ),
        (
            "release from a driver",
            frame(6, "release", &json!({"v": 1, "sender": path, "from": path})),
            6,
            driver,
        ),
        (
            "link of the path as driver",
            frame(
                7,
                "link",
                &json!({"v": 1, "sender": path, "driver": path, "driven": "builder", "note": note}),
            ),
            7,
            INVALID_PARAMS,
        ),
    ];
    for (label, line, id, error) in &rows {
        stream
            .get_mut()
            .write_all(line.as_bytes())
            .expect("the frame");
        let mut reply = String::new();
        stream.read_line(&mut reply).expect("the reply");
        let body: Value = serde_json::from_str(&reply).expect("one JSON reply");
        // The check comes first: an assert_eq that fails prints what it read.
        assert_error_body_clean(label, &body, &home, &planted);
        let id = if *id == 0 {
            "null".to_owned()
        } else {
            id.to_string()
        };
        let wire = format!("{{\"jsonrpc\":\"2.0\",\"id\":{id},{error}}}\n");
        assert_eq!(reply, wire, "{label}");
    }

    let lines = wait_details(
        &home,
        &[
            (3, "invalid-params"),
            (4, "invalid-params"),
            (5, "unsupported-version"),
            (6, "release-from-driver"),
            (7, "invalid-params"),
        ],
    );
    let schema = support::hygiene::load_schema(&workspace_path("schemas/diag-line.v1.json"));
    let validator = jsonschema::validator_for(&schema).expect("the schema");
    for line in &lines {
        assert_error_body_clean("a role line", line, &home, &planted);
        assert!(validator.is_valid(line), "{line}");
    }
    let unknown_method = lines
        .iter()
        .find(|line| line["event"] == "channel-response" && line["corr"] == 1)
        .expect("the unknown method's line");
    assert!(unknown_method.get("detail").is_none(), "{unknown_method}");
    let mut another_word = lines
        .iter()
        .find(|line| line["event"] == "channel-response" && line["corr"] == 3)
        .expect("a -32602 line")
        .clone();
    another_word["detail"] = json!("another-word");
    assert!(
        !validator.is_valid(&another_word),
        "the schema admits only the three words"
    );

    pty.write(b"\x03");
    assert_eq!(pty.wait_exit(EXIT_WITHIN), 0);
    drop(pty);
    wait_endpoint_gone(&endpoint, "stop", |_| {});
}
