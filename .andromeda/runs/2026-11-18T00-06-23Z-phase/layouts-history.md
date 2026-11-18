@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `viola verify`'s output structure: one `[NN/MM]` step line per ledger row, the `stamped <ver>  <n> pass  <m> fail` last stdout line, a failing-row run exiting 1 with the stamp still written, the usage `viola verify [--record <DIR>] [-- <program> [args…]]`, and verify's fixed exit-1 `unable:`/`hint:` pairs. This is the origin of the step-line, summary, failing-row, exit-0/exit-1 and usage rows. The new per-run first-screen word goes into the stamp's `measured` field, so it must add no human line or column (decision/human-output-is-contract). A live S0′ with `trust-record` failing still writes the stamp the predicate reads.

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added the static ASCII `viola verify --help` paragraph, which names no path: run verify from a folder trusted in Claude Code, and an unapproved external CLAUDE.md import blocks the probe. It also turned the dirty-recording refusal into the named `a recorded fixture is not clean: <file> <code>` (codes `home-path` · `absolute-path` · `username` · `email`; never the content). This is the help-paragraph row. The chunk's closed first-screen words `trust` and `imports` are the two blockers this paragraph names. Keeping no screen text follows the same rule as the clean-fixture refusal. The `--record` witness must not leak a path either: hints never quote text or name a path.

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — took the verify wireframe to 14 rows. **Kept:** the `--help` sentence that verify never answers the CLI-native external-import dialog, while the probe's own dialogs are answered by its hook, not by verify typing. This is a constraint for this chunk: the rig puts no keys into any dialog or box (R6), and verify pastes only its own ratified literals. An `imports` first screen is something verify reads; it never answers it.

@history_item n=4 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — added verify's interrupted form: a run stopped by Ctrl-C or TERM before its rows are checked prints only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, with stdout empty, no summary and no stamp, and exits 1. The summary now closes only a completed run. This is the source of the stop / verify-stopped / summary-line rows. The rig's standing stop for S0′ (Ctrl-C into its pty, or a group kill) and the reworked `verify_window_` kill must keep this form: a stopped run writes no stamp, so it gives no `measured` words.

@history_item n=5 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — `MM` stopped being a literal (was 17) and became `LedgerRow::ALL.len()`. The rule is that the count carries itself as rows land. Precedent: a new row grows `MM` through the rule, not by editing a wireframe number.

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — made `MM` the relied count (`LedgerRow::relied()`'s length) and set exit 0 to mean every relied row passes, with one step line per relied row. This is the basis of the step-line and exit-0 rows. S0′'s `[NN/MM]` and the floor predicate (21 rows passing in the live home's stamp) read the relied set, not ALL.

@history_item n=7 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — took the wireframe to 22 relied rows (`resume-session` last) and made the failing example sum to the relied count. Part of the run of ledger-row amendments: each landed row grew the example block, the failing example and the Usage's row list.

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — took verify to 23 relied rows (`[23/23] agents-join …`) and ended the Usage row list at `agents-join`. Part of the same ledger-row precedent.

@history_item n=9 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — took the verify wireframe to 24 rows (`local-command-compact` last). `MM` became 24 on Unix and 21 on Windows. The trap: the layout's worked counts drifted from the live relied set. The scope calls `trust-record` the 23rd relied row, and the 2026-11-12 rows migration cut relied records. A planner reading `[NN/MM]` should take MM from `relied()`, not from a wireframe number.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — "verify's relied count and stop on one OS": `MM` is the same on every build, and the stop is Ctrl-C or TERM (`SIGINT` / `SIGTERM`) with the same plain ASCII pair and no stamp. "The .cmd/.bat refusal leaves revive and verify": verify's exit-1 pairs became not-found, version, fixture, stop and home. "The per-OS wording of help": `viola <verb> --help` names the program by its executable (the per-verb usage row). "The Windows VT enable leaves the cli colour surface": the plain-output conditions are `--json`, a pipe, `NO_COLOR` and `TERM=dumb`. Together these give the current shape of verify's stop and refusal set, which the 52 s case and the window kill must not change.

@history_item n=11 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — "the home-tree refusals of verify and the run and revive starts": verify's strict-modes check refuses the home, `ledger/`, `bin/` or `plugin/` before its log opens, or refuses the stamps write, with the home pair, exit 1, stdout empty, no step line and no summary. The run and revive starts refuse a widened home tree before the wrapper log opens. This is the origin of the home-not-private and home-tree-refused rows. The rig's private config home and the `--record` DIR must pass this check, and adding `measured` words must keep the stamps write under it.

@history_item n=12 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch ratified verify's interrupted pair as worded and removed its "planning form" mark. The `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` wording is now the founder's. A change to the window kill or the stop must not reword it.

@history_item n=13 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the no-hint exception names the `unknown` refusal by its detail (a code word or nothing). The rule itself did not move: `unknown`, `wrapper fault` and `internal error` take no hint. This is the `except` clause of decision/hint-for-every-cause. A new verify refusal cause (for example a refused `--record` witness) would need its own distinct hint, with the hint as the last stderr line.

@history_item n=14 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0. Flat verbs: no interactive prompt and no pager in a verb, and the terminal fleet console is no verb's output (decision/no-prompt-or-pager). No web surface (absence/web-surface). This chunk's instruments must stay non-interactive: verify and the rig's `V` arm print plain lines, with no prompt and no TUI.

@history_item n=15 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master's text into rows (every area). Each row was verified against the code. Relied records: 45 kept · 12 cut. Divergences were settled "as the code shows". The picked cli rows took their current wording here, and the wireframe's numbers (24 rows, `stamped 2.1.287`) are no longer master truth. For this chunk the code's `relied()` and the stamp are what bind.
