# Codebase Research — 2026-10-27-bounded-inputs-at-every-boundary

## Scope
- **Depth:** deep · **Reads:** 24 · **Globs/Greps:** 19
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — read in full, its 18 Session Additions applied (the `test(/(^|::)tests::name(::|$)/)` and `binary(<stem>)` filter forms, the `run --e2e` layer for `contract_` / `chaos_` stems, `schema-check` reading kept homes only); `.claude/rules/testing.md` (auto-loaded on `tests/` reads) — its 56 additions read, the remove-the-guard rule, the kill-line rule and the "a case that asserts an absence forces the behaviour on" control applied
- **Platform issues consulted:** none — no runner-only bullet is folded (the CI read at Setup 5a was in progress), and the WATCH carried on this entry plans no instrumentation
- **External inputs:** `inputs#I1` — the operator's phase directive (an ordinary chunk, no forced sample, keep the evidence, stop at a hard cascade failure, founder forks in the away form, no epoch boundary in Epoch 6, `df -h /tmp` before a stamped-home gate, the WATCH carried and tallied) · `inputs#I2` — overseer1's ruling on design-cascade-g1 (a one-off second re-spawn, which passed) and its direction that the rmcp 3.4.1 unbounded `read_until` on `viola mcp`'s stdin is this chunk's own subject, to be planned

## Files inspected
- `crates/viola-mcp/src/lib.rs` (full) — `serve` hands `rmcp::transport::stdio()` straight to `ServiceExt::serve` (line 73): stdin reaches rmcp with no bound of viola's; `ServeError` has `Runtime` · `Initialize` · `Session`, fixed messages
- `src/cmd/mcp.rs` (full) — the `mcp` verb: `home_passes`, obs init, `viola_mcp::serve(...)?`, then `log_self_exit(0, None)` and exit 0; stdin's close is the session's normal end
- `~/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/rmcp-3.4.1/src/transport/async_rw.rs` (lines 50–240, 380–445) — `AsyncRwTransport::receive` reads one line with `self.read.read_until(b'\n', &mut self.line_buf)` over a plain `BufReader<R>`: no length bound; `JsonRpcMessageCodec::new` sets `max_length: usize::MAX` (the codec's `new_with_max_length` serves only the writer side, `FramedWrite`); `Ok(0)` from the reader is `None` (session end); an `Err` is logged by rmcp's own target (OFF in viola's sink) and is `None` too; a line parses through `serde_json` at its default depth
- `crates/viola-channel/src/frame.rs` (lines 20–40) — `read_frame_within`: `io::Read::take(&mut *reader, cap).read_until(b'\n', …)` then `framed(line, cap)` (a line of exactly `cap` bytes, its `\n` included, is a frame; longer is `Oversize`) — the shape the MCP bound mirrors
- `crates/viola-channel/src/tokio_client.rs` (lines 80–92) — the Tokio client's `read_frame`: `.take(MAX_FRAME).read_until(…)` then `framed` — both channel sides bounded
- `crates/viola-channel/src/server.rs` (lines 170–180, 265–285) — `serve_conn` reads through `frame::read_frame`; `reject_frame` writes `parse-rejected{parser:"channel-frame", detail, count:1}` through `obs_event!` from the library crate itself: the precedent for a library-side boundary line
- `src/run/gate.rs` (lines 1–140) — the tee writes the human's bytes and only then offers a copy (`Tee::write`, line 133–136); the bounded `sync_channel(FEED_CAPACITY = 256)` with `try_send`; `Model::step` under `catch_unwind` (line 46) poisons and writes `parse-rejected{vt100-feed, panicked}`; a dropped copy writes `oversize` (line 72)
- `tests/chaos_feed_panic.rs` (full) — asserts one `parse-rejected{vt100-feed, panicked}`, one `event:"panic"`, `send` exit 13 `input-not-ready`, no `send-issued`, and a typed key reaching the child; it asserts nothing about the child's OUTPUT reaching the outer terminal after the panic
- `tests/support/home.rs` (lines 840–850) — `Wrapper::shown()` returns what the outer terminal has shown (the outer PTY's drained bytes): the witness the passthrough half can read
- `src/cmd/run.rs` (lines 190–200, 560–580) — `checked_exe` reads the process's own exe whole (`std::fs::read(std::env::current_exe()?)`); `statusline_command` reads its source through `take(MAX_FRAME)`
- `crates/viola-state/src/pin.rs` (lines 52–90) — the pinned copy is re-hashed through a chunked `digest` reader (bounded memory); `pin_exe` reads verify's own exe whole
- `crates/viola-state/src/fs.rs` (lines 340–358) — `replace_private_shared` falls back to `fs::read(path)` of the target (a plugin file or a ConPTY companion under the home) whole, only to compare it with `bytes`
- `crates/viola-mcp/Cargo.toml` (full) — `tracing` and `viola-core` are normal dependencies (the crate already logs through `obs_event!` in `tools.rs:304`); tokio `rt` only
- `deny.toml` (lines 40–60) — three `[[bans.features]]` tables (veil `toggle`, rmcp `transport-streamable-http-server` / `auth`, tracing-subscriber `env-filter`); none for serde_json
- `scripts/deny-probes.sh` (lines 1–60, 110–120) — one probe row per ban, each a throwaway project whose `cargo deny` output must carry the ban's diagnostic; last line `deny-probes: N/N banned, control clean`
- `scripts/rmcp-features.sh` (full) — the release-graph feature gate shape (cargo tree, one verdict line)
- `schemas/diag-line.v1.json` (lines 85–90) — `parse-rejected.parser` is the closed enum of nine values; `detail` the closed list holding `oversize`
- `tests/contract_lints.rs` (lines 1–45, 219–242) — a root contract test that walks `.rs` sources (`rs_files`) and asserts a source-level rule: the precedent shape for a source census
- `crates/viola-e2e/tests/scan_patterns.rs` (names) — a second source-walking test
- `.claude/rules/verification-harness.md` (full), `scripts/code-graph-cookbook.md` (full)

## Graph impact (from the code-graph query; trace `tree-query-2026-10-27-bounded-inputs-at-every-boundary.json`)
- **`viola_mcp::serve`** — 1 product caller, `cmd/mcp/mcp()` at `src/cmd/mcp.rs:36`; 1 test caller, `tests/serve_child_entry()` at `crates/viola-mcp/src/lib.rs:100`; `crates/viola-mcp/tests/handlers.rs` and `crates/viola-e2e/tests/mcp_tools.rs` drive the server through a spawned `viola mcp` or `Tools::call`, never `serve` with a fake stdin — a signature-preserving change has no caller to thread
- **`validate_paste_text`** — 6 product callers: `src/cmd/send.rs:205` (`refusal_of`), `src/run/send.rs:88` (`send`), `src/cmd/answer.rs:110` (`deliver`), `src/run/dialog.rs:425` (`DialogSlot::answer`), `crates/viola-mcp/src/tools.rs:295` (`refusal_of`), `crates/viola-mcp/src/tools.rs:419` (`answer`) — the client side (CLI send, CLI answer, MCP send, MCP answer) and the wrapper side (send, answer) each run it
- **`typed_text`** — 2 product callers, `src/cmd/send.rs:208` and `src/run/send.rs:91`, as the architecture states
- **`read_frame`** — product callers `client/impl#[Client]request()` at `crates/viola-channel/src/client.rs:77`, `server/serve_conn()` at `crates/viola-channel/src/server.rs:178` and `tokio_client/exchange()` at `crates/viola-channel/src/tokio_client.rs:78`: all three channel reads take the bounded frame
- **`replace_private_shared`** — 1 product caller, `cmd/run/pin_and_plugin()` at `src/cmd/run.rs:464`, and the pin module; its contract tests `replace_private_shared_fails_unless_the_bytes_are_already_there` and `replace_private_shared_in_a_read_only_dir_is_done_only_over_the_same_bytes` in `crates/viola-state/src/fs.rs`

## Patterns detected
- **Bounded line read, oversize closes** (`crates/viola-channel/src/frame.rs:22`): take the cap, read to the newline, judge the line in one place (`framed`); a line past the cap ends the connection with one `parse-rejected{…, oversize}` line.
- **Over-size detection** (`src/cmd/send.rs:80`, `src/cmd/hook.rs:178`, `crates/viola-state/src/stamps.rs:32`): `take(MAX_FRAME + 1)` then a length test, so an over-limit input is detected, never silently truncated.
- **Library-side boundary line** (`crates/viola-channel/src/server.rs:271`): a library crate writes its own codes-only `parse-rejected` through `obs_event!`, the root's subscriber carrying it into the role file.
- **Source census as a contract test** (`tests/contract_lints.rs:219` `rs_files`): a root test reads the workspace's sources and asserts a source rule, with a self-check case over planted text (`past_the_line_fires_at_the_kill_line_and_passes_below_it`).
- **Feature ban with a firing probe** (`deny.toml:44`, `scripts/deny-probes.sh:28`): each `[[bans.features]]` table has a probe row proving the ban fires.

## Conventions to follow
- **Oracles as literals** (testing.md §What to assert): the census's expected boundary set, its caps and its named exceptions are literals in the test, never imported from product code.
- **One fresh home under `target/e2e-home`** (testing.md §Test data): a process-level `viola mcp` witness owns its home; `chaos_feed_panic.rs` keeps the one `TestHome::outside_scan()` carve-out and adds none.
- **No new parser value improvised** (obs-plan §6, standing S2): a `parse-rejected` value an added bound writes joins `schemas/diag-line.v1.json`'s closed `parser` enum and the §6 catalog (an obs Decisions Log entry, the wrap's amendment).
- **Tokio only in `viola-mcp`** (architecture [Concurrency]): the MCP stdin bound is an async reader inside `viola-mcp`; no sync crate gains tokio.
- **Fixed-message errors, one enum per crate** (architecture [Error Handling]): no new error enum; `ServeError` stays the crate's one.
- **Names `<subject>_<condition>_<expected>`, rstest `#[case::label]`** (test-plan §2, §4).

## New files to create
- `crates/viola-mcp/src/stdin.rs` — the line-capped reader `viola mcp`'s stdin passes through before rmcp: a line of at most `MAX_FRAME` bytes, its `\n` counted, passes byte for byte; a longer one ends the stream there, with its inline unit cases
- `tests/contract_bounded_readers.rs` — the external-reader enumeration: a literal table of every boundary (reader, file, cap form, depth, type rule) and a source census over the product sources that reads red on a reader call outside the table or not bounded as the table says, with its self-check over planted text

## Files to modify
- `crates/viola-mcp/src/lib.rs` — `serve` reads stdin through the capped reader; an over-size line ends the session as stdin's close does and writes one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}`; the module declared; its process-level case beside `serve_on_a_closed_stdin_is_the_initialize_error`
- `crates/viola-state/src/fs.rs` — `replace_private_shared`'s compare reads the target through `take(bytes.len() + 1)`, never the whole file; a unit case over an over-long target
- `schemas/diag-line.v1.json` — the `parse-rejected.parser` enum gains `mcp-stdin`
- `deny.toml` — a `[[bans.features]]` table denying serde_json's `unbounded_depth`
- `scripts/deny-probes.sh` — the probe row that proves the serde_json ban fires
- `tests/chaos_feed_panic.rs` — the passthrough half: after the contained panic, the outer terminal has shown the very bytes that panicked the feed
- `crates/viola-e2e/tests/mcp_tools.rs` — an E2E case over a `viola mcp` child the test owns: a line past `MAX_FRAME` ends the session within the bound, one `parse-rejected{mcp-stdin, oversize}` line in `mcp.ndjson`, nothing on stderr and no frame past the ones already answered

## Sweeps
- product reader census `read_to_end\(|read_to_string\(|read_until\(|read_line\(|from_reader\(|fs::read\(|\.lines\(\)|BufReader::new\(` over `src/` and `crates/{viola-core,viola-pty,viola-channel,viola-state,viola-agent-claude,viola-mcp}/src/` outside `#[cfg(test)]` modules, `src/bin/viola-fake-agent.rs` and `test_support` left out (re-derived: `python3 $TMPDIR/{scratchpad}/reader_census.py .`): 31 hits · 1 changed (`crates/viola-state/src/fs.rs:354`, the whole-file compare read) · 30 no-change (24 read through `take(…)` in the same statement; 4 `BufReader::new` wrappers whose reads go through a bounded `read_frame` / per-line `take` — `crates/viola-channel/src/server.rs:175`, `client.rs:46`, `tokio_client.rs:33`, `crates/viola-state/src/events.rs:162`; 2 reads of the process's own executable, not external input — `src/cmd/run.rs:197` `checked_exe`, `crates/viola-state/src/pin.rs:83` `pin_exe`)
- the reader census does not reach a reader inside a dependency: `viola mcp`'s stdin is read by rmcp's `AsyncRwTransport::receive` (`read_until` with no bound), found by reading rmcp 3.4.1's source, not by the census — 1 changed (`crates/viola-mcp/src/lib.rs`)
- `unbounded_depth|disable_recursion_limit` over every `Cargo.toml` and over `src/` and `crates/*/src`: 0 hits (`command grep -rn`), and `Cargo.lock` records no features, so the absence is enforced by no gate today — 0 changed in source, the ban added (`deny.toml`)
- `"instances"` path joins over `src/` and `crates/{viola-state,viola-mcp,viola-channel,viola-agent-claude}/src/` (`command grep -rn -E 'join\("instances"\)|"instances"'`): every product join of `instances/<name>` takes a `ViolaName` (`src/cmd/{wait,last,answer,pause,send,client,link,revive,run}.rs`, `crates/viola-state/src/strict.rs:102`), the rest inside test modules — 0 changed
- `validate_paste_text` product callers (code-graph `calls`): 6, each client and each wrapper side — 0 changed
- `MAX_FRAME` over `crates/viola-mcp/src/` (`command grep -rn`): 0 hits — the crate holds no bound of its own today

## Open questions
- Does the outer PTY show the panicking wide character as the same three bytes on `windows-2025`, where ConPTY re-renders the child's output? → blocks: implementation-scope (the chaos passthrough assertion may need its per-OS oracle: the bytes on Unix, the character's presence on Windows, as the a11y plan's zero-viola-bytes oracle is split)
