proposals:
  - detector: D-layout-surface
    severity: warning
    section: Output structure — `viola list` (piped / `NO_COLOR` / `TERM=dumb`)
    change: 'In the **As built** paragraph, replace "the `-- UNWRAPPED - READ-ONLY --` rack and its rows are not built, and STATUS is always `unknown`, owed to "Unwrapped sessions on the board"" with the built form (chunk 2026-10-16-unwrapped-sessions-on-the-board): one `claude agents --json` read after the board read, its rows joined to wrapped rows by child pid or agent session id; a joined wrapped row reads STATUS `idle` / `busy`; every unjoined row prints after the wrapped rows under the `-- UNWRAPPED - READ-ONLY --` line, sorted by name, NAME = the row''s `sessionId` through `escape_row`, LIVE `live`, `n/a` in WHEEL, DIALOG and CLI, never truncated (the wireframe''s `c1f9e2a4...` clipping is not built); no separator line without an unwrapped row; an empty wrapped rack with unwrapped rows keeps the empty-board line; a failed, oversize or malformed read prints the board as with no `claude`. Also: "then one row per wrapped session in name order" gains "then the unwrapped rack", and NAME''s width is counted in characters, not bytes.'
    sidecar: 'layout-templates `viola list` (piped) As built: the unwrapped rack, its separator and STATUS idle/busy built by chunk 2026-10-16-unwrapped-sessions-on-the-board (no truncation, n/a cells, NAME = sessionId escaped, width in characters); the "not built / always unknown" clause retired.'
    rationale: 'Report Changes, Symbols / APIs (`viola list`: one agents read, join by `child_pid` OR `agent_session_id`, STATUS `idle` / `busy`, unwrapped items sorted after the wrapped, human board prints `-- UNWRAPPED - READ-ONLY --`, `n/a` in WHEEL, DIALOG, CLI, NAME through `escape_row`, NAME width in characters (deviation 6), a failed/oversize/malformed read leaves the board as `board::read` gave it); Counts pattern `unwrapped-rack` lists layouts 91 stale; Decisions (P5 card: NAME = `sessionId`, the empty-rack form, no truncation or colour); Outcome (layouts) "no separator without unwrapped items".'
    basis: '.andromeda/layout-templates.md:91; src/human/board.rs:201-204; src/human/board.rs:581-599; src/human/board.rs:601-610; src/cmd/list.rs:105-119'
  - detector: D-layout-surface
    severity: warning
    section: Component — Primary content block 1 `viola list` strip rows
    change: 'In the **As built** bullet, replace "wrapped rows only, no styling, no separator, no truncation" with "wrapped rows, then (chunk 2026-10-16-unwrapped-sessions-on-the-board) the `-- UNWRAPPED - READ-ONLY --` separator and the unwrapped rows when the agents read lists any; no styling, no truncation", and "its one upstream-derived cell is the recorded CLI version" with "its upstream-derived cells are the recorded CLI version and each unwrapped NAME (a `claude agents --json` `sessionId`), both through `escape_row`".'
    sidecar: 'layout-templates Primary content block 1 As built: the separator and unwrapped rows now built; the unwrapped NAME joins the recorded CLI version as an upstream-derived, escaped cell.'
    rationale: 'Same retired claim (the unwrapped rack not built) restated in the block-1 As built bullet; report Symbols / APIs (`viola list`: unwrapped rows, NAME = `sessionId`, every NAME through `escape_row`) and Coverage (`sessionId` never a `ViolaName`, escaped).'
    basis: 'src/human/board.rs:201-204; src/human/board.rs:554-579'
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: Output structure — `viola revive`
    change: 'Add the new refusal pair to the exit-1 list, between the "no logged session with that id" pair and the recorded-directory pair: `unable: <name>''s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`; and state that the start reads `claude agents --json` once after the logged session is chosen (preflight order: holder, logged session, session live, recorded directory), that this reading is skipped under `--fork`, and that a failed read passes (the start goes on).'
    sidecar: 'layout-templates `viola revive`: the `session-live` refusal pair (open in another claude / start a copy with --fork) added, chunk 2026-10-16-unwrapped-sessions-on-the-board; skipped under --fork, fail-open on a failed read.'
    rationale: 'Report Changes, Symbols / APIs (`viola revive`: preflight chain holder → logged session → `session-live` → recorded directory, four readings; the pair `unable: <name>''s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`; no child; a failed read passes) and `Refusal::SessionLive` wired into `preflight_order`, skipped under `--fork`; Outcome (design) "the `session-live` pair, exit 1, empty stdout, no child; `--fork` and a failed read pass". The section lists every revive refusal pair and lacks this one.'
    basis: 'src/cmd/revive.rs:53; src/cmd/revive.rs:153-158; tests/cli_revive.rs:43-45'
  - detector: D-layout-surface
    severity: warning
    section: Component — Primary content block 2 refusal lines and the `unable` column
    change: 'In the Line form bullet, "the exit-1 refusals of `viola revive` (the same collision pair, and its own four pairs under Output structure — `viola revive`)" now reads "its own five pairs", the fifth the `session-live` pair.'
    sidecar: 'layout-templates Primary content block 2: viola revive''s own refusal pairs four → five (session-live).'
    rationale: 'The revive refusal count restated in block 2 retires with the new pair (report Symbols / APIs, `viola revive`); the report''s `revive-pairs` sweep notes the count is worded by number nowhere its pattern reached (five|six), so this "four" site is outside its listing.'
    basis: 'src/cmd/revive.rs:53'
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: Output structure — `viola verify`
    change: 'The wireframe reads "(a Unix host: 23 relied rows)", every step counter `[NN/23]`, gains the last step line `[23/23] agents-join claude agents --json names the live session by pid and session id  pass` after `[22/23] resume-session …`, the summary `stamped 2.1.287  23 pass  0 fail`, and the failing example `stamped 2.1.287  21 pass  2 fail`.'
    sidecar: 'layout-templates `viola verify` wireframe: relied rows 22 → 23 on Unix, the `agents-join` step line added last, summaries re-counted (chunk 2026-10-16-unwrapped-sessions-on-the-board).'
    rationale: 'Report Symbols / APIs (`LedgerRow::AgentsJoin` appended last to `ALL`, id `agents-join`, words `claude agents --json names the live session by pid and session id`, relied on every OS); Counts (`LedgerRow::ALL` 22 → 23, relied 22 / 19 → 23 / 20; pattern `relied-rows` lists layouts 170 · 199 stale); Outcome "`ALL` = 23, last `agents-join`, `[NN/23]` / `[NN/20]`, `stamped 2.1.287  23 pass  0 fail`".'
    basis: '.andromeda/layout-templates.md:170; crates/viola-agent-claude/src/ledger.rs:132'
  - detector: D-layout-surface
    severity: warning
    section: Output structure — `viola verify`
    change: 'In the Usage paragraph, the ledger-order row list ends `… · resume-session · agents-join`; `MM` is "23 on Unix, 20 on Windows, where the relied set skips the three statusline rows because the status line is not wrapped there and ends at `agents-join`".'
    sidecar: 'layout-templates `viola verify` Usage: row list gains `agents-join`; MM 22/19 → 23/20; the relied set ends at `agents-join` on both OSes.'
    rationale: 'The relied count and the row list ending `resume-session` are restated in the Usage paragraph (report Counts, pattern `relied-rows`, layouts 199 stale: "each states twenty-two / 22 / 19 / nineteen … or the row list ending `resume-session`").'
    basis: '.andromeda/layout-templates.md:199; crates/viola-agent-claude/src/ledger.rs:132'
    dependent-of: D-layout-surface
note: 'layouts 70 · 262 · 283 · 284 · 285 read no-change per the report''s sweep; 284''s design-target truncation stays as written, the built "never truncated" form carried by the As built amendment only. The unwrapped-name exit-21 hint (design-system cli pattern 2) is not built (owned by "Exit-cause code catalogue"), so no hint line is proposed.'
