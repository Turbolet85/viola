# Implement outcome — 2026-11-04-linux-and-macos-parity

**Outcome: green + smoke** · run dir `.andromeda/runs/2026-11-04T20-49-17-implement/` · base HEAD `fdb28e90` (no
pre-CI commit yet) · live sessions 0, the fake agent only (the founder's word for this chunk, `inputs#I1` item 5).

## Files changed (13: 10 modified, 3 new — every one on research's lists; scope: clean, recorded 0)
- New: `tests/support/frozen.rs` (the `Frozen` guard, both OS twins, the Unix bound written `super::watch::WITHIN`) ·
  `tests/contract_os_gated_tests.rs` (the census: `GATES`, 59 rows; three tests) · `scripts/junit-os-sets.py`.
- Modified: `tests/support/mod.rs` (`pub mod frozen;`) · `tests/cli_instance_state.rs` (the two local twins gone,
  `use support::frozen::Frozen;`, three reason lines) · `tests/cross_exit_causes.rs` (the top comment's `Frozen`
  clause names `tests/support/frozen.rs`; two reason lines) · `tests/run_cli.rs` (the pin un-gated, its terminate
  step per OS — Windows `TerminateProcess` as written, Unix `kill -KILL <wrapper pid>` with its exit asserted; one
  reason line) · `tests/cross_list_rows.rs` (`DIM` const; `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`,
  `cfg(unix)`, on `booted_wrapper`) · `tests/cli_fake_agent.rs` (the hangup case and its two helpers `cfg(unix)`,
  `ignores_sighup` through `ps -o sigignore=`; one reason line) · `tests/cli_list.rs` · `tests/cli_plugin_install.rs` ·
  `tests/cli_verify.rs` (one reason line each) · `.config/nextest.toml` (`binary(run_cli)` on the `ci` 20 s kill
  line, 26 → 27 binaries).

## Gates (`gate-2026-11-04-linux-and-macos-parity.json`, one block run)
`entries 24 · green 19 · red 0 · recorded 1 · timeout 0 · not-run 4` — the four not run are the `leg = 'operator'`
entries 21–24 (hygiene, push, CI read, JUnit reading), the operator pass's. Readings:
- 1 `df -h /tmp` recorded (tmpfs 32G, 27G avail).
- 9 integration: `158 tests run: 158 passed, 0 skipped` (the pin `run_child_ends_when_its_wrapper_is_terminated`
  PASS 5.058 s on Linux, its first Unix reading; the widened hangup case passing through the `ps` reader).
- 10 e2e: `69 tests run: 69 passed`, `"ok":true` — the witness PASS 5.492 s, the census binary's 10 cases.
- 11 `mutants` profile: `2 tests run: 2 passed (1 slow)` — the pin 4.909 s, the witness 5.040 s, inside the 10 s kill.
- 12 the root waits' count reads 32 by its rule (`Instant::now() + WITHIN` under `tests/`), unchanged as predicted.
- 13 the preservation guard — its pass line, whole:
  `guard: allowed 13 — .config/nextest.toml, scripts/junit-os-sets.py, tests/cli_fake_agent.rs,
  tests/cli_instance_state.rs, tests/cli_list.rs, tests/cli_plugin_install.rs, tests/cli_verify.rs,
  tests/contract_os_gated_tests.rs, tests/cross_exit_causes.rs, tests/cross_list_rows.rs, tests/run_cli.rs,
  tests/support/frozen.rs, tests/support/mod.rs · companions excluded 0 · changed 13 · excluded 0` then
  `guard: clean`. No file was excluded; each allowed file is a listed path of research.
- 20 `pre-push`: `"ok":true, "stage":"linux-tests"` — coverage suite 3458 passed, 0 failed (3446 + the 12 this
  chunk adds), `gate --require coverage,doctest` no breach.
- No deferral: the diff is test-only Rust and every Rust gate ran (walk-class rust 7 · uncommitted 571 read, nothing
  deferred to void).

## Controls (`.andromeda/cache/chunks/2026-11-04-linux-and-macos-parity/evidence/`)
- `junit-os-sets-control.md` — the JUnit script's known-verdict control before its first graded use: `--run
  41339639367` exit 0, `windows-2025: 3376` · `macos-latest: 3400` · `ubuntu-latest: 3446`, 0 failed / 0 skipped
  each, `## ubuntu-not-macos: 48`, `## macos-not-ubuntu: 2` (and `windows-not-ubuntu: 89`, `ubuntu-not-windows: 159`)
  — every expected value; `--run 1` exit 1 naming the three missing reports; no `--run` exit 2.
- `implement-controls.md` — the census's remove-the-guard reading (a row renamed: red, naming both the unnamed gate
  and the row that no longer stands; restored: 10/10 green) and the macOS check-only clippy (testing.md 2026-10-31)
  exit 0.

## Deviations from the plan, with justifications
- The census reads 52 OS-gated test functions and 7 files (59 rows), where the plan predicted 50 functions: the plan's
  own rule (a `cfg` or `cfg_attr` naming an OS in the attribute run) takes the `cfg_attr` over case rows in
  `tests/hook_fail_open.rs` (two functions) and `hook_dialog_tier_fails_open_silently`'s two rows share one
  predicate. The plan asked that the census reading be recorded; the table's rows are the count.
- Research's split of the 48 ubuntu-not-macOS cases (44 carve-out + 4 Linux-only) names five Linux-only cases;
  counted line by line it is 43 + 5 (`junit-os-sets-control.md`). No code follows from it; the wrap's amendment text
  should carry 43 + 5.
- Step 6's reason lines for `strict_modes_on_the_board_is_exit_21_and_opens_no_log` and
  `strict_modes_on_plugin_install_is_exit_21_and_creates_nothing` name the mechanism each case uses (a widening by
  Unix mode bits): no master states a reason for those two gates (test-plan fct-4uc5ehox names the cases only).
- The census's self-check carries eight planted cases (the three the plan names plus a vanished gate, a `cfg_attr`
  over a case row, a gated file, a feature gate whose value spells `unix`, a broken attribute run), and the OS-word
  match reads outside string values so `feature = "unix-like"` is no hit — in-intent detail of step 7.
- No matrix write: `coverage where chunk = '2026-11-04-linux-and-macos-parity'` reads 0 rows (`v1-53` stays pooled
  with its P5 note, `inputs#I3`).

## Surfaced / stuck
Nothing surfaced, nothing stuck.

## Smoke (P3)
No boot-path change (test-only diff); the plan's smoke entries ran as P2 gates: 14 pre-clean ok · 15 boot ok
(`--unstamped`) · 16 status `state:"ready"`, builder `live`, `idle` · 19 cleanup `processes_gone:true`,
`endpoint_gone:true`, `home_removed:true`, killed none. G2 (17) and G4 (18) green between them.

## Process hygiene
`gate.py census --comm viola --comm viola-fake-agent --comm viola-harness --args projects/viola/target` → `census:
6 running`, none of them this repository's:

| process | started by | final state |
|---|---|---|
| `viola run` ×4 (`~/dev/projects/additional/viola-lab/prototype/target/debug/viola`: pulse-, conductor-, escher-builder, andromeda-worker) | NOT DERIVED — another tree's build (host-linux.md 2026-10-07) | left running: not this run's — the operator's |
| `~/.viola/bin/0.1.0-…/viola mcp` | NOT DERIVED — the installed bridge's MCP server | left running: not this run's — the operator's |
| `~/.local/viola-product/bin/viola wait viola-builder …` | NOT DERIVED — the overseer pair's bridge | left running: not this run's — the operator's |
| the smoke session `p-parity-smoke` (supervisor, wrapper, fake agent) | this run (entries 15–19) | terminated — cleanup `processes_gone:true` |
| the test binaries' wrappers and fake agents (entries 9–11, 20) | this run | terminated — no process under `projects/viola/target` in the census |

## Next
The OPERATOR PASS: entries 21 (hygiene), 22 (push — the first push carries the eight records-door commits through
`fdb28e90`, `inputs#I1` item 9), 23 (`ci.py conclusion --sha HEAD --wait 2400`, `leg = 'operator'`) and 24 (`python3
scripts/junit-os-sets.py --run <id>`, the id entry 23's read prints); then `/andromeda-wrap-session`.

## Decisions & corrections
- The operator's word for this run: live sessions 0, the fake agent only (the founder's word for this chunk); a tool
  fault stops the run with nothing patched, reported with the run dir; the eight local records commits go out with
  this chunk's pushes; the operator pass runs with the `ci.py` conclusion read (`leg = operator`).
- Guard refusal: a `rec.py find` piped into `head` was refused by the Bash guard's filter arm and re-issued bare once.
- Standing learning met: testing.md 2026-10-03 (an import used only by a `cfg(unix)` case goes inside the case — the
  witness imports `snapshot_data`, `Frozen` and `booted_wrapper` by path inside its own body) and 2026-10-31 (the
  macOS check-only clippy before the first push of a `cfg` move, run here).
