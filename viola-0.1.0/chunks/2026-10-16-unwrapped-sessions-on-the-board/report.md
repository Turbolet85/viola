# Report — 2026-10-16-unwrapped-sessions-on-the-board

**Chunk:** Unwrapped sessions on the board — `claude agents --json` read once, unwrapped read-only rows, idle/busy
STATUS, revive refuses a session id live elsewhere (v1-37)
**Date:** 2026-10-16T23:05Z
**Commits:** since `last_wrap` 2026-10-16T18:43:25Z (`92264a7`): `de9ee3f` chore — operator pre-CI commit, for the run
this chunk's verdict reads · `3c1c54d` fix — the fake agent's registry row replays the recorded cwd, never the host's ·
`020c80b` fix — the fake agent's agents listing reads the process table once, whatever the stale rows. Basis: the parent
of the oldest pre-CI commit, `92264a7f254c` (`git log --reverse … --grep 'chore(2026-10-16-unwrapped-sessions-on-the-board): operator pre-CI commit'`).

## Changes (structured — detectors read this)
- **Files:** 53 changed by `gate.py scope` (`scope: clean — changed 53 · listed 37 · recorded 16 (companion 15 ·
  mechanical 0 · in-intent 1 · widening 0) · absorbed 7 · excluded 68`, base `92264a7f`), the per-file added ranges in
  `## New text, by line` below (`cites.py added`: 53 files · 7 new · 259 ranges · 2905 lines).
  - New: `crates/viola-agent-claude/src/agents.rs`, `crates/viola-agent-claude/proptest-regressions/agents.txt`,
    `src/run/agents.rs`, `fuzz/fuzz_targets/claude_agents_json.rs`, `fuzz/corpus/claude_agents_json/` (6 seeds:
    `background-row`, `control-session-id`, `deep-array`, `empty-array`, `measured-row`, `non-array`),
    `schemas/claude-agents-fixture.v1.json`, `fixtures/claude/2.1.287/Agents.recorded-1.json`, `tests/cross_list_rows.rs`.
  - Modified (listed): `crates/viola-agent-claude/src/{lib,ledger}.rs`; `crates/viola-state/src/{board,snapshot}.rs`;
    `src/run/send/slot.rs`, `src/run/mod.rs`, `src/run/version_gate.rs`, `src/cmd/list.rs`, `src/human/board.rs`,
    `src/cmd/revive.rs`, `src/cmd/verify.rs`, `src/cmd/verify/typed.rs`, `src/bin/viola-fake-agent.rs`,
    `schemas/diag-line.v1.json`, `fuzz/Cargo.toml`, `.gitattributes`, `.config/nextest.toml`, `tests/cli_list.rs`,
    `tests/cli_revive.rs`, `tests/cli_verify.rs`, `tests/contract_ledger_probes.rs`, `tests/contract_fixture_hygiene.rs`,
    `tests/contract_fake_agent_drift.rs`, `tests/support/fake.rs`. Listed and unchanged: `fuzz/Cargo.lock` (a `[[bin]]`
    adds no package).
  - Modified (recorded, `scope-record.md`): the 12 `InstanceSnapshot` literal sites + `src/run/send.rs` (step 4),
    `crates/viola-e2e/src/harness/run.rs` and `tests/support/home.rs` (step 10), `tests/support/cli.rs` (step 14);
    `crates/viola-state/src/liveness.rs` also gained `start_times` at the operator pass (deviation 13).
  - Records: `chunks/{marker}/evidence/` (`operator-pass.md`, `live-p4.md`, `live-sessions.ndjson`, the rig's
    `live-pty.py` · `live-ledger.py` · `live-drive.py` · `live-p4-read.py`, `round-204738Z.txt`).
- **Symbols / APIs:**
  - `viola_agent_claude::agents` (new, pure, re-exported `lib.rs:4`): `KEYS: [&str; 7]` (`agents.rs:12-21`),
    `AgentStatus` (idle · busy · unknown, `from_word`, 23-39), `AgentsRow` + `joins(child_pid, agent_session_id)`
    (41-61), `AgentsList { rows, skipped, tolerated }` (63-70), `AgentsFailure` (`Oversize` · `Malformed`, `detail()`,
    72-86), `parse(bytes) -> Result<AgentsList, AgentsFailure>` (88-110); the 512-case property (`proptest!`, 361-387).
  - `viola_agent_claude::ledger`: `LedgerRow::AgentsJoin` appended last to `ALL` (`ledger.rs:44`, `:71`), id
    `agents-join` (`:99`), words `claude agents --json names the live session by pid and session id` (`:132`), relied
    on every OS; `pub struct AgentsCapture` (640-646); the check `agents_joined` (881-909, wired at `:997`);
    `pub fn agents_variant` (914-927, `--record`'s filter to Run B's own row).
  - `viola_state::board`: `SessionStatus::Idle` / `Busy` (`board.rs:123-124`, kebab `idle` / `busy`); `JoinKeys`
    (138-144, not serialised); `Observation` (146-153); `Item::unwrapped` (186-202); `join(&mut Board, Vec<Observation>)`
    (323-345). Callers: `src/cmd/list.rs` only.
  - `viola_state::snapshot::InstanceSnapshot.agent_session_id: Option<String>` (`snapshot.rs:69-72`, additive,
    `#[serde(default, skip_serializing_if = "Option::is_none")]`); written by `src/run/send/slot.rs`'s session-start tap
    (208-213) through `snapshots_with` (74-77), BEFORE the `session-start` line can be read (deviation 4).
  - `viola_state::liveness::start_times() -> HashMap<u32, u64>` (`liveness.rs:33-43`, one process-table read; new
    helper beside `process_start_time`; sole caller the fake agent).
  - Root bin: `src/run/agents.rs` (new, `pub(crate)`): `ReadFailure` (18-25), `AgentsRead` (27-34), `read_logged`
    (36-96, writes the `agents-probe` spawn pair and the `parse-rejected` line), `read_here(home, cwd)` (106-116,
    resolves `claude` by `run::resolve_program` from the current directory); `src/run/version_gate.rs`
    `run_capped(…, cap)` (new; `run_bounded` now calls it with `MAX_FRAME` — its two callers, `verify::run_logged` and
    `version_gate`, read unchanged; `run_bounded` stays unlogged); `src/cmd/list.rs` `observations` (105-119);
    `src/cmd/revive.rs` `Refusal::SessionLive` (detail `session-live`, `revive.rs:53`), `live_elsewhere` (153-158), the
    reading wired into `preflight_order` (`:180`, skipped under `--fork`); `src/cmd/verify/typed.rs` `agents_read`
    (365-376); `src/human/board.rs` the STATUS words, the separator and the unwrapped rows (201-204).
  - `viola list`: one `claude agents --json` read after `board::read`, joined by `child_pid` OR `agent_session_id`;
    joined wrapped rows read STATUS `idle` / `busy`; every unjoined row an unwrapped item (`wrapped:false`, `liveness`
    `live`, `dialog_pending:false`, no `wheel` / `budget_paused` / `cli_version` / `cli_verified`), NAME = the row's
    `sessionId`, sorted by name after the wrapped items; human board prints `-- UNWRAPPED - READ-ONLY --` then the rows,
    `n/a` in WHEEL, DIALOG, CLI, every NAME through `escape_row`, NAME width in characters (deviation 6). A failed,
    oversize or malformed read leaves the board as `board::read` gave it. Spawn pair logged only under a parsed
    `VIOLA_NAME`.
  - `viola revive`: preflight chain holder → logged session → `session-live` → recorded directory (four readings;
    was three); pair `unable: <name>'s session is open in another claude` / `hint: close it there first, or start a
    copy: viola revive <name> --fork`; one `process-exit{subject:"self", exit_code:1, detail:"session-live"}`, no
    child; a failed read passes (fail open); its spawn pair lands in `run-<name>.ndjson`.
  - `viola verify`: Run B makes ONE `claude agents --json` read once its first turn has settled, before the framing
    pastes (deviation 7), program = Run B's, cwd the probe dir, R8 strip, `VERSION_DEADLINE`; capture carried in memory
    (not written to the captures dir); spawn pairs per uninterrupted verify 7 → 8; `--record` writes
    `Agents.recorded-1.json` holding Run B's own row only, through the scrub.
  - Fake agent (test-only, `src/bin/viola-fake-agent.rs`): first word `agents` answers `agents --json` from files
    beside its exe — `agents-mode` (`recorded` · `oversize` · `malformed`, absent = `recorded`), `agents-rows.json`,
    a live `sessions/<pid>.json` registry (row written just before its launch SessionStart fires, `cwd` = the launch
    fixture's recorded `cwd`, `~` without one — deviation 12; dead or reused pids skipped by one `start_times()` read —
    deviation 13); no argv option added, no env.
  - Test support: `tests/support/fake.rs` `empty_bin` · `claude_bin` · `claude_in`; `tests/support/cli.rs`
    `spawn_on` · `viola_on`; a plain `spawn` of `list` runs with `PATH` = an empty bin dir (deviation 9).
- **Crates / modules:** added `viola-agent-claude::agents`, root `run::agents`; changed `viola-state::{board,
  snapshot, liveness}`; no crate added or removed.
- **Dependencies:** none added or bumped (`fuzz/Cargo.lock` unchanged; `cargo deny check` and the fuzz-lock audit
  green).
- **Schema / config:** `schemas/diag-line.v1.json` `process-exit.detail` enum gains `session-live` (`:78`, 12 → 13
  members); new `schemas/claude-agents-fixture.v1.json` (one array of rows over the measured keys); `snapshot.json`
  gains optional `agent_session_id`; `.gitattributes:7` `fuzz/corpus/claude_agents_json/** binary`;
  `.config/nextest.toml:24` the 20 s kill line gains `binary(cross_list_rows)` (18 → 19 names); no `config.json` key,
  env var or CLI flag added.
- **Spec-master edits:** none (implement edits no master; the wrap's P2 owns them).
- **Counts / qualifiers moved:** (sweep `cascade.py sweep --pre-pass`, trail `cascade-2026-10-16-unwrapped-sessions-on-the-board-pre-pass.json`,
  baseline `020c80bd`; per-pattern master figures are the `standing` counts; dispositions per class with counts)
  - verify spawn pairs 7 → 8 (`tests/cli_verify.rs` literal) — pattern `spawn-pairs`: 5 standing (arch 341, sec 114,
    test 586, obs 664, obs 789) · 5 stale. Also `obs-plan.md:788` "verify logs its seven spawns" (row of
    `agents-read` / `start-times`) · stale.
  - `LedgerRow::ALL` 22 → 23, relied 22 / 19 → 23 / 20 (`LedgerRow::relied()`) — pattern `relied-rows`: 16 standing
    (arch 88 · 199 · 356 · 405, sec 539, design 503, layouts 170 · 199, test 505 · 586 · 596, obs 664, registries
    `architecture/ci-cd-approach.md:19` · `architecture/crate-dependency-direction.md:6` ·
    `test-plan/5-command-implementation.md:8` · `:43`) · 16 stale (each states twenty-two / 22 / 19 / nineteen / the
    `[22/22]` · `[19/19]` last lines or the row list ending `resume-session`); on the same lines the `relied()` token
    alone holds (arch 88 @c441 · @c1654 · @c12252, obs 664 @c1988, crate-dependency-direction:6 @c1443). Leaves 9
    (P2 cascade).
  - fixture set 32 → 33 (`ls fixtures/claude/2.1.287 | wc -l`) — pattern `fixture-set`: 2 standing (arch 356, test
    927) · 2 stale.
  - fuzz targets 5 → 6 (`ls fuzz/fuzz_targets | wc -l`; gains `claude_agents_json`) — pattern `fuzz-targets`: 4
    standing · 2 stale (arch 365's "today …" list, registry `architecture/project-directory-structure.md:97`'s brace
    list) · 2 no-change (test 401 and registry `test-plan/5-command-implementation.md:42`: the generic `<target>`
    path).
  - `process-exit` `detail` enum 12 → 13 (gains `session-live`) — pattern `detail-enum`: 2 standing · 1 stale (obs
    753, the `detail` code catalog: no master names `session-live` — the pattern's `session-live` alternative hit 0
    master lines) · 1 no-change (obs 491, a table pointer to §6).
  - the nextest 20 s kill line 18 → 19 `binary(…)` names — pattern `kill-line`: 6 standing, all in test-plan registry
    files (`bootstrap-phases-derive-for-route-setup-project.md:10` · `:11` · `:12` · `:15` · `:17`,
    `test-data-bootstrap.md:16`) · 6 no-change (they state the `mutants` / `ci` profiles and the slow-timeout
    mechanism, never the default profile's 20 s line's list or count).
  - the fake agent's mode words 1 → 2 (`statusline-echo`, `agents`) and its agents files beside the exe — pattern
    `fake-mode`: 5 standing · 3 stale (arch 299 "one mode word", test 926 `FAKE_CLAUDE_AGENTS_MODE`, test 953 "one mode
    word") · 2 no-change (test 742, the `--statusline-echo` harness flag; registry
    `test-plan/5-command-implementation.md:4`, the `--agents-mode` harness flag, still owed to "End-to-end path binary
    and harness legs").
  - verify's child set gains one bounded agents read (no live start added) — pattern `verify-kinds`: 0 — `control:
    verify-kinds … never fired over the seven masters' pre-pass text (020c80bd)` (the masters word verify's child set
    otherwise; the detectors' own sweeps are the control).
  - revive's preflight readings 3 → 4 and its refusal pairs gain `session-live` — pattern `revive-pairs`: 3 standing ·
    1 stale (arch 193 @c801 "second of three") · 2 no-change (sec 105, sec 208: the callers list). The count words
    (`five|six|fifth|sixth … pairs`) hit 0 master lines.
  - STATUS idle / busy built — pattern `status-words`: 6 standing · 1 stale (arch 280 "rows always read `status`
    `unknown`") · 5 no-change (design 126 · 127 · 128 · 216, registry `architecture/project-directory-structure.md:67`).
  - the unwrapped rack built — pattern `unwrapped-rack`: 36 standing · 13 stale (arch 50 · 83 · 237 · 280, sec 110 ·
    212, design 441, layouts 91, test 585, a11y 68 · 134 · 209 · 236 — each an "owed / not built / no unwrapped row is
    listed" clause; the same 13 lines the owner pattern `title` holds) · 23 no-change (contract statements now true:
    arch 90 · 97 · 177, sec 210 · 389, design 13 · 360 · 420 · 433 · 439 · 465, layouts 70 · 262 · 283 · 284, test 81 ·
    253 · 789 · 875 · 876, obs 165 · 866, registry `test-plan/5-command-implementation.md:4`).
  - the `claude agents --json` read built for `list`, `revive` and `verify` — pattern `agents-read`: 48 standing ·
    11 stale (arch 50 · 83 · 88 @c16964 · 237 · 280, sec 110 · 212 @c106 · @c217, design 441, test 926 · 953 @c4133,
    a11y 236) · 37 no-change for this fact (Stack, contract and rule statements now true: arch 29 · 90 · 93 · 177 · 203,
    sec 210 · 221 · 389 · 489 · 516, design 128 · 439, layouts 285, test 56 · 60 · 81 · 82 · 180 · 302 · 315 · 501 · 553
    · 853 · 875 · 899, obs 103 · 104 · 352 · 423 · 741 · 751 · 788 · 789 · 866, registries
    `architecture/crate-dependency-direction.md:6` @c353 · `test-plan/5-command-implementation.md:7` · `:59`); obs 788
    and 789 are stale for the spawn count (above).
  - the join field = ledger row `agents-join`, both keys measured exposed — pattern `agents-join`: 3 standing · 2
    stale (arch 83 the "open item", arch 237 "owed … join-field ledger row and probe") · 1 re-word (arch 177 "part of
    the join-field ledger row" → the row's id).
  - the snapshot's `agent_session_id` written on each `session-start` — pattern `session-id`: 17 standing · 17
    no-change (each states the event field, the replay or the snapshot field; none says it is absent).
  - every test-started `list` / `revive` under a test-scoped `PATH` — pattern `test-path`: 3 standing · 1 stale (test
    926's env column) · 2 no-change (test 553, 983).
  - the fixture class `Agents.*` — pattern `agents-fixture`: 0 — `control: … agents-fixture never fired over the seven
    masters' pre-pass text (020c80bd)`.
  - `run_capped` and `start_times` added — pattern `start-times`: 2 standing (arch 341 @c1836, obs 788) · 2 no-change
    (`run_bounded` stays unlogged).
  - the a11y timeout extension gains the 5 s agents read — pattern `timeout-ext`: 1 standing (a11y 281, the
    `### Timeout extensions` heading) · 1 stale (the section lacks the bound).
  - the `list` row-integrity suite landed — pattern `row-integrity`: 1 standing (test 875) · no-change (now true).
  - Lines over 2 000 chars, (line, fact) dispositions: arch 46 session-id ×3 holds · arch 88 relied @c441 holds,
    @c1098 stale, @c1131 stale, @c1654 holds, @c1852 stale, @c1890 stale, @c12252 holds, @c12374 stale, @c12440 stale;
    agents-read @c16964 stale · arch 90 agents-read @c92 holds, @c397 holds; unwrapped @c451 holds · arch 193
    session-id @c199 · @c2028 · @c3317 holds; revive-pairs @c801 stale · arch 299 fake-mode @c3563 stale, @c3575 stale
    · arch 341 spawn-pairs @c954 stale; start-times @c1836 holds · arch 356 fixture-set @c2771 stale, @c2799 stale;
    relied @c2878 · @c2889 · @c2906 · @c2968 stale · arch 405 relied @c3957 · @c3968 · @c3985 stale · sec 212
    agents-read @c49 holds, @c106 stale, @c217 stale; title / unwrapped @c346 stale, @c387 stale · design 503 relied
    @c251 · @c274 · @c398 stale · test 505 relied @c1209 · @c1220 · @c1237 stale · test 586 relied @c549 · @c609 stale;
    spawn-pairs @c3773 stale · test 953 agents-read @c4133 stale; fake-mode @c3960 · @c3972 stale · obs 664 relied
    @c1988 holds, @c2145 stale; spawn-pairs @c2327 stale · crate-dependency-direction:6 agents-read @c353 holds; relied
    @c1380 · @c1398 stale, @c1443 holds · bootstrap-phases:10 kill-line @c66 · @c249 · @c3151 · @c3569 no-change.
  - Sweep reach: 18 patterns · 20 moved facts, 20 with a pattern (the report's own count) · owner pattern 13 standing
    · by line 15 lines · 35 offsets. The listing reaches its patterns' wording only; a site stating a fact in other
    words is not in it (the `verify-kinds` and `agents-fixture` zeros among them).
- **Dev-tool versions:** none — `claude` 2.1.287 re-read by path (the record round and the P4 leg); the bare name
  resolves to 2.1.296 on the Linux dev host (not this chunk's subject).
- **Harness / gate surface:** `crates/viola-e2e/src/harness/run.rs` `LEDGER_ROWS` / `LIVE_PASS` follow the 23 / 20
  relied rows (step 10); `tests/support/home.rs` `dialogless_home`'s summary `19 pass  4 fail` (`16` on Windows) and
  `stamped_home`'s message now names the failing rows (362-367); no harness verb, status shape or CI step added.
- **Cross-project / external claims:**
  - CI: `ci#38968360744` on `de9ee3f96eeb` — red, `test (windows-2025)` (7 `cli_verify` cases, `Agents.recorded-1.json
    absolute-path`); `ci#38970086070` on `3c1c54df925b` — red, `test (ubuntu-latest)` coverage job (2 `stamped_home`
    reds, `llvm-cov-summary-missing`); `ci#38971772946` on `020c80bd045b` — **green, checks 15/15, attempt 1**; the
    three `test` jobs 2635 / 2647 / 2643 passed; `cli_list` 17 / 19 / 19, `cross_list_rows` 1 / 1 / 1, `cli_revive`
    14 / 17 / 17 PASS lines (`evidence/operator-pass.md`).
  - The installed `claude` 2.1.287 (`$HOME/.local/share/mise/installs/claude/2.1.287/claude`): `stamped 2.1.287  23
    pass  0 fail` (`evidence/round-204738Z.txt`); P4 two writers, P12 follows (`evidence/live-p4.md`).
  - inputs (`inputs.py verify`, `inputs: 2 entries — … n/a 2 · uncited 1 · unparsed 0`):
    - `I1 · message: the founder, live in the overseer dialog, 2026-10-16T19:54:09Z, relayed verbatim by the operator ·
      copy · n/a` — the four P4 forks, cited in `scope.md` and `plan.md`.
    - `I2 · message: the operator, in the phase session, 2026-10-16T20:04:54Z · copy · n/a` — UNCITED by `verify`
      (no file cited it); cited here, `inputs#I2`. It names the P5 review card
      `.andromeda/runs/2026-10-16T18-50-06-phase/p5-card-1.md` (the newest and only card); the word: "yes", the
      operator's own, 2026-10-16, in the phase session. The card's `## Beyond the plan` rows, verbatim:
      - `boot --agents-mode recorded|oversize|malformed` (test-plan §3, not built): owned by the existing entry
        **"End-to-end path binary and harness legs"**, with `status` running `viola list`. Under fork 2 the flag writes the
        fake's `agents-mode` file rather than an env var.
      - The unwrapped-name `instance-unreachable` hint (`<name> is not a wrapped session; only sessions started with viola
        run take commands`, design-system cli pattern 2, not built). This chunk proves the exit 21 only. Owned by the
        existing entry **"Exit-cause code catalogue"** (`v1-18`).
      - The Windows readings of `claude agents --json`, of the `agents-join` row and of `session-live`: appended to the
        existing entry **"Windows-only live measurements"**.
      - **Conditional route insertion**, only if the P4 leg reads `refused` or `copied`: a new working-route entry
        **"Session-live as a warning"**, placed directly after this entry in Epoch 5. It turns `session-live` into a warning
        on the founder's word. Not minted on a `two writers` reading.

      Authority: the operator's own word on the card (`inputs#I2`). The P4 leg read `two writers`, so the fourth row's
      condition is unmet: no "Session-live as a warning" entry is minted (P5).
- **Reverted / negative API facts:** the fake agent's registry row first carried its real working directory — removed
  at `3c1c54d` (it leaked the runner's checkout path into `verify --record`'s kept row on `windows-2025`). A per-row
  sysinfo lookup in the fake's listing — replaced at `020c80b`. A probe read placed directly before `/clear` — moved
  before the framing pastes (deviation 7). A snapshot write after the `session-start` line — moved before it
  (deviation 4).
- **Insufficient fixes (written, kept, not the remedy):** none.
- **Spec claims disproved by measurement:**
  - architecture §Standard Contracts (Session liveness) "open item" — whether a `claude agents --json` row exposes the
    child pid and session id (arch 83, pattern `agents-join`): measured exposed on 2.1.287 (`pid`, `sessionId`; the
    record round's `agents-join` pass; `live-p4.md` (a) join by pid and id) — the open item is answered, not false.
  - test-plan §7 Seed strategies' `FAKE_CLAUDE_AGENTS_MODE=recorded|oversize|malformed` env row (test 926, patterns
    `fake-mode` · `test-path`): superseded by the founder's fork 2 (files beside the exe, `inputs#I1`); no env seam
    exists.
  - research's P4 open question (an interactive resume of a live id: refused, copied, or two writers): measured
    **two writers** on 2.1.287 (`live-p4.md` (c)); P12 measured **follows** (`live-p4.md` (b)). No master states
    either as fact today (the founder's fork 3 made P4 travel with this entry).
- **Expected amendments (from plan):**
  - architecture §Standard Contracts (Session liveness) — carried: Symbols / APIs (`viola list`), Spec claims disproved
    (the open item); patterns `agents-read` (arch 10 + 1 registry), `agents-join` (arch 3), `title` (arch 4),
    `unwrapped-rack` (arch 7), `status-words` (arch 1).
  - architecture §Established Decisions [CLI Version Compatibility] — carried: Counts (relied rows, spawn pairs),
    Symbols / APIs (`LedgerRow::AgentsJoin`, verify Run B); patterns `relied-rows` (arch 4 + registries 2),
    `spawn-pairs` (arch 1), `verify-kinds` (0, control unfired).
  - architecture §Occupied Resources — carried: Symbols / APIs (fake agent `agents`), Counts (fixture set, fuzz
    targets); patterns `fake-mode` (arch 1), `fixture-set` (arch 1), `fuzz-targets` (arch 1 + registry 1).
  - security-plan §Threat Model Summary and §Input Validation — carried: Symbols / APIs (the agents child for `list`,
    `revive`, `verify`); patterns `agents-read` (sec 7), `title` (sec 2), `unwrapped-rack` (sec 4), `spawn-pairs` (sec
    1), `relied-rows` (sec 1), `verify-kinds` (0, control unfired — the seventh verify spawn kind is worded otherwise).
  - test-plan §7 Seed strategies, §6, §5 — carried: Spec claims disproved (the env row), Counts (fixture set, relied
    rows, spawn pairs); patterns `fake-mode` (test 3 + registry 1), `test-path` (test 3), `row-integrity` (test 1),
    `agents-read` (test 14 + registries 2), `relied-rows` (test 3 + registries 2), `spawn-pairs` (test 1),
    `fixture-set` (test 1), `fuzz-targets` (test 1 + registry 1), `title` (test 1).
  - obs-plan §6 and §1 — carried: Schema / config (`session-live`), Counts (spawn pairs, detail enum); patterns
    `detail-enum` (obs 2), `spawn-pairs` (obs 2), `agents-read` (obs 9), `relied-rows` (obs 1); `oversize` for
    `claude-agents-json`: the parser enum and the detail word are already members (obs 751).
  - design-system §Surface: cli patterns 1 and 2 — carried: Symbols / APIs (`viola list`, `viola revive`); patterns
    `title` (design 1), `unwrapped-rack` (design 7), `agents-read` (design 3), `status-words` (design 4),
    `relied-rows` (design 1).
  - layout-templates §Output structure — `viola list` / `viola revive` — carried: Symbols / APIs; patterns `title`
    (layouts 1), `unwrapped-rack` (layouts 5), `agents-read` (layouts 1), `relied-rows` (layouts 2), `revive-pairs`
    (layouts 0 — the pair count is not worded by number there).
  - a11y-plan §1, §4 (P6), §8 — carried: Symbols / APIs (the separator), Coverage (the 5 s read on no keystroke);
    patterns `title` (a11y 4), `unwrapped-rack` (a11y 4), `agents-read` (a11y 1), `timeout-ext` (a11y 1).
  - `matrix#v1-37` — no master site — the ledger flip is P7.3's (`implemented` → `verified`).
- **Coverage of new surfaces:**
  - `claude agents --json` child (`list`, `revive`, `verify` Run B) → validation direct spawn of the resolved program,
    R8 strip, `take(MAX_FRAME + 1)`, 5 s kill, tolerant parse to a closed failure ✓ · instrumentation `agents-probe`
    spawn pair + `parse-rejected{parser:"claude-agents-json"}` ✓ · PII no row text, id, `cwd` or path logged ✓ ·
    tests unit (`agents::tests`, `run::agents::tests`) / integ (`cli_list`, `cli_revive`, `cli_verify`) / e2e (the
    record round, the P4 leg) · a11y n/a · tokens n/a
  - `viola list` unwrapped rack + STATUS words → validation `escape_row` on every NAME, `sessionId` never a `ViolaName`
    ✓ · instrumentation the spawn pair under `VIOLA_NAME` ✓ · PII session ids are content, escaped, never logged ✓ ·
    tests unit (`board_row_*`, `board_join_*`) / integ (`cli_list`, `cross_list_rows` piped · `NO_COLOR` /
    `TERM=dumb` · outer PTY) · a11y the separator line, no SGR, one write ✓ · tokens n/a (plain ASCII)
  - `viola revive` `session-live` → validation closed `Refusal` ✓ · instrumentation `process-exit{detail:"session-live"}`
    ✓ · PII no pid, id or directory in the pair ✓ · tests unit (`preflight_order_*`) / integ (`cli_revive`) / e2e (P4
    leg (d)) · a11y stderr pair, hint last ✓ · tokens n/a
  - `agents-join` ledger row + `--record` agents variant → validation closed check, `--record` scrub ✓ ·
    instrumentation the probe's spawn pair ✓ · PII only Run B's own row kept, scrubbed ✓ · tests unit
    (`check_agents_join_*`, `agents_variant_*`) / integ (`cli_verify`, `contract_ledger_probes`,
    `contract_fixture_hygiene`) / e2e (the record round) · a11y n/a · tokens n/a
  - `snapshot.json` `agent_session_id` → validation additive serde ✓ · instrumentation n/a · PII operational id,
    never in an error body ✓ · tests unit (`send_session_start_*`) · a11y n/a · tokens n/a
  - fake agent `agents` mode + registry, `liveness::start_times` (test seam / helper) → validation n/a · tests unit
    (`agents_output_*`, `live_sessions_skip_*`, `launch_session_replays_*`, `start_times_*`) / contract
    (`contract_fake_agent_drift`) · others n/a
  - `claude_agents_json` fuzz target → tests fuzz replay (`run --fuzz-replay`) · others n/a

## Deviations from intent
The implement run's 13 (outcome-2026-10-16-unwrapped-sessions-on-the-board.md), each justified:
1. Step 1 — `parse` returns `AgentsList { rows, skipped, tolerated }`, not `Vec<AgentsRow>`: the step asks that
   skipped elements and tolerated keys be counted.
2. Step 2 — proptest wrote `proptest-regressions/agents.txt` from a real shrunk failure of the ORACLE (a float that does
   not round-trip through serde_json's text); the oracle now compares against the input re-parsed; the seed is kept.
3. Step 3 — a new `run_capped(…, cap)` under `run_bounded`; one helper `run::agents::read_logged` writes the spawn pair
   and the parse line at each caller's site; `run_bounded` stays unlogged.
4. Step 4 — the snapshot's `agent_session_id` is written BEFORE the `session-start` line can be read (after it raced
   `cli_list`'s `path1` and `e5` readers); the wheel's precedent in the same function.
5. Step 6 — `list` resolves `claude` from its current directory; an unreadable current directory is a failed read.
6. Step 7 — NAME's width counts characters, not bytes.
7. Step 11 — Run B's read is made once its first turn has settled, before the framing pastes (inside the plan's
   window): placed before `/clear` it inherited the framing pastes' failures (two `cli_verify` cases); the capture is
   carried in memory.
8. Step 13 — the fake's row is written just before its launch SessionStart fires; rows of SIGKILLed fakes (verify Runs
   C, D, E) stay and are filtered by the start-time check; the fake removes only its own row.
9. Step 14 — a plain `support::cli::spawn` of `list` runs with `PATH` = an empty bin dir, so three callers needed no
   edit; `spawn_on` / `viola_on` take a planted bin dir.
10. Step 15 — `cli_revive`'s `assert_refused` counts the wrapper's own `process-exit{subject:"self"}` only; the joined
    `cli_list` case holds its turn by a `viola send` the fake never stops; `cross_list_rows`' Windows terminal case
    reads each escaped name as text under a 200-column terminal (ConPTY re-renders escapes), the no-ESC assertion
    Unix-only.
11. The plan's integration filter did not select `cli_answer`, whose `dialogless_home` pinned `18 pass  4 fail`;
    `pre-push` found it (now `19 pass  4 fail`, `16` on Windows).
12. Step 13, after CI red on `windows-2025` — the fake's registry row carries the launch fixture's recorded `cwd`; guard
    in `contract_fake_agent_drift` (red with the real cwd, green restored).
13. Step 13, after CI red on `ubuntu-latest` (coverage) — one process-table read per listing
    (`viola_state::liveness::start_times`, new); 1 860 rows 1.9 s → 0.23 s; cause INFERRED (the stamp's agents read
    past its 5 s bound under coverage), not read; `stamped_home` now names failing rows.

The P5 card's fourth row (a conditional "Session-live as a warning" entry) is not minted: its condition (P4 `refused`
or `copied`) did not occur.

Scope record (`gate.py scope` beside it: `clean — changed 53 · listed 37 · recorded 16 (companion 15 · mechanical 0 ·
in-intent 1 · widening 0)`), each authority self:
- companion (15) — serves step 4 (the `agent_session_id` field's `InstanceSnapshot` literal sites): `src/cmd/hook.rs`,
  `src/cmd/hook/statusline.rs`, `src/run/snapshot.rs`, `src/cmd/run.rs`, `src/run/wheel.rs`, `src/run/dialog.rs`,
  `src/run/budget.rs`, `src/run/links.rs`, `crates/viola-state/src/strict.rs`, `crates/viola-state/src/replay.rs`,
  `crates/viola-state/src/liveness.rs`, `crates/viola-state/tests/state_replay.rs`, `src/run/send.rs`; serves step 10
  (the relied count): `crates/viola-e2e/src/harness/run.rs`, `tests/support/home.rs`.
- mechanical (0).
- in-intent (1) — serves step 14 (the tests' `PATH`): `tests/support/cli.rs`.
- widening (0).

## Decisions & corrections
- The founder's own answers to the four P4 forks (`inputs#I1`, 2026-10-16T19:54:09Z, relayed verbatim): join row
  relied with its probe in Run B (a verify-child widening ratified by him); fake agent files beside its exe (no third
  env seam); `session-live` refusal now + a P4 leg, cap 8 live starts (6 + 2); the "open elsewhere / --fork" pair.
  Held: 8 starts used, none re-fired.
- The operator's own "yes" to P5 card 1 (`inputs#I2`): the one-read narrowing of fork 1, the confirm-class leans (NAME =
  `sessionId`, the snapshot field, `oversize` logged, recording on 2.1.287, the empty-rack form, no truncation or
  colour), the v1-37 concretization and the four Beyond-the-plan placements.
- A fake agent's registry row must replay recorded values, never the host's: its real cwd leaked a CI checkout path
  (`D:\a\…`, outside the user home on `windows-2025`) into `verify --record`'s kept row, which `--record` refused
  `absolute-path` in 7 `cli_verify` cases (`ci#38968360744`); on the Unix runners and the dev host the checkout lies under
  the home, so the scrub hid it (deviation 12).
- A per-row process lookup in a listing that grows with killed processes pushed a bounded read past its deadline under
  coverage: 1 ms per row, 1 860 rows 1.9 s; one table read per listing, 0.23 s (deviation 13; `ci#38970086070`).
- A snapshot field written after its event line raced every reader that waits on that line (cli_list `path1`, `e5`
  red): write it before the line can be read (deviation 4).
- A verify probe placed after a chain of pastes inherited their failures (two `cli_verify` cases): place it right after
  the turn it measures (deviation 7).
- A proptest oracle comparing a row against the pre-serialisation value failed on a float that does not round-trip
  through serde_json text: compare against the input re-parsed from its bytes (deviation 2).
- A chunk's integration filter missed a test binary whose shared support pinned the changed count
  (`dialogless_home`, `cli_answer`); only `pre-push` found it (deviation 11).
- The unexplained `cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` red (firing 5, entry 11,
  `left: Some(0) right: Some(10)`): green in 3 other local firings, 20 two-sided reruns (this tree and `git archive
  92264a7f254c`) all green, PASS on the three runners of `ci#38971772946`; cause not known. The operator's word at this
  wrap's invocation: a **WATCH pin** — P5 places it (route-resolve's watch form) on the entry that next touches the send
  / gate path.
- Leftover dirs, not removed, for the founder's desk (the operator's word at invocation): `target/control-92264a7/`,
  `target/p4-live-20261016T211033Z/`, `target/agents-scale-214213/` (the 1 860-row timing copy),
  `target/e2e-home/viola-record-20261016T204612Z/` (on the tmpfs), and the implementing session scratchpad's
  `control/`, `control.tar`, `p4-rig/` (a recursive `rm` refused by the host's deny rule). P6 names them in the handoff.
- Guard refusals (implement): a `cat` heredoc to a file (re-issued through Write / Edit), a `cd` that moves the cwd
  (twice, re-issued with absolute paths), a recursive `rm` (not re-issued). This wrap: a `cd` that moves the cwd, three
  times (re-issued with absolute paths or a subshell).
- A matrix field path probed before the contract's schema was read (`KeyError: 'acceptance'`); the value lives at
  `verification.acceptance`.
- The P1 site sweep: three pattern controls unfired over the masters (`owner-generic`, `verify-kinds`,
  `agents-fixture`), re-fired with the other 18; the masters word verify's child set and the agents fixture class in
  words no pattern here carried.
- Standing learnings met (implement): `testing.md` 2026-10-16 (an oracle copied from the first run's output);
  `testing.md` 2026-10-06 (an acceptance held only in substance on Windows closed by extending the witness);
  `verification-harness.md` 2026-09-29 (a local red judged two-sided, left open while its cause is unknown).

## Outcome
Acceptance criteria, each against the diff:
- (arch) `matrix#v1-37` — MET: the board's join, STATUS, unwrapped rack, `wrapped:false`, exit 21 on an unwrapped
  name (`cli_list` `e1_send_naming_an_unwrapped_row_is_unreachable`), escapes (`cross_list_rows` three modes),
  `--json` envelope; witnesses PASS on the three CI OSes (`ci#38971772946`). The GUI-envelope clause is out of 0.1.0
  by the founder's ruling of 2026-10-16 (v1-37's notes).
- (arch) a failed / oversize / malformed read equals the no-`claude` board — MET
  (`list_of_an_answer_that_does_not_parse_equals_the_no_claude_board`).
- (arch) `ALL` = 23, last `agents-join`, `[NN/23]` / `[NN/20]`, `stamped 2.1.287  23 pass  0 fail` — MET (ledger unit
  cases, `cli_verify`, `round-204738Z.txt`).
- (security) the agents child direct, R8 strip, `take(MAX_FRAME + 1)`, 5 s; never a `ViolaName`; escaped — MET.
- (security) `Agents.recorded-1.json` holds only the probe's own row, passes the scrub and hygiene walk; the fuzz-lock
  audit green — MET.
- (design) the `session-live` pair, exit 1, empty stdout, no child; `--fork` and a failed read pass — MET (`cli_revive`;
  `live-p4.md` (d)).
- (layouts) escaped one-line names, caption columns, no trailing space, no separator without unwrapped items — MET
  (`board_row_*`, `cross_list_rows`).
- (obs) one `agents-probe` pair per read under an open log; one `parse-rejected` for malformed; one
  `process-exit{detail:"session-live"}`; the schema admits it — MET.
- (tests) property at 512 with regressions committed; `claude_agents_json` replays green — MET.
- (tests) every test-started `list` / `revive` under a test-scoped `PATH` — MET (step 14, deviation 9).
- (a11y) both racks, unchanged `CAPTION`, no SGR / `\r` / cursor sequence — MET.
- (live) at most 8 `claude` starts — MET: 6 (round) + 2 (leg), each ledgered (`live-starts.md`, `live-sessions.ndjson`).

Gates (the latest firing of each on the final tree; trail
`.andromeda/runs/2026-10-16T20-14-16-implement/gate-2026-10-16-unwrapped-sessions-on-the-board.json`, the pass's in
`evidence/operator-pass.md`):
- `cargo fmt --all --check` — green · `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` —
  green (after `contains_key`, the pass's second fix) · G1 `rg … instrument …` — green (`exit 1`, no output) ·
  `bash scripts/lint-probes.sh` — green · `run --unit` — green · `run --unit --filter 'test(/check_agents_join_/) |
  test(/(^|::)agents::tests::/)'` — green.
- `test -x … 2.1.287/claude` (`leg = 'round'`) — green in the round · the record entry (`leg = 'live'`) — green,
  `stamped 2.1.287  23 pass  0 fail` (`round: COMPLETE · legs fired 1/1`, `evidence/round-204738Z.txt`) · the census
  (`leg = 'round'`) — green (no output). Never re-run by the wrap.
- the fixture guard — green.
- the integration filter (11 binaries) — green at its last firings (11.3, 11.4); firing 5's red on `cli_send
  send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins` stays open, cause unknown, owner: the WATCH pin P5
  places (the operator's word); basis "not this chunk's" UNOBTAINABLE (reproduced on neither tree in 20 runs).
- `run --fuzz-replay` — green · `cargo deny check` — green · `cargo deny --manifest-path fuzz/Cargo.toml check
  advisories sources` — green.
- smoke `cleanup` / `boot` / `status` / `cleanup` — green (`processes_gone`, `endpoint_gone` true) · G2 — green · G4
  `schema-check` — green.
- `pre-push` — green (21.6: `"ok":true`, coverage 2647 of 2647, `gate` no breaches).
- `gate.py hygiene` (`leg = 'operator'`) — green, `hygiene: clean` · the push (`leg = 'operator'`) — three
  fast-forwards (`de9ee3f`, `3c1c54d`, `020c80b`), no force push · `ci.py conclusion` (`leg = 'operator'`) — final
  `020c80bd045b verdict: green · checks 15/15 · wall 534 s` (`ci#38971772946`, attempt 1); the two earlier reds were this
  chunk's, fixed (deviations 12, 13).
- No deferral: Rust delta in every firing; the Windows target's clippy (check only) exit 0 beside the block.

Watches: none folded. New: the `cli_send` red → a WATCH pin, placed at P5.

Outcome basis: the operator pass ran — its commits `de9ee3f` · `3c1c54d` · `020c80b` and the final HEAD's CI run
`ci#38971772946` (`evidence/operator-pass.md`) are the basis for the gate verdicts; implement's persisted report
`.andromeda/runs/2026-10-16T20-14-16-implement/outcome-2026-10-16-unwrapped-sessions-on-the-board.md` (the only
implement dir holding this marker) for the deviations, the open red and the census; `journal.md` beside it. This wrap
began in an earlier window (P1 stopped at its context alarm, `p1-progress.md` in this run dir) and was finished here
from those artifacts.

Process hygiene: implement's census (21:13Z) — the record round's `viola verify` and six `claude` children, the smoke
session, the rig hosts `p4a` / `p4b` and their wrappers and children, `viola revive p4a` (exited 1, no child), the
two-sided reruns: all terminated; the implementing session's own `claude` left running (that session). Re-measured
here at 2026-10-16T23:01:36Z: no running process whose executable lies under this repository.
## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 92264a7f (the parent of the oldest pre-CI commit de9ee3f9) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### .config/nextest.toml — added 1 line(s) in 1 range(s)
added: 24
### .gitattributes — added 1 line(s) in 1 range(s)
added: 7
### crates/viola-agent-claude/proptest-regressions/agents.txt — new file · 7 line(s)
### crates/viola-agent-claude/src/agents.rs — new file · 388 line(s)
- 12-21 @13 «pub const KEYS: [&str; 7] = [»
- 23-29 @25 «pub enum AgentStatus {»
- 31-39 «impl AgentStatus {»
  - 32-38 «pub fn from_word(word: Option<&str>) -> Self {»
- 41-53 @43 «pub struct AgentsRow {»
- 55-61 «impl AgentsRow {»
  - 56-60 @57 «pub fn joins(&self, child_pid: Option<u32>, agent_session_id: Option<&str>) -> bool {»
- 63-70 @66 «pub struct AgentsList {»
- 72-77 @74 «pub enum AgentsFailure {»
- 79-86 «impl AgentsFailure {»
  - 80-85 «pub fn detail(self) -> &'static str {»
- 88-110 @89 «pub fn parse(bytes: &[u8]) -> Result<AgentsList, AgentsFailure> {»
  - 90-92 «if bytes.len() as u64 > MAX_FRAME {»
  - 93-95 «let Ok(Value::Array(elements)) = serde_json::from_slice::<Value>(bytes) else {»
  - 97-108 «for element in elements {»
- 112-133 «fn row(fields: Map<String, Value>) -> Option<(AgentsRow, u64)> {»
  - 115-118 «let tolerated = fields»
  - 119-131 «let row = AgentsRow {»
- 135-388 @136 «mod tests {»
  - 145-155 «fn measured() -> Value {»
  - 157-159 «fn parsed(doc: &Value) -> AgentsList {»
  - 161-178 @162 «fn agents_parse_takes_the_measured_seven_key_row() {»
  - 180-189 @181 «fn agents_parse_keeps_an_extra_key_and_counts_it_tolerated() {»
  - 191-212 @192 «fn agents_parse_lists_a_background_row_with_status_unknown_joinable_only_by_id() {»
  - 214-228 @222 «fn agents_parse_skips_and_counts_an_element_without_a_string_session_id(»
  - 230-241 @239 «fn agents_parse_of_a_document_that_is_not_one_array_is_malformed(#[case] bytes: &[u8]) {»
  - 243-254 @244 «fn agents_parse_of_an_array_past_the_default_depth_is_malformed() {»
  - 256-265 @257 «fn agents_parse_takes_max_frame_bytes_and_refuses_one_more_as_oversize() {»
  - 267-270 @268 «fn agents_parse_of_an_empty_array_is_no_rows() {»
  - 272-287 @280 «fn agents_status_maps_each_measured_word_and_nothing_else(»
  - 289-300 @294 «fn agents_parse_reads_a_pid_that_is_no_u32_as_absent(#[case] pid: Value) {»
  - 302-317 @310 «fn agents_row_joins_by_child_pid_or_session_id(»
  - 319-324 @320 «fn agents_failure_details_are_the_closed_words() {»
  - 326-334 «fn config() -> ProptestConfig {»
  - 336-359 @338 «fn arbitrary_json() -> impl Strategy<Value = Value> {»
  - 361-387 «proptest! {»
### crates/viola-agent-claude/src/ledger.rs — added 253 line(s) in 31 range(s)
added: 44 · 48 · 71 · 99 · 132 · 640-647 · 656-657 · 666 · 881-928 · 997 · 1477 · 1504 · 1533 · 1548-1549 · 1551-1553
       1557 · 1559 · 1803 · 1824 · 1980 · 2145-2150 · 2154 · 2169-2174 · 2317 · 2337 · 2728 · 2817-2971 · 2982 · 3115
       3282 · 3316
- 640-646 @643 «pub struct AgentsCapture {»
- 881-909 @883 «fn agents_joined(trusted: &[Capture], agents: Option<&AgentsCapture>) -> bool {»
  - 884-887 «let Some(AgentsCapture {»
  - 888-890 «else {»
  - 891-894 «let first = trusted»
  - 895-897 «let Ok(list) = crate::agents::parse(bytes) else {»
  - 899-908 «match (own.next(), own.next(), first) {»
- 914-927 @917 «pub fn agents_variant(agents: &AgentsCapture) -> Option<Value> {»
  - 920-925 «let own: Vec<Value> = list»
  - 2820-2833 «fn agents_row(pid: Option<u32>, id: &str, status: &str) -> Value {»
  - 2835-2840 «fn agents_of(child_pid: Option<u32>, rows: &[Value]) -> AgentsCapture {»
  - 2842-2851 @2843 «fn clean_agents() -> AgentsCapture {»
  - 2853-2861 «fn agents_check(agents: Option<AgentsCapture>) -> bool {»
  - 2863-2877 @2864 «fn check_agents_join_passes_on_the_recorded_shape() {»
  - 2879-2927 @2880 «fn check_agents_join_fails_on_each_broken_post_condition() {»
  - 2929-2949 @2930 «fn check_agents_join_fails_with_no_capture() {»
  - 2951-2970 @2952 «fn agents_variant_keeps_only_run_b_s_own_row() {»
### crates/viola-agent-claude/src/lib.rs — added 1 line(s) in 1 range(s)
added: 4
### crates/viola-e2e/src/harness/run.rs — added 55 line(s) in 10 range(s)
added: 311 · 334 · 337 · 357 · 909-932 · 934-954 · 997-998 · 1002 · 1004 · 1006-1007
### crates/viola-state/src/board.rs — added 232 line(s) in 15 range(s)
added: 4-6 · 118-119 · 123-124 · 131-132 · 138-162 · 181-202 · 240-243 · 295-298 · 323-346 · 396 · 699 · 793 · 816-855
       857-861 · 864-959
- 138-144 @141 «pub struct JoinKeys {»
- 146-153 @149 «pub struct Observation {»
- 155-160 «impl Observation {»
  - 156-159 «fn joins(&self, keys: &JoinKeys) -> bool {»
  - 186-202 @188 «pub fn unwrapped(observation: Observation) -> Self {»
- 323-345 @327 «pub fn join(board: &mut Board, observations: Vec<Observation>) {»
  - 329-336 «for item in board.items.iter_mut().filter(|item| item.wrapped) {»
  - 337-342 «let mut unwrapped: Vec<Item> = observations»
  - 826-832 «fn seen(pid: Option<u32>, session_id: &str, status: SessionStatus) -> Observation {»
  - 834-842 @835 «fn joinable(tmp: &tempfile::TempDir) -> (Board, SystemTime) {»
  - 844-850 «fn statuses(board: &Board) -> Vec<(&str, bool, &str)> {»
  - 865-878 @866 «fn board_join_by_child_pid_sets_the_status() {»
  - 880-886 @881 «fn board_join_by_session_id_sets_the_status() {»
  - 888-909 @891 «fn board_join_of_a_row_matching_neither_key_is_an_unwrapped_row() {»
  - 911-920 @912 «fn board_join_of_one_row_matching_both_keys_is_one_join_and_no_unwrapped_row() {»
  - 922-934 @923 «fn board_join_of_a_stale_row_keeps_it_stale() {»
  - 936-949 @939 «fn board_join_never_brings_back_a_gone_instance() {»
  - 951-959 @952 «fn board_unwrapped_item_serialises_to_the_contract_literal() {»
### crates/viola-state/src/liveness.rs — added 25 line(s) in 5 range(s)
added: 4 · 33-44 · 107 · 126-135 · 147
- 33-43 @35 «pub fn start_times() -> HashMap<u32, u64> {»
  - 38-42 «sys.processes()»
  - 126-134 @128 «fn start_times_hold_this_process_at_its_own_start_time() {»
### crates/viola-state/src/replay.rs — added 1 line(s) in 1 range(s)
added: 217
### crates/viola-state/src/snapshot.rs — added 5 line(s) in 2 range(s)
added: 69-72 · 191
### crates/viola-state/src/strict.rs — added 1 line(s) in 1 range(s)
added: 739
### crates/viola-state/tests/state_replay.rs — added 1 line(s) in 1 range(s)
added: 37
### fixtures/claude/2.1.287/Agents.recorded-1.json — new file · 1 line(s)
### fuzz/Cargo.toml — added 7 line(s) in 1 range(s)
added: 57-63
### fuzz/corpus/claude_agents_json/background-row — binary, not read
### fuzz/corpus/claude_agents_json/control-session-id — binary, not read
### fuzz/corpus/claude_agents_json/deep-array — binary, not read
### fuzz/corpus/claude_agents_json/empty-array — binary, not read
### fuzz/corpus/claude_agents_json/measured-row — binary, not read
### fuzz/corpus/claude_agents_json/non-array — binary, not read
### fuzz/fuzz_targets/claude_agents_json.rs — new file · 18 line(s)
- 9-18 «fuzz_target!(|bytes: &[u8]| {»
  - 10-17 «if let Ok(list) = agents::parse(bytes) {»
### schemas/claude-agents-fixture.v1.json — new file · 21 line(s)
- 1-21 «{»
  - 8-20 «"items": {»
### schemas/diag-line.v1.json — added 1 line(s) in 1 range(s)
added: 78
### src/bin/viola-fake-agent.rs — added 295 line(s) in 12 range(s)
added: 14 · 23-24 · 50-61 · 423-424 · 434-458 · 469-470 · 472 · 751-867 · 1084-1086 · 1124-1130 · 1143-1147 · 1853-1970
  - 438-448 @441 «fn launch_session(&self) -> Option<(String, String)> {»
- 751-757 @754 «struct Registry {»
- 759-796 «impl Registry {»
  - 760-779 «fn open(dir: &Path, session_id: &str, cwd: &str) -> Option<Self> {»
  - 781-790 @783 «fn set_status(&self, status: &str) {»
  - 792-795 «fn remove(&self) {»
- 798-804 @799 «fn exe_dir() -> Option<PathBuf> {»
  - 800-803 «std::env::current_exe()»
- 806-817 @808 «fn agents() -> ExitCode {»
  - 810-812 «let mode = dir»
- 819-833 @821 «fn agents_output(mode: Option<&str>, dir: Option<&Path>) -> Vec<u8> {»
  - 822-832 «match mode {»
- 835-841 @836 «fn planted_rows(dir: &Path) -> Vec<Value> {»
  - 837-840 «match fs::read(dir.join(AGENTS_ROWS)).map(|b| serde_json::from_slice::<Value>(&b)) {»
- 843-866 @846 «fn live_sessions(dir: &Path) -> Vec<Value> {»
  - 847-853 «let mut paths: Vec<PathBuf> = fs::read_dir(dir.join(SESSIONS))»
  - 856-865 «paths»
  - 1084-1086 «if args.first().is_some_and(|mode| mode == AGENTS) {»
  - 1144-1146 «if let Some(registry) = agent.registry.get() {»
  - 1854-1864 @1855 «fn agents_output_of_each_mode_is_its_bytes() {»
  - 1866-1890 @1867 «fn agents_output_recorded_lists_the_planted_rows_unchanged_then_the_live_sessions() {»
  - 1892-1926 @1895 «fn launch_session_replays_the_recorded_id_and_cwd() {»
  - 1928-1937 @1932 «fn planted_rows_of_a_malformed_rows_file_are_none(#[case] text: &str) {»
  - 1939-1970 @1942 «fn live_sessions_skip_a_dead_or_reused_pid() {»
### src/cmd/hook.rs — added 1 line(s) in 1 range(s)
added: 789
### src/cmd/hook/statusline.rs — added 1 line(s) in 1 range(s)
added: 348
### src/cmd/list.rs — added 65 line(s) in 8 range(s)
added: 1-6 · 16-17 · 89-96 · 105-120 · 154 · 156 · 193 · 238-267
  - 90-96 «if let Ok(list) = std::env::current_dir()»
- 105-119 @106 «fn observations(list: AgentsList) -> Vec<Observation> {»
  - 107-118 «list.rows»
  - 238-266 @241 «fn list_observations_take_the_session_id_pid_and_status() {»
### src/cmd/revive.rs — added 85 line(s) in 22 range(s)
added: 4-5 · 53 · 64 · 82-85 · 130-133 · 137 · 147-149 · 153-159 · 169-170 · 180 · 283-287 · 289 · 293-296 · 300-311
       316 · 325 · 329-332 · 346 · 351-355 · 360-377 · 470 · 516-521
  - 147-149 «if let Some(refusal) = session_live(&session) {»
- 153-158 @155 «fn live_elsewhere(home: &Path, cwd: &Path, id: &str) -> bool {»
  - 156-157 «crate::run::agents::read_here(home, cwd)»
### src/cmd/run.rs — added 2 line(s) in 2 range(s)
added: 281 · 565
### src/cmd/verify.rs — added 34 line(s) in 14 range(s)
added: 4-5 · 240 · 254 · 517-518 · 548-553 · 667 · 682 · 687 · 689-690 · 693-694 · 697-698 · 701-706 · 711 · 713-718
  - 548-553 «if let Some(agents) = probes.agents.as_ref() {»
### src/cmd/verify/typed.rs — added 48 line(s) in 16 range(s)
added: 5-7 · 19 · 31 · 76-78 · 82 · 84 · 89-92 · 103 · 121-123 · 129 · 147 · 149-150 · 152-156 · 161 · 365-377 · 801-807
  - 152-156 «let agents = typed»
- 365-376 @367 «fn agents_read(run: &Run, inputs: &Inputs<'_>, dir: &Path) -> Option<AgentsCapture> {»
  - 368-370 «if (run.stop)() {»
  - 372-375 «(!read.stopped).then(|| AgentsCapture {»
  - 801-806 «fn child_pid(&self) -> Option<u32> {»
### src/human/board.rs — added 137 line(s) in 12 range(s)
added: 2-3 · 144-149 · 151-152 · 154 · 160-166 · 170 · 184-187 · 194-195 · 197-198 · 201-204 · 367 · 507-611
  - 201-204 «if wrapped < board.items.len() {»
  - 507-513 «fn unwrapped(name: &str, status: viola_state::board::SessionStatus) -> Item {»
  - 515-531 @519 «fn board_row_status_cell_is_the_status_word(»
  - 535-552 @536 «fn board_row_the_separator_and_an_unwrapped_row_are_the_literals() {»
  - 554-579 @557 «fn board_row_an_unwrapped_name_with_controls_prints_on_one_line_in_the_captions_columns() {»
  - 581-599 @582 «fn board_row_an_empty_wrapped_rack_with_unwrapped_rows_keeps_the_empty_board_line() {»
  - 601-610 @602 «fn board_row_no_separator_without_an_unwrapped_row() {»
### src/run/agents.rs — new file · 174 line(s)
- 18-25 @21 «pub(crate) enum ReadFailure {»
- 27-34 @30 «pub(crate) struct AgentsRead {»
- 36-96 @39 «pub(crate) fn read_logged(»
  - 48-56 «let ran = run_capped(»
  - 57-61 «let status = ran»
  - 62-68 «obs_event!(»
  - 69-71 «let Ok(ran) = ran else {»
  - 72-77 «if ran.stopped || !ran.status.is_some_and(|s| s.success()) {»
  - 78-90 «let list = agents::parse(&ran.stdout).map_err(|failure| {»
  - 91-95 «AgentsRead {»
- 98-104 «fn unread(stopped: bool) -> AgentsRead {»
  - 99-103 «AgentsRead {»
- 106-116 @109 «pub(crate) fn read_here(home: &Path, cwd: &Path) -> Result<AgentsList, ReadFailure> {»
  - 110-112 «let Ok(program) = super::resolve_program(OsStr::new(PROGRAM), cwd) else {»
- 118-174 @119 «mod tests {»
  - 122-136 @125 «fn read_logged_of_a_program_whose_output_is_no_array_is_malformed() {»
  - 138-149 @139 «fn read_logged_of_a_program_that_cannot_start_is_unreadable() {»
  - 151-158 @152 «fn read_logged_with_the_stop_set_reads_stopped_and_unreadable() {»
  - 160-173 @163 «fn read_logged_of_a_failing_program_is_unreadable() {»
### src/run/budget.rs — added 1 line(s) in 1 range(s)
added: 653
### src/run/dialog.rs — added 1 line(s) in 1 range(s)
added: 474
### src/run/links.rs — added 1 line(s) in 1 range(s)
added: 136
### src/run/mod.rs — added 1 line(s) in 1 range(s)
added: 4
### src/run/send.rs — added 1 line(s) in 1 range(s)
added: 36
### src/run/send/slot.rs — added 83 line(s) in 7 range(s)
added: 35 · 53 · 74-78 · 175-177 · 208-213 · 249 · 406-471
  - 74-77 @75 «pub(crate) fn snapshots_with(&self, snapshots: Arc<Snapshots>) {»
  - 208-210 «let session = (line.kind == EventKind::SessionStart)»
  - 211-213 «if let (Some(id), Some(snapshots)) = (session, slot.snapshots.get()) {»
  - 406-424 «fn snapshot_of(dir: &Path) -> InstanceSnapshot {»
  - 426-456 @429 «fn send_session_start_records_its_string_id_in_the_snapshot() {»
  - 458-470 @460 «fn send_session_start_without_the_snapshot_handle_is_appended() {»
### src/run/snapshot.rs — added 1 line(s) in 1 range(s)
added: 74
### src/run/version_gate.rs — added 44 line(s) in 11 range(s)
added: 48-61 · 74-75 · 105 · 122 · 127 · 275-280 · 332-337 · 342 · 348 · 362-367 · 375-379
### src/run/wheel.rs — added 2 line(s) in 2 range(s)
added: 834 · 984
### tests/cli_list.rs — added 234 line(s) in 3 range(s)
added: 1-9 · 18-19 · 258-480
- 258-267 @260 «fn boot_joined(stamped: StampedHome) -> (Wrapper, std::path::PathBuf) {»
  - 262-265 «let joined = StampedHome {»
- 269-272 @270 «fn scratch(wrapper: &Wrapper) -> &Path {»
- 274-276 «fn status_on(bin: &Path, home: &Path) -> Value {»
- 278-307 @281 «fn list_status_of_a_joined_wrapper_is_idle_then_busy(stamped_home: StampedHome) {»
  - 288-293 «let sent = viola(»
  - 299-305 «assert_eq!(»
- 313-357 @314 «fn list_prints_planted_rows_as_unwrapped_rows(stamped_home: StampedHome) {»
  - 317-322 «let bin = fake::claude_bin(»
  - 325-335 «assert_eq!(»
  - 338-355 «assert_eq!(»
- 359-379 @361 «fn e1_send_naming_an_unwrapped_row_is_unreachable(stamped_home: StampedHome) {»
  - 364-369 «let bin = fake::claude_bin(»
  - 374-377 «assert_eq!(»
- 381-407 @386 «fn list_of_an_answer_that_does_not_parse_equals_the_no_claude_board(»
  - 395-401 «for args in [&["list", "--json"][..], &["list"][..]] {»
- 409-468 @415 «fn list_under_a_name_logs_the_agents_read(»
  - 422-427 «let bin = fake::claude_bin(»
  - 428-434 «document(&viola_on(»
  - 436-439 «let probe: Vec<&Value> = lines»
  - 445-448 «let parse: Vec<&Value> = lines»
  - 449-457 «match rejected {»
  - 460-466 «for line in probe.iter().chain(&parse) {»
- 470-479 @472 «fn list_without_a_name_logs_no_agents_read(stamped_home: StampedHome) {»
### tests/cli_revive.rs — added 125 line(s) in 7 range(s)
added: 2 · 5-6 · 21-22 · 43-45 · 72 · 74-78 · 425-535
- 43-45 «const SESSION_LIVE: &str = "unable: builder's session is open in another claude\n\»
  - 74-78 «assert_eq!(»
- 425-432 @426 «fn plant_agents(home: &TestHome, mode: Option<&str>, rows: &str) {»
  - 428-430 «if let Some(mode) = mode {»
- 434-442 @435 «fn listed(id: &str) -> String {»
  - 436-440 «serde_json::json!([{»
- 444-457 @445 «fn assert_agents_read(home: &Path, before: usize) {»
  - 447-450 «let probe: Vec<&Value> = lines[before..]»
  - 454-456 «for line in probe {»
- 459-485 @462 «fn revive_of_a_session_open_in_another_claude_is_session_live(stamped_home: StampedHome) {»
  - 471-476 «assert!(»
  - 482-484 «for line in &role_lines(home.path())[before..] {»
- 487-513 @491 «fn revive_passes_the_session_live_reading(»
  - 501-511 «if flags.is_empty() {»
- 515-534 @519 «fn revive_whose_agents_read_fails_starts(stamped_home: StampedHome, #[case] mode: &str) {»
  - 526-530 «let rejected: Vec<Value> = role_lines(&home)[before..]»
### tests/cli_verify.rs — added 108 line(s) in 33 range(s)
added: 3 · 28-51 · 54-74 · 77 · 100 · 103 · 123 · 127 · 129 · 251-252 · 264 · 266 · 440-441 · 492-494 · 496 · 525-526
       537 · 541 · 618 · 653-668 · 1098 · 1195-1202 · 1207-1209 · 1232-1234 · 1244 · 1246 · 1288-1289 · 1299 · 1301
       1356 · 1358 · 1434 · 1436
  - 1196-1201 «expected.extend([»
### tests/contract_fake_agent_drift.rs — added 98 line(s) in 2 range(s)
added: 7-10 · 438-531
- 441-452 @442 «fn type_word(value: &Value) -> &'static str {»
  - 443-451 «match value {»
- 454-464 @455 «fn shape(row: &Value) -> Vec<(String, &'static str)> {»
  - 456-461 «let mut keys: Vec<(String, &'static str)> = row»
- 466-530 @470 «fn fake_agent_agents_replay_matches_every_recorded_agents_set() {»
  - 473-528 «for version in recorded_versions(&root) {»
### tests/contract_fixture_hygiene.rs — added 113 line(s) in 7 range(s)
added: 3-4 · 37-40 · 55-60 · 64-72 · 144-230 · 320 · 348-351
- 37-39 «fn agents_schema() -> Value {»
- 55-57 «fn is_agents(path: &Path) -> bool {»
- 144-159 @145 «fn agents_fixtures_pass_hygiene() {»
  - 150-158 «for path in walked {»
- 161-175 «fn agents_row(extra: &Value) -> Value {»
  - 162-170 «let mut row = json!({»
  - 171-173 «if let (Some(row), Some(extra)) = (row.as_object_mut(), extra.as_object()) {»
- 177-190 @180 «fn planted_clean_agents_read_passes(#[case] extra: Value) {»
  - 182-189 «assert_eq!(»
- 192-205 @200 «fn planted_agents_schema_violation_is_rejected(#[case] doc: Value) {»
  - 201-204 «assert_eq!(»
- 207-216 @210 «fn planted_agents_absolute_path_is_rejected(#[case] extra: Value) {»
  - 212-215 «assert_eq!(»
- 218-229 @219 «fn planted_agents_username_is_rejected() {»
  - 221-228 «assert_eq!(»
  - 348-351 «assert_eq!(»
### tests/contract_ledger_probes.rs — added 6 line(s) in 6 range(s)
added: 17 · 40 · 43 · 63 · 134 · 136
### tests/cross_list_rows.rs — new file · 142 line(s)
- 26-32 «const BOARD: &str = "BAY  5H unknown  7D unknown  read unknown  gate open  skipped 0 - 0 - 0\n\»
- 34-47 @36 «fn assert_columns(board: &str) {»
  - 39-46 «for row in lines[2..].iter().filter(|l| !l.starts_with("--")) {»
- 49-55 «fn assert_board(board: &str) {»
  - 52-54 «for line in board.lines() {»
- 57-73 @58 «fn piped(bin: &Path, home: &Path, env: &[(&str, &str)]) -> (Option<i32>, String) {»
  - 59-67 «let out = Command::new(VIOLA)»
  - 69-72 «(»
- 75-142 @76 «fn cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode(stamped_home: StampedHome) {»
  - 92-96 @93 «let wide = Size {»
  - 106-121 @109 «{»
  - 126-131 «let names: Vec<&Value> = doc["ok"]["items"]»
  - 132-140 «assert_eq!(»
### tests/support/cli.rs — added 41 line(s) in 4 range(s)
added: 5-6 · 71-72 · 74-107 · 124-126
  - 74-75 «let bin = (args.first() == Some(&"list"))»
- 79-88 @80 «pub fn spawn_on(»
- 90-99 @91 «pub fn viola_on(»
  - 124-126 «if let Some(bin) = bin {»
### tests/support/fake.rs — added 31 line(s) in 1 range(s)
added: 26-56
- 26-31 @27 «pub fn empty_bin(scratch: &Path) -> PathBuf {»
- 33-50 @36 «pub fn claude_bin(scratch: &Path, name: &str, mode: Option<&str>, rows: Option<&[u8]>) -> PathBuf {»
  - 40-42 «if !claude.is_file() && fs::hard_link(FAKE, &claude).is_err() {»
  - 43-45 «if let Some(mode) = mode {»
  - 46-48 «if let Some(rows) = rows {»
- 52-55 @53 «pub fn claude_in(dir: &Path) -> PathBuf {»
### tests/support/home.rs — added 11 line(s) in 4 range(s)
added: 362-367 · 370 · 381-383 · 392
  - 362-367 @363 «let failing: Vec<&str> = stdout»
