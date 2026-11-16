@history_item n=1 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — added `+ decision/api-error-turn-end-shown-as-turn-end` (PROVISIONAL, delegate's word, founder's owed): an API-error turn ends as a plain `turn-ended` with only the closed `cause` word `api-error` in `--json` data, no new status, colour or wording. This is the closest precedent for this chunk. A wake on `turn-ended` (carrying `background_tasks`) must keep the same event form and must not add any new output word for a background-held end or a notification. The row's guard `list_status_reads_idle_after_an_api_error_turn_end` sits next to the wait tests.

@history_item n=2 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — cli pattern 3. Every non-dialog kind, `session-end` included, takes the `turn-ended` event form `<kind>  <name>  <time>  cursor <n>`. An event with no `dialog_id` prints `dialog unknown`. Message mode escapes every control character except `\n`/`\t` as `\xHH`, and `--json` stays serde-escaped. This is where `contract/viola-wait-output/event-form` and `dialog-form` come from. Any kind that newly wakes `wait` (a `question` raised at a background-held turn end) prints through these forms, with no new form.

@history_item n=3 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `dialog-expired` takes the dialog form, and the time form covers "every other kind without a `dialog_id`" (it used to say "every other non-dialog kind"). Precedent: whether `dialog_id` is present decides the form, not the kind's class. This matters if the chunk documents more wake kinds.

@history_item n=4 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified `wait`'s `dialog-expired` line and dropped its provisional mark. The `contract/viola-wait-output/dialog-form` wording is now settled and must not be reworded by this chunk.

@history_item n=5 marker=2026-10-08-first-live-test-and-self-drive
**2026-10-08-first-live-test-and-self-drive** — the `input-not-ready` hint dropped `viola wait <name>, then send again` because "a `viola wait` woke on nothing" when no turn was running. This is a founder-worded trap about wake semantics: a hint must never steer a driver into a `wait` that has nothing to wake it. The no-false-wake clause rests on it.

@history_item n=6 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (brand, surface and bans scoped to the CLI) — the status guards keep `turn-ended` (`decision/turn-ended-never-fills-readback`) and the no-spinner `busy`. The spinner / live-redraw ban now applies to "a verb's output" (`decision/no-spinners-in-verb-output`: `wait` prints one static `waiting:` line). A `wait` that re-parks after a notification still prints nothing more than its one waiting line and one result line.

@history_item n=7 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the six frame verbs' strict-modes refusal; a home under a regular file) — `wait` checks instance strict-modes first, before any frame. A refusal prints the instance-state pair, exits 21, writes no line, and under `--json` prints the `instance-unreachable` document. This is the source of `contract/frame-verb-refusals/strict-modes`, `contract/strict-modes-refusal/*` and `contract/strict-modes/home-under-regular-file`. Any wait-feed change must keep that order.

@history_item n=8 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — `wait` is one of the frame verbs that verify the server before any frame and write `server-verify-failed` to the log at exit 21. The hint is plain ASCII with no pid or path. A new wait path must go through the same connect.

@history_item n=9 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (server verification across eight frame verbs; the message-mode escaper shared by `wait`, `last` and `link`) — this widened the frame-verb set to eight and made pattern 3's escaper shared (`contract/cli-message-escaping/control-characters`). Changing the escaper for `wait` changes it for `last` and `link` too.

@history_item n=10 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's document names its cause; the `endpoint-lost` hint; how a frame verb reads no endpoint; the catch site's `internal-error` document) — exit 21 `--json` carries `detail` from a closed cause set. `endpoint-lost` is "a wrapper lost with the call open", which is exactly a parked `wait` whose wrapper dies. A failed connect reads `instance-dead`, and the unwrapped reading is by instance directory. Silent catch sites are listed. This is the source of the `contract/exit-21-*` and `contract/refusal-hints/*` rows. A long-parked `wait` must still end on `endpoint-lost`, not hang.

@history_item n=11 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder confirmed the `endpoint-lost` hint wording and the unwrapped-name reading. Both rows are settled wording that `wait` prints unchanged.

@history_item n=12 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a refused socket directory at the eight frame verbs; `socket-dir-refused` as exit 21's sixth cause) — `wait` refuses a socket directory before any frame, prints the `socket-dir` hint, and under `--json` reads `socket-dir-refused`, which is also a log code. This is the source of `contract/frame-verb-refusals/socket-dir` and `contract/exit-21-*cause/socket-dir-refused`.

@history_item n=13 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** (the socket-directory pair's wording confirmed) — the founder settled the wording of `contract/refusal-hints/socket-dir`. The pair is closed to edits.

@history_item n=14 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (cli Platform narrowed to Linux; VT enable retired; server check without the macOS carve-out; strict-modes readings without an OS arm; verify's relied count; stale-row witness on `ubuntu-latest`) — `decision/linux-terminal-platform`: CI is `ubuntu-latest` alone. The colour decision order lost its Windows rung, and the server-verify cause has no OS qualifier. `LedgerRow::relied()` is 24 on every build, so the `stamped_home` fixture's `viola verify` runs all 24 rows. That is the cost behind the CI carry's ~5 s / 20 s-kill timing.

@history_item n=15 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — verify prints one step line per relied ledger row, counted against `MM` = `LedgerRow::relied()`. This is the rule behind `contract/viola-verify-output/step-line` and `summary`. If the CI-carry fix stamps a home once and shares it, it still has to produce a whole `MM`-row stamp.

@history_item n=16 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a `verify` stopped by Ctrl-C or TERM prints no step line and no summary, only the fixed pair, and exits 1 (`contract/viola-verify-output/stopped-or-refused`). This bears on option (b) of the CI carry: a fixture `verify` killed at the nextest `ci` timeout ends this way.

@history_item n=17 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (verify's home-tree and stamps refusals) — `verify` checks its home tree before its log, and a refused `stamps.json` update prints the home-state pair (`contract/viola-verify-output/stamps-refused`). A shared or copied stamped home (CI-carry option a) has to keep owner-only modes, or every test using it hits this refusal.

@history_item n=18 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — this is the source of `contract/viola-verify-output/help` and `contract/viola-verify-refusals/unclean-fixture` (a named unclean fixture with closed codes, never the content). A fake-agent fixture or turn script added under `fixtures/` must stay clean for these codes.

@history_item n=19 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (exit 2's fixed stderr lines; the `--json` failures the witness drives) — the exceptions to `decision/json-one-document-on-stdout` (exit 2 for `answer` and `send`), and `decision/json-failure-documents-clean`, guarded by `tests/cli_error_bodies.rs`. `wait --json` failures fall under that witness: one document, no ESC, path or upstream text.

@history_item n=20 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — this built the colour layer: amber only on `DIALOG`, dim on `stale` rows, `NO_COLOR` read as present, width from `host_size()`. This is the source of `decision/cli-colour-amber-and-dim-only`, `contract/cli-colour-tokens/*` and `contract/cli-colour-decision-order/*`. `wait`'s output stays uncoloured.

@history_item n=21 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — clap is built without `color`, so every styled byte comes from viola's hand-written SGR module (`decision/hand-written-sgr-module`). No new output may pull in a styling dependency.

@history_item n=22 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `viola list` STATUS reads `idle` or `busy` from the `claude agents --json` join. This is the STATUS that `decision/api-error-turn-end-shown-as-turn-end` pins to `idle` after a turn end. A background-held turn end may change what STATUS reads, and the row requires that to stay unmarked.

@history_item n=23 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — the end-to-end witness of `decision/stale-rows-sgr-dim`, `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`, which runs under an outer PTY. Entry 14 above moved it to `ubuntu-latest` alone. It is a neighbour in the slow-PTY test class the CI carry concerns.

@history_item n=24 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` is a hidden verb that writes only MCP frames on stdout (`contract/mcp-verb/stdout`, `contract/cli-colour-decision-order/viola-mcp`). The MCP `wait` tool in `crates/viola-mcp/src/tools.rs` is in scope, and any documented wake behaviour must hold the same over MCP.

@history_item n=25 marker=2026-11-13-rows-proposal-rule-work
**2026-11-13-rows-proposal-rule-work** — `~ contract/refusal-hints/refusal-line` (a shared hint for `send` and `answer`; no hint for an opaque or absent detail) and `~ contract/exit-21-json-cause/unwrapped-name` (a symlinked instance dir reads `strict-modes-failed`). Both are picked rows on `wait`'s refusal path, and their current wording is what `wait` must keep.

@history_item n=26 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Brand Identity and Surface: cli rows) — `~ decision/fixed-phraseology-vocabulary` dropped `budget-paused`, and `~ decision/no-hint-names-viola-release` dropped it from its why. Precedent: a retired word leaves the vocabulary row. Any wake or notification wording must come from the remaining fixed words.

@history_item n=27 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the masters became rows, each checked against the code. Every picked row's handle dates from here, and a change this chunk makes to the wait contract lands as `+`/`~` row entries in this form.
