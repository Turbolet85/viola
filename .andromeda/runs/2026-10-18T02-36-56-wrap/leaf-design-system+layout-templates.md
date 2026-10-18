group: design-system+layout-templates
read: .claude/docs/design-summary.md · 73 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:50   class: omission
  leaf-text: «the `viola send` readback mirror, phraseology verb output, `viola verify` step counter»
  source: .andromeda/design-system.md:502 «`viola plugin install` → four stdout lines in one write: `viola@viola  written  <dir>` / `run these to add it to Claude Code:` / `  claude plugin marketplace add <dir>`»
  replace-with: «the `viola send` readback mirror, phraseology verb output (`viola plugin install`: four plain-ASCII stdout lines in one write, no SGR, `viola@viola  written  <dir>` / `run these to add it to Claude Code:` / `  claude plugin marketplace add <dir>` / `  claude plugin install viola@viola`, `<dir>` quoted only where the host's shell would split it; under `--json` `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`; a refused strict-modes check is exit 21 with `viola list`'s home pair on stderr, or under `--json` the `instance-unreachable` document), `viola verify` step counter»
