# tests · Test Scope Summary — category read

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:78) The harness `boot` builds viola into a fresh home and starts one or more wrappers, and the race fires on that fresh-home multi-instance boot. A boot whose agent's first hook spawn fails, so that `session-start` never lands, must read as a readiness failure. It must never read as a pass.
  > Builds `viola` and creates a fresh temp home, passed as `--home` (arch excerpt, Per-test isolation).
  >     - Starts one or more `viola run <name> -- <fake agent> <args>` wrappers.
  >     - Readiness, per instance: `instances/<name>/snapshot.json` exists with `pid`, `started_at`, `child_pid` and `endpoint`. The `heartbeat` mtime is under 5 s old. The log has a start `wheel` and a `budget-gate` event, followed by `session-start` once "Verify-stamped test homes and harness" (the `boot` step-4 owner) joins it to harness readiness.
- (.andromeda/test-plan-detail/test-scope-summary.md:81) A start the pin refuses (a tampered pinned copy) must still surface verbatim from `boot` as exit 1 after the race fix.
  > On readiness failure it exits non-zero with a JSON reason on stdout. A `run` refusal to start (exit 1: a `live` or `stale` name, a tampered pinned copy, or a `.cmd`/`.bat` child) is surfaced verbatim.
- (.andromeda/test-plan-detail/test-scope-summary.md:115) The standing test-side workaround depends on stamps, and so on `viola verify`'s `pin_exe`, which is the second caller of the absent-copy arm. Stamps must come only from `viola verify` run against the fake agent.
  > Stamps must therefore be produced by running `viola verify` against the fake agent, or the conflict must be resolved upstream. Resolved in §12 (initial entry, ratified by User review 1): the first option is used.
- (.andromeda/test-plan-detail/test-scope-summary.md:116) A race witness must start from no pre-seeded `bin/<key>/`, because the race exists only on a fresh home.
  > No developer-seeded state.
- (.andromeda/test-plan-detail/test-scope-summary.md:117) Any concurrent-pin control that makes its home itself must make it private, or the home-tree check refuses the home before the pin is reached. That would make the control vacuous.
  > A test that makes a home before viola runs makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`: the instance and home-tree checks refuse a `create_dir_all` home (Unix: its 0755 mode fails the read rule; Windows: its inherited Users write ACE).
- (.andromeda/test-plan-detail/test-scope-summary.md:123) This is the `run` start order the fix must keep. The home-tree check covers `bin/<key>/` and the pinned copy. The collision check comes before the pin but is per name, so two names still race. The pinned copy and the plugin folder must still be in place before the child spawn.
  > The home-tree strict-modes check before the wrapper log opens (`check_home_tree`: the home under the read rule, the instance check's set, then `ledger/`, `bin/`, `plugin/`, `bin/<key>/` and the pinned copy, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, each that exists), the collision check (over a replayed snapshot, liveness is the beat alone), pinned copy and plugin folder, the ConPTY sideload (Windows x64, fail-open: any failure leaves the child on the inbox ConPTY), version gate, exclusive endpoint bind, first snapshot and heartbeat, start `wheel` and `budget-gate` events, and only then the child spawn.
- (.andromeda/test-plan-detail/test-scope-summary.md:127) After a concurrent first start, each snapshot must still carry `pinned_bin`, and `plugin/` must still be rewritten. The per-start plugin rewrite is the second concurrency surface the scope names.
  > - `snapshot.json` has `pinned_bin`, `pid`, `started_at`, `child_pid` (`endpoint` from "Wrapper channel" on); `plugin/` is rewritten, and on Unix the instance's `settings.json` (the statusline override: 0600, the absolute pinned path, only for a pinned path of plain characters); on Windows no `settings.json` is written.
- (.andromeda/test-plan-detail/test-scope-summary.md:129) The race fix must leave the tampered-copy refusal and the widened pinned-copy refusal at exit 1 before any log. The hash-before-reuse rule stands unchanged.
  > A duplicate `run` exits 1. A tampered pinned exe (hash mismatch) exits 1. A refused home tree (a 0770 home, a widened `instances/<name>`, `bin/` or pinned copy) exits 1 before any log opens, with the home pair (the instance pair for an instance-level refusal), no line and no child (`tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log`).
- (.andromeda/test-plan.md:24) The tier is Comprehensive, so the race fix needs the full layer breadth: unit, integration and E2E.
  > **Test tier:** `Comprehensive (2)`
- (.andromeda/test-plan.md:51) The pinned-exe hash mismatch belongs to the `viola-state` entity and is testable in a temp home. Concurrent-pin cases must be held at that layer.
  > Unix modes and strict-modes, torn lines, snapshot replay, heartbeat staleness and pinned-exe hash mismatch are testable in a temp `--home`.
- (.andromeda/test-plan.md:67) The pinned exe and the `plugin/` files are one coverage-scope entity, the code-bearing state that this chunk's concurrency change touches.
  > - **Entity:** config and code-bearing state: `snapshot.json`, `config.json`, `settings.json`, `budget.json`, `ledger/stamps.json`, the `plugin/` files (`hooks.json` / `.mcp.json`), the pinned exe (SHA-256 16-hex re-hash), the `heartbeat` file
- (.andromeda/test-plan.md:135) Critical Path 1 (`run` start) must still exit 1 on a tampered pinned exe.
  > a second `run` of the same name, a squatted endpoint, a tampered pinned exe or a refused home tree exits 1.
- (.andromeda/test-plan.md:214) The filesystem vector coverage requires the hash-mismatch refusal and the per-start `plugin/` rewrite to both stay held.
  >     - pinned exe hash mismatch → exit 1
  >     - `plugin/` rewritten on start, and on Unix the instance's `settings.json` (none is written on Windows)
- (.andromeda/test-plan.md:254) The race tests must run on all three runners. The Windows exe arm, and whether it needs the companions' sharing-violation wait, is read on its own runner.
  > **Required test type:** full workspace and fake-agent E2E on all three runners.
- (.andromeda/test-plan.md:275) A race witness or control must key on logged events, never on sleeps.
  > Waits must key on logged events and byte offsets, not sleeps.
- (.andromeda/test-plan.md:309) There are no retries and no sleep-based synchronisation, so the race cannot be hidden by a retry.
  >   - no nextest retries (`retries = 0`) and no `std::thread::sleep` / `tokio::time::sleep` used for synchronisation
- (.andromeda/test-plan.md:421) The concurrent pin is a module ↔ on-disk boundary and must be tested over a real tempfile home. The concurrent-append check is the precedent for a concurrency assertion at this layer.
  > - **Module ↔ DB:** there is no database; module ↔ on-disk state is tested instead over real ndjson, snapshots and `.lock` siblings in a tempfile home, with a concurrent-append check that every line of every shared diagnostics file parses as one JSON object.
- (.andromeda/test-plan.md:431) Every integration home is fresh and not yet existing, so it reaches the absent-copy arm. Windows x64 homes have their ConPTY companions seeded before viola runs, so a test of the companions' race must start unseeded.
  > **Setup / teardown lifecycle:** a fresh tempfile home per test, at a not-yet-existing path under `target/e2e-home` that viola creates, so obs-plan's CI gates, the secret scan and the upload cover integration runs; started wrappers sit in a `Drop` guard that kills and waits, and nothing is shared between tests. Two carve-outs: Windows x64 starts seed the ConPTY companions before viola runs, into a home `seed_conpty` makes private
- (.andromeda/test-plan.md:452) Every new test that starts `viola` (each concurrent start) must drop the three `VIOLA_*` names and compare against `resolved(home)`.
  > - A test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, and a path oracle compares `resolved(home)`, never `TestHome::path()`.
- (.andromeda/test-plan.md:493) The Path 1 signal must hold after the fix: a tampered exe exits 1, and a restart rewrites `plugin/` with absolute hook commands.
  > a duplicate run, a squatted endpoint and a tampered pinned exe each exit 1; a restart rewrites `plugin/` with absolute hook commands, never a PATH lookup
- (.andromeda/test-plan.md:541) The Path 7 signal is the scenario whose unstamped two-instance boot showed the race. Whether it returns to two instances is P4's decision, and its signal must hold either way.
  > - **Verification signal:** on an unverified CLI `send` still confirms, the `question` event is logged once with an empty hook stdout, and `answer` exits 12 `unverified-cli` (MCP alike); `viola verify` at a fixture-stamped version exits 0 with its `stamped <ver>  <n> pass  0 fail` line.
- (.andromeda/test-plan.md:620) The pinned-exe content hash stays pinned by literal vectors. A fix to the pin's write must leave the hash and its oracle unchanged.
  > and the pinned-exe content hash is pinned to its library by literal vectors.
- (.andromeda/test-plan.md:634) The pinned copy in any race test must come from real `viola` runs, starting from `stamped_home` where a viola-made home is needed. It is never hand-placed.
  > - **On-disk product state:** produced only by real `viola` runs against the fake agent in a fresh home, never hand-written — except the chaos tests' corruption of files the product already wrote, the Windows x64 `seed_conpty` companions and the server-identity impostors' test-built `snapshot.json`; a case that needs a home viola created starts from `stamped_home`
- (.andromeda/test-plan.md:661) The fake agent spawns the absolute pinned path directly. That direct spawn is what fails (`ran:false`) under the race, so it is the witness's signal surface.
  > It reads the hook commands from the plugin dir `run` passes and runs every command entry for the event: an absolute `command` is spawned directly, never through a shell or a PATH lookup, and a non-absolute one is never run
- (.andromeda/test-plan.md:760) The reproducible-build measurement goes through `scripts/release-check.sh`'s build. That gate judges a `--release` build of `viola` alone and must stay as it is.
  > **Release build:** `scripts/release-check.sh`, with its probe, judges a `--release` build of `viola` alone and fails on a test-only feature or binary; no upload and no gate step.
- (.andromeda/test-plan.md:795) Any build change the measurement leads to (a founder fork) must keep both release-check endings.
  > - Release-build errors: `scripts/release-check.sh` not ending `release-check: viola only`, or its probe not ending `release-check probes: 5/5 refused, control clean`
- (.andromeda/test-plan.md:816) The new pin code must meet the per-OS coverage gates.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`.
- (.andromeda/test-plan.md:820) Concurrent `viola run` children spawned by the race tests count toward coverage. A test that clears the env must re-add the profile variable.
  > - Child `viola` processes spawned by E2E tests are included, because `LLVM_PROFILE_FILE` propagates; tests that call `env_clear()` must re-add it
- (.andromeda/test-plan.md:854) The race is a measured 6-of-24 intermittent red, so the zero-flake rule applies. It stays red within this chunk until its root cause is fixed, and it is never `#[ignore]`d.
  > For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:876) A Linux-host reproduction of the race, or of the build measurement, is not proof for Windows or macOS.
  > - NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:898) The witness must wait on explicit signals.
  > NEVER use `sleep(N)` for synchronization. Wait for an explicit signal: an `events.ndjson` offset, `viola wait --after` or a status field.
- (.andromeda/test-plan.md:908) A concurrent-pin control must not hand-write `stamps.json` or `snapshot.json`.
  > NEVER hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`. Stamps come from `viola verify`, snapshots from the wrapper and budget from `viola hook statusline`. The ban binds every booted-wrapper test.
- (.andromeda/test-plan.md:918) The witness must run the real pinned binary through the plugin files. Stubbing the hook would remove the very spawn that races.
  > NEVER stub `viola hook` in E2E. The fake agent must invoke the real pinned binary through the plugin files `run` wrote.
- (.andromeda/test-plan.md:922) The race must be closed in the product. Serialising tests is no fix.
  > NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.
- (.andromeda/test-plan.md:928) No retry budget may mask the race.
  > NEVER set nextest `retries` above 0.
- (.andromeda/test-plan.md:946) A concurrent-start control must not rest on a test-owned timer or an elapsed-time verdict.
  > NEVER use real time without injection (use deterministic clocks) in test-side code: no test-owned timer, sleep or elapsed-time verdict.
- (.andromeda/test-plan.md:952) A narrower concurrent-pin fault control must come from explicit process or file operations, or from existing mockall seams.
  > chaos faults come only from explicit process / file operations or existing mockall seams

## Bindings
- **boot readiness ↔ the body's Path 7 scenario**: the scope's standing workaround (Path 7 boots one instance) stands on the boot's readiness rule, which requires a `session-start` per instance. Whether Path 7 returns to two instances is P4's decision (.andromeda/test-plan-detail/test-scope-summary.md:80) (.andromeda/test-plan.md:541)
- **run start sequence ↔ security rules (Home: re-hash before reuse)**: the pinned copy's place in the start order and its exit-1 refusals pair with the unchanged hash-before-reuse rule (.andromeda/test-plan-detail/test-scope-summary.md:123) (.andromeda/test-plan-detail/test-scope-summary.md:129) (.andromeda/test-plan.md:620)
- **release build gate ↔ the reproducible-build measurement (CARRY 11)**: the measurement uses `release-check.sh`'s build, and any change acted on must keep its two endings (.andromeda/test-plan.md:760) (.andromeda/test-plan.md:795)
- **zero-flakiness budget ↔ the race witness**: the two-sided witness must be red before the fix and clean after it, with no retry, sleep or serialisation (.andromeda/test-plan.md:854) (.andromeda/test-plan.md:922) (.andromeda/test-plan.md:928)
- **seed strategies ↔ test homes**: a fresh, private home made by viola reaches the absent-copy arm, while Windows x64 companions are seeded (.andromeda/test-plan-detail/test-scope-summary.md:117) (.andromeda/test-plan.md:431) (.andromeda/test-plan.md:634)
