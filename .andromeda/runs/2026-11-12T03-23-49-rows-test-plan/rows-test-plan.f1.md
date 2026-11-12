@row_add n=1 row=decision/real-claude-local-only area=1-test-scope-summary rec=1,2,3,4,5,6
name: Real claude CLI tested local-only
rule: Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
@row_add n=2 row=contract/viola-hook/stdout area=1-test-scope-summary rec=7,8,9
boundary: viola hook exec-form entry points
item: stdout
promise: stdout is either empty (no decision, async hooks print nothing) or exactly the PreToolUse or PermissionRequest decision body.
@row_add n=3 row=decision/sync-crates-without-tokio area=1-test-scope-summary rec=10,11,12,13,14
name: Sync crates build without tokio
rule: Every listed sync crate builds without tokio and passes the tokio ban; a sync crate failing to build without tokio, or an empty list, fails the build.
guard: cargo check of the sync crates without tokio; deny-sync.toml tokio ban
@row_add n=4 row=decision/release-build-viola-only area=1-test-scope-summary rec=10,15,16,17
name: Release build carries viola only
rule: The release build carries viola alone and fails on any test-only feature or test-only binary in it, with no upload and no gate step.
guard: release-check.sh and its probe
@row_add n=5 row=decision/sha-pinned-actions area=1-test-scope-summary rec=10,18,19
name: SHA-pinned Actions
rule: Every Action a workflow uses is SHA-pinned, never referenced by a mutable tag.
guard: zizmor unpinned-uses
@row_add n=6 row=contract/viola-list-json/budget area=1-test-scope-summary rec=20
boundary: viola list --json
item: budget
promise: The ok payload's budget is {five_hour:{used_percentage,resets_at}, seven_day:{...}, read_at, paused} or "unknown".
@row_add n=7 row=contract/viola-list-json/skipped area=1-test-scope-summary rec=20
boundary: viola list --json
item: skipped
promise: The ok payload's skipped holds unknown_kinds, unknown_fields and torn_lines.
@row_add n=8 row=decision/logs-carry-no-secrets area=1-test-scope-summary rec=21,22
name: Logs carry no secrets
rule: No log contains the Cookie header or a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line.
guard: viola-harness secret-scan
@row_add n=9 row=contract/instance-snapshot/pid area=1-test-scope-summary rec=23,24
boundary: instance snapshot.json
item: pid
promise: The wrapper pid lives in the instance's snapshot.json, with started_at and child_pid; there is no standalone pid file.
@row_add n=10 row=decision/self-bootstrapping-test-data area=1-test-scope-summary rec=25,26
name: Self-bootstrapping test data
rule: Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, budget.json is seeded through viola hook statusline, stamps come from viola verify against the fake agent, and no state is developer-seeded.
@row_add n=11 row=decision/human-never-blocked area=1-test-scope-summary rec=27,28,29
name: The human is never blocked
rule: The human is never blocked: human typing still reaches the agent while automation is refused.
@row_add n=12 row=contract/viola-verify/stamped-line area=1-test-scope-summary rec=30,31
boundary: viola verify
item: stamped line
promise: viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail".
@row_add n=13 row=contract/mcp-stdin/oversize-line area=1-test-scope-summary rec=32,33,34
boundary: viola mcp stdin
item: line past MAX_FRAME
promise: A stdin line past MAX_FRAME is refused whole, passes no byte, and ends the session: earlier lines answered, exit 0, stderr empty, stdout only the earlier frames, and one parse-rejected mcp-stdin oversize WARN line holding no byte of it.
