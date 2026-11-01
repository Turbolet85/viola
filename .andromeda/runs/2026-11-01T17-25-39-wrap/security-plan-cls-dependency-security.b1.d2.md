# Classified — security-plan · dependency-security.b1

## Answers
dependency-security#1.1 · no · the session adds no dependency or bump to the reached Cargo.toml, and the vendored ConPTY blobs and src/conpty.rs are not reached, so the claim that they sit outside every cargo graph still holds
dependency-security#2.1 · no · deny.toml is not reached and no dependency was added or bumped, so the serde_json unbounded_depth ban stands
dependency-security#3.1 · no · no dependency was added or bumped and TokioClient stays behind viola-channel's tokio feature, so the tree-form check still holds on the reached channel files
dependency-security#4.1 · no · fuzz/ and its lockfile are not reached and the session adds no dependency or workspace member, so the separate-workspace exemption stands
dependency-security#5.1 · no · src/conpty.rs and the pinned vendor files are not reached
dependency-security#6.1 · no · windows-mutants.yml is not reached and no workflow changed
dependency-security#7.1 · no · the exemption is for src/cmd/hook/seam.rs, a file the change does not reach; src/cmd/hook.rs is a different file
dependency-security#8.1 · no · no workflow file is reached and no CI step changed, so the action SHA pins hold
