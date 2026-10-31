# Classified — security · dependency-security.b1

## Answers
dependency-security#1.1 · no · ConPTY vendored binaries and their gate; the task's Not needed names ConPTY vendoring and no reached file touches it
dependency-security#2.1 · no · deny.toml's serde_json unbounded_depth feature ban; deny.toml and Cargo features are not reached and the chunk adds no dependency
dependency-security#3.1 · yes · viola-channel's client.rs is edited for the vanished-server code, so its new code must add no tokio to the channel's normal graph (scripts/channel-tokio-free.sh) and the sync-crate tokio bans stay live
dependency-security#4.1 · no · the fuzz/ separate workspace and its lockfile; no reached file is under fuzz/ and no Cargo workspace or lockfile changes
dependency-security#5.1 · no · ConPTY pins in src/conpty.rs; ConPTY vendoring is outside the chunk
dependency-security#6.1 · no · windows-mutants.yml workflow; the scope changes no workflow and no CI job
dependency-security#7.1 · no · standing S9 · no new exemption to the G2 zero-panics gate without its own Decisions Log entry
dependency-security#8.1 · no · action SHA pins in workflows; the scope changes no workflow
