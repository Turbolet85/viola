@history_item n=1 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — ~ decision/no-screen-content-parsing gained its except: the test-only standing live rig reads the trust selector, the input box and modals as closed words, its raw frames kept local and gitignored. This is the rig the scope fences off (`viola-live-rig` owns only its own processes and is not reworked here). Its census is the precedent the «killed» proof copies. Any process census this chunk adds must read no vendor screen content beyond that exception.

@history_item n=2 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — ~ decision/no-screen-content-parsing: the screen is parsed only for compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row only for its opening char. This bounds what a fake-agent screen change in this chunk may be matched against. A change to the fake agent's screens also puts `cli_verify` in the gate block (directive).

@history_item n=3 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — moved the CLI output-discipline evidence into the CI `test` job's single instrumented `coverage` suite under `gate --require coverage,doctest`, which is the origin of decision/a11y-cases-inside-coverage-suite. The CI red this chunk must explain (26 of 3484 failed in that coverage run, shared stamped home absent) is read against that gate.

@history_item n=4 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the cli and tui cases run as nextest cases inside the `coverage` suite with no a11y stage, driver or artifact of their own. The zero-flakiness parallel bans `#[ignore]`, and §11 bans a `suite` value or a separate a11y driver outside `scripts/agent-run.* run`. So the kill harness and census proof must live inside the existing suites, never as an ignored or separately driven case.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the a11y cases' CI home, the one ubuntu-latest leg; the tui boundary on openpty alone) — the matrix became `[ubuntu-latest]` and the runner became `scripts/agent-run.sh run` alone. The TUI is hosted in openpty, and each of the three boundary clauses is its own nextest case on that leg. Those are the decision/boundary-tests-sequenced-on-records, decision/a11y-ci-github-actions and decision/linux-only rows this chunk's wrapper-lifetime work must not break. The acceptance (a CI run on an exact cache hit reading green) is read on that one leg.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (strict-modes refusals, the Windows ACL and NotFound arms retired; the Linux-only platform line) — a home under a regular file is now a strict-modes refusal on every run (verify's home-state pair, exit 1), and the per-OS oracle is gone. The strict-modes cases run on ubuntu-latest. This bears on rebuilding the shared stamped home when its marker stands but its home is absent: a rebuilt home must pass contract/start-refusal/home-strict-modes or every booted `run` and `verify` refuses.

@history_item n=7 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the CLI verbs' strict-modes refusals as scope entities; the strict-modes refusals' text; witnesses and pinned pairs) — created the `run` / revive start / `verify` home strict-modes refusal entity (exit 1 before any log, the home-state pair, no child, verify printing no step line), pinned by `HOME_PAIR` / `INSTANCE_PAIR`. That entity is the row contract/start-refusal/home-strict-modes. The stamped-home fix in `tests/support/home.rs` and `harness/boot.rs` rests on homes that satisfy it.

@history_item n=8 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the start) — `viola run` and a revive start refuse a socket directory that fails its check: exit 1 before any snapshot, bind or child, with the directory left as found. That is the origin of decision/socket-dir-refusal-before-any-frame. Test sockets go under the short `/tmp` root (directive), and a killed test's leftover socket directories must not be widened or cleaned in a way that trips this refusal for later tests.

@history_item n=9 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — set the tui wheel boundary: focus, mouse and resize reports and terminal replies are non-editing, and case (3) is sequenced step-wise on receipts, with the wheel probed by an `answer`. This is the source of decision/wheel-ignores-terminal-events, one of the outer-PTY cases that boots a wrapper. Its booted wrapper and fake agent fall under the scope's «does not outlive its test».

@history_item n=10 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** — added the outer-PTY case `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, sequenced on the wrapper's `channel-request` line and the `wheel` record, never a timer. This is a precedent for decision/boundary-tests-sequenced-on-records and decision/time-bounds-never-on-human. Any teardown or kill step added to boundary tests must keep record sequencing, not sleeps.

@history_item n=11 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the outer-PTY driver on portable-pty =0.9.0) — the tui boundary driver moved to portable-pty `=0.9.0`. a11y inherits it as-is and never re-pins it (decision/tui-boundary-driver-pin, decision/portable-pty-outer-pty-driver). A fixture change that ends the wrapper when its PTY or parent is gone must work on that driver without re-pinning it.

@history_item n=12 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the cli output-discipline checks on std::process::Command and literal pins; the cli tooling in the a11y testing contracts) — the cli cases drive the `viola` binary through `std::process::Command`, with fixed strings pinned by literal. This is the guard named on decision/no-sgr-without-tty. The way tests spawn the binary, which this chunk's wrapper reaping wraps, is that harness.

@history_item n=13 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — recorded the first live reading of the takeover clause as supplemental and never gating, which is the origin of decision/live-readings-never-gate. The scope rules out live `claude` starts, so no proof here may lean on a live reading.

@history_item n=14 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading, again supplemental and never gating. It confirms decision/live-readings-never-gate, so this chunk's acceptance must rest on the gated fixture cases only.

@history_item n=15 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the same TUI passthrough as `viola run` (contract/tui-passthrough/entry) and writes no line of its own while the child holds the terminal (`tests/chaos_revive.rs`). Revive-started wrappers in tests are booted wrappers too, so they fall under this chunk's census.

@history_item n=16 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the 5 s `claude agents --json` read, with a kill bound that falls on viola's own child and fails open. This is the origin of decision/claude-agents-read-fails-open and of the agents read in the time-bounds rows. It is one more child process a test can leave behind, and it falls under the census.

@history_item n=17 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (8. Cognitive Accessibility) — ~ decision/time-bounds-fall-on-the-driver and ~ decision/time-bounds-never-on-human both dropped the budget pause from their lists of bounds. These rows bind this chunk: any new bound it adds (kill deadline, census wait) must fall on the test or driver, never on a human key. If it is a product bound, it is added to those lists.

@history_item n=18 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (8. Cognitive Accessibility) — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver dropped the status-line deadline, which gives their current wording. This is a precedent that a bound added to or removed from the product is amended into both twin rows together.

@history_item n=19 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** (P6's stale-row dim witness under the outer PTY) — `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal` boots a wrapper and freezes it under the `Frozen` guard (`tests/support/frozen.rs`). A frozen wrapper whose test is killed is exactly a booted process no destructor ends, so the guard must be covered by this chunk's reaping and census.

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the cli SGR and escape checks without their Windows arms, the stale-row witness ungated) — the stale-row case under `Frozen` now runs ungated on ubuntu-latest in `coverage`. Its frozen booted wrapper is therefore in every CI run this chunk's acceptance reads.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the three-runner greens re-worded as dated readings) — a CI green is recorded as a dated reading at a named run, never left as current. This is the precedent for stating the CI red at ci#44048204364 and the re-run at ci#44049652505 as dated readings, not as closed or green.

@history_item n=22 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the a11y output-discipline invariants meet mutation only at the epoch-boundary `run --mutants` audit. That audit is the run whose timed-out tests left the spinning wrappers this chunk exists for. No mutation run is part of this chunk.

@history_item n=23 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — `ci.yml` is the one push/PR workflow, and the scheduled `nightly.yml` carries no a11y step (decision/a11y-ci-github-actions). The CI red and its exact-cache-hit acceptance belong to `ci.yml`'s `test` job.

@history_item n=24 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows, each verified against the code. Every picked row's current wording dates from this conversion or a later `~` entry, and an amendment from this chunk is written by row handle.
