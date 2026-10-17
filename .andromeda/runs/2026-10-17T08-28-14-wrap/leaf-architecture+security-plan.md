group: architecture+security-plan
read: .claude/rules/api.md · 42 · «_This section is owned by `/wrap-session`. setup-project pre»
- leaf: .claude/rules/api.md:29   class: omission
  leaf-text: «Human text by default, `--json` for agents (»
  source: .andromeda/security-plan.md:208 «Read through `std::env::var_os` as text with stdout's `IsTerminal`, only compared, never echoed, and decided into the closed `Depth` (`Truecolor` · `Ansi256` · `Ansi16`) or none: none under `--json`, »
  replace-with: «Human text by default (`viola list`'s human mode alone reads `NO_COLOR`, `TERM` and `COLORTERM` in every build, never set, only to decide whether and how deep its SGR bytes are written: none under `--json`, a non-terminal stdout, `NO_COLOR` present, `TERM` exactly `dumb` or a failed Windows VT enable; they configure nothing and reach no log line, error or refusal; `COLUMNS`, `CLICOLOR`, `CLICOLOR_FORCE` and `CI` are not read), `--json` for agents (»
