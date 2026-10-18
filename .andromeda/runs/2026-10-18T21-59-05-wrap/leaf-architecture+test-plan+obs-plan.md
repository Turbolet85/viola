group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:23   class: stale
  leaf-text: «with five compiled pastes (the probe prompt, a long text, a tag-like text, a 113-byte text holding one inner LF, `/clear`)»
  source: .andromeda/architecture.md:87 «`PROBE_NEWLINE_PASTE` (113 bytes, ASCII, exactly one LF at neither end, no CR), `PROBE_COMPACT_COMMAND` (the list's `/compact`) and `PROBE_LOCAL_COMMAND` (the list's `/clear`)»
  replace-with: «with six compiled pastes (the probe prompt, a long text, a tag-like text, a 113-byte text holding one inner LF, `/compact`, `/clear`)»

- leaf: .claude/docs/commands.md:23   class: stale
  leaf-text: «before the four added pastes»
  source: .andromeda/architecture.md:87 «each of the five added ones only into rows that hold the input-box literal and no modal literal»
  replace-with: «before the five added pastes»

- leaf: .claude/docs/commands.md:23   class: stale
  leaf-text: «check the relied ledger rows (`LedgerRow::relied()`: all twenty-three on Unix; twenty on Windows, the first eighteen, `resume-session` and `agents-join`), print `[NN/MM]` step lines (`MM` the relied count; the last `[23/23] agents-join …` on Unix, `[20/20] agents-join …` on Windows)»
  source: .andromeda/architecture.md:87 «`MM` being `LedgerRow::relied().len()`: 24 on Unix, the last `[24/24] local-command-compact …`; 21 on Windows, the last `[21/21] local-command-compact …`)»
  replace-with: «check the relied ledger rows (`LedgerRow::relied()`: all twenty-four on Unix; twenty-one on Windows, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`), print `[NN/MM]` step lines (`MM` the relied count; the last `[24/24] local-command-compact …` on Unix, `[21/21] local-command-compact …` on Windows)»

- leaf: .claude/docs/commands.md:23   class: omission
  leaf-text: «Run B's `paste-<n>` / `clear-<n>` framing variants»
  source: .andromeda/architecture.md:87 «Run B's framing captures as `UserPromptSubmit.paste-<n>.json`, `SessionStart.compact-<n>.json`, `SessionEnd.clear-<n>.json` and `SessionStart.clear-<n>.json` variants»
  replace-with: «Run B's `paste-<n>` / `compact-<n>` / `clear-<n>` framing variants»

- leaf: .claude/docs/commands.md:24   class: stale
  leaf-text: «a version is verified only when its stamp holds every relied row (`LedgerRow::relied()`: all twenty-three on Unix; twenty on Windows, the first eighteen, `resume-session` and `agents-join`) `pass`»
  source: .andromeda/architecture.md:198 «A version is verified only when every row of `LedgerRow::relied()` reads `pass` (all twenty-four on Unix; on Windows twenty-one, the first eighteen, `resume-session`, `agents-join` and»
  replace-with: «a version is verified only when its stamp holds every relied row (`LedgerRow::relied()`: all twenty-four on Unix; twenty-one on Windows, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`) `pass`»

- leaf: .claude/docs/commands.md:24   class: stale
  leaf-text: «a twenty-one-row stamp written before chunk 2026-10-14-resume-and-compact-ledger-rows or a twenty-two-row stamp written before chunk 2026-10-16-unwrapped-sessions-on-the-board (and so, on Windows, does a stamp of the eighteen and `resume-session`; one of the eighteen, `resume-session` and `agents-join` reads verified on Windows only)»
  source: .andromeda/architecture.md:87 «or exactly the twenty-three ids every stamp written before chunk 2026-10-18-compact-post-condition-ledger-row holds, all `pass`, reads unverified on every OS, and so do one holding the eighteen and»
  replace-with: «a twenty-one-row stamp written before chunk 2026-10-14-resume-and-compact-ledger-rows, a twenty-two-row stamp written before chunk 2026-10-16-unwrapped-sessions-on-the-board or a twenty-three-row stamp written before chunk 2026-10-18-compact-post-condition-ledger-row (and so, on Windows, do a stamp of the eighteen and `resume-session` and one of the eighteen, `resume-session` and `agents-join`; one of the eighteen, `resume-session`, `agents-join` and `local-command-compact` reads verified on Windows only)»
