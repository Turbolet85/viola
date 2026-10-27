# Task — a11y-plan · 2026-10-27-bounded-inputs-at-every-boundary

## The session
Bounded every external reader: `viola mcp` stdin now goes through a new line cap (`CappedLines` at `MAX_FRAME`); a line past the cap passes no byte, ends the session and writes one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` WARN line (MCP has no human surface, so a11y is n/a).
`replace_private_shared`'s fallback compare now reads at most `bytes.len() + 1` bytes; `deny.toml` bans serde_json's `unbounded_depth` feature, and the deny probe verdict moved from 13/13 to 14/14.
Added the E2E enumeration and census `tests/contract_bounded_readers.rs` (40 `BOUNDARIES` rows, 2 `EXCEPTIONS` rows); it runs under `run --e2e` inside the existing nextest-e2e suite.
Extended `chaos_feed_panic_refuses_send_and_keeps_passthrough`: after a forced vt100 feed panic, the tui passthrough is now asserted on the outer terminal. On Unix the child's `e4 b8 ad` bytes reach it within `WITHIN`. On every OS none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` appears, `send` is refused `input-not-ready` (exit 13) and a typed key reaches the child. It is green on ubuntu, macOS and windows-2025.
The Windows byte reading is unrunnable here, because ConPTY re-renders a wide character on a 1-column screen. It is owed to «Windows-only live measurements» by a P5 CARRY, PROVISIONAL on a delegate approval.
No spec master was edited and nothing was retired.

## Reached files
- crates/viola-mcp/src/stdin.rs
- crates/viola-mcp/src/lib.rs
- crates/viola-state/src/fs.rs
- schemas/diag-line.v1.json
- deny.toml
- scripts/deny-probes.sh
- tests/chaos_feed_panic.rs
- tests/contract_bounded_readers.rs
- crates/viola-e2e/tests/mcp_tools.rs
- viola-0.1.0/verification-matrix.json

## Touches
- The tui Entity line in scope-entities (`viola run` TUI passthrough, entered by `viola run` and a passed `viola revive`). Its boundary claim (zero viola bytes except the child's output, keystrokes never blocked) is now witnessed after a forced vt100 feed panic too. The report names this line, in the body §1 and in a11y-scope-summary, as the site for the new witness.
- The tui Surface line in scope-surfaces ("boundary assertions on the portable-pty outer-PTY driver check that viola emits zero terminal bytes of its own"). The outer-PTY check now also covers the contained feed-panic case: no viola literal on any OS, and the child's own bytes on Unix only.
- The tui row of the per-surface tool reach table in §4 / aria-landmarks ("zero viola bytes, keystrokes unblocked", all three OS legs). It now reaches the panic case. Windows reads only the literal absence; its byte reading is unrunnable here.
- The P4 tui scenario in path-p4-wheel ("keystrokes delivered unblocked past the current atomic paste" on all three OS legs). The chaos case adds a typed key reaching the child after a contained feed panic, and a `send` refused `not-delivered` / `input-not-ready` with exit 13 while the screen is poisoned.
- The scope-entities Owed lines, the pattern for a dated owed witness. A new owed item comes through the P5 CARRY on «Windows-only live measurements»: the Windows byte oracle for the panicking wide character.
- The Not-assertable plugin entity in scope-entities (hooks and the stdio MCP server, ipc-internal, no UI surface). The MCP stdin is now line-capped and an over-long line ends the session. Re-read unmoved: the report records a11y n/a, no human surface (`mcp-surface` a11y-plan 1, no-change).
- The home-tree check named through the `checked_exe` / `pin_exe` mechanism (`own-exe` a11y-plan 1). Re-read unmoved: the session only names the own-executable reads as no external input (the census `EXCEPTIONS`).
- The passthrough sites the report's sweep re-read and left unmoved: the body tui surface line (§1 Surfaces), multi-language not-triggered (Cyrillic, emoji and CJK appear only as passthrough content; the chaos case's `e4 b8 ad` is such content), a11y-scope-summary's head and later passthrough lines, visual-design-verification's passthrough line, and the registry `keyboard-test-harness` statusline pass-through test (no-change ×3).
- §9 CI Integration and §10 SLO invariants ("tui boundary cases run inside the per-OS `test` job's single instrumented `coverage` suite on all three OS legs"; "no new `suite` enum value"). The chaos case stays inside that suite, green on all three legs. The new `contract_bounded_readers` binary runs inside nextest-e2e and adds no suite value.

## New facts
- F1 · a11y-scope-summary · The tui boundary after a contained vt100 feed panic is witnessed by `chaos_feed_panic_refuses_send_and_keeps_passthrough` (tests/chaos_feed_panic.rs). On every OS none of `panic`, `parse-rejected`, `vt100`, `unable` or `hint:` reaches the outer terminal. On Unix the child's `e4 b8 ad` reach it within `WITHIN`. It is green on ubuntu, macOS and windows-2025.
- F2 · a11y-scope-summary · Owed: the Windows outer-terminal byte reading of the child's wide character after a feed panic. It is unrunnable here because ConPTY re-renders a wide character on a 1-column screen. It is owed to «Windows-only live measurements» by the P5 card's CARRY, PROVISIONAL until the founder's own word.
- F3 · aria-patterns-roles · P4 tui: after a forced feed panic, a typed key still reaches the child, and an automation `send` is refused `not-delivered` / `input-not-ready` (exit 13). The human keystroke is not blocked by the poisoned screen.

## Untouched
- cli surface, visual-sgr and the §6 CLI equivalent: no SGR, colour, NAME cut or `NO_COLOR` / `TERM=dumb` behaviour changed.
- cognitive-errors: no refusal code, exit code, error variant or `hint:` line was added. The `input-not-ready` refusal the chaos case reads is a standing one, and the MCP stdin cut is a session end with no CLI refusal.
- cognitive-plain-language: no fixed CLI string, help text or pinned literal changed.
- cognitive-consistency and path-p6-board: the `viola list` caption order, the `CAPTION` literal, the board, `wait` / `last` and `verify` output are not reached.
- cognitive-timeouts: the session adds no time bound that falls on the human. The MCP line cap is a size bound, and `WITHIN` and the 5 s / 8 s child kill deadlines are test bounds.
- aria-landmarks' cli structural equivalents (the BAY context line, the caption line, the UNWRAPPED separator, the `--help` group labels): not reached.
- The `parse-rejected` parser catalog gaining `mcp-stdin`, the deny probe count 13/13 → 14/14 and the serde_json depth ban: obs-plan, security-plan and test-plan facts with no a11y surface.
- The a11y tier, the compliance cross-check and the not-triggered list (wcag-mapping, regulated-compliance, cognitive, motion, target-size): unchanged.
- The Internal crates not-assertable entity: `viola-mcp` gains a crate-private module only, and no crate was added or removed.
