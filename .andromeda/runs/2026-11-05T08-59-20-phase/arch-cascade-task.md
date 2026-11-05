@task_prose part=chunk
Makes viola Linux-only: sets and pushes the tag multi-os-final on the last three-OS commit before any removal, after re-reading that no multi-os-* tag stands locally or on origin.
Removes every cfg(windows) and target_os = "macos" arm with its tests and fixtures, the four Windows-only files (1 146 lines, re-read), vendor/conpty/, the Windows target dependencies of the root and crate manifests with their deny.toml entries; a cfg(unix) arm whose only sibling left loses its gate.
Removes the Windows and macOS CI jobs of ci.yml and nightly.yml, the whole windows-mutants workflow and any pre-push or harness step run only for those OSes; re-points or retires the OS-gated test census.
Makes the two e2e-home backing harness cases decide on a free-space figure the test controls, never the host's, so msrv goes green; re-measures the cfg-gate pattern count and closes the msrv failure against run ci#41930831242.
Changes no Linux behaviour (exit codes, closed words, wire contracts, hook contract); the plan names the masters' OS-target amendments as expected amendments the wrap writes.
@task_file n=1 path=src/conpty.rs
@task_file n=2 path=crates/viola-pty/src/sideload.rs
@task_file n=3 path=tests/conpty_sideload.rs
@task_file n=4 path=tests/contract_windows_mutation_scope.rs
@task_file n=5 path=vendor/conpty
@task_file n=6 path=Cargo.toml mark=edits
what: the [target.'cfg(windows)'] dependencies
@task_file n=7 path=deny.toml mark=edits
what: the entries of the removed Windows target dependencies
@task_file n=8 path=.github/workflows/ci.yml mark=edits
what: the Windows and macOS jobs (test, lint, perf, release) and the two pwsh shim steps
@task_file n=9 path=.github/workflows/nightly.yml mark=edits
what: the Windows and macOS jobs
@task_file n=10 path=.github/workflows/windows-mutants.yml
@task_file n=11 path=tests/contract_os_gated_tests.rs mark=edits
what: the census of OS-gated tests, re-pointed or retired with the gates it counted
@task_file n=12 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the cases e2e_home_backing_gone_target_is_made_again_owner_only and e2e_home_backing_owner_only_target_is_kept_untouched, and the free-space floor message they panic with
@task_file n=13 path=scripts/agent-run.ps1
@task_item n=1 part=needs
The PTY decision: the sideloaded ConPTY failing open to the inbox one, the System32 DLL-search restriction and the Windows garbage-read rationale for the portable-pty pin all go with the removed Windows arms; the pin and the child-exit-on-handle rule stay for Linux.
@task_item n=2 part=needs
The Message Broker / IPC decision and the IPC endpoints rows: the Windows named pipe, the SQOS Identification open and the test-only SQOS pipe of tests/channel_sqos_open.rs leave; the Linux socket rules (per-user 0700 socket dir, socket-dir-refused, 0600 socket, owner-euid peer check, no abstract names) stay as they are.
@task_item n=3 part=needs
The filesystem rows of the ConPTY companions: bin/<version>-<hash>/conpty/ written by pin_companions, the sideload_fallback words, target/conpty-seed and seed_conpty in tests/support/home.rs, named as a carve-out beside the viola-chaos home; they leave with the code.
@task_item n=4 part=needs
The repository rows for vendor/conpty, scripts/conpty-vendor.sh (its pins parsed from src/conpty.rs) and the .gitattributes binary rule for vendor/conpty; they lose their subject with the removal.
@task_item n=5 part=needs
The Concurrency KEYSTONE's CI enforcement (the sole-root cargo deny tokio ban and the cargo check of the sync crates, run today on three OSes) must stay green and running on the Linux CI after the matrix shrinks.
@task_item n=6 part=needs
The CI/CD decision and the stack and inherited-defaults rows naming GitHub Actions on windows-2025 / macos-latest / ubuntu-latest: the Windows PATHEXT resolution and the npm-shim claude.exe resolution are Windows arms; the fake-agent-only CI and the per-test viola home stay.
@task_item n=7 part=needs
The typed exit-code set (0, 1, 2, 10–14, 20, 21) and viola hook's always-exit-0, no-body, no-stderr contract must come out of the removal unchanged on Linux.
@task_item n=8 part=needs
The instance snapshot's pid and started_at server check, made on Linux and Windows with macOS's missing peer pid as the recorded gap: the Linux check stays, the macOS carve-out loses its code.
@task_item n=9 part=needs
The local endpoint trust boundary: no env, flag or feature seam may force a refusing peer-euid decision while the Unix gates are made unconditional.
@task_item n=10 part=needs
The strict-modes read and write rules of the home tree, the instance files and diagnostics/, each with a Unix half (no group or other bit) and a Windows half (DACL, another SID); the Windows halves leave and the Unix halves lose their gate.
@task_item n=11 part=needs
hook statusline's /bin/sh shell-out (none on Windows) and the per-session settings override written only on Unix: their Unix gates become unconditional with no change in Linux behaviour.
@task_item n=12 part=needs
The R8 child-environment strip: the Windows registry reader of persistent names is a Windows arm, claude_env_keep the Unix pass-list; the identity floor and the names-only rule stay.
@task_item n=13 part=needs
viola-channel's test-support feature, whose Windows helpers canonical_sddl, dacl_of and user_sid leave, while its enabling only through the root dev-dependency and scripts/release-check.sh's refusal of it stay.
@task_item n=14 part=needs
scripts/orphans-check.sh and its probes (a file gated on a target no CI triple compiles, the cfg(windows)/cfg(unix) file-pair control): with CI on Linux alone, any Windows-gated file left behind trips it, and its control must still fire.
@task_item n=15 part=needs
The Windows mutation path: the harness's run --mutants arm with the viola-mutants-scratch guard (inert off Windows) and target/mutants, reached by removing windows-mutants.yml and tests/contract_windows_mutation_scope.rs.
@task_item n=16 part=needs
The harness's per-OS upload names (harness-<os>, junit-<os>, secret-scan-<os>) and target/harness's Windows relink rationale, which the shrunk CI matrix reaches.
@task_item n=17 part=needs
The test-side e2e home: target/e2e-home with its two test-side keepers holding one contract, and the dev host's tmpfs /tmp/viola-e2e-home-<uid>, which the backing cases' free-space floor guards.
@task_item n=18 part=needs
The bounded-readers census of tests/contract_bounded_readers.rs over the product sources, whose enumerated boundaries must follow the removed files (the companions' hashing reads among them).
@task_item n=19 part=needs
The Error Handling census of nine thiserror derives counted by grep over src and crates, and PtyError's hand-written exception: a removed file that held a derive changes the count.
@task_item n=20 part=needs
The workspace crates and their dependency direction: the crates' manifests lose their Windows target dependencies, with the sync-crate list and the crate graph otherwise unchanged.
@task_item n=21 part=needs
The masters' OS-target text the plan names as expected amendments: Design Philosophy's cross-platform-Windows-first truth, the Personal v1 cross-platform decision, Cross-platform discipline, the Scale path, and every per-OS row, including the OPEN item and the dialog-hold leg owned by the retired entry Windows-only live measurements.
@task_item n=22 part=needs
The closed words and refusals that name Windows without a cfg gate (batch-script-child for .cmd/.bat children, conpty-sideload, conpty, sideload_fallback, SERVER_PID_REPORTED, the Git Bash rewritten-path warning, viola.exe naming): the plan lists each one the removal leaves naming code that no longer exists.
@task_standing n=1
convention: each test runs in its own viola home (--home), so parallel tests never share an endpoint, a log or budget.json, and runs against the fake agent, never the real claude in CI
home: Established Decisions
@task_standing n=2
convention: each test home sits under target/e2e-home and a new test home removes only the dirs whose recorded owner process is gone, never by age or name
home: occupied-resources
@task_standing n=3
convention: each test-only evidence report outside the viola home is kept when the test panics or is killed and removed when it passes
home: occupied-resources
@task_standing n=4
convention: each test-harness-only environment variable uses the AGENT_RUN_ prefix and is never read by the viola binary
home: Conventions
@task_standing n=5
convention: each committed fixture is synthetic, scrubbed and hygiene-walked, a fake-agent script validated against schemas/fake-script.v1.json
home: occupied-resources
@task_standing n=6
convention: each log line is one complete JSON object per single write, codes only in the home-level role files and content only in the instance detail files, fixed by the obs-owned diag schemas
home: Cross-cutting Patterns
@task_item n=1 part=not_needed
The channel methods' params and ok payloads and the event data per kind: no wire contract changes and the chunk adds no method or kind.
@task_item n=2 part=not_needed
The Budget Governor, its thresholds and the budget.json reading: no OS arm of it is named.
@task_item n=3 part=not_needed
The Links decision, the link set and its replay.
@task_item n=4 part=not_needed
The Human Takeover / Wheel decision: the human-wins invariant stands untouched.
@task_item n=5 part=not_needed
The Screen Model readiness gate and Delivery Confirmation, including send's CR/LF typing rule.
@task_item n=6 part=not_needed
The MCP decision, its tools and the MCP result shapes.
@task_item n=7 part=not_needed
The capability ledger, the ledger stamps envelope, CLI Version Compatibility and viola verify's probes.
@task_item n=8 part=not_needed
Config management and the config.json schema and keys.
@task_item n=9 part=not_needed
The list payload, session liveness's heartbeat rules and the claude agents --json join.
@task_item n=10 part=not_needed
Timestamps, validation regimes, protocol versioning and the JSON naming patterns.
@task_item n=11 part=not_needed
The deferred frontend and the GUI contracts, deferred to 0.2.0.
@task_item n=12 part=not_needed
The fuzz workspace's targets, corpus and nightly toolchain.
@task_item n=13 part=not_needed
The Plugin Scope, the marketplace written by viola plugin install and the registered hook events.
@task_item n=14 part=not_needed
The Snapshot writer and the ndjson state store's lock and append rules.
