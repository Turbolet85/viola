@proposal n=1 detector=D-security-auth severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Authentication & Authorization → Library (fct-7nb5glkx)
change: IPC: interprocess 2.4.4 (`Stream::peer_creds()`: its `pid()` the client's read of the serving process over Linux `SO_PEERCRED`, its `euid()` the read of the peer on both sides, the wrapper reading a client and a client reading the server), std chmod on the bound Unix socket, and sysinfo 0.39.6 (pid + start time); the windows-sys entry (`GetTokenInformation`, SID/SDDL conversion, SQOS `CreateFileW`), `ListenerOptionsExt::security_descriptor` and the Windows `GetNamedPipeServerProcessId` / macOS no-pid clauses leave, recoverable at the tag `multi-os-final`; the `ListenerOptionsExt::mode` not-used note stays.
sidecar: 2026-11-05 linux-only-code-and-ci — §Authentication Library narrowed to the Linux IPC reads; windows-sys and the Windows/macOS peer-read clauses retired with their code (founder rulings 2026-11-04, inputs#I1, inputs#I5 item 6).
rationale: Report fct-www7rgz5 removes `windows-sys` from every manifest and fct-xxutheix removes the named-pipe/SQOS arms and folds `SERVER_PID_REPORTED`/`PEER_EUID_REPORTED` to their Linux value; fct-tulramlm retires the named-pipe endpoint with its SDDL/SQOS controls and the macOS no-peer-pid carve-out, so the doc's auth library list no longer matches the flow the code uses.
@proposal n=2 detector=D-security-auth severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Authentication & Authorization → IPC client-side server verification (fct-phspeqis)
change: Before the first frame of any method the client runs the instance's strict-modes check (a refusal writing no frame and no line), checks the recorded endpoint's socket directory before the connect (refusal `socket-dir-refused`), then verifies the server against its one snapshot read: the connected peer's pid and that process's start time, and past the connect admits only a peer of its own euid, an unreadable peer refused; a mismatch, a snapshot replayed from the log (no endpoint, pid or start time) or a check that cannot be made reads `server-verify-failed` and sends no frame, never falling back to a default connect; the Windows SQOS identification-level open and the macOS no-pid carve-out are retired.
sidecar: 2026-11-05 linux-only-code-and-ci — client server verification restated as the single Linux flow; Windows SQOS open and macOS no-peer-pid carve-out retired with `server_verdict`'s `peer_pid_reported` parameter and macOS `Matches` arm.
rationale: Report fct-xxutheix: `liveness::server_verdict` / `verify_server` lose `peer_pid_reported` and the macOS `Matches` arm, `peer_euid_admitted(observed, own)` is `observed == Some(own)`, an unreadable euid still refused; the SQOS open arms of `client.rs` removed; fct-tulramlm retires the macOS no-peer-pid carve-out and the SQOS controls.
@proposal n=3 detector=D-security-auth severity=warning dependent_of=D-security-auth basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Security Anti-Patterns → Authentication (fct-6cauxi6n)
change: NEVER write any channel frame before verifying the server (pid, start time and euid, after the socket-directory check) against a `snapshot.json` that passed the strict-modes check in the same process; no carve-out exists (the macOS pid/start-time carve-out retired with macOS support).
sidecar: 2026-11-05 linux-only-code-and-ci — server-verification ban loses the macOS carve-out clause (retired, inputs#I1).
rationale: Report fct-tulramlm retires «the macOS no-peer-pid carve-out»; fct-xxutheix removes the macOS `Matches` arm of `server_verdict`; the ban's exception now names a retired item.
@proposal n=4 detector=D-security-auth severity=warning dependent_of=D-security-auth basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Security Anti-Patterns → Authentication (fct-gqlrkxte)
change: Remove the ban on opening a Windows client pipe without `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION`: the Windows named-pipe client is retired with the Windows code (recoverable at `multi-os-final`).
sidecar: 2026-11-05 linux-only-code-and-ci — SQOS client-pipe ban retired with the Windows named-pipe endpoint.
rationale: Report fct-tulramlm retires the Windows named-pipe endpoint with its SDDL / SQOS / `accept_remote(false)` controls; fct-xxutheix removes the SQOS open arms of `client.rs`, `endpoint.rs`, `tokio_client.rs`.
@proposal n=5 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Authentication & Authorization → IPC access control (Windows) (fct-kny6sapm)
change: Remove the IPC access control (Windows) row (protected user+SYSTEM DACL, remote-client rejection, first-pipe-instance, non-inheritable handle): the Windows named-pipe endpoint is retired; IPC access control is the Unix row alone.
sidecar: 2026-11-05 linux-only-code-and-ci — IPC access control (Windows) row retired with the named-pipe endpoint and its SDDL controls (inputs#I1, inputs#I5 item 6).
rationale: Report fct-tulramlm retires «the Windows named-pipe endpoint (`\\.\pipe\viola-<h12>`), its SDDL / SQOS / `accept_remote(false)` controls»; fct-xxutheix deletes `server/win.rs` and the named-pipe arms.
@proposal n=6 detector=D-retired severity=warning dependent_of=D-retired
section: §Threat Model Summary → Security tier justification (fct-55ydrrgu)
change: Replace «its names are predictable in the shared Windows pipe namespace, the Unix socket standing in a per-user 0700 directory» with «its socket name is predictable, the socket standing in a per-user 0700 directory», and drop the «per-OS» qualifier of the enforcement clause.
sidecar: 2026-11-05 linux-only-code-and-ci — tier justification no longer names the retired Windows pipe namespace.
rationale: Report fct-tulramlm retires the Windows named-pipe endpoint and the OS target «Windows first, macOS and Linux» → «Linux only».
@proposal n=7 detector=D-retired severity=warning dependent_of=D-retired
section: §Threat Model Summary → Infrastructure: Networking (fct-2jp5ovui)
change: Per-instance Unix domain sockets in a verified per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`), never the `/tmp` root; no Linux abstract namespace — the «named pipes (Windows)» alternative removed.
sidecar: 2026-11-05 linux-only-code-and-ci — Networking bullet loses the retired Windows named pipes and the macOS `$TMPDIR` form.
rationale: Report fct-xxutheix: socket-directory forms now `$XDG_RUNTIME_DIR/viola/` and `/tmp/viola-<uid>/`, the macOS `$TMPDIR/viola/` arm and the named-pipe arms removed; fct-tulramlm retires both.
@proposal n=8 detector=D-retired severity=warning dependent_of=D-retired
section: §Data Protection → In transit: TLS (fct-6ikkebgm)
change: TLS: N/A. The IPC channel is local Unix sockets and MCP is stdio; confidentiality in transit depends on the socket-directory and peer checks (Authentication & Authorization).
sidecar: 2026-11-05 linux-only-code-and-ci — in-transit row no longer names the retired named pipes or IPC ACLs.
rationale: Report fct-tulramlm retires the Windows named-pipe endpoint and its SDDL controls.
@proposal n=9 detector=D-retired severity=warning dependent_of=D-retired
section: §Security Anti-Patterns → Authentication (fct-4vfalbv2)
change: Remove the ban on creating an interprocess Windows listener without `ListenerOptionsExt::security_descriptor`: no Windows listener exists (retired, recoverable at `multi-os-final`).
sidecar: 2026-11-05 linux-only-code-and-ci — Windows listener security-descriptor ban retired with `server/win.rs`.
rationale: Report fct-xxutheix deletes `crates/viola-channel/src/server/win.rs`; fct-tulramlm retires the SDDL controls.
@proposal n=10 detector=D-retired severity=warning dependent_of=D-retired
section: §Security Anti-Patterns → Universal (fct-7ackeyph)
change: In the list of controls no switch may disable, replace «the IPC DACL or socket-directory check» with «the socket-directory check».
sidecar: 2026-11-05 linux-only-code-and-ci — not-disableable control list drops the retired IPC DACL.
rationale: Report fct-tulramlm retires the Windows endpoint's SDDL controls.
@proposal n=11 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Threat Model Summary → Attack surface: Filesystem state (fct-2i3lkhzs)
change: Replace «the strict-modes check (owner and mode, or owner and DACL)» with «the strict-modes check (owner and mode)».
sidecar: 2026-11-05 linux-only-code-and-ci — filesystem vector loses the retired Windows owner/DACL strict-modes arm.
rationale: Report fct-tulramlm retires «the Windows home owner/DACL strict-modes arm»; fct-xxutheix removes `strict.rs`'s Windows owner/DACL arm and the off-Unix `Ok(())` arm of `check_socket_dir`.
@proposal n=12 detector=D-retired severity=warning dependent_of=D-retired
section: §Authentication & Authorization → `~/.viola/` access control (fct-jya3vgt6)
change: The home is owner-only (0700 directories and 0600 files, the pinned `viola` 0700), every replaced file written through `replace_private`; a strict-modes check refuses a trusted path another user can write, a home, instance directory or `diagnostics/` another user can read (any group or other bit) or a symlinked `diagnostics/` folder or file — the Windows profile-DACL and allow-ACE clauses removed.
sidecar: 2026-11-05 linux-only-code-and-ci — home access control restated without the retired Windows DACL arm.
rationale: Report fct-tulramlm retires the Windows home owner/DACL strict-modes arm; fct-xxutheix removes `windows_read_verdict`, `READ_RIGHTS`, `outside_profile` and `create_private_dir`'s DACL arm.
@proposal n=13 detector=D-retired severity=warning dependent_of=D-retired
section: §Data Protection → Files (`~/.viola/`) (fct-7aw7emh6)
change: Protected by OS permissions only (0700 directories and 0600 files, the pinned `viola` 0700); drop the Windows DACL clause, the «on Unix only» qualifier of the settings override, and «and the ConPTY companions» from the re-hash clause (the pinned binary alone is re-hashed before use).
sidecar: 2026-11-05 linux-only-code-and-ci — Files row loses the Windows DACL and the retired ConPTY companions.
rationale: Report fct-tulramlm retires the Windows owner/DACL arm and the ConPTY sideload; fct-xxutheix removes `pin_companions` (`pin_bytes` and the pinned copy's re-hash stay).
@proposal n=14 detector=D-retired severity=warning dependent_of=D-retired
section: §Security Anti-Patterns → Data Protection (fct-fmxdaj54)
change: Keep the umask / 0700 / 0600 / pinned-0700 ban; remove the Windows clauses (`--home` outside `%USERPROFILE%`, the owner+DACL check's `#[cfg(windows)]` arm, NULL DACL, non-persistent-ACL volume, inherit-only `GENERIC_*` grant).
sidecar: 2026-11-05 linux-only-code-and-ci — Data Protection ban drops its retired Windows DACL clauses.
rationale: Report fct-xxutheix removes `strict.rs`'s Windows arm, `Refused::NullDacl`, `Refused::NoPersistentAcls`, `outside_profile`; fct-tulramlm retires the arm.
@proposal n=15 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Input Validation → DLL search order + sideloaded ConPTY companions (fct-oqqkxxum)
change: Remove the row: the ConPTY sideload and `restrict_dll_search` are retired (PTY backend `openpty` only), recoverable at `multi-os-final`.
sidecar: 2026-11-05 linux-only-code-and-ci — DLL search / ConPTY sideload row retired with `viola_pty::sideload` and `src/conpty.rs`.
rationale: Report fct-tulramlm retires the ConPTY sideload; fct-xxutheix removes `mod sideload` (`restrict_dll_search`, the pre-load), `main.rs`'s pre-load and `src/conpty.rs`; `pty_backend()` is always `openpty`.
@proposal n=16 detector=D-retired severity=warning dependent_of=D-retired
section: §Security Anti-Patterns → Universal (fct-my3j53so)
change: Remove the ban on reaching a DLL load before `viola_pty::sideload::restrict_dll_search()` and on pre-loading `conpty.dll`: the sideload module no longer exists.
sidecar: 2026-11-05 linux-only-code-and-ci — DLL-search ban retired with the ConPTY sideload.
rationale: Report fct-xxutheix removes `mod sideload` and `src/main.rs`'s `restrict_dll_search`; fct-tulramlm retires the sideload.
@proposal n=17 detector=D-retired severity=warning dependent_of=D-retired
section: §Security Anti-Patterns → Data Protection (fct-gmz2hnmb)
change: Keep the pinned-binary re-hash, cryptographic-hash and plugin-rewrite bans (path `bin/<version>-<hash>/viola`); remove the `bin/<version>-<hash>/conpty/` companion re-hash ban.
sidecar: 2026-11-05 linux-only-code-and-ci — companion re-hash ban retired with the ConPTY sideload.
rationale: Report fct-xxutheix removes `pin.rs`'s `pin_companions` and the companions' held open; fct-tulramlm retires `bin/<version>-<hash>/conpty/`.
@proposal n=18 detector=D-retired severity=warning dependent_of=D-retired
section: §Dependency Security → Audit tool (fct-y5ir4euu)
change: Remove the vendored-ConPTY own-audit paragraph (`scripts/conpty-vendor.sh --verify` / `--probe`): the vendored binaries and the script are deleted.
sidecar: 2026-11-05 linux-only-code-and-ci — vendored ConPTY audit retired with `vendor/conpty/` and `scripts/conpty-vendor.sh`.
rationale: Report fct-yq5vc7zj deletes `scripts/conpty-vendor.sh` and the whole `vendor/conpty/` tree; fct-tulramlm retires the `dep-conpty` supply-chain row.
@proposal n=19 detector=D-retired severity=warning dependent_of=D-retired
section: §Dependency Security → Pinning (fct-hzqtl6ih)
change: Remove the paragraph on the vendored ConPTY package's four pins in `src/conpty.rs`: the file and the vendored package are deleted.
sidecar: 2026-11-05 linux-only-code-and-ci — ConPTY pin paragraph retired with `src/conpty.rs`.
rationale: Report fct-yq5vc7zj deletes `src/conpty.rs` and `vendor/conpty/`; fct-tulramlm retires the sideload.
@proposal n=20 detector=D-retired severity=warning dependent_of=D-retired
section: §Dependency Security → CI integration (fct-v76r7jfp)
change: Remove the bullet on the `windows-2025`-only `ConPTY vendor verification` step of the `test` job.
sidecar: 2026-11-05 linux-only-code-and-ci — ConPTY vendor verification CI step retired.
rationale: Report fct-sy4jxsfp: the `ConPTY vendor verification` step leaves `ci.yml`; fct-tulramlm retires it.
@proposal n=21 detector=D-retired severity=warning dependent_of=D-retired
section: §Bootstrap phases → dep-audit-tooling-install (fct-n3fibuep)
change: cargo-deny `>=0.20.2` with the `deny.toml` `[advisories]`, `[sources]` and `[[bans.features]]` additions; zizmor `>=1.30.1`; cargo-audit `>=0.22.2` deferred to v1.x binary scans — the vendored-ConPTY script clause removed.
sidecar: 2026-11-05 linux-only-code-and-ci — bootstrap dep-audit phase drops the retired `conpty-vendor.sh`.
rationale: Report fct-yq5vc7zj deletes `scripts/conpty-vendor.sh`; fct-tulramlm retires the sideload.
@proposal n=22 detector=D-retired severity=warning dependent_of=D-retired
section: §Bootstrap phases → dep-security-ci-gate (fct-reffq76w)
change: Wire the scheduled weekly `cargo deny check advisories`, the zizmor step, SHA-pinned actions and `permissions: {}` / `contents: read` into `ci.yml` — the `windows-2025` ConPTY vendor verification sentence removed.
sidecar: 2026-11-05 linux-only-code-and-ci — bootstrap CI gate drops the retired ConPTY step.
rationale: Report fct-sy4jxsfp removes the `ConPTY vendor verification` step; fct-tulramlm retires the `windows-2025` leg.
@proposal n=23 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-xxutheix
section: §Input Validation → Child executable resolution (fct-sgmmo3ki)
change: Replace «on Windows a `.cmd`/`.bat` child is refused, except a `claude` shim, resolved to its fixed `claude.exe` sibling when that exists» with «a given path is used as given, a bare name is resolved on `PATH` (`ProgramNotFound` otherwise)».
sidecar: 2026-11-05 linux-only-code-and-ci — child resolution loses the retired `.cmd`/`.bat` refusal and shim resolution.
rationale: Report fct-xxutheix removes `Search::windows`, `pathext`, `is_script`, `SHIM_TARGET`, `AgentError::BatchScriptChild` («a given path as given, a bare name on `PATH`; `ProgramNotFound` unchanged»); fct-tulramlm retires the `.cmd`/`.bat` refusal.
@proposal n=24 detector=D-retired severity=warning dependent_of=D-retired
section: §Security Anti-Patterns → Data Protection (fct-tpzheerr)
change: Remove the ban on spawning a `.cmd`/`.bat` PTY child through portable-pty (a Windows BatBadBut hazard; the refusal is retired with the Windows code).
sidecar: 2026-11-05 linux-only-code-and-ci — `.cmd`/`.bat` spawn ban retired with `batch-script-child`.
rationale: Report fct-tulramlm retires «the `.cmd`/`.bat` refusal (`batch-script-child`, `AgentError::BatchScriptChild`)».
@proposal n=25 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Input Validation → Persistent-environment names (Windows registry) (fct-cpef5o24)
change: Remove the row: `viola run` no longer reads the Windows registry environment.
sidecar: 2026-11-05 linux-only-code-and-ci — registry environment row retired with `src/run/env.rs`'s `HKCU\Environment` reader.
rationale: Report fct-xxutheix removes `src/run/env.rs`'s registry reader and case-insensitive match (the exact strip, `claude_env_keep` and the eleven-name identity floor stay); fct-tulramlm retires the Windows registry environment reader.
@proposal n=26 detector=D-retired severity=warning dependent_of=D-retired
section: §Secret Management → Storage: Production (fct-ugohyrrh)
change: Replace «removed from the child by the R8 rule (the identity floor whatever the persistent set says)» with «removed from the child by the R8 rule (exact-name strip, the identity floor always stripped)».
sidecar: 2026-11-05 linux-only-code-and-ci — R8 statement no longer names the retired persistent (registry) set.
rationale: Report fct-xxutheix removes the registry reader and the case-insensitive match; the exact strip and the identity floor stay.
@proposal n=27 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-sy4jxsfp
section: §Threat Model Summary → CI/CD (fct-ju2d5wlo)
change: GitHub Actions `ci.yml` on push and PR on `ubuntu-latest` (each job's `strategy.matrix.os: [ubuntu-latest]`), plus the weekly `nightly.yml` advisory and fuzz run; no deploy stage, no release artefacts and no secrets in v1.
sidecar: 2026-11-05 linux-only-code-and-ci — CI/CD line narrowed to the Linux leg; `windows-2025` and `macos-latest` legs retired.
rationale: Report fct-sy4jxsfp: `test`, `perf`, `lint`, `release` keep `strategy.matrix.os: [ubuntu-latest]`; fct-tulramlm retires the `windows-2025` and `macos-latest` CI legs.
@proposal n=28 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tjt6dmfu
section: §Dependency Security → CI integration (fct-4ggr436n)
change: CI integration (`.github/workflows/ci.yml`, `.github/workflows/nightly.yml`) — `windows-mutants.yml` removed from the list.
sidecar: 2026-11-05 linux-only-code-and-ci — workflows 3 → 2; `windows-mutants.yml` deleted.
rationale: Report fct-tjt6dmfu: workflows 3 → 2 (`ci.yml`, `nightly.yml`); fct-yq5vc7zj deletes `.github/workflows/windows-mutants.yml`; fct-tulramlm retires it.
@proposal n=29 detector=D-retired severity=warning dependent_of=D-retired
section: §Dependency Security → CI integration (fct-lpbl5pyp)
change: Remove the `windows-mutants.yml` paragraph (dispatch-only epoch audit workflow): the workflow is deleted.
sidecar: 2026-11-05 linux-only-code-and-ci — `windows-mutants.yml` paragraph retired.
rationale: Report fct-yq5vc7zj deletes `windows-mutants.yml`; fct-tulramlm retires the boundary audit's Windows leg.
@proposal n=30 detector=D-retired severity=warning dependent_of=D-retired
section: §Dependency Security → CI integration (fct-pxt7iprv)
change: zizmor's pedantic `concurrency-limits` reads 2 low findings, one per workflow (`ci.yml`, `nightly.yml`); drop the «and `windows-mutants.yml` uploads nothing» clause of the parenthetical, the rest of the rationale kept.
sidecar: 2026-11-05 linux-only-code-and-ci — zizmor `concurrency-limits` 3 → 2 with `windows-mutants.yml` gone.
rationale: Report fct-tjt6dmfu: zizmor `concurrency-limits` suppressions 3 → 2 (`No findings to report. Good job! (2 suppressed)` over `ci.yml` and `nightly.yml`), stale where a count or `windows-mutants.yml` is named.
@proposal n=31 detector=D-retired severity=warning dependent_of=D-retired
section: §Dependency Security → CI integration (fct-npplilir)
change: Drop the final sentence «The dispatch-only `windows-mutants.yml` runs the `--package` arm, which reads no base.»
sidecar: 2026-11-05 linux-only-code-and-ci — event-payload bullet drops the deleted `windows-mutants.yml`.
rationale: Report fct-yq5vc7zj deletes `windows-mutants.yml`; fct-tulramlm retires it.
@proposal n=32 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Security Anti-Patterns → Code Patterns (fct-z5qs3bqk)
change: Remove the ban on `run --mutants` wiping the Windows host mutation scratch `<repo parent>/viola-mutants-scratch`: the scratch and its wipe are deleted.
sidecar: 2026-11-05 linux-only-code-and-ci — mutation-scratch ban retired with `mutants/scratch.rs`.
rationale: Report fct-xxutheix deletes `scratch.rs` (`HOST_SCRATCH`, the Windows scratch wipe, `prepare`); fct-tulramlm retires `viola-mutants-scratch` / `HOST_SCRATCH` / `scratch_bytes`.
@proposal n=33 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-wrfywvkp
section: §Dependency Security → `deny.toml` additions: `[licenses]` (fct-32d4ty7r)
change: `0BSD` is admitted only per crate, by one `[[licenses.exceptions]]` for `doctest-file`, the 0BSD crate interprocess 2.4.4 pulls in on the Linux graph; the `recvmsg` exception is dropped (Windows-only dependency, out of the graph); never added to `allow`, each new exception needing its own Decisions Log entry.
sidecar: 2026-11-05 linux-only-code-and-ci — licence exceptions 2 → 1; `recvmsg` dropped at this wrap (inputs#I5 item 7).
rationale: Report fct-wrfywvkp: `deny.toml`'s licence exceptions now `doctest-file` alone (`recvmsg` is interprocess's Windows-only dependency, out of the Linux graph; CI job 127253812277 warned `license-exception-not-encountered`); fct-tjt6dmfu lists security-plan fct-32d4ty7r as stale.
@proposal n=34 detector=D-retired severity=warning basis_doc=doc:2026-11-05-linux-only-code-and-ci/report basis_fact=fct-tulramlm
section: §Authentication & Authorization → Auth approach (fct-qkuk7pih)
change: OS-user ambient trust, enforced on Linux, on the IPC endpoint and `~/.viola/`.
sidecar: 2026-11-05 linux-only-code-and-ci — «enforced per OS» re-worded to the Linux statement.
rationale: Report fct-tulramlm: OS target → «Linux only»; fct-tjt6dmfu `os-target` class (per-OS wording stale on a Linux-only product).
@proposal n=35 detector=D-retired severity=warning dependent_of=D-retired
section: §Bootstrap phases → auth-scaffolding-baseline (fct-pz6zubwd)
change: Replace «per-OS listener hardening» with «listener hardening (socket directory, chmod 0600, peer euid check)», the rest kept.
sidecar: 2026-11-05 linux-only-code-and-ci — bootstrap auth phase re-worded off the per-OS hardening.
rationale: Report fct-tulramlm retires the Windows listener controls with the OS target; fct-tjt6dmfu `os-target` class.
@remark
Not proposed: fct-p7rzex2i and fct-iftwwapx (handle inheritance) justify the no-leak property by Windows `CreateProcessW` `bInheritHandles`; the report states no Linux mechanism, so their Linux restatement needs an owner read. Detail category files (threat-model-summary, authentication-authorization, input-validation, dependency-security dep-conpty groups, security-anti-patterns) carry many more retired-item sites: the cascade's.
