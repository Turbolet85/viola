@row_add n=1 row=contract/cli-exit-codes/14 area=unit-test-strategy rec=1
boundary: CLI exit codes
item: 14
promise: A refusal whose code word is outside the known list deserialises to the unknown refusal and exits 14 with a --json refusal document; it never panics and never maps to 20.
@row_add n=2 row=decision/unknown-refusal-word-stays-visible area=unit-test-strategy rec=1
name: Unknown refusal word stays visible
rule: A refusal code word outside the closed refusal list stays visible in the JSON document and in the human refused line, with the unknown refusal's exit.
guard: reply_of_reads_the_three_shapes
@row_add n=3 row=contract/cli-arguments/json-flag area=unit-test-strategy rec=1
boundary: CLI arguments
item: --json
promise: The --json flag is read from the arguments only before a `--` separator.
@row_add n=4 row=decision/budget-governor-thresholds area=unit-test-strategy rec=1
name: Budget governor thresholds
rule: The budget gate is open at five_hour 89 and paused at 90, open at seven_day 84 and paused at 85, and a budget override is dropped once its instant has passed.
guard: gate_decide_an_expired_override_is_dropped
@row_add n=5 row=decision/budget-decision-takes-the-instant area=unit-test-strategy rec=1
name: Budget decision takes the instant as an argument
rule: The budget decision is a pure function that takes the wall-clock instant as an argument, so its thresholds and override expiry are unit-tested with no clock seam and no mocked instant.
guard: gate_decide_*
@row_add n=6 row=decision/send-confirmation-exact-match area=unit-test-strategy rec=1
name: Send confirmation by exact normalised match
rule: A send is confirmed only when the submitted prompt text equals the typed text, every CR LF and lone CR read as one LF and the trailing CR and LF removed; a submitted prompt that still holds a CR or the trailing newline claims nothing.
guard: send_a_prompt_that_keeps_the_cr_while_its_inner_cr_is_one_lf_claims_nothing
@row_add n=7 row=decision/send-confirmation-window-expiry area=unit-test-strategy rec=1
name: Send confirmation window expiry
rule: A confirmed send whose window expires without a matching submitted prompt is refused no-prompt-submitted.
@row_add n=8 row=decision/local-command-exact-match area=unit-test-strategy rec=1
name: Local commands matched exactly
rule: A typed text is a local command only by an exact match, without its trailing newlines, against the compiled list (/clear, /remote-control, /compact), never by a leading-slash test.
guard: send_local_command_decision_is_exact_text_never_a_leading_slash
@row_add n=9 row=decision/local-command-unconfirmable area=unit-test-strategy rec=1
name: Local command without a post-condition is unconfirmable
rule: A local command with no measured post-condition, or sent to an unverified CLI, is unconfirmable with no confirmation window.
guard: send_local_command_without_a_measured_post_condition_is_unconfirmable
@row_add n=10 row=decision/clear-confirmed-by-new-session area=unit-test-strategy rec=1
name: /clear confirmed by a new session
rule: /clear on a verified CLI is confirmed only by a session-start with cause clear and a new session id, never by a submitted prompt or another session-start; its window expiry gives no-prompt-submitted with the cursor.
guard: send_local_command_clear_is_confirmed_by_a_new_session
@row_add n=11 row=decision/compact-confirmed-by-same-session area=unit-test-strategy rec=1
name: /compact confirmed by its same session
rule: /compact on a verified CLI is confirmed only by a session-start with cause compact and the remembered same session id; another cause, another id, no id, nothing remembered or a submitted prompt claims nothing.
guard: send_local_command_compact_is_confirmed_by_its_same_session
@row_add n=12 row=decision/compact-confirm-window area=unit-test-strategy rec=1
name: /compact confirmation window
rule: /compact has its own confirmation window of 40 s, clock-driven, whose expiry gives no-prompt-submitted with the cursor; every other confirmed send keeps the 10 s fallback window.
guard: send_compact_confirm_window_is_its_own_bound
state: provisional
@row_add n=13 row=contract/channel-wait/wake-kinds area=unit-test-strategy rec=1
boundary: the channel wait method
item: wake kinds
promise: wait wakes only on turn-ended, question, permission, plan, session-end and dialog-expired, and returns at once when a matching record is at or after `after`.
@row_add n=14 row=contract/channel-wait/timed-out area=unit-test-strategy rec=1
boundary: the channel wait method
item: timed_out
promise: wait returns {timed_out:true} only once the clock passes timeout_ms, never before; the timeout runs in the wrapper process.
@row_add n=15 row=decision/send-refusal-order area=unit-test-strategy rec=1
name: send refusal order
rule: send refuses in the order control-character, empty-text, human-typing, budget-paused, turn-running, input-not-ready, no-prompt-submitted, and after the gate is ready reads human-typing and turn-running again before anything is issued or typed.
@row_add n=16 row=decision/empty-text-refused-before-the-wheel area=unit-test-strategy rec=1
name: Empty text refused before the wheel is read
rule: An empty typed text is refused right after the typed text is taken, with nothing reserved, issued or typed and neither the wheel nor the running-turn state read.
guard: send_empty_text_is_refused_first_after_control_character
@row_add n=17 row=decision/answer-refusal-order area=unit-test-strategy rec=1
name: answer refusal order
rule: answer refuses in the order control-character, human-typing, unverified-cli, unknown-dialog.
@row_add n=18 row=decision/human-output-control-escaping area=unit-test-strategy rec=1
name: Human output escapes control characters
rule: Human-mode wait and last output escape C0 and C1 control characters, keeping newline and tab.
@row_add n=19 row=decision/list-row-control-escaping area=unit-test-strategy rec=1
name: list rows show every control character as hex
rule: In list rows every control character, newline, tab, DEL and C1 included, prints as \xHH text.
guard: escape_row_shows_every_control_character_as_hex_text
@row_add n=20 row=decision/dialog-hold-read-once area=unit-test-strategy rec=1
name: Dialog hold read once at start
rule: The dialog hold is read once at start from the config's dialog_hold_s through a bounded read, a whole number of seconds in range, falling back to sixty with one rejection.
guard: read_hold_falls_back_to_sixty_with_one_rejection
@row_add n=21 row=decision/dialog-hold-expiry area=unit-test-strategy rec=1
name: Dialog hold expiry
rule: An expired dialog hold appends exactly one dialog-expired line and logs one dialog-answered with deadline_hit true and no from; a late answer is unknown-dialog, and a hand-back or an answer writes no dialog-expired line.
guard: hook_dialog_expiry_writes_one_dialog_expired_line_and_one_answered_line
@row_add n=22 row=decision/pending-dialog-before-its-line area=unit-test-strategy rec=1
name: Pending dialog on the snapshot before its line
rule: A held dialog is on the snapshot before its line is logged, and is cleared when the append fails.
guard: hook_dialog_append_that_failed_clears_the_pending_dialog
@row_add n=23 row=decision/dialog-timeouts-outlast-the-hold area=unit-test-strategy rec=1
name: Dialog timeouts outlast the longest hold
rule: The hook's dialog bound is the longest hold plus the reply grace, and the embedded dialog hook timeouts outlast the longest hold.
guard: hooks_json_dialog_timeouts_outlast_the_longest_hold
@row_add n=24 row=decision/scoped-capture-pinned-dispatch area=unit-test-strategy rec=1
name: Scoped log captures through a pinned dispatch
rule: Every scoped log capture in the root binary's unit tests goes through one shared helper that first registers a pinned, always-interested dispatch, never the global default.
why: so no scoped capture is ever the only registered dispatcher and a line whose callsite another, subscriber-less thread registered first is not lost
guard: scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first
except: The three crate-level scoped captures run on a bare scoped default; never observed red, they have no owner.
@row_add n=25 row=decision/one-global-tracing-install area=unit-test-strategy rec=1
name: One global tracing install
rule: The binary installs a global tracing default in exactly one place, the gate.
@row_add n=26 row=decision/server-checked-before-any-frame area=unit-test-strategy rec=1
name: Server checked before any frame
rule: Every client verb and the hook refuse an unverified server inside their own span with no frame written.
guard: request_refuses_an_unverified_server_inside_its_dispatch_span
@row_add n=27 row=contract/unreachable-log-cause/instance-dead area=unit-test-strategy rec=1
boundary: the unreachable log cause
item: instance-dead
promise: Logged during connect.
@row_add n=28 row=contract/unreachable-log-cause/unwrapped-name area=unit-test-strategy rec=1
boundary: the unreachable log cause
item: unwrapped-name
promise: Logged during connect.
@row_add n=29 row=contract/unreachable-log-cause/endpoint-lost area=unit-test-strategy rec=1
boundary: the unreachable log cause
item: endpoint-lost
promise: Logged during call.
@row_add n=30 row=contract/unreachable-log-cause/server-verify-failed area=unit-test-strategy rec=1
boundary: the unreachable log cause
item: server-verify-failed
promise: Logged during connect.
@row_add n=31 row=contract/unreachable-log-cause/socket-dir-refused area=unit-test-strategy rec=1
boundary: the unreachable log cause
item: socket-dir-refused
promise: Logged during connect when the socket directory is refused.
@row_add n=32 row=decision/socket-dir-checked-before-any-frame area=unit-test-strategy rec=1
name: Socket directory checked before any frame
rule: A loosened socket directory is refused inside the verb's or the hook's span before any frame is written.
guard: connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span
@row_add n=33 row=contract/human-hint-cause/not-running area=unit-test-strategy rec=1
boundary: the human hint cause
item: not-running
promise: The hint cause of an instance-dead unreachable is not-running, a word apart from the log's.
@row_add n=34 row=contract/human-hint-cause/socket-dir area=unit-test-strategy rec=1
boundary: the human hint cause
item: socket-dir
promise: The hint cause of a refused socket directory is socket-dir, a word apart from the log's socket-dir-refused.
@row_add n=35 row=decision/hook-dialog-no-frame-to-impostor area=unit-test-strategy rec=1
name: Hook writes no frame to an impostor server
rule: The hook's dialog path writes no frame and emits no decision to a server that is not the recorded wrapper.
guard: ask_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper
@row_add n=36 row=decision/session-end-direct-append area=unit-test-strategy rec=1
name: SessionEnd falls back to a direct append
rule: A SessionEnd hook whose server check fails appends the session end directly.
@row_add n=37 row=contract/link-method/link-twice area=unit-test-strategy rec=1
boundary: the served link methods
item: link of a linked pair
promise: Linking a pair twice writes one line and answers the same since.
@row_add n=38 row=contract/link-method/unlink-then-link area=unit-test-strategy rec=1
boundary: the served link methods
item: unlink then link
promise: An unlink then a link takes a later since.
@row_add n=39 row=contract/link-method/unlink-absent-pair area=unit-test-strategy rec=1
boundary: the served link methods
item: unlink of an absent pair
promise: Unlinking an absent pair writes nothing.
@row_add n=40 row=contract/link-method/bad-pair area=unit-test-strategy rec=1
boundary: the served link methods
item: bad pair
promise: A bad pair is refused with invalid_params.
@row_add n=41 row=contract/link-method/from area=unit-test-strategy rec=1
boundary: the served link methods
item: from
promise: A from on a link or unlink changes nothing.
@row_add n=42 row=contract/link-method/append-failure area=unit-test-strategy rec=1
boundary: the served link methods
item: failed append
promise: An append that fails answers internal with the link set unchanged.
@row_add n=43 row=contract/link-verb-line/since area=unit-test-strategy rec=1
boundary: the link verb line
item: since
promise: The line's time is the reply's since as UTC clock time; a since that is no RFC 3339 instant prints escaped.
@row_add n=44 row=decision/diagnostics-checked-before-narrowing area=unit-test-strategy rec=1
name: Diagnostics path checked before narrowing
rule: A diagnostics folder, role file or detail file that fails the strict check is a failed open, checked before anything is narrowed and left as found; the writer goes to a sink and a detail line is dropped.
guard: write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it
@row_add n=45 row=decision/bounded-readers-keep-the-cap area=unit-test-strategy rec=1
name: Bounded readers keep the frame cap
rule: The status line shell-out, the version gate drain and the thresholds, dialog hold and env-keep readers keep no more than the frame cap of what they read; a command the cut ends is a failed command.
guard: the BOUNDARIES rows of contract_bounded_readers
@row_add n=46 row=decision/e2e-home-headroom-floor area=unit-test-strategy rec=2,4
name: E2E home headroom floor
rule: The end-to-end home base is refused when its filesystem has less free space than the headroom floor, 9 GiB; the floor has no env, flag, feature or config override.
guard: harness::tests::e2e_home_*
@row_add n=47 row=contract/e2e-home-refusal/message area=unit-test-strategy rec=2
boundary: the e2e-home headroom refusal
item: message
promise: e2e-home backing: {free} MiB free, below the {floor} MiB floor
@row_none n=1 rec=3 type=S
@row_add n=48 row=contract/harness-boot/default-instances area=5-command-implementation rec=4,3
boundary: harness boot
item: --instance
promise: With no --instance flag the instances are overseer and builder whatever other flags are given; one or more --instance flags replace that list.
@row_add n=49 row=contract/harness-boot/agents-mode area=5-command-implementation rec=4,3
boundary: harness boot
item: --agents-mode
promise: --agents-mode takes recorded, oversize or malformed, default recorded, an unknown value a usage error; it sets no env var and is recorded as agents_mode in the session records.
@row_add n=50 row=contract/harness-boot/statusline-echo area=5-command-implementation rec=4,3
boundary: harness boot
item: --statusline-echo
promise: --statusline-echo conflicts with --unstamped and plants an owner-only statusline settings source in the session home that viola only reads; it sets no env var and writes no budget file.
@row_add n=51 row=decision/harness-builds-in-its-own-target area=5-command-implementation rec=4
name: Harness builds into its own target dir
rule: The harness build always uses its own target dir, never the target dir the shim's cargo run uses.
why: a running harness binary could not be relinked, and a workspace-wide build re-fingerprints it
@row_add n=52 row=decision/e2e-homes-under-target-e2e-home area=5-command-implementation rec=4
name: E2E homes under target/e2e-home
rule: Every booted session home is a fresh unique directory under the workspace's target/e2e-home, which viola itself creates owner-only.
why: CI's zero-panic gate, schema gate, secret scan and diagnostics upload all read that tree
@row_add n=53 row=decision/e2e-home-link-base area=5-command-implementation rec=4
name: E2E home base may be a link
rule: A link as the e2e home base must have an absolute target, created 0700 when gone with one non-recursive create, and is accepted only when it reads a real directory with no group or other bit; a refused base reads build-failed.
@row_add n=54 row=decision/harness-scoped-path area=5-command-implementation rec=4
name: Harness-scoped PATH
rule: The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process.
@row_add n=55 row=decision/verify-sole-stamp-writer area=5-command-implementation rec=4
name: verify is the only stamp writer
rule: viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
@row_add n=56 row=decision/supervisor-ordinary-child area=5-command-implementation rec=4
name: Supervisor is an ordinary child
rule: The supervisor is spawned as an ordinary child with no detach flags, outlives boot, and stops only through its stop request.
@row_add n=57 row=decision/harness-never-reads-cargo-bin-exe area=5-command-implementation rec=4
name: Harness never reads CARGO_BIN_EXE
rule: The harness takes its binaries from a passed binary dir and never reads CARGO_BIN_EXE_*.
why: Cargo sets those only for the root package's own integration tests
@row_add n=58 row=decision/workspace-root-absolute area=5-command-implementation rec=4
name: Workspace root passed absolute
rule: The harness resolves the workspace root and passes it as an absolute path.
why: nextest runs the e2e crate's tests with that crate as the cwd
@row_add n=59 row=decision/boot-readiness-file-probe area=5-command-implementation rec=5
name: Boot readiness is a file-state probe
rule: boot polls file state at a bounded 100 ms interval against a deadline, in stages, never by sleep-based synchronisation.
@row_add n=60 row=contract/boot-readiness/process-start area=5-command-implementation rec=5
boundary: boot readiness
item: process-start
promise: The instance's run log holds a process-start for self and for claude-child, both pids alive by pid and start time; missing reads run-process-start or child-process-start, and a self process-exit first is run-exited.
@row_add n=61 row=contract/boot-readiness/snapshot area=5-command-implementation rec=5
boundary: boot readiness
item: snapshot
promise: The instance snapshot parses with pid, started_at, child_pid and endpoint in its data; missing reads snapshot, or endpoint when no endpoint.
@row_add n=62 row=contract/boot-readiness/heartbeat area=5-command-implementation rec=5
boundary: boot readiness
item: heartbeat
promise: The heartbeat mtime is less than 5 s old; missing reads heartbeat.
@row_add n=63 row=contract/boot-readiness/events area=5-command-implementation rec=5
boundary: boot readiness
item: events
promise: Event lines 1 to 3 are a wheel line with cause start, then budget-gate, then session-start with source hook; missing reads events.
@row_add n=64 row=contract/harness-boot-exit/0 area=5-command-implementation rec=6
boundary: harness boot exit codes
item: 0
promise: Every instance is ready within the timeout.
@row_add n=65 row=contract/harness-boot-exit/1 area=5-command-implementation rec=6
boundary: harness boot exit codes
item: 1
promise: Not ready, with a document of v 1, cmd boot, ok false, a reason, the instance or null, exit_code or null, and the unmet readiness checks in missing.
@row_add n=66 row=contract/harness-boot-reason/build-failed area=5-command-implementation rec=6
boundary: harness boot refusal reason
item: build-failed
promise: One of the four boot refusal reasons.
@row_add n=67 row=contract/harness-boot-reason/verify-failed area=5-command-implementation rec=6
boundary: harness boot refusal reason
item: verify-failed
promise: One of the four boot refusal reasons.
@row_add n=68 row=contract/harness-boot-reason/run-exited area=5-command-implementation rec=6
boundary: harness boot refusal reason
item: run-exited
promise: The wrapper exited; exit_code is the wrapper's exit code verbatim.
@row_add n=69 row=contract/harness-boot-reason/readiness-timeout area=5-command-implementation rec=6
boundary: harness boot refusal reason
item: readiness-timeout
promise: One of the four boot refusal reasons.
@row_add n=70 row=contract/harness-boot/timeout area=5-command-implementation rec=7
boundary: harness boot
item: timeout
promise: Readiness is bounded at 20 s per instance after the build.
@row_none n=2 rec=8 type=S
@row_add n=71 row=contract/harness-run/default-selection area=5-command-implementation rec=9
boundary: harness run
item: default selection
promise: With no selector and no --local-live, run means --all: unit and integration, then fake-agent E2E; coverage, fuzz replay and mutants never join the default.
@row_add n=72 row=contract/harness-run/mutants-only-flags area=5-command-implementation rec=9
boundary: harness run
item: --file and --package
promise: --file and --package each require --mutants.
@row_add n=73 row=decision/test-layer-filtersets area=5-command-implementation rec=9
name: Test layers by nextest filterset
rule: The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer.
@row_add n=74 row=decision/no-nested-harness-in-nextest area=5-command-implementation rec=9
name: No test shells out to the harness
rule: No nextest test shells out to agent-run or the harness binary, and in-test sessions skip the build, so there is never a nested cargo run inside nextest.
@row_add n=75 row=decision/e2e-boot-model-by-cleanup area=5-command-implementation rec=9
name: E2E boot model chosen by cleanup
rule: A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
@row_add n=76 row=decision/fixture-chain-exists-twice area=5-command-implementation rec=9
name: Fixture chain exists twice
rule: The home, fake agent, stamped home and booted wrapper chain exists as a sync root copy and an e2e-crate copy, against one contract.
why: root test support cannot be imported by another package, and the root package must not dev-depend on the tokio-based e2e crate
@row_add n=77 row=decision/root-wait-bound area=5-command-implementation rec=9
name: Root waits bounded below the kills
rule: Root waits on a child are bounded at 7 s, below the mutants profile's 10 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first.
why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
@row_add n=78 row=decision/killed-wrapper-endpoint-wait area=5-command-implementation rec=9
name: A killed wrapper's endpoint is gone when unconnectable
rule: After a stop or a kill the endpoint is gone when a connect is refused or the path is absent, never by waiting for the file to vanish; an exit code is not the endpoint gone.
why: a killed wrapper removes nothing, so its socket file stays
@row_add n=79 row=contract/harness-session/session-id area=5-command-implementation rec=9
boundary: harness session
item: session id
promise: The session id is the test name, the label (or main) and the pid; each session has its own home and session dir.
@row_add n=80 row=contract/harness-session/cleanup-report area=5-command-implementation rec=9
boundary: harness session
item: cleanup report
promise: A harness cleanup report has no field false, an interim null allowed, the same shape as the cleanup command.
@row_add n=81 row=decision/mutation-base-whole-chunk area=5-command-implementation rec=9
name: Mutation base is the whole chunk
rule: The mutation base is derived from history as the last master flip before the oldest pre-CI commit of a pending record, so every run of a pass mutates the whole chunk; an unresolved base exits 1 base-missing and no empty diff is written.
why: an unbounded search moves mid-pass, and an empty diff reads as a false zero-mutant pass
@row_add n=82 row=decision/chunk-diff-pins-prefixes area=5-command-implementation rec=9
name: Chunk diff pins its prefixes
rule: The chunk diff covers the working tree and untracked files and always pins the a/ and b/ prefixes.
why: a host's diff prefix config would write headers the classification cannot read
guard: chunk_diff_pins_its_prefixes_under_a_hostile_config
@row_add n=83 row=decision/mutation-own-target-dir area=5-command-implementation rec=9
name: Mutation run builds in its own target dir
rule: The mutation run pre-builds the root package and builds every mutant in its own target dir, copied into each scratch tree.
why: a copied default target keeps the original tree's binary paths, so tests drove the unmutated binary
@row_add n=84 row=decision/mutation-keeps-no-test-home area=5-command-implementation rec=9
name: Mutation run keeps no test home
rule: A mutation run keeps no test home, overriding the workflow's keep-homes setting.
why: every home carries a pinned copy of the viola binary
@row_add n=85 row=decision/mutants-output-live area=5-command-implementation rec=9
name: cargo-mutants output streams live
rule: cargo-mutants' output goes live to the harness's stderr with every outcome line printed, never captured.
why: a stalled or cancelled run names the mutant it stopped at
@row_add n=86 row=decision/mutant-build-timeout area=5-command-implementation rec=9
name: Fixed mutant build timeout
rule: Each mutant's build is bounded by a fixed 400 s, five times the largest measured baseline, not a multiplier.
why: a multiplier derives a sub-second bound from a sub-second baseline
@row_add n=87 row=decision/mutation-diff-classification area=5-command-implementation rec=9
name: Diffs with no mutable Rust skip cargo-mutants
rule: A diff naming no Rust path passes as no-rust-delta and one whose Rust paths are all test targets passes as test-only-rust-delta, both without reaching cargo-mutants; any other deletes the old outcomes first and is red with none.
why: cargo-mutants exits 0 and writes no outcomes on such diffs
@row_add n=88 row=decision/mutation-verdict-by-counts area=5-command-implementation rec=9
name: Mutation verdict by counts
rule: A mutation run passes only when missed minus host-excluded is zero, no mutant timed out and unviable mutants do not outnumber caught; only exits 0, 2 or 3 can pass, then the counts decide.
why: exit 3 masks exit 2, and more unviable than caught means the run tested almost nothing
@row_add n=89 row=decision/host-excluded-mutants area=5-command-implementation rec=9
name: Host-excluded missed mutants
rule: A missed mutant whose whole span sits under a cfg proved false for the build host, by compile-time facts, leaves the missed count; anything the reader cannot read stays counted.
@row_add n=90 row=contract/harness-run/mutants-exit-reason area=5-command-implementation rec=9
boundary: harness run
item: mutants-exit-<code>
promise: cargo-mutants exit 1, 4, 5, 6 or 70 fails run --mutants with reason mutants-exit-<code>.
@row_add n=91 row=decision/mutants-tested-by-owning-package area=5-command-implementation rec=9
name: Mutants tested by their own package
rule: Each mutant is tested only by the tests of the package that owns the mutated file, so a crate's mutants must be killed by its own tests.
@row_add n=92 row=decision/members-declare-fake-agent-feature area=5-command-implementation rec=9
name: Mutated members declare fake-agent
rule: A mutable workspace member declares a fake-agent feature, a no-op where it has none.
why: cargo-mutants passes the feature to package-scoped runs, and a missing feature fails the run
@row_add n=93 row=contract/harness-run/package-refused area=5-command-implementation rec=9
boundary: harness run
item: package-refused
promise: A --package that is not viola or a member declaring fake-agent, named in lowercase ASCII, digits, - and _, exits 2 package-refused before any cargo runs.
@row_add n=94 row=decision/coverage-floors area=5-command-implementation rec=10
name: Coverage floors
rule: Coverage fails under 85% lines, 95% functions or 80% regions, measured with the fake agent, the e2e crate, test support and fuzz trees ignored.
@row_add n=95 row=contract/coverage-failure/llvm-cov-exit area=5-command-implementation rec=10
boundary: coverage suite failure codes
item: llvm-cov-exit-N
promise: A red llvm-cov exit over green tests, such as a floor breach.
@row_add n=96 row=contract/coverage-failure/llvm-cov-summary-missing area=5-command-implementation rec=10
boundary: coverage suite failure codes
item: llvm-cov-summary-missing
promise: The coverage summary was not written.
@row_add n=97 row=decision/perf-verdict-belongs-to-gate area=5-command-implementation rec=11
name: Perf verdict belongs to the gate
rule: The perf arm runs only when named and reports its suite; the deadline verdict belongs to the gate's perf requirement, never to run.
@row_add n=98 row=decision/fuzz-replay-linux-only area=5-command-implementation rec=12
name: Fuzz replay on Linux only
rule: Fuzz replay runs on Linux only; elsewhere it exits 2 fuzz-linux-only.
@row_add n=99 row=decision/fuzz-toolchain-single-source area=5-command-implementation rec=12
name: Fuzz toolchain from one source
rule: The fuzz toolchain channel is read from the fuzz workspace's own toolchain file, the one source, and the fuzz dir is a separate cargo workspace excluded from the root.
@row_add n=100 row=decision/fuzz-corpus-never-vacuous area=5-command-implementation rec=12
name: Fuzz replay never passes vacuously
rule: A missing or empty corpus, or no target at all, fails corpus-empty, never a vacuous pass.
@row_add n=101 row=decision/fuzz-reproducers-committed area=5-command-implementation rec=12
name: Fuzz corpus committed with reproducers
rule: The replay corpus is committed, seeded from recorded payloads or synthetic seeds, and every reproducer is committed with its fix; nothing is cached between CI runs.
@row_add n=102 row=decision/nightly-fuzz-fails-on-crash area=5-command-implementation rec=12
name: Nightly fuzz fails on any crash
rule: The nightly time-boxed fuzz run uploads its artifacts and fails the job on any crash.
@row_add n=103 row=contract/harness-run/tool-missing area=5-command-implementation rec=12
boundary: harness run
item: tool-missing
promise: Exits 1 with a detail naming the missing tool or file, such as cargo-fuzz.
@row_add n=104 row=decision/live-verify-never-in-ci area=5-command-implementation rec=13
name: Live verify never in CI
rule: verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
@row_add n=105 row=decision/local-live-runner-seam area=5-command-implementation rec=13
name: Local-live through a runner seam
rule: The local-live build and verify run through a runner seam, so the harness's own tests use a stand-in and never run the real CLI.
@row_add n=106 row=contract/local-live-suite/pass area=5-command-implementation rec=13
boundary: the local-live suite
item: pass
promise: Passes when verify exits 0, each relied ledger row id names exactly one pass line, and the last line reads stamped with zero fail; it claims no live measurement beyond that.
@row_add n=107 row=contract/local-live-suite/failure-codes area=5-command-implementation rec=13
boundary: the local-live suite
item: failure codes
promise: A failure carries one code: build, verify-exit-<n>, verify-exit-none or row-missing.
@row_add n=108 row=contract/harness-run-exit/0 area=5-command-implementation rec=14
boundary: harness run exit codes
item: 0
promise: Every selected suite passes, with no timed-out mutants, no missed mutants beyond the host-excluded, and unviable not outnumbering caught.
@row_add n=109 row=contract/harness-run-exit/non-zero area=5-command-implementation rec=14
boundary: harness run exit codes
item: non-zero
promise: Anything else; survivors, outcomes-missing and unviable-exceeds-caught each make a --mutants run red.
@row_add n=110 row=contract/run-document/suites area=5-command-implementation rec=15
boundary: the harness run document
item: suites
promise: Each entry names its suite and carries passed, failed, skipped, survived, artifact, and failures listing failing test names.
@row_add n=111 row=contract/run-document/mutants area=5-command-implementation rec=15
boundary: the harness run document
item: mutants
promise: A --mutants run adds a mutants object with tested and a verdict of counted, no-rust-delta, test-only-rust-delta, scoped or package, with the full base sha except on package.
@row_add n=112 row=contract/run-document/host-excluded area=5-command-implementation rec=15
boundary: the harness run document
item: host_excluded
promise: Present only when a mutant was left out: an array of name and cfg as written in source, repo-relative.
@row_add n=113 row=contract/run-document/archived area=5-command-implementation rec=15
boundary: the harness run document
item: archived
promise: The repo-relative run-archive dir holding this run's JUnit files and read outcomes, the newest 10 kept and never uploaded; an absent source is skipped and named on stderr.
@row_add n=114 row=contract/run-document/reason area=5-command-implementation rec=15
boundary: the harness run document
item: reason
promise: A document refused by a tool arm carries reason and, where one applies, detail.
@row_add n=115 row=decision/skipped-counts-real-skips-only area=5-command-implementation rec=15
name: skipped counts real skips only
rule: skipped counts only selected tests not run; tests excluded by a layer filterset or compiled out by an OS cfg are never counted.
why: so the gate's suite-skipped breach fires only on a real skip
@row_add n=116 row=decision/doc-examples-never-ignore area=5-command-implementation rec=15
name: Doc examples never use ignore
rule: A doc example that must not run uses no_run or a text fence, never ignore.
why: ignored doctests count as skipped
@row_add n=117 row=decision/scoped-mutants-not-the-verdict area=5-command-implementation rec=16
name: Scoped mutation is the inner fix loop
rule: A file-scoped mutation run is the inner fix loop; the counted chunk mutation run stays the verdict.
@row_none n=3 rec=17 type=S
@row_add n=118 row=decision/status-strips-bridge-env area=5-command-implementation rec=18
name: Harness status strips the bridge env
rule: Harness status runs viola list with the session PATH and with VIOLA_NAME, VIOLA_DIR and VIOLA_BIN removed.
why: so the read files no role line under a host bridge's name
@row_add n=119 row=decision/status-bounded-read area=5-command-implementation rec=18
name: Harness status reads a bounded stdout
rule: Harness status reads the list output through a bound of one frame past the cap and parses it as one JSON object; with no supervisor record it spawns nothing.
