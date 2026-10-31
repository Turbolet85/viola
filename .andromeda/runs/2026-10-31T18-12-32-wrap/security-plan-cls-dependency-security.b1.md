# Classified — security-plan · dependency-security.b1

## Answers
dependency-security#1.1 · no · the ConPTY pins and the `scripts/conpty-vendor.sh` gate sit in `src/conpty.rs` and `scripts/`, neither reached, and the report lists the vendored ConPTY as untouched
dependency-security#2.1 · no · the serde_json `unbounded_depth` ban lives in `deny.toml`, which the session did not reach; Cargo.toml's only change is a `[[test]]` table
dependency-security#3.1 · no · the arch bans live in `deny.toml` and `deny-sync.toml`, neither reached; the Cargo.toml change adds a test feature, no dependency
dependency-security#4.1 · no · Cargo.toml changes only by a `[[test]]` table, so `exclude = ["fuzz"]` and the root-graph reading of `fuzz/` still hold on the changed file; `fuzz/` is untouched
dependency-security#5.1 · no · the four ConPTY pins live in `src/conpty.rs`, not reached, and the report lists the ConPTY package as untouched
dependency-security#6.1 · no · `windows-mutants.yml` sits under `.github/`, which the report says was not reached, and the report adds no mutants package item, so the ten-item matrix stands
dependency-security#7.1 · no · the G2 exemption names `src/cmd/hook/seam.rs`, which the session did not reach, and `scripts/g2-zero-panics.sh` is not reached
dependency-security#8.1 · no · the action SHA pins live in `ci.yml` under `.github/`, not reached, and the report says action pinning stands
