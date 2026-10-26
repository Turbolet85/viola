group: design-system+layout-templates
read: .claude/docs/design-summary.md · 73 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»
- leaf: .claude/docs/design-summary.md:50   class: omission
  leaf-text: «`viola verify` step counter,»
  source: .andromeda/layout-templates.md:170 «`viola link <driver> <driven> [--json]` and `viola unlink <driver> <driven> [--json]` act on the driven instance and each print one stdout line in one write, plain in every mode, exit 0»
  replace-with: «`viola link` / `viola unlink` (one plain-ASCII stdout line in one write, no SGR, exit 0: `link    <driver> -> <driven>  since <HH:MM:SSZ>`, `link    <driver> -> <driven>  already linked` for a held pair, `unlink  <driver> -> <driven>` whether or not a pair was removed; under `--json` `{"v":1,"ok":{"since":<ts>,"already_linked":<bool>}}` / `{"v":1,"ok":{}}`; a refused strict-modes check of the driven instance is exit 21 with `revive`'s instance pair on stderr naming it, or under `--json` the `instance-unreachable` document), `viola verify` step counter,»
