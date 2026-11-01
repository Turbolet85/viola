# tests · Mocking & Stubbing Discipline — category read

## Carried
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:21) A socket directory loosened, wrong-moded or symlinked for this chunk's refusal negatives (the 0755 macOS exit-cause case among them), and the home widened for the board's one rule, must be made on the real path with real modes and refused by the product's own check, never by a stubbed check.
  > none (a real tempfile home, real modes and ACLs) | A widened or read-granted home tree is made on the real file by the §7 widened-test-home helpers and refused by the product's own check, never a stubbed one:
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:21) A directory-verification leg no real file on a one-account runner can show (a foreign owner) must be held by a pure verdict fed synthetic values at the unit tier, the form this row grants, and never by a double of the check at a higher tier.
  > the pure verdicts (`windows_read_verdict` among them) take synthetic descriptors at the unit tier;
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:20) The squatter stays a real listener binding the computed endpoint before `run`, so a squatter negative must keep reading its refusal beside the directory check and the takeover of a leftover socket file.
  > The squatter binds the computed endpoint before `run`;
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:20) The impostor is a real test-owned listener and is carved to Linux and Windows, so the macOS carve-out must take no impostor negative that reads as a pid comparison, and any server stood up for the client-side euid control must be a real `viola_channel::Server::bind` listener, never a double.
  > in the server-verification negatives (Linux and Windows) an impostor, a server on the recorded endpoint while the snapshot records another held live process (Process mocking), stands in for a server that is not the recorded wrapper (root tests bind a test-owned `viola_channel::Server::bind` listener: `tests/security_negatives_server_identity.rs`, `tests/hook_fail_open.rs`'s `Impostor`;
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19) The killed wrapper behind the leftover-socket takeover and the harness `cleanup` measurement must be a real wrapper under a real kill, with no double standing in for the lost process or its left file.
  > none (a real wrapper, a real signal or kill) | A stale holder is a real wrapper under the `Frozen` guard (`tests/cli_instance_state.rs`; on Unix a `SIGSTOP`), which `run_refuses_a_stale_name` stands on; a wrapper lost under an open call is a real kill, `support::start::kill_wrapper` (`tests/support/start.rs`) for the lost row of `tests/cross_exit_causes.rs` and for `tests/chaos_wait_vanish.rs`, `kill_by_pid` (`crates/viola-e2e/tests/mcp_tools.rs`) for `mcp_tools_a_wrapper_killed_under_a_parked_wait_reads_endpoint_lost`; no double stands in for either
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18) The peer euid and directory checks must add no mockall double outside a seam trait, and any wrapper this chunk's tests start must run the real stub executable through the resolution its test tree is allowed.
  > `#[cfg_attr(test, mockall::automock)]` on seam traits only; external processes are real stub executables, resolved through `CARGO_BIN_EXE_viola-fake-agent` in root `tests/` only. `viola-e2e` tests and the harness use the assert_cmd target-dir lookup (§3 boot step 5);
- (.andromeda/test-plan.md:46) The peer of another euid must be asserted at the peer-credential check boundary, since hosted runners cannot drive it, and no test may claim a live cross-user reading.
  > only the cross-user part (a peer of another euid) cannot be driven on hosted runners and is asserted at the peer-credential check boundary.
- (.andromeda/test-plan.md:380) The both-sides euid mismatch must be held as a decision taking injected euids at the unit tier, the wrapper side and the client side each, rather than through a new double.
  > the peer-credential decision with injected euids
- (.andromeda/test-plan.md:396) Unit doubles for this chunk's new checks must stay to the seams the product already defines, so a new trait made only to be mocked is out.
  > - Module logic with mocked dependencies (mockall doubles only for traits the product already defines at its seams: `Pty`, liveness probe, `Clock`)
- (.andromeda/test-plan.md:484) The cross-user case stands skipped at E2E, so this chunk must not build an E2E stand-in for a peer of another euid.
  > - Cross-OS-user peers. Covered by the peer-credential boundary unit test (§4) and same-user SDDL read-back (§5).
- (.andromeda/test-plan.md:715) Unix socket-directory and home modes, owner and symlink readings must run on the real filesystem, the stub exception staying Windows' alone, and this chunk moves no Windows behaviour.
  > - Filesystem: not mocked. A real tempfile home is used everywhere, because modes, `File::lock`, torn lines and symlink handling are the behaviour under test. Only the Windows foreign-SID / non-persistent-ACL descriptors are stubbed, as synthetic SDDL descriptors fed to the check function.
- (.andromeda/test-plan.md:721) The peer euid check on the channel path must be crossed unmocked in any E2E case of this chunk, the Linux and macOS parity suite among them.
  > - The wrapper channel, PTY, vt100 or serde in E2E. Code paths the agent must verify end-to-end go in E2E (§6).
- (.andromeda/test-plan.md:734) Forcing a refused directory or an euid mismatch must go through injection, never a runtime patch or a test-only switch in production code.
  > **Anti-monkey-patching:** prefer dependency injection over runtime patching of production code. Doubles exist only for traits viola already defines at its seams (research, mockall key detail).
- (.andromeda/test-plan.md:894) The socket directory and home state under test must be real files, the integration tier included.
  > - NEVER mock the on-disk state (the arch has no DB; this is the DB-equivalent rule). Use a real tempfile home.
- (.andromeda/test-plan.md:897) The squatter and the leftover-file tests must not use `try_overwrite` on the test side or reuse an endpoint name, and a test must take the endpoint from `snapshot.json`.
  > - NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests. Take the endpoint only from `snapshot.json`.
- (.andromeda/test-plan.md:920) The chunk's fault cases must not over-stub, patch production code or bring in a failpoint crate, its fault seams being real process and file operations.
  > - NEVER over-stub (you stop testing real behavior)
  > - NEVER monkey-patch production code at runtime (use dependency injection)
  > - NEVER introduce a failpoint crate that pulls tokio into the sync crates (for example fail-parallel). Fault seams go through mockall doubles of existing traits plus process and file faults.
- (.andromeda/test-plan.md:923) A refusal of this chunk reaching `hook` as a fail-open must be asserted on the real pinned binary in E2E, never on a stubbed hook.
  > - NEVER stub `viola hook` in E2E. The fake agent must invoke the real pinned binary through the plugin files `run` wrote.
- (.andromeda/test-plan.md:957) Any chaos case added for the killed wrapper's leftover socket must take its fault only from an explicit process or file operation or an existing mockall seam.
  > chaos faults come only from explicit process / file operations or existing mockall seams (§2 Property-based and Chaos rows).

## Bindings
- **Filesystem faults ↔ §7 Test Data & Fixtures**: the real widened file is made by §7's widened-test-home helpers, and a test that makes a home itself makes it through `create_private_dir`; both bind a loosened-directory or widened-home negative, (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:21) with (.andromeda/test-plan.md:639).
- **Filesystem faults ↔ §6 Security control negatives and the Exit-cause matrix**: the Unix socket directory refusal's negative and its `tests/cross_exit_causes.rs` row must read a real loosened directory refused by the product, (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:21) with (.andromeda/test-plan.md:612) and (.andromeda/test-plan.md:610).
- **Fake servers ↔ §6 Exit-cause matrix (the macOS case owed to this entry)**: the server-verification impostor stands on Linux and Windows while the matrix names a macOS case owed here, so the macOS leg must be held without an impostor reading as a comparison, (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:20) with (.andromeda/test-plan.md:610).
- **Peer-credential boundary ↔ §4 viola-channel and §1 Untestable zones**: the injected-euid unit case is the only holder of a peer of another euid, because hosted runners provide one account, (.andromeda/test-plan.md:380) with (.andromeda/test-plan.md:88).
- **Process faults ↔ §6 Chaos suite**: a real kill for the leftover socket sits beside the suite's endpoint-vanish case, (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19) with (.andromeda/test-plan.md:620).
- **`viola hook` unstubbed ↔ §6 Security sweep**: the hook fail-open matrix takes this chunk's refusals on the real binary, exit 0 with no body, (.andromeda/test-plan.md:923) with (.andromeda/test-plan.md:604).
