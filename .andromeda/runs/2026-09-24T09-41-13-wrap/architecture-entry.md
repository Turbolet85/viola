
## 2026-09-24-supply-chain-and-workflow-gates — sole-root tokio ban, four-family deny policy, nightly.yml
**Section:** §Stack Code-quality row · §Established Decisions [Concurrency] · §Occupied Resources → Repository · §Infrastructure Patterns → Build system · directory tree · CI/CD approach
**Change:**
- Build system: the tokio ban moved to `deny-sync.toml`, run once per crate in `scripts/sync-crates.txt` as the sole root. It is no longer checked over a graph with the async crates excluded. `--exclude` false-fails under feature unification, as measured at research.md §Measured facts; the `viola-channel` own-root limit is recorded. `deny.toml` now carries four families: advisories, licences, sources, and bans (C, telemetry, feature).
- [Concurrency] enforcement text names the sole-root ban and the sync-crate list.
- Code-quality row: cargo-deny families corrected; zizmor 1.30.1 added (workflow lint).
- Occupied Resources: `target/deny-probes/` and `target/supply-chain/` registered.
- Tree: `deny-sync.toml`, `scripts/sync-crates.txt`, `scripts/deny-probes.sh` and `workflows/nightly.yml` added; the `deny.toml` comment is corrected.
- CI/CD: two workflows (`ci.yml` push/PR, `nightly.yml` weekly + dispatch). `ci.yml` jobs are now `test`, `mutants`, `lint`, `supply-chain`. Target job 3 reads `scripts/sync-crates.txt`. zizmor is installed by `cargo install --locked`.

**Why:** chunk 2026-09-24-supply-chain-and-workflow-gates shipped these. The report's "Spec claims disproved" #1 falsified the excluded-graph mechanism. The P4 operator decision placed the weekly run in `nightly.yml`. Of the fan-out's 9 D-arch proposals, 8 were applied as re-derived. The Occupied-Resources proposal was applied only in part: the two `target/` dirs were registered, and its config files were routed to the tree (playbook "Registry over-reach").

Sweep over all seven masters, patterns `wrappers list`, `excluded, not with`, `one workflow .ci`, `single workflow`, `separate workflow trigger`, `licences, C-crate bans, tokio`, `licences, C-dependency`, `-p viola-core -p viola-pty`, `check bans -c`, `tokio wrappers`, plus the claim reads `tokio.{0,40}(ban|wrappers)`, `(ban|deny).{0,60}tokio`, `all three targets`:
- Before apply, 17 amend-sites were found: arch :36, :44, :392, :418, :456, :463; security :134, :161, :308, :314, :327; test :486, :1399, :1411, :1434, :1617; obs :1217; a11y :1108. The remaining hits were left unchanged: arch :35; test :98, :164, :363, :396; obs :406, :777, :1239, :1378, :1443; a11y :175, :280, :499, :624, :793, :807, :813, :1063, :1153. They are still true or unrelated "all three OSes" wording.
- After apply: 0 hits in the masters and 0 in the preserve-verbatim homes, playbook and drift-base. The control fired on leaves commands.md:47, stack.md:32 and services/viola-mcp.md:17, which are re-derived in cascade step 3.
