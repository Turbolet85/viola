# tests · Test Scope Summary — category read

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:8) The chunk's surface is the `viola-channel` entity, which the plan holds partially testable; its new Windows listener assertions are scoped under that entity.
  > - **Entity:** `viola-channel`: JSON-RPC 2.0 over ndjson on `interprocess` local sockets. Covers the sync client/server, the Tokio client feature, the FNV-1a endpoint hash, `MAX_FRAME` (16 MiB) and the result/refusal/error shapes
  >   - **Source:** arch excerpt, Workspace / Modules and Standard Contracts (Wrapper channel, Endpoint); security excerpt, Vector 1
  >   - **Testability:** partially-testable
- (.andromeda/test-plan-detail/test-scope-summary.md:11) The squatted-name exit-1 start refusal is a same-user testable behaviour and must stay asserted as such (the chunk keeps `squatted-name`, exit 1).
  > same-user behaviour is testable: framing, method dispatch, refusal order, `-32602` on a higher `v`, the pid/start-time server verification, and `run` exiting 1 on a squatted name.
- (.andromeda/test-plan-detail/test-scope-summary.md:11) Remote-client rejection and the DACL's denial effect (scope items (a) and (c)) cannot be driven cross-user on hosted runners: they must be asserted at the check boundary (the pipe's mode read back, the DACL read back canonical as two FA allow ACEs), with stubs where a runner cannot provision the principal.
  > The cross-user parts cannot be driven on hosted GitHub runners, which have no second OS account: rejecting a peer with a different euid, rejecting remote pipe clients, and denying access through the Windows pipe DACL (set from `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, read back as `D:P(A;;FA;;;<sid>)(A;;FA;;;SY)`, as measured at chunk 2026-09-27-wrapper-channel). They have to be asserted at the peer-credential check boundary. The security excerpt (data classifications) notes the Windows DACL cases are only "partially testable with stubs".
- (.andromeda/test-plan-detail/test-scope-summary.md:57) A listener-property test (a second listener on a held name fails; a remote-mode read-back) is driven through the language-native IPC client or a raw `interprocess` client, not through the CLI alone.
  > - **Driver:** language-native IPC test client (the `viola-channel` sync client, or a raw `interprocess` `local_socket` client for malformed frames) + JSON schema assertion
- (.andromeda/test-plan-detail/test-scope-summary.md:60) The ipc-internal surface defers cross-user peer rejection to section 1's boundary carve-out, which this chunk must honour rather than claim a cross-user drive.
  > Cross-user peer rejection is flagged in section 1.
- (.andromeda/test-plan-detail/test-scope-summary.md:80) The harness `run`'s first step is the workspace suite under plain `cargo test`, the runner under which the folded CARRY's `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 went red; the case must pass there, not only under nextest.
  >       1. the workspace unit and integration tests (`cargo test`)
- (.andromeda/test-plan-detail/test-scope-summary.md:84) The harness `run` is green only when every suite passes, so a red case_4 under `cargo test` keeps the chunk red.
  >     - Exit 0 only if every suite passes and no mutant survives. Non-zero otherwise, with a JSON summary on stdout (suite, passed, failed, survived).
- (.andromeda/test-plan-detail/test-scope-summary.md:92) Any change to the listener's creation must keep `cleanup`'s check that the pipe no longer exists after the wrappers stop.
  >     - Deletes the temp home and verifies the endpoint is gone: a CLI verb against the name returns exit 21, and the Unix socket file or pipe no longer exists.
- (.andromeda/test-plan-detail/test-scope-summary.md:118) Path 1 orders the exclusive endpoint bind before the child spawn and holds a squatted endpoint to exit 1; a listener change must keep both.
  > - **Path:** `run` start sequence. Collision check, pinned copy and plugin folder, the ConPTY sideload (Windows x64, fail-open: any failure leaves the child on the inbox ConPTY), version gate, exclusive endpoint bind, first snapshot and heartbeat, start `wheel` and `budget-gate` events, and only then the child spawn. A second `run` of the same name, or a squatted endpoint, exits 1.
- (.andromeda/test-plan-detail/test-scope-summary.md:124) Path 1's verification signal keeps the duplicate-run exit 1 that the CLI-level squat refusal rests on.
  >     - A duplicate `run` exits 1. A tampered pinned exe (hash mismatch) exits 1.
- (.andromeda/test-plan.md:24) The Comprehensive tier sets the breadth: each security vector the chunk asserts needs its own negative test.
  > **Test tier:** `Comprehensive (2)`
- (.andromeda/test-plan.md:46) The body's broad line for `viola-channel` restates the carve-out: Windows pipe DACL and remote clients are asserted at the boundary, the squatted-name refusal driven same-user.
  > - **Entity:** `viola-channel`: JSON-RPC 2.0 over ndjson on `interprocess` local sockets — partially testable: same-user framing, dispatch, refusal order, server verification and the squatted-name refusal are testable; the cross-user parts (a peer of another euid, remote pipe clients, the Windows pipe DACL) cannot be driven on hosted runners and are asserted at the peer-credential check boundary.
- (.andromeda/test-plan.md:88) A second-user open of the pipe is an untestable zone on hosted runners; scope item (c) must fall back to its Windows-only live measurement where no runner can measure it.
  >   - Cross-OS-user attacks (a second local user squatting or connecting to the endpoint, reading the home). Hosted runners provide one account (security excerpt, Vectors 1 and 7).
- (.andromeda/test-plan.md:135) Path 1's broad line keeps the squatted endpoint exiting 1 after the exclusive bind.
  > - **Path:** `run` start sequence — the collision check, pinned copy and plugin folder, the fail-open ConPTY sideload, version gate, exclusive endpoint bind, first snapshot and heartbeat, and the start `wheel` and `budget-gate` events all precede the child spawn; a second `run` of the same name, a squatted endpoint or a tampered pinned exe exits 1.
- (.andromeda/test-plan.md:178) The IPC vector's required negatives include the squatted endpoint and no inherited channel handles, both of which this chunk re-verifies on Windows.
  > - **Trigger type:** security-vector-coverage (IPC)
  >   - **Source:** security excerpt, Vector 1
  >   - **Required test type:** negative tests covering:
  >     - frame over `MAX_FRAME` → `-32600` + close
  >     - higher `params.v` → `-32602` with `data.supported`/`wrapper`
  >     - `release` with `from` → `release-from-driver`
  >     - server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body
  >     - squatted endpoint → `run` exit 1
  >     - Unix socket dir 0700 and file 0600 asserted
  >     - no inherited channel handles in the child
- (.andromeda/test-plan.md:214) The DACL-effect case (scope item (c)) is asserted where the runner allows and stubbed otherwise.
  >     - Windows DACL owner / ACE cases where the runner allows, stubbed otherwise (section 1)
- (.andromeda/test-plan.md:226) The chunk's CI reads `supply-chain` red on RUSTSEC-2026-0386; the gate's only ignore stays RUSTSEC-2017-0008, so no ignore is added to reach green.
  >   - **Required test type:** CI gates on exit code: `cargo deny check` (only ignore RUSTSEC-2017-0008), `cargo check` of sync crates without tokio, zizmor on workflows, SHA-pinned Actions assertion.
- (.andromeda/test-plan.md:250) The named-pipe and DACL branch tests must run on the Windows runner as OS-branch tests.
  >   - **Required test type:** full workspace and fake-agent E2E on all three runners. OS-branch tests cover named pipe vs Unix socket, ConPTY vs openpty, Windows DACL / strict-modes vs Unix modes, and macOS euid + dir-only server verification.
- (.andromeda/test-plan.md:271) The CARRY's case_4 fix must not rest on a sleep: waits key on logged events and byte offsets.
  >   - **Required test type:** E2E assertions must use hook events, `events.ndjson` and channel payloads, never scraped child-screen content. A negative test proves no path auto-approves a dialog without a verify stamp and wheel `driver`. Waits must key on logged events and byte offsets, not sleeps.

## Bindings
- **Windows DACL / remote-client carve-out ↔ security-plan §Authentication & Authorization (IPC access control, Windows)**: the security clause sets the protected SDDL, `accept_remote(false)`, first-instance and non-inheritable handles; this category holds which of them a hosted runner can drive and which are asserted at the boundary or stubbed (.andromeda/test-plan-detail/test-scope-summary.md:11), (.andromeda/test-plan.md:214)
- **Boundary assertion ↔ the body's §5 Module ↔ IPC and §6 untestable list**: the DACL is read back in canonical form, the squatter bound first through the server API, and cross-OS-user peers covered by the boundary unit test and same-user SDDL read-back (.andromeda/test-plan.md:421), (.andromeda/test-plan.md:476)
- **New negatives ↔ the body's §6 Security control negatives (`security_negatives_*.rs`)**: one negative per control that had none is the home for the remote-client and DACL-effect cases; the CARRY's `tests/security_negatives_channel.rs` coordinate is re-verified against it (.andromeda/test-plan.md:606)
- **OS-branch tests ↔ the body's §10 per-OS coverage and §11 Test Strategy**: `#[cfg(windows)]` code is gated on its own runner, and a Linux-only local run proves nothing for Windows (.andromeda/test-plan.md:813), (.andromeda/test-plan.md:873)
- **Squat tests ↔ the body's §11 Integration**: no `try_overwrite` and no endpoint name reused across tests (.andromeda/test-plan.md:889)
- **CARRY case_4 under plain `cargo test` ↔ the body's §10 Zero-flakiness budget and §11 CI**: the chunk stays red until the root cause is fixed, with no `#[ignore]`, no retries and no `max-threads = 1` serialisation as the fix (.andromeda/test-plan-detail/test-scope-summary.md:80), (.andromeda/test-plan.md:851), (.andromeda/test-plan.md:919), (.andromeda/test-plan.md:925)
- **Supply-chain red ↔ the new route entry "tokio RUSTSEC-2026-0386 lock bump" (scope)**: owned outside this chunk; this category forbids widening the ignore (.andromeda/test-plan.md:226)
