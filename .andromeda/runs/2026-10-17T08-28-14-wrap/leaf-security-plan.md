group: security-plan
read: .claude/docs/security-summary.md · 77 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 44 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/rules/security.md:37   class: stale
  leaf-text: «The only env vars outside `VIOLA_*` any `viola` build reads are the two test seams»
  source: .andromeda/security-plan.md:411 «the terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM`, read in every build by `viola list`'s human mode only to decide whether and how deep SGR is written»
  replace-with: «The only env vars outside `VIOLA_*` any `viola` build reads are the terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM` (every build, `viola list`'s human mode only, only compared and never echoed, deciding only whether and how deep SGR bytes are written; they reach no log line, error or refusal, configure nothing, disable no control and widen no redaction; the founder's own word, chunk 2026-10-17-cli-output-discipline) and the two test seams»

- leaf: .claude/rules/security.md:37   class: omission
  leaf-text: «Another seam, or another G2 exemption, needs a Decisions Log entry.»
  source: .andromeda/security-plan.md:543 «NEVER add another such seam or environment read, let these reach a release build, or let a colour fact change a control or reach a log line, error or refusal, without its own Decisions Log entry.»
  replace-with: «Another seam or environment read, a colour fact changing a control or reaching a log line, error or refusal, or another G2 exemption, needs a Decisions Log entry.»

- leaf: .claude/docs/security-summary.md:38   class: omission
  leaf-text: «both only under `fake-agent`, absent from release builds, switching off nothing; another seam needs its own Decisions Log entry.»
  source: .andromeda/security-plan.md:543 «The terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM` are read in every build (the founder's own word, chunk 2026-10-17-cli-output-discipline), only by `viola list`'s human mode»
  replace-with: «both only under `fake-agent`, absent from release builds, switching off nothing; the terminal colour facts `NO_COLOR`, `TERM` and `COLORTERM` are read in every build, only by `viola list`'s human mode and only to choose whether and how deep SGR bytes are written — they switch off no control, widen no redaction and are not a configuration channel (the founder's own word, chunk 2026-10-17-cli-output-discipline); another seam or environment read, or a colour fact changing a control or reaching a log line, error or refusal, needs its own Decisions Log entry.»
