group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/ci.md · 35 · «- 2026-10-13: A CI job that reads `cancelled` with no steps »
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/security-summary.md:73   class: omission
  leaf-text: «viola itself writes nothing under `~/.claude` or into `~/.claude.json`.»
  source: .andromeda/security-plan.md:262 «It is no product path and no standing permission: another such reading needs its own answer.»
  replace-with: «viola itself writes nothing under `~/.claude` or into `~/.claude.json`. Accepted residual (a scratch footer reading of a newer CLI): one `viola run` start of the CLI under a tmpfs `--home`, in a detached 80×24 pane, its cwd a fresh `.viola-footer-<pid>/` under the repository root, with no key typed; one `viola list` read against that home; at two instants the pane's bottom three screen rows, scrubbed of the home and the user word, written to the chunk's `evidence/`, never a row above the footer; then the start is killed and its cwd removed; it left no transcript under `~/.claude/projects/` (as measured on 2.1.302 at chunk 2026-10-19-newer-claude-cli-rows). It is no product path and no standing permission: another such reading needs its own answer.»
