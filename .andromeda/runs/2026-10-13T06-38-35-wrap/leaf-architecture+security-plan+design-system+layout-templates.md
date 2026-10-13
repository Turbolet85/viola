group: architecture+security-plan+design-system+layout-templates
read: .claude/rules/frontend.md · 42 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/frontend.md:37   class: omission
  leaf-text: «SGR only for amber `DIALOG`, dim `stale` rows, bold NAME — always beside its word; never on `unable` / `fail`.»
  source: .andromeda/layout-templates.md:397 «`viola list` prints this uncoloured form on every stream, a TTY included, as one stdout write»
  replace-with: «SGR only for amber `DIALOG`, dim `stale` rows, bold NAME — always beside its word; never on `unable` / `fail`. As built, `list` prints its uncoloured form on every stream, a TTY included (wrapped rows only, STATUS always `unknown`): the colour layer is owed to "CLI output discipline", the unwrapped rack to "Unwrapped sessions on the board".»

- leaf: .claude/rules/frontend.md:38   class: omission
  leaf-text: «refusals `unable  <name>  <reason>  <detail>` + one `hint:` line»
  source: .andromeda/design-system.md:724 «only the exit-1 refusals of `viola run`, `viola revive` and `viola verify`, and `viola list`'s exit-21 strict-modes refusal»
  replace-with: «refusals `unable  <name>  <reason>  <detail>` (the exit-1 refusals of `run` / `revive` / `verify` and `list`'s exit-21 strict-modes refusal take the fixed form `unable: <text>`) + one `hint:` line (none under `--json`)»

- leaf: .claude/rules/frontend.md:39   class: omission
  leaf-text: «`list` rows escape C0/C1 including `\n`/`\t`; `wait`/`last` text escapes C0/C1 but keeps `\n`/`\t`.»
  source: .andromeda/security-plan.md:437 «Human-mode `list` rows escape every control character, `\n`, `\t`, DEL and C1 included, as `\xHH` text (`escape_row`), so each row stays one line»
  replace-with: «`list` rows escape every control character (C0 with `\n`/`\t`, DEL, C1) as `\xHH` text (`escape_row`); `wait`/`last` text escapes C0, DEL and C1 as `\xHH` but keeps `\n`/`\t`; nothing is stripped.»
