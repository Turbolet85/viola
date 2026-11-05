@answer n=1 entity=stack-and-technologies#1.1 v=yes
why: the msrv job the scope names must go green on the 1.96 toolchain this row selects, so the harness fix stays inside the 1.96 floor
@answer n=2 entity=stack-and-technologies#1.2 v=no
why: the std-threads and Tokio-only-in-mcp model is untouched by removing Windows arms and target deps; the KEYSTONE CI check is a separate entity
@answer n=3 entity=stack-and-technologies#1.3 v=no
why: the clap feature set is not OS-gated and no chunk unit touches the CLI parser
@answer n=4 entity=stack-and-technologies#1.4 v=yes
why: it names vendor/conpty, the Windows DLL-search restriction and LoadLibraryExW sideload, and the Unix-only libc tty arm, which the chunk removes or un-gates
@answer n=5 entity=stack-and-technologies#1.5 v=no
why: the vt100 screen model is not OS-specific and no chunk unit touches it
@answer n=6 entity=stack-and-technologies#1.6 v=yes
why: the Windows SQOS pipe open leaves, and the Unix socket rules (per-user dir, check_endpoint_dir, chmod 0600, euid peer check) lose their cfg(unix) gate and must hold on Linux alone
@answer n=7 entity=stack-and-technologies#1.7 v=no
why: there is no database and nothing OS-specific; no chunk unit touches the ndjson or snapshot stores
@answer n=8 entity=stack-and-technologies#1.8 v=no standing=S6
why: each log line is one complete JSON object per single write, fixed by the obs-owned diag schemas
@answer n=9 entity=stack-and-technologies#1.9 v=yes
why: the Unix half of the strict-modes and private-dir rules (libc modes and owner) keeps and loses its gate, while the windows-sys owner and DACL half leaves with the Windows arms
@answer n=10 entity=stack-and-technologies#1.10 v=yes
why: the Windows x64 full-SHA-256 check of each ConPTY companion (viola_state::pin::pin_companions) is a removed arm; the exe-bytes hash stays
@answer n=11 entity=stack-and-technologies#1.11 v=no
why: the unbounded_depth ban is a deny.toml entry outside the Windows-target-dependency entries the chunk edits
@answer n=12 entity=stack-and-technologies#1.12 v=no standing=S6
why: each log line is one complete JSON object per single write, fixed by the obs-owned diag schemas
@answer n=13 entity=stack-and-technologies#1.13 v=no
why: the nutype newtypes are not OS-specific and no chunk unit touches them
@answer n=14 entity=stack-and-technologies#1.14 v=no
why: the chrono RFC 3339 timestamp rule is not OS-specific and no chunk unit touches it
@answer n=15 entity=stack-and-technologies#1.15 v=no
why: the per-crate thiserror approach and PtyError's hand-written exception stand after the removal; the derive census is a separate entity
@answer n=16 entity=stack-and-technologies#1.16 v=no
why: per-wrapper local sockets are not OS-specific; the Windows pipe arm binds through entity 1.6
@answer n=17 entity=stack-and-technologies#1.17 v=no
why: sysinfo and claude agents liveness is not OS-specific and no chunk unit touches it
@answer n=18 entity=stack-and-technologies#1.18 v=no
why: the rmcp features and MCP tools are not OS-specific and no chunk unit touches them
@answer n=19 entity=stack-and-technologies#1.19 v=no
why: viola calls no model API, so there is no OS arm to remove
@answer n=20 entity=stack-and-technologies#1.20 v=no
why: the mobile framework is N/A for v1
@answer n=21 entity=stack-and-technologies#1.21 v=no
why: the local cargo install and the include_str! plugin are not OS-gated
@answer n=22 entity=stack-and-technologies#1.22 v=yes
why: the windows-2025 and macos-latest matrix and the test-job pins name the ci.yml and nightly.yml jobs the scope removes, and the pre-push Linux gate binds the Linux CI
@answer n=23 entity=stack-and-technologies#1.23 v=yes
why: the perf and lint job tool lines sit in ci.yml jobs the chunk edits, and the cfg reader behind run --mutants' host exclusion (syn, proc-macro2) serves the Windows mutation path the chunk retires
@answer n=24 entity=stack-and-technologies#1.24 v=no
why: dist release tooling is a later v1.x distribution decision, not an OS arm the chunk touches
@answer n=25 entity=stack-and-technologies#2.1 v=yes
why: the Windows GetNamedPipeServerProcessId pid read and the macOS none-carve-out lose their code, while the Unix euid check on both clients stays
