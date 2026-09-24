
## 2026-09-24-supply-chain-and-workflow-gates — Platform: nightly.yml beside the single push/PR ci.yml
**Section:** §9 Pipeline integration → Platform
**Change:** `ci.yml` stays the single push/PR workflow. The scheduled `nightly.yml` sits beside it and runs only the weekly `cargo deny check advisories`.

**Why:** chunk 2026-09-24-supply-chain-and-workflow-gates. It is a plan-carried expected amendment, raised by the orchestrator under Validate check 5; no detector proposed it. The P4 operator decision put the weekly run in `nightly.yml`. Sweep: see architecture-amendments.md, same entry heading. For this master, :1217 was amended. :406, :777, :1239, :1378 and :1443 were left unchanged, because they are still true.
