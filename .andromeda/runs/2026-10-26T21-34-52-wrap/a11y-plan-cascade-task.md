# Task — a11y-plan · 2026-10-26-home-and-code-bearing-file-integrity

## The session
The session put the strict-modes check (owner-only modes on Unix, owner and DACL on Windows) before the first read or open at every entry point that lacked one, and retired all eight dated strict-modes gaps (8 → 0).
The six frame verbs `send`, `wait`, `last`, `answer`, `pause` and `release` now run `check_instance` first through `client::refuse_untrusted`. A refusal prints `link`'s instance pair (`unable …`, then `hint:` last), or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`. The exit is 21, with no frame and no line.
`viola run` and `viola revive`'s start check the home tree before any log. A refusal is exit 1 with the home pair or the instance pair, and no child starts. `viola verify` checks the home, `ledger/`, `bin/` and `plugin/` before its first step line and refuses with the home pair, exit 1.
The session adds a read rule: Unix refuses any group or other bit on the home, the instance directory and `diagnostics/`; Windows refuses a read grant to another SID. It also adds the `conpty/` companions' owner and DACL leg, whose refusal falls back to the inbox ConPTY with `sideload_fallback` `strict-modes-failed` and the exit unchanged. The `viola hook` event and dialog paths and the `viola mcp` frame tools also check first.
Measured: on Unix an unstattable home (a path under a regular file) is now a strict-modes refusal. On Windows such a path stats `NotFound`, so the catch-site fault still stands there. CI is green on all three OS legs (ci#40403389629, 15/15).

## Reached files
- crates/viola-state/src/strict.rs
- crates/viola-state/src/stamps.rs
- crates/viola-state/src/liveness.rs
- crates/viola-state/src/pin.rs
- crates/viola-mcp/src/tools.rs
- src/cmd/client.rs
- src/cmd/link.rs
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs
- src/cmd/hook/statusline.rs
- src/cmd/run.rs
- src/cmd/revive.rs
- src/cmd/verify.rs
- src/obs.rs
- schemas/diag-line.v1.json
- tests/security_negatives_strict_modes.rs
- tests/chaos_snapshot_replay.rs
- crates/viola-mcp/tests/handlers.rs
- tests/hook_fail_open.rs
- tests/hook_statusline.rs
- tests/run_cli.rs
- tests/cli_verify.rs
- tests/conpty_sideload.rs
- tests/cli_controls_not_disableable.rs
- tests/support/home.rs
- tests/cli_revive.rs
- tests/cli_wait_last.rs
- tests/contract_diag_schema.rs
- tests/cli_machine_contract.rs
- tests/tui_env_strip.rs
- viola-0.1.0/verification-matrix.json
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/scope-record.md
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/evidence/remove-the-guard.md
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/evidence/operator-pass.md

## Touches
- Error recovery (§8): any statement listing which verbs print a refusal pair with a final `hint:`. The six frame verbs `send`, `wait`, `last`, `answer`, `pause` and `release` now print the instance pair, exit 21, on a refused strict-modes check. Before this session only `revive` and `link` / `unlink` printed that pair.
- Error recovery (§8): the `--json` refusal of the six frame verbs on a refused check is the hint-free `instance-unreachable` document, `detail: null`.
- Error recovery verification (§8): the parenthetical on legs ("the frame verbs' server-verify refusal on windows-2025 and ubuntu-latest only: macOS reports no peer pid"). The strict-modes refusal of the same verbs is a different check and is witnessed on all three legs (`tests/security_negatives_strict_modes.rs`). Any text that implies the frame verbs' refusals are two-leg only must be read against this.
- Error recovery (§8): any text on `viola verify`'s refusals or its `unable: <text>` interrupted pair. `verify` now also refuses with the home pair, exit 1, before any `[NN/NN]` step line or summary when its home tree fails the check.
- Error recovery (§8), on faults: "faults are not refusals and carry no hint". On Unix an unstattable home moved from a catch-site fault to a strict-modes refusal with a hint (`verify` home pair, frame verbs instance pair). On Windows the catch-site fault stays.
- Error recovery (§8): any list of where the home pair stands (pattern `home-pair`, 3 a11y-plan hits). `run` now prints the home pair, exit 1, for a home-level tree refusal, and the instance pair for an instance-level one. `verify` prints the home pair.
- a11y-scope-summary.md line 8: `home-pair` hit @c1784, read as true. Re-read it against the new `run` / `verify` sites.
- a11y-scope-summary.md line 8 @c2438 and cognitive-accessibility.md line 45: `cannot-occur` hits. These say a server-verify refusal cannot occur on macOS, which is a different fact from obs-plan's "`strict-modes-failed` cannot occur on `wait` / `last`" (now stale). Read them as no-change unless they speak of strict-modes.
- a11y-scope-summary.md line 24: `no-line` hit. Re-read it against the new no-line refusal sites: the six verbs, the hook event and dialog paths, the MCP frame tools, `verify`, the tree check in `run` / `revive`, and the diagnostics opens.
- Plain language verification (§8): the list of fixed CLI strings pinned by literal names the server-verify hint and the `link` / `unlink` result lines. The instance pair and home pair now reach six more verbs plus `run` and `verify`. Check whether the pinned-string list must name these pairs.
- Scope surfaces, the `viola run` TUI entered by `run` and by a passed `revive`: a refused home tree now stops the start before any log or child, so the TUI is never entered. That refusal is printed CLI text on stderr, not a TUI byte.
- Scope surfaces, "viola's diagnostics for `run` go to `diagnostics/`, never to the terminal": `diagnostics/` and its role and detail files are now checked (symlink refused, read rule). A refused open ends at a sink and puts nothing on the terminal. Re-read as holding.
- TUI boundary (zero viola-originated terminal bytes, P4): a `conpty/` companion refused by its owner/DACL check silently degrades to the inbox ConPTY. The exit is unchanged and the only record is the `sideload_fallback` `strict-modes-failed` log word, with no terminal byte. Re-read as holding.
- Scope entities, the CLI verbs list: the verbs are unchanged, but `send`, `wait`, `last`, `answer`, `pause` and `release` gain a refusal (exit 21) in human and `--json` modes.
- Not-assertable surfaces (hooks, MCP server): `viola hook`'s event and dialog paths now fail open silently on a refused or absent instance (exit 0, empty streams). The MCP frame tools return `instance-unreachable`, `detail: null`. They stay ipc-internal; re-read only if the plan describes their output.
- Owed under `viola revive --json` (scope summary): revive's start tree check keeps `strict-modes-failed`. A refusal `run`'s start makes after a passed preflight still prints no document, so the item to Exit-cause code catalogue stays owed. Re-read as unmoved.

## New facts
- F1 · cognitive-accessibility · `send`, `wait`, `last`, `answer`, `pause` and `release` refuse an instance that fails the strict-modes check before any frame. They print the instance pair (`unable …`, then `hint:` last) on stderr with exit 21, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` with no hint, and write no line. This is witnessed on all three OS legs.
- F2 · cognitive-accessibility · `viola run` and `viola revive`'s start refuse a home tree that fails the strict-modes check before any log is opened. A home-level refusal prints the home pair and an instance-level refusal prints the instance pair, exit 1, and no child starts. `revive --json` keeps the `strict-modes-failed` document.
- F3 · cognitive-accessibility · `viola verify` refuses a home whose tree (the home, `ledger/`, `bin/`, `plugin/`) fails the strict-modes check with the home pair, exit 1, before any `[NN/NN]` step line or summary and with no line.
- F4 · cognitive-accessibility · An unstattable home (a path under a regular file) is a strict-modes refusal with a hint on Unix (`verify`: home pair, exit 1; frame verbs: instance pair, exit 21). On Windows the path stats `NotFound` and the catch-site fault, with no hint, stands. The tests use a per-OS oracle.

## Untouched
- Timeout extensions (§8): the session adds no time bound on the human. The hook's empty-streams-under-1.0 s check is a test bound on a fail-open path, not a limit on the human.
- Visual design verification and the SGR rules (§6): no styling changed. The new refusals go through the existing `human::refuse`, so `unable` stays uncoloured, and `viola list`'s rendering is unchanged.
- Consistency and predictability (§8): the `CAPTION` literal and `viola list`'s caption order are unchanged; `board.rs` is not in the diff.
- P6 board (`viola list`): its home-check inconsistency (a 0750 home passing with no instance, refused with one) is routed to a route entry. It is no a11y fact, and `list`'s output did not change.
- P4 wheel path and keyboard (§4, §5, §11 Keyboard): no wheel, keystroke, paste or mouse-handling change.
- ARIA landmarks inventory: the structural equivalents (caption line, separator, `--help` group labels) are unchanged, and `viola --help` gains no row.
- CI integration, SLO invariants and a11y budgets (§9, §10): no harness, suite enum, gate or workflow change. The new tests run inside the existing nextest suites.
- Not triggered triggers (motion, multi-language, target-size, cognitive escalation): no new surface reaches them.
- The macOS server-verify sentences: macOS still reports no peer pid, so the server-verify refusal's two-leg reach is unmoved. Only the separate strict-modes refusal runs on three legs.
