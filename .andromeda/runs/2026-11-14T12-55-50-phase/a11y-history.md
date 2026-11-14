@history_item n=1 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — changed §10 so that the epoch-boundary mutation audit (`run --mutants`) covers the CLI output-discipline invariants held in crate tests, replacing "the tests' mutation gate", after the founder removed the per-chunk and CI mutation gates. This is the origin of decision/mutation-audit-covers-crate-invariants and the reason the chunk runs no mutation itself: the Epoch 1 audit found the two `MAX_FRAME` survivors, and the next audit must catch them.

@history_item n=2 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the client euid refusal's limit of proof) — added the Error recovery verification item saying a seam's wiring is read only by forced refusals in remove-the-guard runs that no committed test repeats, because a mutant replacing a seam's read survives every committed test. This is the precedent behind decision/client-euid-refusal-test-strategy and the one-off remove-the-guard control pattern this chunk uses on `read.rs:77` and `reply.rs:151` (`>` → `>=`).

@history_item n=3 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — moved the CLI output-discipline evidence in CI into the `test` job's single instrumented `coverage` suite under `gate --require coverage,doctest`, with failures surfacing only as nextest failures in `suites[].failures[]`. This is the source of decision/a11y-cases-inside-coverage-suite, decision/gate-requires-coverage-doctest and decision/a11y-failures-are-nextest-failures. The chunk's new `viola-hyprland` tests land in that same gated suite.

@history_item n=4 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — removed the browser a11y leg. The cli and tui cases became nextest cases in the `coverage` suite. The failure condition is a `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`), `#[ignore]` is banned, and §11 bans any `suite` value or separate a11y driver outside `scripts/agent-run.* run`. This entry is the source of decision/a11y-gate-breach-fails-build, decision/zero-flakiness, decision/no-separate-a11y-driver and decision/a11y-runs-in-existing-test-suites. The new guard tests must not be `#[ignore]`d or skipped, and they need no a11y suite of their own.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the a11y cases' CI home, the one ubuntu-latest leg; the Linux-only platform line) — set the CI matrix to `[ubuntu-latest]`, with the `coverage` suite's `gate --require coverage,doctest` as the verdict and `scripts/agent-run.sh run` as the only runner. It also wrote the platform line: Linux only, with the 0.2.0 window features on Hyprland. This is the current form of decision/cli-invariants-pass-in-ci and decision/ci-test-gate-no-skips. The `viola-hyprland` tests run on that one leg.

@history_item n=6 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added a §8 Timeout extensions entry for the `claude agents --json` read: a direct spawn with `take(MAX_FRAME + 1)` and a 5 s kill, where a failed, oversize, malformed or timed-out read fails open and the bound falls on viola's own child, never on the human. This is the precedent for the `MAX_FRAME + 1` drain and oversize pattern that `read_within` mirrors (`read.rs:121`), and for decision/time-bounds-fall-on-the-driver / decision/time-bounds-never-on-human covering a bounded read of viola's own child.

@history_item n=7 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — added `STATUSLINE_DEADLINE` to §8 as a bound on the user's own command, never on a keystroke, because the plan lists every bound viola holds and whom it falls on. This is one of the bounds named in decision/time-bounds-never-on-human. If the chunk changes the hyprctl read's time bound, the change has to be stated against that row.

@history_item n=8 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — recorded the first live reading of the takeover clause on Linux (a `wtype` key on a compositor the test started itself) as supplemental and never gating. This is the origin of decision/live-readings-never-gate. The chunk takes no live desktop step and uses only the fake `hyprctl`.

@history_item n=9 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — added the takeover clause's second live reading (the founder's own key on the product binary), also supplemental and never gating. This confirms decision/live-readings-never-gate, so no gating case can rest on the real compositor or on a live `claude`.

@history_item n=10 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — this was the bounded-inputs chunk. On the a11y side it added only the outer-terminal half of the contained vt100 feed-panic case, leaving §4 P4 and the §3 keyboard harness unchanged. It is the precedent that a test-only bound pin like this chunk's moves no a11y clause unless it adds a human-facing byte.

@history_item n=11 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0 — the frontend stubs retired) — made §5–§7 and the web keys N/A because viola has no web or graphical frontend, and gave the terminal fleet console's a11y scope to «Fleet a11y verification». This is the source of the picked absences and N/A lines: the Hyprland seam is a window feature with no rendered page, so it adds no a11y surface.

@history_item n=12 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master's text into rows (decision · contract · absence), each verified against the code. This is where every picked row's handle and wording comes from, including decision/no-assertions-for-internal-crates, which keeps `viola-hyprland` (an internal crate) without a11y assertions.
