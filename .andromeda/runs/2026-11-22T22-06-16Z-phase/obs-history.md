@history_item n=1 marker=2026-10-10-session-links
**2026-10-10-session-links** (a link has no line and no span): this entry set how a link is traced. The implicit link, recorded on an issued `send` or a taken `answer` whose `from` names another instance, has no process-log line, no `event` value and no span. Its trace is the call's channel pair, the `link` product line `{driver, driven}` (source wrapper) and the snapshot's `links`. The entry also added an unspanned start step, the link set's restore through `viola_state::events::read_from` before the endpoint serves, and had the replay derive the link set through `apply_link_line`. For this chunk: it is the precedent for a wrapper-written product line at the start with no process-log echo (`decision/implicit-link-untraced`, `decision/implicit-link-on-send-and-answer`). The lineage line and the link set are both rebuilt by the same replay pass, so lineage must be kept apart from links (matrix#v2-10).

@history_item n=2 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (E3: the explicit methods served, the hold retired): the driven wrapper now serves `link` and `unlink`. Each reads `driver` and `driven` through `ViolaName::try_new`. A bad name or pair gets `-32602` with no line. `link` appends one `link` line for a new pair, and `unlink` appends one `unlink` line only when it removes a pair. Neither method opens a span or writes a process-log line of its own. For this chunk: these are the "observed links" it must draw apart from parent edges (`contract/events-log/link`, `contract/events-log/unlink`). The name check at the method is the existing parse that "name-checked" display extends.

@history_item n=3 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link pair names in the PII table): link pair names are classed Low. They are allowed only in the wrapper's `link` / `unlink` lines in `events.ndjson`, never in a process-log line. The channel pair logs `method`, never params. The guard is `link_verbs_log_the_channel_pair_and_no_names`. For this chunk: this binds `decision/link-names-never-in-process-log` and `decision/link-names-not-in-process-log`. A parent name in a `lineage` line, or in a list tree, follows the same class: product stream and stdout only, never a role-file line. The scope's question of what lands in a file another session can read starts here.

@history_item n=4 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link verbs log into the driven instance's `cli` file, inside `cli.link` / `cli.unlink`): the link verbs log as `cli` into `cli-<driven>.ndjson` once their instance check passes, never into the caller's file. A refused check opens no log. For this chunk: it sets which instance a cross-instance verb logs under (`decision/instance-source-per-role`, `decision/cli-log-only-under-viola-name`). A launch-time parent declaration must not write into the parent's role file.

@history_item n=5 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** (the link verbs' instance check before their log: a refused check writes no line): `link` / `unlink` run `check_instance` on the driven instance before any role file opens. A refusal is exit 21 with no line, because opening the role file narrows an existing home to 0700. For this chunk: this is the order trap behind `decision/instance-check-before-log-init` and `decision/refused-check-writes-no-line`. Any new parent read at launch must sit after the passed checks and must not open another instance's files first.

@history_item n=6 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** (viola list's lines, its no-line refusal and the replay's second caller): `list` (span `cli.list`) opens `cli-<name>.ndjson` only after its scan passed and only under a `VIOLA_NAME`. A refused scan writes no line. `board::read` became the replay's second caller, writing `state-recovered` with the caller's instance. `board::read` also logs `parse-rejected{state-entry}` counts naming no entry, and `board::scan` logs nothing. For this chunk: this is the standing log shape of the board read model that fleet trees extend. `decision/cli-log-only-under-viola-name` and `contract/state-recovered/scanned` rest on it. The tree drawing must add no line that names an instance or a parent.

@history_item n=7 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** (no-line refusals: the statusline arm's and the board's are the founder's): `viola list`'s refused scan, which opens no process log and writes no line, is the founder's own answer. For this chunk: the board refusal shape any tree-drawing change must keep as ruled (`decision/refused-check-writes-no-line`).

@history_item n=8 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** (revive --list logs as cli; exemption 6 retired; state-recovered names the scanned instance): `viola revive <name> --list` is role `cli`, logged under the caller's `VIOLA_NAME` after its check. `state-recovered` gained `scanned`: the instance dir's own last component when it parses as a `ViolaName`, absent otherwise, never a path. Revive's `--json` refusal documents were also set. For this chunk: `contract/state-recovered/scanned` and `decision/cli-log-only-under-viola-name` come from here. It is the precedent for naming a scanned instance only after it parses as a viola name, the same rule the scope sets for showing link names.

@history_item n=9 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (revive in the log: role `run`, the preflight, the replay's first product caller): revive's start arm is role `run` and logs into `run-<name>.ndjson` (`decision/revive-start-logs-as-run`, `contract/cli-log-file/revive-start`). `read_snapshot_or_replay` got revive as its first product caller, and `replay::session_chain` logs nothing. For this chunk: "parent kept by revive" lands on this path. The revived start writes into the same run log and reads the same replayed state, so the lineage it keeps must be read back from `events.ndjson` through the replay.

@history_item n=10 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (a refused instance check writes no line, on both arms): `viola revive`'s start arm runs its instance check before the log init, and a refusal opens no log and writes no line. For this chunk: this is the source of `decision/instance-check-before-log-init` on revive's start. Any parent handling on revive must come after that check.

@history_item n=11 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** (where `state-recovered` is written and how often): this entry set the replay's `state-recovered{snapshot-replayed}` at WARN, one line per call, and the appender's `torn-line-healed` line, one per heal, with the reader never healing. For this chunk: the scope requires the `lineage` line to read back the same after a crash and a replay. These rows bind how that replay and a torn tail are logged: `decision/appender-heals-torn-line`, `decision/reader-never-heals-torn-lines`, `contract/state-recovered/torn-line-healed`, `contract/state-recovered/snapshot-replayed`.

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers over a replayed snapshot): a snapshot rebuilt from `events.ndjson` carries no `endpoint`, `pid`, `started_at` or `statusline_command`. Hook connect, `live_endpoint` and the collision check treat it as unreachable or heartbeat-only. For this chunk: this backs `contract/replayed-snapshot/live-fields` and `decision/replayed-snapshot-unreachable`. Whatever lineage the snapshot carries must be derivable from the event log alone, or it will be missing after a replay. The list of replay-absent fields is a register this chunk may extend.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5: the replay's product readers two to six, `state.snapshot_recover` placed): six product readers use `read_snapshot_or_replay`, and no product caller of raw `read_snapshot` remains. `state.snapshot_recover` sits under `run.start` before `run.collision_check`. For this chunk: this backs `decision/no-raw-snapshot-reads` and the start span chain (`contract/run-start-spans/span-chain`). A new lineage read on list, revive or run must go through the replaying read.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check of `run` and `viola revive`'s start, before the wrapper log): `checked_exe` and `check_home_tree` run before the log init. A refusal opens no log, writes no line and starts no child. For this chunk: this backs `decision/home-tree-check-before-start-log` and `decision/home-tree-check-before-log-init`. A launch-time parent declaration must not create or open anything under the home before these checks.

@history_item n=15 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the refusal-telemetry record: two strict-modes lines, every other refusal none): Founder Direction 4's distinct-code rule gained its carve-out. A strict-modes check refused before any log opens carries no logged code. For this chunk: this backs `decision/distinct-detail-code-per-exit-cause` (except) and `decision/strict-modes-refusal-writes-no-line`. A new launch refusal (for example a bad parent declaration) needs a distinct, path-free code unless it is such a pre-log check.

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (Founder Direction 4's codes and named carve-outs): the trigger names every exit-21 and exit-1 code. For this chunk: this is the closed exit-1 register (`decision/exit-1-detail-codes`, `decision/detail-codes-no-paths`). A new start refusal tied to lineage adds a word here and to the `process-exit` `detail` enum, a fork for the operator.

@history_item n=17 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder's code is `stale-heartbeat`): `run` and `revive` log `stale-heartbeat` for a stale holder and `already-live` for a live holder or a fresh replayed beat. A `gone` holder is taken over with no line. For this chunk: this backs `contract/run-start-failure-detail/stale-heartbeat`, `contract/run-start-failure-detail/already-live` and `contract/run-span-attributes/run-collision-check`. A reused name taken over after `gone` is the case where "a reused name never inherits an old parent" must hold.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (every refusal of `revive --json`'s start prints its document): every refusal of revive's start prints its closed document under `--json`, with the logged line unchanged. For this chunk: revive's machine surface is closed, so a parent kept or refused by revive adds to that document set only as a fork.

@history_item n=19 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** (`already-live` covers the stale refusal; `gone` is taken over): this entry first set the collision outcomes and the start's product order. For this chunk: it is the origin of the start product order that a `lineage` line will join (`contract/run-start-events/order`).

@history_item n=20 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** (session-start is record three): `session-start{source:"hook"}` was fixed as the start's third record. For this chunk: this is the record numbering a new start-time `lineage` line could shift. It is the history of `contract/run-start-events/order` and `decision/product-order-from-events-log`.

@history_item n=21 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** (boot readiness checks the start records): the harness `boot` stage `start_records` checks lines 1-3 of `events.ndjson`, with missing code `<name>:events`. This is a trap for this chunk: a `lineage` line written at launch changes what the first start lines are. The boot check and the start-order rows (`contract/run-start-events/order`, `decision/product-order-from-events-log`) must be re-read when the line is placed.

@history_item n=22 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (4. Span / Trace Coverage, the start's product order and span chain): `budget-gate` left the start order, which became `wheel{cause:"start"}` then `session-start`, and `run.budget_eval` left the start span chain. For this chunk: this is the last precedent of an event kind leaving the start order. `contract/run-start-spans/span-chain` and `decision/product-order-from-events-log` now read without it, and a `lineage` record placed in the order amends the same rows.

@history_item n=23 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Obs Scope Summary, `contract/run-start-events/order`): the run start order promise was rewritten to `wheel{cause:"start"}` then `session-start`. For this chunk: this is the row a start-time `lineage` record extends.

@history_item n=24 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (the additive-only rules narrowed): `decision/diag-schema-additive-only`, `decision/log-fields-additive-only`, `decision/log-schema-additive-only` and `decision/obs-aligns-to-tests-log-format` were narrowed by the operator to «must not rename or remove a field the product still emits». For this chunk: new `lineage` fields and event values are additive and allowed through a Decisions Log entry. Nothing the product emits may be renamed.

@history_item n=25 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (the start order with folder-trust keys): `contract/run-start-events/order` and `decision/product-order-from-events-log` now name `wheel{human-input}` before `session-start` where trust keys pass through the wrapper. The order is read from that start's own lines, not from the log's first lines. For this chunk: this is the current text of both rows a launch-time `lineage` line must be placed into.

@history_item n=26 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (`decision/events-stream-never-rotated`): `events.ndjson` is never rotated or rewritten. A reused instance name keeps the earlier starts' lines at its head, so a start's own lines are read from where that start begins. This is a trap for this chunk: «a reused name never inherits an old parent». An earlier start's `lineage` line stays in the log, so the reader must take the parent from the current start's own lines only, never the first `lineage` line in the file.

@history_item n=27 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (the reserved web vocabulary leaves): the `process` enum closed at `run|hook|mcp|cli`, and no process logs a liveness transition. Short-lived readers expose `liveness` in `list --json`, with the flip at a heartbeat older than 5 s. For this chunk: this backs `contract/list-json/liveness` and `decision/heartbeat-staleness-flip`. `list --json`'s existing liveness field must survive the tree payload change.

@history_item n=28 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** (stall detection reads an absent instance as degraded): `agent-run status` runs `viola list` and reads `state: degraded` when an instance is `stale` or absent from it. This is a trap for this chunk: the harness parses `viola list` output. Reshaping list into trees, or changing `list --json`, must keep what `status` reads.

@history_item n=29 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (session-live, the agents-probe callers): `run::agents::read_logged` writes the `agents-probe` pair for `list`, only under a parsed `VIOLA_NAME`. For this chunk: this is the existing child-spawn telemetry on the list path that the fleet tree drawing sits beside.

@history_item n=30 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the mcp list tool's lines): `viola mcp`'s `list` tool runs the same board read and writes its `agents-probe`, `state-entry`, `state-recovered` and `config-json` lines into `mcp.ndjson`. For this chunk: a change to the board read model (parent and link fields) also reaches the MCP list tool's payload and log lines.

@history_item n=31 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** (the refusal writer's callers named by verb): `list`'s colour layer adds no stderr writer, and its `unable` / `hint:` pair carries no SGR. For this chunk: this backs `contract/list-stderr/unable-hint-pair`. The «plain when piped» tree form keeps refusals SGR-free whatever the stdout terminal state.

@history_item n=32 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** (run's pre-spawn stderr names every start refusal and the one writer): every start refusal is two fixed stderr lines through `human::refuse` → `write_refusal`. For this chunk: this backs `contract/run-terminal-output/start-refusal` and `decision/stderr-fixed-strings`. A bad parent declaration at `viola run` is refused through this writer with no path or pid.

@history_item n=33 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (3. Observability Harness Contract, `contract/run-terminal-output/child-output`): run may write to the terminal, before the child, only a keyed start's automatic-verify lines and the verify child's output, none under `revive --json`. While the child runs it writes nothing. For this chunk: a launch flag for the parent adds no terminal output of its own (`decision/run-silent-terminal`).

@history_item n=34 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (4. Span / Trace Coverage, `contract/run-process-log/auto-verify-stop`): a stop during the start's automatic verify ends the start with no `process-exit`, no stamp, no snapshot and no session. For this chunk: a start ended in the verify window must write no `lineage` line, so the line belongs after the point where the session is committed.

@history_item n=35 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the start's socket-directory step, no span and one line on a refusal): the start's fourth unspanned step, `socket_dir_ready`, writes `process-exit{exit_code:1, detail:"socket-dir-refused"}` before any snapshot, child or bind. For this chunk: this backs `contract/run-start-failure-detail/socket-dir-refused` and `contract/exit-cause-detail/socket-dir-refused`. It is the latest precedent for adding an unspanned start step and placing it against the snapshot and child.

@history_item n=36 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (6. Log Coverage, `contract/start-trust-log/trust-record`): the start gained a new closed event value, `trust-record`, as its trust step's one telemetry line, with no span. Both schemas' event enums carry it, and `contract/diag-line-fields/trust-record` holds its fields. For this chunk: this is the nearest precedent for a new start-time record. A new `event` value needs its Decisions Log entry and a per-event field row (`decision/event-values-closed`, `contract/process-log/event`). Note that the scope's `lineage` is a product `events.ndjson` kind, a separate register in `viola-core`.

@history_item n=37 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (1. Obs Scope Summary, `decision/claude-internals-not-a-contract` and `decision/r8-strip-logged-as-names-only`): the start reads the kept `CLAUDE_CONFIG_DIR` value and sets its folder's trust flag. These are the bounded exceptions to the claude boundary and the env-value ban. For this chunk: a parent declared from «the launching session's own identity» must not read claude internals or env values beyond these named exceptions. `decision/instance-from-name-not-cwd` and `decision/instance-source-per-role` (run never takes an inherited `VIOLA_NAME` as its own name) bound what a starting wrapper may read of its parent.

@history_item n=38 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (1. Obs Scope Summary, `decision/claude-internals-not-a-contract` rule): the wrapped claude's internal state, transcripts and bundle strings are never read as a contract. For this chunk: this is the current rule text of a picked row. A parent's identity cannot be taken from claude's transcripts or internals.

@history_item n=39 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (1. Obs Scope Summary, `decision/deterministic-event-emission`): events key on hook events and `events.ndjson` offsets, with the one exception of the transcript echo count. For this chunk: a `lineage` event written once at launch must key on the start, not on screen or transcript content.

@history_item n=40 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the closed words that left `diag-line.v1.json` with their emitter): this added the rule that a closed schema word no build can emit leaves the schema with its emitter (`decision/unemittable-schema-words-removed`). For this chunk: any closed word it adds must have an emitter in this chunk.

@history_item n=41 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** (6. Log Coverage: new subject and parser words): `hyprctl-probe` and `hyprctl-json` joined their closed enums, each word as its own contract row. The span-naming area list gained `hyprland`. For this chunk: this is the fleet-epoch precedent for adding closed words one row per word and extending `decision/span-naming` when a new span area appears.

@history_item n=42 marker=2026-09-24-diagnostics-plane
**2026-09-24-diagnostics-plane** (obs_event! field set, schema default-deny keyword): `obs_event!` attaches `event`, `process` and `instance`, and `corr` is caller-supplied. Default-deny is enforced by a top-level `unevaluatedProperties: false`. For this chunk: this backs `contract/obs-event-macro/attached-fields` and `decision/schema-top-level-unevaluated-properties`. Any new field on a log line must be named in the per-event list or the schema rejects it (`decision/default-deny-log-fields`).

@history_item n=43 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (channel-response's detail closed to three words): `channel-response` `detail` is closed to three words. A bad `link` / `unlink` name or pair reads `invalid-params`, and the pair holds no name and no param. For this chunk: link name checks surface only as closed codes. Showing a link name only after it parses must add no name to any log line.

@history_item n=44 marker=2026-11-08T20-32-52-wrap
**2026-11-08T20-32-52-wrap** (version census 0.2.0, the frontend scenario stub replaced): viola has no web or graphical frontend. The terminal fleet console's scenarios (matrix#v2-14) are owed by the route entry «Console scaffold». For this chunk: the boundary of the fleet view. List trees are a CLI board surface, and console scenarios are not this chunk's (`absence/web-or-graphical-frontend`, `absence/interactive-terminal-interface`).

@history_item n=45 marker=2026-11-12T03-23-49-rows
**2026-11-12T03-23-49-rows** (the masters as rows): the obs plan's text became rows, each verified against the code. For this chunk: the picked rows are this conversion's output. An area whose text was dropped («as the code shows») is read from the code, not from a row.
