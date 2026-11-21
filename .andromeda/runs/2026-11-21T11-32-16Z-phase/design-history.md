@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (`viola verify`'s six-row step counter and its exit-1 phraseology) set the verify step counter as one static stdout line per ledger row with a `stamped <ver>  <n> pass  <m> fail` summary, the count growing as owning chunks land rows, and verify's exit-1 refusals each with its own hint; a failing row prints no stderr word. This is the pattern the 29th relied row (`cross-session-prefix`) extends: one more step line, the summary's counts moving, and no new stderr word for a red row.

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — (verify at ten rows and its help paragraph) put `viola verify --help`'s static ASCII paragraph in place (trusted folder, external-import blocker, no path) and turned the recorded-payload refusal into the named `a recorded fixture is not clean: <file> <code>` (closed codes, never the content). The cross-session payload this chunk records and replays is a new fixture class that this screen and refusal must cover, and the sender spawn must not need the help paragraph reworded.

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — moved verify's counter from `[01/10]` to `[01/14]` and named the dialog rows by their words, with the pattern (static appended lines, uncoloured `fail`) unchanged. A precedent: a new row is named by its word on its step line, and the master's example count moves with it.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — moved the counter to `[01/17]` and named three new rows with their step-line words. Same precedent: each new ledger row is one static ASCII step line carrying its own short description.

@history_item n=5 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — moved the counter to `[01/18]` and tied it to `LedgerRow::ALL`, so the count now follows its rule and is not a fixed figure. The 28 to 29 change follows from that rule; no row should state a fixed figure.

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — (verify's step counter at the relied count) changed the counter from one step line per ledger row to one per RELIED row, with `MM` = `LedgerRow::relied()`'s count and n + m = `MM`. This is the source of contract/viola-verify-output/step-line and summary. Whether `cross-session-prefix` is relied decides whether it prints a step line and moves `MM`.

@history_item n=7 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — added the 22nd row (`resume-session`), relied on every OS, at the end of the ledger order. Precedent for where a new row goes (last in the order) and for a relied row with no OS qualifier.

@history_item n=8 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — fixed verify's interrupted pair (`unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1, no step line or summary), later ratified by 2026-10-20-t06-58-31-wrap. This is the source of contract/viola-verify-refusals/interrupted and stopped-or-refused. A probe that spawns a second (sender) CLI must still leave a Ctrl-C/TERM run with no step line and only this pair.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (the `verify` sub-item) moved verify to `[01/23]` with `agents-join` last in the member list. One more precedent for growing the relied set, with the summary sample following it.

@history_item n=10 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — made a `/compact` send on a verified CLI read back only by its measured same-session post-condition (it stays `unconfirmable` on an unverified CLI) and moved verify to 24 rows. This is the source of contract/send-readback/compact. It is the model for a ledger row that unlocks a readback, and the place where a red row that leaves a CLI unverified turns these readbacks to `unconfirmable`.

@history_item n=11 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — (the readback fills on a post-condition too) let a measured post-condition, not only a matching `prompt-submitted`, fill the readback box, with no new word. This is the root of decision/no-success-before-confirmation as applied to the send-readback rows. A cross-session message's prefix must not be taken for a readback.

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (verify's home-tree and stamps refusals; a home under a regular file) made `verify` check the home, `ledger/`, `bin/` and `plugin/` before its log, and made a refused `stamps.json` update print the home-state pair with no step line or summary (exit 1). A home under a regular file became a strict-modes refusal. These are the sources of contract/viola-verify-refusals/home-tree-check, stamps-update, viola-verify-output/stamps-refused and strict-modes/home-under-regular-file. The sender's spawn and its scratch state must keep within this check order. The same entry (run's and revive's start refusals at the home-tree check) holds for the automatic verify behind a start.

@history_item n=13 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count on every build; the .cmd/.bat refusal retired) made `LedgerRow::relied()` the same on every build with no OS qualifier, made `shim-resolution` pass when `claude --version` answers, and removed verify's `.cmd or .bat` refusal. The new row is relied on Linux alone, with no per-OS relied set to keep.

@history_item n=14 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — made `run`/`revive` print nothing once the child starts. It gave the exit-1 start refusals a fixed `unable:` form, one hint per cause, read in a fixed order, showing no path, pid, recorded directory or logged id, and no hint naming `viola release`. These are the sources of decision/run-prints-nothing-while-wrapped, start-refusals-show-no-path-or-pid, cli-start-refusal/fixed-message and no-hint-names-release. Run B's settings literal and a sender started under the automatic verify must not break this silence or these refusal forms.

@history_item n=15 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the catch site's `internal-error` document; `revive --json`'s documents) made the catch site print `{"v":1,"error":"internal-error","detail":null}` under `--json`, and kept `run`, `revive`'s start, `mcp` and `hook` silent there. It gave `revive --json` one exit-1 document per shared-start refusal and printed none for a `pump_child` error. These are the sources of contract/cli-catch-site/json-internal-error, decision/silent-catch-sites and the json-one-document exception. A probe failure must reach these paths and add no new one.

@history_item n=16 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (`--json` failures the witness drives; exit 2's fixed lines; exit 20's fixed `wrapper fault`; `unknown`'s detail) made each `--json` failure one document with no ESC, path or upstream text (`tests/cli_error_bodies.rs`), and gave the exit-2 exception and the no-hint-for-faults line their current wording. These are the sources of decision/json-failure-documents-clean, json-one-document-on-stdout and no-hint-for-faults. The cross-session payload is upstream text: no output may echo it.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory pair at the start of run and revive) added the socket-directory refusal to the run/revive start refusals, before any snapshot, child or bind, in the fixed `unable:` form with no path. These are the start refusals a Run B started by the probe must still meet.

@history_item n=18 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (version census 0.2.0: brand, surface and bans scoped to the CLI) scoped Brand Identity and the spinner, progress-bar and redraw ban to the CLI verbs, and put the readback box's one place in the CLI mirror. These are the sources of decision/no-spinners-in-verb-output, plain-output-modes and cli-tty-static-expression. A longer probe (the 30 s window) must still show only static step lines.

@history_item n=19 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — (the masters as rows) turned the master text into the rows this plan picks, each checked against the code, with relied rows kept or cut. From here on every change is a `+`/`~`/`−` row entry, the form this chunk's design amendment must take.

@history_item n=20 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ contract/refusal-hints/refusal-line`: `send` and `answer` print the same hint for a reason or detail, and an opaque or absent `not-delivered` detail prints no hint. These are the current hint-keying rules, including for the `unverified-cli` hint, which a red `cross-session-prefix` row can bring back.

@history_item n=21 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — `~ decision/fixed-phraseology-vocabulary` dropped `budget-paused`. `~ contract/cli-exit-codes/10-14` and `~ contract/cli-exit-codes/11` mark exit 11 retired and never reused. `~ decision/no-hint-names-viola-release` now names only the human-typing refusal reaching drivers. Any new word or exit code for this chunk must come from the fixed vocabulary and must not reuse exit 11.

@history_item n=22 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — `+ decision/api-error-turn-end-shown-as-turn-end`: an API-error turn end shows as any turn end, its only trace a closed cause word. It was refined by 2026-11-16-granular-api-error-cause (nine words) and ratified by 2026-11-17T05-21-35Z-wrap. A cross-session send that ends Run B's turn on an error reads as a plain turn end, never as the CLI's error text.

@history_item n=23 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `+ contract/send-readback/mcp` and `~ contract/send-readback/unconfirmable` added the transcript-echo confirmer, which needs a verified CLI and a declared transcript. This ties readback to the verified state, so every relied row (including the 29th) affects it.

@history_item n=24 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — `+ contract/viola-verify-output/trust-record-names-nothing`: a ledger row whose probe writes in the Claude config prints nothing that names the file, key or value. This is the nearest precedent for a probe that brings in extra settings (Run B's compiled `crossSessionInbound` accept literal): the screen literal stays unprinted.

@history_item n=25 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify-output/step-line`, `~ summary`, `~ summary-line` and `~ trust-record-names-nothing` added the verdict word `absent`, which only the `browser-tools-prompt` row may write and which counts as a pass there. These were confirmed by the delegate, PROVISIONAL. The new row's step line ends `pass` or `fail` only, and the summary's `n` rule must keep this carve-out.

@history_item n=26 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — `+ contract/viola-verify-output/browser-tools-line`: a fixed stdout line before the summary when a run met the browser-tools prompt. The new row's live run with a sender must not move this line or its placement after the `stamps.json` update.

@history_item n=27 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `~ contract/cli-verification/verified` and `~ unverified-cli`: only a stamp that reads EVERY relied row `pass` reads `verified`, and a stamp made before the relied set grew reads unverified until re-stamped. It also added `+ contract/send-readback/context` and widened `unconfirmable`. Adding the 29th relied row therefore turns every 28-row stamp (the live home's 2.1.328 and 2.1.330) `unverified-cli`. A red `cross-session-prefix` (the `HF` `no-prompt` reading) leaves the version unverified, turning the `/compact`, `/context` and `/mcp` readbacks `unconfirmable` and refusing with exit 12. This is the main trap for this chunk.

@history_item n=28 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `+ contract/auto-verify-line/verifying`, `did-not-verify`, `earlier-red`, `verifying-elsewhere`, `no-stamp` and `line-form`: a start's automatic verify (behind `verify_new_cli`, default off) prints exactly one of five fixed ASCII stderr lines, none under `--json`. These are PROVISIONAL until the founder's own word. The sender also runs under this automatic verify. A red new row produces `did-not-verify` or `earlier-red`, and the closed `no-stamp` words (`spawn-failed`, `unstamped`, `lock-refused`) must hold for a failed sender spawn.
