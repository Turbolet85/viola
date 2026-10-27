# Task — obs-plan · 2026-10-26-home-and-code-bearing-file-integrity

## The session
Built the home's strict-modes check before every read or open at every entry point: the six frame verbs (send, wait, last, answer, pause, release), the hook event and dialog paths, the mcp frame tools, verify, run and revive's start, the diagnostics opens and the conpty/ companions.
Added the read rule (Unix: no group or other bit on the home, the instance dir and diagnostics/; Windows: no read grant to another SID, READ_RIGHTS), the tree checks check_home_tree, check_diagnostics and check_present, the Refused variants Readable and Symlink, StampsError, pin_key and pin_bytes, and refuse_untrusted in client.rs.
Retired all eight founder-ratified dated strict-modes gaps that were owed to the route entry Home and code-bearing file integrity, and retired read_snapshot as a product reader under src/ (3 callers to 0): hook connect, statusline serve, run's collision_check and live_endpoint now read through read_snapshot_or_replay.
By the founder's own answer (inputs#I3 Q2), every new refusal site writes no line. The six frame verbs print link's instance pair (exit 21). run and verify print the home pair (exit 1).
The process-start sideload_fallback enum gains strict-modes-failed. Measured: CI green 15/15 on three OSes (ci#40403389629). run --perf keeps the five hook rows under SPINE_DEADLINE with the check in the timed path.

## Reached files
- crates/viola-state/src/strict.rs
- crates/viola-state/src/stamps.rs
- crates/viola-state/src/liveness.rs
- crates/viola-state/src/pin.rs
- crates/viola-mcp/src/tools.rs
- src/cmd/client.rs
- src/cmd/link.rs
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/hook.rs
- src/cmd/hook/statusline.rs
- src/cmd/run.rs
- src/cmd/revive.rs
- src/cmd/verify.rs
- src/obs.rs
- schemas/diag-line.v1.json
- tests/security_negatives_strict_modes.rs
- tests/chaos_snapshot_replay.rs
- crates/viola-mcp/tests/handlers.rs
- tests/hook_fail_open.rs
- tests/hook_statusline.rs
- tests/run_cli.rs
- tests/cli_verify.rs
- tests/conpty_sideload.rs
- tests/cli_controls_not_disableable.rs
- tests/support/home.rs
- tests/cli_revive.rs
- tests/cli_wait_last.rs
- tests/contract_diag_schema.rs
- tests/cli_machine_contract.rs
- tests/tui_env_strip.rs
- viola-0.1.0/verification-matrix.json
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/scope-record.md
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/evidence/remove-the-guard.md
- viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/evidence/operator-pass.md

## Touches
- §3 harness bullet (obs-plan.md:311): the human::refuse call-site count moved from 15 to 18. run went from 5 to 7, verify from 5 to 6, and link's one site moved to client.rs, shared by link, unlink and the six frame verbs. The figures were re-derived at chunk 2026-10-26-explicit-session-links, and that per-file list is now stale. run's start-refusal list gains a refused home tree (the home pair, or the instance pair for an instance-level refusal).
- Obs anti-patterns, logs (obs-anti-patterns.md:8): the human-stderr refusal rule names no figure, so re-read it against the three new refuse sites.
- wait / last scenario (obs-plan.md:361 and span-trace-coverage.md:102): the sentence that strict-modes-failed cannot occur, with its check owed to Home and code-bearing file integrity, is stale. A refused instance check now exits 21 with the instance pair, or the instance-unreachable document under --json, with no frame and no line. MCP's wait and last return instance-unreachable with no line.
- Every row that names Home and code-bearing file integrity as the owner of a gap, a CARRY or a not-yet-built case (owner-title, 2 obs-plan rows): that entry closed the item.
- §1 Vector 7 trigger (obs-plan.md:215): the claim that a strict-modes failure has a line only on the ledger-stamps read in run's version gate needs re-reading. The new refusal sites write no line, but a refused conpty/ companion now shows as process-start sideload_fallback strict-modes-failed. The symlink refusal telemetry now also has to cover check_diagnostics refusing a symlinked diagnostics folder or file.
- sideload_fallback word list, 4 words to 5 (+strict-modes-failed: hash-mismatch, strict-modes-failed, unreadable, load-failed, not-built): log-coverage.md:10, span-trace-coverage.md:55 and :56, the run-start scenario's statement that a sideload outcome is never a refusal or an exit change, and its D-36-class Decisions Log entry.
- E5 (obs-plan.md:394) and its emitter rows (span-trace-coverage.md:191 and :192, telemetry-strategy.md:33): the product readers on read_snapshot_or_replay went from 2 (revive, list) to 6, adding hook connect, statusline serve, run's collision_check and live_endpoint. The claim that the other snapshot readers write no state-recovered line now covers no product reader under src/. span-trace-coverage.md:30 also names a reader that stays on read_snapshot.
- metric-coverage.md:25: the state-recovered derived count is now reached by four more readers. The event and its detail words are unchanged.
- No-line exemption lists (error-capture-reporting.md:15, span-trace-coverage.md:18, :54 and :217, obs-scope-summary.md:300, slo-invariants-telemetry-budgets.md:17, log-coverage.md:35): each new no-line refusal site joins them. The sites are the six verbs, the hook event and dialog paths, the mcp frame tools, verify, run and revive's tree check, and the diagnostics opens.
- §7 MCP rejects (obs-plan.md:537): instance-unreachable with no line now also covers a frame tool's refused check_instance, which runs before live_endpoint.
- §7 panic window before init step 4 (obs-plan.md:529) and the §10 bounded panic exemptions where no file exists: the instance and home checks before any role file now also run in the hook event and dialog paths, the six frame verbs, verify, and run and revive's start.
- Hook surface exporter (obs-plan.md:119) and E1 (obs-plan.md:388): the hook is a silent no-op without VIOLA_NAME. It is now also silent for a refused instance and for a VIOLA_DIR naming no existing directory: no config.json read, no role file, no stdin read, no line, no frame, no SessionEnd append.
- §3 hook stdout bullet (obs-plan.md:310): statusline prints nothing over a replayed snapshot, because a replay carries no statusline_command.
- The hook-decision detail channel-unreachable (log-coverage detail catalog): hook connect over a replayed or absent snapshot is now NotSent::Unreachable, failing open.
- Run-start scenario (span-trace-coverage.md lines 39 to 60) and obs-plan.md:161: exit-1 causes use a distinct detail code. But run and revive's refused home tree exits 1 before open_wrapper_log, with no log, no line and no child, so that cause has no detail code. This also bears on the Founder Direction 4 trigger (obs-plan.md:253).
- verify / plugin install log coverage (obs-plan.md:400): verify now refuses a widened home tree (the home, ledger/, bin/, plugin/) before its cli log, with no process-start, no step line and no summary. update_stamps refuses before reading, through StampsError::StrictModes.
- Capability ledger bullet (obs-plan.md:398): the strict-modes-refused ledger read in run.version_gate is re-read and unmoved. The new fact is on verify's write side (above).
- check_instance rows (check-instance, 5 obs-plan rows): what the check reads widened. The home, the instance dir and diagnostics/ (new in the set) now fall under the read rule, and so does Windows events.ndjson. Its call sites moved: statusline.rs to hook.rs (trusted), and link.rs to client.rs (refuse_untrusted). The count stays at 3.
- The read rule in the registries row otel-sdk-init.md:15 (@c1154, read-rule) and Vector 7's diagnostics/ 0600-in-0700 (Windows: home DACL) rule: re-read them against the read rule as built. check_diagnostics refuses a widened diagnostics file on Unix and holds it to the write rule on Windows.
- §10 build failure (a diagnostics/ file not 0600 or not DACL-protected): the product now refuses such a file at open, so the role or detail writer ends at sink and the line is silently dropped. Re-read it against the default-deny and zero-unlogged-panics claims.
- Catch-site reporter (obs-plan.md:496, error-capture-reporting): its reach narrowed. On Unix, a home under a regular file is now a strict-modes refusal (verify: exit 1, home pair; frame verbs: exit 21, instance pair) where the cli catch-site line used to stand. On Windows such a path stats NotFound and the line stands.
- §10 performance budgets (slo-invariants-telemetry-budgets): obs overhead on hook was re-measured with the instance check in the timed path: run --perf max 2.5, 2.4, 2.4, 2.4 and 3.3 ms, all under SPINE_DEADLINE. The claim is unmoved.
- Chaos-instrumentation trigger and E5 coverage: E5 steps 1 to 3 are now built (tests/chaos_snapshot_replay.rs). E5's signals are restated: after step 3, wheel holder human does not hold, since every start appends wheel{holder:"driver", cause:"start"}, and the replay writes no file. span-trace-coverage.md:138 states the wheel contract and is re-read as true.
- cli surface (obs-plan.md:111): the cli verbs' refusals now also come from the strict-modes check before any process-log file is opened. Its fixed stderr lines keep no paths or pids.

## New facts
- F1 · span-trace-coverage · state.snapshot_recover now sits under run.start before run.collision_check, and under hook.handle on the hook path. Two span-order oracles list it.
- F2 · span-trace-coverage · revive's preflight makes one snapshot read and hands it to both its collision check and its recorded-directory reading: one replay, one state-recovered line per revive.
- F3 · span-trace-coverage · run's collision_check over a replayed snapshot judges liveness from the heartbeat alone: beat_age at or under STALE_AFTER is already-live, otherwise the name is free. An error or Absent is free.
- F4 · obs-scope-summary · send, wait, last, answer, pause and release each first run refuse_untrusted (check_instance on the home and instances/<name>). A refusal exits 21 with the instance pair, or under --json exactly the instance-unreachable document with detail null. Nothing under the home is opened and no line is written.
- F5 · obs-scope-summary · the hook event and dialog paths take canonical, then trusted, before read_diagnostics_level and obs init. A refused or absent instance exits 0 at once: no config.json read, no role file, no stdin read, no line, no frame, no SessionEnd append. The two capture arms stay ahead, and FAKE_AGENT_HOOK_PANIC stays after obs init.
- F6 · obs-scope-summary · a hook whose VIOLA_DIR names an instance directory that does not exist canonicalises to no session and writes nothing.
- F7 · error-capture-reporting · the viola mcp frame tools run check_instance before live_endpoint. A refusal returns instance-unreachable with detail null, no frame and no line.
- F8 · span-trace-coverage · run and revive's start run check_home_tree before open_wrapper_log, through checked_exe and refuse_tree. The tree is the home (read rule), the instance set, ledger/, bin/, plugin/, bin/<key>/, the pinned exe, plugin/<key>/ and its hooks/ and .claude-plugin/. A refusal exits 1 with the home pair (home level) or the instance pair (instance level): no log, no line, no child. Revive under --json keeps strict-modes-failed.
- F9 · obs-scope-summary · verify runs check_home_tree on the home, ledger/, bin/ and plugin/ before its cli log. A refusal exits 1 with the home pair, no step line, no summary and no line. A StampsError::StrictModes from update_stamps maps to the same pair.
- F10 · log-coverage · open_role_file and write_detail run strict::check_diagnostics on the existing diagnostics/ folder and on the role or detail file before ensure_private_dir. A symlink is refused, a folder is held to the read rule, and a file to the read rule on Unix and the write rule on Windows. A refusal is a failed open, so the writer ends at sink and a detail line is dropped.
- F11 · log-coverage · a conpty/ companion that its owner and DACL check (check_present, before the held open) refuses is PinError::StrictModes. The child then runs on the inbox ConPTY with process-start sideload_fallback strict-modes-failed, the exit unchanged and nothing written.
- F12 · obs-scope-summary · a replayed snapshot carries no endpoint, pid, started_at or statusline_command. live_endpoint over it is None (unreachable), hook connect is NotSent::Unreachable (fail open, channel-unreachable), and statusline serve runs and prints nothing.
- F13 · obs-scope-summary · the founder ruled no line for every new refused check (inputs#I3 Q2), as for link and unlink: logging strict-modes-failed after a refused check was rejected.
- F14 · error-capture-reporting · on Unix, a home under a regular file is a strict-modes refusal with no line (verify: exit 1, home pair; frame verbs: exit 21, instance pair), not the cli catch-site line. On Windows the path stats NotFound and the catch-site line stands.

## Untouched
- §5 metric naming, label cardinality and viola.compat.v_rejected: no -32602 path or channel method changed, and no metric instrument was added.
- §9 CI integration gates G1 to G4, step order, artifact uploads, retention and the secret scan: the report states no harness, CI or gate change.
- §8 data classes, veil, the source allow-list and diag-detail.v1.json: the session adds no log field and no detail-file content field. Its one schema change is an enum word in diag-line.v1.json.
- §7 platform pick, the reporter preconditions and the panic hook's one-line record: no dependency changed and the panic writer is unchanged.
- The torn-append healing bullet: the appender and its state-recovered torn-line-healed line are not reached.
- §1 instrumentation scope entity list: no crate or module was added or removed.
- The Section 12 Decisions Log diagnostics root (D-08, D-22), MSRV floor and channel join key (D-25): not reached.
- §11 OTel, exporter, APM and dashboard bans: no telemetry dependency or sink was added.
- The viola list refusal sites (list 2) and the board read: board.rs is unchanged. Its home-check inconsistency is routed to another entry, not to obs-plan.
