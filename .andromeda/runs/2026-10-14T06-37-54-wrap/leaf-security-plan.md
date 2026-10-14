group: security-plan
read: .claude/docs/security-summary.md · 80 · «**Full plan:** `.andromeda/security-plan.md`.»
read: .claude/rules/security.md · 47 · «_This section is owned by `/wrap-session`. setup-project pre»

- leaf: .claude/docs/security-summary.md:38   class: stale
  leaf-text: «The stamp holds eighteen rows (six spine, four screen and timing, four dialog-tier, three framing: `long-paste-wrapper` · `tag-escaping` · `local-command-clear`, and the newline row `paste-newline`), so a decision flows only when every row passes (`14 pass  4 fail`, or a stamp of the fourteen older ids or of the seventeen written before chunk 2026-10-13-paste-newline-ledger-row all `pass`, → `cli_verified:false`, `answer` exit 12)»
  source: .andromeda/security-plan.md:617 «a stamp is read over the relied set (`LedgerRow::relied()`, counter `/21` on Unix; on Windows, where the status line is not wrapped, the first eighteen, counter `/18`), so a non-`null` decision flows »
  replace-with: «The ledger has twenty-one rows (six spine, four screen and timing, four dialog-tier, three framing: `long-paste-wrapper` · `tag-escaping` · `local-command-clear`, the newline row `paste-newline`, and the three statusline rows `statusline-override` · `statusline-shell` · `statusline-rate-limits`), and a stamp is read over the relied set (`LedgerRow::relied()`: all twenty-one on Unix; on Windows, where the status line is not wrapped, the first eighteen), so a decision flows only when every relied row passes (`17 pass  4 fail` on Unix, or a stamp of the fourteen older ids or of the seventeen written before chunk 2026-10-13-paste-newline-ledger-row all `pass`, or on Unix one of the eighteen written before chunk 2026-10-13-owed-ledger-rows, → `cli_verified:false`, `answer` exit 12)»

- leaf: .claude/docs/security-summary.md:69   class: stale
  leaf-text: «The refusal also rests on the `rate_limits` shape with no ledger row.»
  source: .andromeda/security-plan.md:248 «and `rate_limits` (`statusline-rate-limits`, which the `budget-paused` refusal rests on) rowed beside it, relied on Unix only and stamped on 2.1.287 with the two `StatusLine` fixtures»
  replace-with: «The refusal also rests on the `rate_limits` shape, since chunk 2026-10-13-owed-ledger-rows the ledger row `statusline-rate-limits` (relied on Unix only, stamped on 2.1.287).»

- leaf: .claude/docs/security-summary.md:70   class: stale
  leaf-text: «The three CLI shapes have no ledger row yet.»
  source: .andromeda/security-plan.md:248 «since chunk 2026-10-13-owed-ledger-rows it is the ledger row `statusline-shell`, with the settings override (`statusline-override`) and `rate_limits` (`statusline-rate-limits`, which the `budget-pause»
  replace-with: «Since chunk 2026-10-13-owed-ledger-rows the CLI shapes it relies on are ledger rows (`statusline-override` · `statusline-shell` · `statusline-rate-limits`), relied on Unix only and stamped on 2.1.287 with the two `StatusLine` fixtures; the Windows shell-out is owed to "Windows-only live measurements".»

- leaf: .claude/docs/security-summary.md:75   class: omission
  leaf-text: «five compiled-literal pastes (`PROBE_PROMPT`»
  source: .andromeda/security-plan.md:131 «Run B runs trusted in `<cwd>/.viola-verify-<pid>/`; on a wrapped OS (Unix), and only when the pinned path and the captures dir are shell-plain, it also gets exactly one `--settings <probe dir>/typed/s»
  replace-with: «on Unix, and only when the pinned path and the captures dir are shell-plain, exactly one probe `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair (0600, written by `replace_private`) whose status line is the hidden `hook statusline --capture <probe dir>/typed/statusline --shell "$0"`, none on Windows, no run or start added (the founder's own answer to P4 fork 3 of chunk 2026-10-13-owed-ledger-rows), five compiled-literal pastes (`PROBE_PROMPT`»

- leaf: .claude/docs/security-summary.md:75   class: stale
  leaf-text: «run the user's global hooks and status line»
  source: .andromeda/security-plan.md:291 «and the user's own global hooks run for that session, with the user's own status line in Runs C and D and in Run B on Windows (on Unix Run B's status line is the probe capture its one probe `--setting»
  replace-with: «run the user's global hooks, with the user's own status line in Runs C and D and in Run B on Windows (on Unix Run B's status line is the probe capture its one probe `--settings` names)»

- leaf: .claude/docs/security-summary.md:75   class: omission
  leaf-text: «never content (founder-ratified live at the chunk's wrap)»
  source: .andromeda/security-plan.md:288 «A `workspace.repo` naming this repository's own remote is kept in a fixture: the status line variants' `workspace.repo.owner`, the remote owner's handle, is neither rewritten by the user-word scrub no»
  replace-with: «never content (founder-ratified live at the chunk's wrap); a `workspace.repo` naming this repository's own remote is kept in a fixture, its `workspace.repo.owner` neither rewritten by the user-word scrub nor refused (the founder's own word, chunk 2026-10-13-owed-ledger-rows)»

- leaf: .claude/rules/security.md:15   class: omission
  leaf-text: «exit 0 always. No other caller may register it.»
  source: .andromeda/security-plan.md:231 «**Statusline capture arm (hidden, `hook statusline --capture <DIR> [--shell <WORD>]`, called only by Run B's probe settings):** taken before the statusline pass-through; `<DIR>` must be absolute and e»
  replace-with: «exit 0 always; its statusline form `hook statusline --capture <DIR> [--shell <WORD>]` (Run B's probe settings only; the founder's own answer to P4 fork 3 of chunk 2026-10-13-owed-ledger-rows) holds the same bounds, writes the payload raw and only the closed `ShellVerdict` (`bin-sh` · `other`, never the word), and prints nothing. No other caller may register it.»

- leaf: .claude/rules/security.md:19   class: omission
  leaf-text: «Run B's five pastes are compiled literals»
  source: .andromeda/security-plan.md:131 «Run B runs trusted in `<cwd>/.viola-verify-<pid>/`; on a wrapped OS (Unix), and only when the pinned path and the captures dir are shell-plain, it also gets exactly one `--settings <probe dir>/typed/s»
  replace-with: «Run B on Unix gets exactly one probe `--settings <probe dir>/typed/statusline.json` (0600, `replace_private`, only when the pinned path and the captures dir are shell-plain; none on Windows; no run or start added; the founder's own answer to P4 fork 3 of chunk 2026-10-13-owed-ledger-rows) whose status line is the hidden `hook statusline --capture` arm; Run B's five pastes are compiled literals»

- leaf: .claude/rules/security.md:19   class: omission
  leaf-text: «never the content.»
  source: .andromeda/security-plan.md:288 «A `workspace.repo` naming this repository's own remote is kept in a fixture: the status line variants' `workspace.repo.owner`, the remote owner's handle, is neither rewritten by the user-word scrub no»
  replace-with: «never the content; a `workspace.repo` naming this repository's own remote is kept in a fixture, its `workspace.repo.owner` neither rewritten by the user-word scrub nor refused (the founder's own word, chunk 2026-10-13-owed-ledger-rows).»

- leaf: .claude/rules/security.md:29   class: omission
  leaf-text: «`viola hook statusline` canonicalises its instance directory»
  source: .andromeda/security-plan.md:231 «taken before the statusline pass-through; `<DIR>` must be absolute and exist; stdin through `take(MAX_FRAME + 1)`, refused past `MAX_FRAME`; the payload is written raw at the first free `StatusLine.<k»
  replace-with: «`viola hook statusline` without `--capture` (the hidden capture form is taken first and is `viola verify`'s probe capture alone) canonicalises its instance directory»
