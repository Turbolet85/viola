# Classified — security · dependency-security.b1

## Answers
dependency-security#1.1 · no · the vendored ConPTY binaries and their pins are Not needed for this chunk (the sideload and vendoring are out of scope; no exit cause runs through them)
dependency-security#2.1 · no · the serde_json `unbounded_depth` ban lives in deny.toml, which the chunk does not reach; the chunk's row parse keeps serde_json's default recursion limit and adds no feature
dependency-security#3.1 · yes · the chunk edits viola-channel/src/client.rs, and the channel crate's normal graph must stay tokio-free (channel-tokio-free.sh), so the new vanished-server read adds no tokio dependency
dependency-security#4.1 · no · the fuzz/ workspace, its lockfile and its exclude line are not reached; viola-core's edit adds no dependency
dependency-security#5.1 · no · the ConPTY pins in src/conpty.rs are not reached; ConPTY vendoring is Not needed for this chunk
dependency-security#6.1 · no · the windows-mutants workflow is Not needed: the scope changes no workflow and runs no mutation tooling
dependency-security#7.1 · no · standing S9 · no new exemption to the G2 zero-panics gate without its own Decisions Log entry
dependency-security#8.1 · no · the GitHub Actions SHA pins are Not needed: the scope changes no workflow
