
## 2026-09-29-sideloaded-conpty — the Windows x64 child is hosted in Microsoft's sideloaded ConPTY
**Section:** Stack and Technologies → PTY layer, Content hash; Established Decisions → [PTY]; Infrastructure Patterns → Project directory structure
**Change:**
- [PTY] as-built: every `viola` process restricts its DLL search to System32 as the second statement of `main` (`viola_pty::sideload::restrict_dll_search`); `run` pre-loads the pinned `bin/<version>-<hash>/conpty/conpty.dll` by absolute path, so portable-pty `=0.8.1`'s bare-name load returns it and the child is hosted by the pinned `OpenConsole.exe`; the seam is unchanged and knows no pinned path. `pty_backend()` reads `conpty-sideload` · `conpty` · `openpty`; any sideload failure fails open to the inbox ConPTY. Harness, `OuterPty` and viola-pty tests stay inbox.
- H2 with/without in one windows-2025 run (ci#36563868040): sideloaded 0 of 200, inbox 14 of 200, beside the inbox 13 of 200; no rate.
- The sideloaded preamble `ESC[1t ESC[c ESC[?1004h ESC[?9001h`; its DA1 query holds the child's start until answered (3.54 s vs 0.54 s on the dev host, as measured at the chunk's `evidence/da1-stall.md`); viola stays silent; the headless stall is an open finding owned by the route entry that first runs viola headless.
- Stack: was "Hosts the unmodified `claude` in ConPTY (Windows)"; now the sideloaded ConPTY on Windows x64 (Microsoft.Windows.Console.ConPTY 1.24.260710001, MIT, vendored and embedded), the inbox ConPTY the fallback; windows-sys also covers `SetDefaultDllDirectories` and `LoadLibraryExW`; sha2 also re-hashes the companions.
- Tree: `src/conpty.rs`, `main.rs`'s restriction, viola-pty `sideload`, `scripts/conpty-vendor.sh`, `vendor/conpty/<version>/x64/`.
**Why:** the founder's acceptance was the H2 measurement with and without; the restriction also closes a planting hole that existed before this chunk (a bare-name `conpty.dll` load from the CWD or `PATH`). The vendored delivery is a boundary widening ratified by the founder live (see security-plan's `2026-09-29` Log entry).
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/
