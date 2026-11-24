# Outcome — implement 2026-11-24-viola-up

**Written:** 2026-11-24T05:38:52Z · **Run:** 2026-11-24T05-12-02Z-implement

## Outcome: green — stopped before the operator pass

Every non-`operator`, non-`defer` entry of the block reads its `expect` on the final tree. One red stands RECORDED and
open, outside this chunk's diff: the e2e layer's `tui_channel_fds_the_child_holds_nothing_of_violas` (below). Under
CLAUDE.md's rule, a red met during a chunk before its operator pass folds into the chunk through the phase's revision
door, even outside its diff. That fold moves the plan (a file and a case outside its lists), and the directive
(`inputs#I4`) stops implement before the operator pass on such a red. **The operator pass was NOT run**: no pre-push,
no hygiene entry, no pre-CI commit, no push, no CI read. R4's entries (the two builds, R4a, R4b's plant and after)
were not fired, as the directive orders.

**Next:** the operator's or overseer's word on the open `tui_channel_fds` red: fold it through `/andromeda-phase`'s
revision door, or name another route. Then the operator pass (entries `bash scripts/agent-run.sh pre-push`, the
seeded `gate.py hygiene`, the push, `ci.py conclusion --sha HEAD --wait 1800`), WITHOUT R4.

## Files changed
New: `src/cmd/up.rs`, `tests/cli_up.rs`, `scripts/r4-up.sh`.
Modified: `src/cmd/mod.rs`, `src/cmd/revive.rs`, `src/cmd/open.rs`, `src/human/verbs.rs`, `src/human/hints.rs`,
`src/human.rs`, `src/bin/viola-fake-agent/foot.rs`, `src/bin/viola-fake-agent/test_cases_modes.rs`,
`crates/viola-state/src/liveness.rs`, `crates/viola-state/src/replay.rs`, `crates/viola-state/src/board.rs`,
`schemas/diag-line.v1.json`, `tests/cli_output_plain.rs`, `tests/contract_error_bodies.rs`,
`tests/contract_bounded_readers.rs`, `tests/cross_exit_causes.rs`, `tests/cli_revive.rs`,
`tests/cli_machine_contract.rs`, `tests/contract_diag_schema.rs`.

## Gates (final firing of each, trail `.andromeda/runs/2026-11-24T05-12-02Z-implement/gate-2026-11-24-viola-up.json`)
- `cargo fmt --all --check` green (red once: two files a scripted edit left unformatted; `cargo fmt`)
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings` green
- `cargo nextest run -p viola --features fake-agent --bins --no-fail-fast` green
- `… -E 'test(/::up_plan_|::revive_window_ask_|foot_records_each_spawn/)'` (ctl-root) green
- `cargo nextest run -p viola-state --no-fail-fast` green
- `… --workspace … -E 'test(/^up_/)'` green
- the readers' entry (`binary(cli_output_plain) | …`) green (red once: `help_lists_only_served_verbs`' served list
  lacked `up`; added)
- `bash -n scripts/r4-up.sh` green
- `hostwatch.py read --last 30` recorded, QUIET
- the rehearsal (`cargo build --features fake-agent && bash scripts/r4-up.sh … windows 7 8 --rehearse`) green:
  `r4: green phase=windows order=parents-first placed=2/2 lines=matched`, then `r4: rehearsed — not a live reading`
  (first firing before any graded use, then again in the whole block)
- `bash scripts/r4-up.sh "$(mktemp -d)/r4b" target/debug/viola plant` green (exit 1, `r4: red precondition=r4b-not-granted`)
- `grep -c -F -e .credentials.json -e claudeAiOauth scripts/r4-up.sh` green (exit 1, last line 0)
- `bash scripts/agent-run.sh run --unit` green
- `bash scripts/agent-run.sh run --integration` green (green · RED · green; the red recorded below, fixed)
- `bash scripts/agent-run.sh run --e2e` green (green · RED · green; the red recorded below, OPEN)
- the gated `evidence/r4.md` entry: not run — defer (gated: R4); no `--void`
- operator entries: not fired

Controls (`evidence/control-1.txt`, `--timeout 900` on the ctl-root entry): `control: 4 holds · 0 timeout · 0 does not
hold · 0 unreproduced · clone removed`.

## Reds recorded (the journal, `.andromeda/runs/2026-11-24T05-12-02Z-implement/journal.md`)
- `run --integration` (14.2.log): `cli_up up_without_foot_refuses_terminal_not_found_and_spawns_nothing`, `left:
  "gone  no-session\n" right: ""`. This chunk's own test race: `viola open` returns at the live endpoint, before the
  fake's `session-start` line. Fixed: `cli_up`'s open helper waits for the line. Green after.
- `run --e2e` (15.2.log): `tui_channel_fds_the_child_holds_nothing_of_violas`, `fd 4 holds a viola home file: …
  plugin/0.1.0-48d6da3c62f2489e/hooks/hooks.json` (tests/tui_channel_fds.rs:89). The cause, read from the test: the
  fake agent's launch opens `hooks.json` after its start receipt (the test's own comment says so), and the test
  retries only a link that vanished mid-read, never one read while still open. Neither the test, the fake's launch
  path nor the plugin is in this chunk's diff. The basis "same failure without this chunk's edits" is UNMEASURED. It
  read green on the next firing, which closes nothing. OPEN.

## Deviations (with justification)
1. Named instances narrow the fleet: with names given, `up` reads only those instances (each named one not scanned
   reads `not-taken`); with none, the whole fleet. Step 7 left open whether names add to the selection or narrow it.
   Narrowing makes `viola up <name>` re-open no one else.
2. Step 11's terminal pair: `up` spawns through open's shared `spawn_terminal`, which writes the one
   `process-start{subject:"terminal"}` open writes. No `process-exit{subject:"terminal"}` is written: the terminal
   outlives the verb, and open writes none either (see Spec claims disproved).
3. Exit-1 `process-exit` detail: `not-live` when any instance reads it, else no detail (exit 1 from `no-session`,
   `cwd-missing`, `session-live` or `not-taken` alone). The five words of step 11 are written where they apply:
   `boot-time-unread`, `strict-modes-failed`, `pinned-hash-mismatch` and `terminal-not-found` by `refused`.
4. Lines order: every line that is not a spawn (`already-live`, `stale-heartbeat`, `not-taken`, `no-session`,
   `cwd-missing`, `session-live`) in name order first, then the spawned or `would-open` lines in spawn order.
5. `ended_after_last_start` reads a log with a `session-end` and no `session-start` as ended (true); an absent or empty
   log reads false. The plan did not name the no-start case.
6. The fake `hyprctl` is unchanged. Its `terminal` mode falls through to the recorded set for a move. The tests
   (workspaces 2, 3) and the rehearsal (7, 8) each write, in their own scratch set, a copy of
   `MoveToWorkspace.recorded-1.json` with only the argv's workspace number changed. Nothing is committed under
   `fixtures/`.
7. `scripts/r4-up.sh`:
   - under `--rehearse`, the "workspace by app-id" reading is the instance's own window record, because the fake
     lists no app-id; live, it is the compositor's `clients` by class `viola.<name>`;
   - the stand-in's `fake-args` also carry `--trusted-root <root>/work`, so `--screens` draws no trust form;
   - `agents-mode` is `malformed` for the stand-in. Every stand-in session holds the one recorded session id, so the
     second revive would otherwise read the first as `session-live`;
   - each instance's one `viola send` is made, and its exit is in no verdict: the stand-in logs its session at
     launch.
   The same `agents-mode malformed` stands in `tests/cli_up.rs`, outside the `session-live` case.
8. `up` reads each snapshot once through `read_snapshot_or_replay`, so it writes `state-recovered` as `list` does. It
   does not call `board::read`, so it writes no `parse-rejected{parser:"state-entry"}` line.
9. Scope record, `gate.py scope`: `clean — changed 22 · listed 19 · recorded 3 (companion 2 · mechanical 0 ·
   in-intent 1 · widening 0)`.
   - in-intent: `crates/viola-state/src/board.rs` · serves step 7 · self (`Scan::instances()`, the read accessor `up`
     needs).
   - companion: `src/human.rs` · serves src/human/hints.rs · self (re-exports).
   - companion: `tests/contract_diag_schema.rs` · serves schemas/diag-line.v1.json · self (the new detail word's
     schema case).

## Spec claims disproved by measurement
- `.claude/rules/observability.md` (§Panics and errors, last bullet) states «`viola open` writes one `process-start` /
  `process-exit{subject:"terminal"}` pair around the foot terminal window it spawns». The code writes the
  `process-start` alone (`src/cmd/open.rs` `spawn_terminal`, `obs_event!(… ProcessStart, subject = "terminal" …)`; no
  `ProcessExit` with subject `terminal` in `src/`, `grep -rn '"terminal"' src/`). Plan step 11 repeats the claim for
  `up`.

## Unproven — behind R4 and the operator's forks
- R4a and R4b have not run. No real window was opened, moved or closed, no real `hyprctl` was asked anything, and no
  real `claude` was started. matrix#v2-12 stays GATED with no ref, and its gated entry is deferred.
- R4b's `plant` and `after` refuse until a revision built on fork U6.
- Forks U1–U8 stand PROVISIONAL:
  - U1: the selection rule;
  - U2: the terminal running the pinned `viola revive --origin`;
  - U3: a hand-started instance opening unplaced;
  - U4: a failed start not stopping the rest;
  - U5: CLI-only;
  - U6: R4b's credentials rule;
  - U7: the new words `would-open`, `not-taken`, `boot-time-unread`, the boot-time pair and the not-live hint;
  - U8: the first `up` after a Claude Code update.
- U8 is read from the code only: with `verify_new_cli` on, the first instance reads `not-live` while its automatic
  verify runs past the 30 s wait, and every later start in that `up` comes up unverified behind the held
  `ledger/verify.lock` (src/cmd/run.rs:421-431, src/run/auto_verify.rs:139-144); with it off, every start is
  unverified. No case and no live reading runs it.
- Unmeasured, live only:
  - whether claude 2.1.330 runs SessionEnd on a graceful shutdown (which would read a rebooted session as ended on
    purpose);
  - real foot windows on real Hyprland after a compositor restart;
  - a real `claude --resume` after a reboot.

## Process hygiene
| process | started by | final state |
|---|---|---|
| every test wrapper, stand-in `foot`, fake agent and fake `hyprctl` the gates started | this run (its gate entries) | terminated (census: none of this run's left) |
| the rehearsal's three instances and their stand-in terminals | this run (the rehearsal entry, twice) | terminated by the script's trap (census: none left) |
| the four controls' clone builds | this run (`gate.py control`) | terminated, clone removed |
| 11 listed by `gate.py census` (the operator's overseer feet and claudes, escher-builder's wrapper, `~/.viola` `viola mcp`, the product's `viola wait viola-builder`) | NOT DERIVED: not this run's | left running — the operator's fleet |

Left on disk outside the tree:
- `$TMPDIR/tmp.6JQv5Ve6YK` and `$TMPDIR/tmp.meET73vc55`: empty, one per firing of the R4b refusal entry;
- the lock files the tests' and rehearsals' wrappers left in `$XDG_RUNTIME_DIR/viola/` (the standing pattern).

Left inside `target/` (git-ignored): `target/r4/rehearse-20261124T052802Z` and `target/r4/rehearse-20261124T053311Z`.
None was removed by this run.

## Decisions & corrections
- The `tui_channel_fds` red, met before the operator pass, is held open rather than routed to the wrap, by CLAUDE.md's
  rule over the fix-loop protocol's wrap routing. Evidence: the journal entry at 05:35:09Z, and the directive's stop
  rule (`inputs#I4`).
- Guard refusals:
  - bash-guard refused a heredoc append to `tests/cli_revive.rs`; re-issued through the Edit tool;
  - bash-guard refused `gate.py show … | cut`; re-run bare;
  - the stamp-ahead hook refused two estimated journal stamps; rewritten from `date -u`.
- Sweep hazard:
  - `help_lists_only_served_verbs` keeps a second served-verb list beside the `HELP` literal; a sweep on the `HELP`
    row alone misses it;
  - a scripted (python) edit fires no format hook, and `cargo fmt` must end that call.
- Recurrence of a standing learning (testing.md 2026-09-24, wait on the exact line asserted): `viola open` returning
  live is not the `session-start` line a later kill depends on.
