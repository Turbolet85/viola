# Classified — security · dependency-security.b1

## Answers
dependency-security#1.1 · yes · at run time the SHA-256 pin carries the companions' trust, which the full re-hash in pin_companions keeps (the Needs' companion rules); the build-time signer audit in conpty-vendor.sh is not reached
dependency-security#2.1 · no · standing S6 · each new dependency is workspace-pinned, passes cargo deny check and builds no C, and none enters a sync crate's graph as tokio
dependency-security#3.1 · no · the fuzz/ workspace exemption and its lockfile audit are not reached (the task's Not needed)
dependency-security#4.1 · yes · the conpty.dll and OpenConsole.exe SHA-256 pins in src/conpty.rs are the reference the companions' full re-hash in pin_companions checks against (the Needs' companion rules); the nupkg pin and the version move are not reached
dependency-security#5.1 · no · standing S7 · each new or changed CI job or step pins actions by SHA, keeps permissions {} / contents read, and passes no event value through ${{ }} in run:
dependency-security#6.1 · no · the G2 seam exemption is not reached (the chunk adds no panic, no exemption and no seam.rs edit), and the zero-panic gate is a re-check every change owes
dependency-security#7.1 · no · standing S7 · each new or changed CI job or step pins actions by SHA
