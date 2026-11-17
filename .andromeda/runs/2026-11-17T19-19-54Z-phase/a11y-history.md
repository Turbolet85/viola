@history_item n=1 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — ~ decision/no-screen-content-parsing gained its except: the test-only standing live rig reads the trust selector's place, the input box and modal dialogs as closed words only, raw frames local and gitignored. This chunk's S0 runs a live `viola verify` on that rig and its sixth run reads whether the trust dialog shows, so any new screen read must stay inside that exception or the compiled signatures.

@history_item n=2 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — ~ decision/no-screen-content-parsing widened the compiled signatures to footer literals, the input box's rule and prompt rows (opening char only) and modal literals, as a boundary widening the founder ratified. The sixth run's "first settled frame is no trust dialog" post-condition has to read the screen through those compiled modal/input-box signatures only, never the text after them.

@history_item n=3 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — A `verify` stopped by Ctrl-C/TERM prints no step line and no summary, only the `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` pair, exits 1, pinned by the stop table in `tests/cli_verify.rs`; the stop path adds no time bound. The sixth run's "remove its own record on every exit path" has to land inside this stop path without changing the pair, the exit or the no-step-line rule (contract/interrupted-verify/*, contract/viola-verify/interrupted-pair).

@history_item n=4 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — The interrupted pair's wording was made the founder's own (its provisional mark superseded). The pair's text is fixed; the record clean-up on a stop must not reword or add to it.

@history_item n=5 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (strict-modes refusals as scope entities; their text; witnesses and pinned pairs; unstattable home) — `viola verify` over a home failing strict modes refuses before its log opens: exit 1, the home-state pair, `hint:` last, no step line, no summary (`refuse_home`, `update_stamps`'s `StampsError::StrictModes`); witnesses `verify_refuses_a_widened_ledger_before_any_step` and `verify_under_a_regular_file_refuses_the_home` in `tests/cli_verify.rs`, pair pinned by literal. Rows contract/start-refusal/home-strict-modes, decision/strict-modes-refusal-before-any-write and decision/unstattable-home-is-strict-modes-refusal come from here: a sixth run, the record writer, and a 23rd ledger row must not write anything (the trust record included) before that check.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (strict-modes Windows arms retired; a11y cases' CI home; the Linux-only platform line; tui boundary on openpty; cli SGR checks without Windows arms) — `verify_under_a_regular_file_refuses_the_home`'s oracle is the home-state pair alone; every a11y case runs in the `coverage` suite on the one `ubuntu-latest` leg under `gate --require coverage,doctest`; the platform is Linux only; the TUI is hosted on openpty alone; the colour decision has no Windows VT branch. The writer's file I/O (lock, atomic replace, read-back) and the new `cli_verify` cases need a Linux path only, with nothing skipped.

@history_item n=7 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the start) — `viola run` and a `viola revive` start over a failing socket directory refuse with exit 1 before any snapshot, bind or child; the fixed-message `unable:` / `hint:` pair; the directory left as found (contract/start-refusal/socket-dir, decision/socket-dir-refusal-before-any-frame). It is the precedent that a refusal leaves the file system as found: verify's record writer has to give the same guarantee for the trust record on a refused or failed write.

@history_item n=8 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — Records the first live reading of the takeover clause as supplemental and never gating (decision/live-readings-never-gate). This chunk's live S0 `viola verify` stamp is the same kind of reading: it informs and never gates CI.

@history_item n=9 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — The second live takeover reading, on a real `claude` with the founder's own key, also supplemental and never gating. It is the precedent for a live reading under the founder's cap with no agent typing a key, which matches S0's `keys: none`.

@history_item n=10 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (cli checks on std::process::Command and literal pins; the §8 verification lines; the cli tooling contracts) — Output discipline is checked by `std::process::Command` with each fixed string pinned by a literal (was trycmd/assert_cmd); every fixed CLI string, hints and result lines included, is pinned (decision/fixed-cli-strings-pinned*, decision/cli-a11y-by-literal-tests). The sixth run's `[NN/NN] … pass|fail` step line and the new row's closed words need literal pins, and the step-count sweep has to move every pinned spelling of the count.

@history_item n=11 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — The cli/tui a11y cases became ordinary nextest cases in the `test` job's `coverage` suite, with no a11y stage, driver or suite value of their own; a `gate --require coverage,doctest` breach fails the build; `#[ignore]` is banned. The new verify-probe cases join that suite, with none ignored and no retries (decision/a11y-runs-in-existing-test-suites, decision/a11y-gate-breach-fails-build, decision/zero-flakiness).

@history_item n=12 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — CLI output-discipline evidence reports inside the CI `coverage` suite under `gate --require coverage,doctest`. This is where the `cli_verify` changes this chunk puts in the gate block report.

@history_item n=13 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — `ci.yml` is the one push/PR workflow carrying a11y; the scheduled `nightly.yml` carries no a11y step (decision/a11y-ci-github-actions). Any new verify case belongs in `ci.yml`'s test job.

@history_item n=14 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — Mutation over the CLI output-discipline invariants moved to the epoch-boundary audit, with no per-chunk gate. This matches the scope's "no mutation run".

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the census of error writers; a peer's refusal screened to a code word; `unknown`'s detail) — `error_bodies_each_name_a_writer_and_a_witness_that_stand`, `product_error_writers_are_each_named` and `thiserror_messages_interpolate_nothing` hold every error body. `is_code_word` defines the code-word format. `unknown` (exit 14) is the hint carve-out. A record writer that adds error types (file absent, unreadable, a link, not the expected JSON, oversize) must name a writer and a witness, interpolate nothing, and report its closed words in code-word form.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (one hint per code) — `no_two_codes_share_a_hint`, `no_hint_names_release` and the exit-cause table in `tests/cross_exit_causes.rs` hold every code to its own hint. Any new refusal code or hint the probe's closed words introduce must pass them.

@history_item n=17 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — ~ decision/time-bounds-fall-on-the-driver and ~ decision/time-bounds-never-on-human dropped the budget pause from their lists of time bounds. These rows list every bound viola holds. If the sixth run's settle wait or the writer's 3-attempt lock adds a bound, it must fall on verify's own child, never on a human key.

@history_item n=18 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — ~ decision/time-bounds-never-on-human and ~ decision/time-bounds-fall-on-the-driver dropped the status-line deadline, which gives the current wording of both time-bound rows the chunk rests on.

@history_item n=19 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — A held dialog always returns to the human: the hold bounds only the driver. This is the precedent for the scope's rule that when the CLI does not honour the record, the trust dialog stays the human's.

@history_item n=20 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — The founder kept the a11y tier at `Standard (1)` (decision/a11y-tier-standard). The chunk adds no surface that re-opens the tier.

@history_item n=21 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — Built the colour decision (NO_COLOR, TERM, COLORTERM, stdout's terminal state; truecolor/256/16 depth) and the rule that each SGR span sits beside its word, with `unable`/`fail` never coloured. A 23rd row whose stamp turns 22-row stamps `unverified-cli` on the board, and a new verify step reading `fail`, are both bound by these rules.

@history_item n=22 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — A passed `viola revive` enters the same TUI passthrough boundary as `viola run`, the single vendor-content zone (decision/single-vendor-content-boundary). This chunk changes nothing of `run`/`revive`, and verify's interactive children are not part of that boundary.

@history_item n=23 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — The master text became the rows this chunk is bound by, each verified against the code. Every expected amendment (the child set of five becoming six, the bounded `~/.claude.json` exception for verify's own record) is written as a row change against those handles.
