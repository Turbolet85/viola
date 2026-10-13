# layouts extract

## Relevance
partial — the chunk touches the cli surface only, through `viola verify`'s step lines and summary (new ledger rows grow `MM`), and possibly `viola send`'s outcome line for `/compact`. It changes no web-spa region and adds no focusable element, modal or breakpoint.

## Constraints
- layout-templates §Output structure — `viola verify` requires one stdout step line per ledger row, `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, with `MM` growing "as owning chunks land their rows". Each row this chunk adds to `LedgerRow::ALL` therefore owes a step line with its id and words, in its ledger slot. Whether the step-line printer already derives `MM` and the order from `ALL`, rather than from a literal 17, is research's question.
- layout-templates §Output structure — `viola verify` requires the summary `stamped <ver>  <n> pass  <m> fail` as the last stdout line, plain ASCII, no colour, each step line appended once, no progress bar and no redraw. A probe that needs a live turn (a `/compact` paste, a `--resume` child) or a fifth Run B paste still gets only its one step line and adds no interim or progress output.
- layout-templates §Output structure — `viola verify` requires that `--record` adds no human line, and that the stop pair (`unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`) stays as it is when new probes are added. A run stopped during a new probe still prints that pair only, with no step line and no summary.
- layout-templates §Component — Footer / terminator requires that the `stamped …` summary closes a completed verify run and that no terminator word (`done.`, `success`) or summary banner is added.
- layout-templates §Surface: cli (Signature placement) requires that `/clear` on a verified CLI version be confirmed `[RB] read back` by its new-session post-condition, and that an unmeasured local command read `[  ] unconfirmable …` on stdout with exit 0. If `/compact` joins the local-command list with a ledger post-condition, its `send` outcome takes one of these two existing forms and no new word. Which form it takes (read back by a `session-start` of cause `compact`, or unconfirmable) is P4's decision, and how the code treats `/compact` today is research's question.
- layout-templates §Component — Primary content block 2 requires each `viola verify` exit-1 refusal to stay a fixed `unable: <text>` / `hint:` pair with no name, path or pid. That includes the `--record` fixture-cleanliness pair (`<file> <code>`, codes `home-path` · `absolute-path` · `username` · `email`), which re-recorded fixtures pass through.
- layout-templates §IA notes requires the human output to stay stable as a contract that LLM drivers may read. New row ids and row words are additions within the existing step-line form, not a new column or format.

## Patterns to follow
- The 17 step lines already enumerated in layout-templates §Output structure — `viola verify` (`shim-resolution` … `local-command-clear`): a new row's id is a lower-case hyphenated noun phrase, and its words are one plain clause stating the measured post-condition (e.g. `long-paste-wrapper a long paste unwraps to the text as pasted`).
- The `local-command-clear` row (`/clear starts a new session and submits no prompt`) is the precedent for a `/compact` row, which would also have a session-start post-condition and no prompt submitted (layout-templates §Output structure — `viola verify`).
- The `viola revive --list` cause column already has the words `startup`, `clear`, `resume`, `compact` (layout-templates §Output structure — `viola revive`). A recorded SessionStart of source `resume` that replaces the fake agent's rewritten payload needs no new word there.
- The verify `--help` paragraph is static ASCII and names no path (layout-templates §Output structure — `viola verify`). Any probe-precondition wording a new child needs (a resumable session) follows that form.

## Anti-patterns to avoid
- Do not add a spinner, progress bar, elapsed-time counter, in-place redraw or colour to `verify` while a live probe runs, and never colour `fail` (layout-templates §Output structure — `viola verify`).
- Do not add a new outcome word or glyph to `viola send` for a newly listed local command. Its outcome stays `[RB] read back`, `[/ ] unable` or `[  ] unconfirmable` (layout-templates §Component — Hero / signature output line).
- Do not change the human column set or line form to show the new rows. New machine fields go into `--json` first (layout-templates §IA notes).

## Contract bindings
- The verify step lines are bound to architecture §Cross-cutting Patterns (Capability ledger): the row ids, their order and the `MM` count come from `LedgerRow::ALL` in `crates/viola-agent-claude/src/ledger.rs`. The 17-id list and "`MM` is 17 today" in layout-templates §Output structure — `viola verify` are a master statement that must be amended to the new count, with the rule that produced it (the `ALL` list), when rows land.
- The `send` outcome for a local command is bound to the security-plan input-validation rule: the exact match and the local-command list read the typed text (`.claude/rules/security.md` §Input validation). A `/compact` listing changes which layout §Surface: cli (Signature placement) outcome form applies.
- The verify output form is bound to design-system §Surface: cli (plain ASCII, colour reserved for `DIALOG` / `stale`).

## Acceptance criteria contributions
- (layouts) `viola verify` on a stamping run prints exactly one `[NN/MM] <row id> <row words>  pass|fail` line per `LedgerRow::ALL` entry, in `ALL` order, with `MM` equal to `ALL`'s length after this chunk's rows land, then `stamped <ver>  <n> pass  <m> fail` as the last stdout line where n + m = MM (per layout-templates §Output structure — `viola verify`).
- (layouts) The new probes add no stdout or stderr line beyond their step line: no progress output, no colour, no redraw. A run stopped during one prints only the `unable:` / `hint:` stop pair, with stdout empty (per layout-templates §Output structure — `viola verify`).
- (layouts) If `/compact` is listed as a local command, `viola send` of `/compact` prints either `[RB] read back …` (exit 0, confirmed by its post-condition on a verified CLI) or `[  ] unconfirmable …` (exit 0), and never `[/ ] unable … no-prompt-submitted` (per layout-templates §Surface: cli, Signature placement).
- (layouts) layout-templates §Output structure — `viola verify`'s row-id list and `MM` figure match the landed `ALL` after the wrap's amendment (per layout-templates §Output structure — `viola verify`).
