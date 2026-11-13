@history_item n=1 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (guards: 6 re-pointed, 2 left out), each row checked against the code; a row's `guard:` names a test file (e.g. `decision/json-failure-documents-clean` · guard `tests/cli_error_bodies.rs`), so the file splits and scanner moves of this chunk must leave every guard path and test name a row points at still resolving.

@history_item n=2 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 2's fixed stderr lines; the `--json` failures the witness drives) — `tests/contract_error_bodies.rs` names `UNUSABLE_RESPONSE` / `UNUSABLE_TEXT` in `NOT_ERROR_BODIES`, and `tests/cli_error_bodies.rs` witnesses every `--json` failure as one document, literal exit, empty stderr, no ESC/path/upstream text; the shared masking scanner replacing `blanked` in `contract_error_bodies.rs` must keep these verdicts (rows `contract/cli-exit-codes/2`, `decision/json-one-document-on-stdout`, `decision/json-failure-documents-clean`).

@history_item n=3 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the code word and the peer-reply screen; exit 20's closed `detail`; `unknown`'s detail) — `viola_core::is_code_word` screens peer `refusal`/`detail` in both clients (`src/cmd/client.rs`, `crates/viola-mcp/src/result.rs`), `viola_channel::sanitised_fault` fixes exit 20's detail, `unknown` takes no hint; rows `contract/peer-reply-screening/*`, `contract/wrapper-fault-detail/*`, `decision/no-hint-for-unknown` — the viola-mcp survivor tests and any error-body refactor must not loosen them.

@history_item n=4 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document names its cause; the catch site's `internal-error` document; `revive --json`'s codes; how a frame verb reads no endpoint; the log's codes) — exit 21's closed `detail` causes, the catch site counting `--json` only before `--` and staying silent for `run`, `revive`'s start, `mcp`, `hook`, `pump_child` printing no document, `revive`'s strict-modes refusal writing no log line; rows `contract/exit-21-cause/*`, `contract/cli-catch-site/*`, `decision/no-json-after-child-held-terminal`, `contract/viola-revive-refusals/strict-modes-no-log` — behaviour-unchanged refactors (hook merge, links/snapshot merge) must keep `hook`'s silence and these codes.

@history_item n=5 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (six frame verbs' instance-state pair; run's/revive's start home-tree check; verify's home-tree and stamps refusals; a home under a regular file) — the strict-modes checks in `crates/viola-state/src/strict.rs` back the home-state / instance-state pairs at exit 1 / 21; bears on item 4 (the dead `check_present` in `strict.rs`: delete or wire) and on the one viola-made-home (0700) test helper, since tests on 0755 homes hit these refusals.

@history_item n=6 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the board's one rule on the home; socket-dir pair and `socket-dir-refused`; the wrapper's silent drop of a refused peer; `revive --json`'s eleventh code) — the home held to the read rule (a home another user may read refuses the whole board with the home-state pair), `instances/` to the write rule; this is why the 19 tests on 0755 homes reddened (Epoch 6) and what the shared 0700 home helper must satisfy; rows `contract/viola-list-board/strict-modes-scan`, `contract/exit-21-cause/socket-dir-refused`, `contract/wrapper-socket/refused-peer`.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (cli Platform narrowed to Linux; strict-modes readings without an OS arm; server check without the macOS carve-out; Windows VT enable retired; `.cmd`/`.bat` refusal retired; verify's relied count on every build) — the Linux-only target (row `decision/linux-terminal-platform`) is the precedent for item 10's dead `#[cfg(not(unix))]` constants in `crates/viola-e2e/src/harness/run.rs`; `strict.rs`'s Windows arm already left (item 4 reads the remainder); revive's ten codes and verify's five are pinned in `tests/cross_exit_causes.rs`; `LedgerRow::relied()` is 24 on every build, which the `test_cases.rs` / `cli_verify.rs` splits must keep.

@history_item n=8 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify prints one step line per relied ledger row, `MM` = `LedgerRow::relied()`'s count, summary n + m = `MM` (rows `contract/viola-verify-output/step-line`, `/summary`); splitting `crates/viola-agent-claude/src/ledger/test_cases.rs` and `tests/cli_verify.rs` must leave the row set and these counts unchanged.

@history_item n=9 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — `viola verify --help`'s static paragraph and the named unclean-fixture refusal (`<file> <code>`, closed codes, never the content); both are asserted in `tests/cli_verify.rs`, which this chunk splits (rows `contract/viola-verify-output/help`, `contract/viola-verify-refusals/unclean-fixture`).

@history_item n=10 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's interrupted pair (no step line, no summary, fixed `unable:` / `hint:`, exit 1), worded by the founder at 2026-10-20; held by `tests/cli_verify.rs` tests that must survive the split byte-identical (row `contract/viola-verify-refusals/interrupted`).

@history_item n=11 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — `revive`'s fixed-order start refusals with no path/pid/id, the fixed `unable: <text>` form extended to `run`/`revive`/`verify`, `revive` printing nothing once the child starts; rows `decision/start-refusals-show-no-path-or-pid`, `contract/cli-start-refusal/fixed-message`, `contract/viola-run-output/child-started` — the fake agent (`src/bin/viola-fake-agent.rs`, split here) drives these paths.

@history_item n=12 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink`'s result lines; `--json` documents and exit 20; the strict-modes instance-state pair; the server-verification cause over eight frame verbs; the `handoff:` help row) — the link/unlink outputs (`already linked`, the kept first `since`, printed names from parsed arguments) rest on the wrapper's link state; the `src/run/links.rs` ↔ `src/run/snapshot.rs` merge must keep them (rows `contract/viola-link-output/*`, `contract/viola-link-json/ok`, `contract/viola-unlink-json/ok`, `contract/help-output/verb-groups`).

@history_item n=13 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is a hidden verb writing only MCP frames on stdout (row `contract/mcp-verb/stdout`); the viola-mcp survivor tests (`lib.rs` WARN `parse-rejected`, `stdin.rs` buffer) and the mcp-tap test-or-exclude decision must keep stdout frame-only.

@history_item n=14 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the colour layer and its decision order (`NO_COLOR` present, `TERM=dumb`, non-TTY, `--json`, `viola run`) built in viola's own SGR module; rows `contract/cli-colour-decision-order/*` — refactors and test-helper moves must leave every plain-output verdict unchanged.

@history_item n=15 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap built without its `color` feature, held by `cli_output_plain` (row `decision/clap-without-colour-feature`); no dependency change in this chunk may bring a colour-reading dependency back.

@history_item n=16 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (brand, surface and bans scoped to the CLI) — the spinner / progress-bar / live-redraw ban scoped to a verb's output, the step counter the one progress pattern; rows `decision/no-spinners-in-verb-output`, `decision/one-progress-pattern` — the verify step counter tests this chunk moves.

@history_item n=17 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — message-mode escaping (`\xHH`, nothing stripped; `--json` serde-escaped), shared by `wait`, `last` and `link`'s `since`; rows `contract/cli-message-escaping/*` — the masking scanner over string literals must not alter test fixtures that assert these bytes.
