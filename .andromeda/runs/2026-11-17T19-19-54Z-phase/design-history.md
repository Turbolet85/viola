@history_item n=1 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap built without its `color` feature, so every styled byte is viola's own SGR module's (decision/clap-without-colour-feature); a new verify step line, help text or refusal for the sixth run must stay plain, never via a colour-reading dependency.

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — founded `verify`'s step counter (one static stdout line per ledger row, `[NN/MM] <row> <words>  pass|fail`, then `stamped <ver>  <n> pass  <m> fail`, a failing row printing no stderr word) and its exit-1 refusals with their own hints; the 23rd row's step line and its `fail` outcome take exactly this form (contract/viola-verify-output/step-line, summary, fail-word).

@history_item n=3 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — counter moved to ten rows and `viola verify --help` gained its one static ASCII paragraph naming no path: run it from a folder you trust in Claude Code, and the external-import blocker (contract/viola-verify-output/help); a trust record verify writes itself bears directly on that "trusted folder" paragraph — whether it still holds or needs rewording is this chunk's question. Also the named unclean-fixture refusal (contract/viola-verify-refusals/unclean-fixture), which any new recorded fixture of the sixth run must pass.

@history_item n=4 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — counter to fourteen rows, each new row named with its words; precedent: rows land as static appended lines, uncoloured `fail`, pattern unchanged.

@history_item n=5 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — counter to seventeen, each new row named with one static ASCII step line and its words; precedent for how the 23rd row's name and words are worded in the master.

@history_item n=6 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify stopped by Ctrl-C/TERM before its rows prints no step line or summary, only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1 (contract/viola-verify-refusals/interrupted, contract/viola-verify-output/stopped-or-refused); the sixth run's record removal "on every exit path" must cover this interrupted path without adding output beyond the fixed pair.

@history_item n=7 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — counter to eighteen, the step count derived from `LedgerRow::ALL.len()` ("the count now carries its rule"); the 23rd row moves the count by that rule, not by hand.

@history_item n=8 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the step counter switched to the relied count: `MM` is `LedgerRow::relied()`'s count and n + m equals `MM` (contract/viola-verify-output/step-line, summary-line); a new relied row raises `MM`, and every stamp of the old count reads unverified.

@history_item n=9 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the 22nd row `resume-session` landed (`[01/22]`, `stamped 2.1.287  22 pass  0 fail`); the direct precedent for a single new relied row appended at the end of the ledger order with its words.

@history_item n=10 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (the `verify` step counter part) `[01/23]`, `agents-join` last, `stamped 2.1.287  23 pass  0 fail`; trap: the master's history already counted a 23rd row, while the scope says 22 relied today — the plan must read the live `relied()` count and the master's current counter text before naming `[NN/23]`.

@history_item n=11 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — verify's step lines to `[NN/24]`, `local-command-compact` last, `24 pass  0 fail`; same count trap as n=10: the design history reached 24 rows, so whatever the counter text says now must be reconciled with the scope's "22 relied" before the amendment is worded.

@history_item n=12 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified verify's interrupted `unable:` / `hint:` pair as worded; the pair is fixed text this chunk's cleanup path must not reword.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (verify's home-tree and stamps refusals; a home under a regular file) before its log `verify` checks the home, `ledger/`, `bin/`, `plugin/`, refusing with only the home-state pair, exit 1, no step line; a refused `stamps.json` update prints the same pair (contract/viola-verify-refusals/home-tree-check, stamps-update, contract/strict-modes/home-under-regular-file, contract/fixed-refusal-pairs/home-state); the sixth run's record writing sits after these checks and must not add a new refusal form for its own write failures (the scope routes those to the row reading a closed word, never `pass`).

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the catch site's `internal-error` document) exit 1 at the catch site prints the fixed `error: internal error` or `{"v":1,"error":"internal-error","detail":null}` (contract/cli-exit-codes/1); an unexpected failure of verify's record writer that reaches the catch site lands here, never with paths or chains (decision/no-internals-in-errors).

@history_item n=15 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the .cmd/.bat refusal retired; verify's relied count on every build; Platform narrowed to Linux) verify's closed refusal set is five, pinned in `tests/cross_exit_causes.rs`; `relied()` has no OS arm, so the 23rd row is relied on every build; the cli targets Linux terminals only (decision/linux-terminal-platform) — the record writer's lock and atomic replace need only the Linux path.

@history_item n=16 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (brand, surface and bans scoped to the CLI) the spinner / progress-bar / live-redrawing ban scoped to a verb's output (decision/no-spinners-in-verb-output, decision/no-spinner); the sixth interactive run adds wall time to `verify` but its progress stays the step counter (decision/step-counter-only-progress-pattern).

@history_item n=17 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (decision · contract · absence · name · overview), each verified against the code; this chunk's expected amendment lands as `~`/`+` row changes on the picked contracts (e.g. contract/viola-verify-output/step-line, summary-line, help), not as prose sections.

@history_item n=18 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (Brand Identity; Surface: cli 4 rows) `budget-paused` left decision/fixed-phraseology-vocabulary and exit 11 is retired, never reused (contract/cli-exit-codes/10-14); any closed word the trust row reads when it cannot write its record must come from the fixed vocabulary and must not reuse exit 11.
