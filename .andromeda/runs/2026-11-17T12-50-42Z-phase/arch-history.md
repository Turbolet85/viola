@history_item n=1 marker=2026-09-24-supply-chain-and-workflow-gates
**2026-09-24-supply-chain-and-workflow-gates** — set `deny.toml`'s four families (advisories, licences, sources, bans) and the per-sync-crate `deny-sync.toml` run over `scripts/sync-crates.txt`, and placed the weekly run in `nightly.yml`. These are the configs and jobs part A re-measures; each one reads advisories separately, so an ignore in `deny.toml` alone does not reach `deny-sync.toml`.

@history_item n=2 marker=2026-09-24-quality-gates
**2026-09-24-quality-gates** — set up `fuzz/` as its own workspace with its own `fuzz/Cargo.lock`, outside the root `cargo deny` graph, as an operator-ratified test-only exemption. This is behind rows decision/fuzz-own-workspace and decision/fuzz-workspace-deny-exemption: the root `deny.toml` ignore does not cover the fuzz graph by default.

@history_item n=3 marker=2026-09-24-workspace-tree-and-code-graph-planes
**2026-09-24-workspace-tree-and-code-graph-planes** — wired the fuzz lockfile audit: `supply-chain` runs advisories + sources into `deny-fuzz.json` and the nightly advisories job covers both lockfiles. It runs from the repo root, so cargo-deny finds the root `deny.toml` by walking up. This is why a root ignore may or may not reach the fuzz audit, which part A must measure (row decision/fuzz-lockfile-audited, decision/weekly-advisories).

@history_item n=4 marker=2026-09-25-security-prerequisites
**2026-09-25-security-prerequisites** — recorded the per-crate `0BSD` licence exceptions (`doctest-file`, `recvmsg`) as named exceptions, never through `allow`, as an operator-ratified widening. This is the precedent for admitting one narrowly named exception with its reason in `deny.toml` instead of widening a whole policy.

@history_item n=5 marker=2026-10-26-tokio-rustsec-2026-0386-lock-bump
**2026-10-26-tokio-rustsec-2026-0386-lock-bump** — cleared RUSTSEC-2026-0386 by bumping tokio to a patched 1.53.3, "with no ignore widened". This is the precedent that an advisory with a safe upgrade is fixed by the upgrade. Part A's ignore is admissible only because RUSTSEC-2026-0543 has no safe upgrade.

@history_item n=6 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (RUSTSEC-2026-0441 red) recorded `pin-utils` unmaintained (via portable-pty) as red in supply-chain advisories. Its owner was a CARRY whose fix was the upgrade "with no ignore-list widening". This is the direct precedent for an `unmaintained` advisory reached through an upstream crate. Part A departs from it only under the founder's widened rule (no safe upgrade exists), so the plan should name that difference.

@history_item n=7 marker=2026-11-06-version-done-check
**2026-11-06-version-done-check** — (RUSTSEC-2026-0441 closed / portable-pty =0.9.0) moved portable-pty to `=0.9.0`, dropping `pin-utils`. `cargo deny check` then read advisories ok with `ignore = []` and all seven CI jobs green (ci#42288148289). This is the state part A changes: the ignore list was empty, so this chunk adds its first entry.

@history_item n=8 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (cargo-deny one Linux triple) evaluates `deny.toml` and `deny-sync.toml` for `x86_64-unknown-linux-gnu` alone and dropped the `recvmsg` exception once its crate left the graph (an exception naming an absent crate warns `license-exception-not-encountered`). Part A's ignore must actually match the advisory, or it warns the same way.

@history_item n=9 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (CI on the one ubuntu-latest leg) moved CI to `strategy.matrix.os: [ubuntu-latest]` with two workflows (`ci.yml`, `nightly.yml`) and 7 jobs. The `supply-chain` job is red when only one advisory reads red. This is the job shape the acceptance ("supply-chain green on the pushed sha, the whole run green") is read against (rows decision/ci-ubuntu-fake-agent, decision/github-actions-ci).

@history_item n=10 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — added the `serde_json/unbounded_depth` feature ban to `deny.toml` with its probe row. The deny-probe verdict is now `14/14 banned, control clean`, its count carrying the rule that produced it. This is the latest `deny.toml` edit precedent: part A's edit must leave the bans section and `scripts/deny-probes.sh` green (row decision/cargo-deny-bans).

@history_item n=11 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (Tokio client, tree-form ban) pinned rmcp `>=3.4.1, <3.5`, took `viola-channel` off the sole-root sync list (four crates now) with its tree-form tokio check, and set deny-probes to 13/13. rmcp 3.4.1 is one of the upstream crates the ignore's reason must name as bringing chrono in, and the sync-crate list is the set of `deny-sync.toml` runs part A re-measures.

@history_item n=12 marker=2026-10-16-remove-the-browser-leg
**2026-10-16-remove-the-browser-leg** — (Infrastructure Patterns) narrowed the feature bans, removed the npm step from `supply-chain`, and left `nightly.yml` with two jobs (`advisories`, `fuzz`). This is the current shape of the advisories jobs part A re-measures.

@history_item n=13 marker=2026-10-10-statusline-pass-through
**2026-10-10-statusline-pass-through** — (counts, crate edges) made `viola-agent-claude` take `chrono` at the workspace pin (`clock`, `std`). This shows viola's own direct chrono use, which the scope says stays: the ignore is for upstream reach, and the plan must not imply replacing chrono.

@history_item n=14 marker=2026-10-26-explicit-session-links
**2026-10-26-explicit-session-links** — (log lines and since stamp) has `LinkSet::line` stamp `since` with `Utc::now()` directly. This is another of viola's own chrono uses in the root bin, untouched by this chunk.

@history_item n=15 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — (CI setup) set SHA-pinned actions, toolchains installed by rustup, `permissions: {}`, and the ban on mutable refs. These are the constraints on any workflow touch while re-measuring the advisories jobs (rows decision/ci-ubuntu-fake-agent, decision/gate-steps-fail-closed).

@history_item n=16 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — moved mutation testing out of CI and the pre-push and kept it for the epoch-boundary audit. This is behind row decision/mutation-at-epoch-boundary and the scope's "no mutation run".

@history_item n=17 marker=2026-10-03-mutation-scoring-completion
**2026-10-03-mutation-scoring-completion** — (pre-push native on the Linux host) runs `pre-push` natively on the Linux host under `env -i HOME PATH`, and CI stays the verdict of record. This is the local gate the operator pass runs before the push whose CI run is this chunk's acceptance (row decision/local-pre-push-gate).

@history_item n=18 marker=2026-11-17-booted-wrapper-does-not-outlive-its-test
**2026-11-17-booted-wrapper-does-not-outlive-its-test** — registered the test-only self-spawn markers `ROOT_TIED_TEST_CHILD` and `HARNESS_TIED_TEST_CHILD`, outside `VIOLA_*`. Part B uses this tie and census for `harness_lifecycle.rs` and `cli.rs` and for the tied stamping verify. The tied verify was witnessed by no kill there, which this chunk closes.

@history_item n=19 marker=2026-11-17-standing-live-rig
**2026-11-17-standing-live-rig** — added `AGENT_RUN_RIG` (a census marker set only on the rig's child) and `viola-live-rig` to the harness-variable and test-only-binary rows. This is the latest precedent for how a census marker is registered and kept unread by viola (rows decision/harness-env-var-prefix, decision/test-binaries-never-released).

@history_item n=20 marker=2026-10-20-epoch-5-cleanup-ii
**2026-10-20-epoch-5-cleanup-ii** — (test-only child marker) registered `MCP_SERVE_TEST_CHILD`, a crate's `#[cfg(test)]` child marker outside `VIOLA_*`. This is the precedent the tied-child markers follow and that row decision/harness-env-var-prefix names (`MCP_*_TEST_*`).

@history_item n=21 marker=2026-09-24-three-os-ci-headless-harness-skeleton
**2026-09-24-three-os-ci-headless-harness-skeleton** — (test-only crate, bins, env vars) registered `viola-e2e` with its no-op `fake-agent` feature, `viola-harness`, and the `AGENT_RUN_` prefix for harness-only variables. Rows decision/e2e-noop-fake-agent-feature and decision/harness-env-var-prefix rest on it.

@history_item n=22 marker=2026-09-24-fake-agent-and-test-data-fixtures
**2026-09-24-fake-agent-and-test-data-fixtures** — registered `AGENT_RUN_KEEP_FAILED`, the `viola-session-*/home` (harness) and `viola-test-*/home` (root) test homes, and `tests/support/`. These are the home classes a booted test's supervisor runs in.

@history_item n=23 marker=2026-09-27-epoch-2-cleanup
**2026-09-27-epoch-2-cleanup** — made `AGENT_RUN_KEEP_FAILED` also read by `viola-e2e`'s `harness_lifecycle` `Booted` guard, and enabled viola-channel's `test-support` feature only through root dev-dependencies. This is the guard in the very file part B ties (row decision/test-support-never-in-product).

@history_item n=24 marker=2026-10-17-end-to-end-path-binary-and-harness-legs
**2026-10-17-end-to-end-path-binary-and-harness-legs** — made the harness `session` guard's `Drop`, after `cleanup`, keep the home under the keep variables, and added `bin/agents-mode` written by `boot`. This is the boot/drop lifecycle that a tied boot must still honour when killed.

@history_item n=25 marker=2026-09-29-verify-stamped-test-homes-and-harness
**2026-09-29-verify-stamped-test-homes-and-harness** — has harness session homes stamped at `boot` step 4 by `viola verify` against the fake agent (unless `--unstamped`), and makes `run --local-live` refuse under `CI`. This is the stamping verify part B kills (the earlier victim booted unstamped), and the no-live-claude-in-CI boundary (rows decision/no-live-claude-in-ci, decision/real-cli-probe-local-only).

@history_item n=26 marker=2026-10-13-interrupted-verify-cleanup
**2026-10-13-interrupted-verify-cleanup** — removes verify's probe dirs by drop guard when it passes, fails, or is stopped by Ctrl-C/TERM, only the dirs that run created. SIGKILL or a crash leaves them by hand, and an interrupted verify exits 1 with `detail:"interrupted"`. This is the trap for "nothing of it is left" after killing a tied verify: a SIGKILL leaves probe dirs unless the tie stops it by TERM or the test cleans what it owns (rows decision/verify-probe-dirs-drop-guarded, decision/verify-removes-only-own-dirs).

@history_item n=27 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (verify's stop: SIGINT / SIGTERM alone) narrowed verify's stop to `SIGINT`/`SIGTERM`, and the leftovers to SIGKILL, a crash and a failed handler install. This is the signal set that decides what a killed tied verify leaves.

@history_item n=28 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (verify's refusal codes on its process-exit) has verify's `process-exit` carry its refusal code beside `interrupted`. A refused home tree writes no line, and verify takes no `--json`. This explains rows contract/viola-verify-refusal-codes/interrupted, contract/verify-process-exit/interrupted and contract/verify/no-json, which a killed-verify witness may read.

@history_item n=29 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (home-tree check before verify's log) has `viola verify` run `check_home_tree` before its `cli` log, and refuse with exit 1, no stamp and no line. This is behind row decision/verify-refuses-untrusted-home-or-stamps.

@history_item n=30 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (stamps writer strict-modes check) has `update_stamps` run `check_stamps` before creating `ledger/` and again under its lock, so a refusal reads and writes nothing. This is behind rows decision/stamps-writer-checks-before-and-under-lock and decision/stamps-single-writer: a verify killed mid-stamp must not leave a torn or foreign-writable stamps file.

@history_item n=31 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (ledger rows, stamps envelope) set up the stamps envelope with `writer:"verify"`, a merge that keeps unknown versions, and wrong-shape bytes replaced whole. This is behind rows contract/ledger-stamps/writer and decision/stamps-merge-own-version-only.

@history_item n=32 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (hook --capture arm, probe session) made verify's transient probe the one exception to wrapped-only plugin loading, removed when verify ends. This is behind row decision/plugin-only-in-wrapped-sessions's except: a killed verify's children must also be gone.

@history_item n=33 marker=2026-10-14-resume-and-compact-ledger-rows
**2026-10-14-resume-and-compact-ledger-rows** — (Run E) widened verify's child set to five interactive runs, each in its own guarded dir, killed at its end. This is part of row decision/verify-child-set: the processes a killed stamping verify could leave behind.

@history_item n=34 marker=2026-10-16-unwrapped-sessions-on-the-board
**2026-10-16-unwrapped-sessions-on-the-board** — (agents-join row, Run B's agents read) added verify's one `claude agents --json` read inside Run B. This completes the bounded child set of row decision/verify-child-set.

@history_item n=35 marker=2026-10-05-dialog-rows-and-re-probe
**2026-10-05-dialog-rows-and-re-probe** — has Run D point `plansDirectory` at a 0700 `plans/` inside its own run dir. This is behind row decision/verify-plan-files-in-run-dir, one more on-disk leftover a killed verify could leave.

@history_item n=36 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — (verify's print-mode probe leaves an empty CLI project dir) classes the CLI project dirs and transcripts verify leaves under `~/.claude/projects/` as an accepted residual. Row decision/verify-transcripts-accepted-residual applies: those do not count against "nothing left", though the fake agent should leave none.

@history_item n=37 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (the relied set) made `LedgerRow::relied()` every row on every build. Row decision/version-verified-all-relied-rows applies: a stamp is verified only when every relied row passes, which is what a stamping boot produces.

@history_item n=38 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (Design Philosophy, Established Decisions) the founder ratified StopFailure registered with no ledger row and no verify probe. This is the except on rows decision/capability-ledger and decision/cli-capability-ledger.

@history_item n=39 marker=2026-11-16T06-05-33-wrap
**2026-11-16T06-05-33-wrap** — (Cross-cutting Patterns) dropped "provisionally" from row decision/capability-ledger-gate's except for StopFailure.

@history_item n=40 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (a force-killed wrapper's socket file) leaves a force-killed wrapper's socket file standing. Harness `cleanup`'s `endpoint_gone` reads a refused or absent connect as gone and removes no leftover file. This is behind row decision/cleanup-reads-refused-or-missing-socket-gone: after a tied boot is killed, cleanup reads ok with the file still there.

@history_item n=41 marker=2026-10-10-viola-revive
**2026-10-10-viola-revive** — (kill path's endpoint wait) has `Wrapper::kill` wait on `holder_gone` (connect refused or path absent), because a killed wrapper leaves its socket file. The plan's earlier wait, that the file is gone, was disproved because it could never end. This is the trap for any wait after killing a booted wrapper.

@history_item n=42 marker=2026-10-02-epoch-2b-cleanup
**2026-10-02-epoch-2b-cleanup** — routes test-home removals (the drop and the gone-owner sweep) through `remove_owned`, deleting `owner.json` last. This is behind row decision/test-home-owner-sweep: a home whose tied supervisor is killed must stay sweepable by its owner record.

@history_item n=43 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — set up `target/e2e-home` with an owner record and gone-owner sweep, and liveness by pid + start time, `gone` whatever the beat. This is behind rows decision/test-homes-cleaned-by-owner and contract/session-liveness/gone, which decide when a killed test's home or wrapper reads gone.

@history_item n=44 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — links `target/e2e-home` to a tmpfs on the dev host, with the keepers' strict link checks, and ratified test-side removals outside the working directory. A standing rule says a stalled-start red is a backing finding, never re-run for green (rows decision/dev-host-e2e-home-on-tmpfs, decision/linked-e2e-home-base-strict, decision/test-side-removal-outside-workdir).

@history_item n=45 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — has the keepers refuse a linked base below a 9 GiB headroom floor with a fixed message. This is behind rows decision/e2e-home-headroom-floor, decision/e2e-home-owner-only-headroom and contract/e2e-home-refusal/headroom-message, which matter if killed tests leave homes on the tmpfs.

@history_item n=46 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (v1-47's reading over the homes CI keeps) has each CI job keep its homes after its end-to-end run (`AGENT_RUN_KEEP_HOMES` "1", `if: always()`), the chaos home out. This is behind row decision/end-to-end-homes-kept-after-each-job: tied boots in CI still keep their homes for the obs gates.

@history_item n=47 marker=2026-10-13-epoch-4-survivors
**2026-10-13-epoch-4-survivors** — named the real-PTY watch reports per test process (`<test name>.<pid>.report`), kept on panic or kill and removed on pass. This is the precedent behind rows decision/test-evidence-survives-kill and decision/wait-report-kept-on-failure that a kill test's evidence follows.

@history_item n=48 marker=2026-09-29-h2-conpty-resize-probe
**2026-09-29-h2-conpty-resize-probe** — (viola-pty-watch dir) set up the test's own step report beside the child's, codes and counts only, kept on panic or kill. This is the origin of the kill-surviving evidence pattern.

@history_item n=49 marker=2026-11-15-mutation-and-coverage-baselines-green
**2026-11-15-mutation-and-coverage-baselines-green** — raised the shared root wait bound `WITHIN` to 12 s and bound test channel sockets under a short `/tmp` root, never `TMPDIR`. These are rows decision/root-waits-share-one-bound and decision/test-sockets-under-short-tmp-root, which the scope's boundaries name for new waits and sockets.

@history_item n=50 marker=2026-10-09-epoch-3-cleanup-ii
**2026-10-09-epoch-3-cleanup-ii** — counts `Instant::now() + WITHIN` sites by pattern, not by a census of every wait. Trap: a wait written through `EXIT_WITHIN` or a qualified path falls outside the pattern, so any count this chunk moves must carry its rule.

@history_item n=51 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (root WITHIN waits) re-measured 31 sites in 23 files by `grep -rn`/`grep -rln`, and `holder_gone` is a connect refused or the path absent. This is the last count; new waits in this chunk move it.

@history_item n=52 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (test-owned socket roots) registered `SocketRoot`, `Wrapper::boot_on_socket_root` and `SOCKET_ROOT_VAR`, test-owned socket roots under `/tmp`. Any new tied boot that needs its own socket root uses these.

@history_item n=53 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (socket-directory check) added `check_socket_dir`/`check_endpoint_dir`, run before the bind and each client's connect; they refuse without chmod. This is behind row decision/socket-dir-strict-check.

@history_item n=54 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (wrapper endpoint is a Unix socket only) retired the named pipe. The endpoint is a Unix socket in `$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`, chmod 0600 after the bind. This is behind rows decision/wrapper-ipc-local-socket and decision/no-abstract-namespace-sockets.

@history_item n=55 marker=2026-09-27-wrapper-channel
**2026-09-27-wrapper-channel** — set the endpoint path and lock arbiter, and registered the test-only `viola-test-chan-<pid>-<label>` namespace outside `viola-<h12>`. This is behind rows decision/endpoint-name-stable-hash and decision/test-endpoints-outside-namespace.

@history_item n=56 marker=2026-10-26-server-verification-before-any-frame
**2026-10-26-server-verification-before-any-frame** — (snapshot's pid and started_at as server identity) has every frame-writing client compare the peer pid and OS start time with the snapshot's before its first frame. This is behind rows contract/instance-snapshot/pid-and-started-at and contract/instance-snapshot/started-at.

@history_item n=57 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (exit-21 document carries its cause) has a `wait` whose endpoint vanishes read `instance-unreachable` with `detail` `endpoint-lost`. This explains row contract/channel-wait/timeout-ms, which a test waiting on a killed wrapper reads.

@history_item n=58 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (second fake-agent test seam) ratified `FAKE_AGENT_HOOK_PANIC` beside `FAKE_AGENT_PUMP_DELAY_MS`, each read only under `fake-agent`. This is behind rows decision/fake-agent-test-seams and decision/test-seams-not-configuration: no new seam variable is admitted without its own ratification.

@history_item n=59 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (G2 script) added `scripts/g2-zero-panics.sh`, fail-closed, exempting only the hook seam module. This is behind row decision/zero-panics-gate: a killed or tied test must not leave a panic role line under the end-to-end home.

@history_item n=60 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — (VIOLA_DIR home step ratified) ratified the CLI verbs' home resolution (`--home`, else the `VIOLA_DIR` step, else the default, canonicalised). This is behind rows decision/viola-home-resolution and decision/test-isolated-viola-home.

@history_item n=61 marker=2026-10-17-cli-output-discipline
**2026-10-17-cli-output-discipline** — ratified the `NO_COLOR`/`TERM`/`COLORTERM` reads as the colour exception. This is part of row decision/no-environment-configuration's except list, which a tie marker or census variable must not widen.

@history_item n=62 marker=2026-09-28-cli-output-tokens
**2026-09-28-cli-output-tokens** — made the exhaustive env sentence hold by closing clap's `color` route: a dependency's env read breaks the rule like a direct read. This is behind row decision/no-environment-configuration and decision/viola-env-namespace-product-only.

@history_item n=63 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (OS-gate census faults another OS) has `tests/contract_os_gated_tests.rs` fault any test gate naming `windows`/`target_os = "macos"`, and require every OS gate to carry a `GATES` row. New tests in this chunk land ungated or with their row (row decision/linux-only).

@history_item n=64 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (OS target: Linux only) made viola Linux only, the Windows/macOS code recoverable at `multi-os-final`. This is behind row decision/linux-only: the kill and tie mechanics need only a Linux form.

@history_item n=65 marker=2026-11-04-linux-and-macos-parity
**2026-11-04-linux-and-macos-parity** — (shared Frozen guard) made `tests/support/frozen.rs` `Frozen` the one home for freezing a wrapper (`kill -STOP`/`-CONT` on a pid the test booted). This is the precedent for a test signalling only a process it owns.

@history_item n=66 marker=2026-09-27-instance-state-and-start-order
**2026-09-27-instance-state-and-start-order** — (pinned copy) never deletes pinned copies or plugin folders. This is behind row decision/no-deletion-of-pinned-copies: "nothing left" after a killed stamping verify is read on processes and probe dirs, not on `bin/`/`plugin/`.
