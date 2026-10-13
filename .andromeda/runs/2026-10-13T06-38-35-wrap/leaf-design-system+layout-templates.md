group: design-system+layout-templates
read: .claude/docs/design-summary.md · 78 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:9   class: omission
  leaf-text: «`--json`/non-TTY/`viola run` 0.0»
  source: .andromeda/layout-templates.md:347 «0.0 for `--json`, non-TTY, `NO_COLOR`, `TERM=dumb` and the `viola run` / `viola revive` passthrough: no colour, no non-ASCII glyph and no cursor control.»
  replace-with: «`--json`/non-TTY/`NO_COLOR`/`TERM=dumb`/`viola run` and `viola revive` passthrough 0.0»

- leaf: .claude/docs/design-summary.md:11   class: omission
  leaf-text: «every tape send line and every outbound transfer marker»
  source: .andromeda/layout-templates.md:46 «At the far right of every outbound `<viola-transfer>` marker in the WRAPPED rack gap, once a send on that link has been seen since page open.»
  replace-with: «every tape send line and every outbound transfer marker once a send on its link has been seen since page open (before that the marker prints only the word `unknown`, no box)»

- leaf: .claude/docs/design-summary.md:55   class: omission
  leaf-text: «`viola list` (the k9s-style board, same six columns and words)»
  source: .andromeda/design-system.md:749 «**As built** (chunk 2026-10-11-the-board-viola-list): wrapped rows only, uncoloured. STATUS is always `unknown` (no `claude agents --json` read)»
  replace-with: «`viola list` (the k9s-style board, same six columns and words; as built: wrapped rows only, uncoloured on every stream, STATUS always `unknown`, and a failed strict-modes check refuses the whole board with exit 21)»

- leaf: .claude/docs/design-summary.md:55   class: omission
  leaf-text: «phraseology verb output, `viola verify` step counter»
  source: .andromeda/layout-templates.md:363 «`viola revive <name>` resumes a dead instance in place (its newest logged session, in the recorded directory) and is passthrough like `viola run`.»
  replace-with: «phraseology verb output, `viola verify` step counter, the `viola run` / `viola revive` passthrough (nothing printed while the child runs; `viola revive <name> --list` prints one static line per logged session)»

- leaf: .claude/docs/design-summary.md:59   class: retired
  leaf-text: «native `<tr>` under a `display: contents` host»
  source: .andromeda/design-system.md:380 «The Lit-era host form (light-DOM `viola-*` custom elements via `createRenderRoot(){ return this; }`, role-less `display: contents` hosts) is retired with Lit.»
  replace-with: «native `<tr>` with `<td>` cells, any element between `<tbody>` and it role-less `display: contents` (how a `viola-*` name maps onto React's output is OPEN, owned by the frontend-toolchain entry)»
