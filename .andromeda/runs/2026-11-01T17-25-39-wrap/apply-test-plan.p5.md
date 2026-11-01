# Apply — test-plan · part 5 of 5

## Verdicts
- fanout-test-plan--cascade-g3.md 21 · applied · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:21#1
- fanout-test-plan--cascade-g3.md 22 · applied · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:22+#1
- cascade-compare-test-plan.md 1 · applied · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:20#1
- fanout-test-plan--cascade-g3.md 23 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:18+#1
- cascade-compare-test-plan.md 2 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:26+#1
- fanout-test-plan.md 4 · applied · .andromeda/registries/contracts/test-plan/5-command-implementation.md:73#1
- fanout-test-plan.md 5 · applied · .andromeda/registries/contracts/test-plan/5-command-implementation.md:27#1
- fanout-test-plan.md 6 · applied · .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10#1, .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10#2

## Edits
- line: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:21
  old: «starts its home unseeded (§7, a widened seeded home) |»
  new: «starts its home unseeded (§7, a widened seeded home); a loosened socket directory is made on the real file too, at 0755 or as a symlink to 0700 (`Form` in `tests/security_negatives_socket_dir.rs`), refused by the product's own check and read back as found, and the pure `socket_dir_verdict` takes a foreign owner's uid by injection at the unit tier |»
- after: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:22
  anchor: «| Forced-window test parameters | none (a parameter of the private function under test) | `place_private_new_with`'s `placed` hook (`crates/viola-state/src/fs.rs`), run right after a `Placed::Written` placement, is a test parameter, not a seam: no env read, no feature, no mockall double, and nothing added to the test-seam list |»
  add: «| Peer euid decisions | none (a parameter of the decision or of the private function under test) | `peer_euid_admitted(observed, own, reported)` takes its euids as arguments and the private `serve_admitting` takes its decision as an `Admission` argument (`crates/viola-channel/src/server.rs`): dependency injection, with no env, flag or feature seam |»
- line: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:20
  old: «a root-bin unit test having no portable second process; no HTTP fake servers |»
  new: «a root-bin unit test having no portable second process; a test server's endpoint directory is made through `viola_state::fs::create_private_dir`, never a bare `tempfile::tempdir()`, whose mode `tempfile` 3.27 leaves to the umask (0755 on a default one) and which the client's socket-directory check refuses ahead of the server check (`crates/viola-mcp/tests/handlers.rs`, `tests/hook_fail_open.rs`, `tests/security_negatives_server_identity.rs`, `refused_inside` in `src/cmd/client.rs`); no HTTP fake servers |»
- after: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:18
  anchor: « The Epoch 5 boundary audit's 16 survivors were each disposed by a nextest case in the package that owns the mutated file, 10 killed and 6 after a behaviour-preserving restatement, each red under its hand-applied mutant and green reverted byte-exact, with no mutation run made and none equivalent or recorded (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/evidence/survivors.ndjson`). The boundary audit measures the `cfg(windows)` code by dispatching `windows-mutants.yml` (founder ruling C2, 2026-10-04; §9 Mutation row): `run --mutants --package <member> --file …` per matrix item on `windows-2025` (§9 Mutation row), verdict `package`, report-only and never a gate. In its first run, 37174673472, 19 of 25 misses were `#[cfg(unix)]` twins in the scoped files, which the Windows build compiles out and cargo-mutants grades missed, with 0 timeouts across 508 mutants (`viola-0.1.0/chunks/2026-10-04-windows-boundary-mutation-workflow/evidence/windows-dispatch.md`). The harness now leaves such twins out itself (§3 `run` step 4, Verdict). Run 38036448183 read all nine jobs green over 642 mutants: 537 caught, 75 unviable, 30 left out as host-excluded (the sum of the nine documents' `host_excluded` lengths), 0 missed and 0 timeout, the longest job 49 min against the 120 min ceiling, as measured at `viola-0.1.0/chunks/2026-10-10-windows-mutation-grade/evidence/windows-dispatch.md`. That run was one dispatch outside the boundary audit, made on 2026-10-10 for chunk 2026-10-10-windows-mutation-grade alone and on the founder's own word for it; the ruling stands as written.»
  add: « A mutant replacing a client seam's `peer_admitted` read (`connect` in `src/cmd/client.rs` and in `src/cmd/hook.rs`, `frame` in `crates/viola-mcp/src/tools.rs`) with an always-admit survives every committed test by construction: one account can make no foreign peer, and no seam may be added (security-plan §Security Anti-Patterns, Universal). The mutation audit reads that same survivor at the route entry "Version done-check".»
- after: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:26
  anchor: « Every timed hook also runs its instance strict-modes check (`check_instance`) before any `config.json` read or obs init, and makes its server check (the peer pid and its OS start time against the one snapshot read) before its first frame; with both checks in the timed path the five hook rows read `max` 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms on the Linux dev host, under `SPINE_DEADLINE`, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md` (Outcome, `run --perf`).»
  add: « Each timed hook's `connect` also runs the socket-directory check (`check_endpoint_dir`, before it connects) and the peer euid decision (`peer_admitted`, after it connects), both before its first frame and each a real check on Unix only; with those two in the timed path the five hook rows read a mean of 2.0, 2.0, 2.0, 2.0 and 2.3 ms and a highest `max` of 2.8 ms, under `SPINE_DEADLINE`, as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening (report, Outcome, `run --perf`). The `max` reading above is that of the path without these two, and the gate stays `SPINE_DEADLINE`.»
- line: .andromeda/registries/contracts/test-plan/5-command-implementation.md:73
  old: «on Unix the `<per-user 0700 dir>/viola-<h12>.sock` path no longer exists (`$XDG_RUNTIME_DIR/viola/` on Linux, `$TMPDIR/viola/` on macOS, `/tmp/viola-<uid>/` as the fallback; security IPC access control).»
  new: «on Unix a connect to the `<per-user 0700 dir>/viola-<h12>.sock` path is refused or the path is absent (`$XDG_RUNTIME_DIR/viola/` on Linux, `$TMPDIR/viola/` on macOS, `/tmp/viola-<uid>/` as the fallback; security IPC access control). After a force-killed wrapper on Unix the leftover socket file stays until a next bind of that name takes it over: cleanup removes nothing of it, and its document reads `endpoint_gone` true and `ok:true` (`cleanup_after_a_force_killed_wrapper_reads_the_endpoint_gone`, `unconnectable_reads_a_leftover_socket_file_no_one_listens_on`).»
- line: .andromeda/registries/contracts/test-plan/5-command-implementation.md:27
  old: «on Unix the socket path is gone)»
  new: «on Unix a connect to the path is refused or the path is absent, what `holder_gone` reads there)»
- line: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10
  old: «then the twenty-five verify-driven binaries»
  new: «then the twenty-six verify-driven binaries»
- line: .andromeda/registries/contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10
  old: «path1_run_start tui_wheel`»
  new: «path1_run_start security_negatives_socket_dir tui_wheel`»

## Patterns
[[pattern]]
id = "tp5-01"
fixed = 'path no longer exists'

[[pattern]]
id = "tp5-02"
fixed = 'on Unix the socket path is gone'

[[pattern]]
id = "tp5-03"
fixed = 'twenty-five verify-driven binaries'
