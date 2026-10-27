# Task — a11y · 2026-10-27-bounded-inputs-at-every-boundary

## The chunk
Security hardening: every input crossing into a `viola` process is bounded where it enters (size cap through `Read::take(MAX_FRAME)` or the `MAX_FRAME + 1` over-size form, serde_json default depth, closed types for decisions, `ViolaName::try_new` or a closed-shape validator before every path join).
The boundary set covers the stdin of `hook` and `mcp`, channel frames on both sides, the child's PTY output into the vt100 feed, `claude agents --json` and `claude --version` output, `send`'s `--file` / stdin text, the files read under the home or `~/.claude`, and the status line command's stdout; any reader found without a bound gets one.
It re-verifies the paste rule (`validate_paste_text`) at every client (CLI `send`, MCP `send`) before any frame and authoritatively in the wrapper before the first wheel read: refusal `not-delivered` / `control-character` or `empty-text`, never a strip.
It re-verifies that a vt100 screen-model panic degrades readiness to `input-not-ready` while the child's bytes still pass through to the human's terminal unharmed.
It adds a unit-level witness for each property at each boundary, and an enumeration of the external readers, each with its cap, depth and type rule.

## Reached files
- src/run/gate.rs · re-reads {the `catch_unwind` site: the screen-model panic degrade to `input-not-ready` and the passthrough}
- crates/viola-agent-claude/src/screen.rs · re-reads {the `catch_unwind` site: the screen-model panic degrade}

## Needs
- cli `viola send` paste refusal at the client, before any frame: `not-delivered` / `control-character` and `not-delivered` / `empty-text` (exit 13) in human mode as `unable <reason> <detail>` on stderr with `hint:` as the last stderr line and the typed exit code, and under `--json` one document with no hint.
- The kebab-case codes `not-delivered`, `control-character`, `empty-text` (and any code a newly bounded reader's refusal prints on the CLI) always appear beside the plain word `unable`, never as the only error text, never as a stack trace or bare status number.
- A CLI verb whose input is newly capped (an over-size `send` `--file` / stdin text, an over-size or malformed file under the home) still reports in text: a refusal follows the `unable` + `hint:` layout with a typed exit code, and a fault is no refusal and carries no hint.
- `viola revive`'s `--id` passing its closed session-id shape check: its exit-1 refusals stay the one `unable: <text>` line form, and `--json` the closed `{"v":1,"error":"<code>","detail":null}` document.
- tui boundary (`viola run` and a passed `viola revive`): the cap on the child's PTY output into the vt100 feed bounds only the screen-model feed; the passthrough of the child's bytes to the human terminal is never cut, delayed or blocked by it.
- tui boundary: a vt100 screen-model panic writes zero viola-originated bytes to the terminal (no panic text, no escape sequence); its diagnostics go to `diagnostics/`, never the terminal.
- Human keystrokes through the wrapper (SC 2.1.1 boundary stand-in, P4): no new bound, closed-type check or paste validation blocks, refuses or delays a human keystroke past the current atomic paste; the paste rule and its refusals apply to automation (`send`) only.
- Focus, mouse and resize sequences and terminal replies keep passing through and are still not counted as human editing when the input path gains bounds.
- cli `viola list`: rows built from the `claude agents --json` read, now parsed as closed types (status as a closed word, an unknown value skipped or counted), still render with C0/C1 control characters escaped except as the board rule allows, an unwrapped NAME cut with `...` never inside a `\xHH` escape, and the STATUS word (`idle` / `busy`) carried in text.
- The plugin hooks (`viola hook`, `hook statusline`), the stdio MCP server and the wrapper IPC channel are not-assertable surfaces (machine-to-machine, no UI): their stdin, frame and payload caps need unit witnesses, not output-discipline assertions.
- Internal crates (`viola-core` validators, `viola-state` readers, `viola-agent-claude` parsers) are library-only and not-assertable: their bounds are proven at the unit tier with no a11y assertion.
- `hook` stays exit 0 with no stderr and no body on every newly bounded or refused input (fail open), so no new refusal surface reaches a human terminal through the plugin.

## Standing conventions
- S1 · each new test runs inside the existing `nextest-integration` / `nextest-e2e` suites (CI: the per-OS `test` job's single `coverage` suite on all three OS legs), adds no `suite` value and no separate driver, and the `gate --require coverage,doctest` allows no skip · §1 A11y Scope Summary (CI integration) and §9 CI Integration of the body
- S2 · each new test is zero-flakiness: `retries: 0`, no `#[ignore]`, a flake keeps the chunk red until its cause is fixed · §10 SLO Invariants & A11y Budgets of the body
- S3 · each new or changed fixed CLI string (a refusal reason, detail or hint line) is pinned by trycmd or a literal, so a copy change fails CI · .andromeda/a11y-plan-detail/cognitive-accessibility.md
- S4 · each new human-mode CLI refusal ends with a `hint:` line (only `unknown`, exit 14, has none), carries a typed exit code, and its `--json` form is one document with no hint · .andromeda/a11y-plan-detail/cognitive-accessibility.md
- S5 · each new log or diagnostic line for a wrapped run goes to `diagnostics/`, never the terminal · .andromeda/a11y-plan-detail/a11y-scope-summary.md

## Not needed
- The SGR rules for `viola list` (amber only on `DIALOG`, dim on stale rows, bold on NAME, none under non-TTY / `NO_COLOR` / `TERM=dumb` / `--json`): the chunk changes no styled output and no colour decision.
- The structural equivalents (BAY context line, uppercase caption line, `-- UNWRAPPED - READ-ONLY --` separator, `--help` group labels): the chunk changes no layout or label.
- The `CAPTION` literal and its caption order `NAME · LIVE · STATUS · WHEEL · DIALOG · CLI`: the chunk adds or moves no column.
- The time bounds (`wait` timeout, MCP `wait` cap, dialog hold, readiness gate, confirmation windows, status-line deadline, the 5 s agents read, budget pause): the chunk adds size caps, not time bounds, and moves none.
- The `verify` `[NN/NN] … pass|fail` step lines and the `waiting: <name>` stderr line: the chunk changes neither layout.
- The `[RB]` readback mirror of a confirmed `send`: the chunk changes the refusal side of `send`, not the confirmed readback.
- The `unverified-cli` refusal of `viola answer` (path P3): the chunk changes no dialog-answer refusal text.
- The Windows platform fact that an injected mouse report under win32-input-mode arrives as typed keys and takes the wheel (F-W3): the chunk changes no wheel classification, and Windows-only live readings are out of scope.
- The owed `viola revive --json` document for a refusal `run`'s start makes after a passed preflight: it belongs to "Exit-cause code catalogue".
- The owed outer-PTY witness for dim-only-on-stale rows: it belongs to "Linux and macOS parity".
- The manual NVDA / VoiceOver / Orca passes and the manual keyboard pass: founder-owned supplements, not run by this chunk.
- Any frontend (web) surface and its a11y tooling: deferred to 0.2.0.
