# Classified — security · dependency-security.b1

## Answers
dependency-security#1.1 · no · ConPTY vendoring is named Not needed; no reached file or cause touches src/conpty.rs or the vendored blobs
dependency-security#2.1 · no · deny.toml's unbounded_depth ban is untouched; the chunk adds no serde_json feature and no external-byte parse that the ban rules over
dependency-security#3.1 · yes · edits crates/viola-channel/src/client.rs, so its new vanished-server read must add no tokio to the viola-channel and sync-crate normal graph (channel-tokio-free.sh)
dependency-security#4.1 · no · fuzz/ is not reached; the chunk adds no dependency to the fuzz graph and links no fuzz crate into a shipped binary
dependency-security#5.1 · no · src/conpty.rs and its four ConPTY pins are not reached
dependency-security#6.1 · no · windows-mutants.yml is not reached and the scope changes no workflow
dependency-security#7.1 · no · standing S9 · no new exemption to the G2 zero-panics gate without its own Decisions Log entry
dependency-security#8.1 · no · the action SHA pins live in CI workflows the scope changes none of; no workflow is reached
