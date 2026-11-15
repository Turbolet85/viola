@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `viola verify`'s output structure (a failing-row run exits 1 and still writes the stamp; usage `viola verify [--record <DIR>] …`) and its fixed `unable:`/`hint:` refusal pairs, a dirty recording among them; the origin of the rows contract/viola-verify-output/failing-row and contract/viola-verify-output/record, which item 5's record-home naming rule and the `tests/cli_verify/` `--record` callers (runs.rs, stamps.rs) rest on.

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — the dirty-recording refusal became the named `unable: a recorded fixture is not clean: <file> <code>` with codes `home-path` · `absolute-path` · `username` · `email`, never the content, hint unchanged (`record with a viola home under your user home`); the source of contract/viola-verify-refusals/fixture-not-clean. Item 5's rule (no path component named `home` in a `--record` test home) exists because such a component trips this `home-path` scrub; the helper must keep a recorded home clean under this contract, not change it.

@history_item n=3 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — added verify's stopped-run form (Ctrl-C / TERM: `unable: verify was stopped before it finished` pair, stdout empty, no stamp, exit 1); the behaviour `cli_verify::verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` asserts — the case item 3's receipt-wait (`WITHIN`) times out on; widening the wait must not loosen what this form pins.

@history_item n=4 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count and stop on one OS) — `MM` is `LedgerRow::relied()`'s length on every build and the stop is `SIGINT` / `SIGTERM` unconditional; the Linux-only target bounds the tests this chunk touches (the socket path's `sun_path` bound is a Linux measurement). Its sibling sub-entries drop the `.cmd`/`.bat` pair from verify's exit-1 pairs and from revive's `--json` start codes.

@history_item n=5 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — `viola verify` refuses a non-private home, `ledger/`, `bin/`, `plugin/` or the stamps write with the home pair, exit 1, stdout empty, no step line, no summary, before its log opens; the source of contract/viola-verify-refusals/home-not-private. Any change to `tests/support/home.rs` (item 5's helper candidate) must keep test homes owner-only, or every `--record` case refuses here instead.

@history_item n=6 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — fixed revive's start preflight order (holder, logged session, `session-live` via one `claude agents --json` read, skipped under `--fork`, a failed read passing, then the recorded directory); the source of contract/viola-revive/preflight-order and contract/viola-revive/session-live-check, which `chaos_revive::revive_after_a_killed_wrapper_resumes_the_logged_session_in_the_recorded_directory` (item 2's ~5 s class) exercises; the slow-case margin must hold this sequence's wall time.

@history_item n=7 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — created the `viola revive` output structure: a passing start prints nothing while the child holds the terminal (passthrough like `viola run`), its refusal pairs exit 1; the origin of contract/cli-verbs/revive and contract/cli-verbs/run's passthrough, the behaviour the ~5 s chaos_revive / cli_instance_state cases of item 2 wait on.

@history_item n=8 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — `[RB] read back …` prints when the send is confirmed, by the matching `prompt-submitted` or, for `/clear` on a verified CLI, its new-session post-condition; the source of contract/viola-send-output/read-back and read-back-clear, the receipts whose waits share `tests/support/watch.rs`'s `WITHIN` bound (item 3, the M2 «suspected shape» of shared `fake::wait_for` callers).

@history_item n=9 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` read back by a `session-start` of cause `compact` with the session's own id inside its own 40 s PROVISIONAL window, else `not-delivered` / `no-prompt-submitted`; the source of contract/viola-send-output/read-back-compact. A product window of 40 s sits beside the test-side receipt wait (7 s) and the mutants kill (10 s): item 3 retunes the test bound only, never this window.

@history_item n=10 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (226), each verified against the code; every picked row's handle (contract/cli-verbs/*, contract/viola-verify-*, contract/viola-revive/*, contract/viola-send-output/*) dates from this migration, so a row's wording is the code's as of then and this test-only chunk amends none.
