@history_item n=1 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — `~ decision/no-screen-content-parsing` gained its `except`: the test-only standing live rig may read the trust selector's place, the input box as empty / holds-text / absent, and modal dialogs, as closed words only, raw frames local and gitignored. This chunk's rig `V` arm printing six first-screen words leans on this exception and must stay inside it (closed words, no screen text kept, no frames committed).

@history_item n=2 marker=2026-11-13-verify-readiness-on-the-input-box
**2026-11-13-verify-readiness-on-the-input-box** — `~ decision/no-screen-content-parsing` widened to compiled signatures (footer literals, the input box's rule and prompt rows, modal literals), the prompt row read only for its opening char; ratified as a boundary widening (founder, inputs#I9, card confirm PROVISIONAL). Verify's new per-run first-screen word (`trust` · `imports` · `box` · `none`) must come from these compiled signatures alone; anything beyond them in the product (not the rig) is a new widening.

@history_item n=3 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (decision · contract · absence), each verified against the code; every picked row this chunk reads stands in that form, and later entries amend by handle (`~` / `+` / `−`), the form any amendment this chunk raises must take.

@history_item n=4 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's interrupted pair: Ctrl-C / TERM before rows are checked prints no step line or summary, only `unable: verify was stopped before it finished` and `hint: run viola verify again to stamp this CLI`, exit 1; the stop path adds no time bound (waits end at the stop, kill and pump join at once). Grounds `contract/interrupted-verify/*` and `contract/viola-verify/interrupted-pair`; the chunk's changes to verify's runs and the `verify_window_` kill must keep this pair and its no-new-bound property.

@history_item n=5 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the CLI verbs' strict-modes refusals as scope entities; the strict-modes refusals' text; their witnesses and pinned pairs) — `viola verify` over a home tree failing strict modes refuses before its log opens, exit 1, home-state pair, `hint:` last, no step line or summary; `update_stamps`'s own check gives the same pair; witnesses `verify_refuses_a_widened_ledger_before_any_step`, `verify_under_a_regular_file_refuses_the_home`, `HOME_PAIR` literals. Grounds `contract/start-refusal/home-strict-modes` and `decision/strict-modes-refusal-before-any-write`; the `--record` witness and the stamp's new `measured` field write through `update_stamps`, so this refusal order binds them.

@history_item n=6 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (an unstattable home, per OS) — a home under a regular file is a strict-modes refusal with its hint (verify: home-state pair, exit 1); a per-OS oracle owed wherever a test plants a regular file above a checked path. Origin of `decision/unstattable-home-is-strict-modes-refusal` (Windows arm later retired, see n=8).

@history_item n=7 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the a11y cases' CI home, the one ubuntu-latest leg; the Linux-only platform line) — matrix `[ubuntu-latest]`, cli / tui cases inside the `test` job's single `coverage` suite, `gate --require coverage,doctest` the verdict, runner `scripts/agent-run.sh run` alone; Platform Linux only. Grounds `decision/linux-only`, `decision/a11y-cases-inside-coverage-suite`, `decision/cli-invariants-pass-in-ci`; the chunk's `cli_verify` cases (scope puts `cli_verify` in the gate block) land there.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (strict-modes refusals, the Windows ACL and NotFound arms retired) — `verify_under_a_regular_file_refuses_the_home`'s oracle is the home-state pair alone; strict-modes cases run on ubuntu-latest. Current form of the unstattable-home row the chunk's verify changes must keep green.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the tui boundary on openpty alone) — the TUI hosted in openpty only, each boundary clause its own nextest case on ubuntu-latest, zero-viola-bytes compared byte for byte with an unwrapped fake-agent run. The vendor-content boundary verify's interactive children and the rig's starts run under.

@history_item n=10 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — cli / tui cases run as nextest cases in the `coverage` suite; §10's failure condition is a `gate --require coverage,doctest` breach (`suite-failed`, `suite-skipped`, `suite-missing`, `artifact-missing`); zero-flakiness bans `#[ignore]`; no a11y stage, budget or second driver; §11 bans a separate a11y driver outside `scripts/agent-run.* run`. Origin of `decision/a11y-gate-breach-fails-build`, `decision/zero-flakiness`, `decision/no-a11y-performance-budget`: the 52 s `verify_window_` case must be bounded, not skipped or ignored.

@history_item n=11 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — CLI output-discipline evidence reports in CI inside the single instrumented `coverage` suite gated by `gate --require coverage,doctest`; failures surface only as nextest failures in `suites[].failures[]`. Origin of `decision/gate-requires-coverage-doctest` / `decision/ci-test-gate-no-skips`.

@history_item n=12 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — `ci.yml` is the one push/PR workflow; scheduled `nightly.yml` carries no a11y step. Origin of `decision/a11y-ci-github-actions`.

@history_item n=13 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — the CLI output-discipline invariants (P4, P6) meet mutation only at the epoch-boundary audit (per-chunk and CI mutation gates removed). Consistent with the scope's «no mutation run»: controls are remove-the-guard runs with `--timeout S`, not a mutation gate.

@history_item n=14 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — first live reading of the takeover clause on Linux, recorded as supplemental and never gating. Precedent for `decision/live-readings-never-gate`: the S0′ live start's reading informs the wrap but no CI gate may hang on it.

@history_item n=15 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — the takeover clause's second live reading (the founder's own key on a live `claude` 2.1.296), again supplemental and never gating, no agent typing the key. Precedent for keyless live readings: the chunk's rig types no key into any dialog or box.

@history_item n=16 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (8. Cognitive Accessibility, 6 rows; 1 row) — `~ decision/time-bounds-fall-on-the-driver` and `~ decision/time-bounds-never-on-human` drop the budget pause from the enumerated bounds. These rows enumerate every time bound viola holds; any new or moved bound in verify (the `verify_window_` kill is a test bound, but a product-side read timeout for the first-screen word would be a bound on viola's child) must be listed there and fall on the driver or child.

@history_item n=17 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (8. Cognitive Accessibility) — `~ decision/time-bounds-never-on-human` and `~ decision/time-bounds-fall-on-the-driver` drop the status-line deadline; their current text (wait cap, dialog hold, readiness gate, confirm windows, agents read) is what the chunk reads and would extend.

@history_item n=18 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — the 5 s `claude agents --json` read on viola's own child, fail open, the bound never on a keystroke or the human. Precedent for how a bound on a viola-spawned `claude` child is recorded (the «agents read» in the time-bounds rows), relevant if verify's per-run read gains a bound.

@history_item n=19 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the cli output-discipline checks on std::process::Command and literal pins; the §8 verification lines pinned by literal; the cli tooling in the a11y testing contracts) — cli checks are `std::process::Command` with literal pins (trycmd / assert_cmd retired); every fixed CLI string, the refusal layout, typed exit codes and hint-free `--json` held by literal asserts. Grounds `decision/fixed-cli-strings-pinned*`, `decision/refusal-layout-literal-pins`, `decision/cli-a11y-by-literal-tests`: any fixed string the chunk adds (verify's `measured` words, the `--record` witness line) is pinned by literal.

@history_item n=20 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the SGR layer: decision reads `NO_COLOR`, `TERM`, `COLORTERM`, stdout's terminal state; none under `--json` or a passthrough role; each span beside its word; `viola wait` / `--help` carry no ESC and no lone CR. Origin of `decision/no-sgr-off-terminal`, `decision/sgr-beside-its-word`, `decision/linear-static-cli-output`: verify's step lines and any new output stay plain and linear.

@history_item n=21 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `escape_row` escapes ESC, C1, DEL, `\n`, `\t` as `\xHH`, held by units and no-ESC root cases. Origin of the control-character rows (`decision/control-bytes-escaped`, `contract/cli-output/control-characters`); any value the chunk prints from a recorded or measured source goes through the escape.

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the `--json` failure documents' witness; `unknown`'s detail, a code word or nothing; a peer's refusal screened to a code word) — each `--json` failure one document, literal exit, empty stderr, no ESC, no path or upstream text (`cli_error_documents_hold_no_path_or_upstream_text`); `unknown` (exit 14) prints no hint; `is_code_word` (1–64 bytes `[a-z0-9-]`, lower-case letter first). Grounds `contract/cli-json-failure/document`, `decision/json-failure-documents-are-clean`, `decision/refusal-ends-with-hint`'s except and `contract/code-word/format`; the `--record` witness and any stamp value must hold no path.

@history_item n=23 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the start) — `viola run` and a revive start over a failing socket directory refuse exit 1 before any snapshot, bind or child, the fixed-message pair; verify's interrupted pair named in the same fixed-message form. Origin of `contract/start-refusal/socket-dir` and the `unable:`-line exceptions in `contract/cli-refusal/unable-line`; this chunk changes nothing of `run` / `revive`, so these stand untouched.

@history_item n=24 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — a passed `viola revive` enters the TUI passthrough boundary like `viola run`; zero-SGR list names it. Origin of `decision/single-vendor-content-boundary` and the `viola revive` clauses of the no-SGR rows the chunk must leave as they stand.

@history_item n=25 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — web entities leave; the terminal surfaces stand (CLI output discipline, TUI boundary, manual screen-reader passes); tier `Standard (1)` kept, resting on the terminal surfaces alone. Grounds `decision/a11y-tier-standard` and `decision/no-terminal-conformance-claim`.

@history_item n=26 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder's answer keeps the a11y tier `Standard (1)`. The tier row the chunk plans under is ratified, not provisional.

@history_item n=27 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: no web or graphical frontend; the critical path is cli, `viola list`'s fixed captions and slot order; §5–§7 and the web keys read N/A. Grounds `decision/critical-path-cli-board-output` and the plan's absences the chunk inherits.
