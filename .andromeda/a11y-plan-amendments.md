# a11y-plan — amendments

## 2026-09-24-supply-chain-and-workflow-gates — Platform: ci.yml is the one push/PR workflow
**Section:** §9 Pipeline integration → Platform
**Change:** "one workflow `ci.yml`" now reads "one push/PR workflow `ci.yml`". The scheduled `nightly.yml` carries no a11y step, and the E2E/a11y leg stays in `ci.yml`.

**Why:** chunk 2026-09-24-supply-chain-and-workflow-gates added `nightly.yml`, the weekly `cargo deny check advisories` run. The orchestrator raised this site because the same workflow-count grep hit it; no detector proposed it. Sweep: see architecture-amendments.md, same entry heading. For this master, :1108 was amended; the other "all three OS legs" hits were left unchanged as unrelated.

## 2026-09-24-diagnostics-plane — stale a11y-violation resolved-question bullet
**Section:** §12 A11y Decisions Log → Resolved questions
**Change:** The `a11y-violation` bullet now states what Z7 / D-A11Y-09 decided: it is not a product `event` value. It is a harness-only row validated by the tests-owned `e2e-web/schemas/a11y-row.v1.json`, and the shipped `ObsEvent` and `diag-line.v1.json` enum (19 values) carry none.
**Why:** chunk 2026-09-24-diagnostics-plane, report Spec claims disproved #3 (contract test green). The D-a11y-obs-schema proposal was applied as routine.

Sweep: `accepted as a tests \+ obs enum` over all seven masters gives 1 remaining hit, a11y :1389. It is a Decisions-Log history entry, left unchanged. The §3 wording it names ("new closed-enum value") has 0 hits outside that entry.
