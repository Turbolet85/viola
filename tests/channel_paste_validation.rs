//! The wrapper re-runs the paste validator (security-plan §Input Validation, "Paste text"; test-plan
//! §5 Wrapper channel): a raw channel client that skips `viola send`'s own check still gets each
//! control class refused `not-delivered / control-character`, with nothing typed and no
//! `send-issued`; LF, CR, TAB and multibyte text are accepted, typed whole but for an inner CR,
//! which is typed as one LF (architecture [Delivery Confirmation]). A text with nothing left to
//! type is refused `not-delivered / empty-text` by the wrapper the same way.

#[allow(dead_code)]
mod support;

use std::path::Path;

use serde_json::{Map, Value, json};
use support::fake::{self, of_kind};
use support::home::{StampedHome, TestHome, Wrapper, snapshot_data, workspace_path};
use viola_channel::Client;

const CANARY: &str = "canary-chain-value-5c1e";

fn boot() -> Wrapper {
    let fixtures = workspace_path("fixtures/claude");
    let wrapper = Wrapper::boot(
        StampedHome::unstamped(TestHome::new()),
        "builder",
        None,
        &["--fixtures", fixtures.to_str().expect("utf-8 path")],
    );
    fake::wait_for(&wrapper.receipt(), "the SessionStart hook", |l| {
        !of_kind(l, "hook").is_empty()
    });
    wrapper
}

fn request(instance_dir: &Path, text: &str) -> Value {
    let endpoint = snapshot_data(instance_dir).expect("snapshot")["endpoint"]
        .as_str()
        .expect("endpoint")
        .to_owned();
    let mut client = Client::connect(&endpoint, "cli").expect("connect");
    let mut params = Map::new();
    params.insert("text".to_owned(), text.into());
    client.request("send", params).expect("reply")
}

fn events(instance_dir: &Path) -> Vec<Value> {
    support::ndjson::read_lines(&instance_dir.join("events.ndjson"))
}

#[test]
fn channel_paste_refuses_each_control_class() {
    let wrapper = boot();
    let dir = wrapper.instance_dir();
    for bad in ["\u{1b}[201~", "\u{0}", "\u{7f}", "\u{85}"] {
        let reply = request(&dir, &format!("{CANARY}{bad}tail"));
        assert_eq!(
            reply["result"],
            json!({"refusal": "not-delivered", "detail": "control-character"}),
            "{bad:?}"
        );
    }
    let events = events(&dir);
    assert!(!events.iter().any(|e| e["kind"] == "send-issued"));
    let refused: Vec<&Value> = events
        .iter()
        .filter(|e| e["kind"] == "send-refused")
        .collect();
    assert_eq!(refused.len(), 4);
    for record in refused {
        assert_eq!(
            record["data"],
            json!({"refusal": "not-delivered", "detail": "control-character"})
        );
    }
    let receipt = fake::receipt(&wrapper.receipt());
    assert!(of_kind(&receipt, "prompt").is_empty(), "a prompt was typed");
    let role = support::ndjson::read_lines(
        &wrapper
            .home()
            .join("diagnostics")
            .join("run-builder.ndjson"),
    );
    let lines: Vec<&Value> = role
        .iter()
        .filter(|l| l["event"] == "send-refused")
        .collect();
    assert_eq!(lines.len(), 4);
    assert!(lines.iter().all(|l| l["side"] == "wrapper"
        && l["detail"] == "control-character"
        && l["corr"].is_u64()
        && l["rpc_id"] == 1));
    wrapper.stop();
}

/// A raw client that skips `viola send`'s own check: the wrapper refuses a text whose typed text is
/// empty by itself, before anything is issued or typed. The inputs hold no content, so they carry
/// no canary.
#[test]
fn send_empty_text_straight_to_the_wrapper_is_refused() {
    let wrapper = boot();
    let dir = wrapper.instance_dir();
    let texts = ["", "\n", "\r\n"];
    for text in texts {
        let reply = request(&dir, text);
        assert_eq!(
            reply["result"],
            json!({"refusal": "not-delivered", "detail": "empty-text"}),
            "{text:?}"
        );
    }
    let events = events(&dir);
    assert!(!events.iter().any(|e| e["kind"] == "send-issued"));
    let refused: Vec<&Value> = events
        .iter()
        .filter(|e| e["kind"] == "send-refused")
        .collect();
    assert_eq!(refused.len(), texts.len());
    for record in refused {
        assert_eq!(
            record["data"],
            json!({"refusal": "not-delivered", "detail": "empty-text"}),
            "no cursor: refused before send-issued"
        );
    }
    let receipt = fake::receipt(&wrapper.receipt());
    assert!(of_kind(&receipt, "prompt").is_empty(), "a prompt was typed");
    let role = support::ndjson::read_lines(
        &wrapper
            .home()
            .join("diagnostics")
            .join("run-builder.ndjson"),
    );
    let lines: Vec<&Value> = role
        .iter()
        .filter(|l| l["event"] == "send-refused")
        .collect();
    assert_eq!(
        lines.len(),
        texts.len(),
        "one wrapper line for each request"
    );
    for line in lines {
        assert_eq!(line["side"], "wrapper");
        assert_eq!(line["refusal"], "not-delivered");
        assert_eq!(line["detail"], "empty-text");
        assert!(line["corr"].is_u64(), "{line}");
        assert!(line["conn"].is_string(), "{line}");
        assert_eq!(line["rpc_id"], 1);
    }
    wrapper.stop();
}

#[test]
fn channel_paste_accepts_lf_cr_tab_and_multibyte() {
    let wrapper = boot();
    let dir = wrapper.instance_dir();
    let text = format!("{CANARY}\nline\rcarriage\ttab é 中 🙂");
    let typed = format!("{CANARY}\nline\ncarriage\ttab é 中 🙂");
    let reply = request(&dir, &text);
    assert!(reply["result"]["ok"]["cursor"].is_u64(), "{reply}");
    // The agent receipts its prompt once the hook returns, which can be after the reply.
    let receipt = fake::wait_for(&wrapper.receipt(), "the typed prompt", |l| {
        !of_kind(l, "prompt").is_empty()
    });
    let prompts = of_kind(&receipt, "prompt");
    assert_eq!(prompts.len(), 1);
    assert_eq!(prompts[0]["text"], typed.as_str());
    assert_eq!(prompts[0]["bare_esc"], false);
    wrapper.stop();
}
