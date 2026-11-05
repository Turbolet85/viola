@task_prose part=chunk
Makes viola Linux-only. It first sets the tag multi-os-final on the last commit that still holds the three-OS code and pushes it to origin before any removal commit. Take-up re-reads that no multi-os-* tag stands, locally or on origin.
Removes every cfg(windows) and cfg(target_os = "macos") arm with its tests and fixtures, the four Windows-only files (1 146 lines, re-read at take-up), vendor/conpty/, and the cfg(windows) target dependencies in the manifests along with their deny.toml entries. A cfg(unix) arm whose only sibling is gone loses its gate.
Removes the Windows and macOS CI jobs from ci.yml (test, lint, perf, release) and from nightly.yml, deletes windows-mutants.yml, and drops any pre-push or harness step that runs only for those OSes. The census of OS-gated tests is re-pointed or retired.
Fixes the msrv failure of ci#41930831242: the two viola-e2e e2e-home backing cases must decide on a free-space figure the test controls, never the host's. No Linux behaviour, exit code, closed word or wire contract changes.
Acceptance: every gate is green on Linux, no Windows or macOS job remains in any workflow, no cfg(windows) or target_os = "macos" remains in the tree, and the tag resolves on origin. The plan names the masters' Linux-only amendments as expected; the wrap writes them.
@task_file n=1 path=src/conpty.rs
@task_file n=2 path=crates/viola-pty/src/sideload.rs
@task_file n=3 path=tests/conpty_sideload.rs
@task_file n=4 path=tests/contract_windows_mutation_scope.rs
@task_file n=5 path=vendor/conpty/
@task_file n=6 path=Cargo.toml mark=edits
what: the [target.'cfg(windows)'] dependencies
@task_file n=7 path=deny.toml mark=edits
what: the entries of the removed Windows dependencies
@task_file n=8 path=.github/workflows/ci.yml mark=edits
what: the Windows and macOS jobs (test, lint, perf, release), the two pwsh shim steps
@task_file n=9 path=.github/workflows/nightly.yml mark=edits
what: the Windows and macOS jobs
@task_file n=10 path=.github/workflows/windows-mutants.yml
@task_file n=11 path=tests/contract_os_gated_tests.rs mark=edits
what: the census of OS-gated tests, re-pointed or retired with the gates it counted
@task_file n=12 path=scripts/agent-run.ps1
@task_file n=13 path=crates/viola-e2e/src/harness/mod.rs mark=edits
what: the cases e2e_home_backing_gone_target_is_made_again_owner_only and e2e_home_backing_owner_only_target_is_kept_untouched, and the free-space floor message built at :152
@task_item n=1 part=needs
The recovery tag multi-os-final goes on the last three-OS commit and is pushed to origin before any removal commit. Its resolving on origin is part of the acceptance.
@task_item n=2 part=needs
IPC access control and anti-impersonation. The Windows arms leave: the listener's protected DACL, the SQOS client open and the named-pipe server-pid read. The Unix arms must stay whole and act on Linux exactly as before: the per-user 0700 socket directory checked before bind and connect, chmod 0600 after bind, the server-side peer euid drop, and the client's server verification (peer pid and start time, euid, socket-dir check, and the closed refusals server-verify-failed and socket-dir-refused).
@task_item n=3 part=needs
The macOS carve-out (no peer pid or start-time comparison, server-verify-failed meaning an euid mismatch alone) and its code leave with the macOS arms. The remaining Linux client may not inherit any skipped check from it, and the frame-before-verification ban still binds every client path.
@task_item n=4 part=needs
Strict-modes checks on the viola home (owner, mode bits, no symlinked diagnostics). The Windows owner-and-DACL arm leaves, and the Unix arm, if made unconditional, must keep refusing exactly what it refused before, in every process, before the first read under the home.
@task_item n=5 part=needs
cfg(unix) arms made unconditional. This covers the statusline shell-out (/bin/sh -c, the only shell-out), the Unix-only settings override written only for a pinned path of plain characters, and the scan class for non-0600 diagnostics files. Each keeps its Linux behaviour and gains no new reach.
@task_item n=6 part=needs
Windows-only input and spawn controls leave with their arms: the registry read of persistent-environment names, the .cmd/.bat refusal with the claude shim resolution, and the DLL search restriction called as the second statement of main in every role. Whether text naming them goes is decided item by item, under the P4 rule for text naming removed code.
@task_item n=7 part=needs
The sideloaded ConPTY companions (owner/DACL check, full SHA-256 re-hash by absolute path, silent fallback to the inbox ConPTY) leave with sideload.rs and vendor/conpty/. The pinned binary's own SHA-256 re-hash and the plugin rewrite at every start stay as they are.
@task_item n=8 part=needs
The vendored ConPTY gate: scripts/conpty-vendor.sh parses its four pins from src/conpty.rs, and the windows-2025-only test step ConPTY vendor verification runs its --verify and --probe. Both lose their subject when src/conpty.rs and vendor/conpty/ go.
@task_item n=9 part=needs
Dependency audit after the manifest edits. cargo deny check must stay green over all four check families. The licence allow list stays exactly MIT, Apache-2.0, Zlib and Unicode-3.0, with 0BSD admitted only by the per-crate exceptions. The bans.features entries for rmcp and serde_json, the C-crate bans, deny-sync.toml, channel-tokio-free.sh and deny-probes.sh all stay live. The single RUSTSEC-2017-0008 ignore stays tied to the portable-pty pin.
@task_item n=10 part=needs
Pinning after the removal. Cargo.lock stays committed and every build stays --locked. [workspace.dependencies] keeps pinning every remaining third-party version (portable-pty =0.8.1). The lockfile floors stay in force: rmcp >=3.4.1 and bytes >=1.11.1.
@task_item n=11 part=needs
Workflow hardening on every edited workflow: workflow-level permissions {} and job-level contents: read, actions pinned by full SHA with a version comment, actions/checkout with persist-credentials: false, no github.event value inside run:, matrix values only through env:, and toolchains from rustup steps. zizmor .github/workflows/ on the ubuntu leg must still pass. No concurrency block is added.
@task_item n=12 part=needs
The test job's tool: line in ci.yml is the version source that viola-harness pre-push's tools stage parses (the cargo-nextest, cargo-mutants and cargo-llvm-cov pins). It must survive the matrix removal intact and readable.
@task_item n=13 part=needs
The perf job's per-OS uploads (perf-<os>, diag-perf-<os>, secret-scan-perf-<os>) and the harness-<os> upload shrink to the Linux leg. Each stays gated on its secret-scan passing, and target/agent-run/chunk.diff stays skipped and never uploaded.
@task_item n=14 part=needs
The release job, run through scripts/release-check.sh, keeps its Linux leg: its --probe reads 5/5 refused with a clean control, and it refuses any executable other than viola and any artifact built with a test-only feature (test-support, fake-agent).
@task_item n=15 part=needs
nightly.yml keeps, on Linux, the weekly cargo deny check advisories over the root lockfile and over fuzz/Cargo.lock, and the fuzz job with its upload of crash inputs on failure. The fuzz/ test-only exemption is unchanged.
@task_item n=16 part=needs
The msrv job (rustup toolchain install 1.96, selected by RUSTUP_TOOLCHAIN=1.96) must be green. Its red subject is the e2e-home free-space floor in the viola-e2e harness.
@task_item n=17 part=needs
The e2e-home keepers in crates/viola-e2e/src/harness/mod.rs: a test-side removal under target/e2e-home takes only a directory the test side created; the keepers never follow a linked base unless its target is absolute and an owner-only real directory, create the target 0700, and remove nothing; viola itself never reads or writes the backing. The fix must leave all of this intact.
@task_item n=18 part=needs
run --mutants and its Windows host scratch guard (viola-mutants-scratch, owner-only, never falling back to %TEMP% or the repository, path never printed). Its dispatch-only workflow and its contract test leave. The --package arm and the base taken from git history keep the same ban wherever that code survives.
@task_item n=19 part=needs
The unchanged Linux contracts. The typed exit codes stay 0, 1, 2, 10–14, 20 and 21. hook exits 0 with no body and no stderr, a failing security check included. Security refusals go only to automation, never to the human. The NEVER-log floor stays as it is.
@task_item n=20 part=needs
No environment variable, flag or feature seam may force a refusing peer-euid decision or switch off any control. The two fake-agent test seams stay compiled out of release builds, and no new seam or environment read is added without a Decisions Log entry.
@task_item n=21 part=needs
Product error Display impls (PtyError and the thiserror enums) keep their fixed messages. Removing ConPTY- or Windows-only variants must not let a path or upstream text into an external error, and the closed instance-unreachable causes stay as they are.
@task_item n=22 part=needs
Expected security-plan amendment, written by the wrap: the CI/CD line's three-OS matrix, the ConPTY dependency and vendor rows, and the per-OS CI job rows become Linux-only. The Windows ACL and named-pipe sections stay, as the text for an unsupported OS.
@task_item n=23 part=needs
The G2 zero-panics gate keeps exactly one exemption, the FAKE_AGENT_HOOK_PANIC seam's panic_location, and its --probe runs before every G2 check.
@task_standing n=1
convention: each CI job or step added or kept holds job-level contents: read under workflow permissions {}, uses SHA-pinned actions with a version comment and checkout with persist-credentials: false, and passes event or matrix values only through env:
home: dependency-security
@task_standing n=2
convention: each scan-gated upload in CI waits for the secret-scan step (if: always()) to pass, and no scan ever prints or writes the matched bytes
home: Bootstrap phases (secret-scanning-ci-gate)
@task_standing n=3
convention: each test runs against its own --home, and every child of the pre-push gate runs under env -i with exactly HOME and PATH, so no host value crosses into a child
home: secret-management
@task_standing n=4
convention: each test seam or environment read lives only under a test-only feature compiled out of release builds, switches off no control, and any new one needs its own Decisions Log entry
home: security-anti-patterns
@task_standing n=5
convention: each log, diagnostics or error line obeys the NEVER-log floor: no user content, tool input, absolute path or stripped CLAUDE* value outside diagnostics/, and diagnostics files are 0600
home: Bootstrap phases (logging-redaction-wire)
@task_standing n=6
convention: each fixture or test input is synthetic, with no real prompts, home paths or usernames committed
home: data-protection
@task_standing n=7
convention: each new G2 zero-panics exemption needs its own Decisions Log entry
home: Dependency Security
@task_item n=1 part=not_needed
Paste-text and dialog free-text validation (validate_paste_text, the control-character set, empty-text, decoded-char checking). The chunk changes no send or answer input path.
@task_item n=2 part=not_needed
MCP tool-input handling and the structuredContent refusal codes. No MCP handler or tool schema changes.
@task_item n=3 part=not_needed
The dialog-answering gate: ledger/stamps.json stamps from viola verify, read_stamps_strict, the unverified-cli gate and the closed behavior enum. The chunk answers no dialog and writes no stamp.
@task_item n=4 part=not_needed
viola verify's live probes and their accepted risks and residuals: Runs B to E, the scratch footer reading, the live plugin-install round, the dev-host prerequisite for the real-CLI probe, and the --record fixture scrub. The chunk runs 0 live sessions.
@task_item n=5 part=not_needed
v1.x release prerequisites: dist with attestations and signing, cargo-auditable with cargo audit bin, self_update with signatures, and the rust-cache ban in a release workflow. v1 has no release workflow.
@task_item n=6 part=not_needed
The human terminal's stdin classifier (the closed non-editing list) and bracketed-paste breakout. No PTY input path changes on Linux.
@task_item n=7 part=not_needed
The vt100 feed under catch_unwind and the poisoning of the screen model.
@task_item n=8 part=not_needed
Harness prefixes, screen signatures and local-command lists built only from compiled ledger rows.
@task_item n=9 part=not_needed
Unwrapped-session list rows: escape_row, and a claude agents --json row name never used as a ViolaName or target.
@task_item n=10 part=not_needed
revive's session-id argv built by resume_args, and the no-session refusal.
@task_item n=11 part=not_needed
The driver-originated release guard (release-from-driver), the from param never being an identity, and the accepted risk of driver prompt injection.
@task_item n=12 part=not_needed
events.ndjson retention, the accepted risk to local availability, and the omitted key management.
@task_item n=13 part=not_needed
Secret rotation cadence and the v1.x signing credentials counted as secrets. No secret enters the repo or CI.
@task_item n=14 part=not_needed
The security tier, data classification and compliance triggers of the threat-model summary. The chunk changes none of them.
