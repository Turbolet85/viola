# security · cascade read · group 2 of 3

## Carried
- (.andromeda/security-plan-detail/input-validation.md:11) The enumeration must name the `send` refusals on the CLI, MCP and channel sides with exactly these closed pairs and no text of the refused input.
  > the MCP `send` tool in `viola-mcp`, since chunk 2026-10-17-mcp-server-for-drivers: `validate_paste_text`, then the empty-typed-text check, each refused as `{"refusal":"not-delivered","detail":"control-character"|"empty-text"}` with one client-side `send-refused` line, before any frame)
  (.andromeda/security-plan-detail/input-validation.md:11)
  > **Wire outcome:** refusal `not-delivered` with detail `control-character`, then `not-delivered` with detail `empty-text`, both checked before the Conventions refusal order.
- (.andromeda/security-plan-detail/input-validation.md:14) The `answer` surfaces' refusal for free text must be enumerated as this closed code, on both sides, holding none of the refused text.
  > refused with `control-character`, checked first, on both sides: in `viola answer` before any frame (exit 13) and again, authoritatively, in the wrapper's `answer` handler.
- (.andromeda/security-plan-detail/input-validation.md:17) Every channel protocol fault the wrapper answers must be proved and enumerated with these codes, fixed messages and `data`, and the changed `channel-response` line must tell these `-32602` causes apart.
  > **Size cap:** wrap the stream reader in `Read::take(MAX_FRAME)` before `read_line`; an over-limit frame gets `-32600` and the connection is closed.
  (.andromeda/security-plan-detail/input-validation.md:17)
  > **Version:** the `v` check (`-32602`).
  (.andromeda/security-plan-detail/input-validation.md:17)
  > anything else is `-32602` `"invalid params"` with `data: null`, before `validate_paste_text` runs.
  (.andromeda/security-plan-detail/input-validation.md:17)
  > `release`'s `budget` is absent or a bool (anything else, `null` included, is `-32602` `"invalid params"` with `data: null`), and its `from` is absent or `null` — a string `from` is `-32602` `"invalid params"` with `data: {"reason":"release-from-driver"}`, any other type `-32602` with `data: null`.
  (.andromeda/security-plan-detail/input-validation.md:18)
  > a missing, non-string or invalid name, a `driven` other than the serving wrapper's own name and a `driver` equal to `driven` are each `-32602` `"invalid params"` with `data: null`, no line written;
- (.andromeda/security-plan-detail/input-validation.md:22) Editing `AgentError` and any `Display` or refusal shape must leave `viola hook` failing open on every path, its hidden capture arms included.
  > a malformed dialog payload fails open (`AgentError::DialogMalformed`: exit 0, no body)
  (.andromeda/security-plan-detail/input-validation.md:24)
  > any failure prints and writes nothing; stderr is always empty and the exit always 0.
- (.andromeda/security-plan-detail/input-validation.md:27) The MCP surface's `invalid-params` body must hold this shape in the witness and the enumeration, and no handler may reach a serde failure's text.
  > each handler takes the raw argument object, never rmcp's typed deserialisation (a serde failure would carry serde text)
  (.andromeda/security-plan-detail/input-validation.md:27)
  > A failure is `isError: true` with `structuredContent` `{"error":"invalid-params","detail":"<field>"}` (the closed field words `target` · `text` · `after` · `timeout_ms` · `dialog_id` · `response`), text `refused: invalid-params`, and no frame; an unknown tool name is JSON-RPC `-32602`.
  (.andromeda/security-plan.md:341)
  > MCP `structuredContent` carries only `refusal`/`detail`/`error` codes.
- (.andromeda/security-plan-detail/input-validation.md:30) The owed test must drive a stdin line past `MAX_FRAME` before `initialize` completes and assert the whole-line refusal, the one line holding no byte of it, and exit 11.
  > a longer line releases no byte and is refused whole, never truncated and never skipped, and the reader then reads as end of stream, so the session ends as a stdin close does
  (.andromeda/security-plan-detail/input-validation.md:31)
  > after the session ends, `serve` writes one WARN `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` through `obs_event!`, holding no byte of the line, and returns what the session gave: exit 0 for an initialized session, `ServeError::Initialize` (exit 11) when the cut lands before `initialize` completes.
- (.andromeda/security-plan-detail/input-validation.md:37) The CLI surface's enumeration must hold `revive --json`'s closed documents, codes and session-shaped ids only, leaving the start-refusal documents to the exit-cause catalogue.
  > `--json` is accepted on both arms (chunk 2026-10-17-cli-machine-contract): a refused strict-modes check or a preflight refusal then prints exactly `{"v":1,"error":"<code>","detail":null}` on stdout (stderr empty, exit 1), `<code>` one of the closed `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` (through `client::refused_document`), and `--list --json` prints `{"v":1,"ok":{"sessions":[{"ts","cause","id"},…]}}` in log order (the escaped `ts`, the closed cause word, the session-shaped id) — codes, closed words and session-shaped ids only, never a path or pid; a refusal `run`'s start makes after a passed preflight prints no document (owed to the route entry "Exit-cause code catalogue")
  (.andromeda/security-plan-detail/input-validation.md:36)
  > a refusal is exit 21 with the instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, with nothing opened under the home and no line.
  (.andromeda/security-plan-detail/input-validation.md:36)
  > a refusal returns `instance-unreachable` with `detail: null`, no frame and no line.
- (.andromeda/security-plan-detail/input-validation.md:42) The cap-driving cases of the three `config.json` readers must drive past the cap and assert these outcomes, including that a document no longer an object gives the defaults and no rejection from these reads.
  > a value must be a JSON whole number that fits `Percent` (0–100, `Percent::try_new`), else that key alone takes its default and is reported as one `parse-rejected{parser:"config-json", detail:"malformed"}` line per bad key; a document that is no object, or whose `v` is not 1, gives both defaults and no rejection from this read; no value of either key switches the gate off.
  (.andromeda/security-plan-detail/input-validation.md:42)
  > a JSON whole number of seconds that fits `DialogHold` (60–3600, `DialogHold::try_new`; default 60, `DIALOG_HOLD_DEFAULT`), else the hold takes 60 and is reported as one `parse-rejected{parser:"config-json", detail:"malformed"}` line; a document that is no object, or whose `v` is not 1, gives 60 and no rejection from this read; no value makes the hold zero or unbounded, and no env var or flag sets it
  (.andromeda/security-plan-detail/input-validation.md:42)
  > `claude_env_keep` (the Unix R8 pass-list) is an array of at most 32 names, each matching `^CLAUDE[A-Z0-9_]*$`; a bad value is rejected WHOLE (never partly accepted) and reported as `parse-rejected`, and an identity-floor name in it has no effect (not an error). `v` is checked, and the read is capped at `MAX_FRAME`.
- (.andromeda/security-plan-detail/input-validation.md:45) The witness's planted `CLAUDE*` value must sit under a name the R8 rule strips on every OS the test runs on, and the check must rest on names, never on a value read by the strip.
  > `viola run` reads value NAMES only, never data,
  (.andromeda/security-plan-detail/input-validation.md:45)
  > The names only decide which inherited `CLAUDE*` names the R8 rule keeps, and an identity-floor name is stripped whatever the registry holds
- (.andromeda/security-plan-detail/input-validation.md:54) The terminal colour facts must be among the values the witnesses and enumeration hold out of every error body.
  > They decide only whether and how deep SGR bytes are written around rows already escaped; they reach no log line, error or refusal, are never set by viola, change no escaping or check, configure nothing, disable no control and widen no redaction.
- (.andromeda/security-plan-detail/input-validation.md:58) The recorded `cwd` must be held out of every error body, and the stamps errors' `Display` must stay fixed where they reach `verify`'s pair.
  > the `cwd` the read yields is used as the spawn directory and for nothing else.
  (.andromeda/security-plan-detail/input-validation.md:59)
  > a refusal is `StampsError::StrictModes` (its other error `StampsError::State`), reads and writes nothing, and `verify` maps it to the home pair.
- (.andromeda/security-plan-detail/input-validation.md:66) The cap-driving case for the child pipe drain must drive past the cap and assert the bound and the fail-open or `unknown` outcome.
  > the `--version` probe is read with `Read::take(MAX_FRAME)` and parsed tolerantly.
  (.andromeda/security-plan-detail/input-validation.md:66)
  > its stdout read through `take(MAX_FRAME + 1)` and killed at 5 s, and parsed tolerantly by `viola_agent_claude::agents::parse` (an element with no string `sessionId` skipped and counted, an extra key tolerated and counted) into rows or the closed `AgentsFailure` (`oversize` · `malformed`, one `parse-rejected{parser:"claude-agents-json"}` line); a failed, oversize or malformed read leaves `list`'s board as `board::read` gave it (STATUS `unknown`, no unwrapped row) and passes `revive` (fail open).
  (.andromeda/security-plan.md:160)
  > read under `MAX_FRAME` and parsed tolerantly, a failed `claude agents --json` read failing open;
- (.andromeda/security-plan-detail/input-validation.md:74) Whichever enum holds the batch-script refusal after the fold or ratification, it must keep this exit, line and fixed stderr pair, and its `Display` must be fixed with no path.
  > Every other `.cmd`/`.bat`, including a `claude.cmd` without that sibling, is refused with the closed `Refusal::BatchScriptChild`: exit 1, no child spawned, `process-exit{subject:"self", exit_code:1, detail:"batch-script-child"}`, and the two fixed `unable:`/`hint:` stderr lines with no path or pid — because portable-pty 0.8.1 builds its own `CreateProcessW` command line without std's BatBadBut escaping.
  (.andromeda/security-plan.md:340)
  > The `Display` impls on the `<Crate>Error` enums (`thiserror`, or hand-written for `PtyError`) must not interpolate upstream text or absolute paths.
- (.andromeda/security-plan-detail/input-validation.md:77) `revive`'s refusal lines, hints and `no-session` body must hold no recorded directory, pid or logged id, and its `--list` only ids of the closed shape.
  > An `--id` the log does not hold, or a log with no session, is `no-session`: exit 1 before any spawn.
  (.andromeda/security-plan-detail/input-validation.md:77)
  > `revive --list` prints only ids of that shape. No refusal line or hint holds the recorded directory, a pid or a logged id.
- (.andromeda/security-plan-detail/input-validation.md:80) The `statusline_command` string must be among the values held out of every error body, and the carve-out must read it as standing in the snapshot alone.
  > A source that is absent, unreadable or of another shape gives no command, and the read writes no log line and no span. The command is recorded as `snapshot.json`'s `statusline_command` (omitted when absent; `v` stays 1) and stands in no other file.
- (.andromeda/security-plan-detail/input-validation.md:83) The status-line cap-driving case must be Unix only, drive stdout past `MAX_FRAME`, and assert the capped byte-for-byte print, the bounded wait and no log line or span carrying the command.
  > on Windows that function returns `None` for every input and no command runs.
  (.andromeda/security-plan-detail/input-validation.md:83)
  > Its stderr is discarded. Its stdout is read through `take(MAX_FRAME)` and printed byte for byte, never parsed; a failing command prints nothing. The wait is bounded by `STATUSLINE_DEADLINE` (5 s, PROVISIONAL, `src/cmd/hook.rs`; detail `deadline`). No log line and no span carries the command.
- (.andromeda/security-plan-detail/input-validation.md:96) Every cap-driving case must drive past this one const, and the five edited rows must name a witness that does so, the census and its self-check still holding.
  > **Constants:** `MAX_FRAME = 16 MiB`, one const in `viola-core`,
  (.andromeda/security-plan-detail/input-validation.md:96)
  > child output (the `--version` reads of `run`'s gate and `viola verify`, stdout and stderr drained through the cap, killed at 5 s; and the output of verify's five interactive PTY runs, A to E; and the `claude agents --json` read of `list`, of `revive`'s preflight and of verify's Run B, `take(MAX_FRAME + 1)`, killed at 5 s, over the cap the closed `oversize`)
  (.andromeda/security-plan-detail/input-validation.md:98)
  > **Enumerated and censused:** `tests/contract_bounded_readers.rs` (an E2E `contract_` binary) names each external boundary's bounded read and its witness in `BOUNDARIES`; its census `product_readers_are_each_bounded_or_named` requires every product reader call to be bounded in its statement or named in `EXCEPTIONS`, and a self-check reads a planted unbounded read as a hit.
- (.andromeda/security-plan-detail/data-protection.md:8) The re-read of the owner-only detail leg must read these modes on each OS it claims, or the leg is no claim.
  > **Files (`~/.viola/`):** Plaintext by design (Established Decisions: Database / State Store). Protected by OS permissions only:
  (.andromeda/security-plan-detail/data-protection.md:9)
  > Unix: 0700 directories and 0600 files, except the pinned `bin/<version>-<hash>/viola`, which is 0700.
  (.andromeda/security-plan-detail/data-protection.md:10)
  > Windows: the inherited `%USERPROFILE%` DACL (user + SYSTEM + Administrators), or an explicit protected user + SYSTEM DACL for a `--home` outside `%USERPROFILE%`.
  (.andromeda/security-plan.md:204)
  > `events.ndjson` and the files in `instances/<name>/diagnostics/` are 0600 inside 0700 directories.
  (.andromeda/security-plan.md:205)
  > serde_path_to_error drift reports, which can quote tool `input`, are written only to `diagnostics/`.
- (.andromeda/security-plan-detail/data-protection.md:58) The changed `channel-response` line must add closed cause words only and keep this no-name property for `link` and `unlink`.
  > the wrapper's `link` / `unlink` methods open no span and write no process-log line of their own; its `channel-request` / `channel-response` pair carries `method:"link"|"unlink"` and no name, so no driver name reaches the wrapper's process log.
- (.andromeda/security-plan-detail/data-protection.md:64) Full error detail must go only through this checked open, and a refused open must drop the line, never moving it to stderr or to one of the three surfaces.
  > `open_role_file` and `write_detail` run `viola_state::strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before `ensure_private_dir` narrows anything: a symlink is refused; a folder takes the read rule; a file takes the read rule on Unix and the write rule on Windows; an absent path passes. A refusal is a failed open: the writer ends at `std::io::sink` and the detail line is dropped.
  (.andromeda/security-plan.md:342)
  > anyhow chains stop at the `viola` bin's human-mode stderr. A chain printed there never includes a serde_json or serde_path_to_error source, or any other source whose `Display` can quote upstream text (serde errors quote the offending value). Map it to a fixed message before it joins the chain, and write the full error only to `instances/<name>/diagnostics/` when an instance is resolved (NEVER-log floor, `logging-redaction-wire`).
- (.andromeda/security-plan-detail/bootstrap-phases.md:11) The `release-from-driver` cause must stay its own `-32602` cause, told apart from `InvalidParams` and the unsupported version by a closed word on the changed line.
  > Amendment 4 (`release-from-driver`): landed in the `viola` bin channel dispatch with chunk 2026-10-04-the-wheel (`viola_channel::ProtocolError::ReleaseFromDriver`, −32602).
  (.andromeda/security-plan-detail/bootstrap-phases.md:16)
  > the read rule (Unix `mode & 0o077`; Windows no `READ_RIGHTS` grant to another SID) on the home, `instances/<name>/` and its `diagnostics/`;
- (.andromeda/security-plan-detail/bootstrap-phases.md:24) The re-read of the secret-scan leg must read this scan, its inputs and its upload gate, and the witnesses' planted value must be accounted for against what it reads, with no matched byte printed or written.
  > The CI `test` job does run obs-plan §9's artifact canary scan: `viola-harness secret-scan` over test-home diagnostics, the harness capture except `run --mutants`' `target/agent-run/chunk.diff`, and the nextest JUnit report, checked against this plan's NEVER-log floor.
  (.andromeda/security-plan-detail/bootstrap-phases.md:24)
  > It never prints or writes the matched bytes, and every scan-gated upload (`diag-`, `junit-`, `harness-<os>`) waits for it to pass.
  (.andromeda/security-plan.md:344)
  > Never write `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` or any other R8-stripped `CLAUDE*` value to any log, diagnostic, snapshot, `events.ndjson` or fixture.
  (.andromeda/security-plan.md:345)
  > Scope: this floor governs log lines, diagnostics and error output. It does not restrict the contract payloads that carry content by design: `events.ndjson`, `wait`/`last` results (CLI and MCP), `hook.dialog`, and `statusline_command` in `snapshot.json` (the user's command stands there only; `instances/<name>/settings.json` holds the pinned path and the words `hook statusline`). Outside those payloads, user content (prompts, `last_assistant_message`, plan, question and option text), tool `input`, the `statusline_command` string, and serde_path_to_error drift reports (paths and values) go only to `instances/<name>/diagnostics/`. They never go to stderr log or error lines, CLI `--json` error output, MCP `isError` results or channel `error.data`.
  (.andromeda/security-plan.md:346)
  > Operational metadata (TMS §1): absolute paths, which carry the OS username (the only PII in scope), and internal type names never go to CLI `--json` error output, MCP `isError` results or channel `error.data`. Pids, `agent_session_id`, `started_at` and `budget.json` values may be written to `diagnostics/`.

## Bindings
- **input-validation's channel, MCP and CLI bodies ↔ bootstrap-phases' error sanitisation**: the bodies the validation rows state are the only bodies the sanitisation rule admits, so the enumeration and the rows must name one set, (.andromeda/security-plan-detail/input-validation.md:17), (.andromeda/security-plan-detail/input-validation.md:27), (.andromeda/security-plan-detail/input-validation.md:36) and (.andromeda/security-plan.md:341).
- **input-validation's `-32602` causes ↔ data-protection's process-log rule ↔ obs-plan's `viola.compat.v_rejected` row**: the `channel-response` line tells the causes apart in closed words that hold no param value and no name, read against the metric the line feeds, (.andromeda/security-plan-detail/input-validation.md:17), (.andromeda/security-plan-detail/bootstrap-phases.md:11) and (.andromeda/security-plan-detail/data-protection.md:58).
- **data-protection's owner-only diagnostics ↔ bootstrap-phases' NEVER-log floor**: what the floor sends away from the three surfaces lands only in files the strict open guards, and a refused open loses the line rather than moving it, (.andromeda/security-plan-detail/data-protection.md:64), (.andromeda/security-plan-detail/bootstrap-phases.md:16) and (.andromeda/security-plan.md:345).
- **input-validation's R8 name rule ↔ bootstrap-phases' secret scan**: the witness plants a value under a name the strip removes, and the scan's `claude-stripped` needles are what read the artefacts it leaves, so the planted name and the needles must be reconciled in the witness, (.andromeda/security-plan-detail/input-validation.md:42), (.andromeda/security-plan-detail/input-validation.md:45) and (.andromeda/security-plan-detail/bootstrap-phases.md:24).
- **input-validation's bounded-reader rows ↔ its `BOUNDARIES` census**: each cap-driving case's oracle is its reader's own row, and the census row must name that case as its witness, (.andromeda/security-plan-detail/input-validation.md:42), (.andromeda/security-plan-detail/input-validation.md:66), (.andromeda/security-plan-detail/input-validation.md:83) and (.andromeda/security-plan-detail/input-validation.md:98).
- **input-validation's `Refusal` row ↔ bootstrap-phases' `Display` rule ↔ the hook's fail-open**: the fold or ratification keeps the refusal's exit, line and fixed stderr pair, its `Display` stays fixed, and no change to `AgentError` alters the hook's silence, (.andromeda/security-plan-detail/input-validation.md:74), (.andromeda/security-plan-detail/input-validation.md:22) and (.andromeda/security-plan.md:340).
