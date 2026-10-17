group: architecture+design-system+layout-templates
read: .claude/rules/frontend.md · 33 · «_This section is owned by `/wrap-session`. setup-project pres»

- leaf: .claude/rules/frontend.md:29   class: omission
  leaf-text: «per-verb `--help` is clap's usage, unchanged.»
  source: .andromeda/layout-templates.md:216 «`viola mcp` (chunk 2026-10-17-mcp-server-for-drivers) is a hidden verb (`#[command(hide = true)]`, no argument but the global `--home`), run only by the plugin»
  replace-with: «per-verb `--help` is clap's usage, unchanged; the hidden `viola mcp` (`#[command(hide = true)]`, no argument but the global `--home`, run only by the plugin, never by a person) appears in no row, and the `HELP` literal is unchanged.»

- leaf: .claude/rules/frontend.md:29   class: omission
  leaf-text: «stdout = results; stderr = `waiting:`»
  source: .andromeda/architecture.md:301 «and exits 1 with nothing on stdout or stderr and no role file when its home or `diagnostics/` (each that exists) fails `viola_state::strict::check_path`.»
  replace-with: «stdout = results (`viola mcp` writes only MCP frames, and exits 1 with nothing on stdout or stderr when its home or `diagnostics/` fails `check_path`); stderr = `waiting:`»
