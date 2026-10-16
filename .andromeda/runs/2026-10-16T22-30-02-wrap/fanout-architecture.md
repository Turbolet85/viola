proposals:
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [API Style]
    change: 'Drop the parenthesis "(as landed it reads the disk only: the `claude agents --json` read is owed to \"Unwrapped sessions on the board\")"; say instead that as landed (chunk 2026-10-16-unwrapped-sessions-on-the-board) CLI `list` reads the disk (`board::read`) and then makes one `claude agents --json` read, joined into the board, still not a channel request.'
    sidecar: 'arch [API Style] (line 50): `list` now reads the disk and one `claude agents --json`; the owed-read parenthesis retired (chunk 2026-10-16-unwrapped-sessions-on-the-board).'
    rationale: 'Report Changes, Symbols / APIs: "`viola list`: one `claude agents --json` read after `board::read`, joined by `child_pid` OR `agent_session_id`"; Counts, pattern `agents-read` lists arch 50 as stale.'
    basis: 'src/cmd/list.rs:105-119; .andromeda/architecture.md:50'
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (`list` is not a channel method)
    change: 'Replace "it reads, spawns and parses no `claude agents --json`, so every item is `wrapped: true` with `status: \"unknown\"` and no unwrapped row is listed. The `claude agents --json` read, the unwrapped rows and idle/busy STATUS are owed to … \"Owed ledger rows\")" with the landed shape: after `board::read` CLI `list` makes one `claude agents --json` read (`run::agents::read_here`, `claude` resolved by `run::resolve_program` from the current directory) and joins it by `viola_state::board::join`: a row matching a wrapped item''s `child_pid` OR `agent_session_id` sets that item''s `status` `idle` / `busy`; every unjoined row is an unwrapped item (`wrapped:false`, name = its `sessionId`, `live`, `dialog_pending:false`, no `wheel` / `budget_paused` / `cli_version` / `cli_verified`), sorted by name after the wrapped items; a failed, oversize or malformed read leaves the board as `board::read` gave it; the join field is ledger row `agents-join` (chunk 2026-10-16-unwrapped-sessions-on-the-board).'
    sidecar: 'arch §Standard Contracts `list` (line 237): the `claude agents --json` read, the join, STATUS idle/busy and the unwrapped rows landed; the owed clause retired.'
    rationale: 'Report Changes, Symbols / APIs (`viola list` bullet; `viola_state::board` `join`, `Item::unwrapped`); Counts, patterns `agents-read`, `unwrapped-rack`, `agents-join` list arch 237 as stale.'
    basis: 'crates/viola-state/src/board.rs:323-345; crates/viola-state/src/board.rs:186-202; .andromeda/architecture.md:237'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (Session liveness)
    change: 'Replace "Its readers (`list` and `mcp`, through `viola-agent-claude`) spawn it directly (none does yet: … owed to \"Unwrapped sessions on the board\"), resolving `claude` from their own PATH" with: as landed it is spawned directly by CLI `list`, `viola revive`''s preflight and `viola verify`''s Run B through the root bin''s `run::agents` (`read_logged`: the R8 strip, `take(MAX_FRAME + 1)` through `version_gate::run_capped`, killed at 5 s, one `agents-probe` spawn pair under an open log), parsed by the pure `viola_agent_claude::agents::parse` into a closed `AgentsFailure` (`oversize` · `malformed`); `list` and `revive` resolve `claude` by `run::resolve_program` from their current directory; an oversize read joins the failure outcome.'
    sidecar: 'arch §Standard Contracts Session liveness (line 280): the `claude agents --json` readers landed (`list`, `revive`, verify Run B via `run::agents`); the none-yet clause retired.'
    rationale: 'Report Changes, Symbols / APIs: root bin `src/run/agents.rs` `read_logged` / `read_here`, `run_capped`; Coverage of new surfaces (direct spawn, R8 strip, `take(MAX_FRAME + 1)`, 5 s kill); pattern `agents-read`, `status-words` list arch 280 stale.'
    basis: 'src/run/agents.rs:36-96; src/run/agents.rs:106-116; crates/viola-agent-claude/src/agents.rs:88-110'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [CLI Version Compatibility] (open items list)
    change: 'Retire the open item "`claude agents --json` join field … owed, with the read itself, to \"Unwrapped sessions on the board\""; state it answered: as measured on 2.1.287 a row exposes both the child pid (`pid`) and the session id (`sessionId`); ledger row `agents-join` (`claude agents --json names the live session by pid and session id`), relied on every OS, probed by Run B''s one read and stamped live on 2.1.287 on the Linux dev host (`stamped 2.1.287  23 pass  0 fail`), its Windows reading owed to "Windows-only live measurements" (chunk 2026-10-16-unwrapped-sessions-on-the-board).'
    sidecar: 'arch [CLI Version Compatibility] (line 83): the join-field open item answered, ledger row `agents-join` landed.'
    rationale: 'Report Spec claims disproved: the "open item" measured exposed on 2.1.287 (`pid`, `sessionId`; record round `agents-join` pass); Symbols / APIs `LedgerRow::AgentsJoin`; Cross-project claims `stamped 2.1.287  23 pass  0 fail`; inputs#I2 places the Windows reading on "Windows-only live measurements".'
    basis: 'crates/viola-agent-claude/src/ledger.rs:132; .andromeda/architecture.md:83'
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (Session liveness)
    change: 'Replace "Which of the two fields `claude agents --json` exposes for this join is a capability-ledger row (open item)." with: both are exposed (`pid`, `sessionId`), and the join is ledger row `agents-join`, relied on every OS.'
    sidecar: 'arch §Standard Contracts Session liveness (line 280): the join-field open item answered by row `agents-join`.'
    rationale: 'Report Spec claims disproved (arch 83 open item answered); pattern `agents-join` and `agents-read` list arch 280 stale.'
    basis: '.andromeda/architecture.md:280'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (`list` payload)
    change: 'In the `wrapped: false` bullet replace "(part of the join-field ledger row)" with "(its `sessionId`; the join is ledger row `agents-join`)".'
    sidecar: 'arch §Standard Contracts `list` payload (line 177): the join-field reference names the row id `agents-join`.'
    rationale: 'Report Counts, pattern `agents-join`: "1 re-word (arch 177 \"part of the join-field ledger row\" → the row''s id)".'
    basis: '.andromeda/architecture.md:177'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [CLI Version Compatibility]
    change: 'The ledger now holds twenty-three rows: `LedgerRow::ALL` gains `agents-join` appended last (after `resume-session`); widened by chunk 2026-10-16-unwrapped-sessions-on-the-board added to the chunk list.'
    sidecar: 'arch [CLI Version Compatibility] (line 88): `LedgerRow::ALL` 22 → 23, `agents-join` last.'
    rationale: 'Report Symbols / APIs: "`LedgerRow::AgentsJoin` appended last to `ALL`", id `agents-join`; Counts, pattern `relied-rows`: arch 88 @c1098, @c1131 stale.'
    basis: 'crates/viola-agent-claude/src/ledger.rs:44; crates/viola-agent-claude/src/ledger.rs:71; crates/viola-agent-claude/src/ledger.rs:99'
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [CLI Version Compatibility]
    change: 'The relied set: a Unix host relies on all twenty-three rows and a Windows host on twenty, the first eighteen, `resume-session` and `agents-join`, the last two relied on every OS.'
    sidecar: 'arch [CLI Version Compatibility] (line 88): relied set 22 / 19 → 23 / 20.'
    rationale: 'Report Counts: "`LedgerRow::ALL` 22 → 23, relied 22 / 19 → 23 / 20 (`LedgerRow::relied()`)"; arch 88 @c1852, @c1890 stale; `agents-join` relied on every OS.'
    basis: '.andromeda/architecture.md:88'
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [CLI Version Compatibility]
    change: 'In the verify step-line sentence: `MM` is 23 on Unix, the last `[23/23] agents-join …`; 20 on Windows, the last `[20/20] agents-join …`.'
    sidecar: 'arch [CLI Version Compatibility] (line 88): verify step lines `[22/22]` / `[19/19]` → `[23/23]` / `[20/20]`, last row `agents-join`.'
    rationale: 'Report Outcome: "`ALL` = 23, last `agents-join`, `[NN/23]` / `[NN/20]`"; Counts: arch 88 @c12374, @c12440 stale.'
    basis: '.andromeda/architecture.md:88'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [CLI Version Compatibility]
    change: '`check(row, &Probes)` reads `Probes` with Run B''s agents capture (`agents`, a `pub struct AgentsCapture`) beside the other captures, and the `agents-join` row passes only when that one read parses and names Run B''s live session by its child pid and the session id of Run B''s first SessionStart capture (`agents_joined`); with no capture it fails.'
    sidecar: 'arch [CLI Version Compatibility] (line 88): `Probes` carries the agents capture; the `agents-join` check rule added.'
    rationale: 'Report Symbols / APIs: "`pub struct AgentsCapture` (640-646); the check `agents_joined` (881-909, wired at `:997`)"; Coverage: `check_agents_join_*` unit cases.'
    basis: 'crates/viola-agent-claude/src/ledger.rs:640-646; crates/viola-agent-claude/src/ledger.rs:881-909; src/cmd/verify.rs:548-553'
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [CLI Version Compatibility]
    change: 'In the Run B description add: once its first turn has settled, before the framing pastes, Run B makes ONE `claude agents --json` read (`typed::agents_read`: Run B''s program, cwd the probe dir, the R8 strip, `VERSION_DEADLINE`, none after the stop), its capture carried in memory and not written to the captures dir; no live start is added (the founder''s own answer to P4 fork 1 of chunk 2026-10-16-unwrapped-sessions-on-the-board, inputs#I1); in CI the fake agent''s `agents` mode answers it.'
    sidecar: 'arch [CLI Version Compatibility] (line 88): verify Run B gains one bounded `claude agents --json` read for the `agents-join` row.'
    rationale: 'Report Symbols / APIs, `viola verify` bullet (deviation 7); Decisions & corrections: the founder''s fork 1 (join row relied with its probe in Run B, a verify-child widening ratified by him).'
    basis: 'src/cmd/verify/typed.rs:365-376'
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [CLI Version Compatibility]
    change: 'In the `--record` sentence add: Run B''s agents read as `Agents.recorded-1.json`, holding Run B''s own row only (`ledger::agents_variant`), through the scrub.'
    sidecar: 'arch [CLI Version Compatibility] (line 88): `--record` writes `Agents.recorded-1.json`.'
    rationale: 'Report Symbols / APIs: "`pub fn agents_variant` (914-927, `--record`''s filter to Run B''s own row)"; `--record` writes `Agents.recorded-1.json` holding Run B''s own row only, through the scrub.'
    basis: 'crates/viola-agent-claude/src/ledger.rs:914-927'
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [CLI Version Compatibility]
    change: 'The older-stamp clause: a stamp holding exactly the twenty-two ids every stamp written before chunk 2026-10-16-unwrapped-sessions-on-the-board holds (or any earlier set), all `pass`, reads unverified on every OS, and one holding the eighteen and `resume-session` no longer reads verified on Windows (only the eighteen, `resume-session` and `agents-join` do); so every home stamped before that chunk reads `cli_verified:false` until its next `viola verify`.'
    sidecar: 'arch [CLI Version Compatibility] (line 88): pre-chunk 22-id / 19-id stamps read unverified.'
    rationale: 'Follows from the report''s relied 22 / 19 → 23 / 20 (Counts) and `agents-join` relied on every OS (Symbols / APIs): a version is verified only when every relied row reads `pass`.'
    basis: '.andromeda/architecture.md:88'
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [CLI Version Compatibility]
    change: 'Replace "adding one needs a 23rd row" with "adding one needs a 24th row".'
    sidecar: 'arch [CLI Version Compatibility] (line 88): the next row is the 24th.'
    rationale: 'Report Counts: `LedgerRow::ALL` 22 → 23, so a 23rd row now exists (`agents-join`).'
    basis: '.andromeda/architecture.md:88'
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [CLI Version Compatibility]
    change: 'Replace "The `agents --json` join and plugin precedence land with Epochs 4 and 5." with: the `agents --json` join landed at chunk 2026-10-16-unwrapped-sessions-on-the-board (row `agents-join`); plugin precedence stays owed.'
    sidecar: 'arch [CLI Version Compatibility] (line 88): the agents join landed; plugin precedence still owed.'
    rationale: 'Report Symbols / APIs `LedgerRow::AgentsJoin`; Spec claims disproved (the join open item answered).'
    basis: '.andromeda/architecture.md:88'
  - detector: D-arch-decisions
    severity: warning
    section: §Established Decisions [Session Liveness]
    change: 'Replace "as the first of its three preflight readings (the name''s holder, the logged session, the recorded directory; …)" with four readings: the name''s holder, the logged session, `session-live` (one `claude agents --json` read; a row naming the logged session id refuses, skipped under `--fork`, a failed read passes), the recorded directory; a later reading is not taken once an earlier one refuses.'
    sidecar: 'arch [Session Liveness] (line 90): revive''s preflight readings 3 → 4 (`session-live`).'
    rationale: 'Report Symbols / APIs, `viola revive`: "preflight chain holder → logged session → `session-live` → recorded directory (four readings; was three)"; `Refusal::SessionLive`, `live_elsewhere`, skipped under `--fork`; fail open. The pattern sweep missed this site (worded "first of its three").'
    basis: 'src/cmd/revive.rs:53; src/cmd/revive.rs:153-158; src/cmd/revive.rs:180; .andromeda/architecture.md:90'
  - detector: D-arch-decisions
    severity: warning
    section: §Conventions (exit code `1`)
    change: 'Revive''s preflight has four readings (the name''s holder, the logged session, the logged session open in another `claude`, the recorded directory), and its refusals gain `detail:"session-live"` (`unable: <name>''s session is open in another claude` / `hint: close it there first, or start a copy: viola revive <name> --fork`), no child, one `process-exit{subject:"self", exit_code:1, detail:"session-live"}` in `run-<name>.ndjson`.'
    sidecar: 'arch §Conventions exit 1: revive''s preflight 3 → 4 readings; the `session-live` pair added.'
    rationale: 'Report Symbols / APIs, `viola revive` bullet (the pair and the process-exit verbatim); Schema / config: `process-exit.detail` gains `session-live`. The pattern sweep missed this site ("whose three readings").'
    basis: 'src/cmd/revive.rs:53; schemas/diag-line.v1.json:78'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (Snapshot envelope)
    change: 'Replace "second of three since chunk 2026-10-10-budget-governor moved the strict-modes check ahead of the preflight" with "second of four since chunk 2026-10-16-unwrapped-sessions-on-the-board added `session-live` (second of three since chunk 2026-10-10-budget-governor …)".'
    sidecar: 'arch §Standard Contracts Snapshot envelope (line 193): revive''s readings "second of three" → "second of four".'
    rationale: 'Report Counts, pattern `revive-pairs`: "1 stale (arch 193 @c801 \"second of three\")".'
    basis: '.andromeda/architecture.md:193'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: 'In the `cmd/` comment replace "revive.rs: … the three-reading preflight" with "the four-reading preflight (its third reading the `session-live` agents read)".'
    sidecar: 'arch key Project directory structure: revive.rs''s preflight three → four readings.'
    rationale: 'Report Symbols / APIs, `viola revive` (four readings; was three); the sweep''s count words did not reach "three-reading".'
    basis: '.andromeda/registries/contracts/architecture/project-directory-structure.md'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (Ledger stamps envelope)
    change: 'Relied set: all twenty-three on Unix; on Windows twenty, the first eighteen, `resume-session` and `agents-join`.'
    sidecar: 'arch §Standard Contracts Ledger stamps envelope (line 199): relied 22 / 19 → 23 / 20.'
    rationale: 'Report Counts, pattern `relied-rows`: arch 199 standing, stale.'
    basis: '.andromeda/architecture.md:199'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (Ledger stamps envelope)
    change: 'The older-stamp clause adds the twenty-two pre-2026-10-16-unwrapped-sessions-on-the-board ids (reads unverified on every OS) and says one of the eighteen and `resume-session` now reads unverified on Windows too; one of the eighteen, `resume-session` and `agents-join` reads verified on Windows only.'
    sidecar: 'arch §Standard Contracts Ledger stamps envelope (line 199): pre-chunk 22-id and 19-id stamps read unverified.'
    rationale: 'Follows from the report''s relied 23 / 20 and `agents-join` relied on every OS (Counts; Symbols / APIs).'
    basis: '.andromeda/architecture.md:199'
    dependent-of: D-arch-decisions
  - detector: D-arch-decisions
    severity: warning
    section: §Cross-cutting Patterns (Capability ledger as the single gate)
    change: 'Relied set `LedgerRow::relied()`: 23 rows on Unix, 20 on Windows; and add that `viola list`''s and `viola revive`''s join on `claude agents --json` rests on row `agents-join`, probed by Run B''s one read, relied on every OS, stamped live on 2.1.287 on the Linux dev host (`stamped 2.1.287  23 pass  0 fail`), its Windows reading owed to "Windows-only live measurements".'
    sidecar: 'arch §Cross-cutting Patterns Capability ledger (line 405): relied 22 / 19 → 23 / 20; the `agents-join` row named.'
    rationale: 'Report Counts, pattern `relied-rows`: arch 405 @c3957, @c3968, @c3985 stale; Symbols / APIs `LedgerRow::AgentsJoin`; Cross-project claims (round-204738Z.txt).'
    basis: '.andromeda/architecture.md:405'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Crate dependency direction
    change: 'In the `viola-agent-claude` bullet, "the twenty-two rows (`LedgerRow::ALL`)" → "the twenty-three rows (`LedgerRow::ALL`)".'
    sidecar: 'arch key Crate dependency direction: `LedgerRow::ALL` 22 → 23.'
    rationale: 'Report Counts, pattern `relied-rows`: registry `architecture/crate-dependency-direction.md:6` @c1380, @c1398 stale.'
    basis: '.andromeda/registries/contracts/architecture/crate-dependency-direction.md:6'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Crate dependency direction
    change: 'In the `viola-agent-claude` bullet register the new pure `agents` module (`KEYS`, `AgentStatus`, `AgentsRow` / `joins`, `AgentsList`, `AgentsFailure`, `parse`; no I/O, re-exported from `lib.rs`) and the ledger''s `AgentsCapture` / `agents_variant`; no dependency added.'
    sidecar: 'arch key Crate dependency direction: `viola-agent-claude` gains the pure `agents` module and the ledger''s agents capture.'
    rationale: 'Report Symbols / APIs: `viola_agent_claude::agents` (new, pure, re-exported `lib.rs:4`); `AgentsCapture`, `agents_variant`; Dependencies: none added.'
    basis: 'crates/viola-agent-claude/src/agents.rs:12-21; crates/viola-agent-claude/src/agents.rs:88-110; crates/viola-agent-claude/src/lib.rs:4'
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: 'In the `viola-agent-claude/` comment add the pure `agents` module (the `claude agents --json` parse).'
    sidecar: 'arch key Project directory structure: `viola-agent-claude` lists the `agents` module.'
    rationale: 'Report Changes, Files New: `crates/viola-agent-claude/src/agents.rs`.'
    basis: '.andromeda/registries/contracts/architecture/project-directory-structure.md'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: 'In the `run/` comment add `agents.rs`: the bounded `claude agents --json` read (`read_logged`, `read_here`, `ReadFailure`, `AgentsRead`) shared by `list`, `revive` and verify''s Run B.'
    sidecar: 'arch key Project directory structure: `src/run/agents.rs` registered.'
    rationale: 'Report Changes, Files New: `src/run/agents.rs`; Symbols / APIs root bin `src/run/agents.rs`.'
    basis: 'src/run/agents.rs:18-25; src/run/agents.rs:27-34; src/run/agents.rs:36-96; src/run/agents.rs:106-116'
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: 'In the `cmd/` comment, list.rs: the caller-side board then one `claude agents --json` read joined into it (STATUS idle/busy, the unwrapped rows).'
    sidecar: 'arch key Project directory structure: list.rs gains the agents read and join.'
    rationale: 'Report Symbols / APIs, `viola list` bullet; `src/cmd/list.rs` `observations`.'
    basis: 'src/cmd/list.rs:105-119'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: 'In the `viola-state/` comment, the board read model adds `JoinKeys`, `Observation`, `join` and `Item::unwrapped` (still no Claude shape), and liveness adds `start_times` (one process-table read).'
    sidecar: 'arch key Project directory structure: viola-state board join types and `liveness::start_times` registered.'
    rationale: 'Report Symbols / APIs: `viola_state::board` `JoinKeys`, `Observation`, `Item::unwrapped`, `join`; `viola_state::liveness::start_times`.'
    basis: 'crates/viola-state/src/board.rs:138-144; crates/viola-state/src/board.rs:146-153; crates/viola-state/src/board.rs:323-345; crates/viola-state/src/liveness.rs:33-43'
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Binary, subcommands and exit codes → test-only binaries)
    change: 'Replace "beside the options it takes one mode word, `statusline-echo`, …" with two mode words: `statusline-echo` (unchanged) and `agents`, which answers `agents --json` from files beside its exe — `agents-mode` (`recorded` · `oversize` · `malformed`, absent = `recorded`), `agents-rows.json` (planted rows, printed unchanged) and a live `sessions/<pid>.json` registry (a row written just before its launch SessionStart fires, `cwd` the launch fixture''s recorded `cwd`, `~` without one; dead or reused pids skipped by one `viola_state::liveness::start_times()` read); no argv option and no env added.'
    sidecar: 'arch §Occupied Resources fake agent (line 299): mode words 1 → 2 (`agents`) and its files beside the exe.'
    rationale: 'Report Symbols / APIs, Fake agent bullet (deviations 12, 13); Counts, pattern `fake-mode`: arch 299 "one mode word" stale; the founder''s fork 2 (files beside the exe, no env seam).'
    basis: 'src/bin/viola-fake-agent.rs:806-817; src/bin/viola-fake-agent.rs:819-833; src/bin/viola-fake-agent.rs:843-866; .andromeda/architecture.md:299'
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Filesystem → `diagnostics/`)
    change: 'verify''s pair holds eight spawn pairs: add one `agents-probe` pair around Run B''s `claude agents --json` read.'
    sidecar: 'arch §Occupied Resources diagnostics (line 341): verify spawn pairs 7 → 8 (`agents-probe`).'
    rationale: 'Report Counts, pattern `spawn-pairs`: arch 341 @c954 stale ("verify spawn pairs 7 → 8"); Symbols / APIs `read_logged` writes the `agents-probe` spawn pair.'
    basis: 'src/run/agents.rs:36-96; .andromeda/architecture.md:341'
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Filesystem → `diagnostics/`)
    change: '`list`''s lines (under a valid `VIOLA_NAME`) gain one `agents-probe` `process-start` / `process-exit` pair per read and one `parse-rejected{parser:"claude-agents-json"}` on a malformed or oversize read (no row text, id, `cwd` or path); `viola revive`''s start arm writes its agents read''s `agents-probe` pair into `run-<name>.ndjson`, and a `session-live` refusal one `process-exit{subject:"self", exit_code:1, detail:"session-live"}`.'
    sidecar: 'arch §Occupied Resources diagnostics (line 341): the `agents-probe` pair and `parse-rejected{parser:"claude-agents-json"}` on `list` and `revive`; `session-live` exit detail.'
    rationale: 'Report Symbols / APIs (`read_logged` writes the spawn pair and the `parse-rejected` line; `viola list` "Spawn pair logged only under a parsed `VIOLA_NAME`"; `viola revive` "its spawn pair lands in `run-<name>.ndjson`"); Coverage (PII none logged); Schema / config `session-live`.'
    basis: 'src/run/agents.rs:36-96; schemas/diag-line.v1.json:78'
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Repository → `fixtures/claude/<cli-version>/`)
    change: 'Register the agents class: `--record` writes Run B''s agents read as `Agents.recorded-1.json` (Run B''s own row only, `ledger::agents_variant`), through the same scrub and refusal, validated by `schemas/claude-agents-fixture.v1.json` and hygiene-walked (`agents_fixtures_pass_hygiene`); the fake agent''s `agents` mode replays it (guard `fake_agent_agents_replay_matches_every_recorded_agents_set`); `2.1.287` gains `Agents.recorded-1.json`, recorded live at chunk 2026-10-16-unwrapped-sessions-on-the-board: 33 files by `ls fixtures/claude/2.1.287 | wc -l`.'
    sidecar: 'arch §Occupied Resources fixtures (line 356): the `Agents.*` class registered; 2.1.287 set 32 → 33 files.'
    rationale: 'Report Changes, Files New `fixtures/claude/2.1.287/Agents.recorded-1.json`, `schemas/claude-agents-fixture.v1.json`; Counts, pattern `fixture-set`: arch 356 @c2771, @c2799 stale; pattern `agents-fixture` unfired (worded otherwise).'
    basis: 'tests/contract_fixture_hygiene.rs:144-159; tests/contract_fake_agent_drift.rs:466-530; .andromeda/architecture.md:356'
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Repository → `fixtures/claude/<cli-version>/`)
    change: '`2.1.287` is stamped at the relied set (`LedgerRow::relied()`: 23 rows on Unix, 20 on Windows) by `contract_ledger_probes` (its `ROW_IDS` the twenty-three ids; `23 pass  0 fail` on Unix).'
    sidecar: 'arch §Occupied Resources fixtures (line 356): relied 22 / 19 → 23 / 20, `ROW_IDS` 23, `23 pass  0 fail`.'
    rationale: 'Report Counts, pattern `relied-rows`: arch 356 @c2878, @c2889, @c2906, @c2968 stale.'
    basis: '.andromeda/architecture.md:356'
    dependent-of: D-arch-decisions
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Repository → schemas)
    change: 'Add a bullet: `schemas/claude-agents-fixture.v1.json`: the tolerant schema for recorded `claude agents --json` fixtures (one array of rows over the measured keys; test-side).'
    sidecar: 'arch §Occupied Resources: `schemas/claude-agents-fixture.v1.json` registered.'
    rationale: 'Report Schema / config: "new `schemas/claude-agents-fixture.v1.json` (one array of rows over the measured keys)"; the schemas bullets list every other schema.'
    basis: 'schemas/claude-agents-fixture.v1.json:1-21'
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: 'The `schemas/` comment adds `claude-agents-fixture.v1.json` (test-side).'
    sidecar: 'arch key Project directory structure: schemas list gains `claude-agents-fixture.v1.json`.'
    rationale: 'Report Schema / config (new schema).'
    basis: '.andromeda/registries/contracts/architecture/project-directory-structure.md'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: 'The `fixtures/claude/<cli-version>/` comment adds the `Agents.recorded-<n>.json` agents variants recorded by `viola verify`.'
    sidecar: 'arch key Project directory structure: the fixtures entry names the `Agents.*` class.'
    rationale: 'Report Changes, Files New `fixtures/claude/2.1.287/Agents.recorded-1.json`.'
    basis: '.andromeda/registries/contracts/architecture/project-directory-structure.md'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Repository → `fuzz/`)
    change: 'Fuzz targets today: `viola_name`, `channel_frame`, `hook_stdin`, `vt100_feed`, `paste_text` and `claude_agents_json` (six); `claude_agents_json` has 6 seeds, kept byte-exact by `.gitattributes` `fuzz/corpus/claude_agents_json/** binary`.'
    sidecar: 'arch §Occupied Resources fuzz (line 365): targets 5 → 6 (`claude_agents_json`, 6 seeds, binary attribute).'
    rationale: 'Report Counts, pattern `fuzz-targets`: arch 365 "today …" list stale; Files New `fuzz/fuzz_targets/claude_agents_json.rs`, `fuzz/corpus/claude_agents_json/` (6 seeds); Schema / config `.gitattributes:7`.'
    basis: 'fuzz/fuzz_targets/claude_agents_json.rs:9-18; .gitattributes:7; .andromeda/architecture.md:365'
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: '`fuzz_targets/{viola_name,channel_frame,hook_stdin,vt100_feed,paste_text,claude_agents_json}.rs`.'
    sidecar: 'arch key Project directory structure (line 97): fuzz brace list gains `claude_agents_json`.'
    rationale: 'Report Counts, pattern `fuzz-targets`: registry `architecture/project-directory-structure.md:97` brace list stale.'
    basis: '.andromeda/registries/contracts/architecture/project-directory-structure.md:97'
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → CI/CD approach
    change: 'The relied ledger rows in CI: all twenty-three on the Unix runners, twenty on `windows-2025`, the first eighteen, `resume-session` and `agents-join`.'
    sidecar: 'arch key CI/CD approach (line 19): relied 22 / 19 → 23 / 20.'
    rationale: 'Report Counts, pattern `relied-rows`: registry `architecture/ci-cd-approach.md:19` stale.'
    basis: '.andromeda/registries/contracts/architecture/ci-cd-approach.md:19'
    dependent-of: D-arch-decisions
note: 'No dependency, crate, IPC method, endpoint, port or env var was added (report Dependencies / Crates / Schema-config), so no Stack row moves. Left as now-true per the report''s no-change dispositions: arch 29, 90''s idle/busy sentence, 93, 97, 177''s contract, 203. Not proposed: [Plugin Scope] (line 97) enumerates verify''s transient unwrapped sessions (print probe + five runs); Run B''s agents read is a bounded child, not a session carrying hooks, so it was left; the orchestrator may want one clause there.'
