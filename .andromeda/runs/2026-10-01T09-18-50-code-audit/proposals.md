# Code Audit — viola · Epoch 2b — Windows slice I b: events and ledger · 2026-10-01T12:09:55Z
mode trend · HEAD 95c1a9b5fc · baseline 69abc0d038 (Epoch 2 — Windows slice I: wrapper, events, ledger) · span 1
Overshoot: 3 commits past the boundary 093bffb24c (the flip of 2026-09-29-fake-agent-drift-contract) · source delta: none (files []) — .andromeda/**, .claude/**, refs/**, .gitignore only; operator-confirmed trend at HEAD. The baseline record's overshoot: 0 commits (at its boundary).
Trend-breaks: none (every tool version token equals the baseline's). Ancestry: ok.
Corrections carried in this record: 3 schema-gap fills on 69abc0d038 (`mutation.not_measured` · `mutation.counts` · `mutation.scores`) — 9 of its 11 survivors are `#[cfg(unix)]` spans this host never builds; viola-channel and viola-pty re-score 100.0. Every mutation comparison below reads the corrected values.

## Proposals
_None of the threshold table's checks fired (see Below threshold). The three items below sit OUTSIDE that table: a project gate this audit now judges, and two measurements this run could not take._

### M1 — mutation gate (test-plan §10, judged at the boundary since the 2026-09-28 ruling) · 8 survivors in 3 units
**Movement:** measured survivors 2 → 8 (Epoch 2 — Windows slice I: wrapper, events, ledger → Epoch 2b — Windows slice I b: events and ledger); the gate (`missed == 0`, `timeout == 0`, `unviable <= caught`) fails on `missed` only — timeouts 0, unviable ≤ caught in every unit.
**Evidence** (`c-mutation-{unit}.json`; host x86_64-pc-windows-msvc; site = cargo-mutants file:line:col; text = the tool's own):
| site | mutation | vs baseline |
|---|---|---|
| crates/viola-pty/src/lib.rs:196:9 | replace <impl Pty for PortablePty>::resize -> Result<(), PtyError> with Ok(()) | new vs baseline |
| crates/viola-pty/src/lib.rs:330:9 | replace <impl Drop for HostTerminal>::drop with () | new vs baseline |
| crates/viola-pty/src/sideload.rs:34:5 | replace search_restricted -> bool with true | new vs baseline |
| crates/viola-pty/src/sideload.rs:34:5 | replace search_restricted -> bool with false | new vs baseline |
| crates/viola-state/src/fs.rs:17:5 | replace restrict -> io::Result<()> with Ok(()) | standing (line moved) |
| crates/viola-state/src/pin.rs:81:19 | replace match guard e.kind() == io::ErrorKind::NotFound with true in pin_exe | standing (line moved) |
| crates/viola-state/src/pin.rs:171:23 | replace match guard e.kind() == io::ErrorKind::NotFound with true in pin_companions | new vs baseline |
| src/cmd/run.rs:455:5 | replace refuse_stale with () | first measured (unit declined at Epoch 2) |

Not measured on this host (x86_64-pc-windows-msvc) — 13 mutants whose whole span a false `cfg` excludes, never survivors:
| site | mutation | predicate |
|---|---|---|
| crates/viola-pty/src/lib.rs:315:9 | replace HostTerminal::enter -> Option<Self> with None | cfg(unix) |
| crates/viola-pty/src/lib.rs:317:64 | replace != with == in HostTerminal::enter | cfg(unix) |
| crates/viola-pty/src/lib.rs:315:9 | replace HostTerminal::enter -> Option<Self> with Some(Default::default()) | cfg(unix) |
| crates/viola-pty/src/lib.rs:322:71 | replace == with != in HostTerminal::enter | cfg(unix) |
| crates/viola-pty/src/lib.rs:367:76 | replace != with == in host_size | cfg(unix) |
| crates/viola-channel/src/client.rs:231:5 | replace open_by -> io::Result<Stream> with Ok(Default::default()) | cfg(unix) |
| crates/viola-channel/src/endpoint.rs:76:5 | replace host_socket_dir -> PathBuf with Default::default() | cfg(unix) |
| crates/viola-channel/src/server.rs:75:9 | replace <impl Drop for Guard>::drop with () | cfg(unix) |
| crates/viola-channel/src/server.rs:108:5 | replace listen -> Result<(Listener, Guard), ChannelError> with Ok((Default::default(), Default::default())) | cfg(unix) |
| src/panic_frames.rs:69:5 | replace raw_frames -> Vec<usize> with vec![0] | cfg(unix) |
| src/panic_frames.rs:80:5 | replace module_of -> Option<(String, usize)> with None | cfg(unix) |
| src/panic_frames.rs:80:5 | replace module_of -> Option<(String, usize)> with Some((String::new(), 1)) | cfg(unix) |
| src/cmd/run.rs:318:5 | replace sideload_outcome -> (&'static str, Option<viola_state::pin::HeldCompanions>) with ("xyzzy", None) | cfg(all(windows, not(target_arch = "x86_64"))) |
Project union verdict: none registered (no test-plan line names one for cargo-mutants) — no project union verdict.

**Suspected shape:** five of the six non-standing survivors sit in code this epoch touched — `sideload.rs` is new (both constant returns of `search_restricted` survive), `pin_companions` is new, and `viola-pty/src/lib.rs` took 5 epoch commits (`PortablePty::resize`, `HostTerminal::drop`); `run.rs` `refuse_stale` predates the epoch and is measured for the first time. Each is an effect (DLL-search state, a resize reaching the child, a drop-time restore, a refusal) no assertion in its unit observes.
**Proposal:** per the founder's ruling, these become corrective-chunk items: a killing test per survivor (an observable for `search_restricted`'s result, the child seeing a resize, `refuse_stale`'s refusal) — or, where a survivor is an equivalent mutant, a recorded exemption naming why.

### M2 — coverage not measured · the project coverage command is red at HEAD on this host
**Movement:** coverage.line 96.77 → not measured (Epoch 2 — Windows slice I: wrapper, events, ledger → Epoch 2b — Windows slice I b: events and ledger).
**Evidence:** run 1 (2026-10-01T09:20Z-10:00Z): 985 tests run: 895 passed (30 slow), 89 failed, 1 timed out, 0 skipped — a foreign build ran beside it (overseer measurement). Run 2 (overseer direction, after the mutation units; build 11:33-11:58Z, tests 12:03:38-12:08:54Z): 985 tests run: 905 passed (22 slow), 80 failed, 0 skipped; 0 foreign rustc in 14 test-phase samples, CPU 8–24 %. 79 failures common to both runs; only in run 2: viola::cli_verify verify_under_a_regular_file_prints_only_the_internal_error_line.
Failures by test binary (run 2): viola::run_cli 12 · viola::hook_fail_open 11 · viola::cli_verify 10 · viola::cli_version_gate 8 · viola::cli_instance_state 7 · viola::cli_fake_agent 6 · viola::tui_passthrough 5 · viola::conpty_sideload 5 · viola-e2e::harness_lifecycle 4 · viola::hook_events 3 · viola::tui_env_strip 2 · viola-e2e::cli 2 · viola::channel_endpoint 2 · viola::cli_program_resolution 1 · viola::contract_diag_schema 1 · viola::contract_ledger_probes 1 = 80.
First panic lines (run 2): "viola never exited" ×32 · "the wrapped program never started" ×12 · "wrapper builder not ready" ×7 · "assertion `left == right` failed: {"v":1,"cmd":"boot","ok":false,"reason":"readiness-timeout","instance":"builder","exit" ×6 · "timed out waiting for start" ×5 · "timed out waiting for env" ×2 · "the wrapper never spawned the child" ×2 · "timed out waiting for step 0" ×1.
Contrast: the unmutated viola package suite PASSED as cargo-mutants' baseline in a copied tree (gitignored files not copied) at 10:54Z (367/367 tests across 26 binaries passed, 36 s build + 10.9 s test, NEXTEST_PROFILE=mutants); the same binaries fail in-repo under coverage (twice) and in a plain in-repo nextest probe (2/2, with every inherited CLAUDE*/VIOLA_*/ANTHROPIC* var unset) - a location/local-state-dependent red, cause unknown.
**Suspected shape:** spawned `viola` processes that never start or never exit when built and run in the repository tree, on a quiet host — the host-load hypothesis holds for run 1 at most; CI on 11f135c is green (overseer).
**Proposal:** treat it as an open red until its cause is known (the project's own rule): diff what the in-repo run sees that cargo-mutants' gitignore-filtered copy does not (gitignored local state under the tree — e.g. `target/e2e-home/`, `target/conpty-seed/`, `target/baseline-target/` named in the handoff — or the tree's location), then re-take coverage for this boundary's record by a correction.

### M3 — mutation tier blind to viola-e2e · baseline-test-failure by invocation
**Movement:** viola-e2e unscored at both boundaries (Epoch 2: declined, 677 planned; Epoch 2b: baseline-test-failure, 727 planned).
**Evidence:** `mutants-viola-e2e*/mutants.out/log/baseline.log` (summarized; dirs removed): 32 of 246 harness tests fail in the unmutated copied tree — they spawn `target\debug\viola-fake-agent.exe`, a root-package bin a `--package=viola-e2e` build never produces (os error 2). Three forms, all baseline-built and tested as `nextest run --package=viola-e2e@0.1.0`: per-unit · `--test-package viola-e2e --test-package viola` · `--test-workspace=true` (cargo-mutants 27.1.0).
**Suspected shape:** the harness's spawn targets are another package's bins; the audit's per-unit form has no prebuild step, while the project's own `run --mutants` prebuilds `viola --features fake-agent` and passes `--copy-target=true`.
**Proposal:** give the boundary tier a viola-e2e form that has the root bins in each copied tree (the project's prebuild + copy-target form, measured for its copy size first, or a viola-e2e test seam that builds what it spawns), so the 727 mutants of the largest unit enter the gate.

## Informational
- complexity top-10 entrants: sgr_attributes tests/cli_output_plain.rs:36 (23) · mutants crates/viola-e2e/src/harness/run/mutants.rs:99 (12) · resize_reaches_the_child tests/tui_passthrough.rs:108 (12) · native crates/viola-e2e/src/harness/pre_push/linux.rs:610 (11) · tool_arms crates/viola-e2e/src/harness/run.rs:286 (11) — `sgr_attributes` is the one function over the cognitive ceiling (23 > 15), a test helper.
- sizes top-10 entrants: crates/viola-agent-claude/src/ledger.rs 884 · crates/viola-e2e/src/harness/run.rs 866 · crates/viola-e2e/src/harness/pre_push/linux.rs 625 · src/cmd/run.rs 619
- hotspot entrants: crates/viola-e2e/src/harness/run.rs 66 · src/main.rs 54 · tests/hook_fail_open.rs 45 · crates/viola-e2e/src/harness/pre_push/linux.rs 33 · crates/viola-agent-claude/src/lib.rs 27 — top: crates/viola-e2e/src/harness/run/mutants.rs 132
- fan-in top-20 entrants: `rust-analyzer cargo viola-agent-claude 0.1.0 hook/HookEvent#` 61 · `rust-analyzer cargo viola 0.1.0 support/home/home#` 47 · `rust-analyzer cargo viola 0.1.0 support/home/impl#[TestHome]scratch().` 42
- duplication top-10 entrants: tests/hook_fail_open.rs:338 ↔ tests/hook_fail_open.rs:409 20 L · tests/cli_controls_not_disableable.rs:47 ↔ tests/hook_fail_open.rs:102 15 L · tests/cli_verify.rs:340 ↔ tests/run_cli.rs:114 15 L · crates/viola-e2e/src/harness/cleanup.rs:130 ↔ tests/support/home.rs:371 13 L · crates/viola-agent-claude/src/ledger.rs:412 ↔ tests/support/hygiene.rs:44 10 L — split now src {'pairs': 20, 'lines': 151} · test {'pairs': 25, 'lines': 236} · mixed {'pairs': 8, 'lines': 82}
- graph: cross-unit edges 10 → 11 (new `viola-agent-claude → viola-core`); cycles 0; fan-out viola 5 · viola-e2e 3 · the three library crates 1 each.

## Below threshold — no action
- new-cycle · graph.cycles: 0 → 0
- duplication-up · duplication.pct: 1.29 → 1.35
- complexity-creep · complexity.over_ceiling: 0 → 1
- dead-growth · dead.zero_ref_candidates: 0 → 0
- coverage-drop · coverage.line: 96.77 → None (not evaluable)
- mutation-drop · mutation.scores.viola-core: 100.0 → 100.0
- mutation-drop · mutation.scores.viola-pty: 100.0 → 94.67
- mutation-drop · mutation.scores.viola-state: 96.0 → 97.25
- mutation-drop · mutation.scores.viola-channel: 100.0 → 100.0
- mutation-drop · mutation.scores.viola-agent-claude: 100.0 → 100.0
- mutation-drop · mutation.scores.viola: None → 99.67 (not evaluable)
- mutation-drop · mutation.scores.viola-e2e: None → None (not evaluable)
- monotonic · duplication.pct: 1.42 → 1.29 → 1.35 — not worsened at both diffs
- monotonic · complexity.over_ceiling: 3 → 0 → 1 — not worsened at both diffs
- monotonic · dead.zero_ref_candidates: 0 → 0 → 0 — not worsened at both diffs
- monotonic · sizes.file_max: 1562 → 1335 → 884 — not worsened at both diffs
- monotonic · sizes.over_800: 1 → 4 → 2 — not worsened at both diffs
- monotonic · coverage.line: 98.21 → 96.77 → None — not evaluable: null at one of the three records (coverage skip)
- duplication volume: clones 34 → 53 · duplicated lines 274 → 416 · population (jscpd total_lines) 21210 → 30745 (+45.0 %) — pct rose, so not a count-under-ratio line
- churn 16.97 → 18.41 % · files churned 30 → 41
- sizes: p50 191 → 222 · p90 609 → 617 · max 1335 → 884 · over 800 4 → 2; population 68 → 98 files, 18657 → 26689 LOC
- complexity percentiles: cyclomatic p50/p90 1/4 → 1/4 · cognitive p50/p90 0/2 → 0/2
- dead: candidates 0 → 0 (817 raw zero-ref: 792 tests/-segment, 22 FP-classed, 3 residuals reviewed by hand as FP); unused deps: viola-e2e `proc-macro2` gone, fuzz `arbitrary` standing
- mutation scores (corrected baseline → now): viola-core 100.0 → 100.0 · viola-pty 100.0 → 94.67 · viola-state 96.0 → 97.25 · viola-channel 100.0 → 100.0 · viola-agent-claude 100.0 → 100.0 · viola None → 99.67 · viola-e2e None → None

## Skips
- mutation:viola-e2e — baseline-test-failure: the unmutated tree fails 32 of 246 harness tests in cargo-mutants' copied tree: they spawn target/debug/viola-fake-agent.exe, a ROOT-package bin a -p viola-e2e build never produces (os error 2). Three forms tried, all built and tested --package=viola-e2e only: per-unit; --test-package viola-e2e --test-package viola; --test-workspace=true. The project's own run --mutants prebuild + --copy-target form was not tried.
- coverage — baseline-test-failure: the project coverage command's test run failed both times (90, then 80 failed; 79 common); run 2's test phase had 0 foreign rustc in 14 samples and 8-24% CPU, so the host-load hypothesis is not supported for run 2
- coverage.branch — declined: the project coverage command does not instrument branches (as at the 69abc0d record)
