group: security-plan+obs-plan
read: .claude/rules/ci.md · 35 · «- 2026-10-13: A CI job that reads `cancelled` with no steps »

- leaf: .claude/rules/ci.md:12   class: omission
  leaf-text: «**Step order and conditions** — the gate steps and their conditions, copied into `ci.yml`»
  source: .andromeda/obs-plan.md:937 «**Gate commands.** Each gate is its own `run:` step with `shell: bash` on all three OSes, and is copied verbatim into `ci.yml`:»
  replace-with: «**Gate commands** and **Step order and conditions** — the gate commands (each its own `run:` step with `shell: bash`, copied verbatim) and the gate steps and their conditions, copied into `ci.yml`»
