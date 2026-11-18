@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set up `viola verify`'s output structure: one `[NN/MM]` step line per ledger row, the `stamped <ver>  <n> pass  <m> fail` summary, a failing row still writing the stamp, and verify's fixed `unable:`/`hint:` refusal pairs. `MM` grows as owning chunks land rows. This chunk's 24th relied row extends this register (rows step-line, summary, failing-row).

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added the screen rows `[07/10] modal-signature …` through `confirm-window`, the `--help` paragraph (an unapproved external CLAUDE.md import blocks the probe), and the `a recorded fixture is not clean: <file> row <n>[ seam] <code>` refusal. This is the precedent for a screen-signature ledger row and its probe. A new recorded screen fixture for the browser-tools prompt has to stay clean under that refusal (row viola-verify-refusals/fixture-not-clean).

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — took verify to 14 rows. Its Kept clause holds that "verify never answers" the CLI-native external-import dialog, and the probe's own dialogs are answered by its hook, never by verify typing. This is the direct precedent for this chunk's rule that verify reads the browser-tools modal and never types into it.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — took the verify wireframe to 17 rows, updating every shown counter, the passing and failing summaries, the ledger-order list and `MM`. This is the edit pattern a row-count bump follows across the wireframe.

@history_item n=5 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — 18 rows. `MM` stopped being a literal and became `LedgerRow::ALL.len()`, so the count carries its rule. A new row changes `MM` by that rule, not by hand.

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` became the relied count (`LedgerRow::relied()`'s length), with exit 0 when every relied row passes. The browser-tools row must be a relied row for verify to print it, and it changes the count (rows step-line, exit-0).

@history_item n=7 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — 22 relied rows: a new last step line, the summary, a failing example summing to the count, and the Usage row list's end. Same register, same edit shape.

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (verify at 23 rows) — made `[23/23] agents-join` the last row, with `stamped …  23 pass  0 fail` and `21 pass  2 fail`. The scope's "23 → 24" step starts from a relied count of 23, which this chunk extends.

@history_item n=9 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — took the verify wireframe to 24 rows with `[24/24] local-command-compact` (`MM` 24 / 21). Trap: a 24-row wireframe ending at `local-command-compact` was written before. The live relied count is now 23 (the trust-record run read 23/23; the rows migration cut relied rows). So "the 24th relied row" must be checked against the current ledger order and the current Usage list, not against this old wireframe.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count and stop on one OS) — `MM` became one count on every build (Linux-only), "growing as owning chunks land their rows". The stop became `SIGINT`/`SIGTERM` with the same plain ASCII pair and no stamp. The new row's count is one Linux figure, and its probe's stop behaviour falls under row viola-verify-output/stop.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the .cmd/.bat refusal leaves revive and verify) — verify's exit-1 pairs are now not-found, version, fixture, stop and home. This is the closed refusal set the picked verify-refusal rows hold. The new probe adds no pair unless a design change says so.

@history_item n=12 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the Windows VT enable leaves the cli colour surface) — the uncoloured conditions are now `--json`, a pipe, `NO_COLOR` and `TERM=dumb`. Row decision/plain-output-modes rests on this, and verify output stays plain ASCII.

@history_item n=13 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — the stopped verify run: `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, stdout empty, no summary, no stamp, exit 1. The summary closes only a completed run. A new interactive probe run has to keep this stop form (rows viola-verify-refusals/stopped, viola-verify-output/stop, viola-verify/summary).

@history_item n=14 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch ratified verify's interrupted pair as worded, among other marks. That pair is the founder's wording, so the new probe must not reword it.

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree refusals of verify and the run and revive starts) — verify checks its home tree (home, `ledger/`, `bin/`, `plugin/`) before its log and maps a refused stamps write to the home pair: stdout empty, no step line, no summary. The run start refuses a widened home tree or pinned copy before the wrapper log opens. A new ledger row and its stamp sit inside this check (rows viola-verify-refusals/home-not-private, run-revive-start/home-tree-refused).

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `viola revive --json` post-preflight gap closed) — the post-preflight start refusals shared with `run` print one exit-1 document (`pinned-hash-mismatch`, …). A `pump_child` error prints none, the child having held the terminal. Row run-revive-start-json/pinned-hash-mismatch comes from this. A gate refusal after the child holds the terminal prints no start document.

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder's code under `viola revive --json`) — split `already-live` from `stale-heartbeat`. Background for row viola-run/already-live, whose human pair stays byte-identical.

@history_item n=18 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the socket-directory refusal of the `run` and `revive` starts) — added a fixed-message exit-1 start refusal before any child, with the founder's wording owed. This is part of the run start's closed refusal set, which the modal gate must not widen (row viola-run/start-output: the start adds no refusal).

@history_item n=19 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — rewrote contract/cli-verbs/run (on a verified CLI the start writes the trust record, flag only, "so no trust dialog stands and no key is typed") and contract/viola-run/start-output (the trust step adds no refusal, exit code, hint, `--json` code or terminal byte). This is the immediate predecessor chunk and the model for handling a start-time CLI modal with no typed byte. This chunk's gate reading of the browser-tools prompt must leave both rows' promises intact.

@history_item n=20 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — set the `input-not-ready` hint to the founder's wording `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session`. A send stopped by the pre-send gate at a standing modal lands on this outcome and hint (row viola-send-output/input-not-ready-hint), so the wording stays as is.

@history_item n=21 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `viola send`'s client-side refusals before any frame (`control-character`, `empty-text`) take `[/ ] unable` with exit 13 and the hint last. Whether the issue line prints on a client-side refusal was left unstated. This is the origin of the client clause in row viola-send-output/unable, and the gate's refusal must fit that row's exit set.

@history_item n=22 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1 of 2) — `~ contract/viola-send-output/unable`: the exit set became 13 (or 10/12/14), and exit 11 is retired and never reused. A gate refusal must not reach for 11.

@history_item n=23 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `+ contract/viola-answer-output/not-delivered-hints`: `viola answer` refused not-delivered with `input-not-ready` (among others) reuses `viola send`'s hint. A modal-gate refusal reached through answer shares the same hint table.

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the `unknown` refusal's detail became a code word or nothing, and `unknown`, `wrapper fault` and `internal error` still take no hint. This is the exception clause of row decision/hint-for-every-cause.

@history_item n=25 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the unwrapped-name and endpoint-lost hints) — every exit-21 cause got its own hint, the last stderr line in plain ASCII. Precedent for decision/hint-for-every-cause and decision/hint-last-on-stderr: a new refusal cause, if any, needs its own distinct hint.

@history_item n=26 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — added a second exit-21 cause with its own hint while keeping the line and the `--json` document. This is the pattern for a new cause printed into the existing refusal block without a new surface.

@history_item n=27 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — the 0.2.0 census: no web surface, viola renders no page (absence/web-surface), and "no interactive prompt and no pager in a verb", the fleet console being no verb's output (decision/no-prompt-or-pager). Verify and run's gate must never prompt the user about the browser-tools modal.

@history_item n=28 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — cut Surface: web-spa to a stub, leaving the cli surface standing alone. This is the earlier step behind absence/web-surface.

@history_item n=29 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (decision 51, contract 173, absence 2; relied: 45 kept, 12 cut), each verified against the code. It explains the row form of every picked row, and why the old prose wireframes (24-row examples) are no longer the body.
