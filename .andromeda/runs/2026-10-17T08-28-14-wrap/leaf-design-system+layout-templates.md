group: design-system+layout-templates
read: .claude/docs/design-summary.md · 73 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:50   class: stale
  leaf-text: «as built: uncoloured on every stream»
  source: .andromeda/layout-templates.md:91 «It is uncoloured under `--json`, a piped stdout, `NO_COLOR` (an empty value included), `TERM=dumb` or a failed Windows VT enable; on a terminal that takes colour it is the styled human-TTY form above»
  replace-with: «as built: uncoloured under `--json`, a piped stdout, `NO_COLOR` (an empty value included), `TERM=dumb` or a failed Windows VT enable, else NAME bold, the `DIALOG` word amber with its kind word plain and a wrapped `stale` row dim end to end; on a terminal whose width is known (`viola_pty::host_size()`, never `COLUMNS`) only an unwrapped NAME too long for its column is cut with `...`»
