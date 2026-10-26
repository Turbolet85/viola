proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 15
    section: "## Stack and Technologies"
    change: "In the Wrapper IPC row's Technology cell, after the clause on the server's pipe DACL (before `; on Unix`), add: the root package's Windows-only dev-dependency on windows-sys 0.61.2 (`[target.'cfg(windows)'.dev-dependencies]`, features `Wdk_Storage_FileSystem` and `Win32_System_IO`) serves `tests/security_negatives_channel.rs`'s pipe-type read-back (`NtQueryInformationFile`, `FilePipeLocalInformation`) and its anonymous-token open; both features already reach the Windows graph by unification, so `Cargo.lock` does not move and no release build carries them."
    sidecar: "2026-10-25-windows-endpoint-admission: Stack Wrapper IPC row names the root Windows dev-dependency on windows-sys with both features Wdk_Storage_FileSystem and Win32_System_IO, test-only."
    rationale: "The report's Changes, Dependencies bullet lands a NEW root dev-dependency table `[target.'cfg(windows)'.dev-dependencies] windows-sys` with two features (Cargo.toml 168-171), lock unchanged, no release graph reach; the stack table already records the root's test-only dev-dependency for sha2 (Content hash row) and every windows-sys role per layer, and inputs#I7 asks that the architecture amendment name both features. No new library or contradicted decision (D-arch-decisions holds: windows-sys 0.61.2 is already the pinned stack crate)."
    basis: viola-0.1.0/chunks/2026-10-25-windows-endpoint-admission/report.md:40
note: "The plan's primary site for this fact is §Infrastructure Patterns -> Crate dependency direction (a contracts key file, outside this category); this row's proposal is the Stack table's own record of it, so the orchestrator should keep both wordings naming the same two features."
