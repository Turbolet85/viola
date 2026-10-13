group: obs-plan
read: .claude/docs/obs-summary.md · 77 · «**Full plan:** `.andromeda/obs-plan.md`. Path-scoped rules: »
- leaf: .claude/docs/obs-summary.md:68   class: omission
  leaf-text: «and from `viola revive`'s preflight `no-session|cwd-missing`, its strict-modes refusal writing no line since chunk 2026-10-10-budget-governor;»
  source: .andromeda/obs-plan.md:835 «And `interrupted`: a `viola verify` stopped by Ctrl-C or TERM (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break) before its rows are checked ends with one»
  replace-with: «and from `viola revive`'s preflight `no-session|cwd-missing`, its strict-modes refusal writing no line since chunk 2026-10-10-budget-governor, and `interrupted` from a `viola verify` stopped by Ctrl-C or TERM, after every spawn pair it opened is closed (chunk 2026-10-13-interrupted-verify-cleanup);»
