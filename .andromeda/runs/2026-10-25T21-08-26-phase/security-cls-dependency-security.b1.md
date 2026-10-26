# Classified — security · dependency-security.b1

## Answers
dependency-security#1.1 · no · the vendored ConPTY binaries and their conpty-vendor gate sit outside the listener, its DACL and the chunk's test edits, which name no ConPTY file or check
dependency-security#2.1 · yes · any windows-sys feature or dev-dependency added for the read-back must leave the C-building crate ban in deny.toml and the tokio bans in deny-sync.toml and channel-tokio-free.sh unwidened, and no deny.toml ignore is added to reach green
dependency-security#3.1 · no · the chunk edits no fuzz/ file and adds no fuzz member or link, so the fuzz lockfile exemption is not reached
dependency-security#4.1 · no · the four ConPTY pins live in src/conpty.rs, which the chunk does not reach
dependency-security#5.1 · no · windows-mutants.yml is dispatch-only and never a gate, and the chunk edits and dispatches no such workflow
dependency-security#6.1 · no · the G2 exemption is the hook seam alone, the chunk adds no exemption and reaches no seam file, and zero role-file panics is a re-check every change owes
dependency-security#7.1 · no · the action SHA pins govern the existing ci.yml steps, and the chunk edits no workflow and adds no action use
