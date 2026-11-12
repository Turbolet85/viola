@row_add n=1 row=decision/no-recorder-authored-tests area=11-test-anti-patterns-never-do-these rec=1
name: No recorder-authored tests
rule: Tests are never authored by Selenium IDE or the Playwright codegen recorder.
@row_add n=2 row=decision/no-human-screenshot-review area=11-test-anti-patterns-never-do-these rec=2
name: No human screenshot review
rule: No test step relies on a human reviewing screenshots.
@row_add n=3 row=decision/no-real-network-in-tests area=11-test-anti-patterns-never-do-these rec=3
name: No real network calls in tests
rule: Tests never make real network calls; they use mocks or localhost stubs.
@row_add n=4 row=decision/no-test-owned-real-time area=11-test-anti-patterns-never-do-these rec=4
name: No test-owned real time
rule: Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict.
except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
@row_add n=5 row=decision/no-in-process-clock-across-processes area=11-test-anti-patterns-never-do-these rec=5
name: No in-process clock for another process
rule: An in-process mock clock never controls another process's time; cross-process expiry is asserted in a unit case.
why: A global mock clock reaches only the test process's threads, never a spawned viola run.
@row_add n=6 row=decision/no-interactive-snapshot-review area=11-test-anti-patterns-never-do-these rec=6
name: No interactive snapshot review as a gate
rule: Interactive snapshot review is never a gate step; snapshots change only through the local viola verify fixture refresh and are then committed.
@row_add n=7 row=decision/no-blanket-approval-tests area=11-test-anti-patterns-never-do-these rec=7
name: No blanket-approval tests
rule: No test auto-approves a dialog.
@row_add n=8 row=decision/no-test-passing-unstamped-decision-body area=11-test-anti-patterns-never-do-these rec=7
name: No test passing an unguarded decision body
rule: No test passes when a decision body is emitted without a verify stamp and the wheel held by driver.
@row_add n=9 row=decision/harness-output-json-and-exit-code area=11-test-anti-patterns-never-do-these rec=8
name: Harness commands always give JSON and exit code
rule: Every harness command prints its JSON document and exit code, never a human-only message alone.
@row_add n=10 row=decision/no-flaky-test-retries area=11-test-anti-patterns-never-do-these rec=9
name: No retries for flaky tests
rule: No retry-once policy or any retry budget exists for flaky tests.
why: The zero-flakiness budget.
@row_add n=11 row=decision/proptest-seeds-committed area=11-test-anti-patterns-never-do-these rec=10
name: Property tests replay from committed seeds
rule: Property tests replay deterministically: proptest regression seeds persist in committed regression files.
@row_add n=12 row=decision/chaos-faults-explicit area=11-test-anti-patterns-never-do-these rec=10
name: Chaos faults only from explicit operations
rule: Chaos faults come only from explicit process or file operations or existing mockall seams, so chaos runs replay deterministically.
@row_none n=1 rec=11 type=S
@row_none n=2 rec=12 type=S
@row_add n=13 row=decision/lint-fmt-and-clippy-deny-warnings area=ci-integration rec=13
name: Lint denies format drift and clippy warnings
rule: Lint fails on any rustfmt difference and on any clippy warning across the workspace and all targets, the fake-agent feature on.
guard: CI job lint
@row_add n=14 row=decision/sync-crates-compile-without-features area=ci-integration rec=13
name: Sync crates compile alone without features
rule: Each crate on the sync-crates list is checked alone with no features, so without tokio; a crate joins the list with its creation, and an empty list fails.
guard: CI job lint, one cargo check per listed crate
@row_add n=15 row=decision/no-orphan-modules area=ci-integration rec=13
name: No orphan modules
rule: No lib or bin target of the workspace holds an orphan module.
guard: orphans-check.sh with its probe of two planted orphans
@row_add n=16 row=decision/module-acyclicity-not-gated area=ci-integration rec=13
name: Module acyclicity is a review, not a gate
rule: Module dependency acyclicity is an on-demand boundary review, not a gate.
why: cargo-modules reads every type and its own inherent method as a cycle whatever the filters.
@row_add n=17 row=decision/rmcp-features-server-transport-io area=ci-integration rec=13
name: rmcp takes exactly server and transport-io
rule: rmcp is present in the release graph, and packages other than rmcp request exactly its server and transport-io features.
guard: lint step rmcp features (rmcp-features.sh)
@row_add n=18 row=decision/viola-channel-tokio-free area=ci-integration rec=13,14
name: viola-channel normal graph is tokio-free
rule: viola-channel's normal dependency graph holds no tokio; its Tokio client sits behind its tokio feature.
guard: channel-tokio-free.sh and deny probe channel-tokio-edge
@row_add n=19 row=decision/ci-single-ubuntu-leg area=ci-integration rec=13,17,18,20
name: CI runs on one ubuntu-latest leg
rule: CI jobs run on a single ubuntu-latest leg.
@row_add n=20 row=decision/cargo-deny-supply-chain-gate area=ci-integration rec=14
name: cargo deny supply-chain gate
rule: The supply-chain job fails closed on cargo deny advisories, licences, sources and bans for the workspace, and on advisories and sources for the fuzz workspace.
guard: CI job supply-chain
@row_add n=21 row=decision/serde-json-unbounded-depth-banned area=ci-integration rec=14
name: serde_json unbounded_depth banned
rule: serde_json's unbounded_depth feature is banned from the dependency graph.
guard: deny probe serde_json-unbounded-depth
@row_add n=22 row=decision/tokio-banned-per-sync-crate area=ci-integration rec=14
name: tokio banned from each sync crate
rule: tokio is banned from each crate on the sync-crates list, checked with that crate as sole root.
guard: step Tokio ban per sync crate (sole root)
@row_add n=23 row=decision/deny-bans-probed area=ci-integration rec=14
name: Every deny ban has a negative probe
rule: Every cargo deny ban has a permanent negative probe that must fire, beside a clean control.
guard: deny-probes.sh
@row_add n=24 row=decision/workflows-audited-by-zizmor area=ci-integration rec=14
name: Workflows audited by zizmor
rule: The GitHub workflows are audited by zizmor in the supply-chain job.
guard: CI job supply-chain
@row_add n=25 row=decision/weekly-advisory-scan area=ci-integration rec=14
name: Weekly advisory scan
rule: Advisories for the workspace and the fuzz workspace are re-checked weekly on schedule and on manual dispatch.
guard: nightly job advisories
@row_none n=3 rec=15 type=6
@row_none n=4 rec=16 type=6
@row_none n=5 rec=17 type=6
@row_add n=26 row=decision/one-instrumented-ci-test-run area=ci-integration rec=18
name: One instrumented test run in CI
rule: In CI one instrumented coverage run under nextest replaces the unit, integration and e2e runs, followed by the doctest run; the separate runs are the local equivalents.
guard: CI job test
@row_add n=27 row=decision/ci-jobs-end-in-harness-gate area=ci-integration rec=18
name: CI test jobs end in the harness gate
rule: Each test-bearing CI job's last step is viola-harness gate requiring that job's own suites.
@row_add n=28 row=decision/mutation-testing-off-ci area=ci-integration rec=19
name: Mutation testing outside CI
rule: Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.
@row_add n=29 row=decision/perf-verdict-from-harness-gate area=ci-integration rec=20
name: Perf verdict from the harness gate
rule: The perf verdict comes from viola-harness gate, never from a jq binary; CI neither installs nor pins jq.
guard: CI job perf, gate --require perf
@row_add n=30 row=decision/hyperfine-measures-perf area=ci-integration rec=20
name: hyperfine for perf measurement
rule: hyperfine is what measures perf, exporting the JSON the gate reads.
@row_add n=31 row=decision/msrv-1-96 area=ci-integration rec=21
name: Minimum supported Rust 1.96
rule: The workspace is checked and unit-tested on Rust 1.96, its minimum supported version.
why: 1.96 is the security toolchain floor and the requested workspace rust-version; sysinfo needs 1.95.
guard: CI job msrv, gate --require nextest-unit
@row_add n=32 row=decision/fuzz-corpus-replay-on-push area=ci-integration rec=22
name: Fuzz corpus replay on every push
rule: Every push and pull request replays each fuzz target's corpus without fuzzing.
guard: CI job fuzz-replay, gate --require fuzz-replay
@row_add n=33 row=decision/nightly-fuzzing area=ci-integration rec=22
name: Nightly fuzzing
rule: Each fuzz target is fuzzed for 120 seconds nightly, its crash artifacts uploaded on failure.
guard: nightly job fuzz
@row_add n=34 row=decision/release-build-viola-only area=ci-integration rec=23
name: Release build holds viola only
rule: The release build produces only the viola executable, with no test-only feature and no test-only binary.
guard: release-check.sh with its five probes and a clean control
@row_add n=35 row=decision/no-cache-in-release-workflow area=ci-integration rec=23
name: No build cache in a release workflow
rule: The Rust build cache is used in ci.yml only, never in a release workflow.
@row_add n=36 row=decision/harness-gate-fails-on-breach area=ci-integration rec=24
name: Harness gate fails on any quality breach
rule: viola-harness gate reads the junit, run-summary, coverage and hyperfine outputs and exits non-zero on any quality-gate breach.
@row_add n=37 row=decision/least-workflow-permissions area=ci-integration rec=24
name: Least workflow permissions
rule: Every workflow has empty permissions at the top and contents read per job.
@row_add n=38 row=decision/actions-sha-pinned area=ci-integration rec=24
name: Actions pinned by SHA
rule: Every action a workflow uses is pinned by SHA.
guard: zizmor unpinned-uses
@row_add n=39 row=decision/one-version-source-per-tool area=ci-integration rec=24
name: One version source per tool
rule: Each CI tool has exactly one pinned version source; cargo-deny and zizmor run as pinned CLIs, never through their actions.
@row_add n=40 row=decision/jq-runner-provided area=ci-integration rec=24
name: jq taken from the runner
rule: jq is taken from the CI runner, never installed; the scripts that need it refuse with tool-missing: jq when it is absent.
@row_add n=41 row=decision/ripgrep-pinned-checksum-verified area=ci-integration rec=24
name: ripgrep pinned and checksum-verified
rule: ripgrep is installed from a pinned, checksum-verified official release.
@row_add n=42 row=decision/local-pre-push-gate-installs-nothing area=ci-integration rec=24
name: Local pre-push gate installs nothing
rule: The local pre-push gate installs nothing; it checks the host's tools against the pins parsed from ci.yml and refuses a missing or off-pin tool.
why: ci.yml stays the one version source.
@row_add n=43 row=decision/no-workflow-concurrency area=ci-integration rec=24
name: No workflow concurrency block
rule: No workflow has a concurrency block; zizmor's concurrency-limits finding is declined and stays visible.
why: A concurrency group cancels pending runs even without cancel-in-progress, which would drop a push's always() gate and upload chain.
@row_add n=44 row=decision/ci-stamped-home-kill-line area=ci-integration rec=25
name: 20 s kill line for stamped-home tests
rule: On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.
@row_add n=45 row=decision/no-ignored-advisories area=ci-integration rec=26
name: No advisory ignored
rule: cargo deny ignores no advisory.
guard: CI job supply-chain
@row_add n=46 row=decision/cli-e2e-via-process area=e2e-test-strategy rec=27
name: CLI tested as a process
rule: CLI verbs are tested end to end by running the viola binary as a child process.
@row_add n=47 row=decision/cli-fixed-strings-pinned-by-literal area=e2e-test-strategy rec=27
name: CLI fixed strings pinned by literals
rule: Each fixed CLI string, the human layouts included, is pinned by a literal in its test file.
@row_add n=48 row=decision/tty-styling-asserted-on-sgr area=e2e-test-strategy rec=27
name: Terminal styling asserted on SGR sequences
rule: Terminal styling is checked by running a verb inside the outer PTY and asserting only on the SGR sequences in verb output, never on the child agent screen.
@row_add n=49 row=contract/cli-json-failure/document area=e2e-test-strategy rec=27
boundary: CLI --json failures
item: failure document
promise: Each CLI --json failure gives one document, a literal exit code, empty stderr, no ESC byte, and a body holding no path or upstream text.
@row_add n=50 row=decision/hook-e2e-recorded-stdin area=e2e-test-strategy rec=28
name: Hook verbs fed recorded stdin
rule: The hook verbs are tested by running them as processes fed fixture stdin recorded per agent CLI version.
@row_add n=51 row=decision/env-clear-keeps-profile-file area=e2e-test-strategy rec=28
name: Cleared environments keep the coverage profile
rule: A test that clears a child's environment re-adds LLVM_PROFILE_FILE.
@row_add n=52 row=decision/tui-outer-pty-driver area=e2e-test-strategy rec=29
name: Passthrough tested through an outer PTY
rule: The viola run passthrough is tested through a portable-pty outer PTY over the viola-pty seam, its exit read only by waiting on the child.
@row_add n=53 row=decision/outer-pty-reader-drained area=e2e-test-strategy rec=29
name: Outer PTY reader drained on its own thread
rule: An outer-PTY test drains the master reader on its own thread.
@row_add n=54 row=decision/tui-verdicts-from-records area=e2e-test-strategy rec=29
name: TUI verdicts from records
rule: TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status.
except: The forced vt100 panic case byte-searches what the drained master reader read.
@row_add n=55 row=decision/channel-e2e-driver area=e2e-test-strategy rec=30
name: Channel tested with the sync client
rule: The wrapper channel is tested with the viola-channel sync client, a raw interprocess stream and jsonschema validation.
@row_add n=56 row=decision/channel-endpoint-from-snapshot area=e2e-test-strategy rec=30
name: Channel endpoint read from the snapshot
rule: Channel tests read the wrapper endpoint only from the instance snapshot.
@row_add n=57 row=decision/mcp-tests-own-server-child area=e2e-test-strategy rec=31
name: MCP tests own their server child
rule: MCP tests spawn and own a viola mcp child, closing its stdin and awaiting its exit, never using rmcp's child-process transport.
why: That transport kills the server 3 s after a close, leaving a truncated coverage profile.
@row_add n=58 row=decision/viola-e2e-rmcp-client-only area=e2e-test-strategy rec=31
name: viola-e2e takes rmcp client only
rule: The viola-e2e crate takes rmcp feature client only, as a normal dependency, with no dev-dependencies and no child-process transport.
@row_add n=59 row=decision/library-surface-at-unit-tier area=e2e-test-strategy rec=32
name: Library-only surface covered at unit tier
rule: Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
@row_none n=6 rec=33 type=5
@row_add n=60 row=contract/viola-run-start/event-order area=e2e-test-strategy rec=34
boundary: viola run start
item: first three events
promise: The first three events.ndjson records are wheel{cause:"start"}, budget-gate, then session-start{source:"hook"} with data.cause "startup", in that order.
@row_add n=61 row=contract/viola-run-start/child-after-events area=e2e-test-strategy rec=34
boundary: viola run start
item: child spawn
promise: The child agent is spawned only after the start events are written.
@row_add n=62 row=contract/instance-snapshot/pinned-bin area=e2e-test-strategy rec=34
boundary: instance snapshot
item: pinned_bin
promise: The instance snapshot holds pinned_bin.
@row_add n=63 row=contract/instance-snapshot/pid area=e2e-test-strategy rec=34
boundary: instance snapshot
item: pid
promise: The instance snapshot holds pid.
@row_add n=64 row=contract/instance-snapshot/started-at area=e2e-test-strategy rec=34
boundary: instance snapshot
item: started_at
promise: The instance snapshot holds started_at.
@row_add n=65 row=contract/instance-snapshot/child-pid area=e2e-test-strategy rec=34
boundary: instance snapshot
item: child_pid
promise: The instance snapshot holds child_pid.
@row_add n=66 row=contract/instance-snapshot/endpoint area=e2e-test-strategy rec=34
boundary: instance snapshot
item: endpoint
promise: The instance snapshot holds the wrapper channel endpoint.
@row_add n=67 row=contract/viola-list-json/live-instance area=e2e-test-strategy rec=34
boundary: viola list --json
item: running wrapped instance
promise: A running wrapped instance's item reads wrapped:true and liveness "live".
@row_add n=68 row=contract/viola-run-start/duplicate-run area=e2e-test-strategy rec=34
boundary: viola run start
item: duplicate run
promise: A second run of an instance already running exits 1.
@row_add n=69 row=contract/viola-run-start/squatted-endpoint area=e2e-test-strategy rec=34
boundary: viola run start
item: squatted endpoint
promise: A run whose channel endpoint is already held by another listener exits 1.
@row_add n=70 row=contract/viola-run-start/tampered-pinned-exe area=e2e-test-strategy rec=34
boundary: viola run start
item: tampered pinned exe
promise: A run whose pinned executable was altered exits 1.
@row_add n=71 row=contract/viola-run-start/plugin-hooks area=e2e-test-strategy rec=34
boundary: viola run start
item: plugin hooks.json
promise: Each start rewrites the instance's plugin hooks.json, its hook commands the absolute pinned_bin path, never a PATH lookup.
@row_add n=72 row=contract/viola-run-start/settings-override area=e2e-test-strategy rec=34
boundary: viola run start
item: settings.json
promise: Each start rewrites settings.json whole at 0600 as a statusLine command of the absolute pinned path plus hook statusline, only when every path character is an ASCII letter, a digit or one of _ - . / :.
@row_add n=73 row=contract/viola-run-start/home-tree-check area=e2e-test-strategy rec=34
boundary: viola run start
item: home-tree strict-modes check
promise: The home-tree check runs before the wrapper log opens; a refused tree exits 1 with the home-state or instance-state unable/hint pair, opens no log, writes no line and starts no child.
@row_add n=74 row=contract/viola-run-start/socket-dir-check area=e2e-test-strategy rec=34
boundary: viola run start
item: socket directory check
promise: The socket directory is checked before any snapshot, child or bind, revive included; a refused directory exits 1, logs process-exit with detail socket-dir-refused, binds nothing and spawns no child.
@row_none n=7 rec=35 type=8
@row_none n=8 rec=36 type=5
@row_add n=75 row=contract/viola-send/turn-running area=e2e-test-strategy rec=37,52
boundary: viola send
item: turn-running refusal
promise: While a turn runs, send exits 13 not-delivered with detail turn-running, types nothing, adds only a send-refused record and hints to wait first; after turn-ended a send is accepted.
@row_add n=76 row=contract/mcp-tools/result-body area=e2e-test-strategy rec=37,39,41,46,50
boundary: MCP tools
item: result and refusal body
promise: Each MCP tool's result and refusal equal the CLI's --json document less v.
@row_none n=9 rec=38 type=5
@row_add n=77 row=contract/viola-send/ok-document area=e2e-test-strategy rec=39
boundary: viola send
item: ok document
promise: A confirmed send exits 0 with ok.cursor at the events file's length before the send and ok.submitted_at in RFC 3339 ms Z.
@row_add n=78 row=contract/viola-send/records area=e2e-test-strategy rec=39
boundary: viola send
item: send records
promise: The records after the send's cursor are send-issued{cursor, from} then prompt-submitted{origin:"driver"} holding the typed text.
@row_add n=79 row=contract/viola-send/paste-framing area=e2e-test-strategy rec=39
boundary: viola send
item: typed framing
promise: A sent text is typed as one bracketed paste followed by one CR, its newlines kept inside.
@row_add n=80 row=contract/viola-send/trailing-newlines area=e2e-test-strategy rec=39
boundary: viola send
item: trailing newlines
promise: A text ending in LF, CR, CRLF or two CRs is typed without that ending and confirmed.
@row_add n=81 row=contract/viola-send/empty-text area=e2e-test-strategy rec=39
boundary: viola send
item: empty-text refusal
promise: A text empty once its trailing newlines are removed is refused at once, exit 13 not-delivered/empty-text, and never issued.
@row_add n=82 row=contract/viola-send/inner-cr area=e2e-test-strategy rec=39
boundary: viola send
item: inner CR
promise: A CR or CR LF inside a text is typed as one LF, so the send is confirmed and the driver keeps the wheel.
@row_add n=83 row=contract/viola-send/no-prompt-submitted area=e2e-test-strategy rec=39
boundary: viola send
item: unconfirmed send
promise: A send with no submitted prompt exits 13 {"refusal":"not-delivered","detail":"no-prompt-submitted"} and logs send-refused with the same refusal and detail.
@row_add n=84 row=contract/viola-send/unconfirmable-local-command area=e2e-test-strategy rec=39
boundary: viola send
item: local command with no measured post-condition
promise: Exits 0 with ok {confirmed:false, detail:"unconfirmable", cursor}, logs send-issued then send-confirmed{confirmed:false}, no send-refused, no confirmation window.
@row_add n=85 row=contract/viola-send/compact-confirmed area=e2e-test-strategy rec=39
boundary: viola send
item: /compact on a verified CLI
promise: Confirmed by its post-condition alone, a session-start with cause compact and the same agent_session_id, within its confirmation window, else not-delivered/no-prompt-submitted.
@row_add n=86 row=decision/compact-confirm-window area=e2e-test-strategy rec=39
name: Compact confirmation window
rule: A /compact send waits 40 s for its post-condition before it expires.
state: provisional
@row_add n=87 row=contract/viola-send/clear-confirmed area=e2e-test-strategy rec=39
boundary: viola send
item: /clear on a verified CLI
promise: Confirmed by its post-condition alone, a session-start with cause clear and a new agent_session_id, ending exit 0 {submitted_at, cursor}.
@row_add n=88 row=contract/viola-send/off-list-slash-text area=e2e-test-strategy rec=39
boundary: viola send
item: slash text off the compiled list
promise: Exits 13 not-delivered/no-prompt-submitted.
@row_add n=89 row=contract/viola-send/human-key-during-gate area=e2e-test-strategy rec=39
boundary: viola send
item: human key during the gate wait
promise: The human key wins: the wheel record precedes send-refused, send exits 10 human-typing with nothing typed, and the key reaches the child.
@row_add n=90 row=contract/viola-send-human-mode/read-back area=e2e-test-strategy rec=39
boundary: viola send human mode
item: confirmed line
promise: A confirmed send prints the stdout line [RB] read back … cursor <n>.
@row_add n=91 row=contract/viola-send-human-mode/unable area=e2e-test-strategy rec=39
boundary: viola send human mode
item: refusal lines
promise: A refused send prints a stderr [/ ] unable line naming instance, refusal and detail, then a last-line hint:.
@row_add n=92 row=contract/viola-send-human-mode/unconfirmable area=e2e-test-strategy rec=39
boundary: viola send human mode
item: unconfirmable line
promise: A local command with no measured post-condition prints one stdout line [  ] unconfirmable <name> local command, no measured post-condition, exit 0, nothing on stderr.
@row_none n=10 rec=40 type=5
@row_add n=93 row=contract/turn-ended/background-tasks area=e2e-test-strategy rec=41
boundary: turn-ended event data
item: background_tasks
promise: The Stop array's length, or null.
@row_add n=94 row=contract/mcp-wait/timeout-ms area=e2e-test-strategy rec=41
boundary: MCP wait
item: timeout_ms
promise: The bound sent is clamped to 50 000 ms, which is also its default.
@row_none n=11 rec=42 type=5
@row_add n=95 row=contract/viola-wait/non-waking-records area=e2e-test-strategy rec=43
boundary: viola wait
item: activity and wheel records
promise: Activity and wheel records never wake a parked wait.
@row_add n=96 row=contract/viola-wait/turn-ended area=e2e-test-strategy rec=43
boundary: viola wait
item: turn-ended wake
promise: Exits 0 with event.kind "turn-ended" and a cursor at or past that record's offset.
@row_add n=97 row=contract/viola-wait/already-past area=e2e-test-strategy rec=43
boundary: viola wait
item: event already past the cursor
promise: A wait after a cursor whose event is already logged returns that event at once, exit 0, no timed_out.
@row_add n=98 row=contract/viola-wait/timeout area=e2e-test-strategy rec=43
boundary: viola wait
item: timeout
promise: A wait that times out gives {timed_out:true} with exit 0.
@row_add n=99 row=contract/viola-last/document area=e2e-test-strategy rec=43
boundary: viola last
item: last document
promise: Gives {last_assistant_message, ts} of the last turn.
@row_add n=100 row=contract/cli-exit-codes/21 area=e2e-test-strategy rec=43,41
boundary: CLI exit codes
item: 21
promise: Instance unreachable: {"v":1,"error":"instance-unreachable","detail":<cause>}, the cause endpoint-lost, instance-dead or unwrapped-name.
@row_add n=101 row=contract/mcp-tools/instance-unreachable area=e2e-test-strategy rec=43
boundary: MCP tools
item: instance-unreachable
promise: is_error:true with the CLI document less v, text refused: instance-unreachable.
@row_none n=12 rec=44 type=5
@row_add n=102 row=contract/viola-run/child-on-wrapper-kill area=e2e-test-strategy rec=45
boundary: viola run wrapper
item: wrapper killed
promise: When the wrapper is killed, its child agent is gone within a bounded time.
@row_add n=103 row=contract/question-event/option-details area=e2e-test-strategy rec=46
boundary: question event
item: option_details
promise: One {description, preview} per option, each a string or null.
@row_add n=104 row=contract/viola-answer/unverified-cli area=e2e-test-strategy rec=46
boundary: viola answer
item: permission on an unstamped CLI
promise: The permission is left to the human and an allow is refused exit 12 unverified-cli, with no decision body.
@row_add n=105 row=contract/viola-answer/allow-suggestion-fields area=e2e-test-strategy rec=46
boundary: viola answer
item: permission allow
promise: An allow's suggestion fields never reach the agent CLI.
@row_none n=13 rec=47 type=5
@row_add n=106 row=contract/dialog-events/logged-once area=e2e-test-strategy rec=48
boundary: dialog events
item: question, permission, plan
promise: Each dialog event is logged exactly once with a dialog_id, and a parked wait returns it at once.
@row_add n=107 row=contract/hook-process/exit area=e2e-test-strategy rec=48
boundary: dialog hook process
item: exit and stderr
promise: The hook process exits 0 with empty stderr.
@row_add n=108 row=contract/hook-decision/question area=e2e-test-strategy rec=48
boundary: dialog hook decision body
item: question answer
promise: PreToolUse permissionDecision "allow" with updatedInput.answers, annotations included.
@row_add n=109 row=contract/hook-decision/permission area=e2e-test-strategy rec=48
boundary: dialog hook decision body
item: permission answer
promise: PermissionRequest decision.behavior "allow", or "deny" with message.
@row_add n=110 row=contract/hook-decision/plan-approve area=e2e-test-strategy rec=48
boundary: dialog hook decision body
item: plan approve
promise: PreToolUse allow with updatedInput as the tool's own input unchanged, and no PermissionRequest body.
@row_add n=111 row=contract/hook-decision/plan-revise area=e2e-test-strategy rec=48
boundary: dialog hook decision body
item: plan revise
promise: PermissionRequest deny with message.
@row_add n=112 row=contract/hook-decision/concurrent-dialog area=e2e-test-strategy rec=48
boundary: dialog hook decision body
item: second concurrent dialog
promise: A second dialog raised while one is pending gets an empty hook stdout.
@row_add n=113 row=contract/viola-answer/unknown-dialog area=e2e-test-strategy rec=48,46
boundary: viola answer
item: unknown dialog id
promise: Exits 13 {"refusal":"not-delivered","detail":"unknown-dialog"}.
@row_add n=114 row=contract/viola-answer/human-wheel area=e2e-test-strategy rec=48
boundary: viola answer
item: wheel held by human
promise: answer exits 10 and the hook stdout stays empty.
@row_add n=115 row=decision/no-decision-body-without-driver-and-stamp area=e2e-test-strategy rec=48
name: Decision bodies only for a driver on a stamped CLI
rule: No dialog decision body is ever emitted unless the wheel is driver and the agent CLI version is stamped.
@row_none n=14 rec=49 type=5
@row_add n=116 row=decision/terminal-replies-not-editing area=e2e-test-strategy rec=50
name: Terminal replies are not editing
rule: Terminal reply shapes on a closed list are read as not editing and never take the wheel; any other stdin input is typing.
guard: classifier_terminal_reply_is_not_editing, classifier_typing_is_editing
@row_add n=117 row=decision/focus-mouse-resize-keep-wheel area=e2e-test-strategy rec=50
name: Focus, mouse and resize never take the wheel
rule: Focus, mouse and resize input never takes the wheel from the driver.
guard: tui_focus_mouse_and_resize_never_take_the_wheel
@row_add n=118 row=contract/viola-run-passthrough/ctrl-z area=e2e-test-strategy rec=50
boundary: viola run passthrough
item: Ctrl-Z
promise: Ctrl-Z reaches the child, and later keys still do.
@row_add n=119 row=decision/terminal-reply-reading-live-measured area=e2e-test-strategy rec=50
name: Terminal reply reading measured live
rule: The classifier's reading of terminal replies on a real terminal is a local live measurement, not a suite.
why: The outer-PTY tier writes no terminal reply of its own.
@row_none n=15 rec=51 type=5
@row_add n=120 row=contract/events/human-input-wheel area=e2e-test-strategy rec=52
boundary: events.ndjson
item: human input
promise: Human editing input logs wheel{holder:"human",cause:"human-input"}.
@row_add n=121 row=contract/viola-send/human-typing area=e2e-test-strategy rec=52
boundary: viola send
item: human-typing refusal
promise: While the human holds the wheel, send exits 10 human-typing with detail null.
@row_add n=122 row=contract/mcp-send/human-typing area=e2e-test-strategy rec=52
boundary: MCP send
item: human-typing refusal
promise: is_error:true with text refused: human-typing.
@row_add n=123 row=decision/human-never-blocked area=e2e-test-strategy rec=52
name: The human is never blocked
rule: A human's typed prompt always reaches the agent and is logged prompt-submitted{origin:"human"}.
@row_add n=124 row=contract/viola-release/document area=e2e-test-strategy rec=52
boundary: viola release
item: release document
promise: Gives {wheel:"driver", budget_paused:false}, after which a send exits 0.
@row_add n=125 row=contract/viola-send/manual-pause area=e2e-test-strategy rec=52
boundary: viola send
item: manual-pause refusal
promise: After pause, send exits 10 with detail "manual-pause".
@row_add n=126 row=decision/hints-never-suggest-release area=e2e-test-strategy rec=52,50
name: Driver hints never suggest release
rule: No driver-facing hint, MCP refusal text, structured content, result or tool description names release.
why: Drivers are refused release, so such a hint sends the driver into a dead end.
@row_add n=127 row=contract/viola-release/from-driver area=e2e-test-strategy rec=52
boundary: viola release
item: release from a driver
promise: Exits 20 {"error":"wrapper-fault","detail":{"code":-32602,…}} carrying release-from-driver.
@row_add n=128 row=contract/events/harness-prompt area=e2e-test-strategy rec=52
boundary: events.ndjson
item: harness-injected turn
promise: Logs prompt-submitted{origin:"harness"} and no wheel record.
@row_add n=129 row=contract/viola-release/running-turn area=e2e-test-strategy rec=52
boundary: viola release
item: turn left running
promise: Under a driver-held wheel a bare release leaves a running turn and send stays 13; pause then release clears it.
@row_none n=16 rec=53 type=5
