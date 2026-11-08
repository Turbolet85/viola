@amendment marker=2026-11-08T20-32-52-wrap
title: version census 0.2.0 — the frontend stub and Windows signing retired
section: §Threat Model Summary (Approach) · §API Security · §Dependency Security · §Secret Management
change:
  - Threat Model Summary: was «Arch has decided no enforcement mechanism for that boundary on any OS. That decision is outstanding and belongs to this specialist»; now the enforcement mechanism is this specialist's, set by Authentication & Authorization and enforced on Linux, the one target (census C-091).
  - API Security: the line «The frontend is deferred to 0.2.0; its technology is not chosen» is retired; the section opens on its MCP transport scope (C-109).
  - Dependency Security: was «dist `>=0.33.0` with GitHub Artifact Attestations and Azure Artifact Signing (x86_64)»; now dist `>=0.33.0` with GitHub Artifact Attestations (C-110).
  - Secret Management: was «Azure Artifact Signing credentials (dist) and the zipsign private key …, both held only as CI secrets»; now the zipsign private key alone, held only as a CI secret in the release workflow (C-116).
  - Threat Model Summary, CI/CD (a citation the sweep caught): v1.x's release workflow drops winget/Homebrew/Scoop; dist 0.33.0, cargo-auditable 0.7.6, signing and `self_update` 1.3.0 stand.
why: the 0.2.0 version census (`doc:viola-0.2.0/census`), its card confirmed by the founder in his own words, typed 2026-11-08T19:59:58Z: no web or graphical surface, and no Windows or macOS target, so Windows binary signing has no subject. Each row names what the tree never held (built: no).
ref: .andromeda/runs/2026-11-08T20-32-52-wrap/
