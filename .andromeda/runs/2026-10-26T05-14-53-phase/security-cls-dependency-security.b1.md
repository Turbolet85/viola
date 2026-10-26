# Classified — security · dependency-security.b1

## Answers
dependency-security#1.1 · no · the vendored ConPTY binaries sit outside every cargo graph and their audit is unmoved by a tokio bump (the task's Not needed names it)
dependency-security#2.1 · yes · the tokio ban per sync crate (deny-sync.toml), the channel tree form (channel-tokio-free.sh) and deny-probes are re-read, and viola-channel's Tokio client pin =1.53.1 moves with the workspace pin
dependency-security#3.1 · yes · the scope's fuzz/Cargo.lock advisories-and-sources step must read green on the push, and this entry states how that step runs from the repo root
dependency-security#4.1 · no · the four ConPTY pins live in src/conpty.rs, which the chunk does not reach, and the task's Not needed names the ConPTY audit
dependency-security#5.1 · no · windows-mutants.yml is no reached file, and its hardening is one the task's Not needed names (no workflow is edited)
dependency-security#6.1 · no · the G2 seam exemption is not reached: no panic line in src/cmd/hook/seam.rs changes, and the task's Not needed names the G2 exemption
dependency-security#7.1 · no · the action SHA pins are workflow hardening the task's Not needed names (SHA-pinned actions, no workflow edited); ci.yml is re-read only at its supply-chain job
dependency-security#8.1 · yes · the open RUSTSEC-2026-0386 line is this chunk's own tokio bump to >=1.53.3, with RUSTSEC-2017-0008 kept the only advisory ignore
