@history_item n=1 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — wait / last lines as landed: gave the `session-end` line and the rule that every non-dialog kind takes the `turn-ended` form, `waiting:` only on a terminal stderr without `--json`, and one `--json` document. This is where contract/viola-wait-output/turn-ended and contract/cli-verbs/wait come from. An API-error turn end has to print through this same `turn-ended  <name>  <ts>  cursor <n>` form, and a new `cause` must not add a human column (decision/human-output-is-contract).

@history_item n=2 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — reworded the wait annotation to "every kind without a dialog_id takes the turn-ended form" (was "every non-dialog kind") and added the `dialog-expired` line. This is a precedent for a new wrapper-recorded event waking `wait`. The chunk's API-error end has no dialog_id, so it falls under the turn-ended form with no new wireframe line.

@history_item n=3 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch ratified the `dialog-expired` wording and the other planning forms. Precedent: a new wait-output word is a planning form until the founder words it. If the chunk adds a human word for the cause, it needs the founder's wording; the form-only route avoids that.

@history_item n=4 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — send's client-side refusals before any frame (`not-delivered  control-character` / `empty-text`, exit 13, the `[/ ] unable` line). This sets the shape of contract/viola-send-output/unable. That row covers the `not-delivered turn-running` refusal this chunk must stop after an API-error end. Its Kept notes that nothing measured whether the issue line prints on a client-side refusal.

@history_item n=5 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1 of 2) changed contract/viola-send-output/unable (exit 13 or 10/12/14; 11 retired, never reused), contract/cli-exit-codes/14 (an older wrapper's `budget-paused` reads `unknown`) and contract/viola-list-json/document (`{items, skipped}`). These are the current wording of picked rows. Precedent: a wire word retired with its emitter, and older-wrapper replies read safely. The chunk's added `cause` key needs the same mixed-version care.

@history_item n=6 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `viola list` added one `claude agents --json` read after the board read: a joined wrapped row reads STATUS `idle`/`busy`, else `unknown`. This is the source of contract/viola-list-board/status-join. That STATUS comes from claude's agents view, not from viola's wheel turn-running state, which explains the measured defect: the board read `idle` while send was refused `turn-running`.

@history_item n=7 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board as first built, with STATUS always `unknown` and the list refusals and widths. Background for contract/cli-verbs/list and the board columns. Since then STATUS has been the agents-join reading (n=6), so the chunk's "the instance reads idle" goes through that join and through no new column.

@history_item n=8 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola list` is styled on a TTY and uncoloured under `--json`, a pipe, `NO_COLOR` or `TERM=dumb`; it also built the grouped `--help`. This is the basis of decision/words-carry-every-state. Any state the chunk makes visible on the board must be carried by words, not by colour.

@history_item n=9 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is the driver's machine view: each tool result is the CLI `--json` document less `v`, and no tool, description or result names release, pause, link or unlink. This is the source of contract/mcp-tools/wait|send|last|list and decision/mcp-hides-wheel-and-handoff-verbs. A turn-ended `cause` reaching the MCP wait tool follows the `--json` document. The fix must not need a driver-visible pause/release.

@history_item n=10 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `--json` became the machine view of every data verb, each refusal one stdout document. This is background for contract/cli-json-mode/one-document and contract/cli-exit-codes/json-form, which bind any new field the chunk exposes under `wait --json`. New fields go into `--json` first (decision/human-output-is-contract).

@history_item n=11 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — exit 21's document carries its cause in `detail` from a closed word list (`strict-modes-failed` … ), and the catch site prints the `internal-error` document under `--json`, with `v` staying 1. This is the closest precedent for this chunk's closed cause set: a closed code list, no free text, and a planning question for the founder before the word list is fixed. It also pins contract/cli-json-mode/one-document's exit-21 / wrapper-fault forms.

@history_item n=12 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — a peer's `refusal` that is no code word reads `unknown`, and a string `detail` that is no code word reads as none, so no opaque free text reaches the surface. This is the direct precedent for mapping the CLI's `error` string into a closed set with a fixed fallback member, and for never surfacing the error text (decision/hints-never-quote-or-locate).

@history_item n=13 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the read-back outcome line prints when the send is confirmed, by `prompt-submitted` or by a post-condition. This is background for contract/cli-verbs/send and decision/turn-ended-not-send-outcome. The chunk's "next send accepted" must keep `turn-ended` a wait result, never a send outcome.

@history_item n=14 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the send `input-not-ready` hint became "send again, and if it repeats a human must look at the session". Precedent for send refusal hints (decision/no-hint-names-release, decision/hints-never-quote-or-locate). A changed `turn-running` hint must not name `viola release` or quote the error.

@history_item n=15 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — `pause`/`release` joined the catch-site verbs when the wheel landed. The wheel's turn-running state, which this chunk clears on an API-error end as a `Stop` does, was built with that chunk. Its verbs stay human verbs, never reached by drivers (decision/no-hint-names-release).

@history_item n=16 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — `viola verify`'s output structure: step lines `[NN/MM]`, the `stamped` summary, a failing-row run exits 1 and still writes the stamp, and the refusal pairs. This is the source of the picked verify rows. A new capability-ledger row for StopFailure (a P3/P4 premise) adds a step line under these contracts.

@history_item n=17 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — `MM` became `LedgerRow::ALL.len()` rather than a fixed count, so the count carries its rule. A new StopFailure ledger row grows MM through code, not through new layout wording.

@history_item n=18 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` is the relied count, `LedgerRow::relied()`'s length; exit 0 when every relied row passes. This is where contract/viola-verify-output/step-line and exit-0 come from. A new row must decide whether it is relied, and how its probe passes without forcing an API error.

@history_item n=19 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — the 24th ledger row (`local-command-compact`) and verify at `[NN/24]`. This is the latest precedent of a chunk landing a ledger row with its probe and the wireframe count changing in the same wrap.

@history_item n=20 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count and stop on one OS) all 24 rows are relied on every build, `[NN/24]` measured on Linux, and the stop is `SIGINT`/`SIGTERM`. This is the current base the chunk's possible 25th row extends (`MM` 24 → 25). The same entry also removed the `.cmd`/`.bat` verify pair, a closed word leaving with its emitter.

@history_item n=21 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — named the dirty-fixture refusal `a recorded fixture is not clean: <file> <code>` (never the content), the source of contract/viola-verify-refusals/fixture-not-clean. Any recorded or synthetic StopFailure payload fixture (none exists in `fixtures/claude/`) must pass this cleanliness check. A home given to `--record` goes through `record_home()`.

@history_item n=22 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify exits 1 on a failing row, a refusal or a stop, and the summary closes only a completed run. This is the source of contract/viola-verify-output/exit-1 and contract/viola-verify/summary, which a new ledger row's probe inherits.

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — verify's home pair: a refused home, `ledger/`, `bin/` or `plugin/` before the log opens, or a refused stamps write, gives exit 1, empty stdout and no step line. This is the source of contract/viola-verify-refusals/home-not-private, binding any verify probe the chunk adds.

@history_item n=24 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters as rows: the layout's text became the decision/contract/absence rows the picked set draws from, each verified against the code. Any amendment this chunk makes (for example to contract/viola-wait-output/turn-ended or to the send/wait rows for a `cause`) is a row change (`+`/`~`/`−` handle), not a prose edit.
