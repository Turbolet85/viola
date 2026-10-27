# Classified — security-plan · dependency-security.b1

## Answers
dependency-security#1.1 · no · the ConPTY vendor audit is untouched (the report's Untouched list names it), and its files (the vendored binaries, scripts/conpty-vendor.sh) are outside the reached set
dependency-security#2.1 · yes · stale · the deny-probes verdict moves from 13/13 to 14/14 with the serde_json unbounded_depth ban (F5) and its probe row, and the every-ban claim now covers it
dependency-security#3.1 · no · the session's deny.toml change is one [[bans.features]] table, and the fuzz audit runs advisories and sources, never bans, so the C-build ban and the walk-up resolution still hold on the changed file
dependency-security#4.1 · no · the ConPTY pins live in src/conpty.rs and scripts/conpty-vendor.sh, neither reached, and the report names the ConPTY vendor audit as untouched
dependency-security#5.1 · no · windows-mutants.yml is not a reached file, so its ten-item matrix, its five root-package items and its permissions stand as written
dependency-security#6.1 · no · the G2 gate's one seam exemption stands: scripts/g2-zero-panics.sh is not reached, the session adds no exemption, and CI was green on all three OSes with the change
dependency-security#7.1 · no · the workflow files that hold these SHA pins are not reached, and the report's Untouched list names pinning as unchanged
