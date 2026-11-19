@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `viola verify`'s step counter: one static stdout line per ledger row plus the last-line `stamped <ver>  <n> pass  <m> fail` summary, verify's exit-1 refusals with their own hints, and no stderr word for a failing row. This is the pattern the chunk's new browser-tools-prompt line has to fit inside without changing the stamp or the exit.

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — gave `viola verify --help` one static ASCII paragraph that names no path (a trusted folder, the external-import blocker), and named the unclean-fixture refusal with closed codes and never the content. Precedent for any new verify words: fixed, pathless and closed.

@history_item n=3 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — added the `local-command-clear` ledger row, named by its words in the step list. This is the first local-command row. The chunk measures `/context`'s post-condition live, which is the evidence any future `/context` row would rest on.

@history_item n=4 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — let the readback box fill on a measured post-condition (`/clear`'s new session) as well as on the matching `prompt-submitted`, with no new word, and left a command with no measured post-condition on the existing `unconfirmable` line. This sets the rule for `/context`'s live post-condition: until a post-condition is measured and adopted, a `/context` send stays an ordinary send or `unconfirmable`.

@history_item n=5 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — fixed verify's interrupted pair (`unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1, no step line and no summary), which is the picked row contract/viola-verify-refusals/interrupted. A live 24-row verify that is stopped before its rows are checked lands here and writes no stamp.

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — made `MM` the relied count (`LedgerRow::relied()`) rather than every ledger row, with the summary's n + m equal to `MM`. This is the basis of the "24 relied rows" the live re-stamp must pass.

@history_item n=7 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — added the resume-session row and kept `/compact` on pattern 2's `[  ] unconfirmable … local command, no measured post-condition` line. This is an earlier case of a local command that stays unconfirmable until its post-condition row is relied, the same position `/context` is in now.

@history_item n=8 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — made a `/compact` send on a verified CLI read back by its same-session post-condition and left it `unconfirmable` on an unverified CLI (the picked rows contract/send-readback/compact and contract/send-readback/unconfirmable). It also took verify to 24 rows. This is why the stamp matters to the product: a binary that reads every 22- or 23-row stamp as unverified runs transport-only.

@history_item n=9 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — extended the fixed `unable: <text>` stderr form to the exit-1 refusals of `run`, `revive` and `verify`, ruled that no hint names `viola release`, and made a passed `viola revive` the same passthrough with no styling as `viola run`. These are the forms behind the picked rows contract/cli-refusal-line/fixed-unable-form and decision/plain-output-modes, which any new verify line must respect.

@history_item n=10 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified verify's interrupted `unable:` / `hint:` pair as worded, along with other Epoch 5 wordings. Precedent: a new verify output wording starts as a planning form, and the founder words it later.

@history_item n=11 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — three sub-entries bear on this chunk:
- (verify's home-tree and stamps refusals) verify checks the home tree before its log, and a refused `stamps.json` update prints the home-state pair. These are the picked rows contract/viola-verify-output/stopped-or-refused and contract/viola-verify-output/stamps-refused, and they apply to the live re-stamp of the real `~/.viola`.
- (run's and revive's start refusals at the home-tree check) the start checks the pinned copy and its `plugin/<key>/` (picked row contract/viola-run-start-refusals/home-tree-check). This bears on the re-pin.
- (a home under a regular file) such a home is a strict-modes refusal.

@history_item n=12 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — made every CLI `--json` failure one document with stderr empty and no ESC, path or upstream text (guard: tests/cli_error_bodies.rs). It also wrote down the exit-2 exceptions: an oversize `viola answer` and an unusable `viola send` print a fixed stderr line and no document. These are the picked rows decision/json-failure-documents-clean and decision/json-one-document-on-stdout, which constrain any new verify output under `--json`.

@history_item n=13 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — gave the catch site its `internal-error` document (row 1). It also closed `revive --json`'s exit-1 codes, `pinned-hash-mismatch` among them, with a `pump_child` error printing no document. These are the picked rows contract/cli-exit-codes/1 and contract/viola-revive-json-codes/pinned-hash-mismatch, and they cover the re-pinned product's integrity check.

@history_item n=14 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — made `LedgerRow::relied()` 24 on every build and made the `shim-resolution` row pass when `claude --version` answers (verify's relied count on every build). It also retired verify's `.cmd/.bat` refusal and `batch-script-child`, leaving revive's ten codes with `pinned-hash-mismatch`. This is the relied-count basis the live re-stamp is counted against.

@history_item n=15 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — scoped brand, surface and bans to the CLI (version census 0.2.0): the ban on spinners, progress bars and live redrawing now reads "in a verb's output", and the readback box's one place is the CLI mirror. These are the picked rows decision/no-spinners-in-verb-output and the brand overview, which bind verify's new line.

@history_item n=16 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master into rows (decision, contract, absence, name and overview). Every picked row's handle, and every later `~` or `+` entry below, goes back to this migration.

@history_item n=17 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — changed contract/refusal-hints/refusal-line: `send` and `answer` print the same hint for the same reason or detail, and `answer`'s unverified-cli refusal now carries the `run viola verify for this CLI version` hint. Before the re-stamp, an unverified CLI shows this refusal and hint (exit 12).

@history_item n=18 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — three sub-entries bear on this chunk:
- (Brand Identity) removed `budget-paused` from decision/fixed-phraseology-vocabulary.
- (Surface: cli, 4 rows) retired exit 11 for good (contract/cli-exit-codes/10-14).
- (Surface: cli, 10 rows) reworded the why of decision/no-hint-names-viola-release.
All three are picked rows. Any new word on verify's browser-tools line must come from or extend the fixed vocabulary.

@history_item n=19 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — defined what a match is for contract/send-readback/prompt: the sent text, or the sent text with its trailing whitespace run removed, provisionally at first. This is the prompt readback a verified 2.1.325 stamp keeps live.

@history_item n=20 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance of contract/send-readback/prompt. A leading trim still waits on a live reading.

@history_item n=21 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — added contract/send-readback/mcp (a `/mcp` send is confirmed by a new local-command stdout tag in the transcript) and reworded contract/send-readback/unconfirmable (`/cost`, `/usage` and `/remote-control` have no post-condition; `/mcp` stays unconfirmable on an unverified CLI or with no declared transcript). This chunk added the 22nd row, `local-command-echo`, and with it every stamp written before reads as unverified (C3). It also leaves `/context` with no ledger row and the `/mcp disable viola-probe-absent` config write unmeasured (C6).

@history_item n=22 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added contract/viola-verify-output/trust-record-names-nothing: no verify output names the config file, the trust record's key or its value. Any new verify line about the browser-tools prompt, and the PREREQ's key-name read, must keep config keys and values out of verify's output in the same way.

@history_item n=23 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — added the verdict word `absent` on the browser-tools-prompt row only, which counts as a pass on that row, and counted it in the summary's `n`. This changed contract/viola-verify-output/step-line, contract/viola-verify-output/summary and contract/viola-verify-output/trust-record-names-nothing (the delegate's confirm, PROVISIONAL) and contract/viola-verify-output/summary-line. This is the register the chunk's line saying verify met the prompt extends. The live re-stamp passes only with every row `pass`, or `absent` on that row alone.
