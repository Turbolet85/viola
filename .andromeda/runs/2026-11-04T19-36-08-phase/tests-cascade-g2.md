@carried n=1 label=test-plan/test-strategy fact=fct-7osf2h3d
why: Every unit case must run on all three OSes except the named macOS-gated cases, each gate keeping its stated reason (macOS reads no peer pid), which is the per-OS gating the chunk reads and must hold.
quote: Runs on all 3 OSes, except the two real-cargo-mutants harness tests (§10 Mutation gate) and the server check's captured-span cases, compiled out on macOS, where the server check reads no peer pid: each frame verb's `request_refuses_an_unverified_server_inside_its_dispatch_span`, `hook_refuses_an_unverified_server_inside_hook_handle`, the test-only `cmd::client::refused_inside` helper they share (under `#[cfg(test)]` + `#[cfg(not(target_os = "macos"))]`) and the `last.rs`, `pause.rs` and `unlink.rs` test modules, gated whole (a module is gated whole only when its every case is gated: `src/cmd/link.rs` gates its server-check case alone, and its reply-shape and line cases run on macOS); the decision table `server_verdict_rows` and the macOS `server_pid` case still run there, and so do the socket-directory check's captured-span cases `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` and `hook_refuses_a_loosened_socket_directory_inside_hook_handle`, which read the span recorder `Seen` from the test module `refused_seen` in `src/cmd/client.rs`, outside the macOS-gated `refused_inside`.
@carried n=1 run=2 label=test-plan fact=fct-njccdctv
quote: on all three OSes except the two real-cargo-mutants harness tests and the server check's captured-span cases, compiled out on macOS.
@carried n=2 label=test-plan/test-strategy fact=fct-pe3lhjgn
why: The fake-agent E2E suite, through the outer PTY and the rmcp client, must be the same suite on ubuntu-latest and macos-latest as on windows-2025.
quote: Required for all 7 critical paths plus 4 expansion scenarios (Section 6). They run against harness-booted `viola run … -- viola-fake-agent`, using the outer PTY (portable-pty) and the rmcp client.
@carried n=2 run=2 label=test-plan fact=fct-q3ukwekc
quote: required for all seven critical paths and the expansion scenarios, against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
@carried n=3 label=test-plan/test-strategy fact=fct-zx23mmpw
why: The per-OS perf job the chunk re-reads must gate the hook rows on `max` green on ubuntu-latest and macos-latest, and the v1-40 witness's staleness rests on the >5 s flip held by the pure `classify`.
quote: hyperfine 1.20.0 gates the hook deadlines on `max` (the `--perf` arm, the five hyperfine rows — `pre-tool-use` included, on an unstamped perf session — and the per-OS `perf` CI job are built; §10). The heartbeat staleness flip at >5 s is checked by `viola_state::liveness::classify`, a pure function fed the beat's age (no clock).
@carried n=3 run=2 label=test-plan/test-strategy fact=fct-b6yqyazs
quote: | performance-budget | perf | §10 Performance budgets |
@carried n=3 run=3 label=test-plan fact=fct-egpbbwom
quote: hyperfine gates the hook deadlines on `max`; the heartbeat staleness flip is checked by a pure function fed the beat's age, with no clock.
@carried n=4 label=test-plan/test-strategy fact=fct-zinazvq2
why: The chaos cases the Unix suite runs (a no-EOF child read on the handle over openpty, a SIGSTOP heartbeat freeze, the `viola-pty` seam faults) must draw their faults only from these process, file and mockall sources.
quote: Faults come from sysinfo 0.39.6 / `std` process and file operations (`File::set_len` truncation of a stopped wrapper's log, the torn append as landed in place of a kill mid-write; snapshot corruption, SIGSTOP heartbeat freeze, a no-EOF child), plus mockall 0.15.0 faults on the `viola-pty` seam.
@carried n=4 run=2 label=test-plan/test-strategy fact=fct-x65537y4
quote: | chaos-test | chaos | §6 Chaos suite |
@carried n=4 run=3 label=test-plan fact=fct-2yi4u2mc
quote: faults come from process and file operations and from mockall faults on the `viola-pty` seam.
@carried n=5 label=test-plan/test-strategy fact=fct-cz5jn5fs
why: A timed-out mutant fails a mutation run through nextest, so the v1-40 integration witness that waits out the 5 s stale threshold beside the ~4 s stamp must stay inside the mutants profile's 10 s kill.
quote: at the epoch boundary: `/andromeda-code-audit` runs cargo-mutants 27.1.0 per unit, `run --mutants` [`--file`] runs it `--in-diff`, and `run --mutants --package <member>` runs it over one whole member (the boundary-tier form), both through `--test-tool=nextest` on demand (named only, never in the default or `--all`). No per-chunk, pre-push or CI mutation gate. Any `missed` outcome the harness does not leave out as host-excluded (§3 `run` step 4, Verdict), any `timeout` outcome, or more `unviable` than `caught` outcomes (`unviable-exceeds-caught`), makes that `run` fail.
@carried n=5 run=2 label=test-plan/test-strategy fact=fct-2aejh4x2
quote: | mutation testing | mutation | §3 `run` step 4; §10 |
@carried n=5 run=3 label=test-plan fact=fct-cv73iuuc
quote: a missed mutant the harness does not leave out as host-excluded, a timed-out mutant, or more unviable than caught, fails the run.
@carried n=6 label=test-plan/test-strategy fact=fct-vekwchud
why: The Unix-only statusline replay is a one-OS contract test whose gate must name its reason, and the contract suite is what keeps the fake agent, the chunk's only child, from drifting, so no fake-agent green is claimed as real-CLI proof.
quote: (`fake_agent_statusline_replay_matches_every_recorded_statusline_set`, Unix; the synthetic `statusline_drift_names_a_changed_byte_a_swapped_order_and_a_short_replay`)
@carried n=6 run=2 label=test-plan/test-strategy fact=fct-ffgiydwd
quote: | contract-test-against-sandbox | contract | §6 Contract suite; §7 |
@carried n=6 run=3 label=test-plan fact=fct-xagti75h
quote: the fake agent's hook stdin is compared byte for byte with the recorded `viola verify` fixtures across the spine, dialog and framing tiers, never as snapshots
@carried n=6 run=4 label=test-plan fact=fct-egysilpo
quote: NEVER treat a green fake-agent run as proof of real-CLI behaviour.
@carried n=7 label=test-plan/test-strategy fact=fct-34amt7nu
why: The server-identity negatives must stay off macOS and the socket-directory negatives on Unix, with the Windows listener cases unmoved, each one-OS gate carrying this reason.
quote: root `security_negatives_channel.rs`: the Windows listener's remote-client rejection and DACL denial; root `security_negatives_server_identity.rs`, off macOS: a CLI frame verb against a server that is not the recorded wrapper; root `security_negatives_socket_dir.rs`, Unix: a socket directory at 0755 or a symlink to 0700 refusing a start, the eight CLI frame verbs and the hook, each left as found; root `channel_links.rs`: raw `link` / `unlink` frames with a bad name or pair, each `-32602` with no line written) + Security sweep (the hook fail-open impostor rows, off macOS)
@carried n=8 label=test-plan/test-strategy fact=fct-yhevpdmp
why: Hook exit 0 with no body and no stderr must hold on every OS, re-asserted by the Unix-run security sweep this row names.
quote: | Hook stdin (V4) | integration (assert_cmd hook) | §5 hook boundary; §6 Security sweep |
@carried n=9 label=test-plan/test-strategy fact=fct-mqdaf5bf
why: The filesystem negatives split by OS (the strict-modes frame verbs on three OSes, a Unix umask case, a Windows DACL case) must keep their OS gates with reasons, the Windows case unmoved.
quote: root `security_negatives_strict_modes.rs`: each of the six frame verbs `send` · `wait` · `last` · `answer` · `pause` · `release` refusing a widened instance before any log, exit 21 with the instance pair or the `instance-unreachable` document, its `detail` `strict-modes-failed`, no frame and no line; a home another user may read refusing `run`; a start under umask 000 making every home path owner-only (Unix); a Windows `--home` outside the profile taking the protected DACL, read back through `icacls /save`;
@carried n=10 label=test-plan/test-strategy fact=fct-svqwosl6
why: The child-spawn boundary must be exercised over the openpty spawn on Unix, and Scenario E2's `claude_env_keep` survival is a Unix-only reading.
quote: | Child spawn (V8) | integration + E2E | §5 process boundary; §6 Scenario E2 |
@carried n=11 label=test-plan/test-strategy fact=fct-peucakqx
why: The lint stage's sync-crate check and orphans gate are steps of the per-OS lint job the chunk re-reads green on ubuntu-latest and macos-latest.
quote: §9 Lint stage (sync-crate `cargo check`, `scripts/orphans-check.sh`)
@carried n=12 label=test-plan/test-strategy fact=fct-gldxhydz
why: Parity must be read on the CI matrix itself, every macOS reading from macos-latest, never from a Linux-only local run.
quote: | multi-os-compat | CI matrix | §9 Matrix builds |
@carried n=12 run=2 label=test-plan fact=fct-lktzdsoq
quote: NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
@carried n=13 label=test-plan/test-strategy fact=fct-jbbspgvm
why: Every `--json` error document must hold no path or upstream text on every OS, the loosened-socket-directory witness being a Unix case the shared suite runs.
quote: root `cli_error_bodies.rs` `cli_error_documents_hold_no_path_or_upstream_text` beside its sibling `cli_error_documents_of_a_loosened_socket_directory_hold_no_path_or_upstream_text`
@carried n=14 label=test-plan/test-strategy fact=fct-brzfiy4i
why: The v1-40 witness must read SGR dim from `viola list` output under the outer PTY and the forced vt100 panic case by byte search, never by parsing the rendered child screen.
quote: | discipline (no screen parsing / no blanket approval / no flakes) | all | §6 Scenario 4 negative; §11 |
@carried n=14 run=2 label=test-plan fact=fct-nh5slhtz
quote: NEVER parse the rendered child screen (the vt100 grid, PTY master output) for content.
@binding n=1
**test-strategy multi-os-compat ↔ ci-integration Matrix builds and §11 Test Strategy**: the full workspace and fake-agent E2E must run on `windows-2025`, `macos-latest` and `ubuntu-latest` with `fail-fast: false`, each OS branch verified only on its own runner (test-plan/test-strategy fct-gldxhydz) (test-plan fct-shyrfs6v) (test-plan fct-5akis2u7) (test-plan fct-lktzdsoq)
@binding n=2
**test-strategy Unit macOS gating and IPC V1 ↔ e2e-test-strategy exit-cause matrix**: the server check's captured-span cases and `tests/security_negatives_server_identity.rs` must compile out on macOS while `server_verdict_rows` still runs there, no test reading the macOS check as a comparison (test-plan/test-strategy fct-7osf2h3d) (test-plan/test-strategy fct-34amt7nu) (test-plan fct-ndx7lm2p)
@binding n=3
**test-strategy Performance ↔ quality-gates perf budgets and ci-integration Perf**: the per-OS `perf` job must read green on Unix with each hook row gated on `max` below `viola_core::SPINE_DEADLINE` (test-plan/test-strategy fct-zx23mmpw) (test-plan fct-ggkcj6j7) (test-plan fct-3q2uf724)
@binding n=4
**test-strategy Mutation ↔ quality-gates heartbeat flip row and §11 Universal real time**: the v1-40 integration witness must stay inside the mutants profile's kill bound, its verdict the product's own liveness field, the same 5 s deadline clock-driven in the `classify` unit case (test-plan/test-strategy fct-cz5jn5fs) (test-plan fct-4tlnkrfv) (test-plan fct-45jodiii)
@binding n=5
**test-strategy Chaos ↔ unit-test-strategy viola-pty and e2e-test-strategy Chaos suite**: on openpty a no-EOF child's exit must be read on the handle and a SIGSTOPped wrapper read `stale`, on both Unix runners (test-plan/test-strategy fct-zinazvq2) (test-plan fct-sbml5vi6) (test-plan fct-47fi4ugg)
@binding n=6
**test-strategy Contract ↔ test-data-fixtures fake agent**: the fake agent's Unix process groups and SIGHUP-ignoring exit decide what a killed wrapper's hangup reaches in the Unix child-ends-with-its-wrapper reading, and the Unix statusline replay stays gated with its reason (test-plan/test-strategy fct-vekwchud) (test-plan fct-7o4nbra2)
@binding n=7
**test-strategy discipline ↔ e2e-test-strategy cli driver**: TTY rendering of the v1-40 witness must be asserted only on the SGR sequences of verb output under the outer PTY (test-plan/test-strategy fct-brzfiy4i) (test-plan fct-gnzxb6ex)
@binding n=8
**test-strategy Filesystem V7 ↔ ci-integration Matrix builds**: the Windows DACL read-back cases stay on `windows-2025` unmoved while the Unix umask and modes cases run on both Unix runners (test-plan/test-strategy fct-mqdaf5bf) (test-plan fct-shyrfs6v)
