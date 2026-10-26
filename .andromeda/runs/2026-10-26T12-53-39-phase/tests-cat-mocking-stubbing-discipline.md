# tests · Mocking & Stubbing Discipline — category read

## Carried
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8) The `link` method's `since` instant and any served `link` / `unlink` time read must come through the clock seam; the budget gate's three points are the only landed exception and this chunk may not add another.
  > `chrono::Utc::now()` is read only through the clock seam, with one exception as landed: the budget gate's three evaluation points read it directly and hand the instant to the pure decision (as read in `src/run/send.rs` and `src/cmd/run.rs` at the chunk 2026-10-10-budget-governor wrap; that read named the gate's call sites only and was no census of `Utc::now()`).
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:17) A unit case over the `link` result's `since` (or the `already linked` / `since …Z` result lines) must inject the `Clock` or hand the pure step its instant, never read wall time.
  > Inject the `Clock` dependency through constructors and call `MockClock::advance(Duration::from_secs(6))`; or hand a pure decision its instant as an argument, as the budget gate's `decide` takes `chrono::DateTime<Utc>`
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18) The verbs' root cases (`tests/channel_links.rs`, the `tests/cli_links.rs` cases, Scenario E3) must boot wrappers over the real `viola-fake-agent` stub executable, mockall only on existing seam traits, and any held process an impostor snapshot names must be a real process reaped by a `Drop` guard.
  > `#[cfg_attr(test, mockall::automock)]` on seam traits only; external processes are real stub executables, resolved through `CARGO_BIN_EXE_viola-fake-agent` in root `tests/` only. `viola-e2e` tests and the harness use the assert_cmd target-dir lookup (§3 boot step 5); a held live process whose pid an impostor's snapshot records is a real process, never a double: in root tests the fake agent's `--hold-stdout-internal` holder, in `crates/viola-mcp/tests/handlers.rs` its own `Held` child, entered at `handlers_held_child_entry` only when its `HELD` variable is set, each killed and reaped by a `Drop` guard
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19) If `viola link` / `viola unlink` join the CLI frame verbs that take the server check through `client::connect`, their server-verification negative must use the established impostor pattern: a test-owned `viola_channel::Server::bind` listener on the recorded endpoint while the snapshot records another held live process.
  > in the server-verification negatives (Linux and Windows) an impostor, a server on the recorded endpoint while the snapshot records another held live process (Process mocking), stands in for a server that is not the recorded wrapper (root tests bind a test-owned `viola_channel::Server::bind` listener: `tests/security_negatives_server_identity.rs`, `tests/hook_fail_open.rs`'s `Impostor`; the MCP case is `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`)
- (.andromeda/test-plan.md:703) The driven and driver instances in the link tests must run the native fake agent as their PTY child, never the real `claude`.
  > External services not under test: the real `claude` CLI is replaced by the native `viola-fake-agent` binary, both as the PTY child and as `claude` on a test-scoped PATH.
- (.andromeda/test-plan.md:707) The `link` / `unlink` lines and the snapshot's `links` must be asserted over a real tempfile home, never a mocked filesystem.
  > Filesystem: not mocked. A real tempfile home is used everywhere, because modes, `File::lock`, torn lines and symlink handling are the behaviour under test.
- (.andromeda/test-plan.md:713) Scenario E3 and the verbs' E2E steps must drive the real wrapper channel, never a stubbed one.
  > The wrapper channel, PTY, vt100 or serde in E2E. Code paths the agent must verify end-to-end go in E2E (§6).
- (.andromeda/test-plan.md:726) The `unlink` product path ending `LinkSet::remove`'s `#[cfg_attr(not(test), allow(dead_code))]` must be reached by a real caller under test, not patched in; doubles only for traits viola already defines.
  > **Anti-monkey-patching:** prefer dependency injection over runtime patching of production code. Doubles exist only for traits viola already defines at its seams (research, mockall key detail).
- (.andromeda/test-plan.md:389) Unit cases of the wrapper's `link` / `unlink` handlers may double only the `Pty`, liveness and `Clock` seams.
  > Module logic with mocked dependencies (mockall doubles only for traits the product already defines at its seams: `Pty`, liveness probe, `Clock`)
- (.andromeda/test-plan.md:870) The channel-method integration cases must not over-mock the wrapper or the link set.
  > NEVER over-mock at integration level (you stop testing the integration)
- (.andromeda/test-plan.md:886) The link log lines and snapshot must not be mocked in integration cases.
  > NEVER mock the on-disk state (the arch has no DB; this is the DB-equivalent rule). Use a real tempfile home.
- (.andromeda/test-plan.md:912) The link tests must not over-stub.
  > NEVER over-stub (you stop testing real behavior)
- (.andromeda/test-plan.md:914) No failpoint crate may be added for link fault cases; faults go through existing mockall seams plus process and file faults.
  > NEVER introduce a failpoint crate that pulls tokio into the sync crates (for example fail-parallel). Fault seams go through mockall doubles of existing traits plus process and file faults.
- (.andromeda/test-plan.md:915) The booted wrappers of Scenario E3 must keep the real pinned `viola hook` behind the fake agent.
  > NEVER stub `viola hook` in E2E. The fake agent must invoke the real pinned binary through the plugin files `run` wrote.
- (.andromeda/test-plan.md:944) No test of the `since` instant may steer a spawned wrapper's time with an in-process clock.
  > NEVER use mock_instant `MockClock` (or any in-process clock) to control another process's time. Global mode reaches only threads of the test process, never a spawned `viola run`.

## Bindings
- **Fake servers (impostor) ↔ body §6 Security control negatives**: the impostor listener of the server-verification negatives (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19) is the double the suite naming "raw-channel `link` / `unlink` name validation" and the CLI frame verb writing no frame to a non-recorded server uses (.andromeda/test-plan.md:606); a `link` / `unlink` verb case lands in `tests/security_negatives_server_identity.rs` with it.
- **Process mocking (held live process) ↔ body §7 Seed strategies**: the impostor's held process (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18) pairs with the carve-out letting the server-identity impostors' test-built `snapshot.json` be hand-written (.andromeda/test-plan.md:631); no other link test hand-writes a snapshot (.andromeda/test-plan.md:905).
- **Time (clock seam) ↔ body §4 root bin**: the `since` read through the clock seam (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8) is held in a clock-driven root-bin unit case (.andromeda/test-plan.md:386), never by an E2E wall-clock verdict (.andromeda/test-plan.md:943).
- **Process mocking (fake agent) ↔ body §6 Scenario E3**: E3's booted `overseer` and `builder` (.andromeda/test-plan.md:562) run over the fake agent as the stub CLI (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18).
