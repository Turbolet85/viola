@history_item n=1 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — `already-live` was logged for a live and a stale holder, and a `gone` holder was taken over with no exit-1 line (`run.collision_check` outcome `free`). This is where the holder-liveness taxonomy that `up`'s skip rule uses comes from: a gone instance can be re-opened, a live one is refused.

@history_item n=2 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — every start refusal went through `src/human.rs` `refuse` → `write_refusal` as two fixed stderr lines in one write, and an output module may carry a local print `#[allow]` only where it uses a print macro. This is the precedent for `up`'s per-instance lines going through `src/human/` with no print macros outside the owning output module.

@history_item n=3 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** (the `cli` role's internal-error line) — a `cli` verb's panic or fault, when an instance resolves, writes `process-exit{subject:"self", exit_code:1, detail:"internal-error"}`, puts the chain in `detail-cli.ndjson` only, then prints exactly `error: internal error`. This is the catch-site shape a new `up` verb inherits.

@history_item n=4 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — set where `state-recovered` is written: `read_snapshot_or_replay` writes `snapshot-replayed` / `snapshot-unsupported-v` at WARN, one line per call; `torn-line-healed` is written by the appender only. `up`'s board read over post-reboot snapshots hits these lines once per read call.

@history_item n=5 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — `revive`'s start arm logs as role `run` into `run-<name>.ndjson`, with preflight exit-1 details (`no-session`, `cwd-missing`, later `session-live`) as one `process-exit` each and no child started. `read_snapshot_or_replay` gained revive as its product caller. `up` re-opens each instance through this path, so each re-opened instance's lines land in its own run file, not in a cli file.

@history_item n=6 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (a refused instance check writes no line) — `revive`'s start arm runs its instance check before log init. A refusal prints the stderr pair, exits 1, opens no log and writes no `process-start` or `process-exit`. This trap is why every check must come before the first open, since the open narrows the home. Any `up` check of the home must keep that order.

@history_item n=7 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — `list` (span `cli.list`) opens `cli-<name>.ndjson` only under a parsed `VIOLA_NAME` and after its scan passed. A refused scan (exit 21) writes no line. `board::read` is the replay's caller and writes one `parse-rejected{parser:"state-entry"}` per nonzero skip count, naming no entry. This is the direct precedent for `up`'s board read and for where its cli log may open (rows `cli-log-only-under-viola-name`, `symlink-skip-logged-as-count`).

@history_item n=8 marker=2026-10-13-t06-38-35-wrap
**2026-10-13-t06-38-35-wrap** — the founder took the board's refused-scan no-line shape as his own answer, and revive's start-arm no-line refusal was left for his next batch. This is the provenance of the no-line refusal shape `up`'s board scan would reuse.

@history_item n=9 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** (the reserved web vocabulary leaves) — `process` is closed to `run|hook|mcp|cli`, and no process logs a liveness transition: liveness is exposed only by readers (`list --json`), stale past 5 s. `up`'s live / not-live reading must not add a transition line or a new role.

@history_item n=10 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — `revive`'s preflight reads the holder, the session, `session-live`, then the directory. `session-live` writes one `agents-probe` pair into `run-<name>.ndjson`, plus `parse-rejected{parser:"claude-agents-json"}` on a bad answer, and fails open. Every instance `up` revives runs this preflight and its probe.

@history_item n=11 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — `revive --list` became role `cli`, logging into the caller's `cli-<VIOLA_NAME>.ndjson` after its check, with no log when there is no name. `state-recovered` gained `scanned`: the line carries `instance` (the caller) and `scanned` (the board instance). `up` reading the board under a name writes lines of exactly that shape. Panic exemption 6 was retired.

@history_item n=12 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — the `human::refuse` census is counted by verb and re-derived by `grep -rcE 'human::refuse\(' src`. Any `up` refusal site moves this census and must be re-counted at the wrap.

@history_item n=13 marker=2026-10-17-optional-plugin-install
**2026-10-17-optional-plugin-install** — a `cli.<verb>` span may sit off the dispatch path. The self pair goes into the caller's `cli-<name>.ndjson` only under a parsed `VIOLA_NAME` and after the check passed; with no name there is no log. A refused check (exit 21) opens no role file. This is a precedent for a caller-scoped short-lived verb like `up`.

@history_item n=14 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder ratified every refused check's no-line shape as built (revive, statusline, `budget.json`, list, plugin install). `up` can rely on these as ruled, not provisional.

@history_item n=15 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — the link verbs run their instance check on the named (driven) instance before their `cli` file opens, then log into the named instance's `cli-<driven>.ndjson`, not the caller's. `link` / `unlink` names never enter a process-log line (PII row). This is the precedent for which instance's file a fleet verb writes per instance, and for keeping lineage and link names out of logs.

@history_item n=16 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — `run` and `revive`'s start run `checked_exe` + `check_home_tree` before the wrapper log; a refusal has no line and no detail code. The pre-init window and exemptions 1 and 8 cover these checks, and `check_diagnostics` gates every role and detail file open. Replayed snapshots carry no `endpoint`, `pid` or `started_at`, so `run.collision_check` reads the heartbeat alone and `live_endpoint` gives none. This is central to how `up` can (or cannot) verify pid + start time over post-reboot snapshots. The refusal-telemetry record was narrowed to a few line sites.

@history_item n=17 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — under a concurrent first start the pinned copy is placed no-clobber. A start that loses the placement re-hashes: an equal copy is reused, a different one is `pinned-hash-mismatch` with no added line. This matters if `up` starts several revives at once on a fresh pin (row `pinned-copy-race-adds-no-line`).

@history_item n=18 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — the error-body census (`tests/contract_error_bodies.rs`) holds every product error writer and its witness, and `thiserror` messages interpolate nothing. A new `up --json` error document or a new typed error must join this census.

@history_item n=19 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — the `cli` catch site prints `{"v":1,"error":"internal-error","detail":null}` under `--json`, with `--json` read before any `--`. Every `revive --json` start refusal prints its code document. A stale holder became `stale-heartbeat` (distinct from `already-live`). The `process-exit` detail enum is closed by count and Founder Direction 4 names the distinct codes. `up`'s `--json` document and the per-instance outcome words sit next to these closed sets.

@history_item n=20 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — a refused socket directory on `run` / `revive`'s start is one `process-exit{exit_code:1, detail:"socket-dir-refused"}` before any snapshot, child or bind, and it is no strict-modes failure. The `human::refuse` census moved again. After a reboot the runtime-dir socket parent may be gone, and each `up` start then meets this step.

@history_item n=21 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — Linux-only target. A closed word of `schemas/diag-line.v1.json` that no build can emit leaves the schema with its emitter, and the `process-exit` detail enum was re-counted. Any closed word `up` adds must have an emitter and be counted by the enum's members.

@history_item n=22 marker=2026-11-14-hyprland-seam
**2026-11-14-hyprland-seam** — added the `hyprctl-probe` subject, the `hyprctl-json` parser (`oversize` / `malformed`, never `unknown`) and the `hyprland` span area, and the hyprctl spawn is instrumented at viola's call site. `up`'s workspace placement through the Hyprland seam reuses these, with no new parser or subject.

@history_item n=23 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — exit 11 is retired and never reused (row `exit-code-log-field`), and the start order lost `budget-gate`. The schema-additive rules were narrowed to "never remove a field the product still emits". `up`'s exit codes must not take 11.

@history_item n=24 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — `contract/replayed-snapshot/live-fields` became: a replayed snapshot carries no endpoint, pid, started_at or statusline_command, and run's collision check reads the heartbeat alone. That is the row `up`'s «was live» / skip reading rests on after a reboot.

@history_item n=25 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — `events.ndjson` is never rotated or rewritten. A reused instance name keeps the earlier starts' lines at its head, so a start's own lines are read from where that start begins, and the start order is read from that start's own lines. A trap for `up` reading the last lineage or window record or «was live» from the events log.

@history_item n=26 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — each run and revive start writes exactly one INFO `trust-record` line, its detail one of 16 closed words, and reads the kept `CLAUDE_CONFIG_DIR` value without logging it. Every start `up` triggers writes this line.

@history_item n=27 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** — a keyed start with `verify_new_cli` runs an automatic verify (subject `auto-verify`). A stop during that window ends the start with no `process-exit` of its own. This applies to each revived start of a fleet re-open after a CLI update.

@history_item n=28 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (`contract/run-terminal-output/child-output`) — `run` writes nothing to the terminal but the child's output and, before the child, a keyed start's automatic-verify lines, none of them under `revive --json`. This applies to the terminal `up` opens for each resumed instance.

@history_item n=29 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — every start writes one `lineage {parent, role}` line after `wheel{cause:"start"}` (row `lineage-line-untraced`, PROVISIONAL), with no process-log line or span. Parent and role names reach only the product surfaces, never a process-log or detail line (`lineage-names-confined`, PROVISIONAL), and link names stay out of process logs. `up`'s parents-before-children order reads this record and must keep the names out of its own log lines.

@history_item n=30 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** — the start order became wheel, lineage, then `launch`, then session-start, and the launch line is untraced (PROVISIONAL). `config.json` has seven known keys, with one malformed line per bad value. A revived start under `up` writes the launch pair, and its order is read from the events log.

@history_item n=31 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — `viola open` logs into the opened name's `cli-<name>.ndjson` inside `cli.open`, with a `terminal` subject pair. Exit-1 `terminal-not-found` / `not-live` were added. Under `--origin` the start adds `run.window` (outcome only) and an untraced window line `{address, workspace, placement, origin}`, and open refuses strict modes with exit 1. `up` re-opens in the window `open` would give, so this entry fixes the subject, exit words, span and untraced window record it reuses.

@history_item n=32 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — added `cli-log-file/focus` (`cli.focus` span, carrying no fields) and a separate exit-1 row for focus's six words (the shared exit-1 row would pass its bound and its no-child clause fails once hyprctl has spawned). focus joined the instance-check-before-log-init list with exit 21 on a strict-modes refusal. All of this is PROVISIONAL. It is the latest precedent for how a new verb's cli-log-file row, exit-detail row and check order are added.
