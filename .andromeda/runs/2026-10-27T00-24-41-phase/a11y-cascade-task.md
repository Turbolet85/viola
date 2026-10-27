# Task — a11y · 2026-10-27-pinned-copy-race-and-reproducible-builds

## The chunk
Closes the pinned-copy race: two first `viola run`s of one version on a fresh home must not both replace `bin/<key>/viola(.exe)`, and no agent's hook spawn of the pinned path may fail because another start's write of it is in flight.
The fix sits in the absent-copy write of `pin_bytes` / `pin_exe` (through `replace_private_shared` → `replace_private`), reached from `run`'s start and from `viola verify`, with a two-sided witness: the race reproduced red before the fix and clean after it, beside the single-instance control.
Re-reads whether the Windows exe arm needs the sharing-violation wait `pin_companions` already has, and whether the per-start `plugin/<key>/` rewrite can fail a reader of `hooks.json` / `.mcp.json`.
Measures whether two clean release builds of one committed tree give one file, and where they do not, which sections and notes differ; any build or product change on that basis is a founder fork at P4.
Changes none of the hash-before-reuse, plugin-rewrite or home-tree strict-modes rules; Windows is read on the CI runner only.

## Reached files
- crates/viola-state/src/pin.rs · edits pin_bytes, pin_exe (the absent-copy arm)
- crates/viola-state/src/fs.rs
- src/cmd/run.rs
- src/cmd/verify.rs
- scripts/release-check.sh · re-reads its release build (the profile and `CARGO_TARGET_DIR` the measurement uses)

## Needs
- The `viola run` TUI passthrough boundary: the pin step of `run`'s start (and of a `viola revive` start whose preflight passed, which goes through `run`'s start) writes no byte of viola's own to the terminal; any wait, retry or failure in the concurrent pin goes to `diagnostics/` or the wrapper log, never to the screen.
- Human input priority over the PTY: any wait the fix adds (a lock, or waiting out a sharing violation, as `pin_companions` does) happens before the child is spawned or off the input path, and never blocks, refuses or delays a human keystroke past the current atomic paste.
- Time bounds: a bounded wait or retry added to the pin is one of viola's own time bounds, so it falls on viola's own start, never on the human, and it needs no warn-and-extend pattern.
- Error recovery on the CLI: if the fix adds a refusal or failure exit to `run`'s start or `viola verify` (for example a pin that cannot be completed), it prints `unable  <reason>  <detail>` on stderr with `hint:` as the last stderr line and a typed exit code; under `--json` one document with no hint; a fault is no refusal and carries no hint.
- Plain language: any new reason or detail code from the pin path sits beside the plain word `unable`, and no stack trace or bare status number is the only error text.
- Fixed copy pinned: any new or changed fixed CLI string from the pin path (a refusal line, its hint, a `verify` step line) is pinned by trycmd or by literal, so a copy change fails CI.
- `viola verify`'s output stays linear and static: its `[NN/NN] … pass|fail` step lines keep their order when `pin_exe` changes, and `fail` and `unable` are never coloured.
- Hooks and the plugin (`viola hook`, the `plugin/<key>/` files read by `claude`) are a not-assertable surface: a fix to the hook spawn's failure (receipt `ran:false`) or to the plugin rewrite adds no a11y assertion of its own.
- The two-sided witness and any narrower concurrent-pin control are output-discipline-neutral tests: they assert pin outcomes, not rendered `claude` TUI content, and never parse the vendor screen or use screenshots as evidence.

## Standing conventions
- S1 · Each new test runs inside `nextest-integration` / `nextest-e2e` locally and inside the per-OS `test` job's single instrumented `coverage` suite in CI, gated by `gate --require coverage,doctest`; no new `suite` value and no separate driver outside `scripts/agent-run.* run` · §3 A11y Assertion Harness Contract and §9 CI Integration
- S2 · Zero flakiness: `retries: 0`, no `#[ignore]`, no retry-once policy; a flake keeps the chunk red until its root cause is fixed · §10 SLO Invariants & A11y Budgets and §11 SLO
- S3 · No WCAG conformance claim rests on any assert_cmd / trycmd / portable-pty result this chunk adds · §11 Strategy

## Not needed
- `viola list`'s board: the caption line and the `CAPTION` literal order, the `-- UNWRAPPED - READ-ONLY --` separator, the NAME cut with `...`, and the `\xHH` escaping of rows; the chunk does not touch `list`.
- The SGR rules of `viola list`'s human mode (bold NAME, amber `DIALOG`, dim stale row) and the `NO_COLOR` / `TERM` / `COLORTERM` colour facts; the pin path emits no styled output.
- The owed outer-PTY witness for dim only on stale rows, owed to "Linux and macOS parity".
- The owed `viola revive --json` document for a refusal `run`'s start makes after a passed preflight, owed to "Exit-cause code catalogue".
- The fixed copy set of the empty board (`no wrapped sessions — start one with …`), `unknown` and `n/a`, and the pinned `--help`, `plugin install` and `link` / `unlink` result lines.
- The `send` readback mirror `[RB]`, the `human-typing` refusal, `answer`'s `unverified-cli` refusal and `pause` / `release`; no path P2, P3 or P4 verb changes.
- The wheel's handling of focus, mouse and resize sequences and terminal replies, and the Windows win32-input-mode mouse report fact (F-W3).
- `waiting: <name>` on stderr and the `wait` / `last` outputs.
- The manual NVDA / VoiceOver / Orca and keyboard passes on P4 and P6; they are founder-owned supplements and the chunk adds no step to them.
- The ARIA landmark inventory and the deferred 0.2.0 frontend; 0.1.0 renders no page.
- The untriggered areas: wcag-mapping, regulated compliance, cognitive accessibility AAA, motion, multi-language and target size.
- Internal crates as a not-assertable surface; the pin change inside `viola-state` adds no a11y entity.
