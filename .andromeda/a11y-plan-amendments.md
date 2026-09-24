# a11y-plan — amendments

## 2026-09-24-supply-chain-and-workflow-gates — Platform: ci.yml is the one push/PR workflow
**Section:** §9 Pipeline integration → Platform
**Change:** "one workflow `ci.yml`" now reads "one push/PR workflow `ci.yml`". The scheduled `nightly.yml` carries no a11y step, and the E2E/a11y leg stays in `ci.yml`.

**Why:** chunk 2026-09-24-supply-chain-and-workflow-gates added `nightly.yml`, the weekly `cargo deny check advisories` run. The orchestrator raised this site because the same workflow-count grep hit it; no detector proposed it. Sweep: see architecture-amendments.md, same entry heading. For this master, :1108 was amended; the other "all three OS legs" hits were left unchanged as unrelated.
