@history_item n=1 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — `viola wait`'s result lines gained `session-end`: every non-dialog kind takes the `turn-ended` form (`<kind>  <name>  <time>  cursor <n>`), and message mode escapes control characters as `\xHH`. This is where contract/viola-wait-output/event-form comes from. An API-error turn end that is still `turn-ended` prints this same line, so a `cause` key adds no new wait line form. A new event kind would fall into the time form too.

@history_item n=2 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — The time form was reworded to "every other kind without a `dialog_id`" (was "every other non-dialog kind"). This settles the wait line of any kind by whether it carries `dialog_id`. The line and kind word were a planning form until the founder's word (see 2026-10-20-t06-58-31-wrap). Precedent: a new or extended event kind's wait line and kind word are the founder's wording to give.

@history_item n=3 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — The readback box can now fill on `/clear`'s measured post-condition as well as the matching `prompt-submitted`. Relevant to decision/no-success-before-confirmation, contract/send-readback/prompt and decision/turn-ended-never-fills-readback. The API-error `turn-ended` must not become a new readback trigger: only a named, measured post-condition fills the box.

@history_item n=4 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — `/compact` is read back by its same-session post-condition, and the ledger gained its 24th row, which moved verify's step lines and summary. Two precedents here. First, a post-condition reading counts only on a verified CLI and stays `unconfirmable` otherwise. Second, a new ledger row (such as a StopFailure probe) moves `MM` and the master's verify sample lines.

@history_item n=5 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify prints one step line per relied ledger row, and `MM` is `LedgerRow::relied()`'s count (the summary's n + m equals `MM`). This is the rule behind contract/viola-verify-output/step-line, summary and summary-line. If the chunk adds a capability-ledger row for the StopFailure hook (scope premise), the counter and summary move by this rule. The chunk must also decide whether that row is relied.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's relied count on every build) `relied()` now reads 24 on every build with no OS qualifier, and the summary sample is `stamped 2.1.287  24 pass  0 fail`. This is the current baseline a new StopFailure ledger row would raise to 25.

@history_item n=7 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — Named the `unclean-fixture` refusal: `a recorded fixture is not clean: <file> <code>`, with closed codes and never the content (contract/viola-verify-refusals/unclean-fixture). Any recorded or synthetic StopFailure payload added under `fixtures/claude/` must pass this cleanliness screen. Its `last_assistant_message` / `error_details` text must not leak paths or usernames.

@history_item n=8 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — Built pattern 1's STATUS: a wrapped row joined to a `claude agents --json` row reads `idle` or `busy`, else `unknown` (contract/viola-list-board/status). It also added the `agents-join` ledger row. The scope's defect is a board that reads `idle` while the wheel holds `turn-running`. STATUS comes from the agents join, not the wheel, so the fix must clear the wheel's state. The board's word already reads `idle`.

@history_item n=9 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — The `input-not-ready` hint was reworded, because a `viola wait` there woke on nothing when no turn was running. The `turn-running` hint (`a turn is running; viola wait <name> first`, contract/refusal-hints/not-delivered-turn-running) stayed. This is the chunk's exact trap: after an API-error end, a stuck `turn-running` sends drivers to a `wait` that never returns. The fix keeps the hint true rather than rewording it.

@history_item n=10 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — Added `empty-text` and fixed exit 13's closed detail list (`input-not-ready` … `control-character` / `empty-text`, contract/cli-exit-codes/13), with the founder wording the hint. The chunk adds no refusal. Any new not-delivered detail would extend this closed list, which needs the founder's wording.

@history_item n=11 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — Code words (`viola_core::is_code_word`, 1–64 bytes of lower-case letters, digits and hyphens) screen a peer's `refusal` / `detail`. A non-code-word reads as `unknown` or as no detail. `unknown`'s detail is a code word or nothing, and `--json` failures hold no ESC, path or upstream text (decision/json-failure-documents-clean, guard `tests/cli_error_bodies.rs`). This is the precedent for the chunk's closed cause set: map the CLI's `error` string to a closed code word with a fixed fallback member. The API error text never reaches a document, event or log (decision/no-internals-in-errors).

@history_item n=12 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — Exit 21's document gained a closed `detail` cause set of five words. obs-plan D-20's log detail codes grew, and the catch site got its `internal-error` document. `run`, `revive`'s start, `mcp` and `hook` stay silent at the catch site, under `--json` too (contract/cli-catch-site/silent-verbs, decision/silent-catch-sites). Precedents: a closed cause vocabulary with a named home, and widening one needs the founder's own word shown the widening. Also, `hook` (where the StopFailure payload is read) must stay silent and exit 0 on any error.

@history_item n=13 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`socket-dir-refused`, exit 21's sixth cause and a log code) A new closed word was added to an existing cause set and to D-20's log codes, on the founder's live word with the overseer's delegate confirm. This is the nearest precedent for adding a `cause` member set or key (a wire-contract change) and for who must word it.

@history_item n=14 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `mcp` is a hidden verb, run by the plugin and never by a person (decision/mcp-hidden-verb). Like `hook`, it has no human design surface. A new hook registration in `plugin/hooks/hooks.json` adds no human-facing output or `--help` row.

@history_item n=15 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (exit 2's fixed stderr lines, no `--json` document) Gives the `except` clause of decision/json-one-document-on-stdout and contract/cli-exit-codes/2: `UNUSABLE_TEXT` (an unusable `viola send`) exits 2 with no document. The next send after the turn end must still go through this path unchanged.

@history_item n=16 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `contract/refusal-hints/refusal-line`: `send` and `answer` print the same hint for the same reason or detail. A `not-delivered` refusal with an opaque detail, or none, prints no hint. This governs the `turn-running` refusal and hint the chunk's fix must stop drivers hitting after an API-error end.

@history_item n=17 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — Changed several picked rows. decision/fixed-phraseology-vocabulary lost `budget-paused`. contract/cli-exit-codes/10-14 and 11 now say exit 11 is retired and never reused. decision/no-hint-names-viola-release's why now names only the human-typing refusal. decision/wheel-column-width-23 keeps the width without its widest word. Constraints: the vocabulary is closed and short, exit 11 must not be reused, and any WHEEL word stays within 23 characters.

@history_item n=18 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — The founder confirmed WHEEL padded to 23 and the board's forms (bare skip counts, contract/viola-list-board/skip-counts). If clearing the wheel's turn-running state ever shows on the board, it uses the standing WHEEL words within that width.

@history_item n=19 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — (brand, surface and bans scoped to the CLI) Brand Identity's readback box has its one place in the CLI mirror. Semantic Colors keeps the CLI status guards, `turn-ended` among them, and the spinner / live-redraw ban is scoped to a verb's output. A `turn-ended` with a cause keeps its guard (it never fills the readback, and no colour state is added).

@history_item n=20 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — The masters became rows, each verified against the code. Every picked row's wording dates from here or from later row-level amendments. Changes this chunk makes to a row must be proposed as row edits (`+` / `~` / `−` by handle).
