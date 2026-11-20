@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `viola verify`'s output structure (one `[NN/MM]` step line per ledger row, the `stamped <ver>  <n> pass  <m> fail` summary, a failing-row run still writing the stamp) and its fixed `unable:`/`hint:` refusal pairs; `MM` grows as owning chunks land rows. This chunk's new relied ledger row and its probe extend exactly this structure (rows contract/viola-verify-output/step-line, summary, failing-row).

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — grew the verify wireframe to ten rows and added the `--help` paragraph that verify never answers the CLI-native external-import dialog. Precedent for re-wording the wireframe counters and summary when rows land; the "verify never answers it" sentence must stay true when the new probe answers a question dialog.

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — verify at fourteen rows, adding the dialog rows (`[11/14] question-answer …` … `dialog-concurrency`). Kept: the probe's own dialogs are answered by its hook, not by verify typing, so the `--help` "verify never answers it" sentence stays. The closest precedent: the existing question-answer row is the sibling of this chunk's PermissionRequest-question row, and the probe answers through the hook in the same way.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — verify at seventeen rows; the passing and failing examples name the one stamped CLI version. Precedent: when a stamp's version changes (this chunk re-stamps 2.1.328 / possibly 2.1.329), the example version moves with it.

@history_item n=5 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — verify at eighteen rows; `MM` stopped being a hard number and became a rule (`LedgerRow::ALL.len()`). The new row grows `MM` by that rule, not by a hand-edited count.

@history_item n=6 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` became the relied count, `LedgerRow::relied()`'s length; one step line per relied row; exit 0 when every relied row passes. This chunk adds a relied row, so it grows `relied()` and the stamp's row set (rows contract/viola-verify-output/step-line, exit-0).

@history_item n=7 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — verify example block at 22 relied rows, the failing example summing to the total, the Usage's row list ending at the newest row. The pattern this chunk repeats for its new row.

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (the verify sub-change) verify at 23 relied rows, the new last step line and the Usage row list grown. Another instance of the row-growth pattern this chunk follows.

@history_item n=9 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — verify at 24 relied rows (`[24/24] local-command-compact …`), the summary and failing example re-counted, the Usage list grown. The last amendment that grew the verify wireframe; the wireframe's count wording sits there and this chunk's row (26 relied rows at `1e9f1ea3` by the scope, the new one making 27) must be reflected wherever the count is spelled.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count and stop on one OS) `MM` is all relied rows on every build, with no Windows subset, "growing as owning chunks land their rows"; the stop is `SIGINT` / `SIGTERM`. (the .cmd/.bat refusal leaves revive and verify) verify's exit-1 pairs are not-found, version, fixture, stop and home. (the Windows VT enable leaves the cli colour surface) the uncoloured conditions are `--json`, a pipe, `NO_COLOR`, `TERM=dumb`. Constraint: a new relied row counts on the one Linux build; verify's refusal pairs (rows contract/viola-verify-refusals/*) stand as listed here.

@history_item n=11 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify-output/step-line`, `~ contract/viola-verify-output/summary`, `~ contract/viola-verify-output/exit-0`: the verdict word gained `absent` for the browser-tools-prompt row alone; `n pass` counts an absent row of that one kind; exit 0 when every relied row counts (PROVISIONAL, delegate overseer). Trap: the new question row's verdict is `pass`/`fail` only unless it is ruled otherwise; if the probe could fail to meet its dialog, a second `absent` exception would be a row change needing a ruling.

@history_item n=12 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — verify's stopped form: Ctrl-C / TERM before rows are checked prints only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, stdout empty, no summary, no stamp, exit 1; the summary closes only a completed run (rows contract/viola-verify-output/stop, contract/viola-verify-refusals/stopped). A longer probe for the new row must keep this stop behaviour.

@history_item n=13 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch ratified the `dialog-expired` line and verify's interrupted pair as worded, the provisional marks superseded. Both are rows this chunk sits next to (contract/viola-wait-output/dialog-expired, contract/viola-verify-refusals/stopped); their wording is the founder's and must not drift.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the home-tree refusals of verify and the run and revive starts) verify checks its home tree (`ledger/`, `bin/`, `plugin/`) before its log opens and maps a refused stamps write to the home pair, exit 1, no step line, no summary (row contract/viola-verify-refusals/home-not-private); the re-stamp in this chunk writes through that guarded stamps path. (the six frame verbs' exit-21 instance pair) `answer` is among the frame verbs whose strict-modes refusal comes before any role file or log opens (row contract/frame-verbs/strict-modes-refused).

@history_item n=15 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — added the `viola answer` verb to the shared catch site's verb list (`error: internal error` once, no hint). The answer verb this chunk carries the question's answers through; if its shape changes, the catch-site and output rows (contract/viola-answer-output/answered, contract/cli-internal-error/json) bind it.

@history_item n=16 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `+ contract/viola-answer-output/not-delivered-hints`: the answer hint table carries `unknown-dialog`'s own hint and the send hints for `control-character`, `empty-text`, `input-not-ready`, `no-prompt-submitted`, `turn-running`. Any new not-delivered detail an answer to a PermissionRequest question could produce needs its own hint row or falls under "opaque detail, no hint".

@history_item n=17 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is a hidden verb whose tools (answer, wait among them) are the driver's machine view: success is the `--json` `ok` payload, refusal the `--json` document less `v` with `isError: true` / `refused: <code>`, codes only. If the answer's shape grows to carry structured answers, the MCP `answer` tool and its result rows (contract/mcp-tools/answer, contract/mcp-tool-results/*) must move with the CLI `--json`.

@history_item n=18 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the `viola wait` wireframe gained `dialog-expired  <name>  dialog <id>  cursor <n>` for an expired hold the wrapper records. The scope keeps today's conservative fall-back for an expired hold; this line (row contract/viola-wait-output/dialog-expired) is what a driver sees when the question's hold expires, and it must stay as is.

@history_item n=19 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — the wait/last lines as landed: the dialog form, `dialog unknown` for an event with no dialog_id, one `--json` document, the exit-21 pair. The question dialog raised through PermissionRequest reaches the driver through these lines (rows contract/viola-wait-output/dialog, dialog-unknown).

@history_item n=20 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board as built: the DIALOG column (17 wide) and the `DIALOG unknown` cell (row contract/viola-list-board/dialog-unknown). A question dialog on the PermissionRequest path shows on the board in this column; its kind word must be one the board renders.

@history_item n=21 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — viola list styled on a terminal: bold NAME, `DIALOG` amber with its kind word plain, a stale row dim end to end with only `DIALOG` amber. These are the rows decision/viola-list-styling, decision/amber-means-dialog and contract/viola-list-board/stale-row-with-dialog; a question dialog surfaced on the new path inherits them unchanged.

@history_item n=22 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `viola send` refuses `control-character` and `empty-text` client-side before any frame, `[/ ] unable` on stderr, exit 13, hint last; the wrapper refuses the same two for a frame sent straight to it. These are the send rows whose hints `answer` borrows (contract/send-refusals/control-character, empty-text). Answer text carried into `updatedInput` is content, so these refusals bound what answer text the driver may send.

@history_item n=23 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `input-not-ready` hint became `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session` (row contract/viola-send-output/input-not-ready-hint), the hint `answer` reuses for that detail.

@history_item n=24 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the `unknown` refusal's detail is a code word or nothing; `unknown`, `wrapper fault` and `internal error` take no hint (row decision/hint-for-every-cause's except). A new refusal cause on the answer path needs its own hint unless it is one of these.

@history_item n=25 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (exit 21's document carries its cause in `detail`) the frame verbs' strict-modes refusal under `--json` reads `"detail":"strict-modes-failed"` (row contract/frame-verbs/strict-modes-refused, answer included). (the `cli` catch site's `internal-error` document under `--json`) a failure under `--json` prints `{"v":1,"error":"internal-error","detail":null}` on stdout, stderr empty (row contract/cli-internal-error/json). Both bind `answer`'s machine view if this chunk touches it.

@history_item n=26 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — `answer` is one of the frame verbs that verify the server before any frame and refuse exit 21 with its own hint; a cause-per-hint precedent the answer path keeps (decision/hint-for-every-cause).

@history_item n=27 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (Surface: cli: 6 row(s) (1 of 2)) `~ contract/cli-exit-codes/11` retired and never reused; `~ contract/cli-exit-codes/14` `unknown`, an older wrapper's `budget-paused` reading as `unknown`; `~ contract/viola-send-output/unable` exits 13 (or 10/12/14). A new answer refusal must not map to exit 11.

@history_item n=28 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ contract/cli-verbs/run`, `~ contract/viola-run/start-output`: on a verified CLI the start writes the trust record and adds no refusal, exit code, hint, `--json` code or terminal byte. "Verified" is the verify stamp this chunk grows: re-stamping against the grown relied set decides which CLI versions keep the trust step and the dialog decisions.

@history_item n=29 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows (decision / contract / absence), each checked against the code; the code holds the rest. Constraint: this chunk's master changes are row changes (`+`/`~`/`−` by handle), and only words a planner must respect get rows.

@history_item n=30 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: no web surface (absence/web-surface), and no interactive prompt or pager in a verb (decision/no-prompt-or-pager). The answer stays a non-interactive verb; viola never prompts the human to answer the question for it.
