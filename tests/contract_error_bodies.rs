//! The error bodies of the three surfaces, enumerated (security-plan §Error Handling; test-plan §6
//! Contract suite): `CLI_DOCUMENTS`, `MCP_RESULTS` and `CHANNEL_FAULTS` name each body with its code,
//! its detail form, its writer and the test that witnesses it, `NOT_ERROR_BODIES` names what looks
//! like one and is not, and a census over the product sources reads red on a writer no row names and
//! on a `thiserror` message that interpolates. The tables are literals, never the product's own list.
//!
//! andromeda:walks-tree — it reads every `.rs` file under the product source roots, named or not.

use std::fs;
use std::path::{Path, PathBuf};

use rstest::rstest;
use viola_channel::ProtocolError;

/// One body a machine reader can be handed, and what holds it.
struct Body {
    code: &'static str,
    /// `null`, a closed word, a list of them, or the fault's closed object.
    detail: &'static str,
    /// The CLI's exit; `None` on a surface with no exit.
    exit: Option<u8>,
    file: &'static str,
    /// The text of its writer in `file`.
    needle: &'static str,
    witness: &'static str,
}

const THE_WITNESS_CLI: &str = "cli_error_documents_hold_no_path_or_upstream_text";
const THE_WITNESS_MCP: &str = "mcp_tools_error_results_hold_no_path_or_upstream_text";
const THE_WITNESS_CHANNEL: &str = "channel_error_bodies_hold_no_path_or_upstream_text";

const REVIVE: &str = "client::refused_document(self.detail());";
const SEND_REFUSAL: &str =
    r#"client::document(&json!({"v": 1, "refusal": reason, "detail": detail}));"#;

const CLI_DOCUMENTS: &[Body] = &[
    Body {
        code: "instance-unreachable",
        detail: "strict-modes-failed",
        exit: Some(21),
        file: "src/cmd/client.rs",
        needle: "unreachable_document(UnreachableCause::StrictModes);",
        witness: "frame_verb_refuses_a_widened_instance_before_any_log",
    },
    Body {
        code: "instance-unreachable",
        detail: "strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed",
        exit: Some(21),
        file: "src/cmd/client.rs",
        needle: r#"document(&json!({"v": 1, "error": "instance-unreachable", "detail": cause.as_str()}));"#,
        witness: THE_WITNESS_CLI,
    },
    Body {
        code: "instance-unreachable",
        detail: "strict-modes-failed",
        exit: Some(21),
        file: "src/cmd/list.rs",
        needle: r#"json!({"v": 1, "error": "instance-unreachable", "detail": cause})"#,
        witness: THE_WITNESS_CLI,
    },
    Body {
        code: "instance-unreachable",
        detail: "strict-modes-failed",
        exit: Some(21),
        file: "src/cmd/plugin.rs",
        needle: "client::unreachable_document(UnreachableCause::StrictModes);",
        witness: THE_WITNESS_CLI,
    },
    Body {
        code: "wrapper-fault",
        detail: "{code, message, data}, every field fixed or closed",
        exit: Some(20),
        file: "src/cmd/client.rs",
        needle: r#"json!({"v": 1, "error": "wrapper-fault", "detail": detail})"#,
        witness: THE_WITNESS_CLI,
    },
    Body {
        code: "strict-modes-failed",
        detail: "null",
        exit: Some(1),
        file: "src/cmd/revive.rs",
        needle: REVIVE,
        witness: "revive_json_refuses_state_another_user_could_write_on_both_arms",
    },
    Body {
        code: "already-live",
        detail: "null",
        exit: Some(1),
        file: "src/cmd/revive.rs",
        needle: REVIVE,
        witness: "revive_json_on_a_live_name_is_the_already_live_document",
    },
    Body {
        code: "already-live",
        detail: "null",
        exit: Some(1),
        file: "src/cmd/run.rs",
        needle: "super::client::refused_document(detail);",
        witness: "revive_json_on_a_live_name_is_the_already_live_document",
    },
    Body {
        code: "stale-heartbeat",
        detail: "null",
        exit: Some(1),
        file: "src/cmd/run.rs",
        needle: r#"Some(refused_with(json, "stale-heartbeat", || {"#,
        witness: "revive_json_of_a_stale_holder_is_the_stale_heartbeat_document",
    },
    Body {
        code: "batch-script-child",
        detail: "null",
        exit: Some(1),
        file: "src/cmd/run.rs",
        needle: r#"return Ok(refused_with(launch.json, "batch-script-child", || {"#,
        witness: "a_revive_whose_claude_is_a_script_is_the_batch_script_child_document",
    },
    Body {
        code: "pinned-hash-mismatch",
        detail: "null",
        exit: Some(1),
        file: "src/cmd/run.rs",
        needle: r#""pinned-hash-mismatch","#,
        witness: "a_tampered_pinned_copy_is_pinned_hash_mismatch",
    },
    Body {
        code: "squatted-name",
        detail: "null",
        exit: Some(1),
        file: "src/cmd/run.rs",
        needle: r#"return Ok(refused_with(launch.json, "squatted-name", || {"#,
        witness: "a_held_endpoint_is_squatted_name",
    },
    Body {
        code: "internal-error",
        detail: "null",
        exit: Some(1),
        file: "src/cmd/run.rs",
        needle: r#"Err(_) => return Ok(refused_with(launch.json, "internal-error", || {})),"#,
        witness: "a_revive_with_no_claude_on_its_path_is_internal_error",
    },
    Body {
        code: "internal-error",
        detail: "null",
        exit: Some(1),
        file: "src/cmd/revive.rs",
        needle: r#"client::refused_document("internal-error");"#,
        witness: "a_revive_with_no_claude_on_its_path_is_internal_error",
    },
    Body {
        code: "internal-error",
        detail: "null",
        exit: Some(1),
        file: "src/cmd/mod.rs",
        needle: r#"client::refused_document("internal-error");"#,
        witness: "a_frame_verb_that_cannot_open_its_log_is_internal_error",
    },
    Body {
        code: "no-session",
        detail: "null",
        exit: Some(1),
        file: "src/cmd/revive.rs",
        needle: REVIVE,
        witness: THE_WITNESS_CLI,
    },
    Body {
        code: "session-live",
        detail: "null",
        exit: Some(1),
        file: "src/cmd/revive.rs",
        needle: REVIVE,
        witness: "revive_json_of_a_session_open_in_another_claude_is_the_session_live_document",
    },
    Body {
        code: "cwd-missing",
        detail: "null",
        exit: Some(1),
        file: "src/cmd/revive.rs",
        needle: REVIVE,
        witness: "revive_json_of_a_snapshot_with_no_cwd_is_the_cwd_missing_document",
    },
    Body {
        code: "human-typing",
        detail: "null or manual-pause",
        exit: Some(10),
        file: "src/cmd/send.rs",
        needle: SEND_REFUSAL,
        witness: THE_WITNESS_CLI,
    },
    Body {
        code: "human-typing",
        detail: "null or manual-pause",
        exit: Some(10),
        file: "src/cmd/answer.rs",
        needle: SEND_REFUSAL,
        witness: "path4_a_dialog_pending_at_a_pause_is_handed_back_to_the_human",
    },
    Body {
        code: "budget-paused",
        detail: "five-hour or seven-day",
        exit: Some(11),
        file: "src/cmd/send.rs",
        needle: SEND_REFUSAL,
        witness: "path6_a_reading_over_the_threshold_refuses_the_send",
    },
    Body {
        code: "unverified-cli",
        detail: "null",
        exit: Some(12),
        file: "src/cmd/answer.rs",
        needle: SEND_REFUSAL,
        witness: "mcp_tools_path7_unverified_cli_answer",
    },
    Body {
        code: "not-delivered",
        detail: "control-character, empty-text, unknown-dialog",
        exit: Some(13),
        file: "src/cmd/send.rs",
        needle: SEND_REFUSAL,
        witness: THE_WITNESS_CLI,
    },
    Body {
        code: "not-delivered",
        detail: "turn-running, no-prompt-submitted, input-not-ready",
        exit: Some(13),
        file: "src/cmd/send.rs",
        needle: SEND_REFUSAL,
        witness: "send_window_no_prompt_submitted_refuses_and_logs",
    },
    Body {
        code: "unknown",
        detail: "a code word or null",
        exit: Some(14),
        file: "src/cmd/client.rs",
        needle: "RefusalReason::Unknown.as_str().to_owned()",
        witness: "reply_of_reads_the_three_shapes",
    },
];

const RESULT: &str = "crates/viola-mcp/src/result.rs";
const TOOLS: &str = "crates/viola-mcp/src/tools.rs";

const MCP_RESULTS: &[Body] = &[
    Body {
        code: "instance-unreachable",
        detail: "strict-modes-failed, instance-dead, unwrapped-name, endpoint-lost, server-verify-failed",
        exit: None,
        file: RESULT,
        needle: r#"error("instance-unreachable", Value::from(cause.as_str()))"#,
        witness: THE_WITNESS_MCP,
    },
    Body {
        code: "wrapper-fault",
        detail: "{code, message, data}, every field fixed or closed",
        exit: None,
        file: RESULT,
        needle: r#".map(|detail| error("wrapper-fault", detail))"#,
        witness: "of_reply_maps_each_reply_shape",
    },
    Body {
        code: "invalid-params",
        detail: "target, text, after, timeout_ms, dialog_id, response",
        exit: None,
        file: RESULT,
        needle: r#"error("invalid-params", Value::from(field))"#,
        witness: THE_WITNESS_MCP,
    },
    Body {
        code: "human-typing, budget-paused, unverified-cli, not-delivered",
        detail: "as the CLI's",
        exit: None,
        file: RESULT,
        needle: r#"failed(reason, json!({"refusal": reason, "detail": detail}))"#,
        witness: THE_WITNESS_MCP,
    },
    Body {
        code: "unknown",
        detail: "a code word or null",
        exit: None,
        file: RESULT,
        needle: "RefusalReason::Unknown.as_str()",
        witness: "handlers_a_refusal_of_free_text_reads_as_unknown_with_no_detail",
    },
    Body {
        code: "-32602 unknown tool",
        detail: "data null",
        exit: None,
        file: TOOLS,
        needle: r#"Cow::Borrowed("unknown tool"),"#,
        witness: THE_WITNESS_MCP,
    },
    Body {
        code: "-32603 internal error",
        detail: "data null",
        exit: None,
        file: TOOLS,
        needle: "fn internal_fault() -> ErrorData {",
        witness: "of_reply_reads_any_other_outcome_as_an_internal_fault",
    },
];

/// `invalid-params`' six field words, each where the handler names it.
const MCP_FIELD_WORDS: [&str; 6] = [
    "\"target\"",
    "\"text\"",
    "\"after\"",
    "\"timeout_ms\"",
    "\"dialog_id\"",
    "\"response\"",
];

/// The seven wire bodies, as literals.
const CHANNEL_FAULTS: &[(ProtocolError, &str, &str)] = &[
    (
        ProtocolError::Parse,
        r#"{"code":-32700,"message":"parse error","data":null}"#,
        THE_WITNESS_CHANNEL,
    ),
    (
        ProtocolError::InvalidRequest,
        r#"{"code":-32600,"message":"invalid request","data":null}"#,
        "refuse_oversize_logs_parse_rejected_and_closes",
    ),
    (
        ProtocolError::MethodNotFound,
        r#"{"code":-32601,"message":"method not found","data":null}"#,
        THE_WITNESS_CHANNEL,
    ),
    (
        ProtocolError::UnsupportedVersion,
        r#"{"code":-32602,"message":"unsupported protocol version","data":{"supported":1,"wrapper":"0.1.0"}}"#,
        THE_WITNESS_CHANNEL,
    ),
    (
        ProtocolError::InvalidParams,
        r#"{"code":-32602,"message":"invalid params","data":null}"#,
        THE_WITNESS_CHANNEL,
    ),
    (
        ProtocolError::ReleaseFromDriver,
        r#"{"code":-32602,"message":"invalid params","data":{"reason":"release-from-driver"}}"#,
        THE_WITNESS_CHANNEL,
    ),
    (
        ProtocolError::Internal,
        r#"{"code":-32603,"message":"internal error","data":null}"#,
        "answer_a_panicking_dispatch_is_internal_error_and_closes",
    ),
];

/// The nine served methods and the one notification these faults answer across.
const CHANNEL_METHODS: [&str; 10] = [
    "send",
    "wait",
    "last",
    "answer",
    "pause",
    "release",
    "link",
    "unlink",
    "hook.dialog",
    "hook.event",
];

/// What reads like an error body and is none: where it stands, and why it is out.
const NOT_ERROR_BODIES: &[(&str, &str, &str)] = &[
    (
        "src/main.rs",
        "Err(usage) => usage.exit(),",
        "exit 2's clap usage render echoes the caller's own argument on stderr and is no document",
    ),
    (
        "src/cmd/send.rs",
        "const UNUSABLE_TEXT: &str =",
        "a fixed stderr line for a text never read, with no document (the design table's row 2)",
    ),
    (
        "src/cmd/answer.rs",
        "const UNUSABLE_RESPONSE: &str =",
        "a fixed stderr line for a response never read, with no document (the design table's row 2)",
    ),
    (
        "src/human/mirror.rs",
        r#"format!("error: wrapper fault  {code}\n")"#,
        "human mode's line for exit 20: the integer code alone",
    ),
    (
        "src/human.rs",
        r#"b"error: internal error\n""#,
        "the catch site's fixed human line",
    ),
    (
        "src/cmd/verify.rs",
        "a recorded fixture is not clean: {named}",
        "verify's stderr names the fixture file and a closed code, the founder's ratified form",
    ),
    (
        "src/cmd/mcp.rs",
        "viola_mcp::serve(home.to_path_buf(), own_name, board)?;",
        "viola mcp's exit 1 prints nothing on either stream",
    ),
    (
        "src/run/send.rs",
        "return ctx.unconfirmable(cursor, slot.clock.now());",
        "unconfirmable is a word inside an ok payload, never a refusal",
    ),
    (
        RESULT,
        "CallToolResult::structured(payload)",
        "a result payload carries content by design (wait, last, list)",
    ),
];

/// Census lines that build or route a document a row above names.
const PLUMBING: &[(&str, &str, &str)] = &[
    (
        "src/cmd/client.rs",
        "pub(crate) fn refused_document(code: &str) {",
        "the writer's own head; its calls are the rows",
    ),
    (
        "src/cmd/client.rs",
        r#"document(&json!({"v": 1, "error": code, "detail": null}));"#,
        "the writer's body: each code a row's",
    ),
    (
        "src/cmd/list.rs",
        "fn refused_document() -> Value {",
        "the list row's own head",
    ),
    (
        "src/cmd/list.rs",
        "Err(_) => refused_document(),",
        "the list row's document, in the board's refusal",
    ),
    (
        "src/cmd/list.rs",
        "client::document(&refused_document());",
        "the list row's document, printed",
    ),
    (
        RESULT,
        r#"failed(code, json!({"error": code, "detail": detail}))"#,
        "the writer's body: each code a row's",
    ),
    (
        "crates/viola-channel/src/frame.rs",
        r#"json!({"jsonrpc": "2.0", "id": id, "error": fault.body()})"#,
        "the frame around CHANNEL_FAULTS' bodies",
    ),
];

const PRODUCT_ROOTS: [&str; 7] = [
    "src",
    "crates/viola-core/src",
    "crates/viola-pty/src",
    "crates/viola-channel/src",
    "crates/viola-state/src",
    "crates/viola-agent-claude/src",
    "crates/viola-mcp/src",
];

/// What makes a line a writer of an error body.
const WRITER_MARKS: [&str; 3] = ["\"error\":", "refused_document(", "result::error("];

fn workspace() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// The product files the census reads, repo-relative and `/`-separated: the fake agent and every
/// `test_`-stemmed module left out.
fn product_files(root: &Path) -> Vec<(String, String)> {
    let mut files = Vec::new();
    for dir in PRODUCT_ROOTS {
        rs_files(&root.join(dir), &mut files);
    }
    files
        .into_iter()
        .filter(|path| {
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
            !stem.starts_with("test_") && !path.ends_with("src/bin/viola-fake-agent.rs")
        })
        .map(|path| {
            let name = path
                .strip_prefix(root)
                .expect("under the workspace")
                .to_string_lossy()
                .replace('\\', "/");
            (name, read(&path))
        })
        .collect()
}

/// `text` with its comments blanked and, when `strings`, its string and char literals too (newlines
/// kept).
fn blanked(text: &str, strings: bool) -> Vec<u8> {
    let bytes = text.as_bytes();
    let mut out = bytes.to_vec();
    let blank = |out: &mut Vec<u8>, from: usize, to: usize| {
        for b in &mut out[from..to] {
            if *b != b'\n' {
                *b = b' ';
            }
        }
    };
    let ident = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
    let mut i = 0;
    while i < bytes.len() {
        let rest = &bytes[i..];
        if rest.starts_with(b"//") {
            let end = rest
                .iter()
                .position(|&b| b == b'\n')
                .map_or(bytes.len(), |n| i + n);
            blank(&mut out, i, end);
            i = end;
        } else if rest.starts_with(b"/*") {
            let mut depth = 0usize;
            let mut j = i;
            while j < bytes.len() {
                if bytes[j..].starts_with(b"/*") {
                    depth += 1;
                    j += 2;
                } else if bytes[j..].starts_with(b"*/") {
                    depth -= 1;
                    j += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    j += 1;
                }
            }
            blank(&mut out, i, j);
            i = j;
        } else if (rest[0] == b'r' || rest.starts_with(b"br"))
            && (i == 0 || !ident(bytes[i - 1]))
            && raw_string_opening(rest).is_some()
        {
            let (prefix, hashes) = raw_string_opening(rest).expect("checked");
            let close: Vec<u8> = std::iter::once(b'"')
                .chain(std::iter::repeat_n(b'#', hashes))
                .collect();
            let body = i + prefix;
            let end = bytes[body..]
                .windows(close.len())
                .position(|w| w == close.as_slice())
                .map_or(bytes.len(), |n| body + n + close.len());
            if strings {
                blank(&mut out, i, end);
            }
            i = end;
        } else if rest[0] == b'"' {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j] != b'"' {
                j += if bytes[j] == b'\\' { 2 } else { 1 };
            }
            let end = (j + 1).min(bytes.len());
            if strings {
                blank(&mut out, i, end);
            }
            i = end;
        } else if rest[0] == b'\'' {
            match char_literal_len(rest) {
                Some(len) => {
                    if strings {
                        blank(&mut out, i, i + len);
                    }
                    i += len;
                }
                None => i += 1,
            }
        } else {
            i += 1;
        }
    }
    out
}

/// A raw string's opening (`r"`, `r#"`, `br##"` …): its length and its count of `#`.
fn raw_string_opening(rest: &[u8]) -> Option<(usize, usize)> {
    let after_r = if rest.starts_with(b"br") { 2 } else { 1 };
    let hashes = rest[after_r..].iter().take_while(|&&b| b == b'#').count();
    (rest.get(after_r + hashes) == Some(&b'"')).then_some((after_r + hashes + 1, hashes))
}

/// The length of a char literal starting at `rest`, or `None` for a lifetime.
fn char_literal_len(rest: &[u8]) -> Option<usize> {
    if rest.get(1) == Some(&b'\\') {
        let close = rest[2..].iter().position(|&b| b == b'\'')?;
        return Some(close + 3);
    }
    let text = std::str::from_utf8(&rest[1..rest.len().min(6)])
        .ok()
        .or_else(|| {
            (1..6)
                .rev()
                .find_map(|n| std::str::from_utf8(rest.get(1..1 + n)?).ok())
        })?;
    let c = text.chars().next()?;
    let len = c.len_utf8();
    (rest.get(1 + len) == Some(&b'\'')).then_some(len + 2)
}

/// `text`'s product lines: comments and each `#[cfg(test)]` item blanked, string literals kept. The
/// item's end is found over the text with its literals blanked too, so only code braces count.
fn product_view(text: &str) -> String {
    let code = blanked(text, true);
    let mut view = blanked(text, false);
    let marker = b"#[cfg(test)]";
    let mut at = 0;
    while let Some(found) = code[at..].windows(marker.len()).position(|w| w == marker) {
        let start = at + found;
        let mut j = start + marker.len();
        let mut depth = 0usize;
        let end = loop {
            match code.get(j) {
                None => break code.len(),
                Some(b';') if depth == 0 => break j + 1,
                Some(b'{') => depth += 1,
                Some(b'}') => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        break j + 1;
                    }
                }
                _ => {}
            }
            j += 1;
        };
        for b in &mut view[start..end] {
            if *b != b'\n' {
                *b = b' ';
            }
        }
        at = end;
    }
    String::from_utf8_lossy(&view).into_owned()
}

/// The writer lines of `text` that no `(file, needle)` of `named` covers: `<file>:<line>`.
fn unnamed_writers(file: &str, text: &str, named: &[(&str, &str)]) -> Vec<String> {
    product_view(text)
        .lines()
        .enumerate()
        .filter(|(_, line)| WRITER_MARKS.iter().any(|mark| line.contains(mark)))
        .filter(|(_, line)| {
            !named
                .iter()
                .any(|(f, needle)| *f == file && line.contains(needle))
        })
        .map(|(at, _)| format!("{file}:{}", at + 1))
        .collect()
}

/// The `#[error(…)]` attributes of `text`'s product lines, and how many of them interpolate or hold
/// no plain literal.
fn error_attributes(text: &str) -> (usize, usize) {
    let view = product_view(text);
    let mut seen = 0;
    let mut interpolated = 0;
    for (at, _) in view.match_indices("#[error(") {
        seen += 1;
        let rest = &view[at + "#[error(".len()..];
        let literal = rest
            .strip_prefix('"')
            .and_then(|body| body.split_once("\")]"));
        if literal.is_none_or(|(message, _)| message.contains('{')) {
            interpolated += 1;
        }
    }
    (seen, interpolated)
}

/// Every `fn <name>` the workspace's sources define, tests included.
fn defined_functions(root: &Path) -> String {
    let mut files = Vec::new();
    for dir in ["src", "crates", "tests"] {
        rs_files(&root.join(dir), &mut files);
    }
    files.iter().map(|path| read(path)).collect()
}

fn every_body() -> impl Iterator<Item = &'static Body> {
    CLI_DOCUMENTS.iter().chain(MCP_RESULTS)
}

#[test]
fn error_bodies_each_name_a_writer_and_a_witness_that_stand() {
    let root = workspace();
    let functions = defined_functions(&root);
    let defined = |witness: &str| {
        [format!("fn {witness}("), format!("fn {witness}<")]
            .iter()
            .any(|def| functions.contains(def.as_str()))
    };
    let mut faults = Vec::new();
    for body in every_body() {
        assert!(
            !body.code.is_empty() && !body.detail.is_empty(),
            "{}",
            body.code
        );
        if !read(&root.join(body.file)).contains(body.needle) {
            faults.push(format!("{}: its writer is not in {}", body.code, body.file));
        }
        if !defined(body.witness) {
            faults.push(format!("{}: no test {}", body.code, body.witness));
        }
    }
    for body in CLI_DOCUMENTS {
        assert!(body.exit.is_some(), "{}: its exit", body.code);
    }
    for body in MCP_RESULTS {
        assert!(
            body.exit.is_none(),
            "{}: no exit on this surface",
            body.code
        );
    }
    for (_, _, witness) in CHANNEL_FAULTS {
        if !defined(witness) {
            faults.push(format!("a channel fault: no test {witness}"));
        }
    }
    let tools = read(&root.join(TOOLS));
    for word in MCP_FIELD_WORDS {
        if !tools.contains(word) {
            faults.push(format!("invalid-params: {word} is not in {TOOLS}"));
        }
    }
    let served = read(&root.join("crates/viola-channel/src/server.rs"));
    for method in CHANNEL_METHODS {
        if !served.contains(&format!("\"{method}\",")) {
            faults.push(format!("the channel no longer serves {method}"));
        }
    }
    for (file, needle, why) in NOT_ERROR_BODIES.iter().chain(PLUMBING) {
        assert!(!why.is_empty());
        if !read(&root.join(file)).contains(needle) {
            faults.push(format!("{file}: `{needle}` is gone"));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

/// The literal table against what a fault builds: a new or changed body reads red here.
#[test]
fn channel_faults_are_the_seven_literal_bodies() {
    for (fault, wire, _) in CHANNEL_FAULTS {
        assert_eq!(fault.body().to_string(), *wire);
    }
    assert_eq!(CHANNEL_FAULTS.len(), 7);
}

#[test]
fn product_error_writers_are_each_named() {
    let root = workspace();
    let named: Vec<(&str, &str)> = every_body()
        .map(|body| (body.file, body.needle))
        .chain(PLUMBING.iter().map(|(file, needle, _)| (*file, *needle)))
        .collect();
    let mut unnamed = Vec::new();
    let mut read_lines = 0;
    for (file, text) in product_files(&root) {
        read_lines += product_view(&text)
            .lines()
            .filter(|line| WRITER_MARKS.iter().any(|mark| line.contains(mark)))
            .count();
        unnamed.extend(unnamed_writers(&file, &text, &named));
    }
    assert!(read_lines > 0, "the census read no writer");
    assert!(
        unnamed.is_empty(),
        "error writers no row names: {unnamed:#?}"
    );
}

#[test]
fn thiserror_messages_interpolate_nothing() {
    let root = workspace();
    let mut seen = 0;
    let mut faults = Vec::new();
    for (file, text) in product_files(&root) {
        let (attributes, interpolated) = error_attributes(&text);
        seen += attributes;
        if interpolated > 0 {
            faults.push(format!("{file}: {interpolated}"));
        }
    }
    assert!(seen > 0, "the census read no message");
    assert!(faults.is_empty(), "messages that interpolate: {faults:#?}");
}

/// The self-check over planted text: each writer no row names and each message that interpolates
/// reads as one, and a comment, a test item, a reader of the member or a fixed message as none. Each
/// case gives the unnamed lines, then the attributes read and how many of them interpolate.
#[rstest]
#[case::an_unnamed_key(
    "fn f() -> Value {\n    json!({\"error\": text})\n}\n",
    &["a.rs:2"],
    (0, 0)
)]
#[case::an_unnamed_call("fn f() {\n    refused_document(\"free text\");\n}\n", &["a.rs:2"], (0, 0))]
#[case::an_unnamed_mcp_call("fn f() {\n    result::error(\"x\", detail);\n}\n", &["a.rs:2"], (0, 0))]
#[case::a_named_writer("fn f() -> Value {\n    json!({\"error\": \"named\"})\n}\n", &[], (0, 0))]
#[case::a_writer_in_a_comment("// json!({\"error\": text})\nfn f() {}\n", &[], (0, 0))]
#[case::a_writer_in_a_test_module(
    "fn f() {}\n#[cfg(test)]\nmod tests {\n    fn g() { json!({\"error\": text}); }\n}\n",
    &[],
    (0, 0)
)]
#[case::a_reader_of_the_member("fn f(v: &Value) { v.get(\"error\"); }\n", &[], (0, 0))]
#[case::a_fixed_message("#[error(\"fixed\")]\nA,\n", &[], (1, 0))]
#[case::a_named_field("#[error(\"bad {path}\")]\nA { path: String },\n", &[], (1, 1))]
#[case::a_positional_field("#[error(\"bad {0}\")]\nA(String),\n", &[], (1, 1))]
#[case::transparent("#[error(transparent)]\nA(E),\n", &[], (1, 1))]
#[case::a_message_in_a_comment("// #[error(\"bad {path}\")]\nA,\n", &[], (0, 0))]
#[case::a_message_in_a_test_item("#[cfg(test)]\nmod t {\n#[error(\"bad {0}\")]\nA(u8),\n}\n", &[], (0, 0))]
fn census_finds_an_unnamed_writer_and_an_interpolated_message(
    #[case] text: &str,
    #[case] unnamed: &[&str],
    #[case] attributes: (usize, usize),
) {
    let named = [("a.rs", "\"error\": \"named\"")];
    assert_eq!(unnamed_writers("a.rs", text, &named), unnamed);
    assert_eq!(error_attributes(text), attributes);
}
