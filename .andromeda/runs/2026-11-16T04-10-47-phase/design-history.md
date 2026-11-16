@history_item n=1 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box fills on `/clear`'s measured post-condition as well as on the matching `prompt-submitted`, and no new word was added (`[RB] read back`, the open box for unconfirmable). This is where contract/send-readback/prompt and contract/send-readback/clear come from. A trim-tolerant match widens what counts as "the matching `prompt-submitted`", so it must still fill only on a real readback (decision/no-optimistic-send-confirmation) and must leave the local-command post-conditions alone.

@history_item n=2 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` is read back by a same-session `session-start` of cause `compact` on a verified CLI and stays `unconfirmable` on an unverified one. It added the 24th verify row (`local-command-compact`). This is the source of contract/send-readback/compact and contract/send-readback/unconfirmable. `classify.rs` reads the exact typed text, so a whitespace change must not move a local command's classification.

@history_item n=3 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the `[  ] unconfirmable … local command, no measured post-condition` line covers `/compact` unchanged (contract/viola-send-mirror/unconfirmable). It also shows the precedent that a new ledger row moves verify's step counter.

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — precedent for undocumented CLI text handling resting on capability-ledger rows with verify probes: `long-paste-wrapper` and `tag-escaping` ("unwraps / un-escapes to the text as pasted"). Each new row is one static ASCII step line, and the counter grows with the rows. This bears on whether the CLI's trim needs its own ledger row (scope: P3 reads, P4 decides).

@history_item n=5 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — added the ledger row `paste-newline` (an LF inside a paste stays inside the one submitted prompt), and the step counter derives from `LedgerRow::ALL.len()`. This is the closest precedent for a whitespace-shape ledger row. If the trim tolerance gets one, verify's design wording (`[01/NN]`, the summary sample) has to be amended the same way.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build) — `LedgerRow::relied()` is 24 on every build, with the summary sample `stamped 2.1.287  24 pass  0 fail`. This is the current baseline a new trim ledger row would move. The same chunk narrowed the cli Platform to Linux terminals, so there are no Windows or macOS arms to consider.

@history_item n=7 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `not-delivered · input-not-ready` hint became `<name> was not ready for input; send again, and if it repeats a human must look at the session` (contract/refusal-hints/not-delivered-input-not-ready). It was reworded because no turn runs in any of that refusal's causes. The readiness-gate race fix must keep this split: a turn that starts inside the gate refuses `turn-running` (hint `a turn is running; viola wait <name> first`), not `input-not-ready`.

@history_item n=8 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — added the `not-delivered · empty-text` refusal, its hint `the text is empty once its trailing newlines are removed; send a text with content`, and `empty-text` at the end of the exit-13 detail list (contract/refusal-hints/not-delivered-empty-text, contract/cli-exit-codes/13). The scope keeps `empty-text` as it is. A text made only of the whitespace the CLI trims may need its own decision, and that decision must not reword this hint or add a detail without amending this row.

@history_item n=9 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — precedent for "never stripped": in message mode a control character prints as `\xHH`, never removed. This matches the chunk's rule that the send text is validated as received and a control character is refused (`control-character`), never stripped.

@history_item n=10 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six frame verbs' strict-modes refusal prints the instance-state pair) — `send` runs its instance strict-modes check after reading its text and before any frame. Refused, it prints the fixed pair at exit 21, not the `[/ ]` mirror. This fixes where any new validation or gate step can sit in send's order without changing which refusal prints first.

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 2's fixed stderr lines, no `--json` document; the `--json` failures the witness drives; `unknown`'s detail) — an unusable `viola send` exits 2 with the fixed `UNUSABLE_TEXT` stderr line and no document whatever `--json` says (contract/cli-exit-codes/2, the except of decision/json-one-document-on-stdout). `tests/cli_error_bodies.rs` guards clean `--json` failure documents (decision/json-failure-documents-clean). An `unknown` detail is a code word or nothing (contract/cli-exit-codes/10-14). Validation-as-received must keep these exits and documents.

@history_item n=12 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document) — `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too (decision/silent-catch-sites, contract/cli-catch-site/silent-verbs). The relabel happens in the wrapper's hook path, so it must print nothing there (contract/hook-verb/stderr, contract/hook-verb/exit-code).

@history_item n=13 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the Git Bash rewritten-path warning retired) — prompt text comes only from stdin or `--file`, and `send` writes no stderr warning about its argument any more. The text the chunk validates "as received" is that stdin or `--file` body, and no warning line may come back.

@history_item n=14 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ contract/refusal-hints/refusal-line`: `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint. Any `turn-running` or `control-character` refusal this chunk adds or moves keeps the shared hint keying.

@history_item n=15 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Surface: cli, 4 and 10 rows; Brand Identity, 3 rows) — the same changes in three areas:
- Surface: cli: `~ contract/cli-exit-codes/10-14` (11 retired, never reused) and `~ decision/no-hint-names-viola-release` (why: the human-typing refusal reaches drivers). The trim bug currently strands the driver behind a human wheel until `release`, and no hint may suggest `viola release` as the way out.
- Surface: cli: exit 11 stays retired; a new refusal must not reuse it.
- Brand Identity: `~ decision/fixed-phraseology-vocabulary` keeps `not-delivered`. Short fixed words only.

@history_item n=16 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — `+ decision/api-error-turn-end-shown-as-turn-end`: an API-error turn end is a plain `turn-ended` with a closed `cause`. This is the latest turn-end change before this chunk. The readiness-gate race reads the turn rung, and decision/turn-ended-never-fills-readback still holds: no turn end, of any cause, fills a send's box.

@history_item n=17 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the readback box's one place is its CLI mirror, and the Signature Test checks the `[RB]` / `[  ]` / `[/ ]` column of `viola send` alone (decision/readback-box, decision/readback-box-signature-check). A confirmed trimmed send must print the `[RB] read back` line in that column (contract/viola-send-mirror/read-back).

@history_item n=18 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0: brand, surface and bans scoped to the CLI) — Brand Identity is scoped to the CLI mirror. The status guards keep the folded refused send (decision/refused-send-folds-into-send-line) and the no-spinner rule. The spinner and redraw ban is scoped to a verb's output. These rows stand on this entry; a fix that waits on the readiness gate must still print nothing until the result.

@history_item n=19 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — when the Decisions Log moved out of the body, the Squint Test kept "`open` and `unconfirmable` look identical by design; only the word cell tells them apart". This underlies contract/readback-box/unconfirmable and the `[  ]` mirror.

@history_item n=20 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows, each verified against the code at 8834195b0683. Every picked row (the readback, send-mirror, refusal-hint and exit-code contracts) dates from this conversion unless a later entry above changed it.
