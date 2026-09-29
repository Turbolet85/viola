
## 2026-09-29-sideloaded-conpty — the DLL search order and the companions' integrity controls
**Section:** Input Validation (new row: DLL search order + sideloaded ConPTY companions); Data Protection → Code-bearing artefacts; Security Anti-Patterns → Data Protection, Universal
**Change:**
- Input Validation row: every `viola` process restricts its DLL search to System32 as the second statement of `main`, so a bare-name load (portable-pty's included) never resolves from the CWD or `PATH`; `run` writes the companions write-if-absent, re-hashes each in full through a `FILE_SHARE_READ`-only handle and only then pre-loads `conpty.dll` by absolute path, the handles held until `spawn_child` returns; a failure degrades to the inbox ConPTY, recorded only as codes (`run.conpty_sideload`, `sideload_fallback`).
- Code-bearing artefacts: the list was the pinned exe, the plugin files, `settings.json` and `stamps.json`; it now also holds `bin/<version>-<hash>/conpty/{OpenConsole.exe,conpty.dll}` (write-if-absent, held-handle re-hash, a mismatch left as found).
- Anti-patterns: never load or launch a companion without that re-hash, never overwrite or delete a failing one; never let a Windows `viola` process reach a DLL load before the restriction, never pre-load by a relative path, and a sideload failure is never a refusal, exit change or terminal byte.
**Why:** the controls this chunk shipped, measured by its planted, tamper and two-sided restriction tests; the Threat Model Summary stays a verbatim copy (playbook "Verbatim upstream copy").
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/
