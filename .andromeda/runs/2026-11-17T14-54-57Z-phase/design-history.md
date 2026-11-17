@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `viola verify`'s step counter (one static stdout line per ledger row, then the `stamped <ver>  <n> pass  <m> fail` summary) and its exit-1 `unable:` refusals with their own hints, with no stderr word for a failing row. This is where viola-verify-output/step-line, summary and fail-word come from. A live `viola verify --record` (R4) must print exactly this shape.

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added `viola verify --help`'s static paragraph (a trusted folder; an unapproved external CLAUDE.md import blocks the probe). It also replaced the fixed payload refusal with the named `a recorded fixture is not clean: <file> <code>` (closed codes, never the content), on the founder's ruling. Two parts of this chunk rest on it: the external-imports modal reading (it is the blocker the help names) and R4's record home (a path component named `home` must not trip the `home-path` code).

@history_item n=3 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — changed verify's step lines from one per ledger row to one per relied row: `MM` is `LedgerRow::relied()`'s count, and n + m equals `MM`. This is the rule behind step-line/summary-line. A live record run counts against `relied()`, not against `ALL`.

@history_item n=4 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — brought verify to 24 rows (`local-command-compact` last) and made `/compact` read back by its post-condition. This sets the step-line count that a live R4 run on 2.1.325/2.1.326 should show.

@history_item n=5 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (verify's relied count on every build) — made `relied()` 24 on every build with no OS qualifier, and `shim-resolution` passes when `claude --version` answers. This is the standing count and row wording that R4's live `verify --record` output is checked against.

@history_item n=6 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — set verify's interrupted pair: Ctrl-C/TERM before the rows are checked prints no step line or summary, then `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1. This is the source of viola-verify-refusals/interrupted. A rig that stops a live record run hits this pair.

@history_item n=7 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified verify's interrupted pair wording (and other Epoch 5 forms), removing its planning-form mark. The pair stands as worded: no rewording without his word.

@history_item n=8 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (verify's home-tree and stamps refusals) — before its log, verify checks the home, `ledger/`, `bin/` and `plugin/`, each that exists. A refusal, or a refused `stamps.json` update, prints only the home-state pair, with no step line or summary, exit 1. This is the source of viola-verify-refusals/home-tree-check and stamps-update. R4's record home must pass this check (private modes) or the reading fails here rather than at a probe.

@history_item n=9 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (a home under a regular file) — a home under a regular file is a strict-modes refusal, not the catch-site line: verify prints the home-state pair (exit 1), the frame verbs the instance-state pair (exit 21). This is the source of strict-modes/home-under-regular-file and applies to how a rig-built record home is laid out.

@history_item n=10 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (strict-modes readings without an OS arm; .cmd/.bat retired) — dropped the Windows `NotFound` reading of a home under a regular file. It also removed verify's `the claude CLI is a .cmd or .bat script` refusal, leaving verify's refusal set as listed in the rows (CLI not found, version unreadable, unclean fixture, home tree, stamps, interrupted).

@history_item n=11 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — first worded the home-state pair `unable: the viola home is not private to you` / `hint: the viola home must be readable only by you; viola does not change its permissions` and widened the fixed `unable: <text>` form beyond exit 1. This is the source of fixed-refusal-pairs/home-state. The founder ratified it at 2026-10-13-t06-38-35-wrap.

@history_item n=12 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder made the home-state pair and `list`'s exit-21 refusal his own wording. The pair that verify's refusals print is ratified text and must not move.

@history_item n=13 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — extended the fixed `unable: <text>` form to the exit-1 refusals of `run`, `revive` and `verify`. It also made a passed `viola revive` print nothing once the child starts, the same passthrough as `viola run`. This is the source of run-prints-nothing-while-wrapped, plain-output-modes and cli-refusal-line/fixed-unable-form. Live rig starts wrap `viola run`/`revive` and must expect silence.

@history_item n=14 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (`link` / `unlink`'s exit-21 strict-modes refusal) — added `link`/`unlink`'s exit-21 strict-modes refusal to the fixed-form list. That list is now the one in cli-stderr/fixed-message-refusal and cli-refusal-line/fixed-unable-form.

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six frame verbs' strict-modes refusal) — `send`, `wait`, `last`, `answer`, `pause` and `release` print the fixed instance-state pair at exit 21 (not the `[/ ]` mirror) and join the fixed-form list. This completes the verb list the rows hold for the fixed `unable:` form.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the catch site's `internal-error` document) — the catch site prints `{"v":1,"error":"internal-error","detail":null}` under `--json`, with `run`, `revive`'s start, `mcp` and `hook` silent at the catch site. This is the source of cli-exit-codes/1, silent-catch-sites and cli-catch-site/silent-verbs. A rig-driven wrapper that hits an internal error must stay silent.

@history_item n=17 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — gave `revive` its `--json` refusal documents, the only exit-1 `--json` besides the catch site's. This is the basis of cli-exit-codes/1's "`--json` only for `revive`'s shared-start refusals".

@history_item n=18 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the founder reworded the `input-not-ready` hint, because a turn is running in none of its causes. This is a precedent for the hook-latency window (R3c): a turn the CLI starts before its `UserPromptSubmit` reaches the wrapper sits between `input-not-ready` and `turn-running` (exit-13 details). A live reading of it touches which refusal a send meets.

@history_item n=19 marker=2026-10-09-epoch-3-cleanup
**2026-10-09-epoch-3-cleanup** — added `empty-text` to exit 13's detail list. This closes the not-delivered detail set held in cli-exit-codes/13.

@history_item n=20 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — the readback box fills on the matching `prompt-submitted` or on a measured post-condition. This bears on no-optimistic-send-confirmation and no-success-before-confirmation: a fired wake's prompt-submit (R3a) must not be read back as a send's confirmation.

@history_item n=21 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — changed send-readback/prompt so a match tolerates the trailing whitespace run the CLI trims (provisional at the time). This is the row this chunk's live prompt-submit payloads are read against.

@history_item n=22 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — the founder ratified the trailing-only tolerance in send-readback/prompt. A leading trim ("either end") waits for a live reading carried on the standing live rig. That reading is this rig's to take if a reading meets it, and the row must not widen without it.

@history_item n=23 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — widened send-readback/unconfirmable and added send-readback/mcp, so a confirmer depends on a verified CLI. This is relevant because a reading on 2.1.326 (no stamp) runs as `unverified-cli`.

@history_item n=24 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — set the `viola wait` result forms (every non-dialog kind, `session-end` included, in the `turn-ended` form) and `last`'s message mode (control characters as `\xHH`). This is the source of viola-wait-output/event-form and viola-last-output. R3b reads `turn-ended` live through these forms.

@history_item n=25 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — narrowed the time form to "every other kind without a `dialog_id`", and `dialog-expired` takes the dialog form. This is the current wording of viola-wait-output/event-form.

@history_item n=26 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — `turn-ended` carries one closed cause word of nine in `--json`/MCP `wait` data, with no human-output change. The founder ratified this at 2026-11-17T05-21-35Z-wrap. A live `turn-ended` read for `scheduled_wakes` (R3b) sits beside this cause field. Its human form must stay unchanged.

@history_item n=27 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (`unknown`'s detail, a code word or nothing) — exit 14's detail is a code word or nothing, and a peer's text never reaches output. This is the basis of the cli-exit-codes/10-14 range and no-internals-in-errors: closed words only, which matches this chunk's rule of recording readings as closed words.

@history_item n=28 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Brand Identity) — removed `budget-paused` from decision/fixed-phraseology-vocabulary. This is the current vocabulary the rows hold.

@history_item n=29 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Surface: cli, 4 rows) — cli-exit-codes/10-14 now says 11 is retired and never reused. No new code may take 11.

@history_item n=30 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — refusal-hints/refusal-line now says `send` and `answer` share hints for the same reason or detail, and a not-delivered refusal with an opaque detail or none prints no hint. `send`'s `unverified-cli` now prints its hint. This bears on readings on the unstamped 2.1.326.

@history_item n=31 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the Git Bash rewritten-path warning retired) — the stdin/`--file` prompt rule stands on its own, and `send` writes no warning line. This is the source of decision/prompt-text-from-stdin-or-file: rig sends pass text by stdin or `--file`.

@history_item n=32 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0, brand, surface and bans scoped to the CLI) — scoped the spinner/progress-bar/live-redraw ban to a verb's output and fixed the CLI brand direction. This is the source of no-spinners-in-verb-output and the brand overview. Any progress a live reading or verify run shows stays the step counter.

@history_item n=33 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap is built without `color`, so every styled byte is viola's own SGR module's. This supports plain-output-modes and the no-SGR rule for `viola run`/`hook`.

@history_item n=34 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — turned the masters into rows verified against the code. Every picked row's handle dates from here. A row this chunk's readings contradict is amended by handle (`~`/`+`/`−`), not by section prose.
