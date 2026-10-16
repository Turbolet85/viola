proposals:
  - detector: D-security-input
    severity: escalate
    section: '§Input Validation (the row "Child process output: `claude agents --json`, `--version` probe, `viola verify`''s interactive PTY runs")'
    change: 'Replace "read with `Read::take(MAX_FRAME)` and parse tolerantly; failure becomes `unknown`" for the agents output with: the `claude agents --json` child of CLI `list`, of `viola revive`''s preflight and of `viola verify`''s Run B is a direct spawn of the resolved program under the R8 strip, its stdout read through `take(MAX_FRAME + 1)` and killed at 5 s, and parsed tolerantly by `viola_agent_claude::agents::parse` (an element without a string `sessionId` skipped and counted, an extra key tolerated and counted) into the closed `AgentsFailure` (`oversize` · `malformed`, one `parse-rejected{parser:"claude-agents-json"}` line); a failed, oversize or malformed read leaves `list`''s board as `board::read` gave it and passes `revive` (fail open); an unwrapped row''s NAME is its `sessionId`, never a `ViolaName` or a valid `target`, and human-mode `list` prints it through `escape_row`.'
    sidecar: 'security-plan §Input Validation: the agents child-output row states the landed bound (`take(MAX_FRAME + 1)`, 5 s kill), the closed `AgentsFailure` and the fail-open board / revive reading (chunk 2026-10-16-unwrapped-sessions-on-the-board).'
    rationale: 'Report Changes, Symbols / APIs: `agents::parse` returns `AgentsList { rows, skipped, tolerated }` or `AgentsFailure` (`Oversize` · `Malformed`); `read_logged` writes the `parse-rejected` line; "A failed, oversize or malformed read leaves the board as `board::read` gave it"; revive "a failed read passes (fail open)"; Coverage of new surfaces: "direct spawn of the resolved program, R8 strip, `take(MAX_FRAME + 1)`, 5 s kill, tolerant parse to a closed failure", "`sessionId` never a `ViolaName`", "`escape_row` on every NAME". The row still states `take(MAX_FRAME)` and a failure that "becomes `unknown`".'
    basis: 'crates/viola-agent-claude/src/agents.rs:88-110; src/run/agents.rs:36-96'
  - detector: D-security-input
    severity: escalate
    section: '§Input Validation (the row "Child executable resolution (`run`, `revive`, `claude agents`)")'
    change: 'Retire "As landed CLI `list` resolves no `claude` and reads no `claude agents --json` (chunk 2026-10-11-the-board-viola-list: ...); it joins this rule when "Unwrapped sessions on the board" lands." and say instead: since chunk 2026-10-16-unwrapped-sessions-on-the-board CLI `list` (one read per run, after `board::read`) and `viola revive`''s preflight (its third reading, skipped under `--fork`) resolve `claude` by `run::resolve_program` from the current directory (`run::agents::read_here`; an unreadable current directory is a failed read), and `viola verify`''s Run B reads with Run B''s own program in its probe dir; the output stays untrusted under the same bounds.'
    sidecar: 'security-plan §Input Validation: child executable resolution names the landed `claude agents --json` readers (`list`, `revive`''s preflight, verify Run B) and retires the "joins this rule when it lands" clause.'
    rationale: 'Report Changes, Symbols / APIs: `read_here(home, cwd)` "resolves `claude` by `run::resolve_program` from the current directory"; `viola list`: "one `claude agents --json` read after `board::read`"; `viola revive`: the `session-live` reading in `preflight_order`, "skipped under `--fork`"; deviation 5 (an unreadable current directory is a failed read). Counts: pattern `agents-read` lists sec 212 @c106 · @c217 stale.'
    basis: '.andromeda/security-plan.md:212; src/run/agents.rs:106-116; src/cmd/revive.rs:153-158'
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: '§Threat Model Summary (Vector: Child process spawning and PATH resolution, Entry point)'
    change: 'Replace "As landed none does: CLI `list` spawns none (chunk 2026-10-11-the-board-viola-list; owed to "Unwrapped sessions on the board")." with: as landed (chunk 2026-10-16-unwrapped-sessions-on-the-board) CLI `list` spawns one per run after `board::read`, `viola revive`''s preflight one before the recorded-directory reading (skipped under `--fork`; a live id elsewhere is the closed refusal `session-live`, no child; a failed read passes), and `viola verify`''s Run B one; each spawn pair `agents-probe` is logged only under an open log, holding no row text, id, `cwd` or path.'
    sidecar: 'security-plan §Threat Model Summary: the child-spawning entry point names the three landed `claude agents --json` spawns and retires the "owed" clause.'
    rationale: 'Report Changes, Symbols / APIs (`viola list`, `viola revive` preflight chain holder → logged session → `session-live` → recorded directory, `viola verify` Run B, `read_logged` writing the `agents-probe` pair); Coverage of new surfaces: "PII no row text, id, `cwd` or path logged". Counts: patterns `agents-read` / `unwrapped-rack` list sec 110 stale.'
    basis: '.andromeda/security-plan.md:110'
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: '§Threat Model Summary (Vector: Child process spawning and PATH resolution, the `viola verify` entry point)'
    change: 'Add that Run B also makes ONE bounded `claude agents --json` read once its first turn has settled, before the framing pastes (Run B''s program, the probe dir as cwd, the R8 strip, `VERSION_DEADLINE`; the capture held in memory and never written to the captures dir; `--record` keeps only Run B''s own row in `Agents.recorded-1.json`, through the scrub; no live start added; the founder''s own answer to the P4 fork of chunk 2026-10-16-unwrapped-sessions-on-the-board, its `inputs#I1`), and change "An uninterrupted verify writes seven spawn pairs (`version-probe`, `verify-probe`, five `verify-pty-probe`)" to eight spawn pairs (`version-probe`, `verify-probe`, five `verify-pty-probe`, one `agents-probe`).'
    sidecar: 'security-plan §Threat Model Summary: verify''s child set gains Run B''s one bounded agents read; spawn pairs per uninterrupted verify 7 → 8.'
    rationale: 'Report Changes, Symbols / APIs, `viola verify`: "Run B makes ONE `claude agents --json` read once its first turn has settled, before the framing pastes (deviation 7), program = Run B''s, cwd the probe dir, R8 strip, `VERSION_DEADLINE`; capture carried in memory (not written to the captures dir); spawn pairs per uninterrupted verify 7 → 8; `--record` writes `Agents.recorded-1.json` holding Run B''s own row only, through the scrub"; Decisions: the founder''s fork 1 "join row relied with its probe in Run B (a verify-child widening ratified by him)". Counts: pattern `spawn-pairs` lists sec 114 standing/stale; `verify-kinds` notes the masters word the verify child set otherwise.'
    basis: '.andromeda/security-plan.md:114; src/cmd/verify/typed.rs:365-376'
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: '§Input Validation (the **Constants:** paragraph, `MAX_FRAME` uses)'
    change: 'Add to the child-output uses of `MAX_FRAME`: the `claude agents --json` read of `list`, `revive`''s preflight and verify''s Run B (`take(MAX_FRAME + 1)`, killed at 5 s; over the cap is the closed `oversize`).'
    sidecar: 'security-plan §Input Validation Constants: `MAX_FRAME` names the landed `claude agents --json` read.'
    rationale: 'Report Changes, Symbols / APIs: `src/run/version_gate.rs` `run_capped(…, cap)` new, the agents read through it; `AgentsFailure::Oversize`; Coverage: "`take(MAX_FRAME + 1)`, 5 s kill". The Constants paragraph lists only the `--version` reads and the five PTY runs as child output.'
    basis: 'src/run/agents.rs:36-96'
    dependent-of: D-security-input
  - detector: D-security-input
    severity: escalate
    section: '§Bootstrap phases (derive for route / setup-project) → logging-redaction-wire'
    change: 'Replace "as landed (chunk 2026-10-11-the-board-viola-list) every field comes from viola''s own state; the fields taken from `claude agents --json`, unwrapped session names included, take the same escape when those rows land" with: since chunk 2026-10-16-unwrapped-sessions-on-the-board an unwrapped row''s NAME (the row''s `sessionId`) is taken from `claude agents --json` and printed through the same `escape_row`, one line, its width counted in characters; every other field is viola''s own state or a closed word (STATUS `idle` · `busy` · `unknown`, `n/a`).'
    sidecar: 'security-plan logging-redaction-wire: the `list` escape floor states the landed unwrapped NAME from `claude agents --json`, retiring "when those rows land".'
    rationale: 'Report Changes, Symbols / APIs, `viola list`: "NAME = the row''s `sessionId` ... every NAME through `escape_row`, NAME width in characters (deviation 6)", `n/a` in WHEEL, DIALOG, CLI; STATUS idle / busy. The report''s sweep classed sec 389 no-change, but its "as landed every field comes from viola''s own state" and "when those rows land" restate the not-built claim the primary retires.'
    basis: '.andromeda/security-plan.md:389; src/human/board.rs:201-204'
    dependent-of: D-security-input
  - detector: D-security-auth
    severity: warning
    section: '§Security Anti-Patterns → Universal (the stamp-gate bullet)'
    change: 'The ledger has twenty-three rows (`LedgerRow::ALL`, the list ending `resume-session` and the agents row `agents-join`); the relied set is counter `/23` on Unix and on Windows the first eighteen, `resume-session` and `agents-join`, counter `/20`; the example home reads `19 pass  4 fail` on Unix; and a stamp holding exactly the twenty-two ids of a stamp written before chunk 2026-10-16-unwrapped-sessions-on-the-board records `cli_verified:false` on every OS.'
    sidecar: 'security-plan Anti-Patterns Universal: the stamp gate reads over 23 rows / relied 23 Unix · 20 Windows (`agents-join` relied on every OS).'
    rationale: 'Report Changes, Symbols / APIs: "`LedgerRow::AgentsJoin` appended last to `ALL` ... relied on every OS"; Counts: "`LedgerRow::ALL` 22 → 23, relied 22 / 19 → 23 / 20", pattern `relied-rows` lists sec 539 stale; Harness: `dialogless_home`''s summary `19 pass  4 fail`. The bullet is the authorization flow for non-`null` dialog decisions, which reads over the relied set.'
    basis: '.andromeda/security-plan.md:539; tests/support/home.rs:362-367'
note: 'D-security-deps clean: the report''s Dependencies bullet adds and bumps none (`fuzz/Cargo.lock` unchanged, both `cargo deny` audits green). The new input surface itself is validated as §Input Validation mandates; the six D-security-input proposals amend the plan''s stale statements of that surface (bound, failure reading, not-built clauses), so the escalate severity is the detector''s, not a found unvalidated boundary. The §Parser surfaces line and the Anti-Patterns lines 489 / 516 already hold.'
