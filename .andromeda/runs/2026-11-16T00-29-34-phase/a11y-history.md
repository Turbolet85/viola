@history_item n=1 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — the MCP wait cap: §8 Timeout extensions says the MCP `wait` tool clamps `timeout_ms` to `WAIT_CAP` 50 000 ms, which is also its default (`viola_mcp::wait_timeout`). An MCP `wait` with no timeout is answered `timed-out` after 50 s, and CLI `viola wait` times out only on a timeout the caller passes. This is the origin of rows contract/mcp-wait-tool/timeout-ms and contract/viola-wait/timed-out. A wake-condition change to the MCP `wait` tool (`crates/viola-mcp/src/tools.rs`) must keep the cap and the timed-out answer, and the bound must stay on the driver only.

@history_item n=2 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the dialog hold became configurable (`dialog_hold_s`, 60–3600, default 60). When the hold ends, the wrapper appends one `dialog-expired` line, and that line wakes `wait`. This is the origin of row contract/event-line/dialog-expired: `dialog-expired` is an existing wake kind that the documented wake/log-only split must keep. The hold bounds the driver and never the human.

@history_item n=3 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — the first dialog deadline (60 s, later superseded by the configurable hold). It set the precedent that a time bound falls on the driver's path, with an expired dialog rendered for the human in the TUI. This chunk's turn-end question wake rests on that dialog/answer critical path (row decision/critical-path-dialog-answer).

@history_item n=4 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ decision/dialog-hold-bounds-the-driver`: state changed from provisional to standing after the live measurement. The hold whose end appends the wake line `dialog-expired` is now ratified and is not this chunk's to re-open.

@history_item n=5 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the dialog hold bounds owned by 0.2.0's first entry) — `DIALOG_HOLD_MIN` / `DIALOG_HOLD_MAX` stay PROVISIONAL, owned by 0.2.0's first entry. This applies if the wake-on-`dialog-expired` cases or the fake agent's turn scripts lean on the hold's bounds.

@history_item n=6 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/time-bounds-fall-on-the-driver` and `~ decision/time-bounds-never-on-human` dropped the budget pause from the list of time bounds. These are the closed lists of every bound viola holds (the wait timeout and cap among them). Any new or changed wait bound, or a re-arm, must be listed there and must fall on the driver.

@history_item n=7 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — `~ decision/time-bounds-never-on-human` and `~ decision/time-bounds-fall-on-the-driver` dropped the status-line deadline, which gives the rows their current wording (wait timeout and cap, dialog hold, readiness gate, confirm windows, agents read). Precedent: a chunk that adds or removes a time bound amends both twin rows.

@history_item n=8 marker=2026-10-10-session-links
**2026-10-10-session-links** — `wait` got a case of its own under a closed gate (`path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone`). This is precedent for driver-side `wait` readings that type no human key and read no terminal stream. The budget gate behind it was later removed (2026-11-14-budget-gate-removed).

@history_item n=9 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — §4 P6 records that `viola wait` and `viola --help` carry no ESC and no lone CR. This is the static-linear-output rule (rows decision/linear-static-cli-output, contract/viola-wait/waiting-line) that any change to `wait`'s human output (`src/human/wait.rs`) must keep.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the cli SGR and escape checks; the a11y cases' CI home) — `viola wait`'s no-ESC/no-lone-CR check is now ungated on ubuntu-latest. The cli and tui cases run inside the `test` job's single `coverage` suite on the one `ubuntu-latest` leg, whose `gate --require coverage,doctest` is the verdict (row decision/a11y-cases-inside-coverage-suite). That job is where the carried `path1_run_start_one_home_in_order` TIMEOUT at the `ci` profile's 20 s kill was read.

@history_item n=11 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the a11y cases became nextest cases in the `coverage` suite. The §10 failure condition became a `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`), and the zero-flakiness parallel bans `#[ignore]`. This is the origin of rows decision/a11y-gate-breach-fails-build and decision/zero-flakiness. The CI-red fix (a shared or copied stamped home, or a resized `ci` kill) may not skip, ignore or retry the timed-out case.

@history_item n=12 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — CLI output-discipline evidence reports under CI's single instrumented `coverage` suite, gated by `gate --require coverage,doctest`. This is the base of rows decision/gate-requires-coverage-doctest and decision/ci-test-gate-no-skips, which bound how the `ci` profile and `tests/support/verify.rs` fix may change.

@history_item n=13 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — `verify`'s interrupted pair: if `verify` is stopped before it finishes, it prints no step line and no summary. It prints only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI`, then exits 1, and the stop path adds no time bound. This is the origin of row contract/viola-verify/interrupted-pair. It bears on the CI-red item: a stamped-home fixture whose `viola verify` is bounded or killed must not leave this pair's contract or the step-line row (contract/viola-verify/step-line).

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a lost wrapper and a stale holder; exit 21's `--json` document names its cause) — a frame verb such as `wait` whose wrapper is lost with the call open reads `endpoint-lost`. Against a stale instance it reads `instance-dead`. Under `--json`, exit 21 gives one document whose `detail` is a closed cause word. A parked `wait` that ends on something other than a wake kind must keep these refusal shapes.

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the `--json` failure documents' witness; `--json`'s one document and its exit-2 exception) — each `--json` failure is one document with a literal exit, empty stderr, no ESC byte and a clean body (`cli_error_documents_hold_no_path_or_upstream_text`). The one-document rule has an exception only for the `answer` and `send` exit-2 paths. These are the origins of rows contract/cli-json-failure/document, decision/json-failure-documents-are-clean, decision/json-one-document-no-hint and contract/cli-refusal/json, which `wait --json` stays inside.

@history_item n=16 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the plugin against an impostor wrapper writes no human-facing byte) — the first not-assertable entity for the plugin and MCP frame tools, `wait` among them. Precedent for row decision/no-assertions-for-plugin-and-ipc: the MCP `wait` tool's wake behaviour is proved by handler tests, not by a11y assertions.

@history_item n=17 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the cli output-discipline checks on std::process::Command and literal pins; the §8 verification lines pinned by literal; the cli tooling in the a11y testing contracts) — trycmd and assert_cmd were replaced by literal pins over `std::process::Command`. Every fixed CLI string is pinned by a literal, so a copy change fails CI on purpose. This is the origin of rows decision/fixed-cli-strings-pinned, decision/fixed-cli-strings-pinned-by-literal, decision/cli-a11y-by-literal-tests and the c0-c1 / linear-output guards. Any new or changed `wait` string or waiting line takes a literal pin.

@history_item n=18 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the row escape (ESC, C1, DEL, `\n`, `\t` → `\xHH`) for `viola list` rows. It is the basis of the list-row exception in rows decision/control-character-escaping and decision/control-bytes-escaped, while `wait` and `last` text keep `\n` and `\t` raw. Any text `wait` returns from a woken line must follow the wait/last rule and not the list-row one.

@history_item n=19 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the first live reading of the takeover clause, recorded as supplemental and never gating (row decision/live-readings-never-gate). Precedent for this chunk's boundary that a live `claude` start for the wake clause would be a supplemental reading, with the gating proof over the fake agent.

@history_item n=20 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the second live takeover reading on a real `claude` session, again supplemental and never gating. This confirms that live readings of the product never become CI gates, which bears on any wake-condition reading on claude 2.1.325.

@history_item n=21 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — per-chunk and CI mutation gates were removed. Output-discipline invariants in crate tests meet mutation only at the epoch-boundary audit. This matches the chunk's «no mutation run» boundary, where each new guard is proved by a one-off remove-the-guard control instead.

@history_item n=22 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows. Every picked row (time bounds, wait contracts, output discipline, CI gate) dates from this conversion, verified against the code at 8834195b0683. Amendments to them go by `+`/`~`/`−` on a row handle.
