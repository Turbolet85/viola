group: security-plan
read: .claude/docs/security-summary.md · 80 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 47 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/security-summary.md:75   class: stale
  leaf-text: «(0700, removed on every exit path)»
  source: .andromeda/security-plan.md:283 «All are 0700 and removed when verify passes, fails, or is stopped by Ctrl-C or TERM (a drop guard each; the repo's `.gitignore` carries `/.viola-verify-*/` for a leftover).»
  replace-with: «(0700, removed when verify passes, fails or is stopped by Ctrl-C or TERM; SIGKILL, a crash, a failed handler install and a Windows close, logoff or shutdown leave it to be removed by hand)»

- leaf: .claude/docs/security-summary.md:75   class: omission
  leaf-text: «Ctrl-C ×2 then kill;»
  source: .andromeda/security-plan.md:131 «It is ended by Ctrl-C, Ctrl-C again after 500 ms, then a kill; a run stopped by Ctrl-C or TERM is ended by a kill with no key and pastes nothing more, and no later run starts.»
  replace-with: «Ctrl-C ×2 then kill (a run stopped by Ctrl-C or TERM is killed with no key, pastes nothing more, and no later run starts);»

- leaf: .claude/rules/security.md:19   class: stale
  leaf-text: «are 0700 and removed on every exit path;»
  source: .andromeda/security-plan.md:283 «All are 0700 and removed when verify passes, fails, or is stopped by Ctrl-C or TERM (a drop guard each; the repo's `.gitignore` carries `/.viola-verify-*/` for a leftover).»
  replace-with: «are 0700 and removed when verify passes, fails or is stopped by Ctrl-C or TERM (Unix `SIGINT` / `SIGTERM`, Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT`; each guard removes only what that run created); SIGKILL, a crash, a failed handler install and a Windows close, logoff or shutdown leave them to be removed by hand;»

- leaf: .claude/rules/security.md:19   class: omission
  leaf-text: «a modal start is killed with no key;»
  source: .andromeda/security-plan.md:131 «It is ended by Ctrl-C, Ctrl-C again after 500 ms, then a kill; a run stopped by Ctrl-C or TERM is ended by a kill with no key and pastes nothing more, and no later run starts.»
  replace-with: «a modal start is killed with no key; a run stopped by Ctrl-C or TERM is killed with no key, pastes nothing more, and no later run starts;»
