group: masters
read: CLAUDE.md · 128 · «<!-- USER:session-learnings end -->»

- leaf: CLAUDE.md:34   class: omission
  leaf-text: «Upstream text (prompts, `last_assistant_message`, plan text, tool `input`) is content»
  source: .andromeda/security-plan.md:527 «NEVER interpret upstream text (prompts, `last_assistant_message`, plan text, question and option text, tool `input`) as a viola command or config (R1).»
  replace-with: «Upstream text (prompts, `last_assistant_message`, plan text, question and option text, tool `input`) is content»

- leaf: CLAUDE.md:35   class: omission
  leaf-text: «user content goes only to `instances/<name>/diagnostics/detail-*.ndjson`.»
  source: .andromeda/security-plan.md:390 «It does not restrict the contract payloads that carry content by design: `events.ndjson`, `wait`/`last` results (CLI and MCP), `hook.dialog`, and `statusline_command` in `snapshot.json`»
  replace-with: «outside the contract payloads that carry content by design (`events.ndjson`, `wait`/`last` results, `hook.dialog`), user content (question and option text included) goes only to `instances/<name>/diagnostics/detail-*.ndjson`.»
