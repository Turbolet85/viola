# Classified — security-plan · dependency-security.b1

## Answers
dependency-security#1.1 · no · the vendored ConPTY files and the scripts/conpty-vendor.sh gate are untouched, as the report's Untouched list says
dependency-security#2.1 · no · the bans read green on the changed root Cargo.toml with no ignore, skip or allow added, and the tokio pin stays =1.53.1, so the claim holds
dependency-security#3.1 · no · the report keeps the fuzz lockfile exemption unchanged, and the root Cargo.toml change adds only a Windows dev-dependency table, leaving exclude = ["fuzz"] as it was
dependency-security#4.1 · no · the four ConPTY pins in src/conpty.rs stay unchanged, as the report's Untouched list says
dependency-security#5.1 · no · windows-mutants.yml is not reached, and the session changes neither its ten-item matrix nor its permissions or uploads
dependency-security#6.1 · no · the G2 exemption at src/cmd/hook/seam.rs and its probe are untouched, and the session changes no logged field
dependency-security#7.1 · no · no workflow file is reached, so the four full-SHA action pins stand
