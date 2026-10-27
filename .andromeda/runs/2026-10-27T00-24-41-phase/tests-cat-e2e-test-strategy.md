# tests · E2E Test Strategy — category read

## Carried
- (.andromeda/test-plan.md:135) The run start's order (the pinned copy and the plugin folder before the child spawn) and its exit-1 refusal of a tampered pinned exe must stand unchanged once the absent-copy write is made safe under a concurrent first start.
  > the collision check, pinned copy and plugin folder, the fail-open ConPTY sideload, version gate, exclusive endpoint bind, first snapshot and heartbeat, and the start `wheel` and `budget-gate` events all precede the child spawn; a second `run` of the same name, a squatted endpoint, a tampered pinned exe or a refused home tree exits 1.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:28) Path 1 steps 5 and 6 (one byte of the pinned copy overwritten, then the copy deleted and re-pinned) must still pass after the pin change, so the hash-before-reuse rule and the absent-copy re-pin are both kept.
  > Overwrite one byte of the file at snapshot `pinned_bin`, and write a sentinel into the instance's `plugin/` hooks.json and `settings.json`. Then run `viola run builder --home H` again.
  >   6. Restore the pinned exe by deleting it. Run again, and read the plugin files.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:31) The race's symptom is a hook receipt with `ran:false`, so the race witness must read the fake agent's `hook` receipt, and a clean side must read the same shape Path 1 asserts.
  > the fake agent's `hook` receipt reads `command_absolute:true, ran:true, exit_code:0`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:34) The tampered pinned exe must keep exiting 1. A concurrent start's same-bytes acceptance must never turn into accepting a copy that differs.
  > The duplicate `run` exits 1. The squatted endpoint exits 1. The tampered pinned exe exits 1.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:35) The per-start `plugin/` rewrite must keep writing absolute `pinned_bin` hook commands whatever this chunk does about a concurrent rewrite of `hooks.json` / `.mcp.json`.
  > After restart, `plugin/` hooks.json no longer contains the sentinel, and its hook commands are the absolute `pinned_bin` path, not a PATH lookup (M6).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:37) The home-tree check covers `bin/<key>/` and the pinned copy before the wrapper log opens, and the pin change must not move the copy's write ahead of that check.
  > then `bin/<key>/`, the pinned copy, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`, each that exists) before the wrapper log opens.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:38) The landed Path 1 flow is the regression floor for the pin surface and must stay green on its kill line.
  > steps 1–6 run as one ordered flow over one stamped home in the root path binary `tests/path1_run_start.rs`, `path1_run_start_one_home_in_order(stamped_home)` (`[[test]]` `required-features = ["fake-agent"]`; in the E2E layer, `nextest-e2e`; on the `ci` profile's verify-driven 20 s kill line)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:194) Path 7's unstamped boot is one instance, which is the CARRY's test-side workaround. Whether it goes back to two instances once the race is closed is decided at P4, and the step stays as written until then.
  > `agent-run boot --session p7 --unstamped --instance builder` (no stamps at all). Separately, `agent-run boot --session p7-stamped --instance builder:--report-version 0.0.0-unlisted`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:283) The exit-1 cause for a pinned copy that fails its re-hash must survive the concurrency change.
  > the pinned exe fails its SHA-256 re-hash (Path 1 step 5)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:306) The pinned copy written by the concurrent-safe path must still be 0700 and never group- or world-writable under `umask 000`.
  > Start `viola run` under `umask 000` (via `sh -c 'umask 000; exec viola run …'`). `events.ndjson`, `snapshot.json` and the other home files are still 0600, directories 0700, and the pinned `bin/<version>-<hash>/viola` is 0700 and never group- or world-writable.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:367) The pinned-exe content hash that names `bin/<key>/` is pinned to `sha2` by literal vectors. The reproducible-build measurement reads that same hash, and no change may move it off this contract.
  > `tests/contract_content_hash.rs` pins the pinned-exe content hash (§1 "SHA-256 16-hex re-hash") to its library, `sha2` (a root dev-dependency)
- (.andromeda/test-plan.md:309) A witness of the race (concurrent starts) must be synchronised without sleeps and run under zero retries.
  >   - no nextest retries (`retries = 0`) and no `std::thread::sleep` / `tokio::time::sleep` used for synchronisation
  >   - waits key on `events.ndjson` byte offsets or `viola wait --after <cursor>`
- (.andromeda/test-plan.md:345) Any harness flag or selector the witness adds, such as a two-instance unstamped boot form, stays a usage error until its chunk builds it.
  > a flag or selector whose surface is not built yet is a usage error until its chunk lands, never a vacuous pass
- (.andromeda/test-plan.md:431) A race test needs a fresh home that viola creates under `target/e2e-home`, with wrappers in a `Drop` guard. On Windows x64 the `seed_conpty` carve-out applies to the companion pin.
  > a fresh tempfile home per test, at a not-yet-existing path under `target/e2e-home` that viola creates, so obs-plan's CI gates, the secret scan and the upload cover integration runs; started wrappers sit in a `Drop` guard that kills and waits, and nothing is shared between tests. Two carve-outs: Windows x64 starts seed the ConPTY companions before viola runs, into a home `seed_conpty` makes private
- (.andromeda/test-plan.md:634) `stamped_home` is pinned by `viola verify`, which is the workaround, so a fresh-home race witness must make its home only through `create_private_dir` and must not start from a pre-pinned home.
  > a case that needs a home viola created starts from `stamped_home`, and a test that makes a home itself makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`
- (.andromeda/test-plan.md:661) The agent spawns the pinned path directly, and that spawn is the race's victim, so the fake agent's absolute-command spawn must stay the witness's instrument.
  > an absolute `command` is spawned directly, never through a shell or a PATH lookup, and a non-absolute one is never run
- (.andromeda/test-plan.md:689) `boot` stamps through `viola verify` before the wrappers start, so verify's `pin_exe` pins first. A witness must name which form it boots, stamped or unstamped.
  > The `boot` command creates the home, stamps it via `viola verify`, and starts the wrappers automatically.
- (.andromeda/test-plan.md:760) The reproducible-build measurement goes through `scripts/release-check.sh`'s release build of `viola` alone.
  > **Release build:** `scripts/release-check.sh`, with its probe, judges a `--release` build of `viola` alone and fails on a test-only feature or binary; no upload and no gate step.
- (.andromeda/test-plan.md:795) If the measurement touches the release build, the release-check verdict and its probe line must still end as stated.
  > Release-build errors: `scripts/release-check.sh` not ending `release-check: viola only`, or its probe not ending `release-check probes: 5/5 refused, control clean`
- (.andromeda/test-plan.md:854) A race witness is open to flakes. A once-flaking control keeps the chunk red until its root cause is fixed, and a retry never greens it.
  > **Zero-flakiness budget:** flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk.
- (.andromeda/test-plan.md:871) A narrower concurrent-pin control belongs at the lowest layer that can hold it. The E2E layer asserts only the cross-process hook-spawn failure.
  > NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:876) The mechanism is measured on Linux. The Windows exe arm (`ERROR_SHARING_VIOLATION`) can be shown only on its own CI runner.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:898) The concurrent starts must be ordered by an explicit signal, never by `sleep(N)`.
  > NEVER use `sleep(N)` for synchronization. Wait for an explicit signal: an `events.ndjson` offset, `viola wait --after` or a status field.
- (.andromeda/test-plan.md:901) Two concurrent `viola run` wrappers are long-lived, so they are driven by std::process with a `Drop` guard.
  > NEVER use `assert_cmd` `.assert()` on the long-lived `viola run` wrapper. It blocks until exit; use std::process + a `Drop` guard.
- (.andromeda/test-plan.md:908) A race test must not hand-write `stamps.json` or `snapshot.json` to shape the fresh home.
  > NEVER hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`. Stamps come from `viola verify`, snapshots from the wrapper and budget from `viola hook statusline`. The ban binds every booted-wrapper test.
- (.andromeda/test-plan.md:918) The witness must exercise the real pinned binary through the plugin files `run` wrote. Stubbing `viola hook` hides the race.
  > NEVER stub `viola hook` in E2E. The fake agent must invoke the real pinned binary through the plugin files `run` wrote.
- (.andromeda/test-plan.md:922) Concurrency belongs inside the witness's own home. The suite stays parallel.
  > NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.
- (.andromeda/test-plan.md:946) The race window is found without test-owned timers or elapsed-time verdicts.
  > NEVER use real time without injection (use deterministic clocks) in test-side code: no test-owned timer, sleep or elapsed-time verdict.

## Bindings
- **Path 7's one-instance unstamped boot ↔ §3 `boot` (test-plan-contracts registry) and the route CARRY 16**: the boot form is the harness contract's, and returning Path 7 to two instances is P4's decision (.andromeda/test-plan-detail/e2e-test-strategy.md:194), (.andromeda/test-plan.md:345)
- **Path 1 tamper/re-pin and exit-1 cause ↔ security-plan §Local trust boundary (Home: re-hash a pinned exe before reuse)**: the hash-before-reuse rule the chunk leaves unchanged is held end to end by Path 1 and the exit-cause row (.andromeda/test-plan-detail/e2e-test-strategy.md:28), (.andromeda/test-plan-detail/e2e-test-strategy.md:283)
- **Pinned copy mode ↔ security-plan Home (0700 pinned `bin/<version>-<hash>/viola`, `replace_private`)**: a concurrent-safe write still sets mode explicitly (.andromeda/test-plan-detail/e2e-test-strategy.md:306)
- **Content hash contract ↔ CARRY 11 measurement and the `bin/<key>/` naming**: the hash a rebuild is judged by is the one `contract_content_hash.rs` pins (.andromeda/test-plan-detail/e2e-test-strategy.md:367)
- **Release-check build ↔ §9 CI release build and security-plan §Dependencies and CI (release build carries `viola` only)**: the measurement's build and its verdict lines (.andromeda/test-plan.md:760), (.andromeda/test-plan.md:795)
- **Race witness discipline ↔ §10 Zero-flakiness budget and §11 E2E/Universal bans**: no sleep, no retry, parallel suite, real pinned hook (.andromeda/test-plan.md:854), (.andromeda/test-plan.md:898), (.andromeda/test-plan.md:918)
- **Fresh-home setup ↔ §5 setup/teardown lifecycle and §7 seed strategies (`seed_conpty` carve-out on Windows x64)**: (.andromeda/test-plan.md:431), (.andromeda/test-plan.md:634)
