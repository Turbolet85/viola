group: architecture-2
read: .claude/docs/services/viola-state.md · 49 · «- `.andromeda/architecture.md` §Standard Contracts (snapshot»
read: .claude/docs/services/viola-ui.md · 42 · «- `.claude/rules/api.md` · `.claude/rules/frontend.md` · `.c»
read: .claude/docs/services/viola.md · 58 · «- `.andromeda/architecture.md` · `.claude/rules/api.md` · `.»
read: .claude/docs/stack.md · 70 · «5. Commit as `chore: bump {tool} to {version}`»
read: .claude/docs/workflow.md · 67 · «This project uses a **reference-based** CLAUDE.md architectu»

- leaf: .claude/docs/services/viola.md:6   class: stale
  leaf-text: «checks the eighteen ledger rows (`LedgerRow::ALL`, the last `paste-newline`)»
  source: .andromeda/architecture.md:91 «It checks the relied rows (`check_rows` walks `LedgerRow::relied()`), prints one stdout step line per relied row»
  replace-with: «checks the relied ledger rows (`check_rows` walks `LedgerRow::relied()`: on Unix all twenty-one of `LedgerRow::ALL`, the last `statusline-rate-limits`; on Windows the first eighteen, the last `paste-newline`)»

- leaf: .claude/docs/services/viola.md:6   class: omission
  leaf-text: «Run B trusted in `<cwd>/.viola-verify-<pid>/` with five compiled pastes»
  source: .andromeda/architecture.md:91 «On a wrapped OS (Unix), and only for shell-plain paths, it also gets one `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair»
  replace-with: «Run B trusted in `<cwd>/.viola-verify-<pid>/` (on Unix, for shell-plain paths only, also one `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair, whose status line command `<pinned viola> hook statusline --capture <probe dir>/typed/statusline --shell "$0"` files each status line payload with its shell verdict; none on Windows) with five compiled pastes»

- leaf: .claude/docs/services/viola.md:23   class: omission
  leaf-text: «`viola hook statusline` canonicalises its instance directory»
  source: .andromeda/architecture.md:326 «With `--capture <DIR>` it takes `viola verify`'s probe capture instead, before the pass-through ([Hook Contract]): no instance check, no `budget.json` write, no command run, nothing printed»
  replace-with: «`viola hook statusline` (with `--capture <DIR>` it takes `viola verify`'s probe capture instead, before the pass-through: no instance check, no `budget.json` write, no command run, nothing printed) canonicalises its instance directory»

- leaf: .claude/docs/services/viola.md:36   class: omission
  leaf-text: «`hook <event> --capture <DIR>` (hidden) is `verify`'s capture arm: no `VIOLA_*`, no obs init, no channel, raw 0600 write, exit 0 always.»
  source: .andromeda/architecture.md:364 «Its statusline form `hook statusline --capture <DIR> [--shell <WORD>]` (`cmd::hook::capture_statusline`, sole caller Run B's probe settings) takes the same `<DIR>` and stdin bounds»
  replace-with: «`hook <event> --capture <DIR>` (hidden) is `verify`'s capture arm: no `VIOLA_*`, no obs init, no channel, raw 0600 write, exit 0 always. Its statusline form `hook statusline --capture <DIR> [--shell <WORD>]` (`cmd::hook::capture_statusline`, sole caller Run B's probe settings) files the payload raw at the first free `StatusLine.<k>.json` beside `StatusLine.<k>.shell` holding exactly `bin-sh` or `other` (the word written nowhere), prints nothing and reads no `budget.json` or snapshot.»

- leaf: .claude/docs/workflow.md:49   class: stale
  leaf-text: «all four typed runs and all eighteen ledger rows (`LedgerRow::ALL`)»
  source: .andromeda/registries/contracts/architecture/ci-cd-approach.md:20 «all four typed runs and the relied ledger rows (`LedgerRow::relied()`: all twenty-one on the Unix runners, the first eighteen on `windows-2025`;»
  replace-with: «all four typed runs and the relied ledger rows (`LedgerRow::relied()`: all twenty-one on the Unix runners, the first eighteen on `windows-2025`)»
