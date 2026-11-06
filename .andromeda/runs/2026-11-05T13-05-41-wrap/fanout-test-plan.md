@proposal n=1 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1 Test Scope Summary
change: fct-cyzmyvps: the tier justification reads the surfaces on Linux (CI on `ubuntu-latest`), not "across 3 required OSes", and "Five drivers" becomes "Four drivers" (the multi-platform CI driver retired); the Comprehensive tier stands on the remaining four.
sidecar: §1 tier justification: three required OSes → Linux only, five drivers → four (founder rulings 2026-11-04, inputs#I1, inputs#I5 item 6).
rationale: report Retired (fct-tulramlm): the OS target «Windows first, macOS and Linux» → «Linux only … CI on `ubuntu-latest`», Windows/macOS code recoverable at tag `multi-os-final`; Counts os-target class names «three OSes» stale.

@proposal n=2 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §1 Test Scope Summary
change: fct-yjkq3rgm: the driver "the creator brief requires multi-platform CI (Founder Direction 4, D3)" is dropped from the tier drivers: the founder's rulings of 2026-11-04 make viola Linux-only (CI on `ubuntu-latest`).
sidecar: §1 tier drivers: the multi-platform CI driver retired with the Linux-only ruling.
rationale: report Retired (fct-tulramlm) and Decisions (fct-g4iyzuf4): Viola is Linux-only; the windows-2025 and macos-latest CI legs are retired.

@proposal n=3 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §1 Test Scope Summary
change: fct-njspb5nv: the viola-pty entity drops "plus the windows-sys `TerminateProcess` fallback": the seam over portable-pty `=0.8.1` (spawn, read, write, resize, wait, kill), exit detected on the process handle and never on EOF.
sidecar: §1 viola-pty entity: the windows-sys TerminateProcess kill fallback removed.
rationale: report Symbols / APIs (fct-xxutheix): viola-pty loses the windows-sys `TerminateProcess` kill fallback; Dependencies (fct-www7rgz5): windows-sys removed.

@proposal n=4 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §4 Unit Test Strategy
change: fct-sbml5vi6: the sentence "The `TerminateProcess` fallback path is `#[cfg(windows)]`." is removed; the viola-pty unit coverage is exit from `wait()` on the handle against the mockall double.
sidecar: §4 viola-pty: the cfg(windows) TerminateProcess fallback line removed.
rationale: report Symbols / APIs (fct-xxutheix): the windows-sys `TerminateProcess` kill fallback removed with every caller.

@proposal n=5 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1 Test Scope Summary
change: fct-qg6j3l2y: the viola-channel entity drops the clause on the Windows listener rejecting remote pipe clients and its pipe DACL denying an anonymous open on `test (windows-2025)`; same-user framing, dispatch, refusal order, server verification and the squatted-name refusal are testable, and only the cross-user part is asserted at the peer-credential check boundary.
sidecar: §1 viola-channel entity: the Windows named-pipe listener / DACL clause retired.
rationale: report Retired (fct-tulramlm): the Windows named-pipe endpoint, its SDDL / SQOS / `accept_remote(false)` controls retired; Symbols (fct-xxutheix): `server/win.rs` and the named-pipe arms removed.

@proposal n=6 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §5 Integration Test Strategy
change: fct-ctentyeb: Module ↔ IPC reads a real endpoint bound in a temp home with frames validated by jsonschema, the squatter created first through the server API (never `try_overwrite`), and the Unix socket modes read back; the Windows pipe DACL read-back clause is removed.
sidecar: §5 Module ↔ IPC: the Windows pipe DACL read-back removed.
rationale: report Retired (fct-tulramlm): Windows named-pipe endpoint and its SDDL controls retired; Symbols (fct-xxutheix): `test_support`'s `canonical_sddl` · `dacl_of` · `user_sid` removed.

@proposal n=7 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §5 Integration Test Strategy
change: fct-r32qnzfk: the "Windows DACL read-back" driver bullet (windows-sys 0.61.2, `icacls` grants and `/save` canonical SDDL) is removed.
sidecar: §5 Drivers: the Windows DACL read-back driver retired.
rationale: report Dependencies (fct-www7rgz5): windows-sys removed; Symbols (fct-xxutheix): `tests/support/home.rs` loses the `icacls` arms of `widen` · `grant_read` · `unwiden` · `still_widened`.

@proposal n=8 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §6 E2E Test Strategy
change: fct-lkgqv65p: cross-OS-user peers are covered by the peer-credential boundary unit test (§4) and the same-user socket modes read-back (§5), not "same-user SDDL read-back"; the euid wiring clause stands.
sidecar: §6 untestable list: SDDL read-back → socket modes read-back.
rationale: report Retired (fct-tulramlm): the SDDL controls retired with the Windows named-pipe endpoint.

@proposal n=9 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §4 Unit Test Strategy
change: fct-2hayha6u: viola-channel unit coverage reads "the connected stream's server pid" (not "on each OS") and drops "and Windows handle non-inheritance".
sidecar: §4 viola-channel: per-OS server pid and Windows handle non-inheritance removed.
rationale: report Symbols (fct-xxutheix): the named-pipe arms and `SERVER_PID_REPORTED` / `PEER_EUID_REPORTED` folded to their Linux value.

@proposal n=10 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §6 E2E Test Strategy
change: fct-4uc5ehox: the security negatives read the socket directory refusal on Linux (not "Linux and macOS"), drop the Windows client SQOS, Windows listener remote/anonymous, Windows `--home` outside the profile and Windows x64 ConPTY companion (`tests/conpty_sideload.rs`) negatives, read `tests/security_negatives_strict_modes.rs` on Linux (not "three OSes"), and the server-identity negative without "on Linux and Windows".
sidecar: §6 Security control negatives: Windows / macOS negatives retired, the rest on Linux.
rationale: report Changes Files (fct-yq5vc7zj): `tests/channel_sqos_open.rs`, `tests/security_negatives_channel.rs`, `tests/conpty_sideload.rs` deleted; Retired (fct-tulramlm): SQOS, `accept_remote(false)`, Windows home owner/DACL arm, ConPTY sideload.

@proposal n=11 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1 Test Scope Summary
change: fct-i65r3xxr: viola-state Testability reads testable (Unix modes and strict-modes, torn lines, snapshot replay, heartbeat staleness and pinned-exe hash mismatch in a temp `--home`); the Windows strict-modes reason (foreign-owner SID, non-persistent-ACL volumes, descriptor-check stubs) is removed.
sidecar: §1 viola-state entity: the Windows DACL partial-testability reason retired.
rationale: report Retired (fct-tulramlm): the Windows home owner/DACL strict-modes arm; Symbols (fct-xxutheix): `strict.rs`'s Windows owner/DACL arm, `Refused::NullDacl`, `Refused::NoPersistentAcls` removed.

@proposal n=12 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §1 Test Scope Summary
change: fct-ykziik74: config and code-bearing state reads Testability testable in a temp home; the "same Windows DACL limitation as viola-state" reason is removed.
sidecar: §1 config/code-bearing state: Windows DACL limitation retired.
rationale: report Retired (fct-tulramlm): the Windows home owner/DACL strict-modes arm.

@proposal n=13 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §6 E2E Test Strategy
change: fct-dcxdz6xw: the skipped-as-untestable bullet "Non-persistent-ACL volumes and foreign-owner SIDs. Covered by synthetic SDDL descriptor stubs (§4)." is removed.
sidecar: §6 untestable list: the Windows ACL volume / foreign SID bullet retired.
rationale: report Retired (fct-tulramlm): the Windows owner/DACL strict-modes arm; Symbols (fct-xxutheix): `Refused::NoPersistentAcls` removed.

@proposal n=14 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §8 Mocking & Stubbing Discipline
change: fct-hkmaq6ye: Filesystem: not mocked, a real tempfile home everywhere; the sentence "Only the Windows foreign-SID / non-persistent-ACL descriptors are stubbed …" is removed (nothing on the filesystem is stubbed).
sidecar: §8 Filesystem: the Windows SDDL descriptor stubs retired.
rationale: report Retired (fct-tulramlm): the Windows owner/DACL arm and SDDL controls.

@proposal n=15 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §4 Unit Test Strategy
change: fct-7vfmbe3m: viola-state unit coverage drops "Windows strict-modes (the write and the read rule) over synthetic SDDL descriptors and, on `windows-2025`, real grants", the off-Unix case `check_socket_dir_off_unix_reads_nothing` (its arm gone) and the `windows-2025` `board_scan_holds_a_read_granted_home_…` case; Unix strict-modes and the other cases stand.
sidecar: §4 viola-state: Windows strict-modes / SDDL / windows-2025 cases and the off-Unix socket-dir case removed.
rationale: report Symbols (fct-xxutheix): `strict.rs`'s Windows owner/DACL arm, `windows_read_verdict` and the off-Unix `Ok(())` arm of `check_socket_dir` removed (its Unix body unconditional).

@proposal n=16 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §1 Test Scope Summary
change: fct-lgbstcvc: Vector 7 reads "a home, instance directory or `diagnostics/` another user may read → refused (a group or other bit)", drops the ConPTY companion → inbox ConPTY `sideload_fallback` bullet, reads "`plugin/` rewritten on start, and the instance's `settings.json`" (no Windows carve-out), and drops the "Windows DACL owner / ACE cases" bullet.
sidecar: §1 Vector 7 triggers: Windows read-grant, ConPTY companion and DACL cases retired.
rationale: report Retired (fct-tulramlm): the Windows owner/DACL strict-modes arm and the ConPTY sideload (`sideload_fallback`).

@proposal n=17 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §7 Test Data & Fixtures
change: fct-54gkcbby: on-disk product state drops the "Windows x64 `seed_conpty` companions" exception and the Windows `Writable` clause: a plain home is one the product refuses by the read rule.
sidecar: §7 Seed strategies: seed_conpty exception and the Windows plain-home reading removed.
rationale: report Symbols (fct-xxutheix): `tests/support/home.rs` loses `seed_conpty`; Retired (fct-tulramlm): ConPTY sideload and the Windows owner/DACL arm.

@proposal n=18 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1 Test Scope Summary
change: fct-2ls6kelo: the Path 1 start sequence drops "the fail-open ConPTY sideload": strict-modes check, collision check, pinned copy and plugin folder, version gate, socket directory check, exclusive bind, first snapshot and heartbeat, and the start `wheel` and `budget-gate` events precede the child spawn.
sidecar: §1 Path 1: the ConPTY sideload step removed from the start sequence.
rationale: report Retired (fct-tulramlm): the ConPTY sideload and `run.conpty_sideload`; Symbols (fct-xxutheix): `src/cmd/run.rs`'s `run.conpty_sideload` step and `pin_companions` call removed.

@proposal n=19 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §5 Integration Test Strategy
change: fct-nvtf2boy: Module ↔ PTY reads real openpty spawns of `viola-fake-agent` through `viola-pty` (exit on the handle, resize propagated, key after resize once the child reports the size); "ConPTY", "a `.cmd` child refused" and the Windows x64 sideloaded-ConPTY clause are removed.
sidecar: §5 Module ↔ PTY: ConPTY, .cmd refusal and sideload clauses retired.
rationale: report Retired (fct-tulramlm): ConPTY sideload and the `.cmd`/`.bat` refusal; Symbols (fct-xxutheix): `pty_backend()` always `openpty`.

@proposal n=20 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §5 Integration Test Strategy
change: fct-wwnk7u4j: the lifecycle names one carve-out, `tests/chaos_feed_panic.rs` booting outside the scans; the Windows x64 `seed_conpty` carve-out is removed.
sidecar: §5 Setup / teardown: the seed_conpty carve-out retired, one carve-out left.
rationale: report Symbols (fct-xxutheix): `tests/support/home.rs` loses `seed_conpty`; Retired (fct-tulramlm): `bin/<version>-<hash>/conpty/`.

@proposal n=21 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §5 Integration Test Strategy
change: fct-yj4h2yma: the "DA1 answer" paragraph (under the sideloaded ConPTY the piped test driver answers DA1) is removed.
sidecar: §5 DA1 answer under the sideloaded ConPTY retired.
rationale: report Retired (fct-tulramlm): the ConPTY sideload.

@proposal n=22 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp dependent_of=D-retired
section: §9 CI Integration
change: fct-onifqrhg: Coverage report reads "`agent-run run --coverage` is the `test` job on `ubuntu-latest`"; "on `windows-2025` after the ConPTY vendor verification" is removed.
sidecar: §9 Coverage report: windows-2025 / ConPTY vendor verification removed.
rationale: report Harness / gate surface (fct-sy4jxsfp): the `ConPTY vendor verification` step leaves `ci.yml`, matrix `[ubuntu-latest]`.

@proposal n=23 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-yq5vc7zj dependent_of=D-retired
section: §9 CI Integration
change: fct-dk3rsyuy: the build failure condition "ConPTY vendor errors (`windows-2025`): `scripts/conpty-vendor.sh --verify` …" is removed.
sidecar: §9 Build failure conditions: ConPTY vendor errors retired.
rationale: report Files (fct-yq5vc7zj): `scripts/conpty-vendor.sh` and `vendor/conpty/` deleted; Retired (fct-tulramlm): the `ConPTY vendor verification` CI step.

@proposal n=24 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §11 Test Anti-Patterns
change: fct-ukay3vyw: "NEVER detect child exit by reading master EOF; use `child.wait()` on the handle." (the "(ConPTY)" tag removed).
sidecar: §11 E2E: ConPTY tag dropped from the exit-on-handle ban.
rationale: report Symbols (fct-xxutheix): `pty_backend()` now always `openpty`; child exit still detected on the handle wait.

@proposal n=25 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1 Test Scope Summary
change: fct-phy2lvsc: Vector 8 drops "`.cmd` / `.bat` PTY child → `run` exit 1 (windows runner)", reads "a given program path runs as given and a bare name resolves on `PATH`" for the npm-shim → `claude.exe` line, and the vt100 panic witness reads "the child's own bytes reach the outer terminal and no viola literal appears on it" (the Windows byte reading and its route entry removed).
sidecar: §1 Vector 8: .cmd/.bat refusal and npm-shim resolution retired; vt100 witness Linux-only.
rationale: report Retired (fct-tulramlm): the `.cmd`/`.bat` refusal and «Windows-only live measurements»; Symbols (fct-xxutheix): `resolve_program`'s `Search::windows`, `pathext`, `SHIM_TARGET` removed (a given path as given, a bare name on `PATH`).

@proposal n=26 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §6 E2E Test Strategy
change: fct-47fi4ugg: the chaos suite drops "and a `.cmd` child refused exit 1".
sidecar: §6 Chaos suite: .cmd refusal case retired.
rationale: report Retired (fct-tulramlm): `batch-script-child`, `AgentError::BatchScriptChild` and the start refusal.

@proposal n=27 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §6 E2E Test Strategy
change: fct-ndx7lm2p: the exit-cause matrix drops "the `.cmd` human pair" from the causes outside the table, reads the server-verification negative in `tests/security_negatives_server_identity.rs` with no "on Linux and Windows" and no macOS carve-out parenthetical, the socket-directory case with no "on Linux and macOS", and the strict-modes frame-verb case with no "on three OSes".
sidecar: §6 Exit-cause matrix: .cmd pair, macOS carve-out and per-OS qualifiers removed.
rationale: report Retired (fct-tulramlm): the `.cmd`/`.bat` human pair and the macOS no-peer-pid carve-out; Symbols (fct-xxutheix): `server_verdict_rows` loses its no-pid row.

@proposal n=28 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §4 Unit Test Strategy
change: fct-nr357doe: viola-agent-claude unit coverage reads "program resolution (a given path as given, a bare name on `PATH`)" in place of "npm-shim resolution".
sidecar: §4 viola-agent-claude: npm-shim resolution → program resolution.
rationale: report Symbols (fct-xxutheix): `resolve_program`'s `Search::windows`, `pathext`, `is_script`, `SHIM_TARGET` removed; `ProgramNotFound` unchanged.

@proposal n=29 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §1 Test Scope Summary
change: fct-wcpph3y4: the viola-agent-claude entity reads "program resolution" in place of "npm-shim resolution".
sidecar: §1 viola-agent-claude entity: npm-shim resolution → program resolution.
rationale: report Symbols (fct-xxutheix): the Windows shim resolver removed.

@proposal n=30 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1 Test Scope Summary
change: fct-u3dkdc2w: the inherited-credentials entity's persistent-environment names are `config.json` `claude_env_keep` alone; "Windows registry `Environment` value names, Unix" is removed (the eleven-name identity floor clause stands).
sidecar: §1 inherited credentials: the Windows registry environment reader retired.
rationale: report Retired (fct-tulramlm): the Windows registry environment reader; Symbols (fct-xxutheix): `src/run/env.rs`'s `HKCU\Environment` reader removed, `claude_env_keep` and the eleven-name floor stay.

@proposal n=31 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §1 Test Scope Summary
change: fct-paewrdc5: the hook entity reads "before passing the statusline through"; "(the pass-through on Unix only; on Windows the arm runs no command)" is removed.
sidecar: §1 hook entity: Windows no-command statusline arm retired.
rationale: report Symbols (fct-xxutheix): `WRAPPED_HERE` folded and `statusline::shell_argv` returns `[&str; 3]` (was `Option`).

@proposal n=32 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §1 Test Scope Summary
change: fct-syqfxw5d: Untestable zones read "the real euid read on Linux" (not "Linux and macOS"), and the bullets "Windows volumes without persistent ACLs and foreign-owner SIDs on hosted runners" and "macOS local runs" are removed.
sidecar: §1 Untestable zones: Windows ACL and macOS bullets retired.
rationale: report Retired (fct-tulramlm): Windows owner/DACL arm, the macOS `$TMPDIR/viola/` socket dir and the macOS carve-out.

@proposal n=33 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §6 E2E Test Strategy
change: fct-x7oaiapn: the skipped-as-untestable bullet "macOS local runs, which are CI-only." is removed.
sidecar: §6 untestable list: macOS bullet retired.
rationale: report Retired (fct-tulramlm): the `macos-latest` CI leg and macOS code removed.

@proposal n=34 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §4 Unit Test Strategy
change: fct-fn3ybwy5: the root-bin socket-directory cases drop "both running on macOS too".
sidecar: §4 root bin: macOS qualifier removed.
rationale: report Retired (fct-tulramlm): the macOS code and `macos-latest` leg.

@proposal n=35 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §6 E2E Test Strategy
change: fct-4aawmv2h: the security sweep's hook fail-open matrix reads "a server that is not the recorded wrapper, a socket directory the check refuses or a server of another euid" with no "on Linux and Windows" / "on Unix" qualifiers.
sidecar: §6 Security sweep: per-OS qualifiers removed.
rationale: report Retired (fct-tulramlm): Linux only; Symbols (fct-xxutheix): `SERVER_PID_REPORTED` folded.

@proposal n=36 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §1 Test Scope Summary
change: fct-oftlgaxh: the library-only surface reads "These tests run on the `ubuntu-latest` CI leg." in place of "on all three CI OSes".
sidecar: §1 library-only surface: three CI OSes → ubuntu-latest.
rationale: report Retired (fct-tulramlm): the windows-2025 and macos-latest CI legs.

@proposal n=37 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §2 Test Strategy
change: fct-njccdctv: the Unit row reads inline `#[cfg(test)] mod tests` in all six crates and the root bin with rstest `#[case]` tables, on Linux; "on all three OSes except … compiled out on macOS" is removed.
sidecar: §2 Unit row: three-OS / macOS exception removed.
rationale: report Retired (fct-tulramlm): Linux only; the macOS code removed.

@proposal n=38 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §1 Test Scope Summary
change: fct-5akis2u7: the multi-os-compat trigger is retired: the product is Linux-only, the full workspace and fake-agent E2E run on `ubuntu-latest`, and the census `tests/contract_os_gated_tests.rs` reads a gate naming `windows` or `target_os = "macos"` as a fault (`another OS: …`); the Windows/macOS branches are recoverable at tag `multi-os-final`.
sidecar: §1 multi-os-compat trigger retired with the Linux-only ruling; the OS-gate census holds the boundary.
rationale: report Retired (fct-tulramlm) and Harness / gate surface (fct-sy4jxsfp): census `GATES` and its new `another OS` fault class.

@proposal n=39 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp dependent_of=D-retired
section: §9 CI Integration
change: fct-q4ch4yge: the heading reads "Matrix builds" with no "(from the multi-os-compat trigger)".
sidecar: §9 Matrix builds: multi-os-compat source removed.
rationale: report Harness / gate surface (fct-sy4jxsfp): `strategy.matrix.os: [ubuntu-latest]`.

@proposal n=40 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp dependent_of=D-retired
section: §9 CI Integration
change: fct-shyrfs6v: OS reads `[ubuntu-latest]` alone (openpty, Unix socket, modes; also the deny, zizmor, fuzz and MSRV jobs); the `windows-2025` and `macos-latest` legs are removed.
sidecar: §9 Matrix OS: three legs → [ubuntu-latest].
rationale: report Counts (fct-tjt6dmfu): the CI matrix 3 OS legs → `[ubuntu-latest]`; Retired (fct-tulramlm): windows-2025 and macos-latest CI legs.

@proposal n=41 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu dependent_of=D-retired
section: §9 CI Integration
change: fct-kumfcynn: the toolchain is installed by `rustup toolchain install` on the CI runner and the host, not "on every OS".
sidecar: §9 Language version: "on every OS" re-worded.
rationale: report Counts (fct-tjt6dmfu) os-target class: an «on every OS» clause is stale wording on a Linux-only product.

@proposal n=42 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu dependent_of=D-retired
section: §9 CI Integration
change: fct-q4zg6vp5: "Any unit / integration / E2E / contract / property test fails" with "(on any of the 3 OSes)" removed.
sidecar: §9 Build failure conditions: three-OS qualifier removed.
rationale: report Counts (fct-tjt6dmfu): CI matrix 3 OS legs → `[ubuntu-latest]`.

@proposal n=43 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp dependent_of=D-retired
section: §10 Quality Gates & Coverage Targets
change: fct-ryyndnld: gates are enforced in the `ubuntu-latest` coverage job with `cargo llvm-cov nextest … --fail-under-lines 85 --fail-under-functions 95 --fail-under-regions 80`; the per-OS rationale (`#[cfg(windows)]` / `#[cfg(unix)]` code compiling only on its own runner) is removed.
sidecar: §10 Stack adjustments: per-OS gating rationale retired.
rationale: report Harness / gate surface (fct-sy4jxsfp): `test` keeps `[ubuntu-latest]` alone; Retired (fct-tulramlm): windows-2025 / macos-latest legs.

@proposal n=44 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §11 Test Anti-Patterns
change: fct-lktzdsoq: the ban "NEVER treat a Linux-only local run as proof for Windows or macOS. OS-branch code is verified only on its own runner." is removed.
sidecar: §11 Test Strategy: the Windows/macOS proof ban retired.
rationale: report Retired (fct-tulramlm): Linux only; the Windows and macOS code removed.

@proposal n=45 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu dependent_of=D-retired
section: §6 E2E Test Strategy
change: fct-ktlsbc3h: the forced vt100 panic byte search reads "(the child's own bytes, and no viola literal)" with no "on Unix" / "on every OS".
sidecar: §6 tui driver: per-OS wording removed.
rationale: report Counts (fct-tjt6dmfu) os-target class: «on every OS» stale wording.

@proposal n=46 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu dependent_of=D-retired
section: §5 Integration Test Strategy
change: fct-w5xcngvx: the board on the read rule reads "on Linux" (`tests/cli_list.rs` `list_refuses_a_home_another_user_may_read_with_and_without_an_instance`), not "on all three OSes".
sidecar: §5 CLI: three-OS qualifier removed.
rationale: report Counts (fct-tjt6dmfu) os-target class «three OSes» stale.

@proposal n=47 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu dependent_of=D-retired
section: §9 CI Integration
change: fct-346brcpe: Lint drops "per OS" and "the dev-host Windows- and macOS-target check-only clippy runs"; the `--acyclic` review is not a gate.
sidecar: §9 Lint: per-OS and Windows/macOS-target clippy runs removed.
rationale: report Counts (fct-tjt6dmfu): deny `targets` 3 triples → 1 and the CI matrix → `[ubuntu-latest]`.

@proposal n=48 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §9 CI Integration
change: fct-3x7gvt6f: Platform reads GitHub Actions with one `ci.yml` on push and PR and a scheduled `nightly.yml` for fuzz and weekly advisories; the dispatch-only `windows-mutants.yml` is removed.
sidecar: §9 Platform: windows-mutants.yml retired, workflows 3 → 2.
rationale: report Counts (fct-tjt6dmfu): workflows 3 → 2; Retired (fct-tulramlm): `windows-mutants.yml` (the boundary audit's Windows leg).

@proposal n=49 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §9 CI Integration
change: fct-fo2372wn: Mutation reads no push or pull-request job and no gate, mutation at the epoch boundary through `/andromeda-code-audit` and on demand through `run --mutants`; the Windows leg clause is removed.
sidecar: §9 Mutation: the windows-mutants.yml leg retired.
rationale: report Retired (fct-tulramlm): `windows-mutants.yml`.

@proposal n=50 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §9 CI Integration
change: fct-2ymlhohm: `mutants.out/` is never uploaded (no push or pull-request job runs mutation since 2026-09-28); "and the dispatch-only `windows-mutants.yml` uploads nothing" is removed.
sidecar: §9 Test report format: windows-mutants.yml clause removed.
rationale: report Retired (fct-tulramlm): `windows-mutants.yml`.

@proposal n=51 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm dependent_of=D-retired
section: §2 Test Strategy
change: fct-elca5edh: cargo-mutants writes `mutants.out/outcomes.json` at the repository root; the host mutation scratch `<repo parent>/viola-mutants-scratch/` on a Windows host is removed.
sidecar: §2 Agent-runnable invariants: viola-mutants-scratch retired.
rationale: report Retired (fct-tulramlm): `viola-mutants-scratch` / `HOST_SCRATCH` / `scratch_bytes`; Files (fct-yq5vc7zj): `mutants/scratch.rs` deleted.

@proposal n=52 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §3 Test Harness Contract
change: fct-yqn2bvnu: `scripts/agent-run.sh` (POSIX) is the one thin shim running `cargo run -q -p viola-e2e --bin viola-harness -- <command> [flags]` and forwarding its exit code and stdout unchanged; all logic lives once, in Rust. The `scripts/agent-run.ps1` twin and "both shells … on all 3 OSes" are removed.
sidecar: §3 Harness implementation: the {sh,ps1} shim pair → agent-run.sh alone.
rationale: report Harness / gate surface (fct-sy4jxsfp): `scripts/agent-run.ps1` deleted, the harness is `scripts/agent-run.sh` alone; Retired (fct-tulramlm): the `{sh,ps1}` shim pair and the `pwsh shim` CI steps.

@proposal n=53 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp dependent_of=D-retired
section: §3 → Bootstrap phases (derive for route / setup-project)
change: fct-piiirk6o: 5-command-discipline-wire wires `scripts/agent-run.sh` as the shim over `viola-harness` (boot / run / status / cleanup / logs); the `{sh,ps1}` pair and "both shell variants must expose identical semantics" are removed.
sidecar: §3 Bootstrap phases: shim pair → agent-run.sh alone.
rationale: report Harness / gate surface (fct-sy4jxsfp): `scripts/agent-run.ps1` deleted.

@proposal n=54 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp dependent_of=D-retired
section: §3 → 5-command implementation
change: fct-gvvrytf4: `pre-push` is "forwarded by the shim" (not "both shims"), and "The other OSes are CI's alone." is removed.
sidecar: §3 pre-push: both-shims and other-OS wording removed.
rationale: report Harness / gate surface (fct-sy4jxsfp): the harness is `scripts/agent-run.sh` alone; Retired (fct-tulramlm): Linux only.

@proposal n=55 detector=D-tests-count-literals severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: §3 → Bootstrap phases (derive for route / setup-project)
change: fct-2j6rht5l: the `ci` kill line reads "the twenty-six verify-driven binaries" with `conpty_sideload` dropped from the `binary(…)` list; the dev-dep bullet "windows-sys 0.61.2 with `Win32_Security_Authorization` (cfg windows)" is removed; the viola-e2e libc `=0.2.189` reads as a plain dependency, not "under `cfg(unix)`".
sidecar: §3 Bootstrap phases: kill line 27 → 26 binaries (conpty_sideload gone), windows-sys dev-dep removed, libc unconditional.
rationale: report Counts (fct-tjt6dmfu): nextest `ci` kill line `binary(…)` names 27 → 26 (`.config/nextest.toml` line 24), fct-2j6rht5l named stale; Dependencies (fct-www7rgz5): windows-sys removed, libc moved to plain `[dependencies]` in viola-e2e.

@proposal n=56 detector=D-tests-count-literals severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: §3 → 5-command implementation
change: fct-hgewdmj2: `Instant::now() + WITHIN` reads 31 sites in 23 files under `tests/`, measured at this chunk's wrap; the Windows arms of `endpoint_gone` / `holder_gone` (client NotFound, pipe not found, the stopped wrapper's pipe outliving its exit), the Windows host mutation scratch paragraph (`HOST_SCRATCH`, `viola-mutants-scratch`, `scratch-refused` / `scratch-wipe-failed`, `TMP`/`TEMP` and `--output <scratch>`, the `#[cfg(windows)]` wipe unit), "on a Windows host `<scratch>/mutants.out/`" and "(the host scratch arm unchanged, inert off Windows)" are removed.
sidecar: §3 run: WITHIN waits 32/24 → 31/23; Windows endpoint arms and the host mutation scratch retired.
rationale: report Counts (fct-tjt6dmfu): `within-waits` 32 sites in 24 files → 31 in 23, test-plan key fct-hgewdmj2 stale; Symbols (fct-xxutheix): `scratch.rs` whole, `mutants.rs`'s `TMP`/`TEMP`/`--output` arm and `cleanup.rs`'s Windows pipe arm of `endpoint_gone` removed.

@proposal n=57 detector=D-tests-count-literals severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu dependent_of=D-tests-count-literals
section: §3 → 5-command implementation
change: fct-zxaumenb: boot step 4 stamps every relied ledger row (`LedgerRow::relied()`: all twenty-four), dropping "on Unix, elsewhere twenty-one, the first eighteen, …"; boot step 2 drops the Windows protected user + SYSTEM DACL home, the "Windows strict-modes check" sentence and "on Windows it is the plain create".
sidecar: §3 boot: relied set all twenty-four on every build; Windows home DACL arms removed.
rationale: report Counts (fct-tjt6dmfu): relied set Windows twenty-one → none, `LedgerRow::relied()` all twenty-four on every build; Retired (fct-tulramlm): the Windows home owner/DACL arm.

@proposal n=58 detector=D-tests-count-literals severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu dependent_of=D-tests-count-literals
section: §3 → 5-command implementation
change: fct-tuv4h7zt: `--local-live` reads `LEDGER_ROWS: [&str; 24]` alone, the 21-id form under `cfg(not(unix))` and its Windows-mutation-scope note removed; "unit-proven per OS" reads "unit-proven"; the H2 key-after-resize question points to the open residuals of origin `2026-11-04T23-11-43-wrap` (target `another-os`), not "Windows-only live measurements".
sidecar: §3 run --local-live: the 21-row relied form retired; wolm re-pointed to the another-os residuals.
rationale: report Retired (fct-tulramlm): the Windows relied set of twenty-one; «Windows-only live measurements» retired, its residuals → origin `2026-11-04T23-11-43-wrap`, target `another-os`; Expected amendments (fct-pdxuthnr) wolm re-point.

@proposal n=59 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §3 → 5-command implementation
change: fct-zujg4opw: Output format reads `mutants.out/outcomes.json` (the run's own output dir) with "`<scratch>/mutants.out/` on a Windows host" removed, and the sentence "On a Windows host a counted, scoped or package object also carries `"scratch_bytes"` …" is removed.
sidecar: §3 run Output format: the scratch_bytes key and the Windows scratch dir retired.
rationale: report Symbols (fct-xxutheix): `mutants.rs`'s `scratch_bytes` field removed, «the harness document's `scratch_bytes` key, absent on Linux, is gone»; Harness / gate surface (fct-sy4jxsfp).

@proposal n=60 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §3 → 5-command implementation
change: fct-47aec6c3: the `gate` input reads the `mutants` suite's `survived` from the run's own `mutants.out/outcomes.json`, with ", under the host scratch on a Windows host" removed.
sidecar: §3 gate inputs: Windows host scratch removed.
rationale: report Symbols (fct-xxutheix): `scratch.rs` deleted whole (`HOST_SCRATCH`, the Windows scratch wipe).

@proposal n=61 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §3 → 5-command implementation
change: fct-beydn7zm: cleanup step 4 reads the endpoint gone as a connect to `<per-user 0700 dir>/viola-<h12>.sock` refused or the path absent (`$XDG_RUNTIME_DIR/viola/`, `/tmp/viola-<uid>/` as the fallback); the Windows NotFound clause and `$TMPDIR/viola/` on macOS are removed.
sidecar: §3 cleanup: Windows pipe and macOS socket-dir forms retired.
rationale: report Symbols (fct-xxutheix): the macOS `$TMPDIR/viola/` arm removed (forms now `$XDG_RUNTIME_DIR/viola/` and `/tmp/viola-<uid>/`), `cleanup.rs`'s Windows pipe arm of `endpoint_gone` removed.

@proposal n=62 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §3 → 5-command implementation
change: fct-5z3qj22n: the secret-scan document reads `"mode_check":"unix"`, the `skipped-windows` value removed with `secret_scan.rs`'s OS arm.
sidecar: §3 secret-scan: skipped-windows mode_check word retired.
rationale: report Symbols (fct-xxutheix): viola-e2e `secret_scan.rs`'s OS arm removed.

@proposal n=63 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §3 → 5-command implementation
change: fct-mihqkrxe: the `run` `reason` list drops `scratch-refused` and `scratch-wipe-failed` (the Windows-host mutation scratch) from the standing values and names them among the retired, left with `mutants/scratch.rs` at chunk 2026-11-05-linux-only-code-and-ci.
sidecar: §3 Closed enums: scratch-refused / scratch-wipe-failed retired with scratch.rs.
rationale: report Symbols (fct-xxutheix): `scratch.rs` whole (`HOST_SCRATCH`, the Windows scratch wipe, `prepare`) removed; Retired (fct-tulramlm): `viola-mutants-scratch` / `HOST_SCRATCH`.

@proposal n=64 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix dependent_of=D-retired
section: §3 → Test data bootstrap
change: fct-uiwmcfut: the Mechanism drops the "Windows x64, root tests only: `seed_conpty(home)` …" bullet, the `outside_scan()` bullet names "the §5 carve-out" (not "second"), `prepare_home_base` drops "on Windows it is the plain create", and `headroom_refusal` reads "unit-tested" (not "on every OS").
sidecar: §3 Test data bootstrap: seed_conpty and Windows create arm retired, one §5 carve-out.
rationale: report Symbols (fct-xxutheix): `tests/support/home.rs` loses `seed_conpty`; Retired (fct-tulramlm): `target/conpty-seed/`, `bin/<version>-<hash>/conpty/`; Counts os-target «on every OS» stale wording.

@remark
Detail-file sites (kill-line fct-7d3yogoz, deny-triples ×2 and the other Windows/macOS rows in the ci-integration, test-scope-summary, test-strategy, unit- and integration-test-strategy files) are the cascade's and not proposed here; history-only Windows mentions left as measurements (coverage regex separator rationale fct-zxoe3ae7 / fct-3mj52n2x / fct-b4avhaux, jq runner reads fct-z43sni6d, dated windows CI-run readings in fct-hgewdmj2 / fct-2j6rht5l); D-tests-obs-harness: the shim-pair change (proposals 52-54) should be checked against obs-plan §3 for a one-sided harness statement; D-tests-coverage and D-tests-framework found no drift (the new free-bytes function carries 9 unit cases, runner unchanged).
