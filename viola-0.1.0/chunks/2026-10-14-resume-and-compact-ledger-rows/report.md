# Report — 2026-10-14-resume-and-compact-ledger-rows

**Chunk:** `claude --resume <id>` and `/compact` as probed, stamped capability-ledger rows (v1-34)
**Date:** 2026-10-16T14:08Z
**Commits:** `784d9f9 chore(2026-10-14-resume-and-compact-ledger-rows): operator pre-CI commit, for the run this chunk's
verdict reads` (the only commit since `last_wrap` 2026-10-14T07:06:24Z; `git log --format='%h %s' 880b0563..HEAD`)

## Changes (structured — detectors read this)
- **Files:** (`gate.py scope`: changed 16 · listed 16, base 880b0563)
  - `crates/viola-agent-claude/src/ledger.rs`
  - `src/run/send/classify.rs` (unit rows only)
  - `src/cmd/verify/typed.rs`
  - `src/cmd/verify.rs`
  - `src/bin/viola-fake-agent.rs`
  - `src/run/version_gate.rs` (unit oracle only)
  - `crates/viola-e2e/src/harness/run.rs`
  - `tests/cli_verify.rs`
  - `tests/contract_ledger_probes.rs`
  - `tests/contract_fixture_hygiene.rs`
  - `tests/contract_fake_agent_drift.rs`
  - `tests/cli_fake_agent.rs`
  - `tests/cli_send.rs`
  - `tests/support/verify.rs`
  - `tests/support/home.rs`
  - New: `fixtures/claude/2.1.287/SessionStart.resume-1.json` (578 B, one line ending in LF, recorded live).
  - New: chunk `evidence/` (`live-round.md`, `round-071816Z.txt`, `round-071942Z.txt`, `operator-pass.md`) and
    `scope-record.md`.
  - Untouched, as the preservation guard proves: `src/run/send/slot.rs`, `crates/viola-agent-claude/src/screen.rs`,
    every manifest and lockfile.
- **Symbols / APIs:**
  - `LedgerRow::ResumeSession` (`ledger.rs:43`) is the 22nd and last member of `LedgerRow::ALL`, which becomes
    `[Self; 22]` (`:47`, `:69`).
    - Its id is `resume-session` (`:96`) and its words are `--resume <id> reopens the session under that id`
      (`:128`).
    - `relied_on` leaves it under `_ => true`, so it is relied on every OS.
  - `Probes::resumed: Vec<Capture>` (`ledger.rs:652`) holds Run E's captures in claim order.
  - `pub fn resume_target(trusted: &[Capture]) -> Option<&str>` (`ledger.rs:842-850`, head `:844`).
    - It returns the `session_id` of Run B's FIRST SessionStart capture, and only when `crate::is_session_id`
      accepts it (`:849`).
    - Two callers: `typed.rs:244` (Run E) and the ledger's own `session_resumed`.
  - Private helpers: `resumed_start` (`:852-855`) and `session_resumed` (`:857-865`).
    - The check arm is `LedgerRow::ResumeSession => session_resumed(&probes.trusted, &probes.resumed)` (`:934`).
    - It passes only when Run E's first SessionStart has `source` `resume` and the target's id.
  - `pub fn resume_variants(resumed) -> Vec<(&'static str, &Capture)>` (`ledger.rs:867-873`) names that SessionStart
    `resume-1`. Its sole caller is `verify.rs:540`.
  - `LOCAL_COMMANDS: [(&str, Option<PostCondition>); 3]` (`ledger.rs:197`) appends `("/compact", None)` (`:200`).
    - `PROBE_LOCAL_COMMAND` is still `LOCAL_COMMANDS[0].0` (`/clear`), the only entry with a post-condition.
    - `classify` reads the list unchanged, so `/compact` maps to `Confirms::Nothing` on a verified and an unverified
      CLI alike. No product line of `classify.rs` changed.
  - Run E, `fn resume_run` (`src/cmd/verify/typed.rs:235-266`, head `:238`):
    - `measure` runs it as its fifth `RunStep`, after `plan_run` (`:89`). `Inputs` gains
      `pub(super) resume: RunDirs<'a>` (`:68`).
    - It reads Run B's captures through `super::read_captures` (`:243`, now a second caller of that private fn) and
      takes the id from `ledger::resume_target`. With no target it spawns nothing and returns `Ok(())`, so the row
      fails.
    - It runs in `TrustedDir::create(inputs.cwd, "-resume")` (`:247`): `<cwd>/.viola-verify-<pid>-resume/`, 0700,
      removed by its guard on every exit path.
    - Its argv is the run args, then `--plugin-dir` and the resume plugin, then
      `viola_agent_claude::resume_args(id, false)` (`:251`). That is a second caller of `resume_args` beside
      `viola revive`.
    - It is spawned through `Run::spawn` with input `io::empty()`, so no key is ever typed and there is no
      `--settings`.
    - It waits for one SessionStart capture up to `probe_until`, settles after it, then `end_by_kill`.
  - `src/cmd/verify.rs`:
    - `ProbeDir::resume()` (`:357-359`) makes `resume/`, holding the same capture plugin as `typed/` and its
      `captures/` (`:214-219`, `:314`).
    - `Probes.resumed = read_captures(resume)` (`:252`).
    - `named_payloads` adds `SessionStart.resume-1.json` from `ledger::resume_variants`, scrubbed by the existing
      scrub (`:540-545`).
    - The `verify --help` text names no run count and is unchanged.
  - Fake agent:
    - `RESUME_VARIANT = "resume-1"` (`src/bin/viola-fake-agent.rs:40-41`).
    - `Agent::launch_variant` (`:437-445`, head `:439`): under `--resume`, the launch SessionStart's base is the
      set's recorded `SessionStart.resume-1.json` when it exists, else `default` as before.
    - `resume_fields` is unchanged in code; only its doc moved (`:158-160`). It still sets `source` `resume` and
      `session_id` to the given id, or the compiled fork id under `--fork-session`.
    - No option was added, so the count stays twelve.
- **Crates / modules:** none added or removed. Changed: `viola-agent-claude` (`ledger`), `viola` (`cmd::verify`,
  `cmd::verify::typed`, the fake-agent bin), `viola-e2e` (`harness::run` literals).
- **Dependencies:** none (`Cargo.toml` / `Cargo.lock` untouched; preservation guard green).
- **Schema / config:**
  - **New fixture variant stem** `SessionStart.resume-<n>.json`, first in the 2.1.287 set.
    - 10 key names: `context_tokens`, `cwd`, `estimated_cache_write_usd`, `hook_event_name`,
      `prompt_cache_likely_expired`, `scratchpad_dir`, `seconds_since_last_response`, `session_id`, `source`,
      `transcript_path` (`evidence/live-round.md`).
    - It is scrubbed (home paths `~`) and walked by the hygiene contract.
  - **Probe dirs:** the verify probe dir gains `resume/` (plugin + `captures/`), and a new interactive dir
    `<cwd>/.viola-verify-<pid>-resume/`.
  - No config key, env var, event kind, `detail` value, schema file or exit code changed.
- **Spec-master edits:** none (the masters equal HEAD; the site sweep's pre-pass baseline `784d9f99`).
- **Counts / qualifiers moved** (site lists: the site sweep, `report-patterns.toml`; `cascade.py sweep --pre-pass`
  rows, every row `standing` against HEAD `784d9f99`):
  - **Ledger rows** 21 → 22. The relied set (`LedgerRow::relied()`) moves 21 → 22 on Unix and 18 → 19 on Windows.
    - `viola verify`'s `[NN/MM]` reads MM = 22 / 19, and the stamped line reads `stamped <ver>  22 pass  0 fail` on
      Unix and `19 pass  0 fail` on Windows.
    - Every home stamped before this chunk reads `cli_verified:false` until it is re-stamped. An all-`pass` older
      stamp of 21 (Unix) or 18 (Windows) ids reads unverified. An 18-plus-resume stamp verifies on Windows only
      (`version_gate.rs` oracle).
    - Sites, pattern `rows21` (30 standing):
      - architecture.md:91, :253, :422, :473;
      - security-plan.md:617;
      - design-system.md:811;
      - layout-templates.md:476–493 (the `viola verify` example block, 17 rows) and :504;
      - test-plan.md:568, :657, :667;
      - obs-plan.md:739;
      - registries: `contracts/architecture/ci-cd-approach.md:20`,
        `contracts/architecture/crate-dependency-direction.md:6`, `contracts/test-plan/5-command-implementation.md:8`
        and `:53`.
    - Sites, pattern `rows18` (11 standing): architecture.md:91, :253, :422, :473; design-system.md:811;
      layout-templates.md:504; test-plan.md:568, :657, :667; obs-plan.md:739;
      `contracts/architecture/ci-cd-approach.md:20`.
    - Pattern `rows21` reads 0 rows in a11y-plan.md: it says `[NN/NN]` generically (a11y-plan.md:349).
  - **verify's interactive runs** move 4 → 5 (Run E). Live starts per verify round move 5 → 6.
    - Sites, pattern `verify-runs` (22 standing):
      - architecture.md:18, :91, :372, :406, :412, :413;
      - security-plan.md:131, :252, :282, :283, :291;
      - test-plan.md:408, :657, :1094;
      - obs-plan.md:739, :742, :871;
      - registries: `contracts/architecture/crate-dependency-direction.md:6`,
        `contracts/architecture/project-directory-structure.md:31`, `contracts/test-plan/5-command-implementation.md:8`
        and `:53`, `contracts/test-plan/bootstrap-phases-derive-for-route-setup-project.md:10`.
  - **Spawn pairs** of an uninterrupted `viola verify` move 6 → 7: `version-probe`, `verify-probe` and five
    `verify-pty-probe`.
    - Sites, pattern `spawn-pairs` (8 standing):
      - architecture.md:406 ("six spawn pairs");
      - security-plan.md:131;
      - test-plan.md:657;
      - obs-plan.md:739, :820, :836, :871, :872.
  - **The 2.1.287 fixture set** moves 31 → 32 files (`ls fixtures/claude/2.1.287 | wc -l` = 32, `live-round.md`).
    - Sites, pattern `fixtures31`: architecture.md:422 ("31 files"). The test-plan.md:1094 row is the resume payload's
      "10 key names", a true claim.
  - **`LOCAL_COMMANDS`** moves 2 → 3 entries (`/clear`, `/remote-control`, `/compact`).
    - Sites, pattern `local-cmd` (12 standing):
      - architecture.md:49, :88, :91;
      - security-plan.md:600;
      - test-plan.md:233, :584, :755;
      - obs-plan.md:312, :313, :631;
      - registries: `contracts/architecture/crate-dependency-direction.md:6`,
        `contracts/architecture/project-directory-structure.md:37`.
    - Pattern `compact`: architecture.md:473 alone, the owed-row sentence.
  - **The fake agent's 2.1.287 resume launch payload** moves from 5 key names (rewritten from `SessionStart.default`)
    to 10 (the recorded variant). A set without the variant (2.1.288) keeps the default-based payload.
    - Sites, pattern `fake-resume` (3 standing): architecture.md:362; test-plan.md:1077, :1094.
  - **"One relied-on dependence has no row"** (`--resume`) is no longer true.
    - Sites, pattern `resume-norow` (2 standing): architecture.md:473 (×3); test-plan.md:1094 (the route entry's
      title).
  - **The Session id as a child argv value row** gains a third id source: Run B's first SessionStart capture, inside
    verify.
    - Sites, pattern `sid-argv` (2 standing): security-plan.md:129, :246.
  - **Probe dirs** gain `-resume`.
    - Sites, pattern `probe-dirs` (7 standing): architecture.md:91, :100, :372, :413; security-plan.md:131, :283,
      :291.
  - **Accepted-risk transcripts:** each round adds four CLI transcript files (Run B ×2, Run C, Run D). Run E adds
    none, although its payload names a `-resume` project path (643 → 647 → 651, `live-round.md`).
    - Sites, pattern `transcripts`: security-plan.md:291, "Accepted risk (verify's trusted runs, B, C and D)". The
      other 6 rows are other accepted risks with no change.
  - **Test-literal readers moved**, none in a master: `dialogless_home` 17/14 → 18/15 pass, and the receipt counts
    (env 5 → 6, hooks 37 → 38).
- **Dev-tool versions:** none. `claude` 2.1.287 was re-read by path (the non-priming probe, `test -x` green). Rust
  1.98.1 and every CI pin are unchanged.
- **Harness / gate surface:**
  - `crates/viola-e2e/src/harness/run.rs`:
    - `LEDGER_ROWS` is `[&str; 22]` under `cfg(unix)` (`:335`) and `[&str; 19]` under `cfg(not(unix))`
      (`:357`, `:360`).
    - `LIVE_PASS` names the `resume-session` line (`:944-987`).
  - `boot`'s `viola verify` stamp against the fake agent now runs Run E: print probe and five interactive runs, every
    relied row (22 Unix / 19 elsewhere).
  - No command, status shape or log format changed.
- **Cross-project / external claims:**
  - **CI:** `ci#38868756782` on `784d9f9937d6` reads `verdict: green · checks 15/15 · wall 922 s`, the
    `windows-2025` leg included (`evidence/operator-pass.md`; `ci.py conclusion --sha HEAD --wait 1800`). The sha is
    the pre-CI commit's, and this wrap's commit adds only bookkeeping.
  - **The 2.1.287 CLI, live:** `--resume <id>` found Run B's session from Run E's own `-resume` project dir, the
    cross-directory scan that P5 read from the CLI's code. It is now measured (`live-round.md`).
  - `inputs.py verify` (fired before this report): `inputs: 5 entries — … n/a 5 · uncited 1 · unparsed 0`. The
    uncited one was I5, which this report cites.
    - I1 · message (the operator, relaying the founder live, 2026-10-14T07:31:48Z) · copy · n/a — the forks.
    - I2 · message (the operator, given here, 2026-10-16T06:44Z) · copy · n/a. Its fact names the P5 review card
      `.andromeda/runs/2026-10-14T07-11-44-phase/p5-card-1.md`, answered «yes» (the operator's own word). The card's
      `## Beyond the plan` rows, verbatim:
      - `| 1 | The `/compact` post-condition row (cause `compact`, same id, a confirm window of its own) stays owed. No entry is minted by this phase. | Its owner is named by the operator at this chunk's wrap (fork 1, `inputs#I1`). |`
      - `| 2 | The Windows reading of `resume-session` (a Windows stamp at 19 relied rows) is not measured by this chunk. | The existing entry "Windows-only live measurements" (its `BLOCKED-ON` standing), as a CARRY at this chunk's wrap. |`
    - The revision's card (`.andromeda/runs/2026-10-16T07-29-18-phase/p5-card-1.md`, `## Beyond the plan`: none) is
      named by no inputs entry, and its cause is `inputs#I4`. So I2's card is the one with rows.
    - I3 · message (the overseer, founder-delegated, 2026-10-16) · copy · n/a — re-plan the guard, with no commit or
      push.
    - I4 · message (the operator, the `/andromeda-phase` invocation, 2026-10-16 07:29Z) · copy · n/a — the plan
      revision.
    - I5 · message (the operator, given here as the `/andromeda-implement` invocation arguments, 2026-10-16 07:35Z)
      · copy · n/a. Snapped at this wrap (`inputs#I5`): the authority on which the session fired the operator pass
      (hygiene, the pre-CI commit, the push, the CI read).
- **Reverted / negative API facts:**
  - The new stop case first asserted that Run E's killed exit logs no `child_exit_status`. That read red, so the
    assertion was dropped and the case asserts `duration_ms` only, as Run B's stop case already did (see Spec claims
    disproved).
  - P5 dropped the plan's `pub(super)` change to `read_captures` before the code was written. Its visibility is
    unchanged.
- **Insufficient fixes:** none.
- **Spec claims disproved by measurement:**
  1. **The claim:** "a child killed at the stop closes with `duration_ms` and no `child_exit_status`".
     - Sites, pattern `killed-exit` (3 standing): architecture.md:406, obs-plan.md:739, obs-plan.md:872. Leaves:
       `.claude/rules/observability.md:39`, `.claude/docs/services/viola.md:35`.
     - **Measured:** verify's interactive PTY runs close a child killed at the stop with the pump's exit code as
       `child_exit_status`. portable-pty reports a code for a signalled child.
     - **Evidence:** the stopped implement run's integration red at 07:20Z, its journal, and the new stop case
       `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` (`tests/cli_verify.rs:1071-1178`).
     - **Scope:** pre-existing behaviour, not this chunk's code. Run B's stop case had never asserted it.
- **Expected amendments (from plan):** the site sweep (`report-patterns.toml`, each pattern's `standing` rows above;
  registry files read with the masters).
  - architecture [CLI Version Compatibility] (22 rows, relied 22/19, Run E in the verify child set) → **carried**:
    Counts bullets "Ledger rows" and "verify's interactive runs". `rows21`: arch 4 lines plus 2 key files;
    `verify-runs`: arch 6 lines.
  - architecture Capability ledger (`--resume` is no longer "the one relied-on dependence with no row") → **carried**:
    Counts "One relied-on dependence…". `resume-norow`: arch :473.
  - architecture [Delivery Confirmation] (`/compact` listed, no post-condition; its row owed, owner named by the
    operator at the wrap) → **carried**: Symbols `LOCAL_COMMANDS`; Counts `LOCAL_COMMANDS`. `local-cmd`: arch :49,
    :88, :91; `compact`: arch :473. The owner of the owed row is the I2 card's row 1, which this wrap asks the
    operator.
  - architecture §Occupied Resources (Repository: 2.1.287 at 32 files; the fake agent's resume base) → **carried**:
    Counts "2.1.287 fixture set" and "fake agent's … resume launch payload". `fixtures31`: arch :422;
    `fake-resume`: arch :362.
  - security-plan §Threat Model Summary (the verify child set: five interactive runs, Run E's `--resume` and its id
    source; the founder's own ratification, `inputs#I1` fork 2) → **carried**: Symbols Run E; Counts "verify's
    interactive runs". `verify-runs`: security :131, :252, :282, :283, :291.
  - security-plan §Input Validation, Session id as a child argv value (a third source) → **carried**: Counts "Session
    id as a child argv value row". `sid-argv`: security :129, :246.
  - security-plan §Data Protection (the `-resume` probe dir; the accepted-risk transcript count; relied 22/19) →
    **carried**: Counts "Probe dirs", "Accepted-risk transcripts" and "Ledger rows". `probe-dirs`: security :131,
    :283, :291; `transcripts`: security :291; `rows21`: security :617.
  - test-plan §1, §3 `run` / `boot`, §4, §5, §7 (the literals, Run E, the recorded resume base; the rewrite retired
    for 2.1.287 and kept for a set without the variant) → **carried**: Counts and Harness bullets.
    - `rows21`: test-plan :568, :657, :667 plus `5-command-implementation.md:8`, `:53`.
    - `verify-runs`: test-plan :408, :657, :1094.
    - `fake-resume`: test-plan :1077, :1094.
  - obs-plan §4 Capability ledger → `verify` and §1 (seven spawn pairs, MM 22/19) → **carried**: Counts "Spawn
    pairs". `spawn-pairs`: obs :739, :820, :836, :871, :872; `rows21`: obs :739.
  - design-system §Surface: cli pattern 5 (the `resume-session` step line, the counts) and pattern 2 (`/compact`
    reads `unconfirmable`) → **carried**: Counts "Ledger rows"; Symbols `LOCAL_COMMANDS`.
    - `rows21`: design :811.
    - `local-cmd`: 0 rows in design-system.md. Pattern 2's wording is for the design detector to read.
  - layout-templates §Output structure — `viola verify` (the example block, the ledger-order list, 22/19) and
    §Surface: cli, Signature placement (`/compact`) → **carried**: Counts "Ledger rows".
    - `rows21`: layout :476–493, :504.
    - `local-cmd`: 0 rows in layout-templates.md.
  - a11y-plan §4 P6 (the count) → **not carried: no count there.** `rows21` and `rows18` read 0 rows in a11y-plan.md,
    whose step-line wording is the generic `[NN/NN]` (a11y-plan.md:349). This chunk moved no value it states.
  - `matrix#v1-34` notes ("not claimed") → **written at P5**: the phase run's `matrix-…json` trail
    (`.andromeda/runs/2026-10-14T07-11-44-phase/`). This wrap writes nothing for it.
- **Coverage of new surfaces:**
  - **Run E child spawn** (`resume_run`) → validation `is_session_id` filter in `resume_target` + `resume_args`✓ ·
    instrumentation `verify-pty-probe` spawn pair✓ · PII the recorded payload scrubbed (`--record` scrub, home paths
    `~`)✓; no id or path in any line✓ · tests unit (5 `check_resume_session_*`, `resume_target_*`,
    `resume_variants_*`) / integ (`cli_verify` stop table, spawn pairs, summaries) / live (record round)✓ · a11y n/a ·
    tokens n/a.
  - **The `resume-session` step line** (CLI output) → validation n/a · instrumentation n/a (stdout result) · PII none
    (compiled words)✓ · tests integ (`cli_verify`, `contract_ledger_probes`)✓ · a11y ASCII-only static line, as the
    other step lines✓ · tokens n/a.
  - **`send` of `/compact`** (an existing surface, a new list member) → validation exact-match local-command
    list✓ · instrumentation `send-issued` then `send-confirmed{confirmed:false}`✓ · PII n/a · tests unit
    (`classify` rstest rows) / integ (`send_compact_on_a_verified_cli_is_unconfirmable_at_once`,
    `tests/cli_send.rs:735-769`)✓ · a11y `[  ] unconfirmable` line✓ · tokens n/a.
  - **The fake agent's resume variant** (test-only) → tests `fake_resume_launches_on_the_recorded_resume_variant_…`
    (`viola-fake-agent.rs:1356-1378`), `fake_agent_resume_launches_on_the_recorded_resume_variant`
    (`tests/cli_fake_agent.rs:533-565`), `fake_agent_resume_replay_matches_every_recorded_resume_set`
    (`tests/contract_fake_agent_drift.rs:415-434`) and `claude_resume_variants_are_walked_for_every_resume_set`
    (`tests/contract_fixture_hygiene.rs:533-564`)✓. The other flags are n/a.

## Deviations from intent
- **The test set id.** `tests/support/verify.rs`'s synthetic session id `s-verify-1` became `SESSION_ID`, a literal
  of the session-id shape. `resume_target` refuses an id off the shape, so under the old literal Run E never started.
  `verify_started` also gained an `after` parameter for the new stop case. Plan step 9 named this file only "if
  needed"; the revision recorded it.
- **Run-count readers outside step 9's named sites moved**, measured at implement and named by the revision (step 9's
  last bullet):
  - stop-less set 5/17 (Windows 5/14);
  - no-screens 7/15 (Windows 7/12);
  - `trusted_run_typed` 6 starts;
  - the run-dir case: 6 starts passing and 5 failing;
  - the record file list;
  - the dispatch-error spawn list (+1 pair; the test renamed `…_seven_spawn_pairs`);
  - the `cli_fake_agent` resume and fork cases on the `resume-1` base.
- **Three readers caught by the gates:**
  - a second `[true; 21]` verdict vector in `ledger.rs` (unit entry);
  - env and hook receipt counts 5/37 → 6/38 (integration entry);
  - `dialogless_home` 17/14 → 18/15 (`pre-push`, outside the integration filter).

  The last made the preservation guard red. It is **absorbed by `.andromeda/runs/2026-10-16T07-29-18-phase/`**, the
  plan revision on the operator's word (`inputs#I4`), which added the file to the guard's exclusions and to step 9.
- **Summary forecasts** (step 9), measured: no-dialogs 18/4 (Windows 15/4), no-framing 18/4 (Windows 18/1), tag modal
  20/2 (Windows 17/2). CI's `windows-2025` leg read the Windows halves green.
- **Step 12, CI sizing:** `verify_window_without_screens_fails_every_interactive_row` went 34.42 s → 42.98 s locally.
  Run E's settle waits out `GATE_MAX_WAIT` with no screen, by design. The class's CI kill is 60 s and the mutants
  profile's is 45 s, which leaves about 2 s of margin locally. No kill moved: plan step 12 allows a move only under the
  by-design-floor rule with a planted-hang control.
- **The operator pass** (`inputs#I5`, `evidence/operator-pass.md`):
  - **Hygiene.** The first read was refused on three files. Each hit was the prose glob `/tmp/viola-verify-*`, which
    was rewritten to `/tmp/viola-verify-<suffix>` in `.andromeda/runs/2026-10-14T07-11-44-phase/p5-card-1.md`, the
    stopped run's `journal.md` and `evidence/live-round.md`. The re-read was clean.
  - **`pre-push` re-run.** It was re-run on the committed tree before the push (green, 72.46 s), following entry
    23's note.
  - **The CI read** ran in the background under `timeout 1980`, the entry's own bound, with its output redirected to
    the session scratchpad. That is the as-written call plus a bound.
- **The stop and the revision:**
  - The stopped implement run (`2026-10-16T06-46-40-implement`) halted at 07:00Z on the founder's word, relayed by
    the operator: an account switch at 99%. It resumed on his word.
  - It stopped again on entry 15 (`inputs#I3`). The revision followed (`inputs#I4`), then the re-entry
    (`2026-10-16T07-35-58-implement`).
- **Scope record:**
  - The record holds one line, `- `tests/support/home.rs` · companion · serves step 9 · self`.
  - Since the revision, research lists that file, so the read prints `record: listed — tests/support/home.rs`.
  - The P1 read: `scope: clean — changed 16 · listed 16 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 ·
    widening 0) · absorbed 0 · excluded 87`, base 880b0563.

## Decisions & corrections
- **The founder's word, relayed by the operator** (the stopped run's journal, 2026-10-16T07:00Z): stop mid-run for an
  account switch at 99%. The stop was journaled with no edit in flight, and the run resumed on his word ("finish
  implement, stop before the wrap").
- **The overseer, founder-delegated** (`inputs#I3`): entry 15's red is the guard's own exclusion list, not the code.
  Re-plan the guard, with no commit or push.
- **The operator** (`inputs#I4`): revise the plan so the guard excludes `tests/support/home.rs`, then re-enter at entry
  15. No live start remained.
- **The operator** (`inputs#I5`): re-enter, run fmt and entry 15 onward, then the operator pass with the `ci.py
  conclusion` read, and stop ready for the wrap. The session made the pre-CI commit `784d9f99` and the push on this
  word.
- **The operator, at this wrap's invocation** (given here, 2026-10-16):
  - "The 85 % and 90 % context alarms are expected: finish the wrap, stop only at 95 %."
  - After a host restart, "re-read the two implement run dirs and the revision run dir from disk first". Done before
    Setup.
- **Correction (the stopped run):** a journal line first carried a time ("07:22Z") written without reading the clock.
  It was corrected in place against `date -u` (07:17:03Z), with the correction stated.
- **Recurrence of a standing learning:** `testing.md` 2026-10-07, "a moved bound: list the cases by the function it
  decides". This is the third recurrence, after the paste-newline and owed-ledger-rows chunks.
  - Run E moved verify's run count, and its readers were swept by row-count literal only.
  - Ten readers moved outside the plan's list, three of them found only by red gates.
  - **Sweep hazard:** a reader that builds its summary line from an integer (`dialogless_home`'s `passed`) is
    invisible to a sweep keyed on `N pass` literals. That miss cost a plan revision (the revision card's "Why").
- **Test-data hazard:** a synthetic session id off the session-id shape silently disables any probe whose target
  passes `is_session_id`. Run E never started under `s-verify-1`, and nothing failed loudly until the row read
  `fail`. A test set's ids must be shaped like the real ones.
- **Deterministic hold of a run:** the new stop case holds Run E open by giving it a fixture set with no default
  SessionStart and a shaped `clear-1` id. Run E's launch then fires no SessionStart, so the stop lands inside it.
- **Hygiene hazard:** `gate.py hygiene` P1 reads a prose glob `/tmp/viola-verify-*` as a host temp-dir path, which hit
  three files at the operator pass. Written `/tmp/viola-verify-<suffix>`, the `<` token is excluded. This is related
  to the carried deferred learning "gate.py hygiene reads `/home/<x>/` in prose…".
- **Sweep hazard (this wrap):** the site pattern `-plan/` matched the registry's `contracts/{doc}-plan/` paths (28 of
  35 rows were noise). It was re-fired anchored on `\.viola-verify-<pid>`.
- **Guard refusals:**
  - The stopped run: one `cat` heredoc to a scratch file and one `cd` into the skill's reference dir.
  - This window: one `cd` into the skills dir (the re-entry) and one `cd` into the chunk dir (this wrap).
  - Each was re-issued once with absolute paths.
- **Live measurement:**
  - `claude --resume <id>` on 2.1.287 finds a session from another project directory, and Run E adds no transcript
    file.
  - The P5 code reading is now measured (`live-round.md`). The founder's cap held at 12 of 12 starts, each round
    ledgered with `date -u` before firing and never re-fired.
- **The obs claim disproved** (Spec claims disproved 1). It is pre-existing and for the wrap's reconcile.

## Outcome
**Acceptance criteria**, re-asserted against the diff:
- (arch) **met.** `LedgerRow::ALL` holds 22 rows, the last `resume-session`, relied on every OS, and MM is 22 / 19.
  The 21 / 18 all-`pass` older stamp reads unverified. Proved by:
  - the `ledger.rs` units and `version_gate.rs` oracle (`run --unit`, 2090 tests);
  - `cli_verify` / `contract_ledger_probes` (integration);
  - the live stamp `stamped 2.1.287  22 pass  0 fail`;
  - CI's Windows leg at 19.
- (arch) **met.** The `resume-session` check passes only on Run E's first SessionStart of source `resume` with Run B's
  first id: the `check_resume_session_` filter, `passed 5`.
- (security) **met.** The id reaches argv only through `resume_target`'s `is_session_id` filter and `resume_args`, as
  one element of a direct `Run::spawn` under the R8 strip. The `-resume` dir is a 0700 `TrustedDir` removed on pass,
  fail and stop, and the input is `io::empty()`. Proved by the integration stop table and spawn pairs.
- (tests) **met.** `contract_ledger_probes` and `cli_verify` name `resume-session` once per OS literal and end
  `22 pass` / `19 pass`. The Windows half was read by CI `ci#38868756782` (`windows-2025` green).
- (tests) **met.** `SessionStart.resume-1.json` was recorded live by the record entry, passes the hygiene walk, and is
  replayed byte for byte by `contract_fake_agent_drift`. The fake agent's `--resume` builds on it with the given id.
  Proved by the fixture guard and the integration entry.
- (obs) **met** for the uninterrupted run. `viola verify` writes seven spawn pairs, each `process-exit` with
  `child_exit_status` and `duration_ms` (integration). The stopped-run wording is a separate pre-existing claim (Spec
  claims disproved 1).
- (design / layouts / a11y) **met.** A `send` of `/compact` on a verified CLI exits 0 with `[  ] unconfirmable` and
  logs `send-confirmed{confirmed:false}` (`binary(cli_send)`).
- (tests) **met.** The live round read `"suite":"local-live","passed":1,"failed":0` at 22 relied rows within 12 starts.
  `evidence/live-round.md` records each start.
- (tests) **met.** `pre-push` is green, and `ci.py conclusion` reads `verdict: green` on the final HEAD of the operator
  pass, `784d9f99`, run `ci#38868756782`.

**Gates** (by `run` text; implement runs `2026-10-16T06-46-40-implement` and `2026-10-16T07-35-58-implement`):
- `cargo fmt --all --check`: green (both runs; last 0.91 s).
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`: green (stopped run). Also,
  Windows-target clippy outside the block, exit 0.
- The G1 `rg … instrument` probe: green (`exit 1`, `no output`).
- `bash scripts/lint-probes.sh`: green.
- `bash scripts/agent-run.sh run --unit`: green, 2090 tests, after one fix (the second verdict vector).
- `… run --unit --filter 'test(/check_resume_session_/)'`: green, `passed":5,"failed":0`.
- The live round, fired once by `--live-legs`, never re-fired (`evidence/round-071816Z.txt`,
  `evidence/round-071942Z.txt`):
  - `test -x …/claude/2.1.287/claude` (probe): green.
  - The record entry: green 52.67 s, `stamped 2.1.287  22 pass  0 fail`, `round: COMPLETE`.
  - The census: green, no output.
  - The second probe: green.
  - `… run --local-live`: green 48.43 s, `"suite":"local-live","passed":1,"failed":0`, `round: COMPLETE`.
  - The second census fires only in a whole round. It was run by hand with its exact command at 07:19:50Z and printed
    no pid (`live-round.md`).
- The fixture guard (`git diff … fixtures/claude`): green.
- `… run --integration --filter 'binary(cli_verify) | …'`: green, 196/196, after one fix (the receipt counts).
- The preservation guard (`git diff --quiet 880b056377f4 -- . …`): red in the stopped run on `tests/support/home.rs`;
  green after the revision (0.01 s).
- `cargo deny check`: green.
- `agent-run.sh cleanup` / `boot` / `status` / `g2-zero-panics.sh` / `schema-check` / `cleanup`: green. The boot
  stamped every relied row through Run E (4.51 s), and the cleanup read `processes_gone` and `endpoint_gone` true.
- `bash scripts/agent-run.sh pre-push`: green. Coverage 2552/2552 and Playwright 1/1 in the stopped run; 73.82 s at
  the re-entry; 72.46 s on the committed tree before the push.
- `gate.py hygiene` (operator): refused 3 → clean after the rewrite (`evidence/operator-pass.md`).
- The push entry (`git diff --quiet && … git push origin HEAD`, operator): green, `history moved:
  refs/remotes/origin/build/viola-0.1.0 880b0563→784d9f99`.
- `ci.py conclusion --sha HEAD --wait 1800` (operator): `784d9f9937d6 verdict: green · checks 15/15 · wall 922 s · runs
  ci#38868756782 completed/success`. There was no fix commit.
- No `defer`, no deferral and no skip: the chunk has a Rust delta, so every Rust gate ran.

**Smoke:** the plan's smoke entries ran as P2 gates (boot / status / cleanup above). P3 recorded them and did not
re-drive them.

**Watches:** none.

**Outcome basis:**
- The operator pass ran. Its commit list is `784d9f9` alone, and the final HEAD's CI run `ci#38868756782` is recorded
  in `evidence/operator-pass.md`.
- The implement outcomes, in dir-name order:
  - `2026-10-16T06-46-40-implement/outcome-…md` (surfaced: entry 15). Superseded by the revision
    `2026-10-16T07-29-18-phase/` and by the later run.
  - `2026-10-16T07-35-58-implement/outcome-…md` (green + smoke), held whole in this window.
- The stopped run's journal and outcome were re-read from disk at this wrap, on the operator's word.

**Process hygiene:**

| Process | Started by | Final state |
|---|---|---|
| live round 1's six `claude` starts and `viola verify` | a leg (the record entry) | terminated: the round census found no survivor |
| live round 2's six starts, the harness and `viola verify` | a leg (`--local-live`) | terminated: the hand census at 07:19:50Z found none |
| the smoke session `p-rcl-smoke` | implement (both runs) | terminated: cleanup read `processes_gone` and `endpoint_gone` true |
| `pre-push`'s test processes (three firings) | implement and the operator pass | terminated |
| any process whose exe is under this repo | — | none, re-measured by a `/proc/<pid>/exe` scan at 2026-10-16T14:07:44Z |
## Addendum — the P2 halt, 2026-10-16
- inputs#I6 · message (the wrap P2 halt dialog, 2026-10-16T14:24:59Z) · copy · n/a — the three halt answers
  - bears on **Cross-project / external claims**: adds to it. The answer card (architecture 3, 6, 10, 11, 12;
    security-plan 1–7) was confirmed by the overseer as delegate. That confirm is PROVISIONAL until the founder's Epoch
    5 batch, and the ratification stays his answer to fork 2 (`inputs#I1`).
  - bears on **Spec claims disproved by measurement**: adds to it. Item 1 is settled "amend to measured", scoped to
    verify's interactive PTY runs. A non-PTY probe killed at the stop is recorded as not measured (the overseer, a
    founder-delegated technical fork).
  - bears on **Expected amendments (from plan)**: adds to it. The owner of the `/compact` post-condition row is a new
    markerless entry at the end of Epoch 5, after "Optional plugin install" (the founder's own word «В конец Эпохи 5»,
    relayed verbatim by the overseer).
- inputs.py verify, re-fired: see the P2 record (`fanout-results.md`); moved since the report: uncited 0 → 1 → 0 with
  this addendum.

## Addendum — the second P2 halt question, 2026-10-16
- inputs#I7 · message (the wrap P2 halt dialog, the overseer, founder-delegated) · copy · n/a — the test-plan §4
  local-command case count ("eight functions, 15 cases"), whose rule could not be recovered
  - bears on **Counts / qualifiers moved**: adds to it. That count is dropped from the master rather than restated:
    the eight `send_local_command_` functions are named, and the code's `#[case]` rows are the count. By `#[case]`
    rows it read 14 at the commit that wrote it, 24 at this chunk's base and 32 now (`src/run/send/classify.rs`).
- inputs.py verify, re-fired after this addendum: uncited 1 → 0.

## Addendum — the P7.1 light gate, 2026-10-16
- inputs#I8 · message (the wrap P7.1 ASSERT dialog, the overseer, founder-delegated) · copy · n/a — the treatment of
  round entry 12
  - bears on **Outcome**: adds to it. The light gate (this run dir's `gate-…json`) read `entries 26 · green 17 · red
    0 · recorded 0 · timeout 0 · not-run 9`.
  - The nine not-run are the leg entries, re-verified from `evidence/`:
    - round 1 (7, 8) from `round-071816Z.txt` and round 2 (9, 10, 11) from `round-071942Z.txt`, each `round:
      COMPLETE`;
    - the operator entries 24–26 from `operator-pass.md` (hygiene clean, push green, `ci.py` `verdict: green`);
    - round entry 12 (the census after the `--local-live` round) from its hand record in `live-round.md`.
  - Entry 12 is in no round listing. The round was fired with `--entry 11`, which runs no step after its leg, so the
    plan's exact command was run by hand at 07:19:50Z and printed no pid. That record is accepted as entry 12's
    treatment.
- inputs.py verify, re-fired after this addendum: uncited 1 → 0.

## New text, by line
Generated by `cites.py added` (cites v1.4); pasted by `splice.py`. No line of this section is typed or edited.
The diff: 880b0563 (the parent of the oldest pre-CI commit 784d9f99) → the work tree.
A row is a block this chunk added: `{first}-{last}`, `@{head}` its head line where not the first, «the head line».

### crates/viola-agent-claude/src/ledger.rs — added 221 line(s) in 36 range(s)
added: 43 · 47 · 69 · 96 · 128 · 197 · 200 · 643-644 · 652 · 842-874 · 934 · 1414 · 1440 · 1468 · 1483-1487 · 1491
       1493 · 1736 · 1757 · 1912 · 2034-2037 · 2072 · 2075-2081 · 2089-2094 · 2236 · 2256 · 2646 · 2711 · 2726-2859
       2876 · 2975 · 3042 · 3075 · 3330 · 3337 · 3342-3343
- 842-850 @844 «pub fn resume_target(trusted: &[Capture]) -> Option<&str> {»
  - 845-849 «trusted»
- 852-855 @853 «fn resumed_start(resumed: &[Capture]) -> Option<&Capture> {»
- 857-865 @858 «fn session_resumed(trusted: &[Capture], resumed: &[Capture]) -> bool {»
  - 859-861 «let Some(target) = resume_target(trusted) else {»
  - 862-864 «resumed_start(resumed).is_some_and(|c| {»
- 867-873 @868 «pub fn resume_variants(resumed: &[Capture]) -> Vec<(&'static str, &Capture)> {»
  - 869-872 «resumed_start(resumed)»
  - 2729-2733 @2731 «fn clean_resumed() -> Vec<Capture> {»
  - 2735-2747 «fn resume_check(trusted: Vec<Capture>, resumed: Vec<Capture>) -> bool {»
  - 2749-2766 @2750 «fn check_resume_session_passes_on_the_recorded_shape() {»
  - 2768-2787 @2769 «fn check_resume_session_fails_on_another_source() {»
  - 2789-2806 @2790 «fn check_resume_session_fails_on_another_id() {»
  - 2808-2813 @2809 «fn check_resume_session_fails_with_no_run_e_capture() {»
  - 2815-2835 @2816 «fn check_resume_session_fails_when_run_b_first_id_is_off_the_session_id_shape() {»
  - 2837-2843 @2838 «fn resume_target_is_run_b_first_session_id_of_the_shape() {»
  - 2845-2858 @2846 «fn resume_variants_name_run_e_first_session_start_resume_1() {»
### crates/viola-e2e/src/harness/run.rs — added 54 line(s) in 11 range(s)
added: 335 · 357 · 360 · 379 · 944-966 · 968-987 · 1030-1031 · 1035 · 1037 · 1039 · 1089-1090
### fixtures/claude/2.1.287/SessionStart.resume-1.json — new file · 1 line(s)
### src/bin/viola-fake-agent.rs — added 44 line(s) in 5 range(s)
added: 40-41 · 158-160 · 437-446 · 961-965 · 1356-1379
  - 437-445 @439 «fn launch_variant(&self) -> &'static str {»
  - 1356-1378 @1359 «fn fake_resume_launches_on_the_recorded_resume_variant_when_the_set_holds_one() {»
### src/cmd/verify.rs — added 52 line(s) in 17 range(s)
added: 3-4 · 214-220 · 236 · 252 · 289-292 · 314 · 357-360 · 484-486 · 515-516 · 540-545 · 659-660 · 673 · 678 · 680-681
       684-685 · 688-693 · 697-703
  - 214-219 «let [trusted, questions, plan, resume] = [»
  - 357-359 «fn resume(&self) -> PathBuf {»
  - 540-545 «for (variant, capture) in ledger::resume_variants(&probes.resumed) {»
### src/cmd/verify/typed.rs — added 45 line(s) in 9 range(s)
added: 1 · 11-14 · 56-57 · 68 · 71 · 74 · 84 · 89 · 235-267
- 235-266 @238 «fn resume_run(»
  - 244-246 «let Some(id) = ledger::resume_target(&trusted) else {»
  - 253-259 «let started = wait_capture(»
  - 260-262 «if let Some(started) = started {»
### src/run/send/classify.rs — added 14 line(s) in 7 range(s)
added: 50-51 · 200-201 · 250 · 253-258 · 260 · 268 · 270
### src/run/version_gate.rs — added 23 line(s) in 6 range(s)
added: 260-265 · 315-322 · 325 · 327 · 335-340 · 348
### tests/cli_fake_agent.rs — added 45 line(s) in 8 range(s)
added: 486-487 · 504 · 520-522 · 526 · 529 · 533-566 · 573-574 · 576
- 533-565 @536 «fn fake_agent_resume_launches_on_the_recorded_resume_variant() {»
  - 541-545 «let keys = |v: &Value| {»
  - 546-560 «assert_eq!(»
  - 573-574 «let (offered, recorded) =»
### tests/cli_send.rs — added 36 line(s) in 1 range(s)
added: 735-770
- 735-769 @740 «fn send_compact_on_a_verified_cli_is_unconfirmable_at_once(stamped_home: StampedHome) {»
  - 747-750 «assert_eq!(»
  - 762-765 «let confirmed: Vec<&Value> = role»
### tests/cli_verify.rs — added 220 line(s) in 48 range(s)
added: 3 · 5 · 28-50 · 53-72 · 75 · 97 · 100 · 119 · 123 · 125 · 246-248 · 260 · 262 · 331 · 335-338 · 354 · 369
       441-442 · 486-487 · 489 · 523-524 · 532 · 628 · 691-697 · 851 · 864-866 · 895 · 920-922 · 962-963 · 981 · 996
       1071-1179 · 1198-1201 · 1209 · 1211 · 1213 · 1216 · 1219 · 1255-1256 · 1266 · 1268 · 1288 · 1297 · 1321 · 1323
       1325 · 1401 · 1403
  - 694-697 «assert_eq!(»
  - 920-922 «if !failing {»
- 1071-1178 @1080 «fn verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1(»
  - 1085-1088 «let (_, _, mut cleared) = support::verify::framing_set()»
  - 1090-1096 «fake::write_fixture(»
  - 1104-1112 «let started = support::verify::verify_started(»
  - 1113-1115 «let lines = fake::wait_for(&started.receipt, "Run E's start", |l| {»
  - 1123-1126 «assert_eq!(»
  - 1127-1130 «let cwds: Vec<PathBuf> = of_kind(&lines, "cwd")»
  - 1133-1136 «assert!(»
  - 1137-1139 «for dir in &cwds {»
  - 1140-1143 «assert_eq!(»
  - 1148-1149 «let role =»
  - 1150-1158 «let shape: Vec<(&str, &str)> = role»
  - 1159-1162 «let pty = [»
  - 1163-1169 «let mut expected = vec![»
  - 1170-1172 «for _ in 0..5 {»
### tests/contract_fake_agent_drift.rs — added 56 line(s) in 3 range(s)
added: 6-8 · 16 · 384-435
- 384-413 @386 «fn resume_launch(root: &Path, version: &str, id: &str) -> (String, Vec<u8>) {»
  - 391-402 «let mut child = Command::new(FAKE)»
  - 403-405 «let lines = fake::wait_for(&receipt, "the launch hook", |l| {»
  - 409-412 «(»
- 415-434 @419 «fn fake_agent_resume_replay_matches_every_recorded_resume_set() {»
  - 422-432 «for version in recorded_versions(&root) {»
### tests/contract_fixture_hygiene.rs — added 33 line(s) in 1 range(s)
added: 533-565
- 533-564 @536 «fn claude_resume_variants_are_walked_for_every_resume_set() {»
  - 538-545 «let variant = |p: &PathBuf| {»
  - 546-550 «let mut sets: Vec<&Path> = walked»
  - 552-555 «assert!(»
  - 556-563 «for set in sets {»
### tests/contract_ledger_probes.rs — added 9 line(s) in 8 range(s)
added: 3 · 17 · 39 · 42 · 61 · 63-64 · 132 · 134
### tests/support/home.rs — added 3 line(s) in 2 range(s)
added: 375-376 · 386
### tests/support/verify.rs — added 11 line(s) in 7 range(s)
added: 26-27 · 31 · 99 · 141 · 472-475 · 483 · 494
