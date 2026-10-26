proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 14
    section: '## Dependency Security'
    change: '- The root package carries a test-only `[target.''cfg(windows)''.dev-dependencies]` windows-sys (`workspace = true`, the workspace pin `=0.61.2`) with the features `Wdk_Storage_FileSystem` and `Win32_System_IO`, the instruments of the Windows listener admission tests in `tests/security_negatives_channel.rs` (chunk 2026-10-25-windows-endpoint-admission). Both features already reach the Windows graph by unification (`Wdk_Storage_FileSystem` from mio 1.2.4, `Win32_System_IO` from interprocess 2.4.4, mio 1.2.4 and socket2 0.6.5), so `Cargo.lock` does not move, and under `resolver = "3"` no release build carries them; `cargo deny check bans licenses sources` and `scripts/channel-tokio-free.sh` read green with the table, no ignore, skip or allow added.'
    sidecar: 'dependency-security: the root''s test-only Windows windows-sys dev-dependency (Wdk_Storage_FileSystem, Win32_System_IO; lock unchanged, no release build) joins the deps values group (chunk 2026-10-25-windows-endpoint-admission)'
    rationale: 'The report''s Changes, Dependencies: a new root dev-dependency table (Cargo.toml 168-171), two features on the pinned windows-sys =0.61.2, both reached by unification, Cargo.lock unchanged, resolver 3 keeps them out of release builds, cargo deny and the tokio-free tree green; no standing text of the category names it.'
    basis: .andromeda/security-plan-detail/dependency-security.md:14
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/security-plan-detail/dependency-security.md
    near: 54
    section: '## Dependency Security'
    change: 'A new `values · deps-supply-chain` group holding one item: - CI''s `supply-chain` job reads red on RUSTSEC-2026-0386 (`unsound`) against the unchanged tokio 1.53.1 on every run since the last wrap (ci#40247500584 its only red). Its owner is the founder-approved route entry "tokio RUSTSEC-2026-0386 lock bump": a lock bump of tokio to `>=1.53.3`, no ignore-list widening, so RUSTSEC-2017-0008 stays the only expected ignore.'
    sidecar: 'dependency-security: the supply-chain red on RUSTSEC-2026-0386 (tokio 1.53.1) and its owner, the route entry "tokio RUSTSEC-2026-0386 lock bump" (tokio >=1.53.3, no ignore widened), recorded in a new deps-supply-chain values group (chunk 2026-10-25-windows-endpoint-admission)'
    rationale: 'The report''s Outcome records CI ci#40247500584 red 15/15 with supply-chain alone failing on RUSTSEC-2026-0386 against the unchanged tokio 1.53.1, every run since the last wrap, owned by the route entry "tokio RUSTSEC-2026-0386 lock bump" (inputs#I2, inputs#I6: tokio to >=1.53.3, no ignore-list widening, the founder''s yes on the P5 card).'
    basis: .andromeda/security-plan-detail/dependency-security.md:54
    fact: F5
note: F5 records a red owned by a route entry, so the lock bump's own wrap retires this item; the standing deps item at line 14 still names tokio =1.53.1, which is the strong readers' edit, not proposed here.
