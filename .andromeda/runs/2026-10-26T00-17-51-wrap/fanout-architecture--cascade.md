proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/stack-and-technologies.md
    near: 11
    section: '## Stack and Technologies'
    change: '| Backend framework (concurrency model) | Hybrid: std threads for `run`, `hook`, `send`, `wait` and every other CLI verb; Tokio 1.53.1 built only inside `mcp` | No async runtime on the hot `hook` path. The async ecosystem is used only where rmcp requires it. The pinned tokio 1.53.1 carries the `unsound` advisory RUSTSEC-2026-0386 and fails the CI `supply-chain` job (ci#40247500584, its one red); the route entry "tokio RUSTSEC-2026-0386 lock bump", placed right after "Windows endpoint admission", owns it: a lock bump to `>=1.53.3`, with no ignore-list widening |'
    sidecar: 'stack-and-technologies: the Backend framework row records that the pinned tokio 1.53.1 fails CI supply-chain on RUSTSEC-2026-0386, owned by the route entry "tokio RUSTSEC-2026-0386 lock bump" (chunk 2026-10-25-windows-endpoint-admission)'
    rationale: 'Report, Cross-project / external claims (ci#40247500584 full row red only on supply-chain, `unsound` RUSTSEC-2026-0386, tokio 1.53.1) and Outcome gates (the report-only CI conclusion, owned by "tokio RUSTSEC-2026-0386 lock bump", inputs#I2 / inputs#I6); the Beyond-the-plan route insertion: a bump to >=1.53.3, no ignore widened. The version stays 1.53.1 (Dependencies: none bumped), so the row keeps its pin and gains the advisory and its owner.'
    basis: .andromeda/architecture-detail/stack-and-technologies.md:11
    fact: F2
note: F1 (the root Windows dev-dependency windows-sys with `Wdk_Storage_FileSystem` and `Win32_System_IO`) and F3 (the Windows listener admission asserted on CI) have no picked entity of their category, so the new-detail pass places them. F1 goes beside the Workspace crates test-only feature item (architecture.md:292) and the Infrastructure Patterns Crate dependency direction key; F3 goes beside the cross-cutting Local endpoint trust boundary (architecture.md:420), which binds with security-plan Authentication & Authorization. established-decisions#7.1's add-to is that F3 fact, so this pass places nothing there. The Inherited Defaults tokio 1.53.1 pin (architecture.md:442) still holds.
