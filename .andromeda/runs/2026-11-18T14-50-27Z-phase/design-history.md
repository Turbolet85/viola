@history_item n=1 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — added `+ contract/viola-verify-output/trust-record-names-nothing`: the trust-record row prints its step line ending `pass`/`fail` like every relied row, and no verify output names the config file, key or value. This is the closest precedent. The browser-tools row is the next ledger row after it, and its probe has to keep the same rule: no step line, summary or refusal quotes the modal's upstream text or any config key (e.g. `claudeInChromeDefaultEnabled`, `cachedChromeExtensionInstalled`).

@history_item n=2 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set up `verify`'s step counter: one static stdout line per ledger row, then the `stamped <ver>  N pass  M fail` summary, and a failing row prints no stderr word. Rows `contract/viola-verify-output/step-line` and `summary` come from it. The new row adds one more step line in this same pattern (`decision/step-counter-only-progress-pattern`).

@history_item n=3 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — took the counter to ten rows, added the path-free `viola verify --help` paragraph, and replaced the payload refusal with the named `a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`; closed codes, never the content). This is the base of `contract/viola-verify-refusals/unclean-fixture`. Any recorded browser-tools screen added under `fixtures/` must pass that cleanliness check.

@history_item n=4 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — counter to fourteen rows. The four dialog rows were named with their words (`question-answer` · `plan-approve-revise` · `question-notes` · `dialog-concurrency`). This is the precedent for naming a screen/modal row by a fixed word on its step line; the pattern (static lines, uncoloured `fail`) did not change.

@history_item n=5 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — counter to seventeen rows. Each new row was named with its word and a short plain-ASCII clause (`long-paste-wrapper …`, `tag-escaping …`, `local-command-clear …`), and the example version followed the stamped set. This is the form the new row's step line copies.

@history_item n=6 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — counter to eighteen rows, derived from `LedgerRow::ALL.len()`. The count carries its rule, so adding a ledger member moves `MM` with no hand-edited number.

@history_item n=7 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — changed the counter to one step line per *relied* row: `MM` = `LedgerRow::relied()`'s count, and n + m = `MM`. Rows `contract/viola-verify-output/step-line` and `summary-line` read "relied count". Whether the browser-tools row is relied decides whether it moves `MM` (23 to 24).

@history_item n=8 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — counter to `[01/22]` with `resume-session` last in the ledger order, relied on every OS. This is the precedent for a new relied row joining at the end of the ledger order.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — counter to `[01/23]` with `agents-join` last in the member list. This is another precedent for where a new row lands in the list and how the summary sample is restated.

@history_item n=10 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — took the step lines to `[NN/24]` with `local-command-compact` last, and added the `/compact` same-session post-condition readback (`contract/send-readback/compact`). Trap: the history already shows a 24-row counter, while the scope speaks of a 23 to 24 step for the browser-tools row. P3 has to work out the real current relied count and its member order (after the trust-record row) and must not reuse a stale number.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count on every build) `LedgerRow::relied()` became one count on every build (24 at the time), with no OS qualifier on the summary sample. The `shim-resolution` row now passes on `claude --version` answering. There is no per-OS relied set any more, so the new row needs no OS cut. In the same chunk, the CLI platform was narrowed to Linux terminals.

@history_item n=12 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — added verify's interrupted pair: a run stopped by Ctrl-C/TERM before its rows are checked prints no step line and no summary, only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, and exits 1 (`contract/viola-verify-refusals/interrupted`). The new probe, including a run left with a modal standing, has to keep this stop behaviour.

@history_item n=13 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder took verify's interrupted pair (and other Epoch 5 forms) as his own wording, so the pair stands as worded. The new row must not rephrase it.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (verify's home-tree and stamps refusals) `verify` checks its home tree before its log. A refusal prints no step line or summary, only the home-state pair, and exits 1; a refused `stamps.json` update prints the same pair (`contract/viola-verify-output/stamps-refused`, `contract/viola-verify-refusals/stamps-update`). Sub-entry "run's and revive's start refusals at the home-tree check": `run`'s start refusals print fixed pairs and show no path or pid. The new row's stamp still goes through that stamps update, and `run`'s gate must not add a start-refusal form.

@history_item n=15 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — set that `revive`, like `run`, prints nothing once the child starts, and fixed the form of the exit-1 start refusals (one fixed `unable:` line plus one hint per cause, read in a fixed order, no path, pid or id). This is where `decision/run-prints-nothing-while-wrapped`, `contract/viola-run-output/child-started` and `contract/cli-start-refusal/fixed-message` come from. `run`'s pre-send gate meeting a standing browser-tools modal while wrapped must print nothing to the child's terminal.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the catch site's `internal-error` document; `revive --json`'s documents) `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too. A `pump_child` error prints no document because the child holds the terminal. These are rows `decision/silent-catch-sites`, `contract/cli-catch-site/silent-verbs` and `json-internal-error`. A gate refusal inside `run` has no stdout or stderr outlet while the child runs.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory pair at the start of run and revive) the last addition to `run`'s exit-1 start refusals (fixed pair, before any snapshot, child or bind). It shows how a new start cause is added, with its wording a founder-owed planning form. That matters only if P3 gives `run` a new start-time refusal for a home stamped at the older row count.

@history_item n=18 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — reworded the `not-delivered · input-not-ready` hint to the founder's wording, `<name> was not ready for input; send again, and if it repeats a human must look at the session` (`contract/refusal-hints/not-delivered-input-not-ready`). If the gate's refusal at a standing modal surfaces to a driver as `input-not-ready` (or `unknown-dialog`), it reuses this hint unchanged.

@history_item n=19 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — added `not-delivered · empty-text` with its own hint and put it in exit 13's detail list (`contract/cli-exit-codes/13`). This is the precedent if the browser-tools modal needed a new `not-delivered` detail word: that would be a phraseology widening with founder wording, which the scope's "behaviour unchanged in kind" avoids.

@history_item n=20 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ contract/refusal-hints/refusal-line`: `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint. This governs what any refusal from the gate at the modal prints.

@history_item n=21 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/fixed-phraseology-vocabulary` (dropped `budget-paused`), `~ contract/cli-separators/space-hyphen-space` (example now `skipped 2 - 0 - 1`), `~ contract/cli-exit-codes/10-14` and `~ contract/cli-exit-codes/11` (11 retired, never reused), and `~ decision/no-hint-names-viola-release` (why narrowed to the human-typing refusal). These are the current forms of picked rows. Exit 11 cannot be reused for a modal refusal, and any new word (e.g. a `browser-tools-prompt` modal name) joins the fixed vocabulary.

@history_item n=22 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (exit 2's fixed stderr lines; the `--json` failures the witness drives) added the exit-2 exceptions to the one-`--json`-document rule, and every CLI `--json` failure holds no ESC, path or upstream text (guard `tests/cli_error_bodies.rs`). Sub-entry on `unknown`'s detail: a detail is a code word or nothing. These are rows `decision/json-one-document-on-stdout`, `decision/json-failure-documents-clean` and `decision/no-hint-for-faults`. No modal title text («Claude in Chrome extension detected») may reach a document or line.

@history_item n=23 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box can fill on a measured post-condition as well as on the matching `prompt-submitted`, with no new word, and the open box is reused for `unconfirmable`. This is the base of `decision/no-success-before-confirmation` and `decision/no-optimistic-send-confirmation`: a send the gate refuses at a standing modal must never show as read back.

@history_item n=24 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `~ contract/send-readback/unconfirmable` and `+ contract/send-readback/mcp`: `/mcp` reads back on a new stdout tag, and only on a verified CLI. This sets the current readback rows the chunk's verified-CLI dependence sits beside. A newly unstamped or re-stamped row set changes which sends confirm (`contract/cli-verification/unverified-cli`).

@history_item n=25 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `~ contract/send-readback/prompt`: the match now tolerates a trailing whitespace run, a leading trim does not match. This is the picked prompt-readback row. Only a prompt actually submitted fills the box, so text typed into a modal could never count as read back.

@history_item n=26 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance. A leading trim waits for the live reading carried on «Standing live rig with a screen model». The live rig that met the browser-tools prompt is that same rig, so its live starts here must not reopen the readback match.

@history_item n=27 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — set the readback box's one place as the CLI mirror, and the Signature Test as the `[RB]` / `[  ]` / `[/ ]` column of `viola send` (`decision/readback-box`, `decision/readback-box-signature-check`). Any refusal reaching `send` keeps that mirror (`[/ ] unable …`).

@history_item n=28 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — (Squint Test lift) `open` and `unconfirmable` look identical by design, and only the word cell tells them apart. This bears on `contract/readback-box/unconfirmable`.

@history_item n=29 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (brand, surface and bans scoped to the CLI) the spinner, progress-bar and live-redraw ban now reads "in a verb's output", and "no interactive prompt, no TUI mode, no pager" is scoped to a verb. These are rows `decision/no-interactive-prompt-or-pager`, `decision/plain-output-modes` and `decision/cli-tty-static-expression`. verify's new probe stays static appended lines.

@history_item n=30 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the design master into rows checked against the code. Every picked row's present wording dates from here unless a later entry above changed it. An amendment for this chunk names rows by handle (`+`/`~`/`−`).
