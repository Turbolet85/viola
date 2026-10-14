# layouts extract

## Relevance
partial — the chunk touches no web-spa surface; its one layout tie is the cli surface's `viola verify` step-line / summary output (one step line per ledger row added), plus `viola send`'s outcome line if `/compact` joins the local-command list with a post-condition.

## Constraints
- Every row this chunk adds to `LedgerRow::ALL` requires exactly one stdout step line `[NN/MM] <row id> <row words>  pass|fail`, in ledger order, with `MM` read from `LedgerRow::ALL.len()` (never a literal); the plan names `MM` as 18 today and growing as owning chunks land rows (per layout-templates §Surface: cli · Output structure — `viola verify`).
- The plan's ledger-order list of row ids under the `viola verify` usage paragraph must stay in step with `LedgerRow::ALL`: each new row id is appended to that list in the same order the code checks it (per layout-templates §Surface: cli · Output structure — `viola verify`).
- `viola verify` output stays plain ASCII with no colour, each step line appended once with no progress bar and no redraw; `fail` is never coloured, and `--record` adds no human line (per layout-templates §Surface: cli · Output structure — `viola verify`).
- The completed run's last stdout line is `stamped <ver>  <n> pass  <m> fail`, exit 0 only when every row passes and exit 1 on any failing row with the stamp still written; a run stopped by Ctrl-C or TERM prints only its `unable:` / `hint:` pair with no step line and no summary (per layout-templates §Surface: cli · Component — Footer / terminator).
- If `/compact` is listed as a local command with a post-condition, `viola send`'s outcome line follows the mirror's rules: `[RB] read back` on stdout with exit 0 only when the measured post-condition confirms it on a verified CLI version (the plan names only `/clear`'s new-session post-condition today), `[  ] unconfirmable` with exit 0 for an unmeasured local command; whether the code's outcome-line selection is keyed by command or by `PostCondition` is research's question (per layout-templates §Surface: cli · Signature placement).
- Any new `viola verify` refusal is an exit-1 `unable: <text>` line plus its own `hint:` line on stderr, with no name, path or pid, and a cause distinct from every listed one (per layout-templates §Surface: cli · Component — Primary content block 2).
- `viola revive --list` prints a cause of `startup`, `clear`, `resume` or `compact` as logged and `unknown` otherwise; replacing the fake agent's rewritten `resume` payload with a recorded one must leave those rows unchanged (per layout-templates §Surface: cli · Output structure — `viola revive`).

## Patterns to follow
- The `paste-newline` row's step line `[18/18] paste-newline an LF inside a paste stays inside the one prompt submitted  pass`: id, then a one-clause present-tense statement of the relied-on shape, then the verdict (per layout-templates §Surface: cli · Output structure — `viola verify`).
- The `[17/18] local-command-clear /clear starts a new session and submits no prompt` wording is the template for a `/compact` local-command row's words (per layout-templates §Surface: cli · Output structure — `viola verify`).
- The worked example block in the plan shows the full step run and the `stamped 2.1.287  18 pass  0 fail` summary; an added row moves both the `MM` in every shown step line and the shown summary counts (per layout-templates §Surface: cli · Output structure — `viola verify`).

## Anti-patterns to avoid
- No count literal in the step line or summary: a hard-coded `18` (or the new total) where `LedgerRow::ALL.len()` is the source (per layout-templates §Surface: cli · Output structure — `viola verify`).
- No new human column, banner, spinner, colour or terminator word on any verb to show a new row's result: human columns and words are a stable contract and new fields go to `--json` first (per layout-templates §Surface: cli · IA notes).
- No `[RB] read back` for a local command whose post-condition is not measured and stamped on the running CLI version (per layout-templates §Surface: cli · Component — Hero / signature output line).

## Contract bindings
- layouts ↔ tests: the step-line shape and the `stamped <ver>  <n> pass  <m> fail` summary are what a count-literal probe reads; the testing rule (Tier-2 in `.claude/rules/testing.md`) wants it matched against the full `<n> pass  <m> fail` shape, never a bare `N pass` (per layout-templates §Surface: cli · Output structure — `viola verify`).
- layouts ↔ design-system: the cli refusal pairs and the verify summary wording are shared with design-system §Surface: cli Component Patterns 2 (per layout-templates §Surface: cli · Component — Primary content block 2).
- layouts ↔ architecture: `MM` is `LedgerRow::ALL.len()`, the capability ledger's row set (architecture §Cross-cutting Patterns, Capability ledger) (per layout-templates §Surface: cli · Output structure — `viola verify`).

## Acceptance criteria contributions
- (layouts) `viola verify` prints one `[NN/MM] <row id> <row words>  pass|fail` stdout line for each row in `LedgerRow::ALL`, the new rows included, in ledger order, with `MM` equal to `LedgerRow::ALL.len()` (per layout-templates §Surface: cli · Output structure — `viola verify`).
- (layouts) A completed `viola verify` ends stdout with `stamped <ver>  <n> pass  <m> fail` where `n + m` equals `LedgerRow::ALL.len()`, plain ASCII, no SGR byte (per layout-templates §Surface: cli · Component — Footer / terminator).
- (layouts) The plan's `viola verify` ledger-order list and worked example are amended to name every row this chunk lands, in code order, with the new `MM` (per layout-templates §Surface: cli · Output structure — `viola verify`).
- (layouts) If `/compact` lands on the local-command list, `viola send` of `/compact` prints `[RB] read back` (exit 0) only on its measured post-condition on a verified version and `[  ] unconfirmable` (exit 0) otherwise, never exit 13 `no-prompt-submitted` (per layout-templates §Surface: cli · Signature placement).
