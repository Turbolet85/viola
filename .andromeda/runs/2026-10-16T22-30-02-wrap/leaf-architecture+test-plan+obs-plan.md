group: architecture+test-plan+obs-plan
read: .claude/docs/commands.md · 89 · «- `scripts/agent-run.sh logs --instance <name> | jq -c 'sele»

- leaf: .claude/docs/commands.md:23   class: stale
  leaf-text: «check the relied ledger rows (`LedgerRow::relied()`: all twenty-two on Unix; nineteen on Windows, the first eighteen and `resume-session`), print»
  source: .andromeda/architecture.md:88 «so a Unix host relies on all twenty-three rows and a Windows host on twenty, the first eighteen, `resume-session` and `agents-join`»
  replace-with: «check the relied ledger rows (`LedgerRow::relied()`: all twenty-three on Unix; twenty on Windows, the first eighteen, `resume-session` and `agents-join`), print»

- leaf: .claude/docs/commands.md:23   class: stale
  leaf-text: «the last `[22/22] resume-session …` on Unix, `[19/19] resume-session …` on Windows»
  source: .andromeda/architecture.md:88 «`MM` being `LedgerRow::relied().len()`: 23 on Unix, the last `[23/23] agents-join …`; 20 on Windows, the last `[20/20] agents-join …`»
  replace-with: «the last `[23/23] agents-join …` on Unix, `[20/20] agents-join …` on Windows»

- leaf: .claude/docs/commands.md:23   class: omission
  leaf-text: «a 113-byte text holding one inner LF, `/clear`), Run C»
  source: .andromeda/architecture.md:88 «Once Run B's first turn has settled, before its framing pastes, verify makes ONE `claude agents --json` read»
  replace-with: «a 113-byte text holding one inner LF, `/clear`) and one `claude agents --json` read once its first turn has settled, before the four added pastes (held in memory, spawn pair `agents-probe`, no start added; in CI the fake agent's `agents` mode answers it), Run C»

- leaf: .claude/docs/commands.md:23   class: omission
  leaf-text: «Run E's first SessionStart as `SessionStart.resume-1.json` and signature-only»
  source: .andromeda/architecture.md:88 «Run B's agents read as `Agents.recorded-1.json` (Run B's own row only, `ledger::agents_variant`)»
  replace-with: «Run E's first SessionStart as `SessionStart.resume-1.json`, Run B's agents read as `Agents.recorded-1.json` (Run B's own row only) and signature-only»

- leaf: .claude/docs/commands.md:24   class: stale
  leaf-text: «a version is verified only when its stamp holds every relied row (`LedgerRow::relied()`: all twenty-two on Unix; nineteen on Windows, the first eighteen and `resume-session`) `pass`»
  source: .andromeda/architecture.md:199 «A version is verified only when every row of `LedgerRow::relied()` reads `pass` (all twenty-three on Unix; on Windows twenty, the first eighteen, `resume-session` and `agents-join`)»
  replace-with: «a version is verified only when its stamp holds every relied row (`LedgerRow::relied()`: all twenty-three on Unix; twenty on Windows, the first eighteen, `resume-session` and `agents-join`) `pass`»

- leaf: .claude/docs/commands.md:24   class: stale
  leaf-text: «an eighteen-row stamp written before chunk 2026-10-13-owed-ledger-rows or a twenty-one-row stamp written before chunk 2026-10-14-resume-and-compact-ledger-rows (a stamp of the eighteen and `resume-session` reads verified on Windows only)»
  source: .andromeda/architecture.md:88 «or exactly the twenty-two ids every stamp written before chunk 2026-10-16-unwrapped-sessions-on-the-board holds, all `pass`, reads unverified on every OS, and so does one holding the eighteen and `resume-session` on Windows»
  replace-with: «an eighteen-row stamp written before chunk 2026-10-13-owed-ledger-rows, a twenty-one-row stamp written before chunk 2026-10-14-resume-and-compact-ledger-rows or a twenty-two-row stamp written before chunk 2026-10-16-unwrapped-sessions-on-the-board (and so, on Windows, does a stamp of the eighteen and `resume-session`; one of the eighteen, `resume-session` and `agents-join` reads verified on Windows only)»

- leaf: .claude/docs/commands.md:27   class: omission
  leaf-text: «(`--statusline-echo` is not built; it is owed to the route entry "End-to-end path binary and harness legs")»
  source: .andromeda/registries/contracts/test-plan/5-command-implementation.md:4 «`--agents-mode` (still not built, owed to "End-to-end path binary and harness legs", with `status` running `viola list`) writes the fake agent's `agents-mode` file»
  replace-with: «(`--statusline-echo` and `--agents-mode` are not built; both are owed to the route entry "End-to-end path binary and harness legs"; `--agents-mode` will write the fake agent's `agents-mode` file beside the session's `bin/claude[.exe]` copy, setting no env var)»

- leaf: .claude/docs/commands.md:21   class: omission
  leaf-text: «resuming its newest logged session in the recorded directory;»
  source: .andromeda/architecture.md:90 «`session-live` (chunk 2026-10-16-unwrapped-sessions-on-the-board) is one `claude agents --json` read: a row whose `sessionId` is the logged id about to be resumed refuses, `--fork` skips the reading»
  replace-with: «resuming its newest logged session in the recorded directory (refused `session-live`, exit 1, when one `claude agents --json` read lists that session live in another `claude`; `--fork` skips the reading and a failed read passes);»
