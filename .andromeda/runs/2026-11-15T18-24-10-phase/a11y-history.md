@history_item n=1 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** (CLI output-discipline evidence reports under the CI `coverage` suite) — moved the a11y cli cases into the test job's single instrumented `coverage` suite gated by `gate --require coverage,doctest`, failures surfacing only as nextest failures. This is the origin of decision/a11y-cases-inside-coverage-suite, decision/cli-invariants-pass-in-ci and decision/a11y-failures-are-nextest-failures: the green coverage suite this chunk makes reachable (item 3, the receipt wait red twice in the audit's coverage run) is the a11y verdict itself.

@history_item n=2 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — P4/P6 output-discipline invariants held in crate tests are covered by the epoch-boundary mutation audit (`run --mutants`), not a per-chunk or CI mutation gate (founder ruling 2026-09-28). Origin of decision/mutation-audit-covers-crate-invariants: this chunk makes the `viola` mutation baseline reachable for that audit and must run no mutation itself.

@history_item n=3 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli / tui a11y cases became plain nextest cases in the `coverage` suite with no stage, budget, suite value or driver of their own; §10's failure condition is a `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`) and the zero-flakiness parallel bans `#[ignore]`. Origin of decision/a11y-gate-breach-fails-build, decision/no-a11y-performance-budget, decision/a11y-runs-in-existing-test-suites and the no-`#[ignore]` half of decision/zero-flakiness: a slow case may not be skipped or ignored to green the suite; the fix must be margins or root cause.

@history_item n=4 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the a11y cases' CI home, the one ubuntu-latest leg) — the matrix became `[ubuntu-latest]`, the cli / tui cases in that leg's `coverage` suite whose `gate --require coverage,doctest` is the verdict, runner `scripts/agent-run.sh run` alone. The leg this chunk's green coverage suite is judged on.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Linux-only platform line) — added «Platform (every surface): Linux only», the source of decision/linux-only; bounds item 1's `sun_path` (107 B, Linux) computation to Linux only.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (server-verify and socket-directory refusals; strict-modes refusals) — retired the macOS/off-Unix carve-outs: the hook socket-directory witness and server-identity cases run ungated on ubuntu-latest. Bears on contract/hook-call/socket-dir-refused and contract/hook-call/server-verify-failed, whose fail-open rows are read in `tests/hook_fail_open.rs`, the file item 1's socket name lives in.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the tui boundary on openpty alone) — each of the three boundary clauses is its own nextest case on `ubuntu-latest`; the keyboard-boundary cases' shape (decision/boundary-tests-sequenced-on-records) a test-only change must not merge or re-time.

@history_item n=8 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the plugin against an impostor wrapper writes no human-facing byte) — added the hook entity behind contract/hook-call/server-verify-failed (no frame, fail open, exit 0, empty streams, under 1.0 s), witnessed by the impostor rows of `tests/hook_fail_open.rs`. Item 1 rewrites that file's test channel socket name: the impostor rows and their 1.0 s bound must stay intact.

@history_item n=9 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the plugin against a strict-modes-refused instance, not assertable) — added the entity behind contract/hook-call/strict-modes-failed (exit 0 at once, no `config.json`/stdin read, no frame, no line; a `VIOLA_DIR` naming no instance directory writes nothing), witnessed by `hook_refused_by_the_instance_check_fails_open_and_writes_nothing`. A neighbour case in the hook fail-open tests item 1 touches.

@history_item n=10 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (hook and MCP on a refused socket directory: not-assertable) — added the entity behind contract/hook-call/socket-dir-refused (fail open, one `hook-decision` WARN with `detail` `socket-dir-refused`, `security_negative_a_hook_fails_open_on_the_socket_directory`). Socket-directory checks sit on the same endpoint path family item 1 shortens; a shorter test socket name must keep these rows' witnesses reading as before.

@history_item n=11 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the client euid refusal's limit of proof) — recorded that a seam's wiring is read only by forced refusals in remove-the-guard runs no committed test repeats, so an always-admit mutant survives by construction. Precedent for item 4: a survivor is either killed by a test failing on the mutated site (its remove-the-guard control) or stated, with its reason, as outside the tier.

@history_item n=12 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — the outer-PTY gate-wait case is sequenced on the wrapper's `channel-request` line and the `wheel` record, never a timer, its stated limit left to the unit tier; the readiness gate's 8.5 s bound falls on the driver only. Precedent for decision/boundary-tests-sequenced-on-records and the driver-only time bounds: item 3's receipt wait (`WITHIN`) and item 2's kill margin are test-harness timeouts on record waits, not a sequencing timer, and must not become a human-facing bound.

@history_item n=13 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — recorded the first live takeover reading as supplemental and never gating. Origin of decision/live-readings-never-gate: this chunk (no live `claude` start, cap 0) leaves the live readings out of its acceptance.

@history_item n=14 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading, again supplemental and never gating; keeps decision/live-readings-never-gate standing with no live step owed by this test-only chunk.

@history_item n=15 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (8. Cognitive Accessibility: decision/time-bounds-fall-on-the-driver and decision/time-bounds-never-on-human) — `~ decision/time-bounds-fall-on-the-driver` and `~ decision/time-bounds-never-on-human` dropped the budget pause from the listed bounds. The rows' current lists are what a test-only chunk may not extend: changing test timeouts adds no product bound.

@history_item n=16 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (8. Cognitive Accessibility: the two time-bounds rows) — `~ decision/time-bounds-never-on-human` and `~ decision/time-bounds-fall-on-the-driver` dropped the status-line deadline, leaving wait cap, dialog hold, readiness gate, confirm windows and agents read; the last edit to the picked time-bounds rows.

@history_item n=17 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the cli tooling in the a11y testing contracts) — the cli cases drive `viola` through `std::process::Command` with literal pins in the `nextest-integration` / `nextest-e2e` suites and the CI `coverage` suite on ubuntu-latest; the a11y tooling install reads none. Confirms decision/a11y-runs-in-existing-test-suites: the nextest profile item 2 retunes (`[profile.mutants]`) carries these cases, with no a11y profile of their own.
