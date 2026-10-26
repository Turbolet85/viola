# tests · Unit Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/unit-test-strategy.md:13) The Windows handle non-inheritance unit case is the chunk's "already asserted" non-inheritable leg; it must stand, re-verified rather than rebuilt, and any new Windows listener test keeps it the both-ends `GetHandleInformation` reading.
  > `#[cfg(windows)]` handle non-inheritance, which backs E2 and the §12 open question. After the `viola-channel` server creates a pipe instance and a client connects, `GetHandleInformation` (windows-sys) on both handles (via `AsRawHandle`) reports `HANDLE_FLAG_INHERIT` clear. This is the only Windows evidence; there is no child-side probe.
- (.andromeda/test-plan-detail/unit-test-strategy.md:11) `endpoint.rs` is a named surface of the chunk; the endpoint golden vectors, the Windows pipe form included, must keep passing unchanged (no wire contract changes).
  > FNV-1a endpoint golden vectors: `viola-<12 hex>` of `ViolaName + "\0" + abs home`, with the Windows `\\.\pipe\` form
- (.andromeda/test-plan-detail/unit-test-strategy.md:12) The cross-user rejection boundary is held at the unit level by an injected-identity decision function; the DACL-effect leg (c) that a hosted runner cannot drive with a second user is asserted at this kind of boundary, never by a second account.
  > The peer-credential decision function with injected `(peer_euid, own_euid)`, covering the cross-user rejection boundary from test-scope Sec 1.
- (.andromeda/test-plan-detail/unit-test-strategy.md:22) The precedent for a Windows case a hosted runner cannot provision: synthetic security descriptors built from SDDL strings fed to a check function; a DACL-effect case (c) that cannot be measured on `windows-2025` follows this stub form or is owed as a Windows-only live measurement.
  > Windows strict-modes feeds synthetic security descriptors built from SDDL strings (via windows-sys) to the descriptor-check function. This covers a foreign-owner SID, a NULL DACL, an inherit-only GENERIC_WRITE ACE, a read grant on `diagnostics/`, and `CREATOR OWNER` treated as the user. These are the stubs for the cases hosted runners cannot provision (test-scope Sec 1).
- (.andromeda/test-plan-detail/unit-test-strategy.md:50) The folded CARRY's red case is one of the budget governor's named unit families; the chunk must make `send_budget_paused_is_refused_with_the_window_word_and_no_cursor` case_4 pass under a plain `cargo test` (three runs) or fix its log capture, keeping it in this file and family.
  > `send_budget_paused_*` (`src/run/send/budget.rs`)
- (.andromeda/test-plan.md:46) The `viola-channel` entity's testability carve-out binds legs (a) and (c): remote pipe clients and the DACL's effect on another principal are not driven on hosted runners.
  > the cross-user parts (a peer of another euid, remote pipe clients, the Windows pipe DACL) cannot be driven on hosted runners and are asserted at the peer-credential check boundary.
- (.andromeda/test-plan.md:88) Cross-OS-user attack cases are an untestable zone on hosted runners; (c) must not claim a CI reading it cannot make.
  > Cross-OS-user attacks (a second local user squatting or connecting to the endpoint, reading the home). Hosted runners provide one account (security excerpt, Vectors 1 and 7).
- (.andromeda/test-plan.md:476) The cross-user peer is covered by the §4 boundary unit test plus the §5 same-user SDDL read-back; the chunk's DACL assertions sit in those two layers.
  > Cross-OS-user peers. Covered by the peer-credential boundary unit test (§4) and same-user SDDL read-back (§5).
- (.andromeda/test-plan.md:185) The IPC coverage trigger names the squatted-endpoint refusal and the no-inherited-handles case the chunk re-verifies.
  > squatted endpoint → `run` exit 1
- (.andromeda/test-plan.md:187) Same trigger, the non-inheritance leg.
  > no inherited channel handles in the child
- (.andromeda/test-plan.md:349) New unit cases run under the pinned framework through nextest.
  > **Framework:** cargo test (libtest, current Rust stable pinned in `rust-toolchain.toml`, ≥ 1.96; MSRV 1.96, edition 2024), executed through cargo-nextest 0.9.146 (process-per-test, JUnit).
- (.andromeda/test-plan.md:355) New listener unit cases (remote-client mode read-back, first-instance on a held name) sit inline in the source file they test.
  > Test file location: inline `#[cfg(test)] mod tests` in the same source file, in every `crates/viola-<area>` and in root `src/`.
- (.andromeda/test-plan.md:356) Naming rule for every new case.
  > Test function naming: `<subject>_<condition>_<expected>` in snake_case.
- (.andromeda/test-plan.md:357) A table-driven case uses labelled rstest cases.
  > Table-driven cases use rstest `#[case]`, and each case name must be a readable label (for example `#[case::esc_refused]`).
- (.andromeda/test-plan.md:376) The `viola-channel` unit scope the chunk extends; Windows handle non-inheritance is already in it.
  > **viola-channel:** the ndjson frame codec at the `MAX_FRAME` boundary, the JSON-RPC error mapping, the FNV-1a endpoint golden vectors, the peer-credential decision with injected euids, and Windows handle non-inheritance.
- (.andromeda/test-plan.md:386) The root bin's unit scope holds the budget governor family the CARRY case belongs to.
  > the budget governor's pure decision and override expiry
- (.andromeda/test-plan.md:389) Any double the new tests need is a mockall double of an existing seam trait only; no double of the pipe or interprocess.
  > Module logic with mocked dependencies (mockall doubles only for traits the product already defines at its seams: `Pty`, liveness probe, `Clock`)
- (.andromeda/test-plan.md:881) Expected values (the SDDL/ACE read-back, the pipe-mode flag) are written as literals in the test, never imported from the product.
  > NEVER import the product's own `IDENTITY_FLOOR`, ledger rows or refusal ordering as the test oracle. The expected values (the 11 identity-floor names, the refusal order, exit codes) are written as literals in the test.
- (.andromeda/test-plan.md:421) The second-listener-on-a-held-name case (b) creates its squatter through the server API, and the DACL read-back compares the canonical form.
  > the squatter created first through the server API (never `try_overwrite`), and the Unix socket modes and the Windows pipe DACL read back — the DACL compared in its canonical form, never the literal it set.
- (.andromeda/test-plan.md:889) The first-instance test must not use `try_overwrite` or reuse an endpoint name across tests.
  > NEVER use interprocess `try_overwrite`, or reuse an endpoint name across tests.
- (.andromeda/test-plan.md:873) Every `#[cfg(windows)]` assertion is closed only on the `windows-2025` CI leg, never by the Linux dev host.
  > NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner.
- (.andromeda/test-plan.md:767) The Windows runner is where the named-pipe and DACL read-back cases run.
  > `windows-2025` (ConPTY, named pipe, DACL / SDDL read-back, `.cmd` refusal)
- (.andromeda/test-plan.md:813) New `#[cfg(windows)]` code counts toward the Windows job's coverage gate.
  > Gates are enforced per OS job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`. Per-OS gating is deliberate: `#[cfg(windows)]` / `#[cfg(unix)]` code compiles only on its own runner.
- (.andromeda/test-plan.md:851) The CARRY's once-red case is a flake under this budget: it is fixed at its root cause in this chunk, never parked.
  > **Zero-flakiness budget:** flaky tests are NOT tolerated. The nextest `retries = 0`. If a test flakes once, it gets quarantined immediately and fixed at the root cause, never with a retry-once budget. For viola, quarantine means the chunk stays red until the root cause is fixed within that chunk. `#[ignore]` is not allowed as a parking place.
- (.andromeda/test-plan.md:919) The CARRY's libtest-thread log-capture fault may not be closed by serialising tests.
  > NEVER serialise tests (`max-threads = 1`): isolation comes from the per-test home.
- (.andromeda/test-plan.md:925) Nor by a retry.
  > NEVER set nextest `retries` above 0.
- (.andromeda/test-plan.md:606) The suite the CARRY's coordinate names holds the Windows client SQOS negative (the next entry's surface), not the DACL read-back; the coordinate is re-verified, the suite not re-scoped.
  > **Security control negatives** (`security_negatives_*.rs`): one negative test per security-plan control that had none — the Unix socket directory refusal, Windows client SQOS at the identification level,

## Bindings
- **Windows non-inheritance unit case ↔ body §1 IPC trigger**: the unit case (.andromeda/test-plan-detail/unit-test-strategy.md:13) is the evidence behind the trigger's "no inherited channel handles in the child" (.andromeda/test-plan.md:187); both stand as the chunk's re-verified leg.
- **Peer-credential boundary / SDDL stubs ↔ body §1 entity carve-out, §6 untestable list ↔ security-plan §Authentication (IPC access control, Windows)**: legs (a) remote-client rejection and (c) DACL effect are asserted at the unit boundary (.andromeda/test-plan-detail/unit-test-strategy.md:12, .andromeda/test-plan-detail/unit-test-strategy.md:22) and same-user read-back (.andromeda/test-plan.md:476), because hosted runners cannot drive them (.andromeda/test-plan.md:46, .andromeda/test-plan.md:88); what neither layer can measure is owed to "Windows-only live measurements", per the scope.
- **First-instance case (b) ↔ §5 Integration (Module ↔ IPC) and §11 Integration bans**: the squatter is bound through the server API, never `try_overwrite`, no endpoint name reused (.andromeda/test-plan.md:421, .andromeda/test-plan.md:889); the CLI-level squat refusal stays the §1 trigger's `run` exit 1 (.andromeda/test-plan.md:185).
- **Budget CARRY ↔ §10 zero-flakiness ↔ §11 CI bans**: `send_budget_paused_*` (.andromeda/test-plan-detail/unit-test-strategy.md:50) must go green under plain `cargo test` by a root-cause fix (.andromeda/test-plan.md:851), never by `max-threads = 1` or retries (.andromeda/test-plan.md:919, .andromeda/test-plan.md:925).
- **Windows cases ↔ §9 matrix ↔ §10 per-OS coverage gate**: every new `#[cfg(windows)]` test is closed on `windows-2025` (.andromeda/test-plan.md:767, .andromeda/test-plan.md:873) and its code counts in that job's 85/95/80 gate (.andromeda/test-plan.md:813).
- **CARRY coordinate ↔ §6 security negatives suite**: `tests/security_negatives_channel.rs` holds the SQOS negative (.andromeda/test-plan.md:606), which belongs to "Server verification before any frame", not this chunk; the DACL read-back lives in the `server/win.rs` unit cases.
