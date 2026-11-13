@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — founded `verify`'s step counter (one static stdout line per ledger row, then `stamped <ver>  <n> pass  <m> fail`) and its exit-1 fixed refusals with own hints, a failing row printing no stderr word; any readiness ledger row this chunk adds or re-probes lands in this counter and its form (`contract/viola-verify-output/step-line`, `contract/viola-verify-output/summary`, `decision/step-counter-only-progress-pattern`).

@history_item n=2 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap built without its `color` feature so every styled byte is viola's own SGR module's (`decision/clap-without-colour-feature`); a supply-chain bump in `Cargo.toml` / `Cargo.lock` (rmcp, rustix) must not re-enable clap's `color` feature or pull a dependency reading colour/terminal variables.

@history_item n=3 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's counter at ten rows and its static `--help` paragraph; the unclean-fixture refusal `a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`, closed codes, never the content) (`contract/viola-verify-refusals/unclean-fixture`) — the recorded screens this chunk adds under `fixtures/claude/<cli-version>/` (the footer-notice replay) must pass this cleanliness check.

@history_item n=4 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — the counter grew to fourteen with the dialog rows on a re-probe; precedent that the step-line pattern (static appended lines, uncoloured `fail`) is unchanged when rows are added or re-probed (`contract/viola-verify-output/fail-word`).

@history_item n=5 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — landed the framing rows `long-paste-wrapper`, `tag-escaping`, `local-command-clear` as step lines; these are exactly Run B's long, tag-like and local-command pastes whose readiness gate (`input_box_up` in `framing_turns`) this chunk re-keys, so their step words and pass/fail lines must not move.

@history_item n=6 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — `/clear` is read back by its measured post-condition (a `session-start` of cause `clear`), beside `prompt-submitted`; the readback discipline (`decision/no-success-before-confirmation`) the typed run's local-command paste verifies — a readiness change must not let a paste count as landed before its confirmer.

@history_item n=7 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — rewrote the `not-delivered · input-not-ready` hint to `<name> was not ready for input; send again, and if it repeats a human must look at the session` (`contract/refusal-hints/not-delivered-input-not-ready`); the run pump's readiness gate (scope Out unless it shares the footer literal) is what raises `input-not-ready`, so widening there would touch this refusal and exit 13.

@history_item n=8 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — added `empty-text` to exit 13's detail list (`contract/cli-exit-codes/13`); the closed list beside `input-not-ready` — no new detail code is to be added by a readiness change.

@history_item n=9 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — the stderr rule's fixed `unable: <text>` form named the exit-1 refusals of `run`, `revive` and `verify` (`contract/cli-stderr/fixed-message-refusal`, `contract/cli-refusal-line/fixed-unable-form`); any verify refusal this chunk touches keeps that form with its hint, and no hint names `viola release` (`decision/no-hint-names-viola-release`).

@history_item n=10 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify stopped by Ctrl-C/TERM before its rows prints no step line or summary, only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1 (`contract/viola-verify-refusals/interrupted`, `contract/viola-verify-output/stopped-or-refused`); a reworked `box_wait` must keep this interruption behaviour.

@history_item n=11 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the eighteenth row `paste-newline` (Run B's newline paste), and the rule that the step counter derives from `LedgerRow::ALL`; a new readiness ledger row changes the count by that rule.

@history_item n=12 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the step counter counts relied rows (`MM` = `LedgerRow::relied()`), the summary's n + m = `MM` (`contract/viola-verify-output/summary-line`); introduced `statusline-rate-limits`, the account-dependent row the scope's proof exempts from "every row pass".

@history_item n=13 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — `resume-session` added as a relied row, `[01/22]`; another precedent of the counter and summary moving with a new relied row.

@history_item n=14 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify's counter at `[01/23]` with `agents-join` last; the member list order the step lines follow.

@history_item n=15 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` read back by its same-session post-condition; `local-command-compact` the 24th row, `[NN/24]`; this is Run B's `/compact` paste whose readiness gate this chunk changes.

@history_item n=16 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's word made verify's interrupted `unable:` / `hint:` pair his wording; the pair stands verbatim and is not to be reworded.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (verify's home-tree and stamps refusals; a home under a regular file) — verify checks the home, `ledger/`, `bin/`, `plugin/` before its log, and a refused `stamps.json` update prints the home-state pair, exit 1, no step line or summary (`contract/viola-verify-refusals/home-tree-check`, `contract/viola-verify-refusals/stamps-update`, `contract/viola-verify-output/stamps-refused`, `contract/fixed-refusal-pairs/home-state`, `contract/strict-modes/home-under-regular-file`); the proof's stamps are written by verify through this check.

@history_item n=18 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink`'s exit-21 strict-modes refusal) — extended the closed list of the fixed `unable: <text>` form's users (`contract/cli-refusal-line/fixed-unable-form`, `contract/cli-stderr/fixed-message-refusal`); the list this chunk's verify refusals stay in, unchanged.

@history_item n=19 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document) — exit 1's catch site prints `error: internal error` or, under `--json`, `{"v":1,"error":"internal-error","detail":null}` (`contract/cli-exit-codes/1`); an error from verify's typed run that reaches the catch site prints that fixed line, no chain or path (`decision/no-stack-traces`, `decision/no-internals-in-errors`).

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build; the .cmd/.bat refusal retired; strict-modes readings without an OS arm) — `LedgerRow::relied()` is 24 on every build, `shim-resolution` passes when `claude --version` answers; verify's `.cmd`/`.bat` refusal removed, verify's five refusals pinned in `tests/cross_exit_causes.rs`; the home under a regular file refused with no OS lead — the current baseline the proof run's count and refusals are read against.

@history_item n=21 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (brand, surface and bans scoped to the CLI) — the spinner / progress-bar / live-redrawing ban scoped to "a verb's output", `busy` with no animation, `verify`'s step counter the one progress pattern (`decision/no-spinners-in-verb-output`, `decision/busy-not-animated`, `decision/one-progress-pattern`); a wait for the input box in `box_wait` prints nothing in place, no spinner or elapsed counter.

@history_item n=22 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (verified against the code at 8834195b0683); every picked row of this chunk is one of them, and its CI reds (`supply-chain`) started across these row commits — the rows are the binding text the plan cites by handle.
