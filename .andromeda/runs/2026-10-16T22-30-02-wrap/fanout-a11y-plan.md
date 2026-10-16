proposals:
  - detector: D-a11y-surface
    severity: warning
    section: "§1 A11y Scope Summary → A11y surfaces & assistive tech reach → cli → ARIA roles inventory"
    change: "Retire the 'as built (chunk 2026-10-11-the-board-viola-list) ... the separator and its unwrapped rows are owed to Unwrapped sessions on the board' clause: as built (chunk 2026-10-16-unwrapped-sessions-on-the-board) the board is the BAY line, the caption row and the wrapped rows, then, only when the one `claude agents --json` read lists a session no wrapped row joins, the `-- UNWRAPPED - READ-ONLY --` separator and the unwrapped read-only rows (NAME = the row's `sessionId` through `escape_row`, `n/a` in WHEEL, DIALOG and CLI); no separator without unwrapped items, and a failed, oversize or malformed read leaves the board as before."
    sidecar: "2026-10-16-unwrapped-sessions-on-the-board: §1 cli ARIA roles inventory — the separator and the unwrapped rows are built; the 'owed' clause retired."
    rationale: "Report Changes → Symbols / APIs (`viola list`: unjoined rows become unwrapped items, the human board prints `-- UNWRAPPED - READ-ONLY --` then the rows, `n/a` in WHEEL, DIALOG, CLI, every NAME through `escape_row`); Counts → pattern `unwrapped-rack` names a11y 68 stale; Outcome (layouts) 'no separator without unwrapped items' MET; Expected amendments name a11y-plan §1."
    basis: ".andromeda/a11y-plan.md:68"
  - detector: D-a11y-surface
    severity: warning
    section: "§1 A11y Scope Summary → Critical paths → 6. CLI board, wait/last and verify output → Required ARIA roles"
    change: "The caption line and the `-- UNWRAPPED - READ-ONLY --` separator now both stand (the separator built at chunk 2026-10-16-unwrapped-sessions-on-the-board, printed only when unwrapped rows exist); only the `--help` group labels stay owed to 'CLI output discipline'."
    sidecar: "2026-10-16-unwrapped-sessions-on-the-board: §1 Critical path 6 — the separator is built; only the --help group labels remain owed."
    rationale: "Same retired claim (separator owed to 'Unwrapped sessions on the board') restated in the P6 critical path; report pattern `unwrapped-rack` names a11y 134 stale; Symbols / APIs `src/human/board.rs` the separator and the unwrapped rows."
    basis: ".andromeda/a11y-plan.md:134"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§4 ARIA Patterns & Roles → Per-surface landmark roles inventory (cli row)"
    change: "As built: BAY line → caption line → wrapped rows → (only when unwrapped items exist) the `-- UNWRAPPED - READ-ONLY --` separator and the unwrapped rows (chunk 2026-10-16-unwrapped-sessions-on-the-board); the `--help` group labels ('CLI output discipline'; the built help is clap's ordered list) are still not built."
    sidecar: "2026-10-16-unwrapped-sessions-on-the-board: §4 cli landmarks row — the separator is built; the --help group labels stay owed."
    rationale: "Same retired claim in the landmarks table; report pattern `unwrapped-rack` names a11y 209 stale; Coverage of new surfaces → `viola list` unwrapped rack: a11y the separator line, no SGR, one write ✓."
    basis: ".andromeda/a11y-plan.md:209"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§4 ARIA Patterns & Roles → Must-be-accessible path scenarios → P6 → C0/C1 escaped → Setup"
    change: "Retire 'As built ... `viola list` reads no `claude agents --json` ... the fixture-row setup waits on Unwrapped sessions on the board': `viola list` now makes one `claude agents --json` read and prints every unwrapped NAME (the row's `sessionId`, never a `ViolaName`) through `escape_row` as one line; the escape is held by the `board_row_*` units and by `tests/cross_list_rows.rs` in three modes (piped, `NO_COLOR` / `TERM=dumb`, the outer PTY; the no-ESC assertion Unix-only, on windows-2025 each escaped name read as text under a 200-column terminal because ConPTY re-renders escapes), beside the existing CLI-version cases."
    sidecar: "2026-10-16-unwrapped-sessions-on-the-board: §4 P6 C0/C1 setup — `viola list` reads `claude agents --json`; unwrapped names escaped, held by cross_list_rows in three modes."
    rationale: "Report Counts → patterns `unwrapped-rack` and `agents-read` both name a11y 236 stale; Symbols / APIs (every NAME through `escape_row`); Coverage of new surfaces (`escape_row` on every NAME, `sessionId` never a `ViolaName`; integ `cross_list_rows` piped · `NO_COLOR` / `TERM=dumb` · outer PTY); Deviations 10 (the Windows terminal case, no-ESC Unix-only)."
    basis: ".andromeda/a11y-plan.md:236"
    dependent-of: D-a11y-surface
  - detector: D-a11y-surface
    severity: warning
    section: "§6 Visual Design Verification → CLI equivalent"
    change: "Add to the as-built clause: the STATUS words `idle` / `busy` on joined wrapped rows and the unwrapped rack's `n/a` cells are words too, with no SGR, no truncation and no colour (chunk 2026-10-16-unwrapped-sessions-on-the-board); `viola list` still emits no SGR."
    sidecar: "2026-10-16-unwrapped-sessions-on-the-board: §6 CLI equivalent — STATUS idle/busy and the unwrapped rack's n/a are printed words, no SGR."
    rationale: "New visible elements this chunk (report Symbols / APIs: joined wrapped rows read STATUS `idle` / `busy`; unwrapped rows `n/a` in WHEEL, DIALOG, CLI); Decisions & corrections: the operator's P5 'yes' covers 'no truncation or colour'; Outcome (a11y) both racks, no SGR / CR / cursor sequence MET. §6's state-word list names only `stale`, `DIALOG <kind>`, `expired`, `budget-paused`, `unverified-cli`."
    basis: ".andromeda/a11y-plan.md"
  - detector: D-a11y-surface
    severity: warning
    section: "§8 Cognitive Accessibility → Timeout extensions"
    change: "Add the bound: `viola list` and `viola revive` each make one `claude agents --json` read (direct spawn, `take(MAX_FRAME + 1)`, a 5 s kill); a failed, oversize, malformed or timed-out read leaves `list`'s board as `board::read` gave it and passes `revive`'s preflight (fail open). The bound falls on viola's own child only, never on a keystroke or the human, and no key is blocked, refused or delayed by it."
    sidecar: "2026-10-16-unwrapped-sessions-on-the-board: §8 Timeout extensions — the 5 s `claude agents --json` read of list and revive, fail-open, on no keystroke."
    rationale: "Report Counts → pattern `timeout-ext`: a11y 281 standing, stale ('the section lacks the bound'); Expected amendments: a11y-plan §8 'Coverage (the 5 s read on no keystroke)'; Coverage of new surfaces (the agents child: 5 s kill, tolerant parse to a closed failure); Symbols / APIs (`list`: a failed read leaves the board; `revive`: a failed read passes)."
    basis: ".andromeda/a11y-plan.md:281"
note: "D-a11y-obs-schema holds: the only obs schema change is `diag-line.v1.json` `process-exit.detail` gaining `session-live`; the a11y violation schema emits no a11y rows (failures are nextest failures in `suites[].failures[]`), so the two stay consistent. The §8 Timeout extensions proposal sits outside D-a11y-surface's §5–§7 scope but is the report's own expected §8 amendment; the orchestrator may re-home it."
