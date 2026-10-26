# tests · Mocking & Stubbing Discipline — category read

## Carried
- (.andromeda/test-plan.md:707) The DACL check before any home read and the Windows proof of `check_instance` (CARRYs 5, 12, 13) must run over a real tempfile home; only the cases a hosted runner cannot provision (foreign SID, non-persistent-ACL volume) may be stubbed, and only as synthetic SDDL descriptors fed to the check function.
  > - Filesystem: not mocked. A real tempfile home is used everywhere, because modes, `File::lock`, torn lines and symlink handling are the behaviour under test. Only the Windows foreign-SID / non-persistent-ACL descriptors are stubbed, as synthetic SDDL descriptors fed to the check function.
- (.andromeda/test-plan.md:51) The Windows strict-modes cases this chunk adds must split the same way: testable cases in a temp `--home` (modes, strict-modes, pinned-exe hash mismatch), the unprovisionable descriptor cases stubbed at the descriptor-check layer.
  > Unix modes and strict-modes, torn lines, snapshot replay, heartbeat staleness and pinned-exe hash mismatch are testable in a temp `--home`. Windows strict-modes cases that need a foreign-owner SID or a volume without persistent ACLs (FAT/exFAT) cannot be provisioned on windows-2025 hosted runners. They get stubs at the descriptor-check layer, per the security excerpt's Config classification ("partially testable with stubs for the Windows DACL cases").
- (.andromeda/test-plan.md:477) The untestable Windows zones must stay covered by the §4 descriptor stubs, never by a mocked filesystem.
  > - Non-persistent-ACL volumes and foreign-owner SIDs. Covered by synthetic SDDL descriptor stubs (§4).
- (.andromeda/test-plan.md:214) The filesystem vector's Windows DACL owner / ACE negatives must be real where the runner allows and stubbed only otherwise.
  > - Windows DACL owner / ACE cases where the runner allows, stubbed otherwise (section 1)
- (.andromeda/test-plan.md:378) The instance check and Windows strict-modes the chunk extends must be unit-tested in `viola-state` over synthetic SDDL descriptors.
  > Unix strict-modes and Windows strict-modes over synthetic SDDL descriptors, and the instance check.
- (.andromeda/test-plan.md:712) The new strict-modes and re-hash integration cases must use real files in a tempfile home, never a mocked on-disk state.
  > - On-disk state at integration level (the arch has no DB). Use real files in a tempfile home.
- (.andromeda/test-plan.md:886) The same ban, stated as an integration anti-pattern, binds every new home-integrity integration case.
  > - NEVER mock the on-disk state (the arch has no DB; this is the DB-equivalent rule). Use a real tempfile home.
- (.andromeda/test-plan.md:389) Any double the chunk's tests need must be a mockall double of a trait the product already defines at its seams; no new trait may be introduced only to be mocked.
  > - Module logic with mocked dependencies (mockall doubles only for traits the product already defines at its seams: `Pty`, liveness probe, `Clock`)
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18) Process doubles stay on seam traits only, and the fake agent and pinned binary run as real executables resolved by the named lookup.
  > `#[cfg_attr(test, mockall::automock)]` on seam traits only; external processes are real stub executables, resolved through `CARGO_BIN_EXE_viola-fake-agent` in root `tests/` only.
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18) When the strict-modes check goes ahead of the snapshot read in the frame verbs, hook and `mcp` tools (CARRYs 3, 7–10, 15, 18), the server-identity negatives must keep a real held live process behind the impostor's snapshot, never a double.
  > a held live process whose pid an impostor's snapshot records is a real process, never a double: in root tests the fake agent's `--hold-stdout-internal` holder, in `crates/viola-mcp/tests/handlers.rs` its own `Held` child, entered at `handlers_held_child_entry` only when its `HELD` variable is set, each killed and reaped by a `Drop` guard
- (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:19) The impostor fixtures in the server-verification negatives are reached only after the new strict-modes check, so their test-built homes must pass it for the negative to still prove the server check (and a case refused by strict-modes first proves the other control).
  > in the server-verification negatives (Linux and Windows) an impostor, a server on the recorded endpoint while the snapshot records another held live process (Process mocking), stands in for a server that is not the recorded wrapper (root tests bind a test-owned `viola_channel::Server::bind` listener: `tests/security_negatives_server_identity.rs`, `tests/hook_fail_open.rs`'s `Impostor`; the MCP case is `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`), and the hook unit impostor varies the recorded start time instead (a second early, or an unparseable `started_at`), a root-bin unit test having no portable second process
- (.andromeda/test-plan.md:449) The MCP handler tests' `TestWrapper` writes its snapshot through the product writer; with the `mcp` frame tools gaining the strict-modes check (CARRY 15), that stub wrapper's home must satisfy it.
  > - **MCP:** adapter-level handler tests in `crates/viola-mcp/tests/handlers.rs` drive the transport-free `viola_mcp::Tools` handle (the same `Server::call` `serve` answers with) against a `viola_channel::Server` test wrapper (`TestWrapper`, its snapshot written through `viola_state::snapshot::write_snapshot`);
- (.andromeda/test-plan.md:631) The only hand-made on-disk state is the named carve-outs: the `seed_conpty` companions (CARRYs 6 and 13 ask whether a home a test made first is one the product must refuse) and the impostors' test-built `snapshot.json`; every other home-integrity case starts from a home viola created.
  > - **On-disk product state:** produced only by real `viola` runs against the fake agent in a fresh home, never hand-written — except the chaos tests' corruption of files the product already wrote, the Windows x64 `seed_conpty` companions and the server-identity impostors' test-built `snapshot.json`; a case that needs a home viola created starts from `stamped_home`.
- (.andromeda/test-plan.md:905) The single-writer proof for `stamps.json` and `snapshot.json` must not itself hand-write either file; a malformed state file is planted only by altering the writer's own bytes.
  > - NEVER hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`. Stamps come from `viola verify`, snapshots from the wrapper and budget from `viola hook statusline`. The ban binds every booted-wrapper test. A reader's own unit cases may plant a malformed state file only by altering the writer's own bytes: write through the writer, then pad, replace or re-version (the operator's answer at the chunk 2026-10-10-budget-governor wrap, where `budget_read_*` landed in that form).
- (.andromeda/test-plan.md:714) The per-start plugin rewrite and pinned-binary re-hash must be exercised by the fake agent calling the real pinned binary, never a stubbed hook.
  > - `viola hook` invocations inside E2E. The fake agent calls the real pinned binary.
- (.andromeda/test-plan.md:915) The same, as a mocking ban: the hook in E2E runs through the plugin files `run` wrote.
  > - NEVER stub `viola hook` in E2E. The fake agent must invoke the real pinned binary through the plugin files `run` wrote.
- (.andromeda/test-plan.md:914) Faults the chunk needs (a tampered pinned exe, a widened home mode, the `pin_exe` race of CARRY 16) must come from process and file operations or existing mockall seams, never a failpoint crate.
  > - NEVER introduce a failpoint crate that pulls tokio into the sync crates (for example fail-parallel). Fault seams go through mockall doubles of existing traits plus process and file faults.
- (.andromeda/test-plan.md:726) The strict-modes and re-hash seams must be reached by dependency injection, never by runtime patching of production code.
  > **Anti-monkey-patching:** prefer dependency injection over runtime patching of production code. Doubles exist only for traits viola already defines at its seams (research, mockall key detail).
- (.andromeda/test-plan.md:910) Windows security APIs are not the product's own, so a test must not mock them without an abstraction layer; the descriptor-check function taking a synthetic SDDL is that layer.
  > - NEVER mock what you don't own (libraries / language runtime) without
  >   abstraction layer
- (.andromeda/test-plan.md:912) The descriptor stubs must stay limited to the unprovisionable cases.
  > - NEVER over-stub (you stop testing real behavior)

## Bindings
- **Filesystem not mocked, descriptor stubs only ↔ §1 entity and §4 viola-state unit**: the §8 rule (.andromeda/test-plan.md:707) is the same split the `viola-state` entity states (.andromeda/test-plan.md:51) and the §4 unit list holds (.andromeda/test-plan.md:378); the §6 untestable zone (.andromeda/test-plan.md:477) points back to it.
- **Impostor doubles ↔ §6 Security control negatives and §5 MCP handler tests**: the impostor rows (.andromeda/test-plan-detail/mocking-stubbing-discipline.md:18, .andromeda/test-plan-detail/mocking-stubbing-discipline.md:19) and the `TestWrapper` (.andromeda/test-plan.md:449) sit beside the server check; once the strict-modes check precedes the snapshot read, each such home must pass it, and the carve-out allowing their test-built snapshot is (.andromeda/test-plan.md:631).
- **Hand-made state carve-outs ↔ §7 Test Data and §11 Test Data**: the `seed_conpty` and impostor carve-outs (.andromeda/test-plan.md:631) and the writer-only ban (.andromeda/test-plan.md:905) bind the single-writer negatives and the seeded-home question of CARRYs 6 and 13.
- **Real pinned hook in E2E ↔ §6 Path 1**: (.andromeda/test-plan.md:714) and (.andromeda/test-plan.md:915) bind the plugin-rewrite and pinned-binary checks Path 1's restart signal asserts.
- **Fault seams ↔ §2 Chaos / §11 Universal**: (.andromeda/test-plan.md:914) binds the tampered-exe and race faults to process and file operations.
