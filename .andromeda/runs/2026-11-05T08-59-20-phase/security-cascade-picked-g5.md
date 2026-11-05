@account n=1 entity=dependency-security#1.1 v=carried
why: the chunk removes `vendor/conpty/`, the subject of the `conpty-vendor.sh` gate and its `windows-2025` CI step
@account n=2 entity=dependency-security#2.1 v=carried
why: the chunk edits `deny.toml` and the task re-asserts its `[[bans.features]]` entries and `deny-probes.sh` stay live
@account n=3 entity=dependency-security#3.1 v=carried
why: the task keeps the C-crate bans, `deny-sync.toml`, `channel-tokio-free.sh` and `deny-probes.sh` live after the manifest edits
@account n=4 entity=dependency-security#4.1 v=carried
why: the chunk edits `nightly.yml` and `ci.yml`, and the task keeps the separate `fuzz/Cargo.lock` audit and the `fuzz/` exemption on Linux
@account n=5 entity=dependency-security#5.1 v=carried
why: `src/conpty.rs`, the four pins' one textual home, and `vendor/conpty/` are removed
@account n=6 entity=dependency-security#6.1 v=carried
why: the chunk deletes `windows-mutants.yml` whole
@account n=7 entity=dependency-security#7.1 v=carried
why: the `test` and `perf` jobs that run G2 lose their Windows and macOS legs and the task re-asserts G2's one exemption and its probe
@account n=8 entity=dependency-security#8.1 v=not_in_scope standing=S1
why: each CI job or step kept uses SHA-pinned actions with a version comment and checkout with persist-credentials: false
@account n=9 entity=bootstrap-phases#1.1 v=carried
why: the Windows arms of listener hardening, client server verification and the strict-modes checks leave, the Unix arms staying whole on Linux
@account n=10 entity=bootstrap-phases#2.1 v=carried
why: the per-OS scan-gated uploads shrink to the Linux leg and the nightly fuzz upload stays the one unscanned upload
@account n=11 entity=secret-management#1.1 v=carried
why: the Windows registry reader of persistent-environment names leaves, the R8 strip keeping its Linux half
