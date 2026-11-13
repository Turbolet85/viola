@history_item n=1 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (226), each verified against the code; the picked rows (decision/*, contract/viola-verify-output/*, contract/frame-verbs/strict-modes-refused, contract/viola-revive-json/*, etc.) are this landing's, so a refactor that keeps every verdict must keep each row's text true as the code shows it.

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — landed `viola verify`'s output structure (step lines, `stamped` summary, failing-row exit 1 with the stamp still written, usage) and its refusal pairs, pinned in `tests/cli_verify.rs`; that file is split here (item 3), so every test asserting these lines (contract/viola-verify-output/*, contract/viola-verify-refusals/*) must move byte-identical with its name kept.

@history_item n=3 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added the static `--help` paragraph and the named fixture refusal `a recorded fixture is not clean: <file> <code>` (codes home-path · absolute-path · username · email, never the content); contract/viola-verify-output/help-paragraph and contract/viola-verify-refusals/fixture-not-clean are pinned in `tests/cli_verify.rs` and the fixture scanner — the split and the shared source-masking scanner must leave them unchanged.

@history_item n=4 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's stopped form: stdout empty, no summary, no stamp, the `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` pair, exit 1 (contract/viola-verify-refusals/stopped, contract/viola-verify-output/stop); its tests live in `tests/cli_verify.rs` and move with the split.

@history_item n=5 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — `MM` became a rule (`LedgerRow::ALL.len()`) rather than a literal, later the relied count (2026-10-13-owed-ledger-rows); the step-line contract (contract/viola-verify-output/step-line) rests on it, and `crates/viola-agent-claude/src/ledger/test_cases.rs` (split in item 3) holds the ledger cases whose count must stay unchanged.

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` is `LedgerRow::relied()`'s length, one step line per relied row, exit 0 when every relied row passes (contract/viola-verify-output/exit-0, step-line); the ledger test-case split must not change the relied set or its order.

@history_item n=7 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — verify at 24 relied rows ending `local-command-compact`, the last row landed; the count the split ledger test cases and `tests/cli_verify.rs` assert today.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count and stop on one OS) — `WRAPPED_HERE` folded, Windows relied set and console-control stop arm removed, `SIGINT` / `SIGTERM` unconditional; the precedent for item 10's dead `#[cfg(not(unix))]` constants in `crates/viola-e2e/src/harness/run.rs` on the Linux-only tree, with Linux output byte-identical.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows VT enable leaves; the .cmd/.bat refusal leaves; the server-check hint without its OS scope; per-OS help wording) — the rule that a closed word or arm no build can emit leaves with its emitter and that per-OS clauses are re-worded with the claim kept; the trap for this chunk is removing non-unix code without moving any human or `--json` line (decision/human-output-is-contract).

@history_item n=10 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the census dropped the «on Unix» qualifiers on the socket-directory refusals (contract/run-revive-start/socket-dir-refused) and fixed Linux as the one target; confirms the dead non-unix constants carry no surface of their own.

@history_item n=11 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — `check_home_tree` / `TreeRefused`: verify, run and revive refuse a widened home, `ledger/`, `bin/`, `plugin/` or pinned copy before the log opens (contract/run-revive-start/home-tree-refused, contract/viola-verify-refusals/home-not-private); the strict-modes code in `crates/viola-state/src/strict.rs` where the dead `check_present` sits (item 4), and the reason tests on 0755 homes went red (item 6's helper must make homes at 0700 as viola does).

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six frame verbs' exit-21 instance pair) — `client::refuse_untrusted` runs the strict-modes check first in every frame verb, before any role file or log opens, mode left as found (contract/frame-verbs/strict-modes-refused); deleting or wiring `check_present` must not alter this order or its verdicts.

@history_item n=13 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the strict-modes refusal of the driven instance) — the trap: opening a role file narrows an existing home to 0700, so a check placed after `client::start` passes a 0770 home; `link_verbs_refuse_a_widened_instance_before_any_log` reads the mode after the refusal. Bears on the 0700 home helper (item 6) and on the `src/run/links.rs` ↔ `src/run/snapshot.rs` merge (item 2), which must keep the check-before-open order.

@history_item n=14 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the `link` / `unlink` result lines; exit-21 lines, server check and catch site) — link/unlink act on the driven instance through `src/run/links.rs`'s state, their lines byte-pinned by `link_verbs_print_the_design_lines`; the links/snapshot merge (item 2) is behaviour-unchanged against these.

@history_item n=15 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document carries its cause in `detail`) — strict-modes refusals under `--json` carry `"detail":"strict-modes-failed"` (contract/instance-unreachable-detail/strict-modes-failed, contract/viola-list-json/strict-modes-failed, contract/viola-plugin-install/strict-modes-refused); the strict.rs cleanup and the error-kind guard tests of `fs.rs` / `pin.rs` (item 5) must keep these codes.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's `internal-error` document; revive post-preflight gap closed) — one catch site prints `error: internal error` or `{"v":1,"error":"internal-error","detail":null}` (contract/cli-catch-site/internal-error, contract/run-revive-start-json/*); `tests/contract_error_bodies.rs`, whose `blanked` scanner item 1 replaces, guards these bodies — its verdicts must stay unchanged.

@history_item n=17 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `reply_of` / `of_reply` screen peer replies so `unknown` carries a code word or nothing; the error-surface sanitising that `tests/contract_error_bodies.rs` asserts, which item 1's shared scanner must serve with verdicts unchanged.

@history_item n=18 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (socket-directory refusal of the starts; `socket-dir-refused` at the frame verbs) — `refuse_socket_dir` runs before any snapshot, child or bind and never changes a mode (contract/run-revive-start/socket-dir-refused, contract/run-revive-start-json/socket-dir-refused); a precedent constraint on the `src/run/snapshot.rs` merge and on the home/guard helpers' modes.

@history_item n=19 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is a hidden verb with no human surface, its tools the CLI verbs' `--json` less `v`, refusals `isError: true` (decision/mcp-server-without-human-surface, contract/mcp-tool-results/*); item 5's viola-mcp survivor tests (`lib.rs` WARN `parse-rejected`, `stdin.rs` buffer) and `viola-mcp-tap` tests exercise this surface and must not add a human line.

@history_item n=20 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — one `claude agents --json` read joined to the board, a failed read printing the board as with no claude (contract/viola-list-board/status-join, agents-read-failure), and revive's session-live preflight order; the fake agent `src/bin/viola-fake-agent.rs` (split in item 3) serves these reads in tests, so its split must keep every behaviour.
