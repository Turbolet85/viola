# P5 review card 1 — 2026-10-17-end-to-end-path-binary-and-harness-legs

**Chunk:** End-to-end path binary and harness legs · viola-0.1.0 · Epoch 5 — Driver surface
**Plan:** `viola-0.1.0/chunks/2026-10-17-end-to-end-path-binary-and-harness-legs/plan.md` (12 steps, 22 gate entries,
12 acceptance criteria, 320 prose lines) · research `…/research.md` · scope `…/scope.md`

## Goal
The harness and the E2E layer catch up with test-plan §3 and §6 where the product already exists:
- `run --e2e` runs suite `nextest-e2e`, and the integration layer gets its `!binary(…)` exclusion. The E2E regex
  already matches 18 root binaries, an exact partition measured on nextest 0.9.146.
- `boot --statusline-echo` and `boot --agents-mode recorded|oversize|malformed`.
- `status` runs `viola list` and nests its document.
- Path 1 runs as one ordered flow over one home (root `tests/path1_run_start.rs`).
- Path 6's harness-session leg with step 7's two-instance release (`crates/viola-e2e/tests/path6_budget_harness.rs`).
- The CARRY 5 cleanup case survives a dangling `target/e2e-home` link.

No product source changes; a guard entry asserts it against `6042af635c3e`.

## Decisions on record
- Fork 1: Path 1 lives at the root. Fork 2: `harness_session` keeps the spawned `supervise`. Both were answered by
  the overseer as the founder's delegate (`inputs#I1`), so they are PROVISIONAL until the founder's Epoch 5 batch.
- Leaned (decisive, cited in the plan):
  - `harness_session` is a plain guard, since `viola-e2e` has no dev-dependencies.
  - A booted instance absent from `list` reads `degraded`.
  - `--statusline-echo` conflicts with `--unstamped`.
  - `last_error:"list-exit-none"` is a new closed value, an expected amendment.

## Top acceptance criteria
1. `v1-02` (claimed, below), on the three CI OSes.
2. `run --e2e` reads `nextest-e2e` with passed ≥ 1 and failed 0. The re-cut integration layer selects no E2E-prefixed
   binary.
3. Path 6 step 7:
   - after `release builder --budget`, `.ok.budget.paused` stays true, `builder` is false and `overseer` true;
   - there is one `budget-gate{paused:false, override_until}`, for `builder` only;
   - `send` exits 0 to `builder` and 11 `five-hour` to `overseer`;
   - the marker is asserted on Unix, and on Windows the statusline arm runs no command.
4. Path 1 steps 1–6 pass in one flow over one home.
5. `status`: `ready` / `degraded` / `down` from `list`, `last_error:"list-exit-<code>"`; an oversize or malformed
   agents mode stays `ready`.

## Touchpoints
- New:
  - `crates/viola-e2e/src/harness/session.rs`
  - `crates/viola-e2e/tests/path6_budget_harness.rs`
  - `tests/path1_run_start.rs`
  - `tests/support/start.rs`
- Modify:
  - harness `run/nextest.rs`, `run.rs`, `bin/viola-harness.rs`, `boot.rs`, `mod.rs`, `status.rs`, `cleanup.rs`;
  - the `Selection` / `BootOptions` literal sites in `run/{perf,coverage,fuzz,mutants}.rs`;
  - `crates/viola-e2e/tests/{harness_lifecycle,cli}.rs`;
  - `tests/cli_instance_state.rs`, `tests/support/mod.rs`, `Cargo.toml` (`[[test]]`), `.config/nextest.toml`
    (`binary(path1_run_start)` on the 20 s kill line).

## Test commands (22 entries)
- Lint: fmt, clippy, Windows-target clippy (new, green), G1, lint-probes.
- `run --unit`, plus the new unit cases by name (red: 0 selected, to write).
- The no-product-change guard (green; clone controls: modified `src/main.rs` red, new fixture red).
- `run --integration` over the five touched binaries (green, 68 passed).
- `run --e2e` whole and the two new binaries by name (red: `--e2e` is a clap usage error until step 2).
- The empty-intersection probe (red now `artifact-missing`, built `nextest-exit-4`).
- Harness smoke `boot --statusline-echo --agents-mode recorded` → `status` ready → cleanup (red: unbuilt flag).
- G2, G4, `pre-push`; operator: hygiene, push, `ci.py conclusion --wait 1800`.

Every red baseline is this chunk's own subject; none is a pre-existing property. Controls:
`.andromeda/runs/2026-10-17T00-19-29-phase/p5-controls.md`.

## Concretization preview — `v1-02` (the five-command harness), claimed by this chunk
- **Current acceptance:** On each of the three OSes an agent drives a product session through boot, run, status,
  cleanup and logs via identical shell shims, each command emitting exactly one JSON document and a typed exit
  status; boot reaches readiness or names the unmet check; cleanup run twice succeeds and proves endpoints, ports and
  token files gone; status shows the CLI board and the GUI sessions agreeing.
- **Proposed concrete form:** On each of the three CI OSes (the run the `ci.py conclusion` entry reads), the
  `scripts/agent-run.*` shims drive a fake-agent session through `boot`, `run`, `status`, `cleanup` and `logs`:
  - each command prints exactly one JSON document and exits 0, 1 or 2;
  - `boot` reaches readiness or names its unmet check;
  - `status` exits 0 with `state:"ready"` and a `list` member that is the `viola list --json` document naming the
    booted instance `wrapped:true, liveness:"live"`;
  - `cleanup` run twice exits 0 with `processes_gone` and `endpoint_gone` true.

  Witnessed by `ci.yml`'s harness lifecycle step on the three runners and by `crates/viola-e2e/tests/cli.rs` /
  `harness_lifecycle.rs` under `run --coverage`.
- **Never-weaken check:** the CLI half is unchanged in outcome, and "status shows the CLI board" is now provable. The
  «ports and token files» and «GUI sessions agree» clauses leave by the founder's 2026-10-16 ruling, already in the
  cap's note. No CLI clause is weakened.
- **Not claimed** (one dated `notes` line each, after the word):
  - `v1-35` and `v1-24` keep a Windows half owed to "Windows-only live measurements".
  - `v1-53` belongs to "Linux and macOS parity".

## Validation
- Mechanical: all checks pass (fence 22 entries parsed; planlint 0 hits; baselines written on the 11 `new` entries;
  inputs 0 unparsed; lists parsed 21 items; size 320 prose lines).
- Validation-1: aligned. `scope.md` §1 was amended (intent-incomplete) to record fork 1's root home.
- WATCH (observation, no task): `send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`, 0/3.

## Beyond the plan
none
