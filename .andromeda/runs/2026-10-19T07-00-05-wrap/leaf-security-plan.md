group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/ci.md · 35 · «- 2026-10-13: A CI job that reads `cancelled` with no steps »
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/security.md:23   class: stale
  leaf-text: «and the hidden `viola mcp` (chunk 2026-10-17-mcp-server-for-drivers; its use written as built and NOT ratified either, a delegate's answer);»
  source: .andromeda/security-plan.md:204 «and the hidden `viola mcp` (chunk 2026-10-17-mcp-server-for-drivers), resolves the home through `resolved_home`»
  replace-with: «and the hidden `viola mcp` (chunk 2026-10-17-mcp-server-for-drivers);»

- leaf: .claude/rules/security.md:23   class: stale
  leaf-text: «The CLI verbs' `VIOLA_DIR` step and the canonicalisation widen what chooses a CLI verb's home and are NOT ratified: the founder's own word is owed (a delegate's answer, `inputs#I3` of that chunk).»
  source: .andromeda/security-plan.md:204 «`mcp`'s use of them included, widen what chooses a CLI verb's home: a widening the founder ratified (his own word, relayed verbatim by the operator, 2026-10-19).»
  replace-with: «The CLI verbs' `VIOLA_DIR` step and the canonicalisation, `mcp`'s use of them included, widen what chooses a CLI verb's home: a widening the founder ratified (his own word, relayed verbatim by the operator, 2026-10-19).»

- leaf: .claude/docs/security-summary.md:64   class: stale
  leaf-text: «and the hidden `viola mcp` (chunk 2026-10-17-mcp-server-for-drivers; its use written as built and NOT ratified either, a delegate's answer) (`hook` keeps its own shape check)»
  source: .andromeda/security-plan.md:204 «and the hidden `viola mcp` (chunk 2026-10-17-mcp-server-for-drivers), resolves the home through `resolved_home`»
  replace-with: «and the hidden `viola mcp` (chunk 2026-10-17-mcp-server-for-drivers) (`hook` keeps its own shape check)»

- leaf: .claude/docs/security-summary.md:64   class: stale
  leaf-text: «the `VIOLA_DIR` step and the canonicalisation widen what chooses a CLI verb's home and are NOT ratified — the founder's own word is owed (`inputs#I3`).»
  source: .andromeda/security-plan.md:204 «`mcp`'s use of them included, widen what chooses a CLI verb's home: a widening the founder ratified (his own word, relayed verbatim by the operator, 2026-10-19).»
  replace-with: «the `VIOLA_DIR` step and the canonicalisation, `mcp`'s use of them included, widen what chooses a CLI verb's home: a widening the founder ratified (his own word, relayed verbatim by the operator, 2026-10-19).»

- leaf: .claude/docs/security-summary.md:73   class: stale
  leaf-text: «at `--scope local` from a rig dir under `target/` that is its own git repo (the CLI takes the local scope's project from the git work tree root); the CLI's writes under `~/.claude` are its own — after the removal its orphaned plugin cache `~/.claude/plugins/cache/viola/viola/<version>/` stays, `installed_plugins.json` and `known_marketplaces.json` back as before (as measured on claude 2.1.287 at chunk 2026-10-17-optional-plugin-install); viola itself writes nothing under `~/.claude`.»
  source: .andromeda/security-plan.md:262 «at `--scope local`, a fresh count of four per rig, from a `proj/` that is its own git repo outside this work tree (on the e2e-home tmpfs, no ancestor `CLAUDE.md`)»
  replace-with: «at `--scope local`, a fresh count of four per rig, from a `proj/` that is its own git repo outside this work tree (on the e2e-home tmpfs, no ancestor `CLAUDE.md`): the CLI takes the local scope's project from the git work tree root, and a `proj/` inside the work tree meets the external-imports dialog after the trust dialog; the rig passes the folder-trust dialog by keys into its own pty only, never a desktop key (on a read screen at most 2 arrows toward the trust row and 1 Enter only when that row reads selected, any other screen no key; on a wrapped start the keys move the wheel to the human, and `viola release <name>` precedes the driver's send); the CLI's writes are its own — its trust entry in `~/.claude.json` for each `proj/` whose dialog was accepted, the session transcripts under `~/.claude/projects/`, and after the removal its orphaned plugin cache `~/.claude/plugins/cache/viola/viola/<version>/` (re-written in place by a later install and uninstall), `installed_plugins.json` and `known_marketplaces.json` back as before (as measured on claude 2.1.287 at chunk 2026-10-18-plugin-precedence-live-readings); viola itself writes nothing under `~/.claude` or into `~/.claude.json`.»
