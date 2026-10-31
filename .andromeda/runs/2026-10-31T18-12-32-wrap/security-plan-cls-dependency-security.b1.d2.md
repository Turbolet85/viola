# Classified — security-plan · dependency-security.b1

## Answers
dependency-security#1.1 · no · the ConPTY blobs and their gate sit outside the reached files, and Untouched holds the vendored ConPTY pins
dependency-security#2.1 · no · the serde_json ban lives in deny.toml, which the session did not reach, and Cargo.toml gains only a [[test]] entry with no dependency or feature moved
dependency-security#3.1 · no · the arch bans, deny-sync.toml, scripts/sync-crates.txt and the channel's tokio feature are unreached, and no crate or tokio dependency was added or moved
dependency-security#4.1 · no · fuzz/ and its lockfile are unreached, and the session added a type to viola-core without any dependency change, so the path dep's graph stands
dependency-security#5.1 · no · the four ConPTY pins live in src/conpty.rs, which is not in Reached files and is held by Untouched
dependency-security#6.1 · no · .github/ is not reached, so windows-mutants.yml's matrix, permissions and pins stand, and the session adds no mutants package or file split
dependency-security#7.1 · no · the G2 gate, scripts/g2-zero-panics.sh and src/cmd/hook/seam.rs are unreached, and the session makes no change to the seam exemption
dependency-security#8.1 · no · ci.yml's pinned action SHAs sit under .github/, which is not reached, and CI ran green on 8a4c1a2c with no workflow change
