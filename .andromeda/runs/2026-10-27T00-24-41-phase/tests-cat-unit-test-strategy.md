# tests · Unit Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/unit-test-strategy.md:28) The pin's existing unit cases must keep passing unchanged across the absent-copy write's concurrency fix: the content key stays the version and the content hash, and a refused companion path still writes nothing.
  > Stamps and pin: `update_stamps_refuses_a_widened_ledger_and_leaves_it` (`StampsError::StrictModes`, checked before `create_private_dir(ledger/)` and again under the lock, nothing read or written), `stamps_error_messages_are_fixed`, `pin_key_is_the_version_and_the_content_key` and `pin_companions_refuses_a_path_another_user_may_write` (`PinError::StrictModes`, nothing written).
- (.andromeda/test-plan-detail/unit-test-strategy.md:27) The home-tree check covers the pinned copy's `bin/<key>/` and `plugin/<key>/`, so a tree written by a concurrent first start must still pass the viola-wrote-it case, and each widened path must still be refused.
  > The home-tree and diagnostics checks: `check_home_tree_of_a_tree_viola_wrote_passes`, `check_home_tree_refuses_each_widened_path`,
- (.andromeda/test-plan-detail/unit-test-strategy.md:68) Any new root-bin unit case that captures a log line, for example a pin or race outcome logged by `run`'s start, must use the shared scoped-capture helper and never a bare `with_default`.
  > Scoped log captures: every scoped capture in the root bin's unit modules goes through `crate::test_support::with_scoped_default` (`src/main.rs`), which registers once per process a pinned, always-interested `tracing::Dispatch` over a bare `tracing_subscriber::registry()` (no layer, no filter) in a `static OnceLock`, never the global default, then runs `tracing::subscriber::with_default`;
- (.andromeda/test-plan.md:382) viola-state's unit coverage must keep the pin strict-modes refusals and the home-tree check alongside any new concurrent-pin case in `pin.rs` / `fs.rs`.
  > the home-tree and diagnostics checks, `live_endpoint` over a replayed snapshot, and the stamps and pin strict-modes refusals.
- (.andromeda/test-plan.md:289) New unit cases for the pin race must be inline `#[cfg(test)]` modules that run on all three OSes, with any table written as rstest `#[case]` rows.
  > inline `#[cfg(test)] mod tests` in all six crates and the root bin, with rstest `#[case]` tables for the refusal-order, exit-code and fixture-replay matrices, on all three OSes except the two real-cargo-mutants harness tests and the server check's captured-span cases, compiled out on macOS.
- (.andromeda/test-plan.md:359) New pin and replace tests must follow the location, naming and labelled-case conventions.
  > Test file location: inline `#[cfg(test)] mod tests` in the same source file, in every `crates/viola-<area>` and in root `src/`.
  > - Test function naming: `<subject>_<condition>_<expected>` in snake_case.
  > - Test grouping: flat modules per source file. Table-driven cases use rstest `#[case]`, and each case name must be a readable label (for example `#[case::esc_refused]`).
- (.andromeda/test-plan.md:364) A shared temp home for the concurrent-pin cases must be built as an rstest fixture.
  > rstest 0.27.0 `#[fixture]` for shared constructors, for example a `TempHome` or a `Clock`.
- (.andromeda/test-plan.md:392) A race witness at unit level may use doubles only for seam traits the product already defines. No new mock seam may be introduced just for the pin.
  > - Pure functions
  > - Module logic with mocked dependencies (mockall doubles only for traits the product already defines at its seams: `Pty`, liveness probe, `Clock`)
- (.andromeda/test-plan.md:51) The pinned-exe hash mismatch stays testable in a temp home, and the hash-before-reuse refusal must be asserted beside the race fix, never weakened by it.
  > Unix modes and strict-modes, torn lines, snapshot replay, heartbeat staleness and pinned-exe hash mismatch are testable in a temp `--home`.
- (.andromeda/test-plan.md:214) The filesystem vector's negative case must keep holding after the change: a present copy that differs is refused, never overwritten.
  > pinned exe hash mismatch → exit 1
- (.andromeda/test-plan.md:620) The pinned-exe content hash stays pinned to its library by literal vectors. A rebuild-reproducibility measurement or fix must not change the hash oracle.
  > and the pinned-exe content hash is pinned to its library by literal vectors.
- (.andromeda/test-plan.md:710) The race must be exercised over a real tempfile filesystem (rename, open handles and modes are the behaviour under test), never a mocked filesystem.
  > Filesystem: not mocked. A real tempfile home is used everywhere, because modes, `File::lock`, torn lines and symlink handling are the behaviour under test.
- (.andromeda/test-plan.md:917) The concurrent-pin control's fault injection must come from process and file faults or existing mockall seams. No failpoint crate is allowed in `viola-state`.
  > NEVER introduce a failpoint crate that pulls tokio into the sync crates (for example fail-parallel). Fault seams go through mockall doubles of existing traits plus process and file faults.
- (.andromeda/test-plan.md:309) The two-sided race witness must synchronise its concurrent starts without sleeps and must pass with zero retries.
  > no nextest retries (`retries = 0`) and no `std::thread::sleep` / `tokio::time::sleep` used for synchronisation
- (.andromeda/test-plan.md:946) The race test may not use a test-owned timer or an elapsed-time verdict.
  > NEVER use real time without injection (use deterministic clocks) in test-side code: no test-owned timer, sleep or elapsed-time verdict.
- (.andromeda/test-plan.md:854) A race witness that passes only sometimes is a flake: it keeps the chunk red until its root cause is fixed, and it is never parked with `#[ignore]`.
  > flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:922) Concurrent-pin tests must take their isolation from per-test homes, never from serialising the suite.
  > NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.
- (.andromeda/test-plan.md:880) Pin race tests must assert observable behaviour (the copy's bytes and hash, a spawnable path, no failed replace) rather than private internal state.
  > - NEVER test implementation details (private methods / internal state):
  >   test public API + observable behavior
- (.andromeda/test-plan.md:876) A Windows arm of the exe pin (sharing violation or same-bytes acceptance) is proven only by `windows-2025` CI, never by a Linux reading.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:816) New pin code, and any `#[cfg(windows)]` arm of it, must stay inside the per-OS coverage gate.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:795) Any release-profile or build change the founder takes on the reproducibility measurement must keep the release check and its probe ending as written.
  > Release-build errors: `scripts/release-check.sh` not ending `release-check: viola only`, or its probe not ending `release-check probes: 5/5 refused, control clean`

## Bindings
- **Pin unit cases ↔ security.md §Local trust boundary (Home)**: the re-hash before reuse and "never overwritten" rule that `pin_exe` holds, which this chunk may not change, is pinned at the unit level by the pin cases and the hash-mismatch negative (.andromeda/test-plan-detail/unit-test-strategy.md:28), (.andromeda/test-plan.md:214).
- **Home-tree unit cases ↔ architecture `check_home_tree` (`bin/<key>/`, `plugin/<key>/`)**: a concurrently pinned tree must still pass and a widened one must still be refused (.andromeda/test-plan-detail/unit-test-strategy.md:27).
- **Race witness ↔ §10 zero-flakiness and §11 timing bans**: the two-sided witness for CARRY 16 is held to zero retries, no sleeps and no test-owned timer (.andromeda/test-plan.md:854), (.andromeda/test-plan.md:309), (.andromeda/test-plan.md:946).
- **Pinned-exe hash vectors ↔ CARRY 11 measurement**: the contract suite's literal vectors fix the hash function, while the measurement reads only whether two clean builds give one file (.andromeda/test-plan.md:620).
- **Reproducible-build change ↔ §9 release-check gate**: a founder-taken build change stays under the release check's endings (.andromeda/test-plan.md:795).
- **Windows exe arm ↔ §9 matrix `windows-2025`**: proof comes from the CI runner only, as the scope's Boundaries also state (.andromeda/test-plan.md:876).
