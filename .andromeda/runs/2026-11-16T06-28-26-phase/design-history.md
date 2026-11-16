@history_item n=1 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box fills on a measured post-condition too (`/clear`'s `session-start` cause `clear`), not only on the matching `prompt-submitted`; no new word: a post-condition-confirmed local command prints `[RB] read back`, an unconfirmed one reuses `unconfirmable` and the open box. The direct precedent for this chunk: a local command's echo as a further confirmer must land the same way (existing words, no optimistic fill), under contract/send-readback/clear, decision/no-optimistic-send-confirmation and decision/no-success-before-confirmation.

@history_item n=2 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` read back on a verified CLI by its same-session `session-start` cause `compact`; on an unverified CLI it stays `unconfirmable`; the 24th ledger row `local-command-compact`; the `unconfirmable` line kept for `/remote-control` and an unverified CLI. Matters here: the echo-confirmed reading must coexist with contract/send-readback/compact and contract/send-readback/unconfirmable, and decide whether `/remote-control`'s `unconfirmable` reading moves; a relied echo is a new ledger row that grows verify's count.

@history_item n=3 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — pattern 2's `[  ] unconfirmable  <name>  local command, no measured post-condition` was kept to cover `/compact` unchanged (verify at 22 rows). The fixed note of contract/viola-send-mirror/unconfirmable is the line this chunk's echo may displace for some commands; its wording and exit 0 are the standing form.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — verify's counter gained `local-command-clear` (/clear starts a new session and submits no prompt), each ledger row one static ASCII step line. Precedent for a local-command ledger row's naming and its step line (contract/viola-verify-output/step-line) if the echo becomes a relied row.

@history_item n=5 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — among the Decisions Log lifts: the Squint Test states `open` and `unconfirmable` look identical by design, only the word cell telling them apart. Constrains this chunk: an echo-confirmed local command must move to `[RB] read back`, never a new drawing (decision/readback-box, contract/readback-box/unconfirmable).

@history_item n=6 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the readback box's one 0.1.0 place is its CLI mirror; the Signature Test checks the one `[RB]` / `[  ]` / `[/ ]` column of CLI `viola send`. Matters for decision/readback-box and decision/readback-box-signature-check: any new send outcome renders only in that column.

@history_item n=7 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (brand, surface and bans scoped to the CLI) — the readback box's one place is the CLI mirror; status guards keep the refused-send fold and `turn-ended`; the no-spinner ban scoped to a verb's output. Holds decision/refused-send-folds-into-send-line, decision/turn-ended-never-fills-readback and decision/no-spinners-in-verb-output, all of which bind how a local-command send waits and reports.

@history_item n=8 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — ~ contract/send-readback/prompt: the `prompt-submitted` match tolerates a trailing ASCII-whitespace run, provisionally; a leading trim does not match. The scope names this match untouched; the local-command path must not reroute ordinary prompt sends through the echo.

@history_item n=9 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — ~ contract/send-readback/prompt made non-provisional; the trailing-only tolerance ratified by the founder, a leading trim waiting on the live reading at «Standing live rig with a screen model». Precedent: an undocumented CLI behaviour relied on waits for a live measurement before it widens, as this chunk's echo may.

@history_item n=10 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — + decision/api-error-turn-end-shown-as-turn-end: an API-error turn ends as plain `turn-ended` with a closed `cause` word, no new wording, PROVISIONAL. Nearby precedent: a new confirming or ending signal adds no output wording, and decision/turn-ended-never-fills-readback stays: a turn end is no echo that fills the box.

@history_item n=11 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — added `not-delivered · empty-text` with its hint and extended exit 13's detail list. Precedent for contract/cli-exit-codes/13: if this chunk adds or retires a `not-delivered` detail for a local command (today the unlisted command ends `no-prompt-submitted`), the exit-13 list, a hint and the founder's wording follow.

@history_item n=12 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `input-not-ready` hint reworded by the founder, no other hint, exit code or refusal moved. Shows that exit-13 hint wording is the founder's to choose; matters if the unlisted local command's `no-prompt-submitted` reading (contract/refusal-hints/not-delivered-no-prompt-submitted) changes.

@history_item n=13 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — ~ contract/refusal-hints/refusal-line: `send` and `answer` print the same hint for the same reason or detail; a `not-delivered` refusal with an opaque detail or none prints no hint. Binds any local-command refusal this chunk keeps or adds.

@history_item n=14 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Brand Identity; Surface: cli, 4 rows and 10 rows) — ~ decision/fixed-phraseology-vocabulary (`budget-paused` dropped), ~ contract/cli-exit-codes/10-14 and /11 (11 retired, never reused), ~ decision/no-hint-names-viola-release (why narrowed to the human-typing refusal). The current form of rows this chunk rests on: no new vocabulary word without need, and no exit code reused.

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the code word and the screen on a peer's reply; the `--json` failures the witness drives) — a peer's `refusal` / `detail` screened through the code word, a non-code-word detail reading as none; each `--json` failure one document, literal exit, empty stderr, no ESC, path or upstream text. Binds contract/peer-reply-screening/* and decision/json-failure-documents-clean: a local command's echo or output is upstream text and must reach no document, detail or hint.

@history_item n=16 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — verify's step counter: one static stdout line per ledger row, the count growing as owning chunks land rows, then the `stamped …  n pass  m fail` summary; a failing row prints no stderr word. The pattern a new relied echo row joins (contract/viola-verify-output/step-line, /summary, decision/step-counter-only-progress-pattern).

@history_item n=17 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — the counter derives from `LedgerRow::ALL.len()`, the count carrying its rule. Matters: a new ledger row moves the counter and the summary sample without hand edits.

@history_item n=18 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — the step counter counts relied rows, `MM` = `LedgerRow::relied()`'s count, the summary's n + m = `MM`. Whether the echo row is relied decides the count this chunk's verify prints.

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build) — `LedgerRow::relied()` reads 24 on every build, no OS qualifier; summary sample `stamped 2.1.287  24 pass  0 fail`. The baseline count a new local-command echo row would move to 25.

@history_item n=20 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — verify at `[01/23]` with `agents-join` appended last in the member list. Precedent that a new ledger row is appended at the list's end with its words, the summary sample following.

@history_item n=21 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows, verified against the code; divergences recorded «as the code shows». Every picked row's wording stems from it; a row this chunk changes (e.g. contract/viola-send-mirror/unconfirmable, contract/send-readback/*) is amended by handle in this form.
