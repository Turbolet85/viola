
## 2026-09-25-security-prerequisites — SQOS client open, sha2 content hash, licence MIT OR Apache-2.0, 0BSD exceptions
**Section:** §Stack and Technologies (Wrapper IPC row; new Content hash row); §Established Decisions [Message Broker / IPC]; §Occupied Resources → IPC endpoints (test-only pipe); §Infrastructure Patterns → Build system (shared `[workspace.package]` inheritance, Dependency policy licences, new Licence bullet), Project directory structure (root `LICENSE-MIT`, `LICENSE-APACHE`, `README.md`), Crate dependency direction (`viola-channel` + windows-sys on Windows); §Inherited Defaults → Publishability.
**Change:**
- The Wrapper IPC row and the IPC decision record the Windows client open. It is windows-sys `CreateFileW(SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED)` adopted by `Stream::try_from`, never the default connect. A non-overlapped handle hangs, as measured at the chunk.
- A Content hash row names sha2 `=0.11.0` (`default-features = false`) for the 16-hex `<hash>` keys, a root dev-dependency until `viola-state`.
- `viola-channel`'s planned third-party deps gain windows-sys (Windows only).
- The licence `MIT OR Apache-2.0` is recorded:
  - in `[workspace.package]` + `license.workspace = true`;
  - as a literal in `fuzz/Cargo.toml`;
  - by the root texts, and in Publishability.
- Dependency policy records the licence `allow` list and the per-crate `0BSD` exceptions (`doctest-file`, `recvmsg`), never through `allow`.
- The tree gains the three root files.
- The test-only pipe `\\.\pipe\viola-test-sqos-<pid>-<label>` is registered, outside the `viola-<h12>` namespace.
**Why:**
- The chunk's report: Dependencies, Schema / config, Files, Symbols / APIs, Cross-project claims, Expected amendments.
- Founder ruling 2026-09-25 (licence), relayed by the overseer, whose wrap P2 directive item (4) routes the licence to every arch project-metadata site.
- Operator ratification of the 0BSD exceptions (see security-plan sidecar, same marker; boundary widening).
- Orchestrator-raised (Validate check 5): the `[workspace.package]` inheritance line (:403), a metadata site the detector did not propose.
- Rejected: widening the PTY layer row's windows-sys role (:17). That row's claim, the `TerminateProcess` fallback in `viola-pty`, stays true; the SQOS use belongs to the channel and lands in the IPC row and the dependency direction instead.
**Sweep:**
- Cascade step 2 read every `windows-sys` line across the masters (29).
  - architecture :17, :46 and the tree's `viola-pty` line (kill fallback, own ConPTY): true, no change;
  - :19, :51 and :430 amended.
- `viola-channel\` → \`viola-core\`, interprocess\.` = 0 after the apply (1 before).
- `publish = false` metadata sites: :350 and :351 (per-crate registry notes, no metadata restatement) no change; :403, :418 (+ new Licence bullet) and :589 amended.
- Leaves re-derived:
  - `.claude/docs/stack.md` (Wrapper IPC row, Content hash row, licence line);
  - `.claude/docs/conventions.md:55` (`license.workspace = true`);
  - `.claude/docs/services/viola-channel.md` (the resolved spike).
- CLAUDE.md `GENERATED` blocks recomputed from the amended sections: no line changes (none states the IPC open, the hash crate or the licence).
