# Classified — security · dependency-security.b1

## Answers
dependency-security#1.1 · no · the vendored ConPTY audit is in the task's Not needed list, and the chunk reaches no ConPTY file
dependency-security#2.1 · yes · the sync-side bounds in viola-channel and the other sync crates must stay on std Read::take and add no tokio (deny-sync.toml, channel-tokio-free.sh), the Tokio client's take staying behind its tokio feature
dependency-security#3.1 · no · standing S4 · a new fuzz or property target stays in the separate fuzz/ workspace with its own lockfile audit, a convention the task lists
dependency-security#4.1 · no · the ConPTY pins in src/conpty.rs are not reached, and the vendored ConPTY audit is in the task's Not needed list
dependency-security#5.1 · no · standing S6 · a new CI step pins actions by full SHA and keeps permissions {} / contents: read, a convention the task lists
dependency-security#6.1 · no · the G2 exemption covers only the hook seam's forced panic (src/cmd/hook/seam.rs, not reached), and the chunk's vt100 panic witness is unit-level, so it writes no role-file panic line
dependency-security#7.1 · no · standing S6 · actions pinned by full commit SHA, a convention the task lists
