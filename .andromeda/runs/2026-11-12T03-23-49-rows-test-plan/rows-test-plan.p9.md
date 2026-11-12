@row_add n=1 row=contract/statusline-settings-source/source-file area=e2e-test-strategy rec=1
boundary: statusline settings source
item: source-file
promise: viola reads the user statusline command from <home>/statusline-source.json (the Claude settings shape) when it exists, else in the default home from the user's .claude/settings.json, and in any other home from none; viola only reads it.
@row_add n=2 row=contract/statusline-payload/resets-at area=e2e-test-strategy rec=1,2
boundary: statusline payload
item: resets_at
promise: The statusline reader takes resets_at as epoch seconds or RFC 3339; a malformed value is recorded as "unknown" in budget.json.
@row_add n=3 row=decision/override-expiry-tested-at-unit-level area=e2e-test-strategy rec=1
name: Budget override expiry tested at unit level
rule: Budget override expiry is asserted only in unit cases of the pure gate decision, which takes the instant as an argument, never end to end.
why: No in-process clock reaches the separately spawned wrapper, and waiting in real time would break the sleep(N) ban.
guard: gate_decide_an_expired_override_is_dropped, gate_release_override_ends_at_its_instant
@row_add n=4 row=contract/budget-paused-refusal/send-exit-11 area=e2e-test-strategy rec=2
boundary: budget-paused refusal
item: send exit 11
promise: A send while the budget is paused exits 11 with detail naming the window, "five-hour" or "seven-day".
@row_add n=5 row=decision/five-hour-pause-threshold area=e2e-test-strategy rec=2
name: Five-hour pause threshold at 90
rule: A five-hour reading of exactly 90 percent pauses automated sends, and 89 does not.
guard: path6_the_boundaries_and_the_window_word
@row_add n=6 row=contract/statusline-hook/stdout area=e2e-test-strategy rec=2
boundary: viola hook statusline
item: stdout
promise: The wrapped statusline prints the user's command output byte for byte, with exit 0 and empty stderr.
@row_add n=7 row=decision/statusline-command-shell-and-bound area=e2e-test-strategy rec=2
name: Statusline command run through sh under a bound
rule: The wrapped statusline arm runs the recorded user command as /bin/sh -c <command> under a 5 s bound.
state: provisional
@row_add n=8 row=contract/budget-json/write-condition area=e2e-test-strategy rec=2
boundary: budget.json
item: write condition
promise: budget.json is written only when the statusline payload holds a rate_limits object.
@row_add n=9 row=contract/budget-gate-record/paused-true area=e2e-test-strategy rec=2
boundary: budget-gate record
item: paused true
promise: A budget pause logs a budget-gate{paused:true, window} record.
@row_add n=10 row=contract/budget-gate-record/paused-false area=e2e-test-strategy rec=2,3
boundary: budget-gate record
item: paused false
promise: A per-instance release logs one budget-gate{paused:false, override_until} record for that instance alone, its override_until later than its ts and equal to the instance snapshot's budget_override_until.
@row_add n=11 row=contract/release-budget-json/document area=e2e-test-strategy rec=2
boundary: release --budget --json document
item: document
promise: release --budget gives {budget_paused:false, override_until} when a pause stood and {budget_paused:false} alone when none did; the next send exits 0.
@row_add n=12 row=decision/statusline-command-never-runs-unsafe area=e2e-test-strategy rec=2
name: Statusline command never runs over unsafe state
rule: The user statusline command never runs when strict-modes fails or over a replayed snapshot; the arm then exits 0 and prints nothing.
guard: statusline_over_a_replayed_snapshot_runs_nothing
@row_add n=13 row=contract/list-json/budget-paused area=e2e-test-strategy rec=2
boundary: list --json
item: budget_paused
promise: The top-level .ok.budget.paused reports the budget reading's pause, while each item's budget_paused reports that instance's own gate, false under a per-instance override.
@row_add n=14 row=decision/budget-override-per-instance area=e2e-test-strategy rec=2,3
name: Budget override is per instance
rule: release --budget lifts the pause for the named instance alone; other instances under the same reading stay paused and are refused.
guard: path6_two_instance_release_over_a_harness_session, gate_release_on_one_instance_leaves_another_paused
@row_add n=15 row=decision/driver-hints-never-name-release area=e2e-test-strategy rec=2,16
name: Driver-facing hints never name release
rule: A driver-facing refusal hint (human-typing, budget-paused) never contains release.
why: Drivers are refused release.
guard: no_hint_names_release, path6_the_refusal_hint_never_names_release
@row_add n=16 row=contract/release-verb/driver-caller area=e2e-test-strategy rec=2
boundary: release verb
item: driver caller
promise: A driver calling release is refused with -32602, exit 20.
@row_add n=17 row=decision/unknown-reset-does-not-pause area=e2e-test-strategy rec=3
name: Unknown or passed reset does not pause
rule: A budget window whose resets_at is "unknown", unparseable or already passed does not pause.
guard: path6_an_unknown_or_passed_reset_does_not_pause
@row_add n=18 row=decision/release-without-pause-records-nothing area=e2e-test-strategy rec=3
name: Release with no pause records nothing
rule: release --budget with no pause standing records nothing.
guard: path6_release_budget_lifts_the_pause
@row_add n=19 row=decision/budget-override-ends-with-wrapper area=e2e-test-strategy rec=3
name: Budget override ends with its wrapper
rule: A budget override does not outlive its wrapper, so a restart over the same reading pauses again.
guard: path6_a_restart_pauses_again
@row_add n=20 row=decision/budget-gate-covers-send-only area=e2e-test-strategy rec=3
name: Budget gate covers send only
rule: The budget gate refuses automated send alone; answer, wait, last and pause behave under a closed gate as under an open one.
guard: path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone
@row_add n=21 row=contract/mcp-tool-result/budget-paused area=e2e-test-strategy rec=3
boundary: MCP tool result
item: budget-paused
promise: A budget-paused send is isError:true with {"refusal":"budget-paused","detail":"five-hour"} and no cursor, equal to the CLI's document less v.
@row_add n=22 row=contract/list-human/wheel-budget-paused area=e2e-test-strategy rec=3
boundary: viola list human board
item: WHEEL cell
promise: A budget-paused instance's WHEEL cell reads "driver - budget-paused" on that instance's row alone, with no ESC byte.
@row_none n=1 rec=4 type=5
@row_none n=2 rec=5 type=5
@row_add n=23 row=decision/unverified-cli-gates-answer-only area=e2e-test-strategy rec=6
name: Unverified CLI version gates answer only
rule: Under an unverified claude CLI version a send still proceeds and the question event is logged, while answer is refused unverified-cli.
@row_add n=24 row=contract/cli-answer/unverified-cli area=e2e-test-strategy rec=6
boundary: viola answer
item: unverified-cli
promise: answer on an unverified CLI version exits 12 with {"refusal":"unverified-cli","detail":null}.
@row_add n=25 row=contract/mcp-tool-result/unverified-cli area=e2e-test-strategy rec=6
boundary: MCP tool result
item: unverified-cli
promise: MCP answer on an unverified CLI version gives is_error:true with text "refused: unverified-cli".
@row_add n=26 row=contract/verify-output/last-line area=e2e-test-strategy rec=6
boundary: viola verify output
item: last line
promise: A passing verify exits 0 with a last line matching ^stamped \S+  \d+ pass  0 fail$.
@row_none n=3 rec=7 type=5
@row_none n=4 rec=8 type=5
@row_add n=27 row=decision/fd-test-reads-live-child-table area=e2e-test-strategy rec=9
name: fd test reads the live child's fd table
rule: The env-strip and fd tests read the running child's live fd table, never the fake agent's self-dump.
why: The fake agent's own receipt and /dev/fd handle show as extra fds in its self-dump.
@row_add n=28 row=decision/claude-child-env-strip area=e2e-test-strategy rec=10,9
name: claude child environment strip
rule: The claude child receives none of the identity-floor names nor unknown CLAUDE_ names; only names on claude_env_keep survive.
guard: tui_env_strip
@row_add n=29 row=contract/process-start-claude-child/env-stripped-count area=e2e-test-strategy rec=10
boundary: process-start{subject:"claude-child"} record
item: env_stripped_count
promise: Carries the count of environment names stripped from the claude child.
@row_add n=30 row=contract/process-start-claude-child/env-stripped-known area=e2e-test-strategy rec=10
boundary: process-start{subject:"claude-child"} record
item: env_stripped_known
promise: Carries the known stripped environment names, names only, never values.
@row_add n=31 row=contract/process-start-claude-child/env-kept area=e2e-test-strategy rec=10
boundary: process-start{subject:"claude-child"} record
item: env_kept
promise: Carries the environment names kept for the claude child, names only, never values.
@row_add n=32 row=decision/claude-child-holds-nothing-of-viola area=e2e-test-strategy rec=10
name: claude child holds nothing of viola's
rule: The wrapped claude child holds no viola socket and no path under the viola home, and its fds 0 to 2 are the PTY.
guard: tui_channel_fds
@row_add n=33 row=decision/no-fd-count-assertion area=e2e-test-strategy rec=10
name: No fd-count assertion on the child
rule: The child fd test asserts what the child holds, never a count of its descriptors.
why: portable-pty closes every inherited descriptor above 2 at spawn, so a planted leak cannot turn a count red, and the fake agent's own files add fds.
@row_add n=34 row=decision/secrets-never-in-logs area=e2e-test-strategy rec=10,15
name: Secrets never in logs or diagnostics
rule: No stripped-variable value and no Cookie header appears in any log or diagnostics file.
@row_none n=5 rec=11 type=5
@row_add n=35 row=decision/link-driver-self-reported area=e2e-test-strategy rec=12
name: Link driver is a self-reported name
rule: The driver of a link is a self-reported name and need not be a booted instance.
@row_add n=36 row=decision/send-from-another-instance-links-once area=e2e-test-strategy rec=12
name: Send from another instance links once
rule: An issued send from another instance appends one link{driver, driven} record with source wrapper before its send-issued, and a later send of the same pair adds none.
guard: links_a_send_from_another_instance_links_once
@row_add n=37 row=decision/refused-or-own-send-records-no-link area=e2e-test-strategy rec=12
name: Refused or own-name send records no link
rule: A refused send and a send to an instance's own name record no link.
guard: links_a_refused_send_and_an_own_name_send_record_none
@row_add n=38 row=contract/channel-link/already-linked area=e2e-test-strategy rec=12
boundary: wrapper link method
item: already_linked
promise: A second link of an already linked pair appends no record, keeps the first since, and answers already_linked: true.
@row_add n=39 row=contract/channel-unlink/unlink-record area=e2e-test-strategy rec=12
boundary: wrapper unlink method
item: unlink record
promise: unlink appends one unlink record (source: wrapper, {driver, driven}) and empties the snapshot's links; a second unlink appends nothing, and a later link is a new pair with a later since.
@row_add n=40 row=decision/links-survive-wrapper-restart area=e2e-test-strategy rec=12,13
name: Links survive a wrapper restart
rule: A wrapper started over a log restores the link set from it, an unlinked pair restored as absent, writes that set into its first snapshot and appends no line.
guard: links_survive_a_wrapper_restart, links_an_unlink_survives_a_wrapper_restart
@row_add n=41 row=decision/snapshot-replay-from-log area=e2e-test-strategy rec=13
name: Snapshot replayed from the event log
rule: Over an unreadable or newer snapshot.json the product readers replay instance state from the event log, budget pause, override end and links included.
guard: e5_the_replay_reads_what_the_wrapper_wrote, e5_the_replay_reads_the_link_the_wrapper_wrote
@row_add n=42 row=decision/replay-writes-no-file area=e2e-test-strategy rec=13
name: Replay writes no file
rule: The snapshot replay writes no file, so no snapshot is rebuilt from it.
@row_add n=43 row=decision/replayed-snapshot-has-no-endpoint area=e2e-test-strategy rec=13
name: Replayed snapshot has no endpoint or cwd
rule: A replayed snapshot carries no endpoint, pid, started_at or cwd: it gives no live endpoint, hook connect reads the instance unreachable and fails open, and a revive over it is refused cwd-missing.
guard: live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line, connect_over_a_replayed_snapshot_is_unreachable
@row_add n=44 row=decision/collision-check-over-replay-reads-beat area=e2e-test-strategy rec=13
name: Collision check over a replay reads the beat
rule: Over a replayed snapshot the start's collision check reads the heartbeat alone: a beat no older than STALE_AFTER is already-live, an older one leaves the name free.
guard: collision_check_over_a_replayed_snapshot_reads_the_beat_alone
@row_add n=45 row=decision/list-reads-live-through-replay area=e2e-test-strategy rec=13
name: list reads a live wrapper through the replay
rule: list reads an instance with an unreadable or newer snapshot and a fresh beat as live, never stale, through the replay, logging one state-recovered line.
guard: e5_list_reads_a_live_wrapper_through_the_replay
@row_add n=46 row=contract/event-log/start-wheel-record area=e2e-test-strategy rec=13
boundary: instance event log
item: first record of a start
promise: Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} as its first record.
@row_add n=47 row=decision/security-sweep-under-nextest area=e2e-test-strategy rec=14
name: Security sweep runs under nextest alone
rule: The security sweep runs under nextest alone.
@row_add n=48 row=decision/hook-fails-open area=e2e-test-strategy rec=14
name: Hook fails open silently
rule: A hook that cannot proceed (oversize or malformed stdin, clap error, channel failure, panic, missing VIOLA_NAME, no instance dir, refused instance, impostor server, refused socket dir) exits 0 with empty streams and no frame.
guard: hook_fails_open_silently_within_the_spine_bound, hook_dialog_tier_fails_open_silently
@row_add n=49 row=decision/hook-instance-check-before-config area=e2e-test-strategy rec=14
name: Hook instance check before any config read
rule: The hook's event and dialog paths take the strict-modes instance check before any config.json read or observability init.
guard: hook_refused_by_the_instance_check_fails_open_and_writes_nothing
except: The two --capture arms stay ahead of the check.
@row_add n=50 row=decision/fake-agent-panic-seam-test-only area=e2e-test-strategy rec=14
name: Forced-panic seam is test-only
rule: The hook forced-panic seam fires only on exactly 1 and compiles only with feature fake-agent, so a release build of viola never contains it.
why: Ratified in the security-plan Decisions Log.
guard: cli_controls_not_disableable
@row_add n=51 row=decision/hook-panic-payload-to-detail-file area=e2e-test-strategy rec=14
name: Hook panic payload kept to the detail file
rule: A caught hook panic writes one payload-free role panic line, and its payload and backtrace go to one detail line only.
guard: hook_forced_panic_fails_open_with_one_role_line_and_one_detail_line
@row_add n=52 row=contract/caught-panic-exit-code/hook area=e2e-test-strategy rec=14
boundary: exit code on a caught panic
item: hook
promise: hook exits 0 on a caught panic.
@row_add n=53 row=contract/caught-panic-exit-code/run area=e2e-test-strategy rec=14
boundary: exit code on a caught panic
item: run
promise: run exits 1 on a caught panic.
@row_add n=54 row=contract/mcp-tool-input/schema-violation area=e2e-test-strategy rec=14
boundary: MCP tool input
item: schema violation
promise: A schema-violating tool call is invalid-params naming its field, revalidated before any frame, with no send-issued.
@row_add n=55 row=contract/mcp-tool-input/empty-or-control-text area=e2e-test-strategy rec=14
boundary: MCP tool input
item: empty or control-character text
promise: A send text that is empty or holds a control character is not-delivered.
@row_add n=56 row=contract/mcp-stdin/oversize-line area=e2e-test-strategy rec=14
boundary: MCP stdin
item: line past MAX_FRAME
promise: A stdin line past MAX_FRAME ends the session with exit 0 and empty stderr, and logs exactly one parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1} line in mcp.ndjson.
@row_add n=57 row=contract/hook-decision/server-verify-failed area=e2e-test-strategy rec=14
boundary: hook-decision record
item: server-verify-failed
promise: A hook facing a server that is not the recorded wrapper logs one hook-decision line with detail "server-verify-failed", on the dialog path after hook-invoked{corr:null} with decision_emitted:false.
@row_add n=58 row=contract/hook-decision/socket-dir-refused area=e2e-test-strategy rec=14
boundary: hook-decision record
item: socket-dir-refused
promise: A refused socket directory logs one hook-decision line at WARN with detail "socket-dir-refused", on the dialog path after hook-invoked{corr:null} with decision_emitted:false.
@row_add n=59 row=decision/error-bodies-hold-no-path area=e2e-test-strategy rec=15
name: Error bodies hold no path or upstream text
rule: CLI --json documents, MCP isError results and channel error.data contain no absolute path, no serde path and no Caused by.
guard: channel_error_bodies, cli_error_bodies, contract_error_bodies
@row_add n=60 row=decision/error-bodies-held-by-surface-witnesses area=e2e-test-strategy rec=15
name: Error bodies held by surface witnesses
rule: No events helper runs after every E2E test; error bodies are held by three surface witnesses and a census of writers instead.
@row_add n=61 row=contract/cli-json/internal-error area=e2e-test-strategy rec=15
boundary: CLI --json document
item: internal-error
promise: Under --json an internal error prints {"v":1,"error":"internal-error","detail":null} on stdout and leaves stderr empty.
@row_add n=62 row=contract/cli-human/internal-error area=e2e-test-strategy rec=15
boundary: CLI human output
item: internal error line
promise: Without --json an internal error prints the line "error: internal error" byte for byte; run, revive's start, mcp and hook stay silent there.
@row_add n=63 row=contract/channel-error/-32603 area=e2e-test-strategy rec=15
boundary: channel error
item: -32603
promise: Every -32603 is exactly "internal error" with data null.
@row_add n=64 row=decision/internal-error-held-at-unit-layer area=e2e-test-strategy rec=15
name: -32603 held at the unit layer
rule: No -32603 is driven in a scanned home and no failpoint or counted panic stands for one; the unit layer holds it.
guard: answer_a_panicking_dispatch_is_internal_error_and_closes
@row_add n=65 row=decision/diagnostics-files-0600 area=e2e-test-strategy rec=15
name: Diagnostics files are owner-only
rule: diagnostics/ files are mode 0600.
@row_add n=66 row=decision/synthetic-inputs-with-canary area=e2e-test-strategy rec=15
name: Synthetic test inputs carry a canary
rule: Every integration and E2E input is synthetic and embeds one fixed canary, which never appears in a home-level diagnostics line, only in events.ndjson and instance detail files.
@row_add n=67 row=decision/scan-hits-never-print-bytes area=e2e-test-strategy rec=15
name: Scan hits never print matched bytes
rule: A scan hit is reported as file, line, byte offset and pattern class, never the matched bytes.
guard: error_body_check_reports_no_byte_of_what_it_read
@row_add n=68 row=decision/ci-scan-before-upload area=e2e-test-strategy rec=15
name: CI secret scan before any upload
rule: The canary scan runs in CI after each job's end-to-end run over the kept test homes, the harness capture and the junit report, before any upload.
except: The harness chunk.diff and the viola-chaos-* home are out of the scan.
@row_add n=69 row=decision/each-exit-cause-own-hint area=e2e-test-strategy rec=16
name: Each exit cause has its own hint
rule: Every exit cause ends human output with a hint line naming that cause, no two causes share a hint, and a fault carries none.
guard: no_two_codes_share_a_hint, shared_hints_reads_a_planted_duplicate
@row_add n=70 row=contract/exit-21-detail/strict-modes-failed area=e2e-test-strategy rec=16
boundary: exit 21 instance-unreachable detail
item: strict-modes-failed
promise: The home or instance fails strict-modes (a widened mode); refused before any log, frame or line.
@row_add n=71 row=contract/exit-21-detail/instance-dead area=e2e-test-strategy rec=16
boundary: exit 21 instance-unreachable detail
item: instance-dead
promise: No instance is running for the name with its directory standing, a stale wrapper included; during connect.
@row_add n=72 row=contract/exit-21-detail/unwrapped-name area=e2e-test-strategy rec=16
boundary: exit 21 instance-unreachable detail
item: unwrapped-name
promise: The name holds no instance directory (an unwrapped session or a name never started); its own hint; during connect.
@row_add n=73 row=contract/exit-21-detail/endpoint-lost area=e2e-test-strategy rec=16
boundary: exit 21 instance-unreachable detail
item: endpoint-lost
promise: The wrapper was lost with the call open; its own hint; during call.
@row_add n=74 row=contract/exit-21-detail/server-verify-failed area=e2e-test-strategy rec=16
boundary: exit 21 instance-unreachable detail
item: server-verify-failed
promise: The listening server is not the recorded wrapper (pid or start time mismatch, or another euid); its own hint; logged as process-exit at WARN; during connect.
@row_add n=75 row=contract/exit-21-detail/socket-dir-refused area=e2e-test-strategy rec=16
boundary: exit 21 instance-unreachable detail
item: socket-dir-refused
promise: The socket directory is refused, on the eight frame verbs alone; its own hint; process-exit at WARN with no channel-request; during connect.
@row_add n=76 row=contract/cli-json/instance-unreachable area=e2e-test-strategy rec=16
boundary: CLI --json document
item: instance-unreachable
promise: Exit 21 prints {"v":1,"error":"instance-unreachable","detail":"<cause>"}; MCP's text is "refused: instance-unreachable".
@row_add n=77 row=contract/start-exit-1/already-live area=e2e-test-strategy rec=16
boundary: run and revive exit 1 causes
item: already-live
promise: The name is held by a live wrapper or a fresh replayed beat; the hint is viola list.
@row_add n=78 row=contract/start-exit-1/stale-heartbeat area=e2e-test-strategy rec=16
boundary: run and revive exit 1 causes
item: stale-heartbeat
promise: The holder's pid is live but its heartbeat is older than 5 s; stale heartbeat is exit 1's cause alone.
@row_add n=79 row=contract/start-exit-1/squatted-name area=e2e-test-strategy rec=16
boundary: run and revive exit 1 causes
item: squatted-name
promise: The endpoint name is squatted.
@row_add n=80 row=contract/start-exit-1/pinned-hash-mismatch area=e2e-test-strategy rec=16
boundary: run and revive exit 1 causes
item: pinned-hash-mismatch
promise: The pinned executable fails its SHA-256 re-hash.
@row_add n=81 row=contract/start-exit-1/strict-modes-failed area=e2e-test-strategy rec=16
boundary: run and revive exit 1 causes
item: strict-modes-failed
promise: The home or instance state could be written, or read, by another user; no log opened, no line, no child.
@row_add n=82 row=contract/start-exit-1/socket-dir-refused area=e2e-test-strategy rec=16
boundary: run and revive exit 1 causes
item: socket-dir-refused
promise: The socket directory is refused before any snapshot, child or bind; one process-exit line; a failed create reads the same code.
@row_add n=83 row=contract/revive-exit-1/no-session area=e2e-test-strategy rec=16
boundary: revive exit 1 causes
item: no-session
promise: No logged session for the name, or an --id the log does not hold.
@row_add n=84 row=contract/revive-exit-1/session-live area=e2e-test-strategy rec=16
boundary: revive exit 1 causes
item: session-live
promise: One claude agents --json read lists the logged session live in another claude; no child starts.
@row_add n=85 row=contract/revive-exit-1/cwd-missing area=e2e-test-strategy rec=16
boundary: revive exit 1 causes
item: cwd-missing
promise: The recorded directory is gone, or the snapshot holds no cwd.
@row_add n=86 row=contract/revive-json/refusal-document area=e2e-test-strategy rec=16
boundary: revive --json document
item: exit-1 refusal
promise: Each exit-1 refusal prints {"v":1,"error":"<code>","detail":null} with empty stderr; the human unable:/hint: pair is unchanged without --json.
@row_add n=87 row=decision/revive-preflight-order area=e2e-test-strategy rec=16
name: revive preflight order
rule: revive takes the strict-modes check first, then holder, session, session-live and recorded directory in that order, a later reading not taken once an earlier one refuses.
@row_add n=88 row=decision/session-live-read-fails-open area=e2e-test-strategy rec=16
name: session-live reading fails open
rule: revive --fork skips the session-live reading, and a failed claude agents read passes it.
guard: revive_passes_the_session_live_reading, revive_whose_agents_read_fails_starts
@row_add n=89 row=decision/revive-json-no-document-after-terminal area=e2e-test-strategy rec=16
name: No revive document once the child holds the terminal
rule: A pump_child error under revive --json prints no document.
why: The child has held the terminal.
@row_add n=90 row=decision/strict-modes-before-any-log area=e2e-test-strategy rec=16,17
name: Strict-modes check before any read or open
rule: Frame verbs, link verbs and starts take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found.
why: A check placed after the role file opens would pass, because opening a role file narrows an existing home to 0700.
guard: link_verbs_refuse_a_widened_instance_before_any_log, frame_verb_refuses_a_widened_instance_before_any_log
@row_add n=91 row=decision/one-negative-per-security-control area=e2e-test-strategy rec=17
name: One negative test per security control
rule: Every security-plan control has a negative test of its own.
@row_add n=92 row=decision/socket-dir-check-never-chmods area=e2e-test-strategy rec=17
name: Socket directory refused, never repaired
rule: The per-user socket directory is refused when it is a symlink, wider than 0700, not a directory or of another owner, and the check never chmods it.
guard: security_negative_a_start_refuses_the_socket_directory_before_any_bind
@row_add n=93 row=contract/mcp-frame/socket-dir-refused area=e2e-test-strategy rec=17
boundary: MCP frame
item: socket-dir-refused
promise: A refused socket directory gives instance-unreachable with no line at MCP's frame.
@row_add n=94 row=contract/channel-link-names/invalid-name area=e2e-test-strategy rec=17
boundary: channel link and unlink methods
item: invalid name
promise: A driver or driven that is not a valid name, a driven other than the serving wrapper's own name, or a driver equal to driven gets -32602 with data null and appends no record.
@row_add n=95 row=decision/home-readable-by-other-refused area=e2e-test-strategy rec=17,16
name: Home another user may read is refused
rule: A pre-existing home another user may read is refused at run with exit 1.
guard: a_home_another_user_may_read_refuses_run
@row_add n=96 row=decision/umask-independent-modes area=e2e-test-strategy rec=17
name: Umask-independent file modes
rule: Under any umask, home files are 0600, directories 0700, and the pinned viola binary 0700, never group- or world-writable.
guard: security_negative_a_clean_start_under_an_open_umask_keeps_the_socket_private
@row_add n=97 row=decision/single-writers area=e2e-test-strategy rec=17
name: Single writers of snapshot and stamps
rule: Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json.
guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
@row_add n=98 row=decision/client-verifies-server-identity area=e2e-test-strategy rec=17
name: Client verifies the server before any frame
rule: A CLI frame verb writes no frame to a server that is not the recorded wrapper and names no pid, start time, path or endpoint in its output.
guard: security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper
@row_add n=99 row=decision/list-rows-escape-control-chars area=e2e-test-strategy rec=18
name: list rows escape control characters
rule: viola list never prints raw newline, tab or C1 bytes in a NAME cell, so each row stays one line and every column starts at its caption offset in every mode.
guard: cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode
@row_add n=100 row=contract/list-human/columns area=e2e-test-strategy rec=18
boundary: viola list human board
item: columns
promise: The caption row is NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, and the table is fixed-width.
@row_add n=101 row=contract/list-human/unwrapped-separator area=e2e-test-strategy rec=18
boundary: viola list human board
item: unwrapped separator
promise: Unwrapped sessions follow the separator line "-- UNWRAPPED - READ-ONLY --".
@row_add n=102 row=contract/list-json/name area=e2e-test-strategy rec=18
boundary: list --json
item: name
promise: list --json carries the original name string, escaped as JSON.
@row_add n=103 row=decision/stale-row-dim area=e2e-test-strategy rec=19
name: Stale row shown dim
rule: On a terminal a stale row is dim from first byte to last with its NAME dim and bold, shows stale, and no SGR touches the caption or separator.
guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
@row_add n=104 row=decision/torn-append-heals area=e2e-test-strategy rec=20
name: Torn append heals on the next start
rule: A torn last log line is counted in skipped.torn_lines, the next append starts on a fresh line with every earlier byte unchanged, and one state-recovered line is logged.
guard: chaos_torn_append
@row_add n=105 row=decision/torn-append-by-truncation area=e2e-test-strategy rec=20
name: Torn append simulated by truncation
rule: The torn-append chaos case stops the wrapper and shortens its log in place rather than killing it mid-append.
why: The file on disk is the same, and a killed instrumented process can leave a short coverage profile.
@row_add n=106 row=decision/exit-detected-on-handle area=e2e-test-strategy rec=20
name: Child exit detected on the handle
rule: The wrapper detects the child's exit on the process handle (exit_source "handle-wait"), not on output EOF.
@row_add n=107 row=contract/list-json/liveness-stale area=e2e-test-strategy rec=20
boundary: list --json
item: liveness stale
promise: A wrapper whose heartbeat is older than 5 s reads liveness "stale"; a killed one is omitted as gone.
@row_add n=108 row=decision/vt100-panic-keeps-passthrough area=e2e-test-strategy rec=20,22
name: vt100 panic keeps human passthrough
rule: After a caught vt100 feed panic the screen model reads input-not-ready and drops bytes, send is refused, human passthrough still reaches the child, and the outer terminal shows no panic text.
guard: chaos_feed_panic_refuses_send_and_keeps_passthrough
@row_add n=109 row=contract/cli-send/input-not-ready area=e2e-test-strategy rec=20
boundary: viola send
item: input-not-ready
promise: send exits 13 with input-not-ready when the screen model is not ready.
@row_add n=110 row=decision/forward-compatible-log-reading area=e2e-test-strategy rec=20
name: Forward-compatible event log reading
rule: Lines of an unknown kind and unknown fields on a known kind are counted and skipped; list --json still exits 0 and the known-kind line still counts toward item state.
guard: chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads
@row_add n=111 row=contract/list-json/skipped-counts area=e2e-test-strategy rec=20
boundary: list --json
item: skipped
promise: skipped.torn_lines, skipped.unknown_kinds and skipped.unknown_fields each count the log lines skipped for that reason.
@row_none n=6 rec=21 type=5
@row_add n=112 row=decision/property-tests-with-proptest area=e2e-test-strategy rec=22
name: proptest for property tests
rule: Input parsers and validators are property-tested with proptest at 512 cases, each with its regression file committed.
@row_add n=113 row=decision/fuzzing-with-cargo-fuzz area=e2e-test-strategy rec=22
name: cargo-fuzz in a separate workspace
rule: Fuzz targets live in a separate fuzz/ cargo workspace under cargo-fuzz with shared arbitrary inputs and synthetic seeds; each parser joins with its own target.
@row_add n=114 row=decision/fuzz-corpora-byte-exact area=e2e-test-strategy rec=22
name: Fuzz corpora kept byte-exact
rule: Byte-sensitive fuzz corpora are marked binary in .gitattributes so their seeds stay byte-exact.
@row_add n=115 row=decision/vt100-fuzz-silent-panic-hook area=e2e-test-strategy rec=22
name: vt100 fuzz target installs a silent panic hook
rule: The vt100_feed fuzz target installs a silent panic hook once before its first feed.
why: libfuzzer-sys's own hook aborts on every panic, a caught one included; an escaping panic still aborts through its catch.
@row_add n=116 row=decision/statusline-proptest-whole-seconds area=e2e-test-strategy rec=22
name: Statusline property draws whole seconds
rule: The statusline reader's property strategy draws whole seconds across the date range.
why: As first written the property stayed green on a stub.
@row_add n=117 row=decision/agents-proptest-reparse-oracle area=e2e-test-strategy rec=22
name: agents parser oracle re-parses its input
rule: The claude agents --json property oracle compares a row against the input re-parsed from its bytes.
why: A float does not round-trip through serde_json text.
@row_add n=118 row=decision/fixture-drift-byte-compare area=e2e-test-strategy rec=23
name: Fixture drift compared byte for byte
rule: The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails.
guard: contract_fake_agent_drift
@row_add n=119 row=decision/fixtures-match-tolerant-schema area=e2e-test-strategy rec=23
name: Fixtures checked against the tolerant schema
rule: jsonschema checks every fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
@row_add n=120 row=decision/pinned-exe-hash-sha2 area=e2e-test-strategy rec=23
name: sha2 for the pinned-exe content hash
rule: The pinned-exe content hash is SHA-256 from sha2, truncated to 8 bytes as 16 hex.
guard: sha256_published_vector_matches_digest, sha256_truncated_to_eight_bytes_yields_sixteen_hex
@row_add n=121 row=decision/contract-tables-are-literal area=e2e-test-strategy rec=23
name: Contract tables are literal
rule: Contract tests hold literal tables, never the product's own list.
@row_add n=122 row=decision/bounded-readers area=e2e-test-strategy rec=23
name: Every external reader is bounded
rule: Every product read of external input is bounded or named as an exception.
except: Reads of viola's own executable take no external input.
guard: product_readers_are_each_bounded_or_named
@row_add n=123 row=decision/no-serde-json-unbounded-depth area=e2e-test-strategy rec=23
name: serde_json unbounded depth banned
rule: The serde_json unbounded-depth feature is banned in deny.toml.
guard: serde_json_unbounded_depth_is_banned_in_deny_toml
@row_add n=124 row=decision/error-writers-each-named area=e2e-test-strategy rec=23
name: Every product error writer is named
rule: Every product error writer is named in the error-body enumeration with a writer and a witness that stand.
guard: product_error_writers_are_each_named
@row_add n=125 row=decision/thiserror-messages-interpolate-nothing area=e2e-test-strategy rec=23
name: thiserror messages interpolate nothing
rule: thiserror error messages interpolate no value.
guard: thiserror_messages_interpolate_nothing
@row_add n=126 row=contract/channel-error/fault-bodies area=e2e-test-strategy rec=23
boundary: channel error
item: fault bodies
promise: The channel's faults are seven literal bodies.
@row_none n=7 rec=24 type=5
@row_add n=127 row=decision/os-gated-tests-each-named area=e2e-test-strategy rec=25
name: OS-gated tests each named with a reason
rule: Every OS-gated test or file is a row of the GATES table with its reason, and a gate naming Windows or macOS is a fault even when named.
guard: os_gated_tests_are_each_named_with_their_reason
@row_none n=8 rec=26 type=5
@row_add n=128 row=decision/module-boundary-integration-tests area=integration-test-strategy rec=27
name: Module boundaries tested by direct calls
rule: The module-to-module boundary is tested by direct calls across crate APIs with shared rstest fixtures, run by cargo-nextest.
