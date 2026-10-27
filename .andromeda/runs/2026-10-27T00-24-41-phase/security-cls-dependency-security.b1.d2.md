# Classified — security · dependency-security.b1

## Answers
dependency-security#1.1 · yes · the run-time SHA-256 pin that the Windows companion re-hash under bin/<key>/conpty/ relies on, which pin_companions re-asserts; not the vendor audit
dependency-security#2.1 · no · standing S6 · each new dependency is workspace-pinned, passes cargo deny check, builds no C and stays out of a sync crate's graph as tokio
dependency-security#3.1 · no · the chunk reaches no fuzz/ file and links no fuzz crate into viola, and the Not-needed list excludes the exemption
dependency-security#4.1 · yes · the four pins in their one textual home src/conpty.rs, which the companion re-hash reads at run time, so no second copy may be made; not the version move
dependency-security#5.1 · no · standing S7 · each new or changed CI job or step pins actions by full commit SHA, keeps permissions {} / contents: read and passes no event value through ${{ }} in run:
dependency-security#6.1 · no · the chunk adds no G2 exemption and touches no src/cmd/hook/seam.rs; its new code's zero-panic count is a re-check every change owes
dependency-security#7.1 · no · standing S7 · each new or changed CI job or step pins actions by full commit SHA, keeps permissions {} / contents: read and passes no event value through ${{ }} in run:
