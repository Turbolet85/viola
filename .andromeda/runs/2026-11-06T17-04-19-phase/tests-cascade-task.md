@task_prose part=chunk
The version done-check that closes viola-0.1.0 on Linux. It is mechanism and record work, with no new verb, flag, env var or channel method.
It moves portable-pty off =0.8.1 to a release whose graph drops pin-utils, clearing RUSTSEC-2026-0441 in cargo deny and CI's supply-chain job without widening the ignore list.
It removes the Windows code remnants of freight item 17, or keeps each one with its reason, and the masters follow.
It re-measures at HEAD the missing trycmd/assert_cmd tests/cmd, PTY_SIDELOAD_TEST_CASE, the thiserror derive census and the channel-handle no-leak property under /proc/<child pid>/fd, and it runs up to two headless live sessions (v1-53's live viola run on Linux and a driver driving a builder through viola).
It re-reads every gate green: fmt, clippy, check, test, cargo deny, deny probes, tokio-free checks, release check and CI's seven jobs. It also records a proved or deferred disposition for each of the 17 unclaimed caps, on the founder's word.
@task_file n=1 path=Cargo.toml mark=edits
what: the portable-pty dependency pin (=0.8.1 moved to a release without pin-utils)
@task_file n=2 path=Cargo.lock
@task_file n=3 path=deny.toml mark=re-reads
what: cargo deny check advisories reading RUSTSEC-2026-0441 cleared with no ignore-list widening
@task_file n=4 path=crates/viola-pty
@task_file n=5 path=tests/cli_output_plain.rs mark=edits
what: the Windows remnants at lines 104 and 116
@task_file n=6 path=crates/viola-agent-claude/src/lib.rs mark=edits
what: the Windows remnant at line 2
@task_file n=7 path=crates/viola-e2e/src/harness/session.rs mark=edits
what: the Windows remnant at line 52
@task_file n=8 path=crates/viola-e2e/src/harness mark=edits
what: the EXE_SUFFIX joins, the COVERAGE_FLOORS per-OS comment and the COVERAGE_IGNORE Windows-path comment
@task_file n=9 path=src/cmd/mod.rs mark=edits
what: plain_drive
@task_file n=10 path=tests/contract_os_gated_tests.rs mark=re-reads
what: the OS-gated test census (39 gated test functions, 2 gated files) that v1-53's claim reads
@task_file n=11 path=.github/workflows/ci.yml mark=re-reads
what: CI's seven jobs read green at the wrap, the supply-chain job included
@task_file n=12 path=scripts/deny-probes.sh mark=re-reads
what: the deny probes
@task_file n=13 path=scripts/channel-tokio-free.sh mark=re-reads
what: the tokio-free checks
@task_file n=14 path=scripts/release-check.sh mark=re-reads
what: the release check
@task_item n=1 part=needs
The supply-chain coverage trigger and the CI supply-chain job. cargo deny runs over the root and fuzz lockfiles, its only ignore is RUSTSEC-2017-0008, and the weekly nightly advisories re-run comes back clean. The portable-pty move has to clear RUSTSEC-2026-0441 inside these rules.
@task_item n=2 part=needs
The viola-pty entity and its seam over portable-pty, which the plan pins at =0.8.1. The seam keeps six methods, exit is read from the process handle and never from reader EOF, and consumers are tested against a mockall double. The new portable-pty release must keep all of this, and the plan's version text follows the new pin.
@task_item n=3 part=needs
The Module-to-PTY integration boundary. It covers real openpty spawns of the fake agent through viola-pty, exit detected on the handle, a resize propagated, and a key written after a resize. A dependency move under viola-pty must keep these green.
@task_item n=4 part=needs
The lint gates must stay green: fmt, clippy -D warnings, the tokio-free cargo check of each crate listed in scripts/sync-crates.txt, the cargo-modules orphans gate and its probe, and the rmcp release-graph assertion.
@task_item n=5 part=needs
The supply-chain build-failure lines must stay green: deny-probes 14/14 banned with a clean control, the sole-root tokio ban per sync crate, channel-tokio-free ok, and zizmor.
@task_item n=6 part=needs
The release-build line, release-check: viola only, with its probes 5/5 refused and a clean control. The MSRV 1.96 job and the fuzz corpus replay are also among the seven CI jobs that must read green.
@task_item n=7 part=needs
The coverage gate in the test job on ubuntu-latest: lines 85, functions 95 and regions 80. COVERAGE_IGNORE's separator-agnostic regex over its four trees stays unwidened while its Windows-path comment is reworded or removed. COVERAGE_FLOORS and the plan's per-OS coverage wording follow item 17.
@task_item n=8 part=needs
The multi-os-compat trigger: the product is Linux-only, and the census in tests/contract_os_gated_tests.rs treats any gate whose predicate names windows or macos as a fault. Removing remnants must leave the census count and its gated files consistent.
@task_item n=9 part=needs
The viola --help literal HELP in tests/cli_output_plain.rs, which pins the human layout. Editing that file's Windows remnants must keep the literal assertion holding.
@task_item n=10 part=needs
The harness pieces the EXE_SUFFIX joins sit in, such as the session record and boot paths: each harness command still prints one JSON document and exits 0, 1 or 2 after the joins go.
@task_item n=11 part=needs
Item 1 (tool or text). The plan names trycmd 1.2.1 case files as the human-mode layout driver, trycmd only in verify mode, and trycmd cases with snapbox redactions as the expected output. No tests/cmd exists and no manifest holds trycmd, so the masters-agree check decides between the tool and the text, and the plan's wording follows.
@task_item n=12 part=needs
The test-child markers MCP_SERVE_TEST_CHILD and PTY_SIDELOAD_TEST_CASE. Each one either gets a row in architecture's Occupied Resources or has the absence explained, and PTY_SIDELOAD_TEST_CASE's presence at HEAD is re-measured.
@task_item n=13 part=needs
The DialogHold bounds of 60 to 3600 s in its unit cases, with a 60 s default. The 3600 s ceiling stays PROVISIONAL until a hold near it is read live in one of this chunk's live sessions.
@task_item n=14 part=needs
The untestable zone for the real claude CLI. A live reading is the route entry's own local measurement, kept in its evidence/ with its own launcher, driver and readers, never a test layer or a CI step. A green fake-agent run is never proof of real-CLI behaviour. Both live sessions sit inside this rule.
@task_item n=15 part=needs
v1-53's claim reads the census and the ubuntu-latest JUnit alone (target/nextest/ci/junit.xml, the junit artifact of the test job). CI conclusions are read with gh run view --json jobs, and artifacts are fetched with gh run download. v1-01's CI-run outcome and advisories clause are read the same way.
@task_item n=16 part=needs
The fake agent's Unix behaviour that v1-53's live run confirms. On Unix every command it spawns runs in its own process group, and its exit ignores SIGHUP from the first step. The fake-agent reading run_child_ends_when_its_wrapper_is_terminated also stands for item 11.
@task_item n=17 part=needs
The channel-handle no-leak property. The IPC trigger requires no inherited channel handles in the child, the fake agent writes an fds receipt kind, and the E2 scenario says the child holds no viola socket. Item 15 measures this on Linux through /proc/<child pid>/fd under the fake agent.
@task_item n=18 part=needs
The cross-OS-user untestable zone for item 8. An always-admit peer_admitted mutant at the three client seams survives every committed test, and no seam may be added. The plan's section 1 holds it as the untestable zone, and matrix v1-44 holds the dated note.
@task_item n=19 part=needs
The MCP tool list of exactly send, wait, last, answer and list, never release or link. The E3 link/unlink scenario has its CLI halves built. Together they are why v1-13's MCP link and budget-release clauses and v1-36's board link set need a deferral or a reworded acceptance.
@task_item n=20 part=needs
The wheel's closed non-editing list, behind Path 5's human-input wheel flip. A shape joins it only when measured by its exact grammar on a live terminal with a real claude, on the founder's word (item 10).
@task_item n=21 part=needs
The test-plan's own Windows wording, which follows item 17's remnant removal: the per-OS session-end perf row, the Windows dev-host perf reading, the identity floor measured on the Windows host, the Windows ConPTY EOF source on viola-pty, and the Windows-path note on the coverage regex.
@task_item n=22 part=needs
The socket-directory verdict tests and the board's read rule (list refusing a home another user may read). Item 7 asks the founder's word on the socket-directory pair's wording and on the board's read rule.
@task_item n=23 part=needs
The pinned copy's SHA-256 16-hex re-hash and its key under bin/<key>/. Measurement B (item 5) found one tree keying two folders through unequal build-id, symtab and strtab sections. Acting on that finding is the founder's call.
@task_item n=24 part=needs
The process-start diagnostics line. obs-plan's row narrows pty_backend to openpty and drops sideload_fallback, and the schema conformance step (gate G4 against diag-line.v1.json) must still pass every home-level diagnostics line.
@task_standing n=1
convention: each new Rust integration test binary takes its surface_topic prefix (cli_, hook_, tui_, channel_, chaos_, contract_ in root; path_, mcp_, cross_ in viola-e2e) so the layer filtersets select it
home: ## 2. Test Strategy
@task_standing n=2
convention: each new test function is named subject_condition_expected in snake_case, and table cases carry readable labels
home: ## 4. Unit Test Strategy
@task_standing n=3
convention: each new test makes its own fresh home under target/e2e-home through viola_state::fs::create_private_dir, shares nothing, and seeds no state by hand
home: ## 7. Test Data & Fixtures
@task_standing n=4
convention: a new test never sleeps for synchronisation; its waits key on events.ndjson byte offsets or wait --after, and it reads live-appended files only through tests/support/ndjson.rs
home: ## 11. Test Anti-Patterns
@task_standing n=5
convention: a new test sets env per child with Command::env, never set_var, and any env_clear re-adds LLVM_PROFILE_FILE
home: ## 11. Test Anti-Patterns
@task_standing n=6
convention: no retries, no #[ignore] parking, and zero flakiness; a flaky test keeps the chunk red until its root cause is fixed
home: ## 10. Quality Gates & Coverage Targets
@task_standing n=7
convention: the product is Linux-only; a new test gate naming windows or macos is a census fault
home: test-scope-summary
@task_standing n=8
convention: any workflow change keeps permissions {} at the top, contents read per job, every uses SHA-pinned, rust-cache as the only cache, and no concurrency block
home: ## 9. CI Integration
@task_standing n=9
convention: no test or CI step runs the real claude CLI; real-CLI readings are local live measurements only
home: ## 11. Test Anti-Patterns
@task_item n=1 part=not_needed
Path 2's confirmed-send scenario: bracketed-paste framing, CR/LF typed-text rules, local-command post-conditions and the empty-text refusal. The chunk changes no send logic.
@task_item n=2 part=not_needed
Path 7's unverified CLI version gate scenario and its stamped/unstamped verify line.
@task_item n=3 part=not_needed
The E5 snapshot-corruption replay scenario and its chaos_snapshot_replay cases.
@task_item n=4 part=not_needed
The list row-integrity suite in cross_list_rows.rs, which covers escaped unwrapped names and column offsets.
@task_item n=5 part=not_needed
viola-core's unit cases for RefusalReason serde, ViolaName, Percent, the v constants and validate_paste_text.
@task_item n=6 part=not_needed
viola-mcp's handler re-validation of schema-violating inputs, its stdin line cap at MAX_FRAME, and its reply screen.
@task_item n=7 part=not_needed
The fake agent's turn-script grammar, the fake-script schema, the control-file gating and its mode flags.
@task_item n=8 part=not_needed
The verify recorder's scrub, the screen-fixture scrub-as-detector and the recorded MCP exchange tap. The chunk records no fixture.
@task_item n=9 part=not_needed
The insta decision-body snapshots per CLI version and the dialog decision bodies of Path 4.
@task_item n=10 part=not_needed
Mocking discipline for time (Clock, mock_instant, paused tokio clock) and the absence of HTTP mocks.
@task_item n=11 part=not_needed
The exit-cause matrix's hint table in cross_exit_causes.rs: one hint per cause and no hint naming release.
@task_item n=12 part=not_needed
The budget data source and statusline parsing: rate_limits thresholds, budget-gate events and the unknown fallback.
@task_item n=13 part=not_needed
The E1 scenario, where unwrapped session hooks are a silent no-op.
@task_item n=14 part=not_needed
Running the mutation gate itself (run --mutants, outcomes.json missed/timeout/unviable verdict). The code audit runs once at the version end and is not this chunk's work.
