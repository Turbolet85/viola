# Task — security · 2026-10-27-bounded-inputs-at-every-boundary

## The chunk
Makes every input that enters a `viola` process from outside bounded and validated where it enters: a `Read::take(MAX_FRAME)` (or the `MAX_FRAME + 1` over-size form) before any buffering, serde_json at its default depth, closed types for every decision, `ViolaName::try_new` (or a closed-shape validator) before every path join; a reader found without its bound gets one.
The external readers it covers: hook and mcp stdin, channel frames on both sides, PTY output into the vt100 feed, `claude agents --json` and `claude --version` output, `send`'s `--file` / stdin text, the home's files and `~/.claude/settings.json`, and the status line command's stdout.
Re-verifies the paste rule at every client (CLI `send`, MCP `send`) and, authoritatively, in the wrapper before the first wheel read; re-verifies that a vt100 parser panic degrades readiness to `input-not-ready` while byte passthrough to the human continues.
Adds the proof: a unit-level witness per property and per boundary, plus a checkable enumeration of every external reader with its cap, depth and type rule, so a new reader without a bound shows up.

## Reached files
- crates/viola-core/src/lib.rs
- crates/viola-channel/src/frame.rs
- crates/viola-channel/src/server.rs
- crates/viola-channel/src/client.rs
- crates/viola-channel/src/tokio_client.rs
- src/cmd/hook.rs
- src/cmd/hook/statusline.rs
- crates/viola-agent-claude/src/hook.rs
- crates/viola-agent-claude/src/dialog.rs
- crates/viola-agent-claude/src/agents.rs
- crates/viola-mcp/src/tools.rs
- src/cmd/send.rs
- src/cmd/answer.rs
- src/run/send.rs
- src/run/agents.rs
- src/run/version_gate.rs
- src/run/budget.rs
- src/cmd/run.rs
- src/cmd/revive.rs
- src/cmd/mod.rs
- crates/viola-state/src/events.rs
- crates/viola-state/src/snapshot.rs
- crates/viola-state/src/stamps.rs
- crates/viola-state/src/budget.rs
- src/run/gate.rs · re-reads the vt100 feed's `catch_unwind` site (the degrade to `input-not-ready` and the passthrough)
- crates/viola-agent-claude/src/screen.rs · re-reads the screen model's `catch_unwind` site (the poisoned model's readiness)

## Needs
- The `MAX_FRAME` constant (16 MiB, one const in `viola-core`) and `Read::take(MAX_FRAME)` / the `MAX_FRAME + 1` over-size form on every external reader before `read_line` / `read_to_end` / any buffering. An over-limit input is detected and refused, never silently truncated.
- Channel frames on both sides: the cap comes before `read_line`; an oversize frame is `-32600` and closes the connection; serde_json default depth; the `v` check; closed types for `behavior`, `dialog_id` and names; each method's params checked by the wrapper (`-32602`) before `validate_paste_text`; an invalid `hook.dialog` is not registered and an invalid `hook.event` is not appended.
- Hook stdin, every event plus `hook statusline`: capped at `MAX_FRAME`; mapped into the closed `HookEvent`; one JSON object; security-deciding fields go into closed enums. `option_details` description / preview are kept only as strings (else `null`), and the Stop's `background_tasks` is read as `IgnoredAny` and kept only as its length. Every failure fails open: exit 0, no body, no stderr.
- The hidden capture arms' bounds: `hook <event> --capture` stdin through `take(MAX_FRAME + 1)`; the `--answers` id through `take(64)` into the closed `ProbeAnswer`; the statusline capture writes only the closed `ShellVerdict`; no `VIOLA_*` read, no channel, exit 0 always.
- MCP stdin and tool inputs (`target`, `text`, `dialog_id`, `response`, `after`, `timeout_ms`): the schemars schemas are advisory only, so each handler checks the raw arguments itself (`ViolaName`, `u64`, the closed response enums, `validate_paste_text` on text and free-text answers). A failure is `invalid-params` with a closed field word and no frame.
- The paste rule (Input Validation, Paste text and Dialog free text rows): over decoded `char`s, never raw bytes. LF / CR / TAB are allowed; every other C0, DEL and C1 is refused `not-delivered` / `control-character`, whole, never stripped. The typed text has each CR LF and each other CR as one LF, with no trailing CR or LF. An empty typed text is `empty-text` (exit 13). The check runs in the client before any frame and again in the wrapper before the first wheel read; answer free text gets the same check in `viola answer` and in the wrapper's `answer` handler.
- CLI `send` and `answer` input (stdin / `--file`): read under `MAX_FRAME`, into the closed enums or through `validate_paste_text`, before any frame and after the instance check `client::refuse_untrusted`.
- The PTY output bytes into the vt100 feed: fed under `catch_unwind`, the human's bytes written first. A caught panic or a dropped copy poisons the screen model, which then reads `input-not-ready` while passthrough continues. The feed queue is bounded, and a full queue never blocks passthrough.
- Child process output: `claude agents --json` is read through `take(MAX_FRAME + 1)` from a direct spawn under the R8 strip, killed at 5 s; a failed, oversize or malformed read fails open; an unwrapped row's `name` is never a `ViolaName` or a `target`, and a row's status is a closed word. The `claude --version` probe output is read under the cap and parsed tolerantly, failure becoming `unknown`. `viola verify`'s PTY output is read only against the compiled signature literals.
- Own state files on read (`snapshot.json`, `events.ndjson` replay and tail, `ledger/stamps.json`, `budget.json`, `heartbeat`): each ndjson line is capped at `MAX_FRAME`, and an over-long, non-object or unterminated line counts as torn. A reader never writes. Reads happen only after the home strict-modes check. A snapshot replayed from the log carries no `endpoint`, `pid`, `started_at` or `statusline_command`. `budget.json` is read only through `read_budget` (`take(MAX_FRAME + 1)`), and every failure there is no reading.
- `config.json`: a tolerant parse under `take(MAX_FRAME)` with `v` checked; `five_hour` / `seven_day` go into `Percent` (0–100) and `dialog_hold_s` into `DialogHold` (60–3600). A bad value takes its key's default and writes one `parse-rejected{parser:"config-json"}` line per key. No value switches the budget gate off or makes the hold zero or unbounded, and an identity-floor name in `claude_env_keep` has no effect.
- The statusline source read at `run` start: `<home>/statusline-source.json`, else, for the default home only, `~/.claude/settings.json`, both under `take(MAX_FRAME)`. The command is taken only from a `statusLine` of type `command`, non-empty, with no NUL. viola writes neither source.
- The status line shell-out's stdout (Unix): under `take(MAX_FRAME)`, printed byte for byte and never parsed, stderr discarded, the wait bounded at 5 s; the command reaches no line or span, and this remains the only shell-out.
- Depth: every serde_json parse of external bytes runs at the default recursion limit; serde_json's `unbounded_depth` feature is never enabled anywhere in the product graph.
- Closed types: `behavior`, `dialog_id`, the wheel holder, the dialog kind, a hook event kind, a channel method and its params, an MCP tool argument, a config value and an agents row's status are never free `String`s. An unknown value is refused, or skipped and counted as its format's rule says, never passed into a decision.
- Names before path joins: `ViolaName::try_new` in the clap `value_parser` (CLI positionals, `link` / `unlink`'s `<driver> <driven>`), in the MCP handlers (`target`), in the channel `link` / `unlink` params (`driver` / `driven`) and in the notify path filter (tail only `instances/<name>/events.ndjson` under the canonical home, ignoring symlinks and names that fail). Never join `instances/<name>` with an unvalidated string.
- Home resolution, through which `VIOLA_NAME` / `VIOLA_DIR` reach a path: `--home` as given, else the `instance_of` home only when `VIOLA_NAME` passes `ViolaName::try_new` and `VIOLA_DIR` is an absolute `<home>/instances/<name>` (a failing pair falls through and never refuses), else `<user home>/.viola`. `canonical_home` creates nothing.
- `revive --id` and the session id as a child argv value: the closed session-id shape (`is_session_id`); one argv element of a direct spawn built only by `resume_args`; an id the log does not hold is `no-session` before any spawn; no refusal line holds the recorded directory, a pid or a logged id.
- The pinned key that joins `bin/<version>-<hash>/` and `plugin/<key>/`: a closed shape (version plus truncated SHA-256) before the join.
- Ordering a new bound must respect: every read under the home comes after that entry point's strict-modes check (`check_instance` / `check_home_tree` / `check_path`), so a bounded read the chunk adds or moves never comes before it.
- Upstream text (prompts, `last_assistant_message`, plan, question and option text, tool `input`) is content, never a command or config. A closed-type mapping over a hook payload decides only on fields viola owns.
- The human always wins: a bound or refusal the chunk adds goes only to automation (`send` / `answer` / MCP / channel clients), never blocks or delays a human keystroke, and the hook keeps failing open.
- The refusal shapes the witnesses read, unchanged: channel `-32600` for an oversize frame, `-32602` for bad params, `result.refusal` `not-delivered` / `control-character` and `not-delivered` / `empty-text`, MCP `invalid-params` with a closed field word, hook exit 0 with no body.
- The parser surfaces named for fuzz and property coverage (`validate_paste_text`, channel ndjson framing with `MAX_FRAME`, the hook stdin parser including `hook statusline`, the `claude agents --json` parser, the vt100 feed under `catch_unwind`) as the proof's per-boundary witnesses; tests own the cases.
- The threat model's boundary set (paste, terminal, hook stdin, MCP, CLI, filesystem, child spawn, and the IPC frame's input half) as the list the external-reader enumeration is checked against.

## Standing conventions
- S1 · Each new test runs under its own `--home` (never the user's `~/.viola` or `~/.claude`), with a fake agent first on `PATH` where it reaches `claude agents --json` · .andromeda/security-plan-detail/secret-management.md
- S2 · Each new test input and fixture is synthetic: no R8-stripped `CLAUDE*` value, real prompt, home path or username; the CI artifact canary scan reads it before any upload · .andromeda/security-plan-detail/bootstrap-phases.md
- S3 · A new log line goes only through `obs_event!`, and user content, tool `input` and drift-report values reach only `instances/<name>/diagnostics/` (0600), never stderr, `--json` errors, MCP `isError` or channel `error.data` · .andromeda/security-plan-detail/bootstrap-phases.md
- S4 · A new fuzz or property target stays in the separate `fuzz/` workspace, never linked into `viola` and never in the root `[workspace]`, its lockfile audited on its own · .andromeda/security-plan-detail/dependency-security.md
- S5 · A test-only helper, seam or feature stays out of the release build (`scripts/release-check.sh` refuses `test-support` / `fake-agent` artifacts); a new environment read or seam needs its own Decisions Log entry · .andromeda/security-plan-detail/dependency-security.md
- S6 · A new CI step pins actions by full SHA, keeps `permissions: {}` / `contents: read`, and takes no `github.event` value through `${{ }}` in `run:` · .andromeda/security-plan-detail/dependency-security.md
- S7 · A test-side removal under `target/e2e-home` removes only a dir the test itself created · .andromeda/security-plan-detail/security-anti-patterns.md

## Not needed
- The Windows pipe listener's protected SDDL, `accept_remote(false)` and the SQOS client open, the Unix 0700 socket directory, the socket chmod, the `peer_creds` euid check and the macOS no-peer-pid gap: owned by "Unix endpoint and home hardening", out of scope.
- Re-auditing every existing error body for fixed messages, anyhow chains and absolute paths: owned by the next entry, "Sanitised error surfaces".
- The exit-cause code catalogue and the documents a refusal after a passed `revive` preflight owes: owned by "Exit-cause code catalogue".
- The pinned copy's no-clobber placement, its re-hash before reuse and the `plugin/` rewrite at every start: landed by the previous chunk, not reopened here.
- Windows DLL search restriction and the sideloaded ConPTY companions' strict-modes check and SHA-256 re-hash, plus the vendored ConPTY audit: no input boundary this chunk bounds.
- `viola verify`'s interactive runs (which children, which pastes, probe dirs and their removal, the transcripts accepted under `~/.claude`): the chunk adds no run, paste or spawn.
- `viola plugin install`'s marketplace tree and its `.mcp.json` exec path: not an external reader this chunk bounds.
- The `events.ndjson` retention and local-disk exhaustion accepted risk: an arch-owned open item, not a size cap on a read.
- The driver prompt-injection accepted risk, the `unverified-cli` stamp gate on non-`null` decisions and `release` carrying `from` (`release-from-driver`): decision policy, not input bounding.
- The `Last-Event-ID` / resume-cursor clause and any web or frontend input surface: carried to 0.2.0.
- Secret rotation, access auditing and the v1.x release-signing prerequisites (dist, cargo-auditable, self_update `signatures`): no release in v1.
- Windows-only live readings of any boundary: owned by "Windows-only live measurements".
