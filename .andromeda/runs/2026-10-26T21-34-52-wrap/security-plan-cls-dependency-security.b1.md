# Classified — security-plan · dependency-security.b1

## Answers
dependency-security#1.1 · no · the companion path in the reached pin.rs gains only the check before the held open (F9), so its SHA-256 re-hash still carries the signer at run time
dependency-security#2.1 · no · the Dependency Security set is unchanged (no Cargo.toml, Cargo.lock, script or workflow changed), so the tokio and C-build bans read the same graph
dependency-security#3.1 · no · the Dependency Security set is unchanged, fuzz/ included, so the separate-workspace exception stands as written
dependency-security#4.1 · no · the four ConPTY pins stay in the unedited src/conpty.rs, and the reached pin.rs changes (F9, F19) add no pin text
dependency-security#5.1 · no · no workflow changed, so windows-mutants.yml keeps its dispatch-only trigger and its ten-item matrix
dependency-security#6.1 · no · the seam.rs exemption lives in the unchanged G2 script and seam.rs is unreached, and F7's diagnostics check moves no panic_location
dependency-security#7.1 · no · no workflow changed, so ci.yml still uses the same four commit-SHA-pinned actions
