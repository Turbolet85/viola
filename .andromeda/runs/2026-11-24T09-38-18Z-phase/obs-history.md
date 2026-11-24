@history_item n=1 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (revive in the log) — revive's start arm logs as role `run` into `run-<name>.ndjson` through the setup it shares with `run`. Its preflight added exit-1 details `no-session` and `cwd-missing`. `human::refuse` gained revive as its second caller. `read_snapshot_or_replay` got its first product caller, so a replayed snapshot writes `state-recovered{snapshot-replayed}` into the run log. Why it matters: `adopt` is built on revive's start path. This is where its role, its log file and its exit-1 detail vocabulary come from (rows `contract/cli-log-file/revive-start`, `decision/revive-start-logs-as-run`, `contract/run-exit-1-detail/*`).

@history_item n=2 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (a refused instance check writes no line) — revive's start arm runs its instance check before log init. A refusal is the stderr pair and exit 1, with no log opened and no line. `strict-modes-failed` left the exit-1 detail codes. Why it matters: this is the trap behind `decision/instance-check-before-log-init`. If `adopt` opened its log before checking, the open would narrow a widened home and the check would then pass it. Any new entry point has to check first.

@history_item n=3 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder accepted the no-line shapes of the refused checks as built, `viola revive`'s instance check among them. Why it matters: a refused strict-modes check on `adopt`'s start, if it reuses revive's, inherits a no-line shape that the founder has ratified. A new shape would need its own ruling.

@history_item n=4 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — verify gained Run E, the `--resume <id>` start of an earlier session id. `resume-session` is relied on, and no verify line carries a session id. Why it matters: `adopt` resumes a session by id through `resume_args`. The relied `resume-session` ledger row is the CLI capability it rests on, and the rule that no line carries a session id carries over to `adopt`'s id.

@history_item n=5 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added `session-live` as revive's exit-1 preflight detail. The reading is skipped under `--fork` and fails open. It writes one `agents-probe` process-start/process-exit pair into `run-<name>.ndjson`, plus `parse-rejected{parser:"claude-agents-json"}` on an oversize or malformed answer, and `run::agents::read_logged` is its writer. Why it matters: this is the live-elsewhere refusal that `adopt` reuses (rows `contract/revive-preflight-detail/session-live`, `contract/revive-process-log/agents-probe`, `decision/session-live-check-fails-open`). Its fail-open stance is a trap for a guard whose job is to refuse.

@history_item n=6 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `revive --list` became role `cli`, writing into the caller's `cli-<VIOLA_NAME>.ndjson` only after its check passes. Panic exemption 6 was retired. Revive's refusals under `--json` print `{"v":1,"error":"<code>","detail":null}`. `state-recovered` gained `scanned`. Why it matters: this is the precedent for picking `adopt`'s role and file by arm (`cli` against `run`) and for its `--json` refusal document.

@history_item n=7 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — named the callers of the refusal writer `human::refuse` → `write_refusal` by verb and set the census rule (`grep -rcE 'human::refuse\(' src`). Why it matters: a refusal from `adopt` through `refuse` moves the census, and §3 and §11 of obs-plan must be re-derived by that rule.

@history_item n=8 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (home-tree check of `run` and `viola revive`'s start) — `checked_exe` plus `check_home_tree` run before `open_wrapper_log`. A refusal opens no log, writes no line and starts no child, then exits 1 with the home or instance pair, or the `strict-modes-failed` document under revive `--json`. Why it matters: this is row `decision/home-tree-check-before-start-log`. A start made by `adopt` must keep this check ahead of its log.

@history_item n=9 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (pre-init window and panic exemptions) — the pre-step-4 window and exemption 8 gained the home-tree check of `run` and revive's start. A refusal there writes no line, so a panic there leaves no record. Why it matters: a new start arm must be listed in the pre-init window and the bounded exemptions, or the zero-unlogged-panics invariant is misstated.

@history_item n=10 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5: replay readers, `state.snapshot_recover` placed) — `state.snapshot_recover` sits under `run.start` before `run.collision_check`. Revive's preflight makes one snapshot read for its collision check and its directory reading, so there is one replay and one `state-recovered` line. Why it matters: `adopt` lands on a name that may already hold a snapshot. Its reads should keep one replay per start (rows `contract/state-recovered/*`, `decision/no-raw-snapshot-reads`).

@history_item n=11 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers over a replayed snapshot) — a replayed snapshot carries no endpoint, pid, `started_at` or statusline command. Over it, `run.collision_check` reads the heartbeat alone. Why it matters: the outcome of "adopt onto a name already held" is decided by this collision reading (rows `contract/replayed-snapshot/live-fields`, `contract/run-span-attributes/run-collision-check`).

@history_item n=12 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (refusal-telemetry record) — a strict-modes failure gets a line in only a few places. Founder Direction 4's distinct-code rule excepts a strict-modes check refused before any log opens. Why it matters: this explains rows `decision/distinct-detail-code-per-exit-cause` (its except) and `decision/strict-modes-refusal-writes-no-line`, which every new exit cause of `adopt` must respect.

@history_item n=13 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (every refusal of `revive --json`'s start prints its document) — `refused_with` prints the exit-1 document in place of the human pair. An anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` prints the `internal-error` document. A `pump_child` error prints none, because the child has held the terminal. Why it matters: if `adopt` takes `--json` through revive's start, it inherits this shape and the no-document trap after the child starts.

@history_item n=14 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the `cli` catch site's document under `--json`) — the cli catch site prints `{"v":1,"error":"internal-error","detail":null}` under `--json` and `error: internal error` without it. `run`, revive's start, `mcp` and `hook` print neither. Why it matters: the panic and internal-error outcome of `adopt` depends on which role its arm takes (rows `contract/panic-outcome-by-role/*`, `contract/cli-internal-error/*`).

@history_item n=15 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (a stale holder's code is `stale-heartbeat`) — a stale holder reads `stale-heartbeat` and a live holder or fresh replayed beat reads `already-live`. Both apply to revive's logged preflight. The `run.collision_check` outcome stays `already-live`. Why it matters: these are the codes an adopt onto a held name would write (rows `contract/run-exit-1-detail/stale-heartbeat`, `contract/run-start-failure-detail/already-live`).

@history_item n=16 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (Founder Direction 4's codes and carve-outs) — named every exit-1 and exit-21 code, revive's `no-session`, `session-live` and `cwd-missing` among them. The distinct codes are an additive `detail` on `process-exit`. Why it matters: any new closed outcome of `adopt` (a malformed id, a second adopt of the same id, a name already held) must be a distinct, path-free and pid-free code under this rule.

@history_item n=17 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (the start's socket-directory step) — `run` and revive's start refuse a socket directory with one `process-exit{exit_code:1, detail:"socket-dir-refused"}` before any snapshot, child or bind. `refused_with` prints its document. The step has no span. Why it matters: a start made by `adopt` inherits this step and its line (rows `contract/exit-cause-detail/socket-dir-refused`, `contract/run-start-failure-detail/socket-dir-refused`).

@history_item n=18 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the `.cmd`/`.bat` start refusal retired; refusal-writer counts) — `batch-script-child` left every exit-1 list. The `human::refuse(` census was re-derived as `run` 7 and `verify` 5, and revive adds three refusals to `run`'s five. Why it matters: these are the latest census baseline and refusal count that an `adopt` refusal site would move.

@history_item n=19 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (closed words leave `diag-line.v1.json` with their emitter) — the `process-exit` `detail` enum stands at 19 at that chunk. The new rule is that a closed word no build emits leaves the schema with its emitter. Why it matters: a new exit word for `adopt` joins this enum and moves its count. This is row `decision/unemittable-schema-words-removed`.

@history_item n=20 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (1. Obs Scope Summary: `exit-code-log-field`) — exit 11 is retired and never reused. The never-remove log-schema rules were narrowed to "a field the product still emits". Why it matters: `adopt`'s exit codes must not reuse 11 (rows `decision/exit-code-log-field`, `decision/diag-schema-additive-only`, `decision/log-fields-additive-only`).

@history_item n=21 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `events.ndjson` is never rewritten. A reused instance name keeps the earlier starts' lines at its head, so the start product order is read from that start's own lines. A start whose trust keys pass through the wrapper adds `wheel{human-input}`. Why it matters: `adopt` onto a name that once existed reuses its events log, so the order check and the lineage/first-snapshot reading must start at this start's offset (rows `decision/events-stream-never-rotated`, `decision/product-order-from-events-log`).

@history_item n=22 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify writes and removes its temp folder's trust record. It reads `CLAUDE_CONFIG_DIR` when the strip keeps that name, and its refusals ride the stamps with no line. Why it matters: this is the origin of the exceptions in rows `decision/claude-internals-not-a-contract` and `decision/refusal-telemetry`, and of the bounded env read that `adopt`'s start inherits.

@history_item n=23 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — the run and revive start writes its folder's trust flag and logs one INFO `trust-record` line with a closed start word. It reads the kept `CLAUDE_CONFIG_DIR` value and never logs it, and the trust step opens no span. Why it matters: a start made by `adopt` writes this line for the adopted session's cwd (rows `contract/start-trust-log/trust-record`, `decision/claude-boundary-only-instrumentation`, `decision/r8-strip-logged-as-names-only`, `decision/env-vars-not-configuration`, `decision/trust-record-write-uninstrumented`).

@history_item n=24 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — the start's automatic verify child is logged as an `auto-verify` subject pair in the run log. A stop during it ends the start with no `process-exit` of its own. `config.json` gained `verify_new_cli`. Why it matters: a start made by `adopt` on a keyed new CLI goes through this window (rows `contract/diag-subject/auto-verify`, `contract/run-process-log/auto-verify-stop`).

@history_item n=25 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (3. Observability Harness Contract: `run-terminal-output/child-output`) — before the child, `run` may write the automatic-verify lines and the verify child's output, and writes none under `revive --json`. While the child runs it writes nothing of its own. Why it matters: if `adopt` runs in the terminal it is typed in (the revive arm), its terminal output is bound by this row.

@history_item n=26 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (1. Obs Scope Summary: `claude-internals-not-a-contract`) — the CLI's internal state, transcripts and bundle strings are never read as a contract. Why it matters: `adopt`'s cwd, model and effort sources must not read claude's internal state or transcripts as a contract. A reading from another process is a crossing named at P3 (row `decision/claude-internals-not-a-contract`).

@history_item n=27 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — every run and revive start writes one lineage line after `wheel{cause:"start"}`, with no line, span or terminal byte of its own (provisional). The parent and role names reach only `events.ndjson`, the snapshot, the board, `list --json` and the MCP list result, each re-parsed and never logged. Why it matters: `adopt`'s operator-given name and parent land exactly here (rows `contract/run-start-events/order`, `decision/lineage-line-untraced`, `decision/lineage-names-confined`).

@history_item n=28 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — the start writes one untraced launch line after lineage, and the start order became wheel → lineage → launch → session-start. `default_model` and `default_effort` joined `config.json`, a bad value writing one `parse-rejected` config-json malformed line. Why it matters: the model and effort of an adopted session, where no snapshot recorded them, flow through this launch line and these defaults (rows `decision/launch-line-untraced`, `decision/product-order-from-events-log`).

@history_item n=29 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (6. Log Coverage) — exit-1 gained `terminal-not-found` and `not-live` from `viola open`, and the diag-line subject enum gained `terminal`. Why it matters: if `adopt` opens a terminal (a P3 fork), it reuses this subject and these exit words (rows `contract/process-exit-detail/exit-1`, `contract/diag-subject/terminal`).

@history_item n=30 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (4. Span / Trace Coverage) — under the hidden `--origin`, `run.window` follows the trust step, and the window line is untraced. `viola open` logs into the opened name's `cli-<name>.ndjson` inside the `cli.open` dispatch span. Why it matters: this is the precedent for a verb that spawns a start in a new window and for its cli log file and span (rows `contract/cli-log-file/open`, `contract/run-start-spans/span-chain`, `decision/window-line-untraced`).

@history_item n=31 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (2. Telemetry Strategy) — `open` joined the verbs that exit 1 on a strict-modes refusal and print the `strict-modes-failed` document under `--json`. Why it matters: a new verb must be added to this closed list (row `contract/strict-modes-refusal-exit/1`).

@history_item n=32 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — added row `cli-log-file/focus` and a separate `process-exit-detail/exit-1-focus` row, because the standing exit-1 row would pass its bound. `focus` joined `decision/instance-check-before-log-init` and `contract/strict-modes-refusal-exit/21` (provisional). Why it matters: this is the pattern a new verb follows, with its own cli-log-file row, its own exit-1 detail row and a place on the check-before-log lists.

@history_item n=33 marker=2026-11-24-viola-up
**2026-11-24-viola-up** — the `terminal` subject is one process-start with no process-exit, because the terminal outlives the verb (the claimed pair was disproved). `up`'s home-tree refusal after its board scan writes `strict-modes-failed`, an exception to `decision/strict-modes-before-diagnostics`. Rows `contract/cli-log-file/up` and `process-exit-detail/exit-1-up` were added. Why it matters: these are the latest shapes for a spawned terminal and for a strict-modes refusal after a log opens, and the latest lists `adopt` would extend (`contract/process-log/child-and-probe-lifecycle`, `decision/strict-modes-failure-lines`, `contract/strict-modes-refusal-exit/*`).

@history_item n=34 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — `run.pin_copy` places an absent copy without clobbering and never replaces one. A lost placement re-hashes the standing copy: an equal one is reused, a different one is `pinned-hash-mismatch`. Why it matters: a start made by `adopt` shares `pin_copy` (rows `decision/pinned-copy-never-replaced`, `contract/run-span-attributes/run-pin-copy`).

@history_item n=35 marker=2026-11-20-harness-prefix-and-identity-floor-ledger-rows
**2026-11-20-harness-prefix-and-identity-floor-ledger-rows** — the hook capture arm's env-names file and verify's stamp carry env names only. The identity floor is always stripped, beside a compile-time list of non-identity hook names. Why it matters: these are the env-capture surfaces that `tests/security_negatives_env_capture.rs` guards, and this chunk's folded CI item re-times two of its cases (rows `decision/env-names-only`, `decision/identity-floor-always-stripped`).

@history_item n=36 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — `state-recovered` is written by `read_snapshot_or_replay` at WARN, one line per call: `snapshot-replayed` and `snapshot-unsupported-v`. `torn-line-healed` is written by the appender, and a reader never heals. Why it matters: these are the rows an adopt's snapshot read and first write rest on (`contract/state-recovered/*`, `decision/reader-never-heals-torn-lines`).
