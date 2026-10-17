group: masters
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:27   class: stale
  leaf-text: «`list` (`src/cmd/list.rs`: uncoloured, the strict-modes scan before any log,»
  source: .andromeda/layout-templates.md:91 «It is uncoloured under `--json`, a piped stdout, `NO_COLOR` (an empty value included), `TERM=dumb` or a failed Windows VT enable; on a terminal that takes colour it is the styled human-TTY form above»
  replace-with: «`list` (`src/cmd/list.rs`: styled only on a terminal that takes colour, through `src/human/style.rs` (bold NAME, amber `DIALOG`, a dim stale row; no SGR under `--json`, a pipe, `NO_COLOR` or `TERM=dumb`), the strict-modes scan before any log,»

- leaf: CLAUDE.md:37   class: omission
  leaf-text: «env vars are not a configuration channel (no `RUST_LOG`, `EnvFilter` or `OTEL_*`).»
  source: .andromeda/security-plan.md:543 «The terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM` are read in every build (the founder's own word, chunk 2026-10-17-cli-output-discipline), only by `viola list`'s human mode»
  replace-with: «env vars are not a configuration channel (no `RUST_LOG`, `EnvFilter` or `OTEL_*`; outside `VIOLA_*` plumbing a build reads only the `fake-agent` test seams and, in every build, `NO_COLOR` / `TERM` / `COLORTERM`, by `viola list`'s human mode only to choose its SGR bytes).»
