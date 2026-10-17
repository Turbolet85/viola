group: design-system+layout-templates
read: .claude/docs/design-summary.md · 73 · «**Full plans:** `.andromeda/design-system.md` + `.andromeda/»

- leaf: .claude/docs/design-summary.md:50   class: omission
  leaf-text: «`viola revive <name> --list` prints one static line per logged session)»
  source: .andromeda/layout-templates.md:275 «`--json` is the machine view of every data verb, `viola revive` included (both arms: its refusal document and `--list`'s `ok.sessions`; chunk 2026-10-17-cli-machine-contract).»
  replace-with: «`viola revive <name> --list` prints one static line per logged session; under `--json` both arms print one stdout document: `--list`'s `{"v":1,"ok":{"sessions":[…]}}`, a strict-modes or preflight refusal's `{"v":1,"error":"<code>","detail":null}` with exit 1 and no hint)»
