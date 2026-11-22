@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `verify`'s step counter as one static stdout line per ledger row plus the `stamped <ver>  <n> pass  <m> fail` summary, and verify's exit-1 refusals with a failing row printing no stderr word. Origin of the count literals this chunk's item 9 (the relied-row count from ONE source) touches; whatever source it picks must keep the step-line/summary pattern unchanged.

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — moved the counter `[01/06]` → `[01/10]` and the example summary version; the first of a run of amendments that each re-wrote count literals by hand as rows landed. Precedent for item 9 and the wrap freight's drift-base detector over count and version literals.

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — counter `[01/10]` → `[01/14]`, summary at 14; same hand-edited count literal pattern (item 9 / drift detector precedent); the static-lines pattern and uncoloured `fail` restated unchanged.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — counter `[01/14]` → `[01/17]`, adding the framing rows (`long-paste-wrapper`, `tag-escaping`, `local-command-clear`); the framing rows are what `framing_turns`' per-paste probes (discretionary item 8) exercise; another hand-edited count literal (item 9).

@history_item n=5 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — counter at 18 and, decisively, "the step counter derives from `LedgerRow::ALL.len()`. The count now carries its rule": the first statement that the count has a single code source. Item 9 extends exactly this rule.

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — changed the counter to one line per RELIED row: `MM` is `LedgerRow::relied()`'s count, not `ALL.len()`; summary n + m equals `MM`. The current source of truth the relied-row count (item 9) must come from; the row contract/viola-verify-output/step-line and summary-line rest on it.

@history_item n=7 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — relied count to 22 (`resume-session`); another hand-moved count literal; also restated pattern 2's `[  ] unconfirmable` covering `/compact` (contract/viola-send-mirror/unconfirmable).

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — built the board's `claude agents --json` join (STATUS `idle`/`busy` else `unknown`), fixed that a failed, oversize or malformed agents read prints the board as with no `claude`, added revive's `session-live` pair (a failed read passes), and the `agents-join` verify row (count to 23). Directly behind item 4 (verify's bounded agents read in `src/run/agents.rs`): rows contract/viola-list-board/failed-agents-read, contract/viola-list-board/status and contract/viola-revive-refusals/session-live state what a failed/timed-out read must still print; a new bound must keep "a failed read passes / prints as no claude".

@history_item n=9 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` read back by a same-session `session-start` of cause `compact` on a verified CLI, else `unconfirmable`; verify count to 24. Behind contract/send-readback/compact, one of the confirmers `SendSlot::confirm` (item 1) settles on; another hand-moved count literal (item 9).

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build) — `LedgerRow::relied()` reads 24 on every build, `WRAPPED_HERE` folded, Windows set retired; also (sibling sub-entry) revive's ten and verify's five codes pinned in `tests/cross_exit_causes.rs`, and the CI matrix `ubuntu-latest` alone. The last structural change to the relied-count source item 9 reads; pinned code lists in tests are another place count literals live.

@history_item n=11 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `viola verify` stopped by Ctrl-C or TERM before its rows are checked prints no step line, no summary, only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1 (the operator chose exit 1 with a fixed pair over a new code). Bears on item 7 (a verify ended by SIGKILL and its trust-record residue): SIGKILL is not in this contract's Ctrl-C/TERM set; any mend must not add a new word or exit without the operator's fork (rows contract/viola-verify-output/stopped-or-refused, contract/viola-verify-refusals/interrupted).

@history_item n=12 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified verify's interrupted `unable:` / `hint:` pair wording (among others), removing its planning-form mark. The pair item 7 may sit beside is the founder's wording; do not re-word it.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (verify's home-tree and stamps refusals; run's and revive's start refusals) — before its log `verify` checks the home, `ledger/`, `bin/`, `plugin/`; a refusal or a refused `stamps.json` update prints only the home-state pair, no step line or summary, exit 1. Behind contract/viola-verify-refusals/home-tree-check and the stopped-or-refused row; the stamped-set table (item 9) and any verify cleanup mend (item 7) pass through this check.

@history_item n=14 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added contract/viola-verify-output/trust-record-names-nothing: verify writes and removes a trust record in the Claude config file, and no output names the file, key or value. Item 7 measures whether a SIGKILLed verify leaves a project entry in that config under its scratch root; a named residual or mend must keep the config file, key and value out of every line.

@history_item n=15 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — step line gains verdict word `absent` (only on `browser-tools-prompt`, counted as a pass), summary `n` counts it; trust-record row reworded ("like every relied row" dropped); the companion sub-entry moved contract/viola-verify-output/summary-line the same way. Any single relied-count source (item 9) must count `absent` into `n`.

@history_item n=16 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — added contract/viola-verify-output/browser-tools-line: one fixed stdout line before the summary when a run met the browser-tools prompt. Item 9's restructure of verify's output sources must keep this line's place.

@history_item n=17 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — added contract/send-readback/context, widened contract/send-readback/unconfirmable, and changed contract/cli-verification/verified to "a stamp reading every relied row `pass`" (a stamp of an earlier relied set reads unverified). Ties the stamped-set table (item 9) to the relied set: holding it once must keep that verdict; `/context` is also a confirmer `SendSlot::confirm` (item 1) waits on.

@history_item n=18 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — added the six automatic-verify stderr lines of `viola run` / `revive` (verifying, did not verify, earlier red, verifying elsewhere, no stamp, line form; silent under --json). Another reader of the stamped set (item 9) and a verify path whose scratch/trust-record cleanup item 7 measures.

@history_item n=19 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — the founder ratified every Epoch 4 PROVISIONAL mark: the browser-tools step-line/summary/trust-record rows, the six auto-verify lines, and paste-write-bounded's exit-13 `paste-cut` and its hint; also added decision/permission-request-question-held. The rows this chunk rests on are now the founder's word: tests may pin them, no mend may re-word them without a fork.

@history_item n=20 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — contract/cli-exit-codes/13 gained `paste-cut`; added contract/refusal-hints/not-delivered-paste-cut (wording provisional, human mode only). The paste writer whose three mutation survivors item 3 kills (`crates/viola-pty/src/pump.rs`) produces this refusal; killing tests must hold `paste-cut` / exit 13 / the hint as written, with no new word.

@history_item n=21 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box fills on `/clear`'s measured post-condition as well as on a matching `prompt-submitted`; no new word. First widening of what settles a send — behind contract/send-readback/clear and the confirm path item 1 (`SendSlot::confirm`, a claimed line whose settle never comes) mends.

@history_item n=22 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — added contract/send-readback/mcp (a local-command stdout tag confirms) and widened contract/send-readback/unconfirmable. One more settle source on the slot item 1 touches; a claimed line never settled must still end in a printed state of the mirror, never success before confirmation.

@history_item n=23 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — contract/send-readback/prompt: a match is the sent text or the sent text with its trailing ASCII whitespace run removed (then provisional). The matching rule `SendSlot::confirm` (item 1) uses to settle a claimed line; a mend must not loosen it.

@history_item n=24 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance of contract/send-readback/prompt; a leading trim waits for a live reading. Item 1's mend must keep trailing-only matching; leading trim is not this chunk's.

@history_item n=25 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `input-not-ready` hint became `<name> was not ready for input; send again, and if it repeats a human must look at the session` (founder's wording). Behind contract/refusal-hints/not-delivered-input-not-ready; if item 1's never-settled claimed line ends as a `not-delivered` refusal, its existing hint wording is fixed.

@history_item n=26 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — a prior epoch cleanup chunk added the `empty-text` refusal, its hint (founder's wording, relayed) and the exit-13 word, the plan raising it itself as no detector covered the vocabulary. Precedent: a cleanup chunk that adds a closed word must raise it in the masters and to the operator/founder (this chunk's boundary: a new closed word is the operator's fork).

@history_item n=27 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — a prior cleanup chunk cut web-surface references; fixed the readback box's one place as its CLI mirror (decision/readback-box, decision/readback-box-signature-check). Precedent of a cleanup chunk's master edits on the founder's word; the mirror column stands for item 1.

@history_item n=28 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (brand, surface and bans scoped to the CLI) — the readback box's one place is the CLI mirror; the spinner / progress-bar / live-redraw ban reads "in a verb's output". Behind decision/no-spinners-in-verb-output, decision/readback-box and decision/readback-box-signature-check: verify's step counter stays the one progress pattern through any item 9 refactor.

@history_item n=29 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — the fixed `unable: <text>` form widened to the exit-1 refusals of `run`, `revive` and `verify`; no hint names `viola release`. Origin of contract/cli-refusal-line/fixed-unable-form, contract/cli-stderr/fixed-message-refusal and decision/no-hint-names-release, which any verify refusal this chunk might touch (item 7) must follow.

@history_item n=30 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink`'s exit-21 strict-modes refusal takes the fixed instance-state pair) — extended the closed fixed-form list to `link` / `unlink`. Part of the lineage of the fixed-form lists in contract/cli-refusal-line/fixed-unable-form and contract/cli-stderr/fixed-message-refusal.

@history_item n=31 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — `unknown`'s detail a code word or nothing (exit 14 row), faults print the code only with detail to `diagnostics/`, exit 2's fixed stderr lines with no `--json` document, and every `--json` failure driven by `tests/cli_error_bodies.rs` is one document, stderr empty, no ESC/path/upstream text. Behind contract/cli-exit-codes/14, decision/no-hint-for-faults, decision/json-one-document-on-stdout and decision/json-failure-documents-clean (guard `tests/cli_error_bodies.rs`): a new end for a never-settled send (item 1) or a bounded agents read (item 4) must not leak upstream text or a stack trace.

@history_item n=32 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document; `revive --json`'s documents) — `error: internal error` / `{"v":1,"error":"internal-error","detail":null}` at the catch site; `run`, `revive`'s start, `mcp` and `hook` stay silent there; a `pump_child` error prints no document. Behind contract/cli-exit-codes/1, decision/silent-catch-sites and the json-one-document exception: an error raised by a mend in the run/pump path (items 1, 3) lands under these rules.

@history_item n=33 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (each verified against the code; divergences "as the code shows"); every picked row's current text dates from this conversion. A row this chunk finds false against the code is amended by handle (`~`), not by prose.

@history_item n=34 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ contract/refusal-hints/refusal-line`: `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint. Binds any refusal item 1 or 3 adds or tests.

@history_item n=35 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/fixed-phraseology-vocabulary` (`budget-paused` dropped), `~ contract/cli-exit-codes/10-14` and `/11` (11 retired, never reused), `~ decision/no-hint-names-viola-release` (why narrowed to human-typing). Precedent for retiring a word and its exit without reuse; the vocabulary and exit table this chunk must not widen without the operator's fork.
