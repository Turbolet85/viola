
## 2026-09-24-supply-chain-and-workflow-gates — Lint row from sync-crates.txt, Supply-chain stage, wrappers sentence retired
**Section:** §2 trigger map (Supply chain V9 row) · §9 Pipeline structure (Lint row + new Supply-chain row, paragraph after the table) · §9 Build failure conditions · §12 Test crate deviation
**Change:**
- The Lint row's `cargo check` reads one `-p` per crate in `scripts/sync-crates.txt`; an empty list fails.
- A new Supply-chain row covers the ubuntu job `supply-chain`: cargo deny, the sole-root `deny-sync.toml` tokio ban, `scripts/deny-probes.sh`, zizmor, and the JSON artifact `supply-chain`. It also covers the weekly `nightly.yml` advisories job.
- The §2 V9 location now names both stages plus `nightly.yml`.
- The least-privilege sentence now covers both workflows. The cargo-deny 0.20 CLI form (global `--config`, `check -c` rejected) is recorded as measured on 0.20.2.
- Failure conditions: the deny and zizmor findings move under a new Supply-chain bullet, which also adds the sole-root and probe failures and the nightly run.
- §12: `viola-e2e` is no longer "added to the cargo-deny tokio wrappers list". It is never a sole root of the tokio ban.

**Why:** chunk 2026-09-24-supply-chain-and-workflow-gates. The report's "Spec claims disproved" #2 falsified the wrappers mechanism. Its Harness/gate surface gives the new jobs. The fan-out's 5 D-tests-framework proposals were all applied as re-derived. Sweep: see architecture-amendments.md, same entry heading, where one sweep served every master. For this master, 5 sites were amended (:486, :1399, :1411, :1434, :1617). Hits at :98, :164, :363 and :396 were left unchanged, because they are still true or unrelated.
