# Classified — security · dependency-security.b1

## Answers
dependency-security#1.1 · no · the chunk changes no Windows behaviour; the vendored ConPTY binaries and their gate are Windows-only and listed as not needed
dependency-security#2.1 · no · the chunk adds no dependency and no new serde_json parse of external bytes, so the unbounded_depth ban does not reach its code
dependency-security#3.1 · no · standing S7 · a dependency a change adds keeps the workspace pin, the licence list and the bans, `tokio` in the sync crates included
dependency-security#4.1 · no · the chunk touches no fuzz/ workspace or lockfile; the task's not-needed list names the fuzz exemption
dependency-security#5.1 · no · the ConPTY pins in src/conpty.rs are Windows-only; the chunk moves no Windows vendoring
dependency-security#6.1 · no · standing S8 · a CI step keeps SHA-pinned actions, `permissions: {}` with job-level `contents: read`, and no event value in `run:`
dependency-security#7.1 · no · standing S4 · each new test's role files pass the G2 zero-panics gate, which takes no further exemption without its own Decisions Log entry
dependency-security#8.1 · no · standing S8 · a CI step keeps SHA-pinned actions, `permissions: {}` with job-level `contents: read`, and no event value in `run:`
