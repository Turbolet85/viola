# tests · Mocking & Stubbing Discipline — category read

## Carried
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:20) The race fix's tests, the absent-copy write under a concurrent first start and the `plugin/<key>/` rewrite included, must run on the real file in a real tempfile home with no stubbed filesystem or rename, and any refusal they read must be the product's own check.
  > | Filesystem faults | none (a real tempfile home, real modes and ACLs) | A widened or read-granted home tree is made on the real file by the §7 widened-test-home helpers and refused by the product's own check, never a stubbed one
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18) The two-sided witness (the unstamped two-instance boot or a narrower concurrent-pin control) must use real processes for its concurrent starts and the agent's hook spawn, through the fake agent resolved by the named lookup for its layer, and must add no mockall double outside an existing seam trait.
  > `#[cfg_attr(test, mockall::automock)]` on seam traits only; external processes are real stub executables, resolved through `CARGO_BIN_EXE_viola-fake-agent` in root `tests/` only. `viola-e2e` tests and the harness use the assert_cmd target-dir lookup (§3 boot step 5)
- (.andromeda/test-plan.md:706) The real `claude` that spawns the pinned hook path in the race must be stood in for by `viola-fake-agent`, never the real CLI.
  > - External services not under test: the real `claude` CLI is replaced by the native `viola-fake-agent` binary, both as the PTY child and as `claude` on a test-scoped PATH.
- (.andromeda/test-plan.md:710) The pin race is filesystem behaviour under test, so no test may mock the filesystem to reach or close the race window.
  > - Filesystem: not mocked. A real tempfile home is used everywhere, because modes, `File::lock`, torn lines and symlink handling are the behaviour under test.
- (.andromeda/test-plan.md:715) An integration-tier concurrent-pin control must read and write the real `bin/<key>/` and `plugin/<key>/` files.
  > - On-disk state at integration level (the arch has no DB). Use real files in a tempfile home.
- (.andromeda/test-plan.md:717) The failure the witness must show red then clean is the fake agent's spawn of the real pinned binary, so the E2E witness must keep that spawn real.
  > - `viola hook` invocations inside E2E. The fake agent calls the real pinned binary.
- (.andromeda/test-plan.md:918) The E2E witness must not stub `viola hook`: the hook spawn of the pinned path through the plugin files `run` wrote is the race's observed failure.
  > - NEVER stub `viola hook` in E2E. The fake agent must invoke the real pinned binary through the plugin files `run` wrote.
- (.andromeda/test-plan.md:725) Any process double the fix's tests need is a mockall `automock` on an existing seam trait; external processes stay real stub executables.
  > - **Process mocking:** mockall `automock` on seam traits only; external processes are real stub executables (`viola-fake-agent`).
- (.andromeda/test-plan.md:729) A narrower concurrent-pin control must reach the race window by dependency injection or real concurrency, never by runtime patching, and must define no new trait only to stand a double in for the write or the rename.
  > **Anti-monkey-patching:** prefer dependency injection over runtime patching of production code. Doubles exist only for traits viola already defines at its seams (research, mockall key detail).
- (.andromeda/test-plan.md:915) The race control must not over-stub `pin_bytes` / `pin_exe` / `replace_private_shared` so far that the real concurrency is no longer what passes.
  > - NEVER over-stub (you stop testing real behavior)
- (.andromeda/test-plan.md:916) No test may monkey-patch the pin or replace code at runtime to widen or force the race.
  > - NEVER monkey-patch production code at runtime (use dependency injection)
- (.andromeda/test-plan.md:917) A fault or race seam in the sync `viola-state` crate (`pin.rs`, `fs.rs`) must not come from a failpoint crate that pulls tokio in; it must go through existing mockall seams or real process and file faults.
  > - NEVER introduce a failpoint crate that pulls tokio into the sync crates (for example fail-parallel). Fault seams go through mockall doubles of existing traits plus process and file faults.

## Bindings
- **Process mocking (fake-agent lookup) ↔ the body's §3 harness contract**: the two-instance boot witness boots through the harness, whose fake-agent lookup is §3 boot step 5's assert_cmd target-dir lookup, a root-test control resolving `CARGO_BIN_EXE_viola-fake-agent` instead, (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18)
- **The real pinned hook spawn in E2E ↔ the body's §6 Path 7 / Path 1**: the CARRY's test-side workaround (Path 7 boots one instance) and Path 1's pinned-copy leg rest on the fake agent invoking the real pinned binary through the plugin files `run` wrote; whether Path 7 returns to two instances is P4's, (.andromeda/test-plan.md:717) and (.andromeda/test-plan.md:918)
- **Real tempfile home ↔ the body's §5 lifecycle (Windows x64 `seed_conpty`)**: a Windows concurrent-first-start control runs on a real home, and on Windows x64 the root tests seed the ConPTY companions before viola runs, so the exe arm's race is read beside `pin_companions`' seeded companions, (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:20)
- **Failpoint ban ↔ the security plan's tokio ban on sync crates (`deny-sync.toml`, `scripts/sync-crates.txt`)**: a race seam added to `viola-state` must leave its sole-root tokio ban green, (.andromeda/test-plan.md:917)
