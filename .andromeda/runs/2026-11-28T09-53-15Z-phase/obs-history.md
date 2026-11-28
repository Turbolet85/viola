@history_item n=1 marker=2026-11-28-console-scaffold
**2026-11-28-console-scaffold** — (all its sub-entries) gave the console its role: `console` joined the process enum (+ contract/diagnostics-files/console, ~ contract/diag-line/process, ~ contract/log-line/process, ~ contract/process-log/process, ~ contract/process-log-line/process, ~ contract/diagnostics-line/role-file-line-size, ~ contract/diagnostics-files/paths, ~ decision/log-file-basenames-follow-tests, ~ decision/structured-json-process-logs, ~ decision/zero-unlogged-panics), named `viola console` as the except of decision/no-process-log-without-instance (home-level console.ndjson, no instance key), added + contract/diag-line-fields/console-frame and + contract/diag-line-fields/console-key, + contract/panic-outcome-by-role/console, + contract/console-output/stdout, and replaced − absence/interactive-terminal-interface with + decision/no-interactive-terminal-interface (the console its one named exception); ~ decision/distinct-detail-code-per-exit-cause excepts the console's terminal check as lineless. This is the register the view extends: `console-frame` gaining `shown` / `hidden` is an edit of the row this entry landed, and the board and agents lines the view writes land in this role file with no instance. Most rows carry the delegate overseer's PROVISIONAL mark (inputs#I2 Q3, I4, I5).

@history_item n=2 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — set where the board read writes: `board::read` is the call site of the one `parse-rejected{parser:"state-entry", detail:"symlink-ignored"|"malformed", count}` per nonzero scan count, naming no entry, and `board::scan` logs nothing; `board::read` became the replay's second product caller, writing `state-recovered` per unreadable or newer snapshot into the caller's role file with the caller's instance; `list`'s refused strict scan writes no line; `read_thresholds` read under `cli.list`. The console now calls this same board read once a second, so these lines (decision/symlink-skip-logged-as-count, contract/state-recovered/*) reach console.ndjson at that rate unless a change-only rule holds them.

@history_item n=3 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — made `viola list`'s refused scan, which opens no process log and writes no line, the founder's own answer. This is the precedent for how a board-read caller treats a strict-modes scan refusal, and the console's re-read loop has to choose a shape for that refusal.

@history_item n=4 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the mcp list tool's lines) added a second caller of the board read and the agents read that is not `cli`: the `agents-probe` pair and the board read's `parse-rejected{state-entry}`, `state-recovered` and per-key `config-json` lines go into `mcp.ndjson` for that tool (was `cli-<name>.ndjson` under a `VIOLA_NAME` only). This is the direct precedent for the console writing the same lines into its own role file with no instance (scope sec-mrt47foy).

@history_item n=5 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `run::agents::read_logged` writes the `agents-probe` pair for `list`, `revive`'s session-live reading and verify's Run B, plus one `parse-rejected{parser:"claude-agents-json"}` on an oversize or malformed answer; `oversize` became a `claude-agents-json` detail; `run_bounded` / `run_capped` stay unlogged. The console's agents read (at most every 5 s, off the loop) inherits these lines and their rows (contract/diag-subject/agents-probe, contract/parse-rejected/claude-agents-json, contract/child-spawn-log/claude-agents-json-parse-failure), so it writes a pair per read.

@history_item n=6 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** — (6. Log Coverage, 3 rows) the last edit of ~ contract/diag-subject/agents-probe, whose promise lists its writers by name (viola list, the session-live reading of revive and adopt, verify's Run B, the mcp list tool). A console agents read writing the pair into console.ndjson makes that closed list incomplete, so the row needs a set.

@history_item n=7 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — ~ decision/symlink-skip-logged-as-count gained its except: «viola up, which never takes the strict board read, writes no parse-rejected state-entry line». This is the one standing exception to the once-per-scan count and the precedent for the console's F7 exception (the skip-count WARN line only when the count changes), which has to sit beside it in the same except. The same entry also set ~ contract/process-log/child-and-probe-lifecycle (the terminal of open / up gets a process-start alone).

@history_item n=8 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — fixed the `state-recovered` emitters and shape: `snapshot-replayed` / `snapshot-unsupported-v` at WARN with no corr, one line per call, none for a present or absent snapshot; the reader's per-read skip counts have no process-log carrier, and `list --json` is named their only carrier. A console re-read each second makes «one line per call» a per-second line for an unreadable snapshot. This is the trap behind contract/state-recovered/snapshot-replayed and decision/no-per-heartbeat-logging.

@history_item n=9 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `state-recovered` gained `scanned` on the two snapshot details (the scanned instance's name when it parses, never a path or literal null), and the E5 line carries `instance` the caller plus `scanned`. In console.ndjson there is no caller instance, so `scanned` is the only name on the line (contract/state-recovered/scanned).

@history_item n=10 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (E5, the replay's readers two to six; the `diagnostics/` opens behind `check_diagnostics`) `read_snapshot_or_replay` became the read of every product snapshot reader, each writing one `state-recovered` per call. A role or detail file now opens only past `check_diagnostics`, and a refusal is a failed open ending at `std::io::sink`. Both bear on the console: its board re-read multiplies replay lines, and its console.ndjson open goes through the same check.

@history_item n=11 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — retired `liveness-changed` (D-02): no process logs a liveness transition, `run` touches `heartbeat` every 1 s, and readers expose `liveness` in `list --json` with the flip at more than 5 s. The console shows LIVE live·stale from the board and must not log a transition when a row flips (decision/liveness-transitions-not-logged, decision/heartbeat-staleness-threshold, decision/no-per-heartbeat-logging).

@history_item n=12 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** — §4's frontend stub was replaced: there is no frontend scenario because viola has no web or graphical frontend, and the terminal fleet console's scenarios (matrix#v2-14) are owed by «Console scaffold» (census C-312). The view chunk is where that console must-trace coverage gets settled, and decision/no-human-only-dashboards and absence/web-or-graphical-frontend still stand.

@history_item n=13 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — + decision/lineage-names-confined: a start's parent and role reach only events.ndjson, the snapshot, the viola list board, list --json and the MCP list result, never a process-log or detail line. ~ decision/link-names-never-in-process-log kept link names out of process logs. The console draws lineage trees from the board, so it is a new surface showing parent names that the row's closed list does not name. Its console-frame lines must carry counts only.

@history_item n=14 marker=2026-11-27T19-07-11Z-wrap
**2026-11-27T19-07-11Z-wrap** — (8. PII Scrubbing & Compliance) ~ decision/lineage-names-confined state provisional → standing on the operator's own word. The confined-surfaces list is ratified text, so adding the console as a surface is a row edit for the operator, not a delegate.

@history_item n=15 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — ~ decision/events-stream-never-rotated: events.ndjson is never rotated or rewritten, and a reused instance name keeps the earlier starts' lines at its head. The view's LAST cell (the age of the newest events.ndjson line) has to read the log's tail, and a reused name's head holds stale lines.

@history_item n=16 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — (the four «Сузить правило» sets) narrowed decision/diag-schema-additive-only, decision/log-fields-additive-only, decision/log-schema-additive-only and decision/obs-aligns-to-tests-log-format to «must not rename or remove a field the product still emits». Adding `shown` / `hidden` to `console-frame` is additive under these rows. Removing or renaming `cols` / `rows` / `fits` is not allowed.

@history_item n=17 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** — `obs_event!` attaches `event`, `process` and `instance` from `ProcessCtx` (corr caller-supplied), and default-deny is a top-level `unevaluatedProperties: false` over per-event field lists. The new `console-frame` fields must be declared in `schemas/diag-line.v1.json`'s per-event list or G4 fails (decision/schema-top-level-unevaluated-properties, contract/obs-event-macro/attached-fields).

@history_item n=18 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — retired the inner allow: `clippy.toml` bans `tracing::{info,warn,error,debug,trace}` by path, and raw `event!` is caught by the fail-closed grep in `scripts/lint-probes.sh`. Any new console line goes through `obs_event!` with explicit typed fields (decision/source-allow-list-logging).

@history_item n=19 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (D-05 `parse-rejected` gains `mcp-stdin`) recorded that a new closed value or field takes a Decisions Log record, and that log lives in this amendments file. The `console-frame` `shown` / `hidden` extension (T2) needs the same record (contract/process-log/event, contract/process-log-line/event).

@history_item n=20 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — ~ decision/panic-caught-at-every-role moved the run PTY threads under catch_unwind from two to three when the paste writer thread was added. This is the precedent that a new worker thread joins the named catch sites. The console's off-loop agents read thread (F6) needs its own catch_unwind and a named place in that row, or a panic there leaves the loop running with no record.

@history_item n=21 marker=2026-10-04-readiness-gate-and-timing-constants
**2026-10-04-readiness-gate-and-timing-constants** — set the shape of a contained detached worker: the vt100 feed thread's per-call catch_unwind writes the `panic` role line plus one `parse-rejected` and sends nothing to `main`. This is a model for how the console's background agents reader can contain a panic without ending the console.

@history_item n=22 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — gave decision/parse-failures-surface-as-unknown and decision/parse-failures-unknown-counter their hyprctl except, and named the hyprctl replies in decision/per-parser-parse-failure-counts and the hyprctl spawn in decision/instrument-call-sites-not-os-internals. The console view dispatches nothing to the compositor, so it adds no hyprctl line. Its agents parse failure stays on the `unknown`-plus-counter path.

@history_item n=23 marker=2026-11-16-granular-api-error-cause
**2026-11-16-granular-api-error-cause** — the last edit of the excepts of decision/parse-failures-surface-as-unknown and decision/parse-failures-unknown-counter: the StopFailure outside word reads api-error. The board's STATUS `unknown` (from a failed agents read) is still the rule's value, and the view must show it verbatim.

@history_item n=24 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** — the last rule edit of ~ decision/claude-internals-not-a-contract: internal state, transcripts and bundle strings are never a contract. The view reads claude only through `claude agents --json`, which is instrumented at its boundary and untrusted (decision/agents-cli-output-untrusted, decision/claude-agents-rows-never-logged).

@history_item n=25 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — the current except of ~ decision/claude-internals-not-a-contract (the echo tag count, verify's trust record, a verified start's trust flag). No other read of claude internals is admitted, so the view's LAST must come from events.ndjson and not from a session transcript.
