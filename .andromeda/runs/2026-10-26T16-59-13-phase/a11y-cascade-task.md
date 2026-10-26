# Task — a11y · 2026-10-26-home-and-code-bearing-file-integrity

## The chunk
Security hardening of the viola home: the strict-modes (Unix mode/owner, Windows owner + DACL) check moved before any read or open of the home at every entry point, closing the dated exceptions.
The entry points it reaches: the CLI frame verbs `send`, `wait`, `last`, `answer`, `pause`, `release`; `viola hook`'s nine events (failing open); `viola mcp`'s frame tools; `verify`'s stamps read; `run`'s start (home, instance dir, `bin/`, `plugin/`, `ui/`, trusted files, Windows `conpty/` companions); and `client::start`'s role-file open.
It also re-hashes the pinned `bin/<version>-<hash>/viola` before reuse and owns the home copy's first-start race, rewrites `plugin/` atomically on every start, keeps one writer each for `snapshot.json` and `ledger/stamps.json`, and looks at the `marketplace/` tree's integrity.
It re-measures whether two clean release builds of one tree differ, and reads on Windows which DACL entry refuses a test-seeded home. Moving the snapshot readers onto the log replay is the founder's to rule at P4.

## Reached files
- src/cmd/client.rs · edits client::start's role-file open
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs · edits the snapshot read through VIOLA_DIR
- src/cmd/hook/statusline.rs
- src/cmd/mcp.rs
- src/cmd/verify.rs · edits update_stamps' stamps read
- src/cmd/run.rs · edits run's start checks and collision check
- src/cmd/plugin.rs
- src/obs.rs · edits open_role_file
- src/conpty.rs
- crates/viola-state/src/strict.rs · edits check_instance
- crates/viola-state/src/fs.rs · edits create_private_dir
- crates/viola-state/src/liveness.rs · edits live_endpoint
- crates/viola-state/src/pin.rs · edits pin_exe
- crates/viola-state/src/stamps.rs
- crates/viola-state/src/snapshot.rs
- tests/cli_controls_not_disableable.rs · edits its 0770 `--home` negative (exit 21) and the table's completeness case
- tests/support/home.rs · edits seed_conpty
- tests/hook_statusline.rs

## Needs
- The CLI frame verbs' new refusal when the strict-modes check fails before the snapshot read (`send`, `wait`, `last`, `answer`, `pause`, `release`): in human mode, `unable  <reason>  <detail>` on stderr, then `hint:` as the last stderr line, with a typed exit code (exit 21); under `--json`, one document with no hint.
- A kebab-case reason or detail code that the new refusals print always sits beside the plain word `unable`; no stack trace or bare status number is ever the only error text, including for a check that cannot be made.
- Refusal and `fail` text is never coloured, and no SGR is written under non-TTY, `NO_COLOR`, `TERM=dumb` or `--json` on any line these checks add.
- The tests for the new refusals: assert_cmd pins the typed exit codes and the hint-free `--json`, and trycmd pins the `unable … hint:` layout, on the three OS legs. The strict-modes refusal holds on macOS too, unlike the server-verify refusal, which is pinned on windows-2025 and ubuntu-latest only.
- `tests/cli_controls_not_disableable.rs`: its 0770 `--home` negative is a CLI refusal (exit 21, stderr `unable … hint:`, nothing on stdout in human mode).
- `viola run`'s start, now checking the home, the instance dir, `bin/`, `plugin/`, `ui/`, the trusted files and the Windows `conpty/` companions, and re-hashing the pinned binary: any refusal comes before the child starts. Once the passthrough runs, viola writes zero terminal bytes of its own, and its diagnostics go to `diagnostics/`, never to the terminal.
- A `viola revive` whose preflight passed enters `run`'s start, so the same zero-byte TUI boundary holds there. Under `--json`, a refusal that `run`'s start makes after a passed preflight still prints no document; that stays owed to "Exit-cause code catalogue" and is not filled here.
- Any time bound the chunk adds (a lock or wait around the pinned-binary copy at a first start, a re-hash, the `plugin/` rewrite) falls on viola's own start or child. None blocks, refuses or delays a human keystroke.
- The new checks run before `run`'s child exists or in separate processes (`hook`, the CLI verbs, `mcp`). None sits on the human keyboard path through the wrapper, and none delays a keystroke past the current atomic paste.
- `viola verify`'s output if the stamps read is refused by the new check: linear static step lines `[NN/NN] … pass|fail`, or the one-line `unable: <text>` form, with `fail` never coloured.
- `viola plugin install`'s output if the `marketplace/` integrity work changes it: its fixed result lines are pinned by literal, so a changed line updates the pinned literal on purpose.
- `hook` keeps failing open on a refused check (exit 0, no body, no stderr), and the MCP frame tools return `instance-unreachable` as a code. Neither writes to a terminal.

## Standing conventions
- S1 · every new CLI or TUI output-discipline case runs inside `nextest-integration` / `nextest-e2e` locally and inside the per-OS `test` job's single `coverage` suite on all three OS legs; no new `suite` value and no separate a11y driver outside `scripts/agent-run.* run` · §3 A11y Assertion Harness Contract / §9 CI Integration / §11 CI
- S2 · every fixed CLI string a change adds or alters is pinned by trycmd or by a literal, so a copy change fails CI until it is updated on purpose · .andromeda/a11y-plan-detail/cognitive-accessibility.md
- S3 · zero flakiness: `retries: 0`, no `#[ignore]`, no retry-once policy; a flake keeps the chunk red until its root cause is fixed · §10 SLO Invariants / §11 SLO
- S4 · the per-OS `gate --require coverage,doctest` never allows skips, and a breach fails the build · §9 CI Integration / §10 SLO Invariants
- S5 · no WCAG conformance claim rests on assert_cmd, trycmd or portable-pty results; they stand in for output discipline only · §11 Strategy

## Not needed
- The deferred 0.2.0 frontend: page ARIA landmarks, a browser a11y leg, design tokens for a page. 0.1.0 renders no page.
- The `viola list` board structure: the `CAPTION` literal and its order, the BAY context line, the `-- UNWRAPPED - READ-ONLY --` separator, and the `--help` group labels. The chunk does not change the board's layout.
- The P2 confirmed-send `[RB]` readback mirror: the chunk adds a check before the frame, not a change to the readback.
- The P3 `unverified-cli` withholding text of `viola answer`: unchanged by a home check.
- The wheel's own behaviour: focus, mouse and resize sequences not moving the wheel, the Windows F-W3 injected-mouse fact, and the `human-typing` refusal of `send`.
- The manual NVDA / VoiceOver / Orca and keyboard passes: a founder-owned supplement this chunk does not schedule.
- The triggers that did not fire (motion and spinners, multi-language, target size, cognitive AAA escalation, regulated compliance): no surface of this chunk reaches them.
- The `dim`-on-stale-rows outer-PTY witness owed to "Linux and macOS parity": not this chunk's.
