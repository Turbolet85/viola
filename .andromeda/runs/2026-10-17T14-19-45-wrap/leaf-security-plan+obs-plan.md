group: security-plan+obs-plan
read: .claude/rules/ci.md · 35 · «- 2026-10-13: A CI job that reads `cancelled` with no steps »

- leaf: .claude/rules/ci.md:11   class: omission
  leaf-text: «- `.andromeda/security-plan.md` §Dependency Security, the **CI integration** paragraph — the pins, the permissions, the audits and what no workflow may do»
  source: .andromeda/security-plan.md:145 «the per-OS `rmcp features` lint step (`scripts/rmcp-features.sh`: rmcp's features in `viola`'s release graph exactly `server transport-io`, rmcp present), the ubuntu supply-chain job (`cargo deny check`»
  replace-with: «- `.andromeda/security-plan.md` §Dependency Security, the **CI integration** paragraph — the pins, the permissions, the audits and what no workflow may do — and the gate steps it does not list, read in §Threat Model Summary's **CI/CD** Jobs line and §Dependency Security's ban list (the per-OS `rmcp features` lint step, `scripts/rmcp-features.sh`; `scripts/channel-tokio-free.sh` in the supply-chain step `Tokio ban per sync crate (sole root)`)»
