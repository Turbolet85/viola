# Codebase Research — 2026-10-11-the-board-viola-list

## Scope
- **Depth:** deep on the state readers and the cli edge; none on the web or the harness · **Reads:** 24 · **Globs/Greps:** 21
- **Harness rules consulted:** `.claude/rules/verification-harness.md` and `.claude/rules/testing.md`, both whole as loaded with their Session Additions. Applied: a whole root file is selected with `binary(<stem>)`; `run --e2e` is a usage error; a stamped-home case sits under the 10 s `mutants` kill; no test kills a process it did not start; a wrapper is stopped cleanly before its files are altered.
- **Platform issues consulted:** none — no runner-only bullet and no CI-reading entry outside the operator leg.
- **External inputs:** `inputs#I1` — the operator's direction for this take-up: the cut, the founder-only set, logged link names parsed again where first shown, no mutation, witness or dispatch run, the CI read by `ci.py conclusion`.

## Files inspected
- `src/cmd/mod.rs` (full) — the clap `Command` enum and `dispatch`; no `List` variant. `resolve_home` is `--home`, else the default home. `cli_sink` builds a `DetailSink` from a target name; `verify` builds one from the caller's `VIOLA_NAME`.
- `src/main.rs` (1–145) — `role_of` files every first word but `hook`, `run`, `revive` and a leading `-` flag under `Role::Cli`, so `list` is a `cli` word with no edit; the catch site prints `error: internal error` once for that role.
- `src/cmd/client.rs` (full) — `start(home, name)` opens the `cli` role file and logs the self start; `own_name()` reads `VIOLA_NAME` through `ViolaName::try_new`; `document(doc)` writes one `--json` document as one write; `unreachable` writes exit 21's document `{"v":1,"error":"instance-unreachable","detail":null}`.
- `src/obs.rs` (40–141) — `open_role_file` calls `ensure_private_dir(home)` first, which creates a missing home and narrows an existing one to 0700. A check made after it passes a widened home.
- `src/cmd/revive.rs` (20–247) — `trusted()` runs `check_instance` on the absolute home before anything opens; `Refusal::pair` holds the state-files `unable:` / `hint:` texts; `list_rows` is the landed static-rows writer.
- `src/human.rs` (1–125, item index) — one write per line on a locked handle, no print macro. `escape_message` shows every control character but LF and TAB as `\xHH`, DEL and C1 included. No row-mode escaper and no board writer exist.
- `src/run/budget.rs` (item index) — `Thresholds`, `read_thresholds(home)`, the crossed rule and `decide` are root-bin items, `pub(crate)`.
- `crates/viola-state/src/snapshot.rs` (1–162) — `InstanceSnapshot` holds `pid`, `started_at`, `cli_verified`, `cli_version: Option<String>`, `wheel`, `budget_paused`, `pending_dialog: Option<{dialog_id, kind}>`, and three fields the board must never show: `endpoint`, `cwd`, `statusline_command` (and `pinned_bin`). It holds no idle or busy field.
- `crates/viola-state/src/replay.rs` (1–170) — `read_snapshot_or_replay` returns `Recovered::{Snapshot, Absent, Replayed{cause, state}}`; `Replayed` holds `wheel: Option`, `budget_paused: Option`, `dialog_pending` (always false), `links` and `skipped`. It writes one `state-recovered` line per replay through `obs_event!`.
- `crates/viola-state/src/events.rs` (120–244) — `read_from(dir, 0)` yields the known-kind lines; `LoggedLines::skipped()` is whole once the lines are exhausted.
- `crates/viola-state/src/liveness.rs` (1–54), `heartbeat.rs` (item index) — `classify(beat_age, same_process)`, `same_process(&snapshot)`, `beat_age(dir, now)`.
- `crates/viola-state/src/budget.rs` (1–97) — `read_budget(home)` returns `BudgetReading{five_hour, seven_day}`; its `Stored` envelope does not read `read_at`.
- `crates/viola-core/src/lib.rs` (226–290) — `Reading<T>`, `BudgetWindow{used_percentage: Reading<f64>, resets_at: Reading<String>}`, `BudgetReading`.
- `tests/support/cli.rs` (full), `tests/support/home.rs` (item index) — `viola(home, args, stdin, from)` runs the binary with `VIOLA_NAME` removed unless `from` is given; `stamped_home`, `Wrapper::boot`, `stop_keep`, `snapshot_data`.
- `tests/cli_budget.rs` (item index, 545–578), `tests/chaos_torn_append.rs` (item index) — the Path 6 and E5 cases and the torn-append case the new readings sit beside.
- `crates/viola-e2e/src/harness/status.rs` (full) — `status` prints `"list": null` as a stand-in and runs no `viola list`.
- `.config/nextest.toml` (the override lines) — the `[profile.ci]` 20 s line names fifteen verify-driven root binaries.
- `schemas/diag-line.v1.json` (102–103) — the `parser` enum holds `state-entry`; the `detail` enum holds `symlink-ignored`, `unreadable` and `strict-modes-failed`.
- `.andromeda/playbook.md` (15–69) — the four `verdict: escalate` patterns, read for P4.

## Graph impact (rust plane; trace `tree-query-2026-10-11-the-board-viola-list.json`)
- **read_snapshot_or_replay** — 1 product caller, `src/cmd/revive.rs` `preflight`. The board is the second.
- **check_instance** — 2 product callers, `src/cmd/revive.rs` `trusted` and `src/cmd/hook/statusline.rs` `trusted`. The board is the third.
- **read_budget** — 1 product caller, `src/run/budget.rs` `BudgetGate::decide_at`, and 15 call sites in its own unit module. Its signature stays; the board calls a second form of the same read.
- **read_from** — product callers `replay`, `session_chain` (viola-state), `LinkSet::restore`, `WaitFeed::rebuild`, `wait::scan` (root). The board adds one pass per instance.
- **read_thresholds** — 1 product caller, `src/cmd/run.rs` `start`. The board is the second.
- **classify / same_process / beat_age** — product callers `client::live_endpoint` and `run::collision_check`. Unchanged signatures.
- **escape_message** — 6 product callers in `src/human.rs` and `src/cmd/revive.rs`. Unchanged; the row mode is a second function beside it.
- **role_of** — `main` and one table test. No product edit.

## Patterns detected
- **Check before any open** (`src/cmd/revive.rs:102`): the instance check runs on the absolute home before the wrapper log opens, because the open narrows the home.
- **A `cli` verb's log start** (`src/cmd/client.rs:21`): `read_diagnostics_level`, `viola_obs_init(home, Cli, Some(name), level)`, `log_self_start`, then any config rejection.
- **One document, one write** (`src/cmd/client.rs:116`): `--json` output is `format!("{doc}\n")` in one `write_all`.
- **Static rows** (`src/cmd/revive.rs:226`): two-space-separated fields, one `human::result` per row, an escaped field where the text is not viola's own.
- **A reader's unit cases alter the writer's bytes** (`crates/viola-state/src/replay.rs:172`, `budget.rs:99`): write through the writer, then cut or re-version.

## Conventions to follow
- **Human text only in `src/human.rs`**: writers take `&mut impl Write` and are unit-tested against literals (`src/human.rs:11`).
- **No environment seam**: a time-dependent decision takes its instant as an argument (`src/run/budget.rs` `decide`), so the board's read model takes `now`.
- **Root cases start from `stamped_home`** and use `tests/support/` helpers only; a new verify-driven binary is named on the `[profile.ci]` 20 s line.
- **Fixed human strings are pinned by literal asserts**: `trycmd`, `snapbox` and `assert_cmd` are in no manifest (`grep -E "trycmd|snapbox|assert_cmd|predicates" Cargo.toml crates/*/Cargo.toml`: 0 hits), and `tests/cli_verify.rs` is the precedent.

## Findings the plan rests on
- **No on-disk source of idle or busy.** The snapshot has no such field and a running turn records nothing. The masters source `status` from `claude agents --json` and give `unknown` when that read is missing, failed or not joined, never an error. Every row of this slice reads `unknown`.
- **`skipped` needs a pass over every log.** The counts exist only as the result of a read. A snapshot that reads needs no log, so a board that passed only replayed logs would show a torn tail on a healthy instance as 0. The total is the sum of one whole-log pass per instance directory that passes its check, gone instances included (the torn-append case's wrapper is stopped, so its instance is gone and its log still counts). A replayed instance's pass is the replay's own.
- **`budget` needs two things the reader does not give.** `read_at` (dropped by `read_budget`) and `paused`, which is the gate's crossed rule over thresholds only the wrapper reads today. The equality the criterion needs: for a reading with `five_hour.used_percentage` 91 and a `resets_at` later than now, `decide` under the default thresholds reads paused with window `five-hour` (`src/run/budget.rs`, `gate_decide_five_hour_pauses_at_ninety_and_not_below`).
- **An unreadable or newer snapshot has no pid.** Its liveness rests on the heartbeat alone and is never `stale`: a beat at most 5 s old is `live`, anything else leaves the instance off the board. An absent snapshot gives nothing to show.
- **The heartbeat is outside `check_instance`'s five paths.** Its age is read from a file inside an instance directory the check has passed, which no other user can write into.
- **An absent home passes the check** ("each that exists") and is an empty board.
- **With `VIOLA_NAME` set, a `cli` verb's log start creates a missing home.** That is what `send`, `wait` and the others do today; `list` takes the same start, after its checks.
- **The home's refusal pair is worded and not built.** design-system gives `unable: the viola home is not private to you` with its hint as `run`'s strict-modes refusal; no source file holds the text (`grep -rn "not private to you" src`: 0 hits), because `run` makes no such check yet. `list` is its first writer. The instance pair is built, in `src/cmd/revive.rs` `Refusal::pair`.
- **The built `--help` is clap's ordered list**, not the grouped table the masters draw. Grouping is no part of this slice.
- **E5 on the board is drivable without a restart.** A booted wrapper's `snapshot.json` is cut mid-object while the wrapper idles (bytes the wrapper wrote); `list --json` then takes the replay for that live instance. `run`'s collision check still calls `read_snapshot`, so the scenario's step 3 stays as its As landed note words it.
- **The budget readings are Unix root cases.** `budget.json` is written only by `viola hook statusline`, which runs a command on Unix only.

## New files to create
- `crates/viola-state/src/board.rs` — the read model: the scan of `instances/`, the per-instance check and read, liveness, the item, the summed `skipped`
- `src/cmd/list.rs` — the verb: the checks, the log start, the document and the human rows
- `tests/cli_list.rs` — the root cases of the verb
- `tests/chaos_forward_compat.rs` — the forward-compat reading on `list --json`

## Files to modify
- `crates/viola-state/src/lib.rs` — the new module's declaration
- `crates/viola-state/src/budget.rs` — a second form of the one checked read that also returns `read_at`
- `src/cmd/mod.rs` — the `List` subcommand and its dispatch arm
- `src/cmd/revive.rs` — its state-files pair read from the one shared text in `src/human.rs`
- `src/human.rs` — the board's writers and the row-mode escaper
- `src/main.rs` — one `role_of` table row for `list`
- `src/run/budget.rs` — the crossed rule reachable from the verb
- `tests/chaos_torn_append.rs` — the `skipped.torn_lines` reading on `list --json`
- `tests/cli_budget.rs` — Path 6's `list --json` readings and the board's budget cells
- `.config/nextest.toml` — the new verify-driven binaries on the `[profile.ci]` 20 s line

## Open questions
- What `list` does when the home, `instances/`, or one instance's files fail the strict-modes check: no master words it for a verb that reads many instances. → blocks: plan-decision
- Whether `src/run/budget.rs` needs an edit at all, or `decide` and `read_thresholds` are reachable from `src/cmd/list.rs` as they stand. → blocks: implementation-scope
