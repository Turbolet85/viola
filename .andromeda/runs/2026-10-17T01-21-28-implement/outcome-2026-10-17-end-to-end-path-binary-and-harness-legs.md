# Outcome — 2026-10-17-end-to-end-path-binary-and-harness-legs

**green + smoke** — block run 2: `entries 22 · green 19 · red 0 · recorded 0 · timeout 0 · not-run 3` (the three
`leg = 'operator'` entries 20–22). Trail: `.andromeda/runs/2026-10-17T01-21-28-implement/gate-2026-10-17-end-to-end-path-binary-and-harness-legs.json`.
Two fix-loop iterations (block run 1 read red 9, 11, 12, 19; one targeted re-run of 9; block run 2 green).

Readings the criteria name:
- `run --e2e`: `nextest-e2e` 157 tests across 20 binaries, all passed (the 18 standing E2E-prefixed root binaries +
  `path1_run_start` + `path6_budget_harness`); the plan predicted 20.
- `run --integration --filter 'binary(path1_run_start)'`: 0 tests across 0 binaries, `nextest-exit-4` (entry 12).
- `boot --statusline-echo --agents-mode recorded` → `status` → `cleanup` (entries 13–18): status `state:"ready"`,
  `last_error:null`, `list` the `viola list --json` document naming `builder` and `overseer` `wrapped:true,
  liveness:"live", status:"idle"`; cleanup `processes_gone:true, endpoint_gone:true`; G2 clean; G4 11 files 0 failures.
- `pre-push`: `linux-tests` green, 2666 tests passed (`run --coverage`, `gate --require coverage,doctest`).
- Walls (`evidence/walls.md`): the Path 1 flow 4.687 s / 5.140 s (instrumented) against the 10 s root kill; the Path 6
  leg 5.028 s / 5.287 s against the 30 s `viola-e2e` kill.
- CARRY 5 (`evidence/carry5-dangling-link.md`): red before (exit 100, `AlreadyExists` at `cleanup.rs:362:43`) and green
  after over a dangling `target/e2e-home` in a scratch export of HEAD `6042af635c3e`.
- WATCH `viola::cli_send send_under_the_paste_hint_a_human_key_during_the_gate_wait_wins`
  (`evidence/watch-cli-send.md`): green in both runs that executed it (pre-push, block runs 1 and 2: 8.067 s,
  8.098 s); never re-run for green. Tally for the wrap: 2 green, 0 red this chunk.
- Matrix: `v1-02` `planned → implemented`, ref `crates/viola-e2e/tests/cli.rs::status_logs_and_cleanup_drive_a_booted_session`
  · `crates/viola-e2e/tests/harness_lifecycle.rs::harness_session_boots_reports_logs_and_tears_down` · `ci.yml`'s
  harness lifecycle step on the three runners (the CI half is read by entry 22 in the operator pass).
- No gate deferred: rust delta 17 files, every gate ran.

## Files changed

New (4): `crates/viola-e2e/src/harness/session.rs` · `crates/viola-e2e/tests/path6_budget_harness.rs` ·
`tests/path1_run_start.rs` · `tests/support/start.rs`.

Modified (14 source + 2 config): `crates/viola-e2e/src/harness/run/nextest.rs` · `run.rs` · `boot.rs` · `mod.rs` ·
`status.rs` · `cleanup.rs` · `run/perf.rs` · `crates/viola-e2e/src/bin/viola-harness.rs` ·
`crates/viola-e2e/tests/harness_lifecycle.rs` · `crates/viola-e2e/tests/cli.rs` · `tests/cli_instance_state.rs` ·
`tests/support/mod.rs` · `Cargo.toml` · `.config/nextest.toml`. `run/coverage.rs`, `run/fuzz.rs` and `run/mutants.rs`
(listed) needed no edit: every `Selection` literal there spreads `..`. No product source, plugin file, fixture or
schema changed (entry 8 green).

Scope: `gate.py scope` → `scope: clean — changed 18 · listed 18 · recorded 0`. No `scope-record.md` line.

Evidence: `evidence/carry5-dangling-link.md` · `evidence/walls.md` · `evidence/watch-cli-send.md`. Input:
`inputs#I2` (below). Journal: `.andromeda/runs/2026-10-17T01-21-28-implement/journal.md`.

## Deviations from plan, with justifications

1. **The E2E regex is `^(path[0-9]*|tui|mcp|cross|chaos|contract)_`, not test-plan §3's `^(path|…)_`** (step 1's
   literals, `nextest.rs`). The plan's two binaries, `path1_run_start` and `path6_budget_harness`, do not match the
   written regex (the digit sits before the underscore): block run 1 read `run --e2e` at 18 binaries and ran `path1`
   under `run --integration`. Put to the operator as a plan↔spec conflict; answered by the Viola overseer (technical,
   founder-delegated), snapped `inputs#I2`: "numbered path binaries name the test plan's numbered Paths; the widened
   pattern matches no other binary today. The wrap amends test-plan §3's regex and the testing.md / a11y copies to the
   same form." The unit case was renamed `e2e_layer_is_the_e2e_prefix_regex`.
2. **The `agents-mode` file is written after the stamp, not right after the `claude` copy** (step 3, `boot.rs`).
   verify's Run B makes its one `claude agents --json` read through that same session copy, so a `malformed` mode
   written first failed the `agents-join` row and boot read `verify-failed` (block run 1, entry 9 / 19). A stale
   file is removed before the stamp; until the write, the absent file answers `recorded`.
3. **The harness's own throwaway-workspace cases plant one E2E binary** (`run.rs`, `with_an_e2e_binary`). nextest
   0.9.146 refuses a `binary()` regex that matches no binary (measured on a scratch crate: exit 94 "operator didn't
   match any binary names"), so the re-cut integration layer cannot parse in a mini workspace without one; the two
   real-nextest cases whose integration layer runs (`run_reports_unit_integration_and_doctest_suites`,
   `run_with_a_filter_matching_nothing_is_red`) write `tests/path_it.rs` first. The first now also proves the
   exclusion (integration passes 1, the `path_` test left out).
4. **`harness_session`'s drop keep rule** (step 7) read two ways; implemented as keep = `AGENT_RUN_KEEP_HOMES=1` or
   (`AGENT_RUN_KEEP_FAILED=1` and panicking), `cleanup` always called with the home kept so the guard decides
   (`session.rs`, `keep_home` with its case table).
5. **`status` spells `viola --home <home> list --json`** (step 5 wrote `list --json --home <home>`): the global flag
   first, as boot's stamp and the supervisor spell it.
6. **`last_plugin_dir` moved to `tests/support/start.rs` with the four named helpers** (step 10): the flow needs it,
   and a root test file defines no copy of its own.
7. **`run --e2e` alone runs no doctest**: doctest stays with unit, integration and coverage, as before.
8. **Cases beyond the plan's names**: `boot_flags_agents_mode_reads_its_kebab_value` and
   `boot_flags_statusline_echo_command_names_the_fake_agent_and_the_marker` (`boot.rs`, the shell-plain refusal),
   `status_derives_ready_for_no_booted_instance` and `status_derives_down_with_no_session_bin_to_run` (`status.rs`),
   the `session.rs` cases, `run_e2e_is_a_flag_of_its_own` (`viola-harness.rs`).
9. **The Path 6 leg compares instants as text** (`stored_instant`: 24 chars, `Z`): `viola-e2e` carries no chrono, and
   viola writes every instant in the one RFC 3339 ms form, which sorts as time.

## Surfaced for the wrap (nothing authored)

- test-plan §3 (`run` step 1 and step 2, the E2E regex) and its copies in `.claude/rules/testing.md` and a11y-plan
  §3 / §9: the regex becomes `^(path[0-9]*|tui|mcp|cross|chaos|contract)_` (`inputs#I2`, delegate overseer).
- The plan's own expected amendments stand as listed (§Implementation notes), with: `boot`'s `--agents-mode` writes its
  file after the stamp; `last_error:"list-exit-none"` lands as a new closed value; a booted instance absent from `list`
  reads `degraded`.

## Process hygiene

| process | started by | final state |
|---|---|---|
| the `p-e2e-smoke` supervisor, its two `viola run` wrappers and their fake-agent children (entries 13–18, both block runs) | this run | terminated — entry 18 `processes_gone:true, endpoint_gone:true` |
| `harness_session` / lifecycle / `cli.rs` booted sessions inside nextest (entries 9, 10, 11, 19) | this run (through the tests) | terminated — each case's own `cleanup` / drop guard |
| the CARRY 5 witness nextest runs in the scratch export | this run | terminated (exited 100, then 0) |
| any process whose executable lies under this repository or the scratch export | — | none running — measured by `/proc/*/exe` at 2026-10-17T01:43Z |

The `viola` and `claude` processes running on the host belong to the overseer bridge's prototype tree
(`~/dev/projects/additional/viola-lab/prototype/…`) and the operator's own CLI sessions, never this run's.

Left on disk for the operator: `target/carry5-witness/` (the witness build dir) and the scratch export under the
session scratchpad.

## Decisions & corrections

- The overseer's word on the E2E prefix (`inputs#I2`, technical, founder-delegated): widen the regex, the wrap amends
  test-plan §3 and the testing.md / a11y copies.
- The operator's invocation rules for this chunk: no live `claude` start (none made); each tool run bare and read as
  printed; each long run journalled in the run dir before the wait (`journal.md`); the watched `cli_send` case
  recorded each time, never re-run for green; then the operator pass with the `ci.py conclusion` read.
- `evolve.py append` refused one batch (`REFUSED ts: … later than the clock`) whose `ts` was hand-typed ahead of the
  read clock; re-fired with the `date -u` reading.
- Two journal stamps were written as approximations before the clock was read and corrected after (01:33Z → 01:32Z
  for the witness; `01:3x/4x/5x` placeholders replaced).

## Next

The OPERATOR PASS (plan §Test Commands, `leg = 'operator'`): entry 20 (`gate.py hygiene`, before the pre-CI commit),
the pre-CI commit, entry 21 (the push), entry 22 (`ci.py conclusion --sha HEAD --wait 1800`) — then
`/andromeda-wrap-session`.
