@history_item n=1 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (4. Span / Trace Coverage, amd-somqwxfd) — `+ decision/adopt-start-logs-as-run`: adopt's start logs as process run into `run-<name>.ndjson` through run's own lines and spans, adding no event, detail word, field, subject or span of its own; `~ decision/home-tree-check-before-start-log`, `~ decision/launch-line-untraced`, `~ decision/lineage-line-untraced` gain adopt's start. Matters most here: any word or line this chunk adds for an at-once child exit departs from "adds no detail word" and must set this row, or be written through run's existing lines.

@history_item n=2 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (6. Log Coverage, amd-p3n3nso3) — `+ contract/process-exit-detail/exit-1-adopt`: adopt's preflight ends exit 1 with one process-exit detail session-live and no child started, its strict-modes-failed refusal writing no line; `~ decision/session-live-check-fails-open` and `~ contract/diag-subject/agents-probe` gain adopt's start. A new exit-1 word for an early child exit after the spawn is a cause this row does not hold ("no child started"): it is the register this chunk extends or sets beside.

@history_item n=3 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (1. Obs Scope Summary, amd-nfytistv) — `~ contract/exit-1-detail/session-live` and `~ contract/run-exit-1-detail/session-live` now name viola adopt beside revive. The precedent for how an adopt exit-1 cause is registered: one distinct detail code naming no path or pid, on both the exit-1-detail and run-exit-1-detail lists.

@history_item n=4 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (2. Telemetry Strategy and 3. Observability Harness Contract, amd-idjn364c · amd-p3wvgazh) — `~ contract/strict-modes-refusal-exit/1`, `~ decision/home-tree-check-before-log-init`, `~ decision/instance-check-before-log-init` gain adopt's start: its strict-modes and tree checks run before `open_wrapper_log`, a refusal writes no line, exits 1 and under --json prints the strict-modes-failed document. Any move of the spawn folder must keep these checks ahead of the log.

@history_item n=5 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (6. Log Coverage, 1. Obs Scope Summary, 3. Observability Harness Contract, amd-6pluk4bc · amd-2whzt7ul · amd-m3r652po, PROVISIONAL on the delegate word) — `~ contract/start-trust-log/trust-record` (one INFO trust-record line per adopt start, a closed set of 16 start words, no path or project key), `~ decision/r8-strip-logged-as-names-only` and `~ decision/env-vars-not-configuration` (adopt's start reads the kept CLAUDE_CONFIG_DIR value to find the claude config). The trust step flags the start's folder; a changed spawn folder changes which folder that line's word is about, and the line must stay path- and key-free.

@history_item n=6 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (7. Error Capture & Reporting and 8. PII Scrubbing & Compliance, amd-pusa3esc · amd-izmkoxpc) — `~ contract/cli-internal-error/human-line`: adopt's start prints neither `error: internal error` nor the document at the catch site; `+ decision/adopt-session-id-never-logged`: the typed session id never appears in any line, field pair or detail. Any new early-exit line, hint or event must carry neither the session id nor the resume folder's path.

@history_item n=7 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (2. Telemetry Strategy and 6. Log Coverage, amd-cvulnlnx · amd-u25b25rl) — `~ contract/socket-dir-refused-exit/codes` and `~ contract/diag-detail/socket-dir-refused` name the start of run, revive and adopt as writers of process-exit exit 1 socket-dir-refused. These are the standing adopt-start exit-1 causes the new early-exit outcome must stay distinct from.

@history_item n=8 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive's start arm logs as role run in `run-<name>.ndjson` and its preflight adds `no-session` and `cwd-missing` exit-1 details. `cwd-missing` is the recorded-directory reading, done before any child starts. This is the precedent for the resume folder: revive takes it from the snapshot's recorded cwd and refuses with a distinct code when it is gone. Adopt has no such record, and that is this chunk's gap.

@history_item n=9 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — added `session-live` to revive's preflight in the order holder, session, session-live, directory, with its agents-probe pair in `run-<name>.ndjson` failing open. Adopt reuses this check (`live_elsewhere`). The order of preflight readings is the place a new adopt folder reading would join.

@history_item n=10 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (E5: the replay's product readers, amd-477gj7qu; the home-tree check, amd-yfhlqjyk) — revive's preflight makes one snapshot read for both its collision check and its recorded-directory reading: one replay, one state-recovered line. The run and revive starts check the home tree with `checked_exe` before the wrapper log, and a refusal writes no line. If adopt starts reading a recorded or derived folder, this is the one-read and no-line pattern it must follow.

@history_item n=11 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (every refusal of revive --json's start prints its document, amd-lyzkv6h4) — `refused_with` prints the exit-1 document in place of the human pair before `spawn_child`. A `pump_child` error prints no document, because the child has held the terminal and viola writes nothing past it. This is a trap for an early child exit: anything said after the spawn falls outside the pre-spawn refusal path, and this rule forbids writing past the child's terminal.

@history_item n=12 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (Founder Direction 4's codes and named carve-outs, amd-tnjfck73; the process-exit detail enum's count, amd-6jizktze; a stale holder's code, amd-nen7xeyv) — each exit-1 and exit-21 cause gets a distinct detail code, and the schema counts `process-exit` `detail` by its enum's members. This backs `decision/distinct-detail-code-per-exit-cause` and `decision/exit-1-detail-codes`: a new early-exit cause owes its own code and a schema enum entry.

@history_item n=13 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the closed words that left diag-line.v1.json, amd-7tvqnh5t) — added the rule that a closed word of `schemas/diag-line.v1.json` that no build can emit leaves the schema with its emitter, and set `process-exit` `detail` at 19. This is the closed-word discipline for any detail word this chunk adds; the count has moved since through open, focus and up.

@history_item n=14 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** (6. Log Coverage, amd-6d2oxlhy) — `~ contract/process-exit-detail/exit-1`: open's terminal-not-found and not-live were added with the carve-out "bar not-live's terminal" to the no-child-started clause. This is the precedent for an exit-1 word written after a spawn, set onto the standing exit-1 list.

@history_item n=15 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** (6. Log Coverage, amd-gqypaosh · amd-mv3ctbds) — `+ contract/process-exit-detail/exit-1-focus` was made a row of its own because the standing exit-1 row would pass its bound, and its no-child-started clause does not hold where focus has spawned hyprctl. This is the closest precedent for an adopt early-exit word after the child spawned: a per-verb exit-1 row rather than a longer `contract/process-exit-detail/exit-1`.

@history_item n=16 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (6. Log Coverage, amd-otyhke6a · amd-4ohqryia) — `+ contract/process-exit-detail/exit-1-up` (up's exit 1 from no-session, cwd-missing, session-live or not-taken) and `+ contract/process-exit-detail/boot-time-unread`, each a new row because the exit-1 row cannot take it within its bound. This shows how per-instance start outcomes, cwd-missing among them, reach a verb's exit and detail, and the same bound applies here.

@history_item n=17 marker=2026-11-24-switching-a-session-s-model-and-effort
**2026-11-24-switching-a-session-s-model-and-effort** (4. Span / Trace Coverage, amd-f7kltdky) — `~ contract/claude-child-process-log/handle-wait-exit`: process-exit{subject:"claude-child"} is written once each child is reaped or the pump returns, carrying child_exit_status and exit_source, never on reader EOF. `~ contract/run-process-log/process-exit-self` follows the child the pump ends on, and `~ decision/launch-line-untraced` covers it too. A child that exits at once is already recorded by this pair plus run's self process-exit; any new word rides on or beside these lines.

@history_item n=18 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (1. Obs Scope Summary and 4. Span / Trace Coverage, amd-6fxc5tvh · amd-xp7a45vz) — `~ contract/run-start-events/order` (wheel{cause:"start"}, lineage, launch, then session-start), `~ decision/product-order-from-events-log`, `+ decision/launch-line-untraced`. These are the start's product lines, and lineage and launch are untraced by rule. An event recording an at-once exit would join this order before or without session-start, so the order row must be set if a new event is added.

@history_item n=19 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** (2. Telemetry Strategy, amd-b3kn6ucj) — `~ decision/events-stream-never-rotated`: events.ndjson is never rewritten, and a reused instance name keeps the earlier starts' lines at its head, so a start's own lines are read from where that start begins. This bears on whether the name is released after an early exit: a re-adopt under the same name appends to that stream.

@history_item n=20 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — a `gone` holder of a name is taken over with no exit-1 line (collision_check outcome `free`), while live and stale holders refuse. Whether the instance directory and name are released after an at-once exit is decided by which holder state the dead start leaves behind.

@history_item n=21 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** (amd-ye23emtv · amd-v67jn6xb · amd-es35y2ig · amd-mt35652l, ratified 2026-11-22) — `~ decision/claude-boundary-only-instrumentation` (the start's one trust-record line), `~ decision/claude-internals-not-a-contract` (a verified start sets its folder's trust flag only), `~ decision/refusal-telemetry` (the trust write refuses a symlinked, non-regular or foreign config with no refusal line), `+ contract/start-trust-log/trust-record`, `~ decision/trust-record-write-uninstrumented`. Moving adopt's spawn folder moves the folder this step trusts in the operator's claude config, with no span and only the one word.

@history_item n=22 marker=2026-11-21-mcp-calls-bounded
**2026-11-21-mcp-calls-bounded** (1. Obs Scope Summary, amd-mupfskrf) — `~ decision/claude-internals-not-a-contract`: transcripts and bundle strings are never read as a contract. The chunk's hypotheses depend on where the CLI keeps a session (hypothesis A, the folder it was made in; hypothesis B, no transcript on disk). Designing on a transcript file's location or presence would read claude internals as a contract, which this row and `decision/transcripts-not-a-contract` forbid beyond the named echo exception.

@history_item n=23 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — verify's Run E is the `--resume <id>` start of Run B's first session id, and `resume-session` is relied on in the capability ledger. This is the one ledger row about resume. The scope's measurement of what the ledger holds about a resume from another folder reads this row; it is measured from the same folder only.

@history_item n=24 marker=2026-11-22T17-29-41Z-wrap
**2026-11-22T17-29-41Z-wrap** (3. Observability Harness Contract, amd-u2v2pm5c) — `~ contract/run-terminal-output/child-output`: run writes nothing to the terminal except the child's output and, before the child, a keyed start's automatic-verify lines; none under revive --json; while the child runs, neither run nor a revived start writes anything of its own. A said line for an at-once exit is a new terminal write after the child, which this promise must name if the chunk adds one.

@history_item n=25 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the refusal writer's site counts, amd-uo5ljwnp) — the `human::refuse(` census reads run 7 and verify 5, by the rule `grep -rcE 'human::refuse\(' src`, the start's refusal pairs being fixed stderr lines (`decision/stderr-fixed-strings`). A new adopt hint or pair through `human::refuse` moves this census and must stay a fixed string with no path.

@history_item n=26 marker=2026-10-10-budget-governor
**2026-10-10-budget-governor** (a refused instance check writes no line, amd-4qeubhvt) — revive's start arm runs its instance check before log init, and strict-modes-failed left its exit-1 detail codes. This is the origin of `decision/pre-log-strict-refusals-write-no-line` and of the `run-exit-1-detail` set adopt's start shares.
