By construction, three parts, all required:

(a) The release build contains no test-only binary.
- `bash scripts/release-check.sh --probe` prints `release-check probes: 3/3 refused, control clean`. It refuses records naming `viola-harness` or `viola-fake-agent`, and refuses an empty stream.
- `bash scripts/release-check.sh` prints `release-check: viola only`. It reads the `compiler-artifact` records of `cargo build --release --locked --bin viola`.
- The three `release (windows-2025|macos-latest|ubuntu-latest)` check-runs read `success` on the chunk's pushed sha.

(b) The tokio ban still holds for every sync crate.
- The sole-root `cargo deny --config deny-sync.toml --manifest-path crates/<c>/Cargo.toml check bans` loop over `scripts/sync-crates.txt` exits 0.
- `bash scripts/deny-probes.sh` prints `deny-probes: 13/13 banned, control clean`.

(c) The architecture lists every test, observability and accessibility artifact.
- `viola-0.1.0/chunks/2026-09-24-workspace-tree-and-code-graph-planes/artifact-inventory.tsv` enumerates, with owner anchor and architecture home (tree or repository), every artifact path that these plans name: test-plan §2/§3/§9, obs-plan §3/§8, a11y-plan §3, and design-system / layout-templates §Surface: web-spa.
- The architecture's §Project directory structure, §Occupied Resources → Repository and Build-system dependency policy list every row.
- The inventory gate re-run after the wrap's architecture amendment prints `0` as its last line: no row token is absent from `.andromeda/architecture.md`.
