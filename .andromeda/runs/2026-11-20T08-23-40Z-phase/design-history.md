@history_item n=1 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — rewrote the `not-delivered · input-not-ready` hint to `<name> was not ready for input; send again, and if it repeats a human must look at the session` (was `viola wait <name>, then send again`), the founder's wording, because no cause of that refusal has a turn running. This is the refusal the defect raises wrongly (contract/refusal-hints/not-delivered-input-not-ready): the fix changes when it fires, never its words, and a quoted literal making it "repeat" forever is the exact case the hint sends to a human.

@history_item n=2 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — added `empty-text` to exit 13's detail list beside `control-character`, with its own hint, the founder's wording. Shows how exit 13's closed detail list (contract/cli-exit-codes/13) grows; this chunk must add no new detail word, only stop `input-not-ready` misfiring.

@history_item n=3 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ contract/refusal-hints/refusal-line`: `send` and `answer` print the same hint for the same reason or detail, and an opaque or absent `not-delivered` detail prints no hint. Any refusal path the gate change touches (send and the dialog relay/answer) must keep the shared hint keying.

@history_item n=4 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — Brand Identity `~ decision/fixed-phraseology-vocabulary` dropped `budget-paused`; Surface: cli `~ contract/cli-exit-codes/10-14` and `~ contract/cli-exit-codes/11` retire exit 11 and never reuse it. The vocabulary and exit range this chunk's refusal side rests on are closed: no new word or code for a "quoted modal" outcome.

@history_item n=5 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — `~ contract/send-readback/prompt`: a send fills its readback box only on a matching `prompt-submitted`, trailing ASCII whitespace tolerated, a leading trim not matching (provisional then). A send the fixed gate now lets through must still be read back by this match before `[RB]`; delivery is not confirmation (decision/no-optimistic-send-confirmation).

@history_item n=6 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — ratified the trailing-only tolerance of `contract/send-readback/prompt` on the founder's word; a leading trim waits for a live reading on «Standing live rig with a screen model». Precedent: a screen/transcript-matching rule is widened only after measurement, never on inference, which bears on choosing the gate's discriminator with no live start.

@history_item n=7 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box fills on the matching `prompt-submitted` or a command's measured post-condition (`/clear`'s new session). Frames decision/no-optimistic-send-confirmation and decision/readback-box: the gate change sits before delivery and must leave both confirmers untouched.

@history_item n=8 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` read back by its same-session post-condition; the readback anchor and the no-spinner rejected default name it. Part of the readback register the delivered send still flows into.

@history_item n=9 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — `~ contract/send-readback/unconfirmable`, `+ contract/send-readback/mcp`: local commands with no post-condition stay `unconfirmable`; the `/mcp` echo confirms only on a verified CLI with a declared transcript. Same readback register; a gate fix must not alter which sends are confirmable.

@history_item n=10 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — `~ contract/cli-verification/verified` and `~ contract/cli-verification/unverified-cli`: `verified` only when the stamp reads every relied row `pass`, else transport only as `unverified-cli`; `+ contract/send-readback/context`. Both picked rows: the gate's verdict and any fixture/ledger reading must not shift the relied set or the stamp, per the scope's "no relied ledger row changes".

@history_item n=11 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — verify's step line gained `absent` on the `browser-tools-prompt` row (a compiled modal the product recognises on screen), counted as a pass on that row alone; summary rows reworded. Prior treatment of a modal literal read off the screen; the new discriminator must keep this modal (and every compiled modal literal) refusing when it truly stands, and must not touch verify's wording.

@history_item n=12 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — `+ contract/viola-verify-output/browser-tools-line`: a fixed verify stdout line when a run met the browser-tools prompt. Same modal's surface wording; out of this chunk's change but a trap if the shared screen predicate is moved under verify.

@history_item n=13 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — verify's framing rows `long-paste-wrapper` and `tag-escaping` landed. The scope says verify's Run B framing pastes use the same input-box settle predicate; changing it risks these relied rows, which must not change.

@history_item n=14 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — verify's dialog rows (`question-answer`, `plan-approve-revise`, `question-notes`, `dialog-concurrency`). The standing-modal side of the gate is what these rows exercise; the refusal direction must stay intact for them.

@history_item n=15 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — verify's exit-1 refusal `a recorded fixture is not clean: <file> <code>` (a screen: `<file> row <n>[ seam] <code>`), closed codes, never the content. Any synthetic screen added beside `fixtures/claude/<cli-version>/` must pass this cleanliness check and the refusal never quotes screen text (decision/hint-never-quotes-text, decision/no-internals-in-errors).

@history_item n=16 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `viola wait`'s `dialog-expired` takes the dialog form `<kind>  <name>  dialog <id>  cursor <n>`. The dialog relay register (contract/viola-wait-output/dialog-form); the scope asks whether the relay reads the same predicate — a fix moving them together must keep this output.

@history_item n=17 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `viola list` STATUS reads `idle` / `busy` from the joined `claude agents --json` row, else `unknown` (contract/viola-list-board/status). This is the reading that showed the defect's session idle with no dialog while `send` refused; the chunk decides whether `list`/`status` share the gate's predicate.

@history_item n=18 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour layer: the `DIALOG` word of a wrapped row with a pending dialog in the amber attention token, kind word plain. Ties decision/dialog-pending-cocked-strip and decision/colour-carries-meaning to what the board reads as a standing dialog; a quoted literal must not light it either if `list` shares the predicate.

@history_item n=19 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — the readback box's one place is the CLI mirror; the cocked strip keeps its slot and shows `DIALOG` plus the kind, amber on the CLI. Source of decision/readback-box, decision/readback-box-signature-check and decision/dialog-pending-cocked-strip as picked.

@history_item n=20 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (brand, surface and bans scoped to the CLI) the readback box's one place is the CLI mirror; the CLI status guards keep the refused send folded into its send line and `turn-ended` never filling the box; spinner/progress ban scoped to a verb's output. Origin of decision/refused-send-folds-into-send-line and decision/turn-ended-never-fills-readback, which the send outcome on both sides of the gate must keep.

@history_item n=21 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — `+ decision/api-error-turn-end-shown-as-turn-end`: an API-error turn end shows as any turn end, STATUS `idle`. Precedent of an upstream screen/transcript condition surfacing as plain idle; the gate must read idle sessions consistently with it.

@history_item n=22 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — `~ decision/api-error-turn-end-shown-as-turn-end`: the trace is one closed cause word of nine in `--json` and MCP `wait`, no human mark. Keeps the picked row's current wording; no new human mark for a gate outcome either.

@history_item n=23 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — ratified decision/api-error-turn-end-shown-as-turn-end (state standing) on the founder's word. The picked row is standing, not provisional.

@history_item n=24 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — message mode spelled out: every control character but `\n` and `\t` prints as `\xHH`, never stripped; `wait` result lines gain `session-end` and `dialog unknown`. Source of contract/cli-message-escaping/control-characters; transcript rows quoting modal text reach `last`/`wait` through this escaper, unchanged by the chunk.

@history_item n=25 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — the board as built: the row mode escapes every control character, DEL and C1 included, as `\xHH` (`escape_row`). Source of contract/viola-list-board/row-escaping and decision/list-rows-escape-all-controls; relevant if `list` is moved onto the shared predicate.

@history_item n=26 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the six frame verbs' strict-modes refusal) `send` runs the instance strict-modes check after reading its text and before any frame, refusing with the fixed instance-state pair at exit 21, not the `[/ ]` mirror. Fixes the order of `send`'s refusals ahead of the pre-send readiness gate.

@history_item n=27 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a refused socket directory at the eight frame verbs) exit 21 before any frame, `unable  <name>  instance-unreachable` with `send` in its mirror's form, hint `socket-dir`, `--json` detail `socket-dir-refused`. Source of contract/frame-verb-refusals/socket-dir: another pre-frame refusal of `send` whose mirror form the gate's refusal shares.

@history_item n=28 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the `--json` failures the witness drives) each CLI `--json` failure prints one document, exits its literal code, stderr empty, no ESC, path or upstream text (`tests/cli_error_bodies.rs`). Guard of decision/json-failure-documents-clean, which the exit-13 `input-not-ready` document keeps; no screen or transcript text may enter it.

@history_item n=29 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master's text became rows (decision / contract / absence / name / overview), each verified against the code. Every picked row above stems from this conversion; a change to a row is a `~`/`+`/`−` amendment in this form.
