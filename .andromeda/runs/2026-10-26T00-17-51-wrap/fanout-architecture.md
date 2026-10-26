proposals:
  - detector: D-arch-decisions
    severity: warning
    section: "§Infrastructure Patterns → Crate dependency direction"
    change: "The `viola` root bin bullet gains its one Windows-only dev-dependency: windows-sys (the workspace pin =0.61.2) under `[target.'cfg(windows)'.dev-dependencies]` with features `Wdk_Storage_FileSystem` and `Win32_System_IO` (both needed by `NtQueryInformationFile` and its `IO_STATUS_BLOCK`), used only by `tests/security_negatives_channel.rs`'s pipe-type read-back; both features already reach the Windows graph by unification (mio, interprocess, socket2), `Cargo.lock` does not move, and under resolver 3 no release build carries them (chunk 2026-10-25-windows-endpoint-admission)."
    sidecar: "Crate dependency direction: the root bin's new Windows dev-dependency windows-sys with `Wdk_Storage_FileSystem` + `Win32_System_IO` (test-only, no lock move) recorded."
    rationale: "The report's Dependencies bullet lands a NEW root dev-dependency table `[target.'cfg(windows)'.dev-dependencies] windows-sys = { workspace = true, features = [Wdk_Storage_FileSystem, Win32_System_IO] }` (Cargo.toml 168-171); its Expected amendments name this key for it and inputs#I7 requires both features named; the root bin bullet today lists windows-sys only as a normal dependency and no master names these features (pattern wsys-dev, control unfired)."
    basis: "Cargo.toml:168-171"
note: "No other drift: no crate, endpoint, IPC method, event, env var or library added (Crates / modules none; Dependencies none added or bumped); the new test pipes stay in the registered `viola-test-chan-<pid>-<label>` namespace, which architecture.md:261 already names with tests/security_negatives_channel.rs (sweep row neg-file, true); Retired none."
