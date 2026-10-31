# Classified — security-plan · dependency-security.b1

## Answers
dependency-security#1.1 · no · the ConPTY blobs, pins and conpty-vendor.sh are untouched and the changed Cargo.toml adds no manifest edge
dependency-security#2.1 · no · deny.toml, its serde_json unbounded_depth ban and its probe are untouched and no dependency was bumped
dependency-security#3.1 · no · deny-sync.toml and the per-sync-crate tokio ban are untouched, and the reached viola-channel files add no tokio dependency or feature
dependency-security#4.1 · no · the fuzz/ exemption is untouched and the changed root Cargo.toml keeps exclude = ["fuzz"] with no manifest edge moved
dependency-security#5.1 · no · the ConPTY pins in src/conpty.rs are untouched and that file is not among the reached files
dependency-security#6.1 · no · windows-mutants.yml is untouched and no src file was added or renamed, so the ten-item matrix and its five root-package items still hold
dependency-security#7.1 · no · the G2 gate and its one seam.rs exemption are untouched with no seam, failpoint or exemption added, and CI read green at fa7be1f3
dependency-security#8.1 · no · the .github workflows and their SHA pins are untouched and no workflow file is among the reached files
