# Classified — security-plan · dependency-security.b1

## Answers
dependency-security#1.1 · no · the vendored ConPTY binaries and their gate are not reached; the session touched no pty or ConPTY file
dependency-security#2.1 · no · deny.toml is not reached and no dependency was added or bumped, so the serde_json unbounded_depth ban holds
dependency-security#3.1 · no · deny-sync.toml and the channel tree gate are not reached; no dependency was added or bumped, so the tokio bans hold
dependency-security#4.1 · no · the fuzz/ lockfile and its root exclusion are untouched; no dependency was added or bumped
dependency-security#5.1 · no · src/conpty.rs and the ConPTY pins are not reached
dependency-security#6.1 · no · no workflow file or mutants matrix item is reached and no CI step changed, so the windows-mutants rows hold
dependency-security#7.1 · no · the exemption's file src/cmd/hook/seam.rs is not reached; edits to hook.rs do not move its lines, so the G2 exemption holds
dependency-security#8.1 · no · no workflow or action ref changed; ci.yml is not reached and no CI step changed
