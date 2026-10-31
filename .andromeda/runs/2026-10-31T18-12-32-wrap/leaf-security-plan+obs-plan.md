group: security-plan+obs-plan
read: .claude/rules/ci.md · 35 · «- 2026-10-13: A CI job that reads `cancelled` with no steps »
- leaf: .claude/rules/ci.md:11   class: omission
  leaf-text: «`.andromeda/security-plan.md` §Dependency Security, the **CI integration** paragraph — the pins, the permissions, the audits and what no workflow may do»
  source: .andromeda/security-plan-detail/dependency-security.md:54 «Actions are pinned by full commit SHA (the set `ci.yml` uses; SHAs resolved 2026-09-24 via `gh api repos/{repo}/commits/{tag}`):»
  replace-with: «`.andromeda/security-plan.md` §Dependency Security, the **CI integration** paragraph, with its detail `.andromeda/security-plan-detail/dependency-security.md` (read whole), which holds the action SHAs — the pins, the permissions, the audits and what no workflow may do»
- leaf: .claude/rules/ci.md:12   class: omission
  leaf-text: «`.andromeda/obs-plan.md` §9 CI Integration, **Gate commands** and **Step order and conditions** — the gate commands (each its own `run:` step with `shell: bash`, copied verbatim) and the gate steps and their conditions, copied into `ci.yml`»
  source: .andromeda/obs-plan.md:590 «Detail: .andromeda/obs-plan-detail/ci-integration.md — read whole.»
  replace-with: «`.andromeda/obs-plan.md` §9 CI Integration, **Gate commands** and **Step order and conditions**, with its detail `.andromeda/obs-plan-detail/ci-integration.md` (read whole), which holds the gate command text and the uploads' conditions — the gate commands (each its own `run:` step with `shell: bash`, copied verbatim) and the gate steps and their conditions, copied into `ci.yml`»
