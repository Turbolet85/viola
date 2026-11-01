# Classified — security · dependency-security.b1

## Answers
dependency-security#1.1 · no · vendored ConPTY binaries sit under the task's Not needed (ConPTY sideload and its vendoring); no Windows behaviour moves
dependency-security#2.1 · no · serde_json unbounded_depth ban in deny.toml: the chunk adds no parse of external bytes and does not edit deny.toml, so a re-check every change owes
dependency-security#3.1 · no · tokio ban in the sync crates' graphs and the channel tree form: the chunk adds no tokio dependency and reaches no sync crate's graph, so a re-check every change owes
dependency-security#4.1 · no · fuzz/ separate workspace and its lockfile exemption: under the task's Not needed (fuzz/ workspace), and the chunk adds no parser surface
dependency-security#5.1 · no · ConPTY pins in src/conpty.rs: under the task's Not needed (ConPTY vendoring); no Windows vendored file moves
dependency-security#6.1 · no · windows-mutants.yml workflow: under the task's Not needed (nightly and windows-mutants.yml workflows); the chunk adds no CI workflow and no release
dependency-security#7.1 · no · standing S4 · each new test's role files pass the G2 zero-panics gate, which takes no further exemption without its own Decisions Log entry
dependency-security#8.1 · no · standing S8 · a CI step a change adds keeps SHA-pinned actions, permissions {} with job-level contents: read, and no event value in run:
