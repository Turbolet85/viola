@history_item n=1 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (1. Obs Scope Summary; 4. Span / Trace Coverage) — `~ contract/run-start-events/order` and `~ decision/product-order-from-events-log` put one lineage line after wheel{cause:"start"} and before session-start; `+ decision/lineage-line-untraced` makes that line log-only product audit data with no process-log line, span or terminal byte (PROVISIONAL, delegate overseer F3). Why it matters: the model/effort pair is to be recorded "beside the lineage record"; a new or widened start record moves this order contract and the untraced precedent again, and the PROVISIONAL marks are still open.

@history_item n=2 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (8. PII Scrubbing & Compliance) — `+ decision/lineage-names-confined` confines the self-reported parent and role to product surfaces (list, list --json, MCP list) and keeps them out of every log line; `~ decision/link-names-never-in-process-log` dropped its "only in the event lines" half once names reached the board. Why it matters: if the pair is shown on `viola list` / `--json` / MCP `list` (a fork in scope), it needs the same kind of PII and surface ruling, and must stay out of process-log lines.

@history_item n=3 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `~ contract/run-start-events/order`, `~ decision/product-order-from-events-log` (wheel{human-input} also before session-start on a folder-trust start; read from that start's own lines) and `~ decision/events-stream-never-rotated` (a reused instance name keeps earlier starts' lines at the head of events.ndjson). Trap: a per-instance pair record read from events.ndjson must be read from the current start's own lines, never from the log's head; a resume reading the "recorded pair" must pick the right start's record.

@history_item n=4 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — `~ contract/config-json/known-keys` went from four to five keys with verify_new_cli; `+ contract/diag-subject/auto-verify`, `~ contract/diag-line-fields/process-start`, `+ contract/run-process-log/auto-verify-stop` (all ratified 2026-11-22). Why it matters: this is the most recent precedent for adding a start-time config.json key. A default model/effort key would move known-keys to six (any other key still counted as unknown-keys, nothing refuses), and a new closed log word needs its own row.

@history_item n=5 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** — Ratified known-keys and auto-verify rows. `~ contract/run-terminal-output/child-output` now allows a keyed start's automatic-verify lines and the verify child's output before the child, none under revive --json. Why it matters: run must stay silent on the terminal. Any pair-related output (for example a refused or defaulted pair) has no terminal channel apart from the fixed refusal pair.

@history_item n=6 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (3. Observability Harness Contract; 1. Obs Scope Summary) — `~ contract/config-json/known-keys` (retired five_hour / seven_day counted as unknown-keys in one parse-rejected line, nothing refuses), `~ contract/config-json/v` (a bad doc gives the hold default and no further line), `~ contract/run-start-spans/span-chain` (run.budget_eval removed), `~ contract/run-start-events/order`, `~ decision/exit-code-log-field` (11 retired, never reused). Why it matters: these set how config keys are added and retired and what the start's span chain holds. A pair default key, plus any new start step or span, must fit them. Exit 11 must not be reused for a pair refusal.

@history_item n=7 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Q2 narrowing) — `~ decision/diag-schema-additive-only`, `~ decision/log-fields-additive-only`, `~ decision/log-schema-additive-only`, `~ decision/obs-aligns-to-tests-log-format`: removal is allowed only for a field no build emits (operator Q2, «Сузить правило»). Why it matters: any new pair field on process-start or in the schema is additive only. It goes through a Decisions Log entry and default-deny (`decision/default-deny-log-fields`, G4).

@history_item n=8 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** — run's start reads config.json a third time for dialog_hold_s, writing exactly one parse-rejected{config-json, malformed} on a bad value; CONFIG_KEYS grew. This was a boundary widening the founder answered himself (P4 fork). Why it matters: it is the template for `contract/config-json/dialog-hold-s` and for how a new start-time config value (here the default pair) is read and rejected: one WARN line, then a fallback to the default. It also shows that a new start read is a founder-level fork.

@history_item n=9 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (run.budget_eval in the start chain) — The start chain gained a span. A config re-read at the wrapper's start logged one per-key parse-rejected config-json malformed line per bad key, and CONFIG_KEYS in `src/obs.rs` was named. The list was not on the plan's expected amendments; the operator directed it at wrap. Why it matters: it is the precedent and a trap for any new start step (choosing or recording the pair). Say whether the step opens a span and changes `contract/run-start-spans/span-chain`, and expect the amendment at wrap.

@history_item n=10 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — The start's settings override write and source read were recorded as start steps with no span and no line by design, and `--settings`, the source path and the command appear in no line or span (`decision/child-settings-never-logged`). Why it matters: if the CLI channel for model/effort is a settings key or `--settings` argument, it falls under the child-settings-never-logged rule and the "start step with no span" precedent.

@history_item n=11 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (4. Span / Trace Coverage, 1 of 3) — Removed `decision/no-statusline-settings-in-telemetry`, `decision/statusline-settings-steps-unlogged` and others: run no longer passes --settings to its child, and `decision/child-settings-never-logged` alone keeps the settings argument out of logs. Why it matters: reintroducing a `--settings` argument or override for the pair would re-open this retired surface. v2-31 promises the user's settings stay byte-identical.

@history_item n=12 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — process-start{subject:"claude-child"} gained env_kept (names only). D-34: every stripped and kept CLAUDE* name is logged, and values are never read. Why it matters: `contract/claude-child-process-start/env-*` and `contract/run-process-log/process-start-claude-child` are the field set a pair field on the claude-child process-start line would extend. If the pair rides the environment, only names may be logged, never values.

@history_item n=13 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** — `~ decision/env-names-only` (strip, logs, the capture arm's env-names file and the verify stamp carry names only) and `~ decision/identity-floor-always-stripped` (floor names always removed; a compile-time list of non-identity hook names beside it). Both were ratified 2026-11-22. Why it matters: if the model/effort channel is an environment name, viola's own value meets the R8 strip and the identity floor. Its name may be logged and its value never.

@history_item n=14 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (Windows host arms retired) — The strip matches CLAUDE* names exactly, and the persistent set is config.json claude_env_keep. Why it matters: this is how `decision/claude-env-strip` works today. Any env-borne pair has to be set after the strip or kept apart from claude_env_keep.

@history_item n=15 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — `~ decision/claude-internals-not-a-contract` and `~ decision/refusal-telemetry` (verify's trust-record write exceptions), `~ decision/r8-strip-logged-as-names-only` and `~ decision/env-vars-not-configuration` (the one CLAUDE_CONFIG_DIR value read), `+ decision/trust-record-write-uninstrumented`, and verify's interactive runs from five to six. Why it matters: each new reading or writing of claude's config or env by viola became a named, ruled exception. A model/effort channel through a settings key or config file would need the same treatment. The entry is also a recent ledger-row precedent for verify's spawn counts.

@history_item n=16 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — The start reads CLAUDE_CONFIG_DIR and writes the trust flag only. It added `~ decision/claude-boundary-only-instrumentation` (the start's one trust-record line), `~ decision/claude-internals-not-a-contract` ("a verified start sets its folder's trust flag only"), `~ decision/refusal-telemetry`, `+ contract/start-trust-log/trust-record` and `+ contract/diag-line-fields/trust-record`. These were ratified 2026-11-22. Why it matters: this is the closest precedent for a start writing into claude's config. The pair must not widen "flag only" or write to the user's settings (v2-31) without a ruling of its own. Any new start record line follows the trust-record line's closed-word pattern.

@history_item n=17 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (1. Obs Scope Summary) — `~ decision/claude-internals-not-a-contract`: bundle strings, the MCP client's tool-call bound among them, are never read as a contract. Trap: the P3 measurement of the CLI's model/effort flag or env name must not rest on bundle strings. Undocumented behaviour belongs in a ledger row with a verify probe.

@history_item n=18 marker=2026-11-21-cross-session-prefix-ledger-row
**2026-11-21-cross-session-prefix-ledger-row** — Added the verify-sender subject (`+ contract/diag-subject/verify-sender`, `+ contract/verify-spawn-subject/verify-sender`), `~ contract/process-log/child-and-probe-lifecycle`, and process-start's subject stated by rule (`~ contract/diag-line-fields/process-start`), with verify spawn pairs counted by rule. Why it matters: it is the latest precedent for a ledger row that adds a verify run or spawn. If the model/effort channel becomes a ledger row, these are the rows it extends.

@history_item n=19 marker=2026-11-18-browser-tools-prompt-modal
**2026-11-18-browser-tools-prompt-modal** — `~ contract/verify-stdout/step-line`, `~ contract/verify-stdout/summary-line` and `~ contract/verify-exit-codes/0` (the absent verdict). These were ratified 2026-11-22. Why it matters: a new relied ledger row for the model/effort channel moves MM and the step/summary lines these rows promise.

@history_item n=20 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — Verify's Run E is the `--resume <id>` start of Run B's session, the resume-session row is relied on, and a killed PTY child carries child_exit_status. Why it matters: the scope says "what a resume does with model and effort today is measured at P3". Run E is the existing probe of a resumed start, so a pair-on-resume reading would extend it.

@history_item n=21 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive's start arm is role run, logs into `run-<name>.ndjson` and shares run's setup; its preflight exit-1 details were added; read_snapshot_or_replay gained its first product caller. Why it matters: replaying the recorded pair on revive happens in this arm (`decision/revive-start-logs-as-run`, `contract/cli-log-file/revive-start`). Any pair-related refusal or line lands in run's log under its rules.

@history_item n=22 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — revive's preflight reads the holder, then the session, then session-live, then the directory, and session-live joined the exit-1 details. Why it matters: `decision/revive-preflight-order` and `contract/revive-preflight-detail/*` fix the preflight sequence. Reading the recorded pair on revive must not be slotted in as a new refusing reading without amending this order.

@history_item n=23 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (every refusal of revive --json's start prints its document) — `refused_with` prints the exit-1 document for each start refusal before spawn_child, and anyhow errors print internal-error. Why it matters: any new start refusal (for example an invalid pair) must go through refused_with with a closed code and the --json document.

@history_item n=24 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the process-exit detail enum's count) — The schema closes process-exit detail by its members (19 at the time), and a new word joins the enum with its emitter. Why it matters: `decision/distinct-detail-code-per-exit-cause` and `contract/process-exit-detail/exit-1` require a distinct, path-free code for every new exit-1 cause the pair might add.

@history_item n=25 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (verify writes its refusal's code) — A refused update_stamps leaves verify's process-exit with strict-modes-failed. A failing row is no refusal and carries no code. Why it matters: these are `decision/strict-modes-failure-lines` and `contract/process-exit-detail/strict-modes-failed`. A new ledger row's failure adds no detail code.

@history_item n=26 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (closed words left diag-line.v1.json with their emitter) — Rule: a closed schema word no build can emit leaves the schema with its emitter, and os is closed to linux. Why it matters: `decision/unemittable-schema-words-removed`. Any closed pair vocabulary added to the schema must have an emitter in the same build.

@history_item n=27 marker=2026-10-04-dialog-answers-by-dialog-id
**2026-10-04-dialog-answers-by-dialog-id** — parse-rejected{parser:"ledger-stamps"} gained strict-modes-failed: run's version gate reads the stamps strictly, and a failed read counts as unverified (cli_verified false). Why it matters: `contract/run-log-parse-rejected/ledger-stamps` and `decision/unreadable-ledger-is-unverified`. If the pair is passed only on a verified CLI (behaviour gated by a ledger row), the version gate's unverified reading decides it.

@history_item n=28 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the home-tree check of run and revive's start) — checked_exe and check_home_tree run before the wrapper log, and a refusal opens no log and writes no line. Why it matters: `decision/home-tree-check-before-start-log` and `decision/home-tree-check-before-log-init`. A pair read from config.json or the instance has to come after this check and the log init, not before them.

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers over a replayed snapshot) — A snapshot rebuilt from events.ndjson carries no endpoint, pid, started_at or statusline_command, and run's collision check reads the heartbeat alone. Why it matters: `contract/replayed-snapshot/live-fields`. A pair recorded in the snapshot must also be rebuilt by replay from its events line, or revive would read none after a recovery.

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5: the replay's product readers two to six, state.snapshot_recover placed) — state.snapshot_recover sits under run.start before run.collision_check, and revive's preflight makes one snapshot read and writes one state-recovered line. Why it matters: `contract/run-start-spans/span-chain` and `contract/state-recovered/*`. Reading the recorded pair on revive should reuse that one read and not add a second replay or recovery line.

@history_item n=31 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — Set where state-recovered is written and how often: snapshot-replayed and snapshot-unsupported-v once per read call, torn-line-healed by the appender only. Why it matters: `contract/state-recovered*` and `decision/appender-heals-torn-line`. A new pair line appended to events.ndjson heals a torn tail through the same appender.

@history_item n=32 marker=2026-10-10-session-links
**2026-10-10-session-links** — A link is a product line in events.ndjson plus a snapshot field, with no process-log line, no event value and no span. The start's restore is a read pass with no span, taking the start's unspanned steps from two to three. Why it matters: this is the precedent for a per-instance record kept in events and snapshot and restored at start without instrumentation, which is the shape the pair record would take.

@history_item n=33 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — Defined viola list's lines, its no-line refused scan, board::read as a replay caller, and the state-entry parse-rejected. Why it matters: if the pair is shown on list or list --json (a fork in scope), it is read through board::read under these logging rules (`contract/list-json/liveness`, `contract/viola-mcp-tools/list`).

@history_item n=34 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the start's socket-directory step, no span and one line on a refusal) — It added a fourth unspanned start step and socket-dir-refused as one process-exit before any snapshot, child or bind. Why it matters: `contract/exit-cause-detail/socket-dir-refused` and `contract/run-start-failure-detail/socket-dir-refused`. They are the pattern for a start step that writes a line only when it refuses, and they fix the order the pair step must not break.
