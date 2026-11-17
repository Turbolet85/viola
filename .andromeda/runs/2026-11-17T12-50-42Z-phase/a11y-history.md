@history_item n=1 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — §9 Platform: `ci.yml` became "the one push/PR workflow" when the chunk added `nightly.yml`, the weekly `cargo deny check advisories` run, which carries no a11y step. Matters here because part A re-measures that weekly advisories run along with the root `supply-chain` job. A `deny.toml` ignore changes advisories only and leaves the a11y leg in `ci.yml` untouched.

@history_item n=2 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — the CLI output-discipline tests report under `nextest-integration` / `nextest-e2e` locally and, in CI, inside the `test` job's single instrumented `coverage` suite gated by `gate --require coverage,doctest`. This is the origin of the picked rows decision/a11y-inside-existing-suites and decision/a11y-cases-inside-coverage-suite. The re-tied `tests/cli.rs` / `tests/harness_lifecycle.rs` boots and the new kill-the-stamping-verify test stay ordinary cases of those suites.

@history_item n=3 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants meet mutation only at the epoch-boundary audit (`run --mutants`), since the founder removed the per-chunk and CI mutation gates. This agrees with the scope's "no mutation run" boundary: the chunk owes no per-chunk mutation proof for the a11y invariants.

@history_item n=4 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line and no summary, only the `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` pair last on stderr. It exits 1, and the pair is pinned by the stop table in `tests/cli_verify.rs`. Its stop path adds no time bound: its waits end at the stop, and its kill and pump join end at once. This entry is the source of the picked rows contract/interrupted-verify/exit, contract/interrupted-verify/stderr and contract/viola-verify/interrupted-pair. Part B's test kills the tied stamping `viola verify`, and that test must not contradict them: a SIGKILL is not the Ctrl-C/TERM stop path, so the pair is not expected there. Any TERM-based witness reads the pair.

@history_item n=5 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch superseded the provisional mark on the interrupted pair's wording (2026-10-13-interrupted-verify-cleanup): the wording is now his. Any test this chunk adds that reads verify's interrupted pair pins the founder's own text and must not re-word it.

@history_item n=6 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — the browser a11y leg left. The cli / tui cases run as nextest cases in the `test` job's `coverage` suite, and results land in `suites[].failures[]` with no a11y artifact, stage, budget or second driver. §10's failure condition became a `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`), and the zero-flakiness parallel bans `#[ignore]`. §11 bans a new `suite` value or a separate a11y driver outside `scripts/agent-run.* run`. This entry is the source of the picked rows decision/no-own-a11y-artifact, decision/no-separate-a11y-driver, decision/a11y-gate-breach-fails-build and decision/zero-flakiness. The chunk's new kill tests may not be `#[ignore]`d, skipped or retried, and they need no a11y suite of their own.

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the a11y cases' CI home, the one ubuntu-latest leg) — §9's matrix became `[ubuntu-latest]`. The cli / tui cases run in the `coverage` suite on that leg, and its `gate --require coverage,doctest` is the verdict. The runner is `scripts/agent-run.sh run` alone. The chunk's pre-CI acceptance (the whole run green) is read on that one leg.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Linux-only platform line) — "Platform (every surface): Linux only — the core on any current distribution, its CI on `ubuntu-latest`, and the 0.2.0 window features on Hyprland". This is the source of the picked row decision/linux-only. The tie / census and the SIGKILL proofs need only the Linux process model, and no other OS arm is owed.

@history_item n=9 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the cli output-discipline checks on std::process::Command and literal pins) — the cli checks drive the `viola` binary through `std::process::Command` and pin each fixed string by a literal in its test file; trycmd and assert_cmd were retired. Any output assertion the re-tied `tests/cli.rs` boots or the kill-the-verify test makes follows that form.

@history_item n=10 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows, each verified against the code. This is the origin of every picked row's current handle and text, so the chunk plans against the rows, not the pre-row section prose.

@history_item n=11 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — ~ decision/time-bounds-fall-on-the-driver and ~ decision/time-bounds-never-on-human: the budget pause left both rows' lists of bounds. Matters as the last narrowing of the picked time-bound rows. The chunk adds test-side bounds only (the 20 s lint, `--timeout S`), none on a human key, so neither row moves.

@history_item n=12 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver: the status-line deadline left both lists, which now name the wait cap, dialog hold, readiness gate, confirm windows and agents read. Those are the picked rows' current text. A new product time bound in this chunk would have to be added to both. The chunk is planned to add none.

@history_item n=13 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — ~ decision/no-screen-content-parsing gained the test-only standing-live-rig exception, which keeps raw frames local and gitignored. That rig has its own isolation and process hygiene, and its successor built the tie and census this chunk extends. Matters as the nearest precedent: this chunk starts no live `claude`, so it uses neither the rig nor its exception.
