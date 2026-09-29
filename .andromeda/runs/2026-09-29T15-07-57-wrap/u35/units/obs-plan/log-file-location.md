### Log file location

- **Path:** `<home>/diagnostics/{run-<name>,hook-<name>,mcp,ui-<port>,cli-<name>}.ndjson`.
  - `<home>` is resolved `--home` → grandparent of `VIOLA_DIR` → `~/.viola/`. The basenames and root follow tests (binding); `cli-<name>` is accepted (D-06) and added to the tests file list by amendment D-21. Arch names only `instances/<name>/diagnostics/`, so the home-level root plus instance detail files (D-08) go to arch as amendment request D-22.
  - Content-bearing detail goes to `<home>/instances/<name>/diagnostics/detail-<process>.ndjson` (D-08), which satisfies security Vector 2: only instance-scoped diagnostics may hold user-derived content.
  - Home-level files are **codes-only by construction**: they carry no user content, token, cookie or `CLAUDE*` value.
  - Permissions: files 0600 in 0700 directories on Unix; the home DACL (user, SYSTEM, Administrators) on Windows. The tests secret-scan test enforces both.
- **Rotation:** **N/A in v1, and no rotation library.** Retention is the lifetime of the session home (tests `logs`; obs-scope §5 audit-log-retention). `events.ndjson` must never be rotated by a generic library, because byte offsets are the `wait after`, `send cursor` and `Last-Event-ID` cursors.
  - Future rotation of `diagnostics/` only: the candidate is logroller 0.1.12. Phase 5 must first verify 0600 creation, keep `xz2` off (`lzma-sys` is a C-building crate), keep `flate2` on its pure-Rust backend, and accept the duplicate `thiserror 1.x`.
  - tracing-appender 0.2.5 is rejected: its `non_blocking` writer drops lines, and it pulls in `time` (RUSTSEC-2026-0009 range).
