@task_prose part=chunk
Makes viola Linux-only. First it sets the tag multi-os-final on the last commit that still holds the three-OS code and pushes it to origin, before any removal commit. It re-reads at take-up that no multi-os-* tag stands and that the four Windows-only files hold 1 146 lines.
Removes every cfg(windows) and target_os = "macos" arm with its tests and fixtures, the four Windows-only files, vendor/conpty/, and the cfg(windows) target dependencies with their deny.toml entries. Any cfg(unix) arm whose only sibling is gone loses its gate.
Removes the Windows and macOS jobs from ci.yml and nightly.yml, deletes windows-mutants.yml whole, and removes any pre-push or harness step that runs only for those OSes. Re-points or retires the census of OS-gated tests.
Makes the two viola-e2e harness cases that read the host's real free space against the e2e-home floor decide on a figure the test controls, so the msrv job goes green. P3 closes this against ci#41930831242 on abf00915.
Changes no Linux behaviour, closed word, exit code or wire contract. The masters' OS-target change, including the obs-plan's per-OS logging rows, is the wrap's amendment, and this chunk's plan names it as expected.
@task_file n=1 path=src/conpty.rs
@task_file n=2 path=crates/viola-pty/src/sideload.rs
@task_file n=3 path=tests/conpty_sideload.rs
@task_file n=4 path=tests/contract_windows_mutation_scope.rs
@task_file n=5 path=vendor/conpty/
@task_file n=6 path=.github/workflows/windows-mutants.yml
@task_file n=7 path=Cargo.toml mark=edits
what: the [target.'cfg(windows)'] dependencies
@task_file n=8 path=deny.toml mark=edits
what: the entries of the removed Windows dependencies
@task_file n=9 path=.github/workflows/ci.yml mark=edits
what: the Windows and macOS jobs (test, lint, perf, release) and the two pwsh shim steps
@task_file n=10 path=.github/workflows/nightly.yml mark=edits
what: the Windows and macOS jobs
@task_file n=11 path=tests/contract_os_gated_tests.rs mark=edits
what: the census of OS-gated tests, re-pointed or retired with the gates it counted
@task_file n=12 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the cases e2e_home_backing_gone_target_is_made_again_owner_only and e2e_home_backing_owner_only_target_is_kept_untouched, and the free-space floor message they panic with
@task_file n=13 path=scripts/agent-run.ps1
@task_item n=1 part=needs
The CI platform and matrix: the windows-2025 / macos-latest / ubuntu-latest matrix shrinks to ubuntu-latest. Each remaining Linux job keeps its gate steps G1 to G4, each its own bash run step copied verbatim into ci.yml.
@task_item n=2 part=needs
The CI step order and conditions on the remaining Linux test and perf jobs. Tests come first, then G2 and G4 under if: always() with homes kept. Then come the harness capture on failure(), the secret-scan step under if: always(), and the diag-<os>, harness-<os> and junit-<os> uploads gated on secret-scan success, with the secret-scan-<os> hit report uploaded on scan failure.
@task_item n=3 part=needs
The nightly.yml jobs that stay: the weekly cargo deny advisories job and the fuzz job (120 s per target, fuzz/artifacts/ uploaded on failure() only). Only the Windows and macOS jobs leave.
@task_item n=4 part=needs
The dispatch-only windows-mutants.yml, which is the boundary audit's Windows mutation leg. It is deleted, and the mutation gate (run --mutants, survived mutants in obs code) stays on Linux.
@task_item n=5 part=needs
The perf budgets on the per-OS perf job, which now has a Linux leg only. run --perf still times the five hook rows, gate --require perf requires all five, and the hyperfine max stays under the 1.0 s budgets with VIOLA_NAME set and a home under target/e2e-home/.
@task_item n=6 part=needs
The error budget and its measurement: zero panic lines by G2, zero G4 schema-conformance failures and zero secret-scan hits, read from the in-job gate exit codes on the Linux jobs that remain.
@task_item n=7 part=needs
Gates that the removal edits must still pass. cargo deny (Tokio, C-crate, veil toggle and banned obs crates) runs after the Windows dependencies and their deny.toml entries leave. G3 (no panic = abort in Cargo.toml, cargo config or workflows) runs over the edited manifests and workflows. Clippy disallowed-macros, print_stdout, print_stderr and dbg_macro also apply, as does scripts/lint-probes.sh.
@task_item n=8 part=needs
The multi-platform-exporter-compat trigger (the same file sink and line schema on every OS) and the obs-plan's per-OS rows. Examples are the ConPTY / openpty and named pipe / Unix socket OS APIs, the windows-sys TerminateProcess fallback on the viola-pty entity, and the 0600 / 0700 rule with its Windows home-DACL arm. These are the expected amendments the wrap writes, and the chunk's plan names them.
@task_item n=9 part=needs
The run start scenario's ConPTY sideload companions: the sideload_fallback field on process-start (strict-modes-failed on a refused conpty/ companion) and the closed words conpty-sideload and conpty. They name code this chunk removes. P3 lists each one, and P4 decides it against the masters' change.
@task_item n=10 part=needs
The wait / last and MCP server-verification paths. The macOS carve-out (an euid mismatch alone, no peer pid) leaves with the macOS arm. On Linux, the peer-pid and euid comparison, server-verify-failed, socket-dir-refused and strict-modes-failed details, exit 21 and instance-unreachable stay unchanged. No env, flag or feature seam may force a refusing peer-euid decision.
@task_item n=11 part=needs
Linux wire and log contracts stay fixed. The closed event enum, the detail code catalog, schemas/diag-line.v1.json and schemas/diag-detail.v1.json keep every Linux value, and the typed exit set 0, 1, 2, 10–14, 20 and 21 stays. Any OS words in the schemas are left for P3 to list.
@task_item n=12 part=needs
The PTY seam on Linux: process-start for the claude child, process-exit from the handle-wait thread and never on reader EOF, and exit_source kill-fallback. These are unchanged once the Windows TerminateProcess fallback leaves viola-pty.
@task_item n=13 part=needs
The run start refusal pair written through src/human.rs refuse. The .cmd / .bat refusal is Windows text and stays only where it still names live code. P3 lists it, and the fixed two-line stderr pair keeps its shape for the refusals that remain.
@task_item n=14 part=needs
The hook contract, which the removal must not move: exit 0, no body, no stderr, and the silent no-op without VIOLA_NAME. The NEVER-log floor and the zero-unlogged-panics invariant also stand as they are.
@task_item n=15 part=needs
The e2e-home rule that every integration-test viola home lives under target/e2e-home/, so G2, G4, the secret scan and the diag upload cover it. The harness free-space fix must keep the homes there and keep the 0600 / 0700 owner-only checks those two cases assert.
@task_item n=16 part=needs
The msrv job and the MSRV floor of 1.96 (D-22). The job must go green on Linux once the two harness cases stop depending on the runner's real free disk.
@task_item n=17 part=needs
The agent-readable consumption path through the harness logs and status commands, which name scripts/agent-run.sh with a .ps1 twin. Whether the PowerShell twin leaves with the Windows CI steps is P3's to read and P4's to decide. The sh harness path stays.
@task_item n=18 part=needs
The fake agent as the only CI agent: viola verify runs in CI only against the fake agent, and G2's one exemption is the fake-agent seam's forced panic at src/cmd/hook/seam.rs. Live sessions are 0 for this chunk.
@task_standing n=1
convention: each CI gate is its own run step with shell bash, copied verbatim into ci.yml. G2, G4 and the secret scan run if: always(), and a failed test step never skips them.
home: ci-integration
@task_standing n=2
convention: an artifact upload runs only after the secret-scan step passes, uses an action pinned by commit SHA and checked by zizmor, and keeps retention-days 7. A failed scan uploads the hit report instead.
home: ci-integration
@task_standing n=3
convention: every new or changed integration test keeps its viola home under target/e2e-home/, so the gates and the secret scan cover it.
home: ci-integration
@task_standing n=4
convention: tests and telemetry assertions run with retries = 0 and no retry-once policy. Every verdict is an exit-code-setting assertion, never human review.
home: slo-invariants-telemetry-budgets
@task_item n=1 part=not_needed
Derived metric coverage (viola.<area>.<measure> metrics, viola.compat.v_rejected, label cardinality). The chunk adds and changes no Linux log line that these metrics read.
@task_item n=2 part=not_needed
The error reporting platform pick, the rejected sentry and tracing-panic crates, and the four conditions on a future reporter. No reporter is added.
@task_item n=3 part=not_needed
The must-trace scenarios for confirmed send (CL-1), dialog to answer, the human taking the wheel, and the budget governor, with their span fields. No Linux path among them moves.
@task_item n=4 part=not_needed
The data classification rules, veil Redact on payload types, and the source allow-list for logged fields. The chunk logs no new field.
@task_item n=5 part=not_needed
The log levels mapping and the per-module Targets table (rmcp, portable_pty, sysinfo and interprocess OFF). No level or target changes.
@task_item n=6 part=not_needed
The telemetry signal pyramid, the OTel SDK and OTLP exporter bans, and service identity constants. Nothing in the pipeline changes.
@task_item n=7 part=not_needed
Heartbeat ticks and the liveness flip at more than 5 s. The heartbeat code is shared Linux code that stays as it is.
@task_item n=8 part=not_needed
The Section 3 alignment gaps and the Decisions Log event enum extensions (release-from-driver, state-recovered and hook-invoked corr timing). The chunk adds no event value.
@task_item n=9 part=not_needed
The panic hook's per-role behaviour (the hook's catch_unwind, the cli internal-error document, and mcp's Tokio task containment). None of it is OS-gated.
@task_item n=10 part=not_needed
Trace context propagation and the corr plus conn join key across channel calls. The removal leaves the channel wire unchanged.
@task_item n=11 part=not_needed
Operational retention and the absence of rotation for events.ndjson and diagnostics files.
@task_item n=12 part=not_needed
Edge flows E3 (link and unlink) and E5 (snapshot replay and its six product readers), and the torn-append healing line.
@task_item n=13 part=not_needed
The statusline shell-out and STATUSLINE_DEADLINE, and the capability ledger read inside run.version_gate. Neither is touched by the OS removal.
