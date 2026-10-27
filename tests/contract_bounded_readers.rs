//! Every external reader bounded where it enters (security-plan §Input Validation; test-plan §11
//! Unit): `BOUNDARIES` names each boundary with the text of its bounded read, its cap, depth and
//! type rule and the test that witnesses it, and a census over the product sources reads red on a
//! reader call that is neither bounded in its own statement nor a named exception. The tables are
//! literals, never the product's own list.
//!
//! andromeda:walks-tree — it reads every `.rs` file under the product source roots, named or not.

use std::fs;
use std::path::{Path, PathBuf};

use rstest::rstest;

/// One external reader: where its bounded read stands and what holds it.
struct Boundary {
    boundary: &'static str,
    file: &'static str,
    /// The text of its bounded read (or its validator) in `file`.
    needle: &'static str,
    cap: &'static str,
    depth: &'static str,
    types: &'static str,
    witness: &'static str,
}

const DEFAULT_DEPTH: &str = "serde_json default";
const NO_PARSE: &str = "no JSON parse";

const BOUNDARIES: &[Boundary] = &[
    Boundary {
        boundary: "hook event stdin",
        file: "src/cmd/hook.rs",
        needle: "stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes)?;\n    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME {\n        reject_stdin(\"oversize\");",
        cap: "MAX_FRAME + 1",
        depth: DEFAULT_DEPTH,
        types: "closed HookEvent; an oversize or malformed payload fails open",
        witness: "handle_refuses_oversize_stdin_and_reads_a_frame_at_the_cap",
    },
    Boundary {
        boundary: "hook dialog stdin",
        file: "src/cmd/hook.rs",
        needle: "stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes)?;\n    let classified = if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME {",
        cap: "MAX_FRAME + 1",
        depth: DEFAULT_DEPTH,
        types: "closed DialogKind and per-kind response enums",
        witness: "handle_dialog_refuses_oversize_stdin_and_prints_nothing",
    },
    Boundary {
        boundary: "hook --capture stdin",
        file: "src/cmd/hook.rs",
        needle: "stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes).ok()?;\n    file_from(event, dir, bytes, free_k(dir)?)",
        cap: "MAX_FRAME + 1",
        depth: NO_PARSE,
        types: "raw bytes filed 0600, never parsed",
        witness: "capture_keeps_one_byte_past_the_cap",
    },
    Boundary {
        boundary: "hook statusline --capture stdin",
        file: "src/cmd/hook.rs",
        needle: "stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes).ok()?;\n    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FRAME {\n        return None;",
        cap: "MAX_FRAME + 1",
        depth: NO_PARSE,
        types: "raw bytes; the shell word into the closed ShellVerdict",
        witness: "capture_statusline_files_a_payload_at_the_cap_and_nothing_past_it",
    },
    Boundary {
        boundary: "hook --capture --answers file",
        file: "src/cmd/hook.rs",
        needle: "file.take(ANSWER_CAP).read_to_string(&mut id)",
        cap: "ANSWER_CAP",
        depth: NO_PARSE,
        types: "closed ProbeAnswer id",
        witness: "answer_reads_no_relative_answers_dir_and_no_oversize_file",
    },
    Boundary {
        boundary: "hook statusline stdin",
        file: "src/cmd/hook/statusline.rs",
        needle: "io.stdin.take(MAX_FRAME + 1).read_to_end(&mut bytes)?;",
        cap: "MAX_FRAME + 1",
        depth: DEFAULT_DEPTH,
        types: "rate_limits read tolerantly into Percent",
        witness: "statusline_stdin_of_exactly_the_frame_cap_is_taken",
    },
    Boundary {
        boundary: "status line command stdout",
        file: "src/cmd/hook/statusline.rs",
        needle: "pipe.take(MAX_FRAME).read_to_end(&mut bytes);",
        cap: "MAX_FRAME",
        depth: NO_PARSE,
        types: "bytes passed through unparsed",
        witness: "statusline_shell_out_passes_the_output_through_byte_for_byte",
    },
    Boundary {
        boundary: "statusline-source.json and ~/.claude/settings.json",
        file: "src/cmd/run.rs",
        needle: "File::open(source)\n        .ok()?\n        .take(MAX_FRAME)\n        .read_to_end(&mut bytes)",
        cap: "MAX_FRAME",
        depth: DEFAULT_DEPTH,
        types: "a statusLine of type command, non-empty, no NUL",
        witness: "statusline_source_over_the_frame_cap_is_no_command",
    },
    Boundary {
        boundary: "config.json thresholds",
        file: "src/run/budget.rs",
        needle: "Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {\n            Ok(_) => parse_thresholds(&bytes),",
        cap: "MAX_FRAME",
        depth: DEFAULT_DEPTH,
        types: "whole numbers into Percent",
        witness: "thresholds_read_each_key_as_a_whole_percentage",
    },
    Boundary {
        boundary: "config.json dialog_hold_s",
        file: "src/run/dialog.rs",
        needle: "Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {\n            Ok(_) => parse_hold(&bytes),",
        cap: "MAX_FRAME",
        depth: DEFAULT_DEPTH,
        types: "a whole number of seconds into DialogHold",
        witness: "read_hold_takes_a_whole_number_of_seconds_in_range",
    },
    Boundary {
        boundary: "config.json claude_env_keep",
        file: "src/run/env.rs",
        needle: "Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {\n            Ok(_) => parse_env_keep(&bytes),",
        cap: "MAX_FRAME",
        depth: DEFAULT_DEPTH,
        types: "listed CLAUDE names, the whole list refused on a bad entry",
        witness: "env_keep_rejects_the_whole_list_on_any_bad_entry",
    },
    Boundary {
        boundary: "config.json diagnostics_level",
        file: "src/obs.rs",
        needle: "Ok(file) => match file.take(MAX_FRAME).read_to_end(&mut bytes) {\n            Ok(_) => parse_diagnostics_level(&bytes),",
        cap: "MAX_FRAME",
        depth: DEFAULT_DEPTH,
        types: "closed info | debug",
        witness: "read_diagnostics_level_reads_at_most_sixteen_mib",
    },
    Boundary {
        boundary: "send text (stdin or --file)",
        file: "src/cmd/send.rs",
        needle: "reader.take(MAX_FRAME + 1).read_to_end(&mut bytes).ok()?;",
        cap: "MAX_FRAME + 1",
        depth: NO_PARSE,
        types: "UTF-8, then validate_paste_text",
        witness: "read_text_takes_utf8_up_to_max_frame",
    },
    Boundary {
        boundary: "answer response",
        file: "src/cmd/answer.rs",
        needle: "reader.take(MAX_FRAME + 1).read_to_end(&mut bytes).ok()?;",
        cap: "MAX_FRAME + 1",
        depth: DEFAULT_DEPTH,
        types: "one of the three closed answer shapes",
        witness: "read_response_stops_at_max_frame",
    },
    Boundary {
        boundary: "channel frame (sync server and client)",
        file: "crates/viola-channel/src/frame.rs",
        needle: "io::Read::take(&mut *reader, cap)\n        .read_until(b'\\n', &mut line)",
        cap: "MAX_FRAME",
        depth: DEFAULT_DEPTH,
        types: "closed methods and params, -32600 and close past the cap",
        witness: "read_frame_refuses_one_byte_over_the_bound",
    },
    Boundary {
        boundary: "channel frame (server connection)",
        file: "crates/viola-channel/src/server.rs",
        needle: "let reply = match read_frame(&mut reader) {",
        cap: "MAX_FRAME",
        depth: DEFAULT_DEPTH,
        types: "an oversize frame is one parse-rejected line and a close",
        witness: "refuse_oversize_logs_parse_rejected_and_closes",
    },
    Boundary {
        boundary: "channel frame (Tokio client)",
        file: "crates/viola-channel/src/tokio_client.rs",
        needle: ".take(MAX_FRAME)\n        .read_until(b'\\n', &mut line)",
        cap: "MAX_FRAME",
        depth: DEFAULT_DEPTH,
        types: "judged by the sync reader's framed",
        witness: "tokio_client_refuses_a_reply_over_the_frame_bound",
    },
    Boundary {
        boundary: "viola mcp stdin",
        file: "crates/viola-mcp/src/lib.rs",
        needle: "let stdin = stdin::CappedLines::new(stdin, MAX_FRAME);",
        cap: "the per-line cap MAX_FRAME, its newline counted",
        depth: DEFAULT_DEPTH,
        types: "rmcp's JSON-RPC messages; a longer line ends the session",
        witness: "mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line",
    },
    Boundary {
        boundary: "MCP tool arguments",
        file: "crates/viola-mcp/src/tools.rs",
        needle: ".and_then(|name| ViolaName::try_new(name.to_owned()).ok())",
        cap: "the per-line cap MAX_FRAME",
        depth: DEFAULT_DEPTH,
        types: "raw arguments re-validated: ViolaName, u64, Response::parse",
        witness: "handlers_schema_violations_write_no_frame",
    },
    Boundary {
        boundary: "claude --version output",
        file: "src/run/version_gate.rs",
        needle: "run_capped(program, args, cwd, strip, deadline, stopped, MAX_FRAME)",
        cap: "MAX_FRAME",
        depth: NO_PARSE,
        types: "the documented first line only",
        witness: "version_gate_degrades_for_a_child_that_is_not_the_cli",
    },
    Boundary {
        boundary: "child pipe drain (claude --version, claude agents --json)",
        file: "src/run/version_gate.rs",
        needle: "let _ = pipe.take(cap).read_to_end(&mut bytes);",
        cap: "the caller's cap",
        depth: NO_PARSE,
        types: "bytes handed to the caller's parse",
        witness: "run_bounded_returns_the_exit_and_stdout",
    },
    Boundary {
        boundary: "claude agents --json output",
        file: "src/run/agents.rs",
        needle: "VERSION_DEADLINE,\n        stopped,\n        MAX_FRAME + 1,",
        cap: "MAX_FRAME + 1",
        depth: DEFAULT_DEPTH,
        types: "rows or the closed AgentsFailure",
        witness: "agents_parse_takes_max_frame_bytes_and_refuses_one_more_as_oversize",
    },
    Boundary {
        boundary: "verify hook captures",
        file: "src/cmd/verify.rs",
        needle: ".take(MAX_FRAME + 1)\n                .read_to_end(&mut bytes)\n                .ok()?;\n            Some(Capture::read(event, &bytes))",
        cap: "MAX_FRAME + 1",
        depth: DEFAULT_DEPTH,
        types: "ledger post-conditions over a tolerant parse",
        witness: "read_captures_reads_one_byte_past_the_cap",
    },
    Boundary {
        boundary: "verify status line captures",
        file: "src/cmd/verify.rs",
        needle: ".take(MAX_FRAME + 1)\n                .read_to_end(&mut bytes)\n                .ok()?;\n            let mut word = Vec::new();",
        cap: "MAX_FRAME + 1",
        depth: DEFAULT_DEPTH,
        types: "the payload read tolerantly",
        witness: "read_statusline_reads_one_byte_past_the_cap",
    },
    Boundary {
        boundary: "verify shell verdict file",
        file: "src/cmd/verify.rs",
        needle: ".and_then(|f| f.take(SHELL_CAP).read_to_end(&mut word))",
        cap: "SHELL_CAP",
        depth: NO_PARSE,
        types: "closed ShellVerdict",
        witness: "read_statusline_orders_by_k_and_reads_each_shell_verdict",
    },
    Boundary {
        boundary: "verify PTY output",
        file: "src/cmd/verify/typed/pty.rs",
        needle: "if self.total > MAX_FRAME {\n            self.screen.poison();",
        cap: "MAX_FRAME in all",
        depth: NO_PARSE,
        types: "vt100 screen, poisoned past the cap",
        witness: "feed_reaching_exactly_the_frame_cap_does_not_poison_and_one_byte_past_does",
    },
    Boundary {
        boundary: "the wrapper's vt100 feed",
        file: "src/run/gate.rs",
        needle: "let (tx, rx) = mpsc::sync_channel(FEED_CAPACITY);",
        cap: "FEED_CAPACITY",
        depth: NO_PARSE,
        types: "a dropped copy or a caught panic poisons to input-not-ready",
        witness: "overflow_poisons_and_writes_one_oversize_line_until_a_resize",
    },
    Boundary {
        boundary: "the vt100 feed panic",
        file: "src/run/gate.rs",
        needle: "catch_unwind",
        cap: "FEED_CAPACITY",
        depth: NO_PARSE,
        types: "a caught panic poisons; the human's bytes go first",
        witness: "chaos_feed_panic_refuses_send_and_keeps_passthrough",
    },
    Boundary {
        boundary: "snapshot.json",
        file: "crates/viola-state/src/snapshot.rs",
        needle: "if file.take(MAX_FRAME).read_to_end(&mut bytes).is_err() {",
        cap: "MAX_FRAME",
        depth: DEFAULT_DEPTH,
        types: "the envelope's v; an unsupported one replays the log",
        witness: "read_snapshot_reads_at_most_sixteen_mib",
    },
    Boundary {
        boundary: "events.ndjson",
        file: "crates/viola-state/src/events.rs",
        needle: "std::io::Read::take(&mut *reader, self.cap).read_until(b'\\n', &mut line)?;",
        cap: "the per-line cap MAX_FRAME, its newline counted",
        depth: DEFAULT_DEPTH,
        types: "closed kinds; unknown kinds, fields and torn lines counted",
        witness: "events_three_counts_a_line_past_the_cap_is_torn",
    },
    Boundary {
        boundary: "budget.json",
        file: "crates/viola-state/src/budget.rs",
        needle: "File::open(&path)\n        .ok()?\n        .take(MAX_FRAME + 1)\n        .read_to_end(&mut bytes)",
        cap: "MAX_FRAME + 1",
        depth: DEFAULT_DEPTH,
        types: "tolerant parse at v 1, every failure no reading",
        witness: "budget_read_over_the_frame_cap_is_nothing",
    },
    Boundary {
        boundary: "ledger/stamps.json",
        file: "crates/viola-state/src/stamps.rs",
        needle: "file.take(MAX_FRAME + 1).read_to_end(&mut bytes)?;",
        cap: "MAX_FRAME + 1",
        depth: DEFAULT_DEPTH,
        types: "per-version rows, unknown versions kept",
        witness: "read_stamps_is_none_when_absent_and_refuses_over_the_cap",
    },
    Boundary {
        boundary: "replace_private_shared's compare read",
        file: "crates/viola-state/src/fs.rs",
        needle: "file.take(cap).read_to_end(&mut read).is_ok() && read == bytes",
        cap: "the bytes' length + 1",
        depth: NO_PARSE,
        types: "an exact byte compare",
        witness: "replace_private_shared_reads_no_more_than_one_byte_past_the_bytes",
    },
    Boundary {
        boundary: "instance names (ViolaName)",
        file: "crates/viola-core/src/lib.rs",
        needle: "validate(predicate = is_valid_name),",
        cap: "1 to 32 characters",
        depth: NO_PARSE,
        types: "[a-z0-9-], a letter first, before any path join",
        witness: "viola_name_prop_agrees_with_the_oracle",
    },
    Boundary {
        boundary: "CLI instance positional",
        file: "src/cmd/send.rs",
        needle: "ViolaName::try_new(raw.to_owned()).map_err(|_| \"invalid instance name\".to_owned())",
        cap: "1 to 32 characters",
        depth: NO_PARSE,
        types: "ViolaName through the clap value parser",
        witness: "viola_name_prop_rejects_hostile_neighbours",
    },
    Boundary {
        boundary: "VIOLA_NAME and VIOLA_DIR (CLI home)",
        file: "src/cmd/mod.rs",
        needle: "let name = ViolaName::try_new(name?.into_string().ok()?).ok()?;",
        cap: "1 to 32 characters",
        depth: NO_PARSE,
        types: "a ViolaName and an absolute <home>/instances/<name>, else fall through",
        witness: "resolve_home_takes_the_flag_then_the_session_then_the_default",
    },
    Boundary {
        boundary: "VIOLA_NAME and VIOLA_DIR (hook)",
        file: "src/cmd/hook.rs",
        needle: "instance_of(Some(name.into()), Some(dir.into_os_string()))",
        cap: "1 to 32 characters",
        depth: NO_PARSE,
        types: "a wrapped instance directory only",
        witness: "instance_of_refuses_anything_but_a_wrapped_dir",
    },
    Boundary {
        boundary: "session ids (revive --id, --resume)",
        file: "crates/viola-agent-claude/src/lib.rs",
        needle: "pub fn is_session_id(id: &str) -> bool {",
        cap: "36 characters",
        depth: NO_PARSE,
        types: "the closed session-id shape",
        witness: "session_id_shape_is_thirty_six_hex_characters_with_four_hyphens",
    },
    Boundary {
        boundary: "the pinned key",
        file: "crates/viola-state/src/pin.rs",
        needle: "pub fn pin_key(bytes: &[u8]) -> String {",
        cap: "the version and a truncated SHA-256",
        depth: NO_PARSE,
        types: "computed from the bytes, never read from outside",
        witness: "pin_key_is_the_version_and_the_content_key",
    },
    Boundary {
        boundary: "the colour facts NO_COLOR, TERM, COLORTERM",
        file: "src/human/style.rs",
        needle: "no_color: std::env::var_os(\"NO_COLOR\").is_some(),",
        cap: "compared only",
        depth: NO_PARSE,
        types: "decide only whether and how deep SGR is written",
        witness: "colour_decision_each_rung_switches_colour_off",
    },
];

/// Reads of this process's own executable: no external input, and larger than `MAX_FRAME` in a
/// debug build.
const EXCEPTIONS: &[(&str, &str, &str)] = &[
    (
        "src/cmd/run.rs",
        "std::fs::read(std::env::current_exe()?)",
        "checked_exe: the running binary's own bytes, hashed into the pin key",
    ),
    (
        "crates/viola-state/src/pin.rs",
        "std::fs::read(exe)",
        "pin_exe: verify's own executable, copied into the home",
    ),
];

const READ_CALLS: [&str; 7] = [
    "read_to_end(",
    "read_to_string(",
    "read_until(",
    "read_line(",
    "from_reader(",
    "fs::read(",
    "fs::read_to_string(",
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

/// Repo-relative, `/`-separated.
fn relative(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .expect("under the workspace")
        .to_string_lossy()
        .replace('\\', "/")
}

/// The product files the census reads: the fake agent and every `test_`-stemmed module (a
/// `#[cfg(test)]` file, `test_support` among them) left out.
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
        .map(|path| (relative(&path, root), read(&path)))
        .collect()
}

/// `text` with every byte of a comment, a string or a char literal turned into a space (newlines
/// kept), so only code is searched and only code braces are counted.
fn code_only(text: &str) -> Vec<u8> {
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
            blank(&mut out, i, end);
            i = end;
        } else if rest[0] == b'"' {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j] != b'"' {
                j += if bytes[j] == b'\\' { 2 } else { 1 };
            }
            let end = (j + 1).min(bytes.len());
            blank(&mut out, i, end);
            i = end;
        } else if rest[0] == b'\'' {
            match char_literal_len(rest) {
                Some(len) => {
                    blank(&mut out, i, i + len);
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

/// `code` with each `#[cfg(test)]` item (a module, a function, a `mod x;`) blanked.
fn without_test_items(code: &mut [u8]) {
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
        for b in &mut code[start..end] {
            if *b != b'\n' {
                *b = b' ';
            }
        }
        at = end;
    }
}

/// One reader call: its line, and its statement up to the call (from the last `;`, `{` or `}`).
struct Hit {
    line: usize,
    statement: String,
}

impl Hit {
    fn bounded(&self) -> bool {
        self.statement.contains("take(")
    }
}

/// Every reader call in `text`'s product code, a call matched by two patterns counted once.
fn reader_calls(text: &str) -> Vec<Hit> {
    let mut code = code_only(text);
    without_test_items(&mut code);
    let mut opens = Vec::new();
    for call in READ_CALLS.map(str::as_bytes) {
        for (at, window) in code.windows(call.len()).enumerate() {
            let named_alone = at == 0 || !matches!(code[at - 1], b'_' | b'a'..=b'z');
            if window == call && named_alone {
                opens.push(at + call.len() - 1);
            }
        }
    }
    opens.sort_unstable();
    opens.dedup();
    opens
        .into_iter()
        .map(|open| {
            let start = code[..open]
                .iter()
                .rposition(|&b| matches!(b, b';' | b'{' | b'}'))
                .map_or(0, |n| n + 1);
            Hit {
                line: code[..open].iter().filter(|&&b| b == b'\n').count() + 1,
                statement: String::from_utf8_lossy(&code[start..open]).into_owned(),
            }
        })
        .collect()
}

/// Every `fn <name>` the workspace's sources define, tests included.
fn defined_functions(root: &Path) -> String {
    let mut files = Vec::new();
    for dir in ["src", "crates", "tests"] {
        rs_files(&root.join(dir), &mut files);
    }
    files.iter().map(|path| read(path)).collect()
}

#[test]
fn boundaries_each_name_a_bounded_read_that_stands() {
    let root = workspace();
    let functions = defined_functions(&root);
    let mut faults = Vec::new();
    for b in BOUNDARIES {
        assert!(
            !b.cap.is_empty() && !b.depth.is_empty() && !b.types.is_empty(),
            "{}: every rule named",
            b.boundary
        );
        if !read(&root.join(b.file)).contains(b.needle) {
            faults.push(format!("{}: its read is not in {}", b.boundary, b.file));
        }
        let defined = [format!("fn {}(", b.witness), format!("fn {}<", b.witness)]
            .iter()
            .any(|def| functions.contains(def.as_str()));
        if !defined {
            faults.push(format!("{}: no test {}", b.boundary, b.witness));
        }
    }
    for (file, needle, why) in EXCEPTIONS {
        assert!(!why.is_empty());
        if !read(&root.join(file)).contains(needle) {
            faults.push(format!("exception {file}: `{needle}` is gone"));
        }
    }
    assert!(faults.is_empty(), "{faults:#?}");
}

#[test]
fn product_readers_are_each_bounded_or_named() {
    let root = workspace();
    let mut unbounded = Vec::new();
    let mut excepted = Vec::new();
    for (file, text) in product_files(&root) {
        for hit in reader_calls(&text) {
            if hit.bounded() {
                continue;
            }
            let line = text.lines().nth(hit.line - 1).unwrap_or("");
            match EXCEPTIONS
                .iter()
                .find(|(f, needle, _)| *f == file && line.contains(needle))
            {
                Some(exception) => excepted.push(exception.1),
                None => unbounded.push(format!("{file}:{}", hit.line)),
            }
        }
    }
    assert!(
        unbounded.is_empty(),
        "reader calls with no take( in their statement and no exception: {unbounded:#?}"
    );
    excepted.sort_unstable();
    let mut named: Vec<&str> = EXCEPTIONS.iter().map(|e| e.1).collect();
    named.sort_unstable();
    assert_eq!(excepted, named, "every exception names a read that stands");
}

#[rstest]
#[case::unbounded(
    "fn f(r: impl Read) {\n    let mut b = Vec::new();\n    r.read_to_end(&mut b);\n}\n",
    vec![(3, false)]
)]
#[case::bounded_in_its_statement(
    "fn f(r: impl Read) {\n    let mut b = Vec::new();\n    r.take(MAX_FRAME).read_to_end(&mut b);\n}\n",
    vec![(3, true)]
)]
#[case::bounded_across_lines(
    "fn f(p: &Path) {\n    File::open(p)\n        .ok()?\n        .take(MAX_FRAME + 1)\n        .read_to_end(&mut b)\n        .ok()?;\n}\n",
    vec![(5, true)]
)]
#[case::a_take_in_an_earlier_statement_bounds_nothing(
    "fn f(r: impl Read) {\n    let c = r.take(1);\n    let s = fs::read(p);\n}\n",
    vec![(3, false)]
)]
#[case::whole_file_read(
    "fn f(p: &Path) -> Vec<u8> {\n    std::fs::read_to_string(p).unwrap()\n}\n",
    vec![(2, false)]
)]
#[case::inside_a_test_module(
    "fn f() {}\n#[cfg(test)]\nmod tests {\n    fn g() { let s = fs::read(p); let c = '}'; }\n}\nfn h(r: impl Read) { r.read_line(&mut s); }\n",
    vec![(6, false)]
)]
#[case::in_a_comment_or_a_string(
    "// r.read_to_end(&mut b)\nfn f() { let s = \"fs::read(p)\"; let t = r#\"read_line(\"#; }\n",
    vec![]
)]
#[case::a_lookalike_name(
    "fn f() { s.my_read_line(x); s.buf_read_to_end(x); }\n",
    vec![]
)]
fn census_finds_an_unbounded_read_and_passes_a_bounded_one(
    #[case] text: &str,
    #[case] expected: Vec<(usize, bool)>,
) {
    let got: Vec<(usize, bool)> = reader_calls(text)
        .iter()
        .map(|h| (h.line, h.bounded()))
        .collect();
    assert_eq!(got, expected);
}

#[test]
fn serde_json_unbounded_depth_is_banned_in_deny_toml() {
    let deny = read(&workspace().join("deny.toml"));
    let banned = deny.split("[[bans.features]]").skip(1).any(|table| {
        let table = table.split("\n[").next().unwrap_or("");
        table.lines().any(|l| l.trim() == r#"crate = "serde_json""#)
            && table
                .lines()
                .any(|l| l.trim() == r#"deny = ["unbounded_depth"]"#)
    });
    assert!(banned, "deny.toml bans serde_json's unbounded_depth");
}
