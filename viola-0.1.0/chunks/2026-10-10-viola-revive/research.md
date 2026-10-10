# Codebase Research — 2026-10-10-viola-revive

## Scope
- **Depth:** deep · **Reads:** 16 · **Globs/Greps:** 14
- **Harness rules consulted:** `.claude/rules/verification-harness.md` — in full, its 20 Session Additions read; 6
  applied (the fake agent's fixture rule, the `key` receipt line, the exit-on-EOF quiesce, the program started twice
  per wrapper start, the `binary(<stem>)` selector, the stalled-start rule). `.claude/rules/testing.md` — in full; 5
  applied (the kill rule of 2026-10-10, the restart-over-a-gated-script rule, the stop's own `wheel` record, the
  10 s mutants kill for a stamped-home case, the remove-the-guard run).
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg
- **External inputs:** `inputs#I1` — the operator's take-up direction (no mutation, witness or dispatch run; live
  starts wait for the founder's number; the wheel question on the card; the `ci.py conclusion` operator read; a cut
  at the card). `inputs#I2` — no planned test sends a kill at a process it did not start. `inputs#I3` — the three
  P4 forks answered (P4, 2026-10-10): the founder, live: revive lands with no ledger row, `--resume` recorded at
  the wrap as a relied-on shape with no row, owed on a route entry; the operator: the fake agent sets `source` and
  `session_id` under `--resume`; the founder, live: the recorded cwd as the child's spawn directory is ratified as
  a widening, with the strict-modes check before the value is used.

## Files inspected
- `refs/session-memory-options.md` (§0, §1, §3 common failure modes and V2, §5, §6, §8) — the study the entry's
  first CARRY cites; tracked in this repository.
- `src/cmd/run.rs` (60-566) — `run`, `start`, `collision_check`, `start_state`, `spawn_child`, the five
  `refuse_*` writers. `start` reads `std::env::current_dir()` once (`:182`) and uses it for program resolution,
  the version gate and `SpawnSpec.cwd`; the program and its arguments come from `RunArgs.program` (`:178-181`).
  Every start refusal is `refused(detail)`: `process-exit` with the detail, exit 1 (`:286-289`).
- `src/cmd/mod.rs` (full) — the clap `Command` enum and `dispatch`; `run` gets `ObsProcess::Run`, every client
  verb `cli_sink` with `ObsProcess::Cli`.
- `src/main.rs` (36-135) — `role_of` (`:90-106`) files every first word but `run`, `hook` and a leading `-` under
  `Role::Cli`, whose catch site prints `error: internal error`.
- `src/run/mod.rs` (90-143) — `child_launch` puts `--plugin-dir <dir>` first, then the user's arguments;
  `resolve_program` resolves against this process's `PATH`.
- `crates/viola-state/src/snapshot.rs` (1-151) — `InstanceSnapshot` (11 fields, no `cwd`, no
  `agent_session_id`); every optional field is `#[serde(default, skip_serializing_if = "Option::is_none")]`;
  `SNAPSHOT_V` is 1.
- `crates/viola-state/src/replay.rs` (1-190) — `replay` returns the last `agent_session_id` only; no chain.
  `read_snapshot_or_replay` returns `Recovered::Snapshot` with no id on its snapshot arm.
- `crates/viola-state/src/strict.rs` (1-56) — `check_path` (Unix owner and mode, Windows owner and DACL) and
  `check_stamps`; no home or instance check exists.
- `src/bin/viola-fake-agent.rs` (425-500, 745-795, the option list) — options are argv; `SessionStart.default`
  fires once at launch (`:788`); unknown arguments are ignored; a `cwd` receipt is written (`:556-557`).
- `tests/support/home.rs` (262-511), `tests/support/outer_pty.rs` (the pub fns) — `Wrapper::boot` passes the fake
  agent's path as the program after `--`; `stop_keep` hands the home back for a second start; `OuterPty` has no
  kill.
- `tests/run_cli.rs` (597-660) — `run_child_ends_when_its_wrapper_is_terminated` is `#[cfg(windows)]`.
- `schemas/diag-line.v1.json` — read by a walk of every `detail` property.
- `fixtures/claude/` — the three committed sets' SessionStart and SessionEnd files.
- `.andromeda/architecture.md` (`:462`, the capability-ledger pattern) and `.andromeda/playbook.md` (30-55, the
  two `verdict: escalate` patterns).
- `viola-0.1.0/chunks/2026-10-02-epoch-2b-cleanup/evidence/p5-child-survival.md` (full).

## Graph impact (from the code-graph query; plane `rust`, `db_state` fresh, 65 rows)
- **start** — 1 product caller, `run @ src/cmd/run.rs:148`, and one test
  (`start_opens_the_scenario_one_spans_under_run_start @ src/cmd/run.rs:937`). A second verb entering it changes
  its signature for both.
- **collision_check**, **start_state**, **spawn_child**, **child_launch** — each called once, from `start`
  (`src/cmd/run.rs:194`, `:245`, `:261`, `:253`). `child_launch` has two unit callers in `src/run/mod.rs`.
- **resolve_program** — `start @ src/cmd/run.rs:183` and `measure @ src/cmd/verify.rs:109`.
- **refuse** (`src/human.rs`) — 9 call sites: five in `src/cmd/run.rs`, four in `src/cmd/verify.rs`.
- **role_of** — `main @ src/main.rs:53` and its case table (`:331`).
- **read_snapshot_or_replay**, **replay** — no caller outside `crates/viola-state` (the query's rows are the
  crate's own tests and `read_snapshot_or_replay @ crates/viola-state/src/replay.rs:91`). Revive is the first.
- **Snapshots::init** — `start_state @ src/cmd/run.rs:429`, where the first snapshot is written.

## Patterns detected
- **A start refusal** (`src/cmd/run.rs:286-289`, `:531-566`): one `human::refuse(unable, hint)` (two fixed stderr
  lines, one write), then `run::log_self_exit(1, Some(detail))`, exit 1. The process log's `detail` for it is a
  closed enum in the schema: `already-live`, `squatted-name`, `pinned-hash-mismatch`, `batch-script-child`,
  `internal-error`, `instance-dead`, `strict-modes-failed`, `server-verify-failed`, `wrapper-fault` (derivation:
  the walk of `schemas/diag-line.v1.json`, `/allOf/26/then/properties/detail`). `cwd-missing`, `no-session` and
  `session-live` are not in it; `strict-modes-failed` is.
- **The landed collision refusal** (`src/cmd/run.rs:539-550`): `<name> is already live` with hint `viola list`,
  and a second pair for a `stale` holder; both log `already-live`. The entry's `instance-live` is this refusal
  under the study's word.
- **A second start under one name** (`tests/cli_wait_last.rs:283-306`, `last_survives_a_wrapper_restart`):
  `stop_keep`, then `Wrapper::boot` again over the same home.
- **A wrapper terminated from outside** (`tests/run_cli.rs:597-660`, Windows only): pid and child pid read from
  the snapshot, `TerminateProcess` on the wrapper's pid, the child waited for by pid and start time.
- **Fixture replay** (`src/bin/viola-fake-agent.rs:444-475`): a hook's stdin is the recorded file's bytes, with
  `prompt` set for UserPromptSubmit alone; `contract_fake_agent_drift` compares them byte for byte.
- **The process `claude`** is a literal in one product file today, `src/cmd/verify.rs:105` (verify's default
  program); `PLUGIN_DIR_FLAG` is the compiled-name pattern in `viola-agent-claude`.

## Conventions to follow
- **Optional snapshot field**: `#[serde(default, skip_serializing_if = "Option::is_none")]`, no `v` move
  (`crates/viola-state/src/snapshot.rs:40-57`; architecture §Conventions, Protocol versioning).
- **A wait in a root test** is `Instant::now() + WITHIN` with an exit check (`tests/support/home.rs:438-463`); the
  count stands at 23 sites in 17 files (re-derived: `grep -rn 'Instant::now() + WITHIN' tests`, 23 lines, 17
  files with a hit).
- **A verb's module** is `src/cmd/<verb>.rs` with its clap args, declared in `src/cmd/mod.rs`.
- **Unit oracles are literals** in rstest `#[case]` tables with labelled cases (`.claude/rules/testing.md`).

## Findings the plan turns on
1. **The id chain does not exist.** `replay` keeps one id (`crates/viola-state/src/replay.rs:63-67`). `--id`'s
   membership check and `--list` need every `session-start` line that carries an id, with its `ts` and `cause`.
   The chunk adds that reader beside `replay`, over the same `read_from` and its three counts.
2. **No recorded resume payload exists.** The committed SessionStart files carry `source` `startup` three times
   and `clear` once (derivation: `grep -o '"source":"[a-z]*"' fixtures/claude/*/SessionStart*.json`), and every
   `session_id` in them is a UUID (a regex check over the four files). The fake agent fires `SessionStart.default`
   at every launch, so a revived fake child reports `startup` with the fixture's fixed id. The E2E's
   `cause:"resume"` with the dead session's id needs either a recorded variant (a live start) or the fake agent
   setting `source` and `session_id` on the recorded default payload under `--resume`, which test-plan §7 does
   not allow as written. A fork for P4.
3. **The fake agent's id is never the dead session's own.** Every start of the fake agent reports the same
   fixture id, so "the same id" is only a real assertion when the E2E revives with `--id` set to an OLDER id of a
   two-entry chain, or when the fake agent echoes the id it was given. The first needs a chain of two (the
   recorded `clear-1` pair under `--framing` gives one: its SessionStart carries a second id).
4. **`/clear` rotates the id, as recorded.** In `fixtures/claude/2.1.287/` the `clear-1` SessionEnd and
   SessionStart carry different `session_id` values, and the default start's id differs from both (a comparison
   of the three files). That is the `/clear` half of the study's P6, recorded by `viola verify` on 2.1.287 and
   read live on 2026-10-08 (architecture's amendment history). The `/compact` half has no reading.
5. **`run`'s start runs no strict-modes check on the home or the snapshot.** The only strict read on the start
   path is the stamps file (`src/run/version_gate.rs:136`); `collision_check` reads `snapshot.json` plainly
   (`src/cmd/run.rs:294`). `viola_state::strict::check_path` exists and is tested per OS. Security-plan requires
   the check at every entry point that reads `snapshot.json`, and none of the seven dated gaps names revive.
6. **Everything `session-live` reads is routed later.** No `claude agents` call exists in `src` or `crates`
   (the query and a search for `agents` outside the harness). The join-field ledger row, its `viola verify`
   probe, the fake agent's stub and the parser's property and fuzz targets are pinned by three CARRYs on the route
   entry "The board: viola list". Security-plan's accepted PATH risk for that call names `list`, `mcp` and `ui`
   only.
7. **The ledger limit does not reach revive.** Architecture's capability-ledger pattern (`:462`) holds one ruled
   limit, the founder's: "a CLI shape no `send` relies on needs no probe", stated for the long-paste frame. The
   amendment history reads it as scoped to `send`. `--resume <id>` reopening the same session with a SessionStart
   of source `resume` is relied on by revive, has no row, and its only reading is the study's carried F5
   (2026-09-25, CLI 2.1.28x). A row needs a `viola verify` probe: a fifth interactive child (the founder's
   ruling), a re-stamp, and every stamped home reads unverified until then. A fork for P4.
8. **The child's death with its wrapper is pinned on Windows only.** `run_child_ends_when_its_wrapper_is_terminated`
   is `#[cfg(windows)]`; the P5 evidence is a Windows reading of 2026-10-03. On Linux and macOS no test and no
   record shows a wrapper's child ending when the wrapper is killed. The kill-and-revive E2E is the first reading
   there, with the fake agent, which exits at stdin EOF.
9. **A kill is set against coverage.** An instrumented child killed during its exit-time profile write leaves a
   short profile (3 of 4 800 loaded runs, test-plan's history), and the last chunk's chaos case chose a stop over
   a kill for that reason. A wrapper killed while it runs writes no profile; the risk is a hook or the fake agent
   in its own exit at the kill. The E2E kills only after the start's hook has ended and the fake agent is idle.
10. **Revive's program is a name, not an argument.** The study's form is `viola revive <name> [-- extra claude
    args]`; the program is resolved by name at revive time. Root tests today pass the fake agent's path as the
    program word, so a revive test needs a `claude` on a test-scoped `PATH` (the harness's own pattern,
    `crates/viola-e2e/src/harness/boot.rs:125`). With no launch replay the fake agent's own flags
    (`--control`, `--receipt`, `--fixtures`, `--screens`, `--trusted-root`) reach the revived child only after
    `--`.
11. **The role.** `role_of` would file `revive` under `cli`; the verb becomes the wrapper. The process enum is
    closed at five values. A revive that logs as `run` (`run-<name>.ndjson`, the wrapper's panic rules) needs
    `role_of` to name it beside `run`; `--list` is a short read.
12. **Exit 1 has no `--json` document** (design-system's cli component 2, pending an architecture amendment), and
    `run` takes no `--json`. The route entry "CLI machine contract" owns the machine forms.

## New files to create
- `src/cmd/revive.rs` — the verb: its clap arguments, the preflight, `--list`, and the hand-off into `run`'s start
- `tests/cli_revive.rs` — the refusals, `--list` and the `--id` cases against the built binary
- `tests/chaos_revive.rs` — the kill-and-revive case

## Files to modify
- `crates/viola-state/src/snapshot.rs` — the optional `cwd` field
- `crates/viola-state/src/replay.rs` — the session chain reader
- `crates/viola-state/src/strict.rs` — the instance check revive's read runs first
- `crates/viola-state/src/liveness.rs` — its `InstanceSnapshot` literal gains the field
- `crates/viola-state/tests/state_replay.rs` — the chain and the missing-`cwd` cases
- `crates/viola-agent-claude/src/lib.rs` — the resume and fork flags, the program name, the session id shape
- `src/cmd/run.rs` — `start` takes its cwd, program and arguments from its caller; the first snapshot holds `cwd`
- `src/cmd/mod.rs` — the `Revive` arm and its dispatch
- `src/main.rs` — `role_of` names `revive` beside `run`
- `src/cmd/hook.rs` — its `InstanceSnapshot` literal gains the field
- `src/run/dialog.rs` — its `InstanceSnapshot` literal gains the field
- `src/run/snapshot.rs` — its `InstanceSnapshot` literal gains the field
- `src/run/wheel.rs` — its `InstanceSnapshot` literal gains the field
- `src/bin/viola-fake-agent.rs` — the `--resume` and `--fork-session` options
- `schemas/diag-line.v1.json` — the two new `process-exit` details
- `tests/support/home.rs` — a revive start and the wrapper kill beside `Wrapper::boot`
- `tests/support/outer_pty.rs` — the kill of the child it spawned
- `tests/cli_fake_agent.rs` — the cases of the fake agent's two new options

Sweep record, `squatted-name` over `src crates tests schemas scripts`: 3 files · 1 changed
(`schemas/diag-line.v1.json`, the enum) · 2 no-change (`src/cmd/run.rs` writes the value; 
`tests/cli_instance_state.rs` reads `already-live` and `squatted-name` as values in its own cases, lines 188, 247,
321, 701).

## Open questions
- Does `tests/cli_verify.rs` pin the fake agent's option list in a way the two new options move? → blocks:
  implementation-scope (the file is read at the step; it is not on the list until then)

Closed at P4 by `inputs#I3`: the ledger row for `--resume` (none now, owed on a route entry, the founder's word)
and the source of the fake agent's resume payload (two fields set under `--resume`, the operator's word).
