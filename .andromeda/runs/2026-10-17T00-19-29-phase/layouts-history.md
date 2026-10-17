## Relevant amendment history
- **2026-10-10-budget-governor** — fixed the two stdout outcomes of `viola release <name> --budget`: `budget gate lifted until <end>  wheel <holder>` when a pause stood, and `budget gate open  wheel <holder>` when none did. Each is one line, exit 0, with the wheel left where it is. `<end>` is `override_until` as `HH:MM:SSZ` on the current UTC date, else `YYYY-MM-DD HH:MM:SSZ`. Path 6's two-instance leg calls `release builder --budget` and reads its result through `--json` only. The human line is the contract a non-`--json` assertion would have to match, so the leg must not invent another form. The date form and the `open` line were planning forms when they landed (see 2026-10-13-t06-38-35-wrap). Ref: `.andromeda/runs/2026-10-10T22-36-31-wrap/`.
- **2026-10-11-the-board-viola-list** — set down `viola list` as built:
  - the uncoloured form on every stream, a TTY included, written as one stdout write;
  - the widths: NAME is the longer of 4 and the longest name, LIVE 5, STATUS 7, WHEEL 23, DIALOG 17, CLI unpadded. No trailing space and nothing truncated;
  - the `unknown` WHEEL / CLI cells;
  - the `list --json` refusal document: exit 21, empty stderr, no log opened, the home's mode left as found;
  - `list` added to the catch-site verbs (an unlistable `instances/` exits 1).

  The board's budget cell (CARRY 3) is asserted against these as-built characters. `status`'s `down` / `list-exit-21` reading derives from that exit-21 document. The styled TTY form stays unbuilt (owed to "CLI output discipline"), which is why the extract's anti-pattern forbids asserting it. Ref: `.andromeda/runs/2026-10-11T08-23-35-wrap/`.
- **2026-10-13-t06-38-35-wrap** — the founder confirmed several items, so they are his and no longer the operator's word:
  - WHEEL 23 and the design-system cli pattern 1 planning forms. These cover the `<holder> - budget-paused` WHEEL cell the Path 6 leg asserts on `overseer`;
  - the `list --json` exit-21 refusal with no log opened;
  - the human refusal pairs;
  - `release --budget`'s date form and its `budget gate open` line.

  The failed log start and the per-instance no-row read error stay implement's decisions, which the operator accepted and the founder has not ruled on. A leg should not pin those two shapes as if he had ratified them. Ref: `.andromeda/runs/2026-10-13T06-38-35-wrap/`.
- **2026-10-16T14-58-10-wrap** — cut Surface: web-spa to a stub. The cli surface now stands alone: the BAY line, the board, streams, footer and IA notes are cli-only. `list --json`'s item, `budget` and `skipped` shapes became the architecture §Standard Contracts `list` payload contract (no longer "identical to `/api/sessions`"). That contract is the one `status` nests verbatim as its `list` member, and that `.ok.budget.paused` and each item's `budget_paused` are read from. The amendment also made the six captions, words and slot order fixed, which backs the extract's rule that no leg adds or renames a human column or word. Ref: `.andromeda/runs/2026-10-16T14-58-10-wrap/`.
- **2026-10-16-unwrapped-sessions-on-the-board** — `viola list` gained one `claude agents --json` read after the board read:
  - STATUS reads `idle` / `busy` on a joined wrapped row; it was always `unknown`;
  - unjoined rows print under `-- UNWRAPPED - READ-ONLY --`: NAME is the `sessionId` through `escape_row`, LIVE `live`, and `n/a` in WHEEL / DIALOG / CLI, never truncated;
  - no separator prints without an unwrapped row;
  - a failed, oversize or malformed read prints the board as if no `claude` were present.

  This is the mechanism `boot --agents-mode recorded|oversize|malformed` (CARRY 6) switches between, and the reason a booted instance's STATUS under the harness `PATH` depends on the fake agent's join. The amendment kept the design-target truncation of unwrapped NAMEs, which is not built and stays owed to "CLI output discipline". Ref: `.andromeda/runs/2026-10-16T22-30-02-wrap/`.
- **2026-10-10-viola-revive** — (the reuse of `run`'s collision pair) `viola revive`'s refusals included `run`'s collision pair unchanged. That pair is Path 1 step 3's duplicate-`run` refusal: exit 1, one `unable:` line then one `hint:` line last on stderr, stdout empty, no paths or pids. The pair is shared between the two verbs, so the Path 1 assertion pins wording that `revive` also prints. The amendment also recorded that `--json` covers every data verb except `revive`. Ref: `.andromeda/runs/2026-10-10T15-07-22-wrap/`.
- **2026-10-09-epoch-3-cleanup** — set the precedent for a `send` refusal's output forms: the struck `[/ ] unable` line on stderr with its `hint:` as the last stderr line, and exactly one document under `--json`. Path 6's `send` to `overseer` (exit 11 `budget-paused`) is read by the same Primary content block 2 rule: one stdout document with empty stderr under `--json`, otherwise the hint line last. Ref: `.andromeda/runs/2026-10-09T17-10-00-wrap/`.
