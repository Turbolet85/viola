@history_item n=1 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — (pinned actions and toolchain source) set the SHA-pinned action set with `actions/checkout` `persist-credentials: false`, replaced `dtolnay/rust-toolchain` with `rustup toolchain install` from `rust-toolchain.toml`, fixed the 1.98.1 pin and the 1.96 `rust-version` floor, and routed event-payload values only through `env:`. These are the workflow hardening rules every job left in `ci.yml` and `nightly.yml` must keep when the Windows and macOS jobs leave, and they are what the `msrv` job on 1.96 rests on.

@history_item n=2 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — (trust boundary, CI jobs, deny.toml additions, nightly.yml) put the sole-root `deny-sync.toml` tokio ban per sync crate into the trust boundary, added the ubuntu supply-chain job and the separate weekly `nightly.yml`, and recorded that `scripts/deny-probes.sh` proves every ban live. This chunk edits `deny.toml`, may drop a non-Linux target triple from `deny-sync.toml`, and cuts `nightly.yml`'s Windows and macOS jobs, so the ban and its probe have to stay live.

@history_item n=3 marker=2026-09-24-observability-gates
**2026-09-24-observability-gates** — (CI runs the obs artifact canary scan before uploads) recorded that CI runs `viola-harness secret-scan` (`id: secret-scan`) before every test-home upload. When the matrix shrinks to Linux, each upload that remains must still wait for that scan.

@history_item n=4 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — (fuzz lockfile exemption, toolchains, unscanned uploads, declined concurrency) made `fuzz/` a test-only exemption that holds only while it is never linked and never joins the root `[workspace]`, named the three rustup toolchains (1.98.1, `nightly-2026-09-20`, MSRV 1.96 through `RUSTUP_TOOLCHAIN`), added the nightly fuzz job, declined zizmor's `concurrency-limits`, and admitted the nightly `fuzz/artifacts/` as an unscanned upload. Removing jobs from `nightly.yml` must keep the fuzz job, its exemption's conditions, and the declined-concurrency stance (no `concurrency:` block added).

@history_item n=5 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — (fuzz lockfile audit wired, release-check in the CI job list) wired the supply-chain step `Fuzz lockfile audit (advisories, sources)` and the nightly `cargo deny --manifest-path fuzz/Cargo.toml check advisories`, and added the per-OS release build through `scripts/release-check.sh`. Both fuzz audits and the release check must survive on the Linux leg after the per-OS jobs shrink.

@history_item n=6 marker=2026-09-25-security-prerequisites
**2026-09-25-security-prerequisites** — (SQOS spike passed, 0BSD per-crate exceptions) recorded the Windows SQOS identification-level client open, with the default connect banned, and admitted `0BSD` only through per-crate `[[licenses.exceptions]]` for `doctest-file` and `recvmsg` (pulled in by interprocess 2.4.4), never through `allow`, each further exception needing its own entry. The SQOS open leaves with the `cfg(windows)` arm. The two 0BSD exceptions stay, and each must be re-read against the post-removal `Cargo.lock`: an exception whose crate is gone from the graph would fail `cargo deny check`, and none may move into `allow`.

@history_item n=7 marker=2026-09-25-pty-wrapper-on-windows
**2026-09-25-pty-wrapper-on-windows** — (R8 persistent-environment exemption, registry name reader, child resolution as built) added the Windows registry reader of persistent-environment names, the PATHEXT-only search, the `claude.cmd`/`.bat` → `claude.exe` sibling resolution, and the closed batch-script refusal, beside the Unix `config.json` `claude_env_keep` rule. All of these Windows arms leave in this chunk. The Linux R8 rule (exact-match strip, `claude_env_keep` as the persistent set, the identity floor stripped whatever that set holds) must stay unchanged. The entry also kept the Threat Model Summary as a verbatim copy that is not edited in place.

@history_item n=8 marker=2026-09-26-ci-chunk-base-and-union-verdict
**2026-09-26-ci-chunk-base-and-union-verdict** — (chunk.diff out of the canary scan and the harness upload; ci.yml reads no event value) made the scan skip `target/agent-run/chunk.diff` by exact path, excluded that file from the `harness-<os>` upload, and brought `ci.yml`'s `github.event` reads to 0. The Linux `harness-` upload must keep the exclusion, and the edited workflows must read no event value.

@history_item n=9 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — (`FAKE_AGENT_PUMP_DELAY_MS` carve-out; pre-push `env -i`) admitted the `fake-agent`-only pump-delay seam, compiled out of release builds, and added the Universal ban on any other seam without its own Decisions Log entry. It also set pre-push's `env -i` to HOME and PATH only. Making `cfg(unix)` arms unconditional must add no seam or environment read under this ban.

@history_item n=10 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — (listener hardening as landed: SDDL through windows-sys, chmod 0600 after the bind) built the Windows protected DACL through windows-sys into `security_descriptor`, and on Unix replaced `ListenerOptionsExt::mode(0o600)` with a chmod 0600 right after the bind on every Unix OS. The anti-pattern now bans `mode` and forbids treating 0600 as the only control. The SDDL arm and its windows-sys use leave with this chunk. The chmod-after-bind and the `mode` ban are the Unix controls that must survive when the gate is lifted, which the acceptance grep for `ListenerOptionsExt::mode` checks.

@history_item n=11 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — (test-only feature proven absent, host scratch guard) recorded `release-check`'s refusal of `test-support` and `fake-agent` artifacts, proven by a `5/5` probe, and the host mutation-scratch guard. The Linux `release` job must keep `scripts/release-check.sh --probe` at 5/5 refused.

@history_item n=12 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (test seam `FAKE_AGENT_HOOK_PANIC` and G2's exact-path exemption; the per-OS `perf` job and its scan-gated uploads) ratified the second test seam and G2's single whole-string exemption `src/cmd/hook/seam.rs:<digits>`, with `--probe` run before every check. It also added the per-OS `perf` job: `perf-<os>` and `diag-perf-<os>` upload only on a passed scan, and `secret-scan-perf-<os>` only on a failed one. The Linux `test` and `perf` legs must keep G2, its probe, and this upload gating once the other legs leave.

@history_item n=13 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — (leg-verdict upload and download-artifact pin retire) cut the pinned action set to four (checkout, rust-cache, install-action, upload-artifact) and left the nightly `fuzz/artifacts/` as the only unscanned upload. This sets a precedent for retiring CI pieces as a narrowing. After the workflows are edited, the pinned set and the single unscanned upload must stay as they are.

@history_item n=14 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — (vendored Microsoft ConPTY binaries and the third interim gap) committed `conpty.dll` and `OpenConsole.exe` under `vendor/conpty/`, gave them their own audit instead of a `cargo deny` exemption, and made the System32 DLL-search restriction the control against planting. All of this leaves together in this chunk.

@history_item n=15 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — (the DLL search order and the companions' integrity controls) made `restrict_dll_search()` the second statement of `main` in every process, added the write-if-absent companions under `bin/<version>-<hash>/conpty/` with a held-handle SHA-256 re-hash, and added the anti-patterns on companion loading. Once `sideload.rs` and `src/conpty.rs` leave, no Linux role may still call into the sideload code. The companion anti-pattern text becomes the wrap's unsupported-OS amendment.

@history_item n=16 marker=2026-09-29-sideloaded-conpty
**2026-09-29-sideloaded-conpty** — (the vendored binaries' own audit, pins and CI step) added `scripts/conpty-vendor.sh` (`--verify`, and `--probe` at 4/4 refused), the four pins with one textual home in `src/conpty.rs`, the committed binaries, and the `windows-2025` `ConPTY vendor verification` step. This is the rule that `vendor/conpty/`, its pin home and its gate leave together, never one without the others, and the `.gitattributes` `vendor/conpty/** binary` rule goes with them.

@history_item n=17 marker=2026-09-29-t15-07-57-wrap
**2026-09-29-t15-07-57-wrap** — (the security-plan Decisions Log leaves the body) moved into the body the rules that `release-check` refuses any non-`viola` executable and any `test-support` / `fake-agent` artifact (`--probe` 5/5), G2's one exact-path exemption with its look-alike `--probe`, and the `run --mutants` scratch ban. These are the release and G2 gates the Linux leg must keep.

@history_item n=18 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — (the native pre-push crossing; the WSL launchers retired) made `pre-push` Linux-only, running under `/usr/bin/env -i` with HOME and PATH only, and made its `tools` stage check the host against `ci.yml`'s `test`-job `tool:` line. It also set the scratch guard to run only on a Windows host (`HOST_SCRATCH = cfg!(windows)`), with the Linux NOCOW `TMPDIR` never wiped. It is a precedent for retiring Windows tooling as a narrowing. When the `cfg!(windows)` guard leaves, Linux must still touch no scratch, and the `tool:` line must survive the `ci.yml` edit.

@history_item n=19 marker=2026-10-04-windows-boundary-mutation-workflow
**2026-10-04-windows-boundary-mutation-workflow** — (the third workflow under CI integration) added `windows-mutants.yml`: dispatch-only, no `inputs:`, workflow `permissions: {}`, job `contents: read`, SHA-pinned actions, no upload, and never a gate or a `ci.yml` dependency. It also raised zizmor's `concurrency-limits` count to 3 low findings, one per workflow. This chunk deletes the workflow whole, so every rule over it retires with it, and zizmor's per-workflow count must be re-read, not carried over.

@history_item n=20 marker=2026-10-10-windows-mutation-grade
**2026-10-10-windows-mutation-grade** — (the Windows mutation workflow's nine-item matrix, the jobs sentence) recorded the `windows-mutants.yml` matrix count and rewrote the Threat Model's Jobs sentence: the per-OS `perf` job is in, and no `ci.yml` job has run mutation since 2026-09-28. Both texts retire or turn Linux-only with the workflow, through the wrap's amendment.

@history_item n=21 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — (verify's dirs removed on a Ctrl-C / TERM stop, the matrix count) added the Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT` arm of `cmd::verify::stop` beside Unix `SIGINT` / `SIGTERM`, and raised `windows-mutants.yml` to ten items. The matrix change was forced by the contract test that every `cfg(windows)` product file is scored on Windows. That test is `tests/contract_windows_mutation_scope.rs`, one of the four files this chunk removes, so it leaves with the workflow. The Unix stop arm stays.

@history_item n=22 marker=2026-10-13-epoch-4-cleanup
**2026-10-13-epoch-4-cleanup** — (the wiped mutation scratch re-created owner-only) re-creates the scratch through `create_private_dir`, which applies the protected user + SYSTEM DACL on Windows, and forbids loosening the strict-modes check for it. The scratch's Windows-only wipe path leaves with `HOST_SCRATCH`.

@history_item n=23 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — (the statusline source and the shell-out) made `hook statusline` the only shell-out: Unix only, `/bin/sh -c` with the command as one argument, the instance check first, bounded output. The Windows shell-out was left owed to "Windows-only live measurements". This chunk lifts the Unix gate. The shell-out must keep this exact shape and order and stay the only shell-out, and the owner line moves off the retired route entry.

@history_item n=24 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — (the settings override as landed, the hook rows, the arm's check before its log) made the `settings.json` override Unix only, written only for a pinned path of ASCII letters, digits and `_ - . / :` because a shell runs it unquoted, and cut the Windows half on the founder's word, owed to "Windows-only live measurements". When the gate goes, the plain-character guard must stay as the condition.

@history_item n=25 marker=2026-10-13-owed-ledger-rows
**2026-10-13-owed-ledger-rows** — (Run B's probe --settings, the statusline capture arm, the relied set) gave Run B one `--settings` on Unix only, and only for shell-plain paths (Windows passes none), and split the stamp's relied set per OS (`/21` Unix, `/18` Windows). This is the source of the Unix-only Run B arm and the per-OS relied set that this chunk collapses to Linux.

@history_item n=26 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — (Run E; a third session-id source) moved the per-OS relied-set counters to `/22` Unix and `/19` Windows. This is part of the relied set's history before its Windows arm leaves.

@history_item n=27 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (the claude agents --json child) moved the relied-set counters to `/23` Unix and `/20` Windows. This is part of the relied set's history before its Windows arm leaves.

@history_item n=28 marker=2026-10-18-compact-post-condition-ledger-row
**2026-10-18-compact-post-condition-ledger-row** — (Run B's sixth paste and the twenty-four-row stamp) set the current relied set at `/24` Unix and `/21` Windows (`17 pass  4 fail` Windows). When the Windows arm leaves, `/24` must remain the only gate, and a stamp missing a relied row must still read `cli_verified:false`.

@history_item n=29 marker=2026-10-16T14-58-10-wrap
**2026-10-16T14-58-10-wrap** — (the loopback GUI leaves 0.1.0) is the precedent for removing a whole surface: the masters stay true to the tree, and a `deny.toml` ban and scan forms stand while the code or file still holds them. It applies to how this chunk's `deny.toml` and scan edits, and the wrap's expected amendments, track what is actually removed.

@history_item n=30 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — (the npm graph, the Node pin, the axum ban and the GUI scan forms leave) is the precedent for removing a supply-chain leg as a narrowing: a ban left together with its `deny-probes.sh` row, and every remaining ban stays proven live. It also set the current audit-tool list (cargo-deny, zizmor, the deferred cargo-audit, and the ConPTY vendor audit), toolchains from `rustup` alone, `[[bans.features]]` holding rmcp, and the canary classes including the Unix-only non-`0600` diagnostics class. This chunk removes the ConPTY audit from that list, and the deny-probes count must be re-read.

@history_item n=31 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — (terminal colour facts join the admitted env set) admitted `NO_COLOR`, `TERM` and `COLORTERM` as compare-only facts, alongside the failed Windows VT enable (`SetConsoleMode(mode | 0x0004)`), and made them the only carve-out beside the two test seams. The VT arm leaves with this chunk. The colour facts stay compare-only and reach no log, error or refusal.

@history_item n=32 marker=2026-10-17-cli-machine-contract
**2026-10-17-cli-machine-contract** — (CLI home resolution as landed) built `canonical_home`, which on Windows strips only the verbatim-drive prefix. That Windows arm leaves with this chunk. On Linux, home canonicalisation must stay unchanged.

@history_item n=33 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (the tokio ban's tree form and the rmcp release-graph gate) moved `viola-channel` off `scripts/sync-crates.txt` onto `scripts/channel-tokio-free.sh`, proven by `deny-probes.sh`'s `channel-tokio-edge` probe. It also added the per-OS lint step `rmcp features` (`scripts/rmcp-features.sh`) and the supply-chain tree-form step. The lint job loses its Windows and macOS legs, and `rmcp-features.sh` must keep running on the Linux lint leg.

@history_item n=34 marker=2026-10-25-windows-endpoint-admission
**2026-10-25-windows-endpoint-admission** — (the Windows pipe's admission controls gain their CI witnesses) added `security_negative_viola_listener_rejects_remote_clients`, `..._denies_an_anonymous_open` and their controls in `tests/security_negatives_channel.rs` (Windows only, windows-sys 0.61.2) on `test (windows-2025)`. These witnesses leave with the Windows DACL arm and with windows-sys, along with its `deny.toml` entries.

@history_item n=35 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** — (viola-channel's Tokio client pin; the RUSTSEC-2026-0386 red closed) restated the `deny-sync.toml` tokio ban "for three target triples" over the four crates of `scripts/sync-crates.txt`, at `13/13`, closed the advisory without widening any ignore, and kept RUSTSEC-2017-0008 as the only ignore. Dropping a non-Linux triple is this chunk's change to make. The single ignore must stay.

@history_item n=36 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (the client's server-pid read; the observed server pid and the verdict) built the peer-pid read: Linux `SO_PEERCRED`, Windows `GetNamedPipeServerProcessId` on the SQOS end, none on macOS, with `SERVER_PID_REPORTED` false on macOS and `ServerVerdict` giving `Matches` with no comparison when no pid is reported. It also put the Windows exiting-pipe CARRY on "Windows-only live measurements". The Windows and macOS arms of this read leave in this chunk. The Linux path must keep comparing pid and start time and inherit no skipped branch. The retired-owner CARRY is one of the masters re-pointed by the wrap.

@history_item n=37 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (the tests holding the link verbs' controls) recorded `link_verbs_refuse_a_widened_instance_before_any_log` as Unix only, and the server-identity table's `link` / `unlink` rows as run on Linux and Windows and compiled out on macOS. These OS gates must be removed or re-pointed so that every Linux control keeps its test.

@history_item n=38 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`check_instance`'s set and the read rule; the Windows legs measured) built the read rule on both OSes (Windows `READ_RIGHTS`, `windows_read_verdict`), with `events.ndjson` under the write rule on Unix and the read rule on Windows. It measured the Windows protected DACL and the unstattable-home divergence, and named `tests/security_negatives_strict_modes.rs` and `tests/chaos_snapshot_replay.rs` as green on all three OSes. The Windows arms and their cases leave. The Unix arm, once made unconditional, must refuse exactly the same trees.

@history_item n=39 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (ConPTY companions' check; the home tree at start; the diagnostics opens) added `pin_companions` running `check_present` over `conpty/` and the `sideload_fallback` closed set including `strict-modes-failed`, `check_home_tree` (Unix owner and mode, Windows owner and DACL) before the wrapper log, and `check_diagnostics` (a file takes the read rule on Unix and the write rule on Windows). The companions check and the Windows arms leave. Whether `sideload_fallback` survives as a closed word is the scope's open fork. The Unix arms of `check_home_tree` and `check_diagnostics` must stay intact.

@history_item n=40 marker=2026-10-27-pinned-copy-race-and-reproducible-builds
**2026-10-27-pinned-copy-race-and-reproducible-builds** — (pinned `viola` placed by `place_private_new`; Windows plain home refused by its inherited write right) left `replace_private_shared` holding only the plugin files and the ConPTY companions, and measured the Windows plain-home refusal by the inherited write right. The companion use and the Windows case leave. `pin_bytes`' no-clobber placement and its lost-placement re-hash must stay.

@history_item n=41 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (serde_json `unbounded_depth` banned; deny-probes at 14/14 with its counting rule; the feed-panic witness; the bounded-reader census) added the serde_json `[[bans.features]]` row, its probe and its test, and named the `14/14` rule as the `probes=(` rows plus the planted channel probe. The ban must survive the `deny.toml` edit, and the count must be re-read by that rule. The feed-panic witness's Windows byte reading was owed to "Windows-only live measurements", which is now retired. Each reader the removal deletes takes its `BOUNDARIES` or `EXCEPTIONS` row in `tests/contract_bounded_readers.rs` with it.

@history_item n=42 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the product error enums by census; `viola_agent_claude::Refusal` folded into `AgentError`) fixed the `thiserror` derives at nine, counted by the census in `tests/contract_error_bodies.rs`, and made `AgentError::BatchScriptChild` the `.cmd`/`.bat` refusal. Removing Windows- or ConPTY-only variants must keep the census and the error-body enumeration exact. The trap: a bare `Refusal` token search also hits `RefusalReason` and `revive`'s private enum.

@history_item n=43 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (verify's refusal codes; `revive --json` documents every refusal; the exit-cause table) put `batch-script-child` into `verify`'s `process-exit` codes and into `revive --json`'s closed exit-1 set, and added `tests/cross_exit_causes.rs`. The batch-script code must be decided once and the same way across all three surfaces when its Windows arm leaves, with no Linux closed word or exit code changing.

@history_item n=44 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the socket-directory check, built at the bind and the client seams) built `check_socket_dir` (its off-Unix arm returns `Ok(())`) and `check_endpoint_dir`, shared by the bind and the three client seams, and kept every check out of `viola_channel::Client::connect`. When the gate goes, the off-Unix `Ok(())` arm must leave with its sibling and never stay behind as an unchecked path. The `NotADirectory` / `Symlink` / `Unreadable` verdicts must stay exact.

@history_item n=45 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`socket-dir-refused`, the sixth cause; a refused socket directory at a hook and at a start) made exit 21's causes six and `revive --json`'s codes eleven, gave hook fail-open its `socket-dir-refused` line, and added `socket_dir_ready` at `run` and `revive`'s start. The Linux exit codes and closed words this chunk must leave untouched are these.

@history_item n=46 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the server-side drop; the client's peer euid check and the order at a seam) built the `accept_loop` foreign-euid drop before any frame, and `peer_euid_admitted` with `PEER_EUID_REPORTED = cfg!(unix)`, where on Windows no euid is read. It fixed the seam order: directory check, connect, euid decision, server verification. Making `PEER_EUID_REPORTED` unconditional must keep the unreadable-peer refusal and this order.

@history_item n=47 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the board's one rule on the home) made `board::scan` hold the home to `check_read` on every OS, with an OPEN mark owned by the Epoch 7 boundary batch. The rule runs through the strict-modes read arm this chunk edits, so the whole-board refusal must stay exact on Linux, and the OPEN keeps its owner.

@history_item n=48 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (macOS's absent pid comparison, a named carve-out) narrowed the frame-ban exception to macOS's pid and start-time check alone, owned by no route entry and borrowable by no other frame or process, with the server-identity negatives compiled out on macOS (`#![cfg(not(target_os = "macos"))]`). The carve-out leaves with the macOS arms, those negatives must then run on ubuntu, and no Linux path may inherit the skipped comparison.

@history_item n=49 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the tests holding the directory and peer checks, and no seam to drive them) placed `tests/security_negatives_socket_dir.rs` (`#![cfg(unix)]`) on ubuntu and macOS, and added the ban on any env, flag or feature seam that forces a refusing peer-euid decision; the decisions stay proven over injected values. This chunk's scope repeats that ban. The socket-dir negatives lose their macOS leg and must keep holding on ubuntu.

@history_item n=50 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (the Unix socket's place in the threat model) placed the socket in the verified per-user 0700 directory, never the `/tmp` or `$TMPDIR` root. The Linux placement must stay exactly this when the Unix gate is lifted.

@history_item n=51 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (the macOS carve-out's tests named in the census) added `tests/contract_os_gated_tests.rs` to the carve-out's stack reference; its `GATES` rows hold each test kept off macOS under the reason `NO_PEER_PID`. This chunk re-points or retires that census along with the gates it counted, and a census that stays must count no gate that no longer exists.

@history_item n=52 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — (test-side removals through the home link) added the two e2e-home keepers' rules: an absolute target, an owner-only real directory, created at 0700 with nothing removed, and a fixed refusal message that names no path. This chunk's msrv fix edits the keeper cases, and these rules must stay.

@history_item n=53 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — (the e2e-home keepers refuse a backing below the headroom floor) added the 9 GiB `statvfs` floor as a compiled constant, set by no env var, flag or config key, with a fixed message holding the two counts. The red `msrv` cases panic on this floor because they read the host's real free space. The fix must give them a free-space figure the test controls, without adding any seam that sets the floor and without loosening the refusal.

@history_item n=54 marker=2026-10-07-live-rows-and-paste-shapes-on-the-dev-host
**2026-10-07-live-rows-and-paste-shapes-on-the-dev-host** — (two scratch measurements across the credential boundary) recorded that the identity floor came from a Windows-host measurement, stood a Linux reading beside it, and kept the floor at eleven names. Once the registry source leaves, the Linux R8 rule keeps this floor as it is.

@history_item n=55 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (the hidden `hook --capture` arm, verify's probe captures and the fixture scrub) set the `verify --record` scrub to cover both separator spellings, case-folded on Windows, and to refuse a drive path and `\Users\`. Any Windows-only arm of the scrub that this chunk touches must leave the Linux scrub and its refusal unchanged.
