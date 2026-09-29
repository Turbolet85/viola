
## 2026-09-29-sideloaded-conpty — vendored Microsoft ConPTY binaries and the third interim gap (founder live)
**Section:** Security Decisions Log (`2026-09-29`); Authentication & Authorization → `~/.viola/` access control
**Change:**
- New Log entry: `conpty.dll` + `OpenConsole.exe` from `Microsoft.Windows.Console.ConPTY` 1.24.260710001 (MIT) committed under `vendor/conpty/…`, embedded, written to `bin/<version>-<hash>/conpty/` and sideloaded; their own audit (`conpty-vendor.sh`), never a `cargo deny` exemption; no network call at build or run time; the System32 DLL-search restriction as the control for the planting vector.
- A third dated interim gap, beside the two of `2026-09-28`: until "Home and code-bearing file integrity" (Epoch 6) adds the `conpty/` folder and both files to the Windows strict-modes set, `run` loads them under the `FILE_SHARE_READ`-only held handle and the full SHA-256 re-hash alone, without the owner/DACL check.
**Why:** both are boundary widenings (a third-party prebuilt binary outside every lockfile audit hosts the child; code-bearing files loaded before the owner/DACL check), ratified by the founder live on 2026-09-29 at 10:41:12, relayed by the Viola overseer, after both forks were shown at the chunk's phase P4. The gap closes with the Epoch 6 entry (route CARRY).
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/
