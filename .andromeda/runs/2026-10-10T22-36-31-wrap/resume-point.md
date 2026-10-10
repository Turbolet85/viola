# Resume point — wrap of 2026-10-10-budget-governor

**Phase 1 is done. Phase 2 is next.** Written 2026-10-10T22:46Z by the session that ran the operator pass
(implement from step 16) and this wrap's Phase 1. The operator directed the stop with the wrap's invocation
(`inputs#I9`): the reconcile of this chunk is large and runs in a fresh context, and the route directions come
with the resume.

## How to resume

- Setup 2a applies: `chunk_dir/report.md` exists. Resume in THIS run dir
  (`.andromeda/runs/2026-10-10T22-36-31-wrap/`), reuse the report as it is, and start Phase 2 at its first step,
  the citation sweep (`cites.py apply --dry-run`, then without). No fan-out was made. No master, sidecar, key
  file, rule, leaf, route line or matrix entry was touched by this wrap.
- Re-fire Phase 1's two reads first, as a resume does: `gate.py scope` (read `clean` here: changed 31, listed
  31, recorded 0, base `396c108b`) and `cites.py added --out <run dir>/new-text.md` (expect `unchanged`; the
  listing is the report's last section, pasted by `splice.py` at report line 619: 334 lines, 265 blocks).
- `inputs.py verify` read 9 entries, 0 drifted, 0 uncited once the report existed. This wrap snapped one:
  `inputs#I9`, its own invocation. The route directions given with the resume are a new input: snap them
  (`--step wrap`) before Phase 5 cites them.
- Setup facts read here: branch `build/viola-0.1.0`, 0 ahead of `origin/build/viola-0.1.0`, HEAD `c85961d`.
  The basis is `396c108`, the parent of the one pre-CI commit `c85961d`. The pass made no fix commit.
  `state.yaml` read `last_wrap` 2026-10-10T20:35:49Z, `session_count` 61, `tree_db_refreshed_at`
  2026-10-10T19:56:27Z.
- The code-graph refresh fired at this Setup finished: rust 5215 nodes / 27952 edges in 35 s, ts 7 / 1.
  `.andromeda/cache/.refresh-done` was written at 2026-10-10T22:37:14Z; that is Phase 4's
  `tree_db_refreshed_at`.
- The tree is dirty only with bookkeeping and this wrap's products: the friction ledger, the second implement
  run dir's trails and its evolve script, `evidence/operator-pass.md` (its sections written after the pre-CI
  commit), the inputs manifest with `I9`, the report, this run dir.
- Epoch 4 after this chunk: 8 complete, 2 markerless ("Session links", "The board: viola list"). It does not
  close, so step 3b's consolidation does not fire and no evolve nudge applies.
- Phase 3's conversation is gone with the clear. Everything it held for curation, the route and the handoff is
  below; nothing else of it needs to be recalled.

## What this session was, in order

1. `/andromeda-new-session`: orientation only. Health 14 of 14; one pending record; coverage 16/53.
2. `/andromeda-implement` from step 16 (`inputs#I8`): the operator pass. `pre-push` green on the uncommitted
   tree; hygiene `clean` twice; the pre-CI commit `c85961d` (107 files); the guarded push, a fast-forward
   `396c108b→c85961dc`; `ci.py conclusion` green on the first attempt, `ci#38091081413`, 15/15, wall 726 s,
   `run_attempt` 1. No fix commit. Record: `evidence/operator-pass.md`.
3. This wrap's Setup and Phase 1.

Steps 1 to 14 were built by an EARLIER run whose conversation a `/clear` ended before this session. Nothing of
that run was in this window. What it held was read from its 11 implement records in the friction ledger and
from `evidence/red-green.md`, and each item was re-read in the diff before it went into the report. The
report's outcome-basis line says so.

## For Phase 2's Validate: what needs the operator, and what must not be written as ratified

1. **The four items of the plan's step 15 are landed and NOT ratified by the founder** (report: Deviations 5).
   The masters are amended to what is true, each with the authority it has: the operator's word (`inputs#I6`,
   `inputs#I7`), the founder's word owed. None is written as answered by him. The four:
   - the `budget-paused` refusal flows with no `rate_limits` ledger row, no probe, no fixture;
   - the wrapper refuses a driver on `budget.json`, a file a hook arm writes, that the trusted-file list does
     not hold. This is the boundary widening: security-plan records it "not ratified", never among the
     founder-ratified crossings. The playbook's "Boundary widening — what ratifies it" governs. The last wrap's
     appended rule also stands: a widening still halts at the wrap as one card with one confirm, and here the
     confirm can only be of the wording "not ratified", since the founder has not spoken;
   - a refused strict-modes check writes no line on `revive`'s start arm and leaves no trace on the statusline
     arm. This departs from sentences in four masters (report: Spec claims disproved 4 and 5). The operator
     answered it at P4 (`inputs#I3`, question 1: "A refusal that lasts one invocation is no refusal."); the
     scope lists it as a widening on the card, unsettled for the founder;
   - two human lines have no words in any master and landed in planning forms: `<name>  budget gate open
     wheel <holder>` (two spaces between the fields, as the other verb lines), and the lifted line's end as
     `YYYY-MM-DD HH:MM:SSZ` when it falls on another UTC date. design-system and layout-templates record them
     as planning forms, the founder's wording owed.
2. **One amendment the plan's list does not name** (report: Spec claims disproved 1). The start's Must-trace
   span list (`obs-plan.md:307` and `:606`) lacks `run.budget_eval`, which the start now opens between
   `channel.bind` and `state.snapshot_write`. The unit case that pins the chain was restated by the first run
   (`src/cmd/run.rs:1464`). Expect the obs detector to propose it; if it does not, Validate check 6 still owes
   its disposition. The last wrap recorded "two start steps with no span, by design"; this is a third start
   step, and it HAS a span.
3. **Nothing else is an escalation that I can see from the report.** Every acceptance criterion reads met
   against the diff. Criterion 2's clause "`answer`, `wait`, `last` and `pause` are not gated" holds by the
   diff (the gate's five product call sites) and by no test; the report says so in Outcome.

## Findings for the masters and the route (the report carries each; this is the list)

1. **The gate as landed** (report: Symbols). Fail-open by the masters' own rule: no reading, an `unknown`
   figure, a passed or unparseable `resets_at`, an unreadable or refused `budget.json`, and a bad threshold
   each leave the gate open or at its default. A window pauses only when BOTH its figures are known. With both
   crossed the detail is `five-hour`. An override covers both windows until its end and does not outlive its
   wrapper: every start evaluates from `budget.json` alone. The thresholds are read once, at start.
2. **"The pause lifts when `resets_at` passes"** (`architecture.md:72`) is true only at the next of the three
   points: the gate never polls, so a snapshot can read `budget_paused: true` after the reset until a `send`
   arrives or a turn ends. The master's own next sentence says "takes effect at the next of these points";
   the two read together are right, and a surface that shows the snapshot (the board, the page) inherits it.
3. **A plain `release` now answers the gate's recorded `budget_paused`**, where it answered the constant
   `false`. `release`'s result gains optional `override_until`. `test-plan.md:853` words the result as
   `{budget_paused:false}`.
4. **Two bad threshold keys give two `parse-rejected` lines** (report: Deviations 3); a document whose `v` is
   not 1 gives both defaults and no line from this reader.
5. **`check_instance` runs before any file under the home is opened, on both arms**, and a refused check
   writes nothing: `revive`'s start arm prints the state-files pair and exits 1; the statusline arm exits 0
   silently. The preflight has three readings now. Where an operator learns of a refused statusline check
   (plan, Implementation notes): the wrapped session's status line row stays empty, `budget.json`'s `read_at`
   stops moving, and `viola revive <name> --list` prints the pair for the same home.
6. **The two measured qualifiers** (report: Counts): "read in source, not measured on `revive`"
   (`architecture.md:406`, `security-plan.md:213`, and the leaf `.claude/rules/security.md`) and the
   statusline arm's one-invocation hypothesis were both measured at P3 and as the witnesses' red readings, and
   both orders are fixed. The sentences that name "Budget governor" as the order's owner are discharged.
7. **`schemas/diag-line.v1.json` still lists `strict-modes-failed` under `hook-decision.detail`**, and the
   statusline arm no longer writes it. The schema is under the plan's preservation guard and was not changed.
   obs-plan §6 says the arm "uses four of the seven" codes (`obs-plan.md:837`); it uses three.
8. **Path 6 landed in part.** Landed: the nine `binary(cli_budget)` cases (seven `path6_` functions, one with
   two rows, and `e5_the_replay_reads_what_the_wrapper_wrote`). Not landed, each with its owner by title: the
   harness-session form and `boot --statusline-echo` → "The board: viola list" (the cut, `inputs#I4`); step
   7's two-instance leg → the same entry (a unit case holds its rule); `list --json` and the board's cell →
   "The board: viola list"; `/api/sessions`, the ATIS and the Playwright signal → the Epoch 8 entries; the
   MCP form of the refusal → "MCP server for drivers"; E5's link leg → "Session links".
9. **The four CARRYs this entry carried, and what became of each** (Phase 5 reads the frozen line 119 of the
   working route for their text; `route.py pins` printed four `:119` rows at take-up):
   - the first (chunk 2026-10-10-self-healing-state: the `budget-gate` writer, E5's budget pause) —
     discharged: the writer landed; E5's budget leg is a root case; E5 whole still needs `link` and
     `list --json`;
   - the second (chunk 2026-10-10-statusline-pass-through: the `revive` check order, the statusline arm's
     hypothesis) — discharged, each with its acceptance and its witness;
   - the third (the `rate_limits` ledger row and its `viola verify` probe) — NOT built, by `inputs#I1` note 2.
     It needs a next route owner, which the operator's word did not name (`inputs#I6`; scope.md, "Left open by
     the word"). The plan says the wrap's route step asks the operator. My reading, labelled as mine: the
     entry "Paste newline ledger row" already holds five shapes for the founder's ruling on the `viola verify`
     child set, and this would be a sixth; the operator's route directions decide;
   - the fourth (`agent-run boot --statusline-echo`, the harness-session Path 6) — moves to "The board: viola
     list" with step 7's two-instance leg (the cut, confirmed at the P5 review, `inputs#I4`). test-plan §3's
     `boot` key (`registries/contracts/test-plan/5-command-implementation.md:4`) names "Budget governor" as the
     owner of the unbuilt flag and of the dropped `statusline-source-unresolved` reason; it is re-pointed with
     the move.
10. **Every other verb that opens a role file before any check** (`client::start` for six verbs, `hook`'s
    nine events, `verify`, `run`) runs no instance check and narrows an existing home the same way. Not this
    chunk's; scope.md names "Home and code-bearing file integrity". That entry holds 13 CARRY blocks; whether
    one of them already says this was not read here. Phase 5 reads them before pinning a fourteenth.
11. **No mutation run of the new code** (`inputs#I1`). The chunk added mutable code to the root package (a
    998-line module among it), `viola-state` and `viola-core`. The handoff's standing note on
    `windows-mutants.yml` and the epoch boundary's audit covers it; say so again, with this chunk added to the
    count of chunks since the last run (four now).
12. **Leaves the cascade will have to re-derive**, read in this session's context and now stale in at least
    these sentences: `.claude/rules/security.md` (the `viola revive` bullet's last sentence, "opens its log
    before that check … owed to the route entry Budget governor"); `.claude/rules/observability.md` (Init
    order: the statusline arm "still logs a refused check as `hook-decision{detail:"strict-modes-failed"}`";
    and "each preflight refusal one `process-exit`"); `.claude/rules/events.md` (the replay's
    `budget_override_until` now has a writer); CLAUDE.md's Critical Warnings line that places the
    `rate_limits` row "on Budget governor", and its Modules lines for `viola-core` (`Percent` now exists, and
    `BudgetWindowKind` is new), `viola-state` (the `budget.json` read) and the root bin (the gate);
    `.claude/docs/services/viola.md`, `viola-state.md`, `viola-core.md`.

## For the handoff's founder and operator sections

- **For the founder, new at this wrap: the four items of step 15**, in the words of section "For Phase 2"
  item 1. Each is his to rule; an answer that differs comes as a revision or a new chunk. Beside them:
  - a refused statusline check now leaves no trace anywhere, and the three signs an operator has (finding 5);
  - the gate is fail-open in the five cases of finding 1, and a drift in the CLI's `resets_at` shape opens the
    gate for that window (the cost named at P4, scope.md item 5);
  - the gate trusts a payload shape that has no ledger row. CLAUDE.md's "three relied-on dependences have no
    row yet" stands, and the statusline's `rate_limits` shape is now relied on by a `send` refusal, which is
    outside his ruled limit "a shape no `send` relies on needs no probe";
  - the reading's `used_percentage` is still an `f64` from 0 to 100; the thresholds are `Percent`. The carried
    line "one line if he wants the newtype" still applies to the reading.
- **Carried from the last handoff, untouched here:** everything under "For the founder (carried …)" and "For
  the operator's word". The item "a defect in shipped code, on the head entry" is closed by this chunk; the
  hypothesis beside it is confirmed and fixed.
- **No live `claude` start was made, no rig home was made, no ledger row, no re-stamp.** The dev host's bare
  `claude` and the stamped version were not read in this session.
- **Not measured:** the real `claude` against the gate; an account without `rate_limits`; the `spend_limit`
  window; the two check-order fixes on Windows (both witnesses are `cfg(unix)`; they passed on `macos-latest`
  and `ubuntu-latest` in CI); a home another user owns, or one with only an other-writable bit; the Windows
  case of `check_instance` on a widened DACL; override expiry across processes and two instances in one home
  (each a unit case only); any mutation score; which `cli_revive` and `hook_statusline` cases `windows-2025`
  does not build (the counts are in `evidence/operator-pass.md`, the names were not read).
- **Operator desk, added by this session (none committed, none removed):** nothing new under `target/` but
  `target/wincheck/`, used again for the Windows-target lint, and the pre-push run's artifacts under
  `target/agent-run/artifacts/` (`llvm-cov-summary.json` among them). Three CI test-job logs and a few
  throwaway scripts stand in this session's scratchpad; nothing depends on them.
- **The session-end hook rewrote the handoff's last section before this session** (the one changed line the
  pre-CI commit carried: "Completed normally at 2026-10-11 00:15:20", local time). It will do so again when
  this session ends.
- **CI at this wrap:** `c85961d` green, `ci#38091081413`, 15/15, first attempt. The test totals are 2280 /
  2275 / 2279 on `windows-2025` / `macos-latest` / `ubuntu-latest`.

## Curation candidates (Phase 3; the conversation that held them is gone)

Each is a fact met on live work, in this session or in the first implement run's records. Tiering is Phase
3's; the home named is a suggestion.

1. `testing.md` — **a guard test for a strict-modes check on the home widens the HOME and reads its mode
   while the process still runs.** A start whose check wrongly passes does not exit: it runs on as a wrapper,
   so a case that waits for the exit reads the runner's kill, not its own assertion. (An extension of the last
   wrap's candidate on the check before the role file.) Basis: `evidence/red-green.md` (a).
2. `testing.md`, Test data — **a reader's unit cases may plant a malformed state file only by altering the
   writer's own bytes** (write through the writer, then pad, replace or re-version); a booted-wrapper test
   never plants one. The plan's constraint and its step 13 contradicted each other until read this way
   (report: Deviations 2).
3. `testing.md` or `testing-src.md` — **`viola_channel::test_support::JsonFields` records an `f64` field as
   its Debug text**, so a case asserting a numeric log field needs a visitor with `record_f64`. Met as one red
   run in the first implement run.
4. `verification-harness.md` — **`pre-push` prints no coverage figure**: the three numbers are in
   `target/agent-run/artifacts/llvm-cov-summary.json` under `data[0].totals`, and the file's own time shows
   whether the run wrote it. Read twice in this chunk, each time after a search.
5. `ci.md`, The operator pass — **the Windows-target lint on the host is read before the push**
   (`CARGO_TARGET_DIR=target/wincheck cargo clippy --workspace --all-targets --features fake-agent --target
   x86_64-pc-windows-msvc -- -D warnings`). This chunk's first push read green; the previous chunk's first push
   read red on exactly what this lint sees. The command itself is in `testing.md` since the last wrap; the new
   part is its place in the pass.
6. `ci.md` — **the CI read outlives the Bash tool's foreground bound**: `ci.py conclusion --wait 1800` ran 744
   s here. It goes in the background, and its verdict is read from its output file.
7. `host-linux.md` — **a foreground `sleep` is refused by the harness.** A wait on a backgrounded call is its
   completion notice; a bounded poll of the output file needs a non-`sleep` delay (`read -t` on a file that
   never ends was used here, twice). Whether that form should be a rule or the notice alone is Phase 3's call.
8. The operator's memory already holds "the stamp-ahead hook reads quoted example times". A new case of it:
   **rstest literals in source that carry the day's date with later clock times** (`src/human.rs`, the first
   run); the cases were re-dated to 2025. A candidate extension of that memory or a `testing.md` line: a test's
   example instant is a past date.
9. Recurrences despite a standing learning, for the handoff's list:
   - `host-linux.md 2026-09-28/29`: a `cat` heredoc with a file target, refused by the guard, once in this
     session (an empty append left in a probe command); the first run's records name two inline python
     heredocs for source edits (`host-linux.md` Transports);
   - `host-linux.md` Paths: a `cd` outside a subshell opened most Bash calls of this session and of the first
     run; the guard did not refuse because the cwd was already the repository root;
   - the handoff's own deferred learning on long single-line files: multi-KB master lines were read here
     through three scratch scripts where `cascade.py window` is the documented reader.
10. For the playbook, if Phase 2 confirms a pattern: **a plan step that holds the operator pass for the
    founder's answer, lifted later by the operator's word, lands its items as "not ratified" in the masters,
    one card with one confirm** (this chunk is the first with that shape; the last wrap appended the sibling
    rule for a widening the founder HAD answered).

## Counts the masters may state, with their rules (report: Counts / qualifiers moved)

- `CONFIG_KEYS` 5 (the array at `src/obs.rs` 33-39);
- `InstanceSnapshot` struct literals 14 (`grep -rn -E 'InstanceSnapshot \{' src crates tests --include=*.rs`:
  23 lines, less the type's definition and 8 signature lines);
- `check_instance` product call sites 2 (`grep -rn 'check_instance(' src`);
- `revive`'s preflight readings 3, the strict-modes check ahead of them;
- the statusline arm's `hook-decision` fail-open codes 3;
- nextest's 20 s verify-driven binary list 14 (the `binary(…)` names on `.config/nextest.toml:32`);
- root `[[test]]` entries 22; root waits `Instant::now() + WITHIN` 28 lines; `take(MAX_FRAME` lines under
  `src` and `crates` 19;
- the controls table: one new negative with three rows;
- snapshot readers stay five; the capability ledger stays at seventeen rows;
- unit suite 1865 locally; `pre-push` coverage suite 2279; CI totals 2280 / 2275 / 2279 (each run's Summary
  line, run `38091081413`); coverage lines 97.98, functions 97.90, regions 97.76.

## What Phase 7's light gate should know

- The three `leg = 'operator'` entries are re-verified from `evidence/operator-pass.md`, never re-run: hygiene
  `clean`; the push green once, a fast-forward; the CI read green on `c85961dc62b3` (run `38091081413`, 15/15,
  attempt 1). No red was recorded.
- No `defer`, no `leg = 'live'` or `'round'` entry, no `watch:`. No capability is claimed: the coverage gate
  is a no-op (`claimed 0`). The ledger note on `v1-35` was written at the plan's P5 and rode the pre-CI commit.
- The block has 16 entries; `pre-push` takes about 63 s on this host, the whole block about 2 minutes. The
  smoke session is `p-bg-smoke`.
- The preservation guard names the base `396c108ba64c` and stays green as long as no guarded path moves; the
  wrap touches none of them.
- Hygiene at the end of this session's implement run read `clean` against the chunk's base. This run dir adds
  `evolve-report.py`, `relay-1.md`, `new-text.md`, this file and the tool trails.
