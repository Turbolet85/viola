
## 2026-10-13-interrupted-verify-cleanup — verify's dirs removed on a Ctrl-C / TERM stop, the `--settings` move done, the matrix count
**Section:** §Data Protection (Interactive probe dirs · Probe captures · Code-bearing artefacts) · §Threat Model Summary, Child process spawning (Run B's ending) · §Dependency Security, CI integration (`windows-mutants.yml`)
**Change:**
- The probe dir and the interactive probe dirs were "removed on every exit path". Now they are removed when `verify` passes, fails, or is stopped by Ctrl-C or TERM: Unix `SIGINT` / `SIGTERM`, Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT`, through `cmd::verify::stop`, installed by `viola verify` alone; its handler only sets one flag every wait reads. A stopped run kills and reaps its child with no key, and its guards remove only what that run created, never by a pattern or a listing. No owner record and no remover of an earlier run's or another process's dirs. SIGKILL, a crash, a failed handler install (ignored, fail-open) and a Windows close, logoff or shutdown leave the dirs by hand.
- A stopped Run B is ended by a kill with no key, pastes nothing more, and starts no later run.
- Was: `typed.rs` held the `--settings` literal, owed to "Interrupted verify cleanup". Now Runs C and D read `statusline::SETTINGS_FLAG`.
- `windows-mutants.yml` was a nine-item matrix with the root package four; now ten items with the root package five (`viola-cmd-verify-stop` over `src/cmd/verify/stop.rs`).
**Why:** The scope is the founder's narrow entry (relayed by the overseer, 2026-10-10); the event set is the operator's P4 answer. The matrix line is forced by the repository's contract test that every `cfg(windows)` product file is scored on Windows, and it was taken on the operator's word at implement; nothing was dispatched. The stop adds no IPC, file or env crossing: it carries no data, and its decision is closed (two events, two signals).
**Kept:** the same-pid pre-clean in `ProbeDir::create` / `TrustedDir::create` stands, predating the narrow ruling.
**Ref:** .andromeda/runs/2026-10-13T20-00-28-wrap/
