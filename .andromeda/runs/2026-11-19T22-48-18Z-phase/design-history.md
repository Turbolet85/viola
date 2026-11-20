@history_item n=1 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — (Brand Identity: 4 rows) changed `contract/cli-verification/verified` and `contract/cli-verification/unverified-cli` so that only a stamp that reads every relied row `pass` counts as `verified`. A stamp of the earlier relied set now reads `unverified-cli`. It also added `contract/send-readback/context` and widened `contract/send-readback/unconfirmable`. Why it matters: if this chunk's means rests on a new relied ledger row with a verify probe, every existing 25-row stamp (the rig's 2.1.328 scratch stamp, the live home's 24-row stamp) goes unverified until re-stamped. The "what an older or unverified CLI gets" answer must follow this rule.

@history_item n=2 marker=2026-11-17T05-21-35Z-wrap
**2026-11-17T05-21-35Z-wrap** — the founder ratified `decision/api-error-turn-end-shown-as-turn-end` (its state went from provisional to standing), along with the delegate's confirm on the nine closed cause words. This is the wrap that placed this entry and retired its withdrawal clause. Why it matters: it is the standing precedent for telling a submitted or ended turn apart with no human-output change. A distinction can travel only as a closed word in `--json` and MCP `wait` event data, with no new status, colour or hint. A wake origin told apart from a typed one can follow that shape.

@history_item n=3 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — (Surface: cli, 1 row) grew the turn-end cause in `decision/api-error-turn-end-shown-as-turn-end` from the single word `api-error` to a closed set of nine words, reaching `--json` and MCP `wait` with no human-output change. Why it matters: it is a precedent for extending a closed event vocabulary without touching the board, the WHEEL words or the hints. It also warns that a row naming one word must be amended when the set grows.

@history_item n=4 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (Surface: cli, 1 row) added `decision/api-error-turn-end-shown-as-turn-end`: an API-error turn end shows like any turn end, and its only trace is a closed cause word. It landed provisional on the delegate's word. Why it matters: it is the original pattern for a CLI-originated event the product must tell apart without a new status or colour, and for what "provisional, the founder's word owed" costs a later chunk.

@history_item n=5 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — (Brand Identity, 1 row) changed `contract/send-readback/prompt`: a send fills its box only when the matching `prompt-submitted` reads it back, the match being exact or minus a trailing ASCII whitespace run, and a prompt trimmed at the start does not match (provisional at first). Why it matters: a fired wake's prompt is a `prompt-submitted` the driver never sent. Telling it apart must not let it fill or strike a driver's open readback box, and the driver's next send must still be read back by its own prompt.

@history_item n=6 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance in `contract/send-readback/prompt`. A leading trim waits for a live reading carried on «Standing live rig with a screen model». Why it matters: the prompt-match rule is settled. This chunk's origin decision about who submitted a prompt must sit beside that match rule without loosening it.

@history_item n=7 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (Brand Identity, Iconography, Surface: cli, Anti-Patterns) did the following:
- dropped `budget-paused` from `decision/fixed-phraseology-vocabulary`, leaving `human-typing` among the fixed words;
- cut `decision/no-hint-names-viola-release`'s why to the human-typing refusal alone, which reaches drivers;
- kept `decision/wheel-column-width-23` at its width after its widest word went;
- retired exit 11 so it is never reused (`contract/cli-exit-codes/10-14`, `contract/cli-exit-codes/11`).

Why it matters: these are the WHEEL and human-typing registers this chunk rests on. No new wheel word or exit code may reuse 11. Any new WHEEL word must fit the padded 23 columns. The human-typing hint must still never name `viola release`.

@history_item n=8 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — gave `viola wait` the `dialog-expired  <name>  dialog <id>  cursor <n>` line in the dialog form (`contract/viola-wait-output/dialog-form`), the wrapper's record of an expired hold carrying `dialog_id`. Why it matters: the dialog hold for the driver is the behaviour a fired wake currently breaks (the next question dialog is not held). Restoring the hold must keep this output form and add no colour for the kind.

@history_item n=9 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch ratified the `dialog-expired` line, the width source and served-verbs-only `--help` as built, and superseded those entries' provisional marks. Why it matters: the dialog-hold output this chunk restores stands on the founder's word, so it is not this chunk's to reword.

@history_item n=10 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — gave `wait` / `last` the `session-end` line and `dialog unknown` for an event with no `dialog_id`, and spelled out message mode (`\xHH` escaping, `\n` and `\t` kept, `--json` serde-escaped). Why it matters: it is the origin of `contract/viola-wait-output/dialog-form` and `contract/cli-message-escaping/json`. A wake's event, if it reaches `wait`, takes these forms and never quotes the prompt text outside message mode.

@history_item n=11 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the wrapper's silent drop of a refused peer) added `contract/wrapper-socket/refused-peer`: a refused peer is dropped before any frame, with no line written, nothing on the child's terminal and no wheel moved. Why it matters: it is the standing rule that only an admitted origin moves the wheel. The new wake-versus-typed origin decision sits in the same wheel-moving path in `src/run/` and must not let any unadmitted source move the wheel.

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — had the six frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`) run the instance strict-modes check before any frame, printing the instance-state pair with no `[/ ]` mirror (`contract/frame-verb-refusals/strict-modes`). It also gave `verify` its home-tree refusal before its log (`contract/viola-verify-output/stopped-or-refused`). Why it matters: the pause/release wheel verbs and `send`'s refusal ordering this chunk touches keep the strict-modes check first. A verify probe added for a wake row inherits verify's refusal shape.

@history_item n=13 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — (Surface: cli, 2 rows) changed `contract/refusal-hints/refusal-line` so that `send` and `answer` print the same hint for the same reason or detail, and a `not-delivered` refusal with an opaque detail or none prints no hint. Why it matters: the `human-typing` refusal a fired wake wrongly provokes is hinted identically on `send` and `answer`. Any change to when `human-typing` fires reaches both verbs alike.

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (the catch site's `internal-error` document; `revive --json`'s codes) set these rules:
- `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too (`contract/cli-catch-site/silent-verbs`);
- `revive --json` prints one exit-1 document for every refusal of the start it shares with `run`, and none for a `pump_child` error (`contract/viola-revive-output/json-refusal`).

Why it matters: the wake origin decision lives in `run`'s pty pump and `hook`'s payload path. Neither may print anything, whatever the decision.

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the `--json` failures the witness drives; the code word screen) every CLI `--json` failure prints one document, exits its literal code, leaves stderr empty and holds no ESC, path or upstream text (`decision/json-failure-documents-clean`). A peer's `refusal` / `detail` is screened to a code word. Why it matters: any new origin or cause word this chunk adds to an event or refusal must be a closed code word, never prompt text from the payload.

@history_item n=16 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — reworded the `not-delivered · input-not-ready` hint (`contract/refusal-hints/not-delivered-input-not-ready`) because none of its causes has a turn running. Why it matters: it is a precedent that a refusal's hint follows its real cause. If a wake's turn makes the driver's next send meet `turn-running` or `input-not-ready` rather than `human-typing`, the existing hints already name those causes.

@history_item n=17 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — added `not-delivered · empty-text` to exit 13's detail list, with its own hint. Why it matters: exit 13's closed detail list (`contract/cli-exit-codes/13`) is the register a new send-path detail would extend. Adding one needs a status-row entry, a hint and the founder's wording.

@history_item n=18 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box fills on a measured post-condition too (`/clear`'s new session), not only on the matching `prompt-submitted`. No new word: the unconfirmable line reuses the open box. Why it matters: this is how the send path decides what confirms a send. A wake's submitted prompt is a further event that must neither confirm nor strike a pending send.

@history_item n=19 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — read `/compact` back by its post-condition on a verified CLI only. On an unverified CLI it stays `unconfirmable`. The 24th relied row was added and the verify counter grew. Why it matters: it is the precedent for gating a measured behaviour on a verified CLI and a ledger row, with the conservative fallback otherwise. This parallels the wake means falling back to `human` on an older or unverified CLI.

@history_item n=20 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — (Brand Identity, 2 rows) widened `contract/send-readback/unconfirmable` to list commands with no post-condition and an echo command that confirms nothing without a verified CLI and a declared transcript. It also added `contract/send-readback/mcp`. Why it matters: it is the unconfirmable register that rests on verified-CLI gating, the same gating a wake means resting on a relied row would use.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count on every build) made `LedgerRow::relied()` the same on every build, with no OS qualifier on relied rows. Other entries in this group retired Windows and macOS arms (width, VT, server check). Why it matters: a new wake ledger row is relied, or not, on the single Linux target. The verify step counter's `MM` derives from `relied()`, so a new relied row moves every stamp's count.

@history_item n=22 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — changed verify's step counter from one line per ledger row to one per relied row, so `MM` is `LedgerRow::relied()`'s count and the summary's n + m equals `MM`. Why it matters: it is the rule a new ledger row for the wake means lands under (`contract/viola-verify-output/step-line`, `contract/viola-verify-output/summary`, `contract/viola-verify-output/summary-line`).

@history_item n=23 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — the 22nd relied row (`resume-session`) grew the counter and summary. Pattern 2's unconfirmable line covers `/compact` unchanged. Why it matters: it is a precedent for one ledger row landing in one chunk, a step line with its own words, and no output form changed.

@history_item n=24 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — (Surface: cli, 1 row) added `contract/viola-verify-output/trust-record-names-nothing` with a relied row whose probe writes and removes state, promising that no output names the file, key or value. Why it matters: it is the most recent precedent for a relied row with a live probe. A wake probe that schedules a timer would carry the same names-nothing and no-real-config constraints.

@history_item n=25 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — (Surface: cli, 3 rows + 1 row) gave verify's step line the verdict word `absent`, on the `browser-tools-prompt` row only, counted as a pass. It amended `contract/viola-verify-output/step-line`, `summary`, `summary-line` and `trust-record-names-nothing`. Why it matters: a new verdict word is a design-row change and was confirmed provisionally by the delegate. A wake row that cannot always be met live (no wake fired) would face the same `absent` question. The rows name `absent` for the browser-tools row alone.

@history_item n=26 marker=2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin
**2026-11-18-the-live-home-s-re-stamp-and-the-product-re-pin** — (Surface: cli, 1 row) added `contract/viola-verify-output/browser-tools-line`, a fixed stdout line verify prints before the summary when a run met the browser-tools prompt. Why it matters: it is the precedent that any new fixed verify output wording needs its own design row. It also produced the live home's 24-row stamp, which this chunk's carries rest on.

@history_item n=27 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — grew verify to ten rows and named the unclean-fixture refusal `a recorded fixture is not clean: <file> <code>` (`contract/viola-verify-refusals/unclean-fixture`). Why it matters: a recorded fixture of a wake payload added for the means must pass the fixture-cleanliness screen (no home path, username or email).

@history_item n=28 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped by Ctrl-C or TERM prints no step line and no summary, only its fixed pair, and exits 1 (`contract/viola-verify-output/stopped-or-refused`). Why it matters: a new probe that waits on a wake timer must keep this stopped-run behaviour.

@history_item n=29 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — built the board: WHEEL padded to 23 and every control character escaped in rows. Why it matters: it is the origin of `decision/wheel-column-width-23` and `decision/list-rows-escape-all-controls`. The WHEEL words a fired wake must leave unchanged print here.

@history_item n=30 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder confirmed WHEEL 23 and the board's forms as his own wording. Why it matters: the WHEEL column and its words are founder-ratified. A fix that keeps the wheel with its holder must not add a WHEEL form without his word.

@history_item n=31 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — built the colour layer: amber only on a pending dialog's `DIALOG` word with the kind plain, and a dim stale row. Why it matters: these are the source of the picked amber, dim and attention-token rows. A dialog held for the driver after a wake shows on the board exactly as any `dialog_pending` row does.

@history_item n=32 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added the unwrapped rack, where unwrapped rows read `n/a` in WHEEL, DIALOG and CLI (`contract/viola-list-board/unwrapped-row`). Why it matters: the wheel only exists for wrapped sessions under `viola run`, so this chunk's change is confined to them.

@history_item n=33 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (version census 0.2.0: brand, surface and bans scoped to the CLI) moved the readback box's one place to the CLI mirror and the cocked strip to the amber `DIALOG` word. The spinner ban is scoped to a verb's output, and the terminal fleet console (matrix#v2-14) inherits the direction. Why it matters: it is the scope of the brand rows picked here (`decision/dialog-pending-cocked-strip`, `decision/readback-box`, `decision/no-spinners-in-verb-output`).

@history_item n=34 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the master's text into rows verified against the code, with fates set by the founder. Why it matters: every picked row's current wording (the wheel, readback, refusal and verify rows) dates from this conversion. A row's divergence marked "as the code shows" defers to the code.

@history_item n=35 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — `revive` prints nothing once the child starts, and a passed revive is the same passthrough as `run`. Its refusals and hints never name `viola release`. Why it matters: it is the source of `contract/viola-run-output/child-started`, `contract/viola-revive-output/after-child-start` and `decision/run-prints-nothing-while-wrapped`. The wake decision made inside `run` / `revive` prints nothing.

@history_item n=36 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `revive`'s `--json` refusals one stdout document with no hint and stderr empty. Why it matters: it is the root of `contract/viola-revive-output/json-refusal`. The `run` / `revive` start path the wake means touches keeps this machine shape.

@history_item n=37 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (`link` / `unlink`'s result lines; `--json` and exit 20) printed links as `link  <driver> -> <driven>`. A driver's `release` is refused (`-32602`, `release-from-driver`, exit 20). Why it matters: it is the transfer-arrow register (`contract/cli-transfer-arrow/ascii-arrow`) and the reason `release` stays a human verb. When the wheel stays with the driver after a wake, no new transfer marker or hint naming `release` may arise.

@history_item n=38 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — landed the capability ledger and `verify`'s static step-counter lines and refusals. Why it matters: it is the origin of the one progress pattern (`decision/step-counter-only-progress-pattern`, `decision/one-progress-pattern`) that any ledger row this chunk adds prints under.

@history_item n=39 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — witnessed the stale row's dim span end to end under an outer PTY (`contract/cli-colour-tokens/stale`). Why it matters: it shows the board witness pattern under an outer PTY. Any board-visible effect of a held dialog after a wake can be witnessed the same way, with no colour moved.
