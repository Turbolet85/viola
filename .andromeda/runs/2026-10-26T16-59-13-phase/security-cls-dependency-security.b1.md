# Classified — security · dependency-security.b1

## Answers
dependency-security#1.1 · yes · at run time the trust in conpty.dll and OpenConsole.exe is the full SHA-256 re-hash against the compiled pin, never a signer check, which pin_companions must keep
dependency-security#2.1 · yes · the scope's own viola_state need (stays a sync crate; no tokio and no C-building crate in its graph, any new windows-sys feature included) is what the per-sync-crate tokio ban and the C-build ban rule over
dependency-security#3.1 · no · the fuzz/ workspace exemption, which the task's Not-needed list names; no fuzz/ file or the root [workspace] is reached
dependency-security#4.1 · yes · src/conpty.rs (reached whole) is the one textual home of the conpty.dll and OpenConsole.exe SHA-256 pins that the chunk's companion re-hash compares against
dependency-security#5.1 · no · standing S5 · each new or changed CI step stays SHA-pinned, permissions: {} with job contents: read, event values only through env:, uploads behind the artifact canary scan
dependency-security#6.1 · no · standing S7 · each boundary widening, new environment read, new G2 exemption or new dated exception needs its own Decisions Log entry and the founder's own word
dependency-security#7.1 · no · standing S5 · each new or changed CI step stays SHA-pinned, permissions: {} with job contents: read, event values only through env:, uploads behind the artifact canary scan
