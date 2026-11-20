@history_item n=1 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — set the `viola send` specimen's `input-not-ready` hint to the founder's wording `hint: <name> was not ready for input; send again, and if it repeats a human must look at the session` (row contract/viola-send-output/input-not-ready-hint). This chunk's defect is exactly the "it repeats" case, a gate refusing on every send. The fix changes when the refusal fires, not its line or its hint: the founder's wording stays byte for byte, and a real standing modal must still print it.

@history_item n=2 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — `viola send` refuses two texts itself before any frame (`not-delivered control-character` / `empty-text`, `[/ ] unable` on stderr, exit 13, one document under `--json`). The wrapper makes the same two refusals for a frame sent straight to it (row contract/viola-send-output/unable). It shows where `not-delivered` refusals are decided, client-side or at the wrapper. The pre-send gate's `input-not-ready` is a wrapper-side refusal under the same mirror and exit 13. Kept: no new wireframe, and whether the issue line prints on a client refusal was never measured.

@history_item n=3 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the read-back line gained the `/clear` post-condition trigger. Kept: the `viola send` wireframe's `clear.txt` example ending `not-delivered  input-not-ready` stays, because "the refusal rungs apply to a listed command as to any text". The pre-send readiness gate is one of those rungs. A local-command send meets the same gate, and the both-direction tests apply to it too.

@history_item n=4 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1 of 2) — `~ contract/viola-send-output/unable`: exits narrowed from 10/11/12/14 to 10/12/14, and exit 11 is retired and never reused (`~ contract/cli-exit-codes/11`). The current unable row, which a standing-modal refusal still prints with exit 13 (contract/cli-exit-codes/13 `not-delivered`), rests on this. A fix must not move the exit or reuse 11.

@history_item n=5 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `~ contract/viola-send-output/read-back`: confirmation widened to a prompt equal to the typed text, or equal less a trailing run of space, TAB, CR and LF (PROVISIONAL then). A send this chunk lets through into an idle session with a quoting row must still be read back under this exact rule. The gate fix touches delivery, not the confirmation match.

@history_item n=6 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — two entries: the read-back row lost its provisional mark, and the founder ratified the trailing-only tolerance. A leading trim waits for a live reading on «Standing live rig with a screen model». Constraint: this chunk has no live start, so the leading-trim question stays out of it, and the read-back row stays as worded.

@history_item n=7 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `+ contract/viola-answer-output/not-delivered-hints`: `viola answer` refused not-delivered carries the same hint `viola send` holds for `input-not-ready` (and four other details). If the answer path reads the same readiness predicate, the fix moves `answer`'s refusals as well. The scope asks whether the dialog relay shares the predicate, and the hint table is shared either way.

@history_item n=8 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — landed the screen and timing ledger rows, `[07/10] modal-signature …` among them, in verify's wireframe (rows contract/viola-verify-output/step-line, summary). The compiled modal literals the gate matches are this ledger's modal-signature reading. The scope says no relied ledger row or its verify probe is to change, and verify's step lines, count and summary must stay as they are.

@history_item n=9 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/viola-verify-output/step-line`, `~ …/summary` and `~ …/exit-0` gained `absent` for the browser-tools-prompt row (PROVISIONAL, delegate overseer). That chunk is the latest to add a compiled modal (the browser-tools prompt) to the screen model, so it is one of the literals a transcript row can quote. The guard must keep refusing it when it really stands. The verify rows themselves must not move (scope: no ledger row changes).

@history_item n=10 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — `~ contract/cli-verbs/run`: on a verified CLI the start writes the trust record, so no trust dialog stands and no key is typed. The `viola run` start adds no refusal, exit code or byte (`~ contract/viola-run/start-output`). It is a precedent for a modal kept from standing so sends are not refused. The pre-send gate is `viola run`'s predicate, and this chunk changes no `run` output either.

@history_item n=11 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `viola list` gained the `claude agents --json` join, with STATUS `idle` / `busy` for a joined wrapped row, else `unknown` (row contract/viola-list-board/status-join). In the measured defect `list` read the session idle with no dialog while `send` refused it. The scope asks whether `status` / `list` and the dialog relay read the same predicate, and the board's STATUS comes from the agents join, not the screen gate.

@history_item n=12 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — styled `viola list`: `DIALOG` amber with its kind plain, a stale row dim end to end with only `DIALOG` amber and not dim (rows contract/viola-list-board/stale-row-with-dialog, decision/dialog-attention-only-on-board). If the fix moves the dialog relay's reading together with the gate, the DIALOG column's rendering rules stay as they are. The fix changes only which screens read as a dialog.

@history_item n=13 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — the wait/last lines landed with `dialog unknown` and the `--json` single document, and `send` kept its `[/ ] unable` mirror (rows contract/viola-wait-output/dialog-unknown, contract/viola-send-output/unable). The `wait` dialog line is the relay side of a standing modal. A gate change that also moves the relay's reading must leave these lines unchanged.

@history_item n=14 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — the `viola wait` wireframe gained `dialog-expired  <name>  dialog <id>  cursor <n>` (row contract/viola-wait-output/dialog-expired), worded by the founder at the Epoch 5 batch (2026-10-20T06-58-31-wrap). The dialog hold and its expiry sit next to the modal-standing reading. If the relay shares the predicate, this line and its trigger must not move.

@history_item n=15 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows, each verified against the code, "a master holds what a planner must respect and the code holds the rest". Every picked row's current wording, the send output rows included, dates from this migration. Readiness-predicate internals live in the code, not the master, so the fix needs no row unless a promised output changes.
