# Classified — security-plan · dependency-security.b1

## Answers
dependency-security#1.1 · no · the ConPTY vendor files and conpty-vendor.sh are in no reached file, and the report keeps the ConPTY vendor text unmoved
dependency-security#2.1 · no · no ban file or viola-channel manifest is reached, and no crate was added, bumped or banned
dependency-security#3.1 · no · the fuzz/ workspace and its lockfile are in no reached file, and no dependency changed
dependency-security#4.1 · no · the four ConPTY pins live in src/conpty.rs, which is in no reached file, and the report keeps the pinning text unmoved
dependency-security#5.1 · no · windows-mutants.yml is in no reached file, and the report keeps the CI-workflow text unmoved, so the five root items stand
dependency-security#6.1 · no · the G2 gate script and src/cmd/hook/seam.rs are in no reached file, and no new fact adds a G2 exemption
dependency-security#7.1 · no · the pinned action set lives in ci.yml, which is in no reached file, and the report keeps the pinning text unmoved
