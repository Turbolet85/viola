# Task — design-system · 2026-10-27-bounded-inputs-at-every-boundary

## The session
Bounded the last unbounded external reader: `viola mcp` now reads stdin through a new crate-private line reader (`CappedLines`, cap `MAX_FRAME`); a line past the cap passes no byte, ends the session as a closed stdin does, and writes one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` WARN process-log line (exit 0 once initialized, stderr empty, stdout only the earlier frames; a cut before `initialize` exits 11 as a closed stdin does).
`replace_private_shared`'s fallback compare now reads the target through `take(bytes.len() + 1)`, never the whole file; same outcome as a whole-file compare.
`deny.toml` bans serde_json's `unbounded_depth` feature; `scripts/deny-probes.sh` gains its probe row and reads `deny-probes: 14/14 banned, control clean` (was 13/13); the diag-line schema's `parse-rejected.parser` enum gains `mcp-stdin` (9 to 10 values).
Added `tests/contract_bounded_readers.rs` (40 `BOUNDARIES` rows, 2 `EXCEPTIONS` rows, a census of every product reader) and witnessed the tui passthrough after a forced vt100 feed panic: no viola literal on the outer terminal on every OS, the child's bytes reaching it on Unix, `send` refused `not-delivered` / `input-not-ready` (exit 13); the Windows byte reading is owed to Windows-only live measurements.
Measured: an architecture claim that a caught vt100 panic logs detail `oversize` is false (the code writes `panicked`). No IPC method, MCP tool, CLI verb, flag, exit code, env var, error variant or dependency added; nothing retired.

## Reached files
- crates/viola-mcp/src/stdin.rs
- crates/viola-mcp/src/lib.rs
- crates/viola-state/src/fs.rs
- schemas/diag-line.v1.json
- deny.toml
- scripts/deny-probes.sh
- tests/contract_bounded_readers.rs
- tests/chaos_feed_panic.rs
- crates/viola-e2e/tests/mcp_tools.rs
- viola-0.1.0/verification-matrix.json

## Touches
- Surface cli Platform-Specific Notes: `mcp` writes only MCP frames on stdout and has no human design surface — re-read against the new over-long-line end (stdout only the earlier frames, stderr empty, the one line in the process log); claim unmoved.
- Surface cli colour decision order item 3 (`viola mcp`, stdout carries MCP frames only): the oversize cut adds no stdout or stderr byte; unmoved.
- Surface cli colour decision order item 2 (`viola run`, nothing printed while the child runs) and the run component pattern (prints nothing once the child starts): now witnessed after a forced vt100 feed panic — none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` reaches the outer terminal on any OS; unmoved claim, newly tested.
- The cli per-surface ban on colour, glyphs or cursor control under `viola run` (the terminal belongs to the child's screen, a passed `viola revive` the same passthrough): the feed-panic passthrough test reads it; unmoved.
- Typography cli rule that viola's human output is ASCII only: the child's wide character (`e4 b8 ad`) passing through after the panic is the child's bytes, not viola's output; unmoved.
- The `viola send` readback mirror's `[/ ] unable` line and its reason/detail pair: a send after a feed panic is refused `not-delivered` / `input-not-ready`, exit 13; words and exit unchanged.
- Semantic Colors domain status rows for a refused send (folds into its send line, `unable` plus a typed reason): the `input-not-ready` refusal after a feed panic uses the existing treatment; unmoved.
- Exit-code phraseology (codes 0, 1, 2, 10 to 14, 20, 21 and their human forms): `viola mcp`'s exit 0 at an over-long line and exit 11 at a cut before `initialize` print no human form, `mcp` being outside the human surface; no code added or moved.
- Stack traces never print, full detail only under diagnostics: a caught vt100 panic and an over-long MCP line each write only a process-log `parse-rejected` line (detail `panicked` / `oversize`), never terminal text; unmoved.
- The cli ban on printing upstream text, paths or pids, and the NEVER-log floor it cites: no byte of an over-long MCP line reaches any home-level file (the E2E canary read); unmoved.
- `viola list` pattern 1 (the `claude agents --json` join, its C0/C1 escaping): that read's child pipe drain is now a named `BOUNDARIES` row (`take(MAX_FRAME + 1)`); the board's columns, escaping and failed-read behaviour unchanged.
- The `viola list` BAY line's budget figures and gate word, read from `config.json` thresholds and `budget.json`: the three `config.json` readers (thresholds, dialog hold, `claude_env_keep`) are now enumerated `BOUNDARIES` rows; values, defaults and printed words unchanged.
- Component pattern 5's `verify` step counter (relied capability-ledger rows, `n pass m fail`): no capability-ledger row added, its relied count unchanged; verify's shell-verdict file, the two capture arms and the `--answers` file are now enumerated boundary rows only.
- The route verification matrix row v1-46 (every external reader bounded) set `implemented`: reaches design-system only where it cites a matrix row or its state.
- The cli Toolkit line (clap 4.6.7 without `color`, windows-sys 0.61.2, the hand-written SGR module adding no dependency): `Cargo.lock` untouched, no dependency added or bumped; the serde_json feature ban constrains no cli toolkit choice; unmoved.
- Any design-system line naming the deny probe verdict `13/13` or the `parse-rejected` parser catalogue (nine values): now `14/14` and ten values with `mcp-stdin`.

## New facts
- (none)

## Untouched
- Brand Identity (personality, domain anchors, the readback box and its three states, `unconfirmable`, expression level 0.2): no readback, post-condition or vocabulary change.
- Color Palette values, contrast table, surface scale, text hierarchy and border progression: no hex, role or contrast moved.
- Typography role stacks, per-OS fallbacks and Linux render assertions: a 0.2.0 frontend input, untouched.
- Spacing, Depth Strategy, Border Radius and Motion tokens: no length, rule, radius or duration moved.
- Iconography glyphs and the ASCII `->` rule: no glyph added.
- Navigation Pattern: the flat verb list and the `viola --help` rows are unchanged; `mcp` stays hidden, no verb or flag added.
- Component pattern 3 (`wait` / `last` result lines and the message-mode escaper): unchanged.
- Component pattern 4 (pause, release, `release --budget`, link, unlink, answer, `plugin install` lines): unchanged.
- The `revive` lines and the pending owner for run's post-preflight `--json` document (Exit-cause code catalogue): unchanged.
- The exit-21 strict-modes `unable:` / `hint:` pairs of list, link, unlink, send, wait, last, answer, pause, release and `plugin install`: no strict-modes change this session.
- The web-spa surface (deferred to 0.2.0, technology not chosen) and the Self-Validation Protocol's Token Test: no page and no token.
- The Anti-Patterns' rejected defaults (status dots, spinners, rings, auto-sorting, robot icons, glow): no new output form.
