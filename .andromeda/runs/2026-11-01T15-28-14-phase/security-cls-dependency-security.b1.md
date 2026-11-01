# Classified — security · dependency-security.b1

## Answers
dependency-security#1.1 · no · the vendored ConPTY pins and Windows vendoring are named Not needed; no Windows behaviour moves
dependency-security#2.1 · no · the chunk adds no dependency and changes no deny.toml ban; the serde_json feature ban is not reached
dependency-security#3.1 · no · standing S7 · a dependency a change adds keeps the bans, tokio in the sync crates included
dependency-security#4.1 · no · the fuzz/ workspace and its lockfile exemption are named Not needed; the chunk adds no parser surface
dependency-security#5.1 · no · the ConPTY version and hash pins are named Not needed; no vendored Windows package moves
dependency-security#6.1 · no · standing S8 · a CI step keeps permissions: {} with job-level contents: read, SHA-pinned actions and no event value in run:
dependency-security#7.1 · no · standing S4 · the G2 zero-panics gate takes no further exemption without its own Decisions Log entry
dependency-security#8.1 · no · standing S8 · a CI step keeps SHA-pinned actions
