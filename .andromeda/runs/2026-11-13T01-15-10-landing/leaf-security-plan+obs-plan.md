@leaf_group group=security-plan+obs-plan
@leaf_read leaf=.claude/rules/ci.md lines=35
last: - 2026-10-13: A CI job that reads `cancelled` with no steps 
@leaf_edit n=1 leaf=.claude/rules/ci.md line=11 class=stale
leaf-text: the **CI integration** paragraph, with its detail `.andromeda/security-plan-detail/dependency-security.md` (read whole), which holds the action SHAs — the pins, the permissions, the audits and what no workflow may do
source: dec-nzthkcjz «decision/actions-pinned-by-sha · GitHub Actions pinned by full commit SHA — Every GitHub Action is referenced by a full commit SHA with a version comment, never by @stable, @v2 or any mutable ref, and»
replace-with: read whole by `rec.py render --section "Dependency Security"` — every action pinned by full commit SHA with a version comment, the permissions, the audits and what no workflow may do
@leaf_edit n=2 leaf=.claude/rules/ci.md line=12 class=stale
leaf-text: **Gate commands** and **Step order and conditions**, with its detail `.andromeda/obs-plan-detail/ci-integration.md` (read whole), which holds the gate command text and the uploads' conditions — the gate commands (each its own `run:` step with `shell: bash`, copied verbatim) and the gate steps and their conditions, copied into `ci.yml`
source: dec-7nrk2dtm «decision/ci-artifacts-gated-on-secret-scan · CI artifacts uploaded only after a passing secret scan — Diagnostics, harness logs, JUnit XML and perf samples are uploaded on failing runs too, but only when»
replace-with: read whole by `rec.py render --section "CI Integration"` — the gates and their conditions (each fails closed; G2, G4 and the secret scan run even after a failed step; every upload only when the job's secret scan succeeded), kept true in `ci.yml`
