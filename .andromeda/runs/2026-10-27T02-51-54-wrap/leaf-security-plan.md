group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/security.md:14   class: omission
  leaf-text: «which sets the temp file's mode before any byte is written.»
  source: .andromeda/security-plan-detail/authentication-authorization.md:33 «The pinned `viola` is never replaced: its absent copy goes through `viola_state::fs::place_private_new`, which sets the temp file's mode (0700) the same way before any byte is written, `sync_all`s it»
  replace-with: «which sets the temp file's mode before any byte is written; the pinned `viola` is never replaced: its absent copy goes through `viola_state::fs::place_private_new` (mode 0700 set before any byte, `sync_all`, the write handle closed, then `persist_noclobber`, never over a present name), and a copy another start placed first is re-hashed before reuse, different bytes left as found.»

- leaf: .claude/docs/security-summary.md:17   class: omission
  leaf-text: «pinned exe re-hashed (truncated SHA-256) before reuse;»
  source: .andromeda/security-plan-detail/threat-model-summary.md:136 «The pinned binary is placed once and never replaced. An exec of a copy that another start still holds open for write fails with `ETXTBSY` (raw 26 on Linux), so the placement closes its write handle before the name appears»
  replace-with: «pinned exe placed once and never replaced (`place_private_new`: the write handle closed before a no-clobber `persist_noclobber`, so concurrent first starts never exec a copy held open for write) and re-hashed (truncated SHA-256) before reuse, a copy another start placed first included;»
