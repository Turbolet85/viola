# tests · Mocking & Stubbing Discipline — category read

## Carried
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19) The chunk's first-pipe-instance property test (b) and the re-verified squatted-name start refusal must take their squatter from interprocess's own server API, bound to the computed endpoint before `run`, never from a hand-made fake.
  > | Fake servers | interprocess 2.4.4 server API (squatter listener); `viola-fake-agent` as the fake CLI | The squatter binds the computed endpoint before `run`; no HTTP fake servers |
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18) Any new or re-verified Windows admission test that needs a child (the squat refusal's `run`) must use the real stub executable, resolved through `CARGO_BIN_EXE_viola-fake-agent` in root `tests/` only, and a mockall double only on an existing seam trait.
  > | Process mocking | mockall 0.15.0 on the `viola-pty` `Pty` trait and the liveness / `Clock` traits; native stub binaries (`viola-fake-agent`) | `#[cfg_attr(test, mockall::automock)]` on seam traits only; external processes are real stub executables, resolved through `CARGO_BIN_EXE_viola-fake-agent` in root `tests/` only.
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8) The budget CARRY's fix (`send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4, its log capture under libtest threads) must keep the budget gate's instant handed to the pure decision and add no new direct clock read or clock seam for it.
  > `chrono::Utc::now()` is read only through the clock seam, with one exception as landed: the budget gate's three evaluation points read it directly and hand the instant to the pure decision
- (.andromeda/test-plan.md:707) The DACL-effect assertion (c) must not be stubbed: the only stubbed descriptors are the strict-modes check's synthetic SDDL ones, so a pipe-DACL effect is either measured on a real bound pipe on the Windows runner or left to the Windows-only live measurement.
  > - Filesystem: not mocked. A real tempfile home is used everywhere, because modes, `File::lock`, torn lines and symlink handling are the behaviour under test. Only the Windows foreign-SID / non-persistent-ACL descriptors are stubbed, as synthetic SDDL descriptors fed to the check function.
- (.andromeda/test-plan.md:722) The listener tests (remote-client rejection, first instance, DACL) must exercise the real interprocess / Windows pipe, with mockall only on seam traits viola already has.
  > - **Process mocking:** mockall `automock` on seam traits only; external processes are real stub executables (`viola-fake-agent`).
- (.andromeda/test-plan.md:724) The squatter for the held-name cases is the interprocess server API, bound before `run`.
  > - **Fake servers:** the interprocess server API binds the squatter before `run`, and `viola-fake-agent` is the fake CLI; no HTTP fake servers.
- (.andromeda/test-plan.md:726) The chunk must not introduce a new trait or runtime patch in `viola-channel` solely to double the pipe's mode, DACL or first-instance flag for a test; doubles exist only for seams viola already defines.
  > **Anti-monkey-patching:** prefer dependency injection over runtime patching of production code. Doubles exist only for traits viola already defines at its seams (research, mockall key detail).
- (.andromeda/test-plan.md:910) The `PIPE_REJECT_REMOTE_CLIENTS` assertion (a) must read the real pipe's mode back rather than mock interprocess or windows-sys, must not over-stub, and any fault seam added in the sync `viola-channel` must not pull a tokio-bearing failpoint crate.
  > - NEVER mock what you don't own (libraries / language runtime) without
  >   abstraction layer
  > - NEVER over-stub (you stop testing real behavior)
  > - NEVER monkey-patch production code at runtime (use dependency injection)
  > - NEVER introduce a failpoint crate that pulls tokio into the sync crates (for example fail-parallel). Fault seams go through mockall doubles of existing traits plus process and file faults.

## Bindings
- **Fake servers (squatter) ↔ body §5 / §11 Integration / §6 Path 1**: the squatter from the server API (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19) is the same one §5 creates "first through the server API (never `try_overwrite`)" (.andromeda/test-plan.md:421) under the ban on `try_overwrite` and on reusing an endpoint name across tests (.andromeda/test-plan.md:889), and its verdict is Path 1's "a squatted endpoint … exit 1" (.andromeda/test-plan.md:488).
- **Filesystem stub carve-out ↔ body §1 / §6 untestable zones**: the SDDL-descriptor stub carve-out (.andromeda/test-plan.md:707) is the only stub; the cross-user parts (remote pipe clients, the Windows pipe DACL) are "asserted at the peer-credential check boundary" (.andromeda/test-plan.md:46) and covered by "same-user SDDL read-back (§5)" (.andromeda/test-plan.md:476), so scope item (c) beyond what the runner measures stays a live measurement, not a mock.
- **Mock-what-you-don't-own ↔ body §5 drivers**: the remote-rejection and DACL read-backs use the real pipe through the §5 driver "Windows DACL read-back: windows-sys 0.61.2" (.andromeda/test-plan.md:411), never a double of it (.andromeda/test-plan.md:910).
- **Time exception ↔ budget CARRY ↔ §10 zero-flakiness / §11 CI**: the CARRY's fix keeps the gate's direct instant read (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:8) and is fixed at its root cause, never by retry or serialisation (.andromeda/test-plan.md:851, .andromeda/test-plan.md:919).
