@history_item n=1 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (1. Obs Scope Summary) — ~ contract/run-start-events/order: a start writes wheel{cause:"start"}, then lineage, then one launch line before session-start; ~ decision/config-observed-at-load: a bad default_model / default_effort value writes one parse-rejected config-json malformed line per key. The launch line is the record a resume may replay (scope premise); a switch that records a new pair must decide whether it appends another launch line or a new kind, and the start order row is the bound it must not break.

@history_item n=2 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (4. Span / Trace Coverage) — ~ decision/product-order-from-events-log (wheel then lineage then launch before session-start) and + decision/launch-line-untraced (launch line is log-only audit data: no span, no process-log line, no terminal byte, wakes no wait; PROVISIONAL on the delegate). The precedent for an events-log pair record: a switch record would be its sibling and must take the same untraced stance or name why not.

@history_item n=3 marker=2026-11-23-model-and-effort-at-launch
**2026-11-23-model-and-effort-at-launch** (3. Observability Harness Contract) — ~ contract/config-json/known-keys: seven known keys incl. default_model, default_effort; any other key counted skipped in one unknown-keys line. A switch that adds no config key leaves the count; one that does moves it 7→8.

@history_item n=4 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (4. Span / Trace Coverage) — ~ decision/launch-line-untraced and ~ decision/lineage-line-untraced extended to «run, revive and adopt start»; ~ decision/home-tree-check-before-start-log gains adopt; + decision/adopt-start-logs-as-run. Latest precedent for a new verb riding run's start path (as a restart-based switch through revive would): every start-list row names each start, and a new start-like path must be added to each.

@history_item n=5 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (3. Observability Harness Contract · 2. Telemetry Strategy) — ~ decision/instance-check-before-log-init and ~ decision/home-tree-check-before-log-init gain adopt's start; ~ contract/strict-modes-refusal-exit/1 gains adopt (exit 1, --json strict-modes-failed document). A new verb acting on an instance joins these closed lists: instance check before any log, a refusal opening and writing nothing.

@history_item n=6 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (6. Log Coverage · 1. Obs Scope Summary) — + contract/process-exit-detail/exit-1-adopt (the standing exit-1 list could not take more within its bound); ~ contract/exit-1-detail/session-live and ~ contract/run-exit-1-detail/session-live, ~ decision/session-live-check-fails-open and ~ contract/diag-subject/agents-probe gain adopt. Trap: the exit-1 detail row is at its bound, so a switch's own exit-1 words need their own row, as adopt, up and focus took.

@history_item n=7 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (6. Log Coverage · 1. Obs Scope Summary · 3. Observability Harness Contract · 2. Telemetry Strategy) — ~ contract/start-trust-log/trust-record, ~ decision/r8-strip-logged-as-names-only, ~ decision/env-vars-not-configuration, ~ contract/socket-dir-refused-exit/codes, ~ contract/diag-detail/socket-dir-refused each now name adopt's start (PROVISIONAL, delegate overseer). A switch restarting through run's start inherits these: the CLAUDE_CONFIG_DIR read and trust-record line are the only claude-config touch allowed, so the switch's «no side effect on the user's settings» rests on them.

@history_item n=8 marker=2026-11-24-viola-adopt
**2026-11-24-viola-adopt** (8. PII Scrubbing & Compliance · 7. Error Capture & Reporting) — + decision/adopt-session-id-never-logged (operator-typed input kept out of every line); ~ contract/cli-internal-error/human-line gains adopt's start among roles that print nothing at the catch site. Precedent for classing a new verb's typed argument (model / effort word) and its catch-site behaviour.

@history_item n=9 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (4. Span / Trace Coverage · 6. Log Coverage) — + contract/cli-log-file/up, + contract/process-exit-detail/exit-1-up, + contract/process-exit-detail/boot-time-unread (PROVISIONAL). up is one of the resumes the switch must make replay the new pair; precedent that each new cli verb gets its own cli-log-file row and its own exit-1 detail row.

@history_item n=10 marker=2026-11-24-viola-up
**2026-11-24-viola-up** (1. Obs Scope Summary · 2. Telemetry Strategy · 3. Observability Harness Contract) — ~ decision/strict-modes-failure-lines and ~ contract/process-exit-detail/strict-modes-failed gain up's post-scan home-tree refusal; ~ contract/strict-modes-refusal-exit/21 and /1 gain up; ~ decision/strict-modes-before-diagnostics gains its except. Constraint: a strict-modes refusal writes a line only at the named places; a switch adding another must amend these «only» rows.

@history_item n=11 marker=2026-11-24-viola-focus
**2026-11-24-viola-focus** — + contract/cli-log-file/focus (manual span cli.focus into the named instance's cli-<name>.ndjson), + contract/process-exit-detail/exit-1-focus, ~ decision/instance-check-before-log-init and ~ contract/strict-modes-refusal-exit/21 gain focus (PROVISIONAL, delegate). Nearest precedent for a short-lived verb acting on a running named instance: instance check first, exit 21 on refusal, its own dispatch span and log file.

@history_item n=12 marker=2026-11-23-viola-open-2
**2026-11-23-viola-open-2** — ~ contract/process-exit-detail/exit-1 gains open's words (the row then reached its bound); + contract/cli-log-file/open; + decision/window-line-untraced and + contract/run-start-events/window-line (a start product line kept untraced, its own order row since the order row was full); ~ contract/strict-modes-refusal-exit/1 gains open. Trap: contract/run-start-events/order cannot take another line within its bound; a new start-time record goes in a row of its own.

@history_item n=13 marker=2026-11-22-lineage-registry
**2026-11-22-lineage-registry** — ~ contract/run-start-events/order and ~ decision/product-order-from-events-log gain the lineage line; + decision/lineage-line-untraced; + decision/lineage-names-confined; ~ decision/link-names-never-in-process-log. First sibling of the start product lines and the confinement rule for self-reported values on product surfaces only: a recorded pair is the same kind of product record.

@history_item n=14 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — ~ decision/events-stream-never-rotated: events.ndjson never rewritten, a reused name keeps earlier starts' lines, a start's own lines read from where it begins; ~ contract/run-start-events/order and ~ decision/product-order-from-events-log for wheel{human-input}. Bears on «the record a resume replays is the last launch line»: the switch record must be appended, never rewritten, and readers take the latest.

@history_item n=15 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — revive's start arm logs as role run into run-<name>.ndjson; its preflight exit-1 details (no-session, cwd-missing, later session-live); human::refuse gains revive. A restart-based switch through revive with --model / --effort rides this role and these codes.

@history_item n=16 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — revive --list is role cli (role_of decides before --), logging only under a parsed VIOLA_NAME after its check; revive --json refusals print {"v":1,"error":"<code>","detail":null}. Precedent for which role a new revive-adjacent arm or verb logs under and for its --json refusal shape.

@history_item n=17 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — session-live joins exit 1 from revive's preflight, its agents-probe pair and claude-agents-json parse-rejected; schema detail enum grew. A switch that ends and resumes a session meets the same session-live reading and its lines.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — the cli catch site's --json internal-error document; every revive --json start refusal prints its document; exit 21's document names its cause (UnreachableCause); stale-heartbeat split from already-live; process-exit detail enum counted by its members; verify writes its refusal code. The catalogue a new verb's exit codes, closed words and --json document must be placed beside (scope fct-qkvpommq).

@history_item n=19 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** (Founder Direction 4's codes and named carve-outs) — every exit-21 and exit-1 cause gets a distinct detail code with no path or pid, except a strict-modes refusal before any log opens. Binds decision/distinct-detail-code-per-exit-cause and decision/exit-1-detail-codes for the switch's causes.

@history_item n=20 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — exit 21's causes five→six with socket-dir-refused; the frame verbs' socket-directory refusal line (exit 21, during connect, WARN, no channel-request); the order before any frame (check_endpoint_dir, connect, euid, verify_server); refused peer dropped with no line; human::refuse census. If the switch is a channel call from the CLI, it is a ninth frame verb and inherits every one of these.

@history_item n=21 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — new served channel methods link / unlink: no span, no process-log line of their own, traced by the channel pair, an events.ndjson line and the snapshot; -32602 invalid-params causes; channel-response method logged only for a closed-list name; server-verify-failed six→eight verbs; verbs log into the driven instance's cli file inside cli.link / cli.unlink; link names classed in §8. The closest precedent for a new channel method that records a product line and changes snapshot state.

@history_item n=22 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — every CLI frame verb verifies the wrapper inside its dispatch span before any frame, a refusal opening no channel.request and writing one exit-21 server-verify-failed line; captured-span cases per verb. A new frame verb owes the same place and case.

@history_item n=23 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — the frame verbs' instance check through client::refuse_untrusted before any log (no line on refusal); run's and revive's home-tree check before the wrapper log; the refusal-telemetry record (two strict-modes line sites, every other refusal none); replayed snapshot carries no endpoint and frame paths read it unreachable; E5 replay readers. Constraints a new instance verb and its snapshot reads rest on.

@history_item n=24 marker=2026-10-20-t06-58-31-wrap
**2026-10-20-t06-58-31-wrap** — the founder accepted the refused checks' no-line shapes as built. Settles decision/strict-modes-refusal-writes-no-line for a new verb's refused check.

@history_item n=25 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — channel-response detail closed to three -32602 words; v_rejected counts version rejections alone; peer replies screened by sanitised_fault and is_code_word; error-body census requires every product error writer named; mcp writes no self process-exit. A new verb's error bodies and any new -32602 cause must fit these closed words and be named in the census.

@history_item n=26 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — corr required in diag-line for channel-request / channel-response; mis-shaped conn logged as srv_conn, stripped before dispatch. A new channel method's lines carry corr as the JSON-RPC id under these rules.

@history_item n=27 marker=2026-10-04-the-wheel
**2026-10-04-the-wheel** — pause / release traced as pause.client / release.client (CLIENT) › channel.request; release-from-driver line on a release carrying a string from; human-input cause. Bears on the scope's unmeasured «whose wheel the switch needs»: a wheel-gated verb's spans and refusals follow this shape.

@history_item n=28 marker=2026-10-04-running-turn-refusal
**2026-10-04-running-turn-refusal** — turn-running covers a running turn or another send in flight, as in-memory wrapper state with no event, span or line of its own. Bears on a switch mid-turn (scope premise fct-azrutr2y).

@history_item n=29 marker=2026-11-16-send-confirmation
**2026-11-16-send-confirmation** — ~ contract/send-refused-detail/turn-running bounded to a turn the wrapper knows of before the paste (a later UserPromptSubmit stays undetected). A switch's mid-turn guard inherits this bound.

@history_item n=30 marker=2026-11-16-local-command-delivery
**2026-11-16-local-command-delivery** — a local command's transcript echo as a ratified confirmation (decision/claude-internals-not-a-contract and decision/transcripts-not-a-contract excepts, decision/deterministic-event-emission except, decision/refusal-telemetry except, + decision/transcript-echo-read-uninstrumented). Bears on a switch typed into the live session as a command: the one transcript read allowed and how such a send confirms.

@history_item n=31 marker=2026-10-06-local-command-send-outcomes
**2026-10-06-local-command-send-outcomes** — an unconfirmable listed command opens no confirm window: one send-issued and one send-confirmed{confirmed:false}; /clear confirmed by its session-start. Bears on whether a typed model command would be confirmable.

@history_item n=32 marker=2026-11-19-context-as-a-local-command
**2026-11-19-context-as-a-local-command** — + contract/confirm-window-close/context: a newly listed local command confirmed by its stdout tag gets its own close row. Precedent if the switch adds a listed local command.

@history_item n=33 marker=2026-11-18-session-folder-trusted-by-viola
**2026-11-18-session-folder-trusted-by-viola** — the start writes the claude config's trust flag only (claude-internals-not-a-contract except), one trust-record line (+ contract/start-trust-log/trust-record, + contract/diag-line-fields/trust-record), CLAUDE_CONFIG_DIR read by the start; ~ decision/trust-record-write-uninstrumented. The single sanctioned write into ~/.claude*; the switch's «writes nothing to the user's settings» must not widen it.

@history_item n=34 marker=2026-11-17-trust-record-ledger-row
**2026-11-17-trust-record-ledger-row** — verify writes and removes its temp folder's trust record; ~ decision/refusal-telemetry except; + decision/trust-record-write-uninstrumented; verify's spawn pair counts stated by rule. Any measurement of the switch in verify extends these rows; the claude config writer refuses links and foreign files with no line.

@history_item n=35 marker=2026-11-22-paste-write-bounded
**2026-11-22-paste-write-bounded** — paste-cut joins the closed send detail and refusal-reason lists; ~ contract/send-path-log-events/event-order; + decision/send-cut-mark-uninstrumented. Precedent for extending the closed send-refused words if the switch rides a paste.

@history_item n=36 marker=2026-11-14-budget-gate-removed
**2026-11-14-budget-gate-removed** — ~ decision/exit-code-log-field: exit 11 retired and never reused; ~ contract/channel-response/invalid-params-causes; diag schema rules narrowed to «a field the product still emits» (decision/diag-schema-additive-only, decision/log-fields-additive-only). A switch's exit codes may not reuse 11; schema additions stay additive.

@history_item n=37 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** (the closed words that left diag-line.v1.json with their emitter) — rule: a closed schema word no build emits leaves with its emitter; os closed to linux; process-exit detail enum recounted. A new detail word enters the schema only with its emitter.

@history_item n=38 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** (the snapshot readers over a replayed snapshot) — a snapshot rebuilt by replay carries no endpoint, pid, started_at; product readers read through read_snapshot_or_replay. The snapshot's launch pair (scope premise) must survive replay if resumes are to replay a switched pair.
