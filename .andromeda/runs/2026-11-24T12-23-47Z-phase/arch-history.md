@history_item n=1 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Established Decisions: 2 rows) — `~ decision/revive-resumes-in-place` narrowed to "replays no launch spec but the recorded model and effort", and `+ decision/launch-pair-handed-not-verified`: every start hands the pair on argv and it is never read back or verified. This is the base the switch builds on. A switch that "takes effect for the next turn" cannot be verified by the same rule, and a recorded new pair is exactly what revive replays.

@history_item n=2 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Standard Contracts 1 of 2) — `launch` joined the wrapper's log-only kinds (it never wakes `wait`), the replay's closed recovered-field list (`decision/snapshot-replay-log-fields-only`, `contract/snapshot-envelope/unreadable-snapshot`) and `contract/instance-snapshot/data` (`launch?`, no `v` move). A switch record has to land in these same closed lists, or else reuse `launch`.

@history_item n=3 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Standard Contracts 2 of 2) — added `contract/wrapper-event-appends/launch` (once at every start, after `lineage`), `contract/event-data/launch` `{model, effort}`, `contract/instance-snapshot/launch` (the last `launch` line replaces the held value; a non-string member reads none), and `list-item/model` / `list-item/effort`. These are the register the scope's premise leans on: the record a resume replays may already be the last `launch` line. A mid-session `launch` line would break the "once at every start" promise. The list-item rows were PROVISIONAL (W2, delegate).

@history_item n=4 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Occupied Resources 1 of 2) — `--model` / `--effort` literals homed in the agent crate (`decision/claude-flag-literals-in-agent-crate`). Also: revive's `--model`/`--effort` arguments; revive exit 2 for a bad pair value or a pair after `--`; `contract/child-argv/launch-pair` (after `--plugin-dir`, before the caller's words, at every start and resume); `config-json/default-model`. The switch's argument surface and its exit-2 usage refusals have this as their precedent.

@history_item n=5 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Occupied Resources 2 of 2) — added `config-json/default-effort`, `run --model` / `run --effort` (exit 2 on a bad word), `decision/revive-pair-from-record` (override, else the recorded half while it parses, else default; PROVISIONAL on the delegate's word), `decision/recorded-pair-on-child-argv` (a ratified crossing, re-parsed to its closed shape first) and `decision/default-launch-pair` (`DEFAULT_PAIR`, the five effort words). A switched pair recorded for revive/up crosses this same ratified boundary.

@history_item n=6 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Conventions: 3 rows) — added revive's `--model`/`--effort` conflicting with `--list` (`contract/cli-verbs/revive`), `contract/normalised-event-kinds/launch` (a new kind only in viola-core) and `contract/model-word/format`. These are the closed shapes the switch verb validates against. A new event kind for the switch would extend this closed list.

@history_item n=7 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (Established Decisions: 1 row) — `decision/revive-in-place` now keeps "the model and effort pair its real snapshot records". The switched pair must be what the snapshot or replay records, or revive keeps the launch-time pair.

@history_item n=8 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Standard Contracts, both parts) — `window` added to the replay field lists, the snapshot and the wrapper's log-only kinds, with `contract/instance-snapshot/window`: "the last `window` line after the last `launch` line winning and a later `launch` line clearing it". Trap: if the switch appends a `launch` line mid-session, the replay clears the window record that `up`/`revive --origin` use.

@history_item n=9 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (Conventions / Established Decisions) — `contract/cli-exit-codes/1` gained the open refusals, `contract/normalised-event-kinds/window` and `contract/cli-verbs/open` were added, and `decision/no-start-over-live-instance` names open. This is the precedent for registering a new verb in the exit-1 list, the verb list and the kind list.

@history_item n=10 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Conventions: 2 rows) — focus, a CLI verb acting on a live instance, joined `contract/cli-exit-codes/1` and `contract/cli-exit-21/strict-modes-failed` (strict-modes check before any log). This is the nearest precedent for a switch verb aimed at a running instance.

@history_item n=11 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Occupied Resources 1 of 2) — focus logs into `cli-<name>.ndjson` under the target's name (`contract/diagnostics-role-files/cli-name`), with `name/command/focus` and new process-exit detail words in `schemas/diag-line.v1.json`. The switch verb's process-log lines and any new closed details must be registered the same way.

@history_item n=12 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (Established Decisions: 1 row) — `+ decision/focus-has-no-driver-form`: the driver-facing (MCP) form was left out on the conservative arm, PROVISIONAL. This is the precedent for the scope's "MCP tool out unless a fork settles otherwise".

@history_item n=13 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Established Decisions: 4 rows) — `decision/launch-pair-handed-not-verified` now lists every start (run, revive, adopt). `decision/no-start-over-live-instance` and `decision/run-refuses-held-name` gained adopt. `decision/adopt-refuses-live-session` was added (PROVISIONAL). A restart-based switch through the start path inherits all of these refusals.

@history_item n=14 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Standard Contracts / Conventions: silent roles) — added `contract/cli-json-output/adopt-refusal-codes`, `contract/cli-verbs/adopt`, and adopt's start in `contract/cli-json-output/internal-error` and `decision/silent-roles-at-internal-error-catch`. This is the most recent pattern for a new verb's `--json` refusal codes and for how a start-path verb stays silent at the catch site.

@history_item n=15 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (Occupied Resources: 4 rows and 3 rows) — added `name/command/adopt`, the adopted start's `--resume` on the child argv, and `contract/diagnostics-role-files/run-name`. Adopt's start was added to `decision/env-strip-names-only`'s CLAUDE_CONFIG_DIR read and to `contract/frame-verb-exit-detail/socket-dir-refused`. Every new start path had to be named in each of these rows, and a switch that restarts the child would have to be named in them too.

@history_item n=16 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Established Decisions: 2 rows) — `decision/revive-resumes-in-place` now also replays the recorded workspace under the hidden `--origin`, and `decision/lineage-label-only` names up's one read. `up` re-opens through revive, so a switched pair must be in the record revive reads for `up` to replay it.

@history_item n=17 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Conventions / Occupied Resources) — added `contract/cli-verbs/up`, `viola-up-exit-codes/0|1|21`, up in `contract/cli-exit-codes/1` and `diagnostics-role-files/cli-name`, `contract/process-exit-detail/*` words, and the hidden `revive --origin`. This is the register the switch's exit codes and process-exit details extend.

@history_item n=18 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (Established Decisions + Standard Contracts: `up-selection-rule`, `up-parents-first`, `up-writes-no-state`) — up writes no state of its own and leaves every state file to the revives it spawns. A pair switched and recorded is replayed only through revive's own record reading.

@history_item n=19 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Standard Contracts 1 of 2) — precedent for a new wrapper-appended log-only kind (`lineage`). It added an optional snapshot field and the replay's recovered list, with `list-payload/items` keys, and the kind was kept out of the `wait` wake set.

@history_item n=20 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** (Established Decisions: 2 rows) — `decision/revive-in-place` keeps the recorded parent and role, and `decision/lineage-label-only` holds that no control path reads lineage. This is the pattern the model/effort pair later followed for what revive keeps.

@history_item n=21 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the replay's first product reader, revive as the second way a wrapper starts) — revive resumes a `gone` instance in place through the same start as `run`. The log is appended and never truncated, "no launch is replayed", and its first record is `wheel{driver, start}`. A restart-based switch ends and resumes the session, so it would reset the wheel to driver.

@history_item n=22 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** (the `revive` verb registered) — `--resume <id>` on the child argv was a crossing whose ratification was then owed, along with revive's exit-1 preflight refusals and its run-log producer. This is the origin of the start-path registers a restart-based switch would ride.

@history_item n=23 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (four crossings ratified: the override, the shell-out, the source files, the revive session id) — the per-session `--settings` override was a code-bearing file written at every start, ratified as a boundary widening. The logged session id on `--resume` was ratified at the same time. This is the precedent for any settings-layer channel for the switch.

@history_item n=24 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Established Decisions 1 of 2) — `+ absence/status-line-interception`: "no settings layer, no --settings at start". It also retired `decision/statusline-wrapped-per-session`. A switch through a settings write or a per-session settings layer collides with this absence, which the picked rows carry.

@history_item n=25 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Occupied Resources 1 of 2) — retired `decision/per-session-settings-override`, `contract/child-argv/settings` and `contract/instance-settings-json/statusline`. `decision/claude-flag-literals-in-agent-crate` now says only verify reads `--settings`. A start passes the child no `--settings`.

@history_item n=26 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** (Conventions: 2 rows) — `decision/protocol-version-bump-rule` gained the exception that removed snapshot and stamp fields kept every own-format `v` at 1. An additive switch field or kind needs no `v` bump under the same rule.

@history_item n=27 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** (Conventions: 7 rows) — added `decision/retired-exit-codes-never-reused` (exit 11 unassigned) and the first `protocol-version-bump-rule` exception. A new verb's exit codes must avoid the retired 11.

@history_item n=28 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Established Decisions: 2 rows) — `decision/run-start-order` gained the trust-record write into the child's Claude config, and run and revive now write a trust record. Trap: a restart-based switch through the start path writes into the Claude config (`~/.claude.json`), against the scope's "writes nothing to `~/.claude*`".

@history_item n=29 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (Occupied Resources: 3 rows) — the run and revive start read CLAUDE_CONFIG_DIR's value when `claude_env_keep` keeps it (`decision/env-strip-names-only`, `config-json/claude-env-keep`). Any new start path must be named here, as adopt later was.

@history_item n=30 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (Established Decisions: 1 row) — `decision/run-start-order` now reads: bind, any automatic verify, then the trust record on a verified verdict, then the snapshot and start events, then spawn. A restart-based switch inherits the automatic verify and its live-start cost.

@history_item n=31 marker=2026-11-20-a-new-cli-version-stamped-automatically
**2026-11-20-a-new-cli-version-stamped-automatically** (Standard Contracts: 1 row) — `contract/instance-snapshot/cli-verified` is the start's final verdict after any automatic verify. The verified bit gates which switch channel (a typed command, a ledger-rowed behaviour) is usable.

@history_item n=32 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** (Session Liveness: revive refuses a session id live elsewhere) — measured on 2.1.287: a second `claude --resume` of a held id runs two writers on one transcript, so revive refuses `session-live`. A restart-based switch must end the running child before resuming, or it would hit this.

@history_item n=33 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** (the resume row, Run E and /compact listed) — `resume-session` became a relied ledger row ("`--resume <id>` reopens the session under that id"). A restart-based switch relies on that row. Whether a resumed session runs on the new pair has no row, which is the reason for `decision/launch-pair-handed-not-verified`.

@history_item n=34 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** (the capability ledger: three relied-on dependences with no row) — the precedent for relying on an undocumented CLI behaviour with no row, named as an exception with an owner. `decision/capability-ledger-gate` requires a row and probe for any new dependence, such as the CLI's model command or a mid-session switch taking effect.

@history_item n=35 marker=2026-11-19-a-live-stopfailure-payload
**2026-11-19-a-live-stopfailure-payload** (the StopFailure exception settled — Design Philosophy, Established Decisions, Cross-cutting Patterns) — the one standing "relied with no ledger row and no probe" exception (StopFailure), kept on the founder's word. These are the `except` cells of `decision/capability-ledger`, `cli-capability-ledger` and `capability-ledger-gate`, and the only precedent for not rowing a relied behaviour.

@history_item n=36 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** (Established Decisions 1 of 2) — `decision/local-commands-by-ledger` classifies a send as a local command by name, or name plus a space and an argument. A typed `/model …` would be classified this way only if it is listed. `contract/send-results/unconfirmable` covers listed commands with no post-condition. This bears on the switch's "command typed into the live session" route.

@history_item n=37 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** (send consumes the local-command list) — a listed command with no post-condition, or any on an unverified CLI, ends `ok` `unconfirmable`. This is the confirmation model a typed model/effort command would fall under. It also says no `prompt-submitted` claims a local command.

@history_item n=38 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** (the running-turn state as built) — the running turn is held in memory in `WheelSlot`, and `send` is refused `turn-running` while a turn runs. This is the state a mid-turn switch has to read, which the scope leaves open for P3 to measure.

@history_item n=39 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** (Established Decisions: 3 rows) — `+ decision/send-flight-lock`: the post-gate section runs under one wrapper flight lock, shared with the claim, turn and wheel moves, and a human key never takes it. A switch that types into the session, or that restarts it mid-turn, must respect this lock.

@history_item n=40 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** (Established Decisions 1 of 2, and Design Philosophy) — `decision/paste-cut-mark` and the `paste-cut` refusal rungs were added, and `decision/human-always-wins-wheel` now bounds a human key's delay to the paste's in-flight chunk and closing marker. Any typed switch command inherits these rungs and never delays a human key.

@history_item n=41 marker=2026-10-07-send-waits-out-the-paste-hint
**2026-10-07-send-waits-out-the-paste-hint** (the gate waits for the input box, bound 8.5 s) — `send` reads the wheel and the running turn twice, at arrival and again after the gate, so a human who starts typing during the wait is not pasted over. This is the precedent for a switch that waits on the readiness gate.

@history_item n=42 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (Established Decisions: 4 rows) — `decision/wheel-starts-with-driver` holds at every start, a trust-dialog key included. Dialog-hold rows are standing. A restart-based switch gives the wheel back to the driver and drops any human hold. A dialog pending at the switch ends when the child ends.

@history_item n=43 marker=2026-10-19-dialog-hold-for-the-driver
**2026-10-19-dialog-hold-for-the-driver** (the configurable dialog hold and the dialog-expired record) — `pending_dialog` is written before the dialog line and never rebuilt from the log, and `dialog-expired` is the one wrapper kind that wakes `wait`. This is relevant to what a switch does while a dialog is held, which the scope leaves unmeasured.

@history_item n=44 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the strict-modes refusals' exits) — a refused strict-modes check of a frame verb exits 21 with the instance pair and no line. A refused home tree at a start exits 1 before any log (`refuse_tree`, `checked_exe`). The switch verb's check-before-log order follows whichever path it takes: frame verb or start.

@history_item n=45 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (what each replay reader decides; one read per revive) — `decision/revive-reads-snapshot-once`: a replayed snapshot gives one replay and one `state-recovered` line. The record a resume replays comes through this one read, and a replayed snapshot yields only log-derived fields, `launch` among them.

@history_item n=46 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** (the snapshot's pid and started_at as the server identity) — every frame-writing client compares the peer's pid and start time with the snapshot's before its first frame. If the switch is a channel method, its client joins this check, the instance check and the socket-dir check, in the fixed order of `decision/frame-client-check-order`.

@history_item n=47 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** (a client's refused socket directory, CLI, hook and MCP) — `check_endpoint_dir` runs at every frame client's connect. `socket-dir-refused` is exit 21 for the frame verbs and exit 1 for a start. A new frame verb must be added to the seams' caller list.

@history_item n=48 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** ([Links]: the explicit verbs and methods built in the instance-check form) — serving new channel methods was ratified as a boundary widening, with the instance check before any log and an exit-21 refusal. This is the precedent if the switch lands as a served wrapper method rather than a restart.

@history_item n=49 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (exit 21's closed cause set and how each client reaches it; the exit-21 document carries its cause in `detail`) — `UnreachableCause` is a closed set with one CLI writer, `client::unreachable_document`, and `without_endpoint` gives `instance-dead` or `unwrapped-name`. A switch against a non-live instance reads one of these causes.

@history_item n=50 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (`viola revive --json` documents every refusal of its start) — every refusal of the shared start prints the closed-code document, and `refused_with` and `Launch.json` print it, with no document after the child holds the terminal. This is the pattern for a start-path switch's `--json` refusals.

@history_item n=51 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (the code-word rule and the exit-cause table) — every exit-21 cause and exit-1 code passes `is_code_word` and is a row of `tests/contract_error_bodies.rs`, held to its hint by `tests/cross_exit_causes.rs`. The switch's new closed words must land in both tables.

@history_item n=52 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** (the sanitised error surfaces pattern and its contract binary) — the census `product_error_writers_are_each_named` fails on any unnamed error writer, and bodies carry codes and fixed messages only. The switch's refusal documents must be enumerated there.

@history_item n=53 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** (the MCP tools' contract as landed) — the MCP tool set and its result shapes. `decision/new-capabilities-as-verbs` says a capability is exposed as an MCP tool where a driver needs it. The scope keeps the switch CLI-only unless a fork decides otherwise, as focus was.
