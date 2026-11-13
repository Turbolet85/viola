@history_item n=1 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — set the send readiness gate: a verified `viola send` waits for the input box at most `GATE_MAX_WAIT` (8.5 s) and is then refused `not-delivered` / `input-not-ready` with nothing typed; a human key during the wait takes the wheel and the send is refused `human-typing` (`send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, sequenced on records, never a timer). Why it matters: this is the run pump's input-box readiness, the source of rows contract/send-refusal/input-not-ready, decision/human-key-takes-the-wheel and the gate in the time-bounds rows; if `SIGNATURES.input_box` changes, this gate shares the literal and its boundary case must stay green (scope keeps the gate Out unless research shows the same literal and failure).

@history_item n=2 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — fixed verify's interrupted pair: a `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI` last on stderr, exit 1, pinned by the stop table in `tests/cli_verify.rs`; the stop path adds no time bound. Why it matters: rows contract/interrupted-verify/* and contract/viola-verify/interrupted-pair; any rework of the typed run's box wait must keep the stop path ending its waits at once and the pair byte-identical.

@history_item n=3 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — named the confirm windows as driver-only bounds: `CONFIRM_WINDOW_FALLBACK` 10 s and a verified `/compact` send's `COMPACT_CONFIRM_WINDOW` 40 s (PROVISIONAL), unconfirmed → `not-delivered` / `no-prompt-submitted`, no key blocked. Why it matters: the typed run's `/compact` paste and its ledger row are in this chunk's scope; any new or changed wait in the typed run (the box wait) must be listed the same way, falling on the driver or viola's child, never the human (decision/time-bounds-never-on-human).

@history_item n=4 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** (the tui passthrough boundary after a contained vt100 feed panic) — added the boundary entity: after a contained feed panic the driver's `send` is refused `input-not-ready` (exit 13) while a human key still reaches the child, and none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` reaches the outer terminal (`tests/chaos_feed_panic.rs`). Why it matters: rows contract/tui-passthrough/after-feed-panic and vt100-feed-panic; a change to how the screen is read for the input box touches the same vt100 feed and readiness verdict this case witnesses.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the tui boundary on openpty alone; the a11y cases' CI home) — the TUI is hosted in openpty only; the three boundary clauses (focus / mouse / resize, zero viola bytes compared byte for byte with an unwrapped fake-agent run, keystrokes) are each a nextest case on `ubuntu-latest`, inside the `test` job's `coverage` suite gated by `gate --require coverage,doctest`. Why it matters: fake-agent screens and fixtures this chunk edits feed the zero-viola-bytes comparison; the CI home is where the boundary and CLI invariants must stay green (decision/cli-invariants-pass-in-ci, decision/tui-boundary-assertions-only).

@history_item n=6 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli output-discipline and TUI boundary cases run as nextest cases in the `coverage` suite under `gate --require coverage,doctest`; failure is `suite-failed` / `suite-skipped` / `suite-missing`; `#[ignore]` banned (zero-flakiness parallel). Why it matters: rows decision/gate-requires-coverage-doctest, decision/ci-test-gate-no-skips and decision/zero-flakiness — new fixture-replay cases for the footer notice may not be skipped or ignored.

@history_item n=7 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — `ci.yml` became the one push/PR workflow beside the scheduled `nightly.yml` weekly `cargo deny check advisories`, which carries no a11y step. Why it matters: this chunk's supply-chain item (rmcp / rustix advisories, `deny.toml`) closes reds in that advisory gate; the a11y leg stays in `ci.yml` and is untouched by it.

@history_item n=8 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the outer-PTY driver on portable-pty =0.9.0) — the tui boundary driver reads the workspace's exact `=0.9.0` pin; a11y inherits that driver as-is and never re-pins it. Why it matters: the supply-chain item edits `Cargo.toml` / `Cargo.lock`; any bump must leave the outer-PTY driver pin to the workspace, not move it on a11y's account.

@history_item n=9 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the §8 verification lines pinned by literal; the cli output-discipline checks on std::process::Command) — every fixed CLI string is pinned by literal in root and unit tests and the refusal layout, typed exit codes and hint-free `--json` held by literal asserts over `std::process::Command` (was trycmd / assert_cmd). Why it matters: rows decision/fixed-cli-strings-pinned*, decision/refusal-layout-literal-pins, contract/viola-verify/step-line — any verify step line or refusal text this chunk touches must change with its literal pin, on purpose.

@history_item n=10 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the strict-modes refusals' text and witnesses) — `viola verify` over a home whose tree (home, `ledger/`, `bin/`, `plugin/`) fails the check is refused exit 1 before its log, no step line or summary, the home-state pair with `hint:` last; `update_stamps`'s own `ledger/` check gives the same pair (`verify_refuses_a_widened_ledger_before_any_step`, `verify_under_a_regular_file_refuses_the_home`). Why it matters: the proof writes `ledger/stamps.json` through verify itself; the stamps path keeps this refusal and its pinned pair.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (strict-modes refusals, the Windows ACL and NotFound arms retired) — `verify_under_a_regular_file_refuses_the_home`'s oracle is the home-state pair alone (was per OS); strict-modes cases run on ubuntu-latest. Why it matters: the current form of verify's refusal witnesses in `tests/cli_verify.rs`, beside which any new verify case lands.

@history_item n=12 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — recorded the first live reading of the takeover clause on Linux, supplemental and never gating. Why it matters: precedent for decision/live-readings-never-gate — this chunk's live `viola verify` proof run on the newest CLI is an operator step, and its reading must not become a CI gate.

@history_item n=13 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading, on the product binary over a live `claude` 2.1.296 session, the founder's own key; supplemental, never gating. Why it matters: 2.1.296 is the CLI this chunk never re-verifies; live readings on a real `claude` stay operator-held and outside the gate, as this chunk's named live-start step requires.

@history_item n=14 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the census of error writers and the body check) — `error_bodies_each_name_a_writer_and_a_witness_that_stand` and `product_error_writers_are_each_named` (`tests/contract_error_bodies.rs`) hold every error body to a writer and a witness. Why it matters: row decision/error-bodies-have-writer-and-witness; a new or reworded verify failure path in the typed run must be named in that census.

@history_item n=15 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants (P4, P6) meet mutation only at the epoch-boundary audit (`run --mutants`), the per-chunk gate removed. Why it matters: consistent with the scope's Out «any mutation run»; this chunk owes no mutation pass on decision/cli-invariants-pass-in-ci.

@history_item n=16 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (246, verified against the code at 8834195b0683, fates by the founder 2026-11-12). Why it matters: every picked row (e.g. decision/no-screen-content-parsing — the TUI screen parsed only for compiled input-box and modal signatures) is that conversion's row; changing `SIGNATURES.input_box` must stay within «compiled signatures», never wider screen-content parsing.
