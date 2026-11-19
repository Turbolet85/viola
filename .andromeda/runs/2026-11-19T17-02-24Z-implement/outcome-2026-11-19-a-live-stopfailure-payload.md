# Outcome — 2026-11-19-a-live-stopfailure-payload

**Outcome:** green — the operator pass is next (entries 19 pre-push, 20 hygiene, 21 the push, 22 the CI read), then
the wrap.

## Files changed (7, all in research's "Files to modify"; 0 new)
- `crates/viola-e2e/src/rig/isolate.rs` — `keep_record` refuses per CLI version (`kept/<version>` standing → `exists`,
  nothing written), a later version kept beside an earlier one; `sandbox_args` · `sandbox_probe_args` · `on_path`;
  `seed_stop_failure_capture` beside `seed_capture` (one shared `seed_capture_on`); five new unit cases.
- `crates/viola-e2e/src/rig/mod.rs` — `Arm::SF` (`sf`, labels `SF` / `SF-2`, instance `sf-probe`, trusted,
  marker-seeded), `Plan::contain`, the sandbox probe before any credentials read or ledger line (`refused: sandbox`),
  the contained spawn in `run_child`, `arm_sf` and `sf_cell_attrs`, `sandbox_probe` (pub), `net_ns_word`,
  `turn_end_words`, `unknown_value_lines`, `snapshot_pids`, `captured_word`, `cause_matches`, `told_apart`;
  `SF_NAME` · `SF_TURN_BOUND` 480 s · `SF_CAPTURE_BOUND` 10 s; five new unit cases.
- `crates/viola-e2e/src/rig/capture.rs` — a StopFailure payload reads to `event: stop-failure`, key names, the error
  word of `ERROR_WORDS` (the CLI's 13) else `other` (`absent` when missing), two byte lengths; two unit cases.
- `crates/viola-e2e/src/rig/screen.rs` — `Model::cell_attr` and `Model::box_attr` (attributes only, never text); two
  unit cases.
- `crates/viola-e2e/src/bin/viola-live-rig.rs` — `sf` / `SF` / `SF-2`, `rehearse --contain` (`sf` alone, else usage),
  every live `SF` contained, the doc lines.
- `crates/viola-e2e/tests/rig_rehearsal.rs` — four `sf` rehearsals.
- `src/bin/viola-fake-agent.rs` — `--turn-fail <variant>` and `--settings-hooks <file>`, both argv-only.

## Deviations, with their justifications
1. **`--settings-hooks <file>` added to the fake agent** beside the plan's `--turn-fail`. Plan step 10 asserts
   `captured-word: server_error` over the fake agent, but the fake ran only the plugin's `hooks.json`, never the
   private config's `settings.json`, so the rig's StopFailure capture hook could never fire under it (the existing `r3`
   rehearsal reads `send-1-capture: 0` for the same reason). The option is argv-only, compiled only in the
   `fake-agent` binary, reads no env, and splits a command string with no `args` on spaces (no shell). In a listed
   file; no scope-record line (scope: clean).
2. **Step 10's "second planted run" is its own test**, `rig_rehearsal_sf_reads_an_unknown_error_word_as_api_error_and_keeps_no_needle`,
   not a second rig run inside the first: two rig runs in one case would double its wall under the integration
   profile's kill. The plan names three rehearsals; four stand.
3. **The rehearsal fixtures carry `--cli-version 2.1.328`** so the fake reads the copied 2.1.328 set (its default is
   2.1.287).
4. **`hint-attr` reads the first drawn cell past the glyph**, which with a hint standing can be the cursor's cell —
   a named limit carried in `evidence/stopfailure-reading.md` (live read `inverse`).

scope record: none — gate.py scope clean, 0 recorded (`scope: clean — changed 7 · listed 7 · recorded 0`).

## Gates (implement run `2026-11-19T17-02-24Z-implement`)
- 1 `cargo fmt --all --check` — red on the first firing (a scripted edit after the last `cargo fmt`), then green after
  `cargo fmt --all` (1.2.log).
- 2 clippy — green · 3 `nextest -p viola-e2e --lib --no-fail-fast` — green · 4 `run --unit` — green · 5 `run
  --integration --filter 'binary(rig_rehearsal)'` — green.
- Controls (`control-1.toml`): `keep-version` holds · `sandbox-refusal` holds (`evidence/control-1.txt`).
- Live round `--live-legs --entry 13`: 6-12 green (11 recorded, QUIET), 13 `SF` green 186.58 s; `round: COMPLETE · legs
  fired 1/1` (`evidence/round-171459Z.txt`). 14 and 15 (`SF-2`) not fired — spare unspent. 16 (operator census)
  green, `census: 0`, history unmoved (`evidence/operator-16.txt`).
- 17 `run --integration` — green · 18 `run --e2e` — green.
- Smoke: skipped — no boot-path change (the changed binaries are test-only and booted in the rehearsals and the round).
- No guard entry in the block.
- Watch C4 (`rig_rehearsal_reads_the_box_empty_after_x_and_del`): PASS in 5.log, 5.2.log (the control's in-place
  firing), 5.without-sandbox-refusal.log and 17.log — 4 green runs, no red.

## The operator pass's pre-push red, fixed before the pre-CI commit
- Firing 1 of `bash scripts/agent-run.sh pre-push`: red — `rig_rehearsal_sf_contained_reads_its_own_network_namespace`,
  `send-1-reply: instance-unreachable:instance-dead` (pre-push's `env -i` leaves `XDG_RUNTIME_DIR` unset, so viola's
  socket directory fell under the sandbox's private temp root).
- Fix: `sandbox_probe(path, runtime_dir)` refuses `no-runtime-dir` unless the runtime dir is absolute and outside the
  temp root (`isolate::runtime_dir_crosses` + its unit case); the contained rehearsal skips there by its printed probe.
- Entries 1-5 re-fired green (1.3, 2.3, 3.2, 4.3, 5.3); firing 2 of pre-push green (`evidence/operator-19.txt`).
  The live `SF` ran before the fix with the runtime dir set; not re-fired.

## The live reading
GREEN on predicates 1-7 (`evidence/stopfailure-reading.md`): `net-ns: own` · `box: up` · `modal: none` ·
`send-1-reply: ok` · `turn-end: stop-failure` · `cause: server-error` · `captured-word: server_error` ·
`cause-matches: yes` · `unknown-value-lines: 0` · `census: 0` · `credentials: removed` · `scratch: absent` ·
`user-config: changed-by-others`. Readings: `turn-end-after-s: 182`, `hint-attr: inverse`, `typed-attr: plain`,
`hint-told-apart: yes`.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola-live-rig` (SF), `bwrap`, `viola run sf-probe`, `claude` 2.1.328 | a leg (entry 13) | terminated — the rig's `census: 0`, the round's census, operator entry 16 `census: 0` |
| rehearsal rigs, fake agents, `viola` (entries 5, 6, 17, 18, the controls) | this run | terminated |
| 8 `claude` processes (`latest`, the operator's named overseer and builder sessions) | NOT DERIVED — not this run's | left running: the operator's own sessions |

Measured: `gate.py census --comm bwrap --comm viola-live-rig --comm viola-fake-agent --comm claude --args
projects/viola/target/debug/viola` → `census: 8 running`, all eight the operator's sessions.

## Decisions & corrections
- The implement directive (`inputs#I5`): the live cap one `SF` + one spare `SF-2`; hostwatch before each start. The
  round cannot stop on entry 11's report-only host read, so hostwatch was read by hand right before firing (QUIET,
  17:14:45Z) and `--entry 13` kept `SF-2` out of the round.
- Host-shell hazard: a scripted (python) edit fires no format hook, so entry 1 read red; a scripted write ends with
  `cargo fmt --all` before a gate (implement SKILL.md P1 already states it — a recurrence).
