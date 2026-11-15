@history_item n=1 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — set `viola verify`'s output structure (step lines `[NN/MM]`, last stdout line `stamped <ver>  <n> pass  <m> fail`, failing-row run exit 1 with the stamp still written, usage `viola verify [--record <DIR>] [-- <program> [args…]]`) and its `unable:`/`hint:` refusal pairs; the base of the verify rows (step-line, summary, failing-row, usage, cli-not-found, version-unreadable) this round's ledger stamping on a test home and `--record` run lean on.

@history_item n=2 marker=2026-10-05-real-cli-verify-probes
**2026-10-05-real-cli-verify-probes** — added `viola verify --help`'s paragraph (run from a trusted folder; an unapproved external CLAUDE.md import blocks the probe) and renamed the dirty-recording refusal to `a recorded fixture is not clean: <file> <code>` (screen form `<file> row <n>[ seam] <code>`, codes home-path/absolute-path/username/email, never the content); the `--record` scrub that CARRY 1's `Mcp.tools-list-1.json` re-record and CARRY 2's box-only `Screen.*` fixture must pass — the trust-dialog trap for a headless live rig too (fork 2).

@history_item n=3 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — verify grew to 14 rows with the dialog rows; Kept: verify never answers the CLI-native external-import dialog, the probe's own dialogs answered by its hook, not by verify typing — a precedent for fork 2 (a rig typing into its own pty).

@history_item n=4 marker=2026-10-06-local-command-and-paste-framing-rows
**2026-10-06-local-command-and-paste-framing-rows** — verify at 17 rows; the examples name the one stamped CLI set and the failing example came from the chunk's own first record round — precedent for how a live `--record` round's stamps land in the wireframe.

@history_item n=5 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — a verify stopped by Ctrl-C/TERM prints only `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, stdout empty, no summary, no stamp, exit 1; the summary closes only a completed run — governs the verify exit-1 and summary rows, and what a kill-deadlined live `--record` leaves behind.

@history_item n=6 marker=2026-10-13-paste-newline-ledger-row
**2026-10-13-paste-newline-ledger-row** — verify at 18 rows; `MM` became `LedgerRow::ALL.len()`, the count carrying its rule rather than a literal.

@history_item n=7 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — `MM` became the relied count (`LedgerRow::relied()`), adding the three statusline rows (`statusline-override`, `statusline-shell`, `statusline-rate-limits`), relied only where the status line is wrapped; with the status-line interception removed, these rows' fate bears on this round's stamp count and on CARRY 2 (the status-line rows that blocked a clean `--record`) and CARRY 3.

@history_item n=8 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — verify at 22 relied rows (`resume-session`), the Windows set skipping the statusline rows; part of the relied-count chain the step-line and summary rows read.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `viola list`'s unwrapped rack (`claude agents --json` join, `-- UNWRAPPED - READ-ONLY --`), revive's `session-live` pair and start order, verify at 23 rows (`agents-join`); bears on the board row (`contract/cli-verbs/list`), revive refusals and the live start's ledger.

@history_item n=10 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — verify at 24 relied rows (`local-command-compact`, 40 s PROVISIONAL window); the row count the live round's `viola verify` stamps against.

@history_item n=11 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — five sub-entries: verify's `MM` is all 24 rows on every build (`[NN/24]`, stop on `SIGINT`/`SIGTERM` only); `viola list` uncoloured under `--json`, pipe, `NO_COLOR`, `TERM=dumb` (VT enable gone); plugin install's `<dir>` single-quoted unless every character is in `[A-Za-z0-9_-./:]` (the `contract/viola-plugin-install/dir-quoting` row); the `.cmd`/`.bat` refusal left revive and verify (revive's start codes `pinned-hash-mismatch`, `squatted-name`, `socket-dir-refused`, `internal-error`); the server-check hint unscoped — the Linux-only frame for the live wrapped start (v2-03).

@history_item n=12 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — created `viola revive`'s output structure: a passed start prints nothing while the child holds the terminal; each refusal exit 1 with one `unable:` and one `hint:` last on stderr, stdout empty; `run`'s collision pair shared — the source of `contract/viola-revive/refusal-shape`, `already-live` and `state-files-writable`, and the passthrough named in `decision/plain-output-modes`.

@history_item n=13 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `viola revive` took `--json` (refusal document `{"v":1,"error":"<code>","detail":null}`, exit 1, stderr empty; pairs byte-identical without it) and `--json` became every data verb's machine view, revive included — the `decision/json-on-every-data-verb` and `contract/viola-revive-json/refusal-document` rows.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — the run/revive start's home-tree refusal (home, `ledger/`, `bin/`, `plugin/` or pinned copy → the home pair; instance → the state-files pair; exit 1, no log, no child) and verify's home pair (stamps write included); the source of `contract/run-revive-start/home-tree-refused` and `contract/viola-verify-refusals/home-not-private` — the test homes on tmpfs must be owner-only or the live start refuses before any child.

@history_item n=15 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — four sub-entries: the `revive --json` post-preflight gap closed (one exit-1 document for `pinned-hash-mismatch`, `squatted-name`, `internal-error` for a program not found or a start error; a `pump_child` error prints no document, the child having held the terminal) — the `contract/run-revive-start-json/*` rows; `stale-heartbeat` beside `already-live` under `revive --json` (a live or stale name refused, v2-03); the catch site's `internal-error` document under `--json`; exit 21's `detail` cause codes and the unwrapped-name / endpoint-lost hints (`decision/hint-for-every-cause`).

@history_item n=16 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the `unknown` refusal's detail is a code word or nothing; `unknown`, `wrapper fault` and `internal error` still take no hint — the except clause of `decision/hint-for-every-cause`.

@history_item n=17 marker=2026-11-01-t02-46-44-wrap
**2026-11-01-t02-46-44-wrap** — the founder's word superseded the provisional records of the 2026-10-31-exit-cause-code-catalogue entries; the start-refusal documents stand ratified.

@history_item n=18 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — the socket-directory refusal of the `run` and `revive` starts (exit 1 before any snapshot, child or bind, stdout empty, directory left as found; `socket-dir-refused` under `revive --json`) — the `contract/run-revive-start/socket-dir-refused` and `contract/run-revive-start-json/socket-dir-refused` rows; the live rig's socket dir must be owner-only.

@history_item n=19 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — `viola mcp` a hidden verb (only `--home`), run only by the plugin (`.mcp.json` registers the pinned path with `["mcp"]`), in no help row; its tools the CLI verbs' `--json` documents less `v`, no tool, description or result naming release/pause/link/unlink — the rows behind CARRY 1's `Mcp.tools-list-1.json` re-record (the `list` tool's description) and the plugin-run start of v2-03.

@history_item n=20 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — created `viola plugin install`'s output structure (four plain lines, the `--json` ok document, the exit-21 home refusal, `<dir>` the canonical home joined with `marketplace`) — the plugin-install rows the live round's freshly written plugin runs through.

@history_item n=21 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder's Epoch 5 batch: the `dialog-expired` line, plugin install's lines and verify's interrupted pair stand as worded, their planning-form marks gone — the wordings this round reads as fixed.

@history_item n=22 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — `viola wait` gained `dialog-expired  <name>  dialog <id>  cursor <n>` (ASCII, no SGR, written once) when the wrapper records an expired hold — the `contract/viola-wait-output/dialog-expired` row the live hold near the 3600 s `DIALOG_HOLD_MAX` ceiling (v2-02) will exercise.

@history_item n=23 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — `viola wait`'s lines as landed: `dialog unknown`, `waiting:` only on a terminal stderr without `--json`, one `--json` document, the exit-21 pair — the waiting-line, dialog and timed-out wait rows the hour-long hold rig reads.

@history_item n=24 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `viola list` as built: one uncoloured stdout write, the widths, `DIALOG unknown` cells, the `--json` refusal document — the board rows (`contract/cli-verbs/list`, `contract/viola-list-board/dialog-unknown`) the live round's name-collision checks consult (`hint: viola list`).

@history_item n=25 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder confirmed the board's forms, WHEEL 23 and the list refusal as his; the board wording stands ruled.

@history_item n=26 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — `viola list` styled on a TTY (DIALOG amber, stale dim) and uncoloured under `--json`/pipe/`NO_COLOR`/`TERM=dumb`, width from `host_size()` never `COLUMNS`; `--help` the grouped table — the `decision/plain-output-modes`, `decision/human-tty-expression-level`, `decision/dialog-attention-only-on-board` rows; a rig's own pty is a TTY, so its reads see the styled form.

@history_item n=27 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — the cli surface stands alone; `list --json`'s shapes are the list payload contract; «Shared vocabulary» fixes the six captions — origin of `decision/shared-captions-fixed`.

@history_item n=28 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — version census 0.2.0: no web surface (`absence/web-surface`), no interactive prompt or pager in a verb, the terminal fleet console no verb's output and reusing the six captions; the «on Unix» qualifiers dropped from the socket-directory refusals — the `decision/no-prompt-or-pager` and `decision/shared-captions-fixed` rows.

@history_item n=29 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** — the master became rows (226), each verified against the code; the picked rows' handles and wording come from this conversion.

@history_item n=30 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (1 of 2, 2 of 2) BAY line now `BAY  skipped a - b - c`; `contract/viola-list-json/document` lost its `budget` shape (payload `{items, skipped}`); exit 11 retired, an older wrapper's `budget-paused` reads `unknown` (14); `release --budget` lines and the board's gate/window forms removed — the trimmed product this live round runs on, and the reason CARRY 1's MCP `list` description must drop «and the usage budget».
