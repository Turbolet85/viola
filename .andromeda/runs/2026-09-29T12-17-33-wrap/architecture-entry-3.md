
## 2026-09-29-sideloaded-conpty — the embedded ConPTY companions on disk and in the repository
**Section:** Established Decisions → [Deployment / Distribution], [Snapshot writer]; Occupied Resources → Filesystem, Repository; Infrastructure Patterns → Deployment model
**Change:**
- [Deployment / Distribution]: on Windows x64 the binary embeds `OpenConsole.exe` + `conpty.dll` (`src/conpty.rs`, the four pins' one textual home) and writes them write-if-absent to `bin/<version>-<hash>/conpty/`; each start holds them `FILE_SHARE_READ`-only and re-hashes them in full; unlike the exe a mismatch never refuses the start, it is left as found and the child runs on the inbox ConPTY. The subdirectory keeps them off the child's PATH. First-start cost, as measured at the chunk on the dev host under a parallel suite: `pin_companions` median 1 193 ms; CI stayed green; the root tests seed their homes instead; no bound raised.
- [Snapshot writer]: the companions join `replace_private_shared`'s users; the held open retries a Win32 error 32 under `REPLACE_ATTEMPTS` (two concurrent first starts, 5-11 ms measured).
- Filesystem: `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` registered. Repository: `vendor/conpty/<version>/x64/` (MIT nupkg bytes, `.gitattributes` binary, `conpty-vendor.sh` the one writer and verifier) and the test-side `target/conpty-seed/<key>/`.
- Deployment model: `bin/<version>-<hash>/` also holds `conpty/` on Windows x64, pre-loaded before the spawn.
**Why:** new resources this chunk landed, registered where their category lives; the fail-open treatment keeps the human's start unblocked.
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/
