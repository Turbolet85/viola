# Code Audit — viola · Epoch 3 — Windows slice II: driving verbs and live proof · 2026-10-08T16:01:57Z
mode trend · HEAD e304994 · baseline 95c1a9b (Epoch 2b — Windows slice I b: events and ledger) · span 1
overshoot: 0 commits — HEAD is the boundary (the flip of `2026-10-08-first-live-test-and-self-drive`). The baseline record sat 3 commits past its own boundary with 0 source files in the delta; that delta is attributed to its epoch and not re-diffed here.
no ancestry break · no trend-break (all nine tool tokens equal the baseline's)
mutation tier host: `x86_64-unknown-linux-gnu`; the baseline record's was `x86_64-pc-windows-msvc` — the two records measure opposite halves of the `cfg` split, so each unit's `not measured` set differs by construction.

The ledger after this append, the six tracked scalars per record:

| record (ts) | epoch | sha | dup % | over ceiling | zero-ref | file max | files > 800 | line cov % |
|---|---|---|---|---|---|---|---|---|
| 2026-09-24T17:51:10Z | Epoch 1 | a28f696 | 1.42 | 3 | 0 | 1562 | 1 | 98.21 |
| 2026-09-27T13:54:15Z | Epoch 2 | 69abc0d | 1.29 | 0 | 0 | 1335 | 4 | 96.77 |
| 2026-10-01T12:09:55Z | Epoch 2b | 95c1a9b | 1.35 | 1 | 0 | 884 | 2 | null (skip) |
| 2026-10-08T16:01:57Z | Epoch 3 | e304994 | 3.13 | 4 | 0 | 2678 | 12 | 98.12 |

## Proposals

### M1 — duplication-up · duplication.pct — +1.78 pt
**Movement:** 1.35 % → 3.13 % (Epoch 2b → Epoch 3); +131.9 % relative. Fires at +0.5 pt and +15 %.
**Evidence:** clones 53 → 189; duplicated lines 416 → 1668 of 30745 → 53261 (the population grew 73.2 %, the duplicated lines 301.0 %). Split by path, pairs / lines: src 20 / 151 → 73 / 622 · test 25 / 236 → 103 / 1095 · mixed 8 / 82 → 13 / 140. The ten largest fragments (`c-duplication.json`):

| first | second | lines | file pair in the baseline top list |
|---|---|---|---|
| `tests/cli_answer.rs:134` | `tests/cli_wait_last.rs:132` | 29 | new |
| `tests/cli_answer.rs:56` | `tests/tui_wheel.rs:33` | 27 | new |
| `tests/cli_verify.rs:979` | `tests/cli_verify.rs:1067` | 27 | new |
| `tests/cli_answer.rs:56` | `tests/cli_wheel.rs:36` | 26 | new |
| `tests/cli_wheel.rs:68` | `tests/tui_wheel.rs:102` | 25 | new |
| `tests/cli_verify.rs:979` | `tests/cli_verify.rs:1026` | 24 | new |
| `crates/viola-core/src/obs.rs:175` | `tests/contract_diag_schema.rs:20` | 21 | standing |
| `tests/cli_wheel.rs:92` | `tests/tui_wheel.rs:125` | 21 | new |
| `crates/viola-e2e/src/harness/mod.rs:367` | `tests/cli_instance_state.rs:482` | 20 | new |
| `tests/hook_fail_open.rs:465` | `tests/hook_fail_open.rs:536` | 20 | standing |

**Suspected shape:** the growth concentrates in the driving-verb test files — by duplicated lines a file takes part in (`c-duplication-perfile.json`): `tests/cli_verify.rs` 363, `tests/cli_answer.rs` 336, `src/run/send.rs` 254, `tests/cli_send.rs` 198, `tests/cli_wheel.rs` 188, `tests/hook_fail_open.rs` 170, `tests/tui_wheel.rs` 150, `tests/cli_controls_not_disableable.rs` 122; 104 of the 189 pairs sit inside one file. The largest cross-file fragments are per-file copies of the same test scaffolding: a child-run-and-wait helper (`cli_answer.rs:134` / `cli_wait_last.rs:132`), a wrapper boot that waits for `session-start` and an `events()` reader (`cli_answer.rs:56` / `tui_wheel.rs:33` / `cli_wheel.rs:36`).
**Proposal:** a direction for the founder's judgment — lift that scaffolding (the child-run helper, the booted-wrapper-until-session-start helper, the events reader) into `tests/support/`, and look at the within-file repeats of `tests/cli_verify.rs` and `src/run/send.rs` as candidates for one helper each.

### M2 — complexity-creep · complexity.over_ceiling — +3
**Movement:** 1 → 4 functions over cognitive 15 (Epoch 2b → Epoch 3). Fires at +3 and +25 %.
**Evidence:** cyclomatic p90 4.0 → 5.0, cognitive p90 2.0 → 2.0 (unchanged), max 23.0 → 23.0. The functions over the ceiling (`c-complexity.json`):

| function | site | cognitive | over the ceiling at the baseline |
|---|---|---|---|
| `sgr_attributes` | `tests/cli_output_plain.rs:36` | 23.0 | yes |
| `dialog_variants` | `crates/viola-agent-claude/src/ledger.rs:874` | 19.0 | no — entered this epoch |
| `record` | `src/cmd/verify.rs:388` | 17.0 | no — entered this epoch |
| `submit` | `src/bin/viola-fake-agent.rs:452` | 16.0 | no — entered this epoch |

**Suspected shape:** three entrants, each in a file that is also a B2 hotspot this epoch: `dialog_variants` in `ledger.rs` (hotspot 95, the file itself 884 → 2 678 code lines), `record` in `src/cmd/verify.rs` (51), `submit` in `src/bin/viola-fake-agent.rs` (160).
**Proposal:** a direction — split each of the three entrants where its branches already separate; `sgr_attributes` is a test helper that stood over the ceiling at the baseline too.

### M3 — monotonic · duplication.pct and complexity.over_ceiling — worse at both of the last two diffs
**Movement:** `duplication.pct` 1.29 → 1.35 → 3.13; `complexity.over_ceiling` 0 → 1 → 4 (Epoch 2 → Epoch 2b → Epoch 3; the chain by sha 69abc0d → 95c1a9b → e304994).
**Evidence:** the closed set of six, each over the same three records: `duplication.pct` 1.29 → 1.35 → 3.13 (fires) · `complexity.over_ceiling` 0 → 1 → 4 (fires) · `dead.zero_ref_candidates` 0 → 0 → 0 (no) · `sizes.file_max` 1335 → 884 → 2678 (no) · `sizes.over_800` 4 → 2 → 12 (no) · `coverage.line` 96.77 → null → 98.12 (not evaluable).
**Suspected shape:** both scalars are M1's and M2's own subjects; the Epoch 2b step was small (+0.06 pt, +1) and this epoch's is the large one.
**Proposal:** a direction — M1 and M2 taken together in one cleanup chunk, as followed each of the three earlier boundaries (`master-route.md`: epoch-1-cleanup, epoch-2-cleanup, epoch-2b-cleanup); no separate action beyond them.

## Informational

- **Sizes (no single-epoch rule in the table):** file max 884 → 2678 (`crates/viola-agent-claude/src/ledger.rs`), files over 800 code lines 2 → 12, p90 617 → 788; totals 26689 → 46122 code lines in 98 → 124 files (populations, never a movement). Neither size scalar is monotonic (1335 → 884 → 2678; 4 → 2 → 12).
- **Top-N entrants, sizes:** `src/run/send.rs` (1464), `tests/cli_verify.rs` (1061), `src/cmd/hook.rs` (1012), `src/run/dialog.rs` (930).
- **Top-N entrants, complexity:** `dialog_variants` 19.0 (`crates/viola-agent-claude/src/ledger.rs:874`), `record` 17.0 (`src/cmd/verify.rs:388`), `submit` 16.0 (`src/bin/viola-fake-agent.rs:452`), `check_screen` 15.0 (`tests/contract_fixture_hygiene.rs:161`), `step` 14.0 (`src/run/wheel.rs:385`), `dialog_replay` 14.0 (`tests/contract_fake_agent_drift.rs:162`).
- **Top-N entrants, hotspots:** `src/run/send.rs` (121), `crates/viola-agent-claude/src/ledger.rs` (95), `crates/viola-e2e/src/harness/boot.rs` (78), `tests/support/home.rs` (63), `tests/contract_fixture_hygiene.rs` (60), `src/cmd/verify/typed.rs` (55), `src/cmd/verify.rs` (51), `crates/viola-agent-claude/src/screen.rs` (48).
- **Top-N entrants, fan-in:** `viola-channel ProtocolError#` (74), `viola-core EventKind#` (65), `viola support/` (59), `viola support/fake/of_kind().` (55), `viola support/home/impl#[TestHome]new().` (55).
- **Duplication top list:** 8 of the ten largest fragments are new file pairs (M1's table).
- **Churn:** 18.41 % → 48.01 %, files churned 41 → 52 (no rule; 61 commits in the window).
- **Coverage:** line 98.12 % at this record (1 747 of 1 747 tests passed, one run); the baseline record holds null (its test run failed), so `coverage-drop` has no baseline to read. The last scored value is 96.77 % at Epoch 2, named for the eye only.
- **Corrections carried by this record (`corrections[]`, two schema-gap fills on the 95c1a9b record):** `mutation.timing` — scope `unit` for its six scored units, read from its own mutation command, wall time UNKNOWN; `recipes` — the A5 classing recipe recovered from that record's evidence twin and pinned inline in this record's `recipes.dead`. Reading that recipe meant opening five summarizer scripts and two dead-code twins in the baseline's run dir (the files its `commands` name); no judgment or proposal of that run was read.

**Mutation survivors at this boundary — 24 on `x86_64-unknown-linux-gnu`** (no threshold reads them in trend mode; the project's own boundary gate does: `.andromeda/test-plan.md:1212`). Per unit:

| unit | state | caught | missed | not measured | timeout | unviable | score | baseline score | wall s | cap s |
|---|---|---|---|---|---|---|---|---|---|---|
| viola-core | complete 46/46 | 44 | 0 | 0 | 0 | 2 | 100.0 | 100.0 | 126 | 1800 |
| viola-pty | complete 115/115 | 67 | 0 | 25 | 0 | 23 | 100.0 | 94.67 | 449 | 1800 |
| viola-state | complete 269/269 | 166 | 12 | 77 | 0 | 14 | 93.26 | 97.25 | 1133 | 1800 |
| viola-channel | complete 176/176 | 128 | 0 | 31 | 0 | 17 | 100.0 | 100.0 | 271 | 1800 |
| viola-agent-claude | complete 462/462 | 420 | 6 | 0 | 0 | 36 | 98.59 | 100.0 | 644 | 1800 |
| viola | complete 1047/1047 | 886 | 6 | 9 | 0 | 146 | 99.33 | 99.67 | 16623 | 22798 |
| viola-e2e | baseline-test-failure | — | — | — | — | — | — | — (baseline-test-failure there too) | 115 | 1800 |

Score formula `caught/(caught+missed)`; every score at scope `unit`, jobs 1. The survivors, complete:

| unit | site | mutation |
|---|---|---|
| viola-state | `crates/viola-state/src/events.rs:96:19` | replace match guard e.kind() == std::io::ErrorKind::NotFound with true in current_len |
| viola-state | `crates/viola-state/src/events.rs:149:19` | replace match guard e.kind() == ErrorKind::NotFound with true in read_within |
| viola-state | `crates/viola-state/src/events.rs:209:24` | replace < with > in LoggedLines::next_line |
| viola-state | `crates/viola-state/src/fs.rs:270:18` | replace += with *= in replace_private_with |
| viola-state | `crates/viola-state/src/fs.rs:274:20` | replace match guard attempts < REPLACE_ATTEMPTS && retry_replace(e.error.raw_os_error(), HOST_IS_WINDOWS) with false in replace_private_with |
| viola-state | `crates/viola-state/src/fs.rs:275:21` | replace && with \|\| in replace_private_with |
| viola-state | `crates/viola-state/src/fs.rs:274:29` | replace < with == in replace_private_with |
| viola-state | `crates/viola-state/src/fs.rs:274:29` | replace < with > in replace_private_with |
| viola-state | `crates/viola-state/src/fs.rs:274:29` | replace < with <= in replace_private_with |
| viola-state | `crates/viola-state/src/fs.rs:290:19` | replace match guard fs::read(path).ok().as_deref() != Some(bytes) with true in replace_private_shared |
| viola-state | `crates/viola-state/src/stamps.rs:28:19` | replace match guard e.kind() == io::ErrorKind::NotFound with true in read_capped |
| viola-state | `crates/viola-state/src/strict.rs:34:23` | replace match guard e.kind() == io::ErrorKind::NotFound with true in check_stamps |
| viola-agent-claude | `crates/viola-agent-claude/src/ledger.rs:647:81` | replace == with != in both_parallel_answered |
| viola-agent-claude | `crates/viola-agent-claude/src/ledger.rs:670:33` | replace < with <= in parallel_both_before_first_post |
| viola-agent-claude | `crates/viola-agent-claude/src/ledger.rs:685:51` | replace + with * in clear_start |
| viola-agent-claude | `crates/viola-agent-claude/src/ledger.rs:900:76` | replace && with \|\| in dialog_variants |
| viola-agent-claude | `crates/viola-agent-claude/src/ledger.rs:1172:5` | replace is_local_char -> bool with true |
| viola-agent-claude | `crates/viola-agent-claude/src/ledger.rs:1181:74` | replace == with != in has_email |
| viola | `src/bin/viola-fake-agent.rs:328:9` | replace Agent::close_hooks with () |
| viola | `src/bin/viola-fake-agent.rs:437:74` | replace == with != in Agent::run_hook |
| viola | `src/bin/viola-fake-agent.rs:473:46` | replace && with \|\| in Agent::submit |
| viola | `src/bin/viola-fake-agent.rs:473:57` | replace == with != in Agent::submit |
| viola | `src/cmd/mod.rs:133:5` | replace cli_sink -> Option<DetailSink> with None |
| viola | `src/cmd/hook.rs:214:72` | replace > with >= in handle_dialog |

Four of the six `viola` survivors are in the fake agent (`src/bin/viola-fake-agent.rs`, a test-side bin of the root package). The twelve in `viola-state`: a `NotFound` match guard replaced by `true` at four sites (`events.rs:96`, `:149`, `stamps.rs:28`, `strict.rs:34`); six in `replace_private_with`'s retry loop (`fs.rs:270`–`275`), whose guard reads the const `HOST_IS_WINDOWS = cfg!(windows)` — false on this host, so the arm is never taken here; the recipe proves only `#[cfg]` nodes, so they stay missed by this audit's letter (the project's ruling on such a const is at `.andromeda/test-plan.md:1212`); `replace_private_shared`'s content guard (`fs.rs:290`); and `LoggedLines::next_line` (`events.rs:209`).

## Findings outside the threshold table

### F1 — viola-e2e cannot pass its unmutated baseline under the `mutants` profile (both hosts)
**Readings (three, the operator's direction):** (1) here, the baseline's one failure is `viola-e2e::harness_lifecycle boot_with_an_unknown_cli_version_is_verify_failed`, `TIMEOUT [30.003s]`, 257 of 258 passed — the profile's `package(viola-e2e)` override kills at 15 s × 2 (`.config/nextest.toml`); (2) the same test on the same tree in this audit's coverage run, profile `ci`: `PASS [34.477s]`; (3) the Windows runner's `mutants (viola-e2e)` job, run 37761947926: `TIMEOUT [30.009s]`, 249 of 250 passed, `mutants-exit-4`.
**Host:** backing `tmpfs` behind the `target/e2e-home` link; hostwatch for 2026-10-08T11:00:00Z..11:02:10Z reads QUIET (io some peak 5 %, load peak 4.8). Not re-run.
**Suspected shape:** the test's 34.5 s matches four waits of the gate's 8.5 s maximum — the shape the profile's own comment gives a `verify_window_` test — and is above the kill the profile gives its package (30 s).The profile's `verify_window_` override (45 s) does not reach it: the test carries no such name, and the package override sits first.
**Consequence:** the unit is unscored in the ledger for the second boundary running (a different cause at Epoch 2b), and the Windows job cannot grade its 68 mutants.
**Proposal:** a direction — give this test a kill above its designed wait (a name the `verify_window_` override matches, or an override of its own), the founder's choice of which.

### F2 — the Windows workflow's `viola` job no longer fits its 120-minute ceiling
**Readings:** run 37761947926, job `mutants (viola)`: started 10:12:36Z, cancelled 12:13:03Z (`The operation was canceled`), 125 of 140 mutants graded. Its unmutated baseline took 123 s build + 104 s test, and a graded mutant's test phase a median 89 s (101 of the 125 reached their tests); the first dispatch (run 37174673472, 60c569b) read 88 s + 10 s and finished 131 mutants in 26 m 49 s.
**Consequence:** 15 mutants of that job's scope have no Windows grade at this boundary, and the job has no harness document.
**Proposal:** a direction — the root package's test phase on the Windows runner grew about tenfold across the epoch; either the ceiling or the job's split (per file) would bring it back inside, and the growth itself may deserve a look.

### F3 — 15 Windows-side survivors that are not `cfg(unix)` twins
**Readings:** 11 in Windows-only bodies and 4 in shared bodies (the CARRY 1 table below lists each). Among them the strict-modes entry points: `check_stamps → Ok(())`, `check_path → Ok(())` and `win::check → Ok(())` in `crates/viola-state/src/strict.rs` all read MISSED on the Windows runner, the first two caught on Linux.
**Suspected shape:** no test that runs on the `windows-2025` runner fails when the Windows strict-modes check answers `Ok` unconditionally; the killing tests are Unix-side.
**Proposal:** a direction — a Windows-side refusal test for the strict-modes path (and for `win::protect` / `win::dacl_of` / `console::is_console`), or an equivalence argument per site; the route entry that owns the Windows DACL check is the natural home.

## Route CARRYs answered (`working-route.md:105`)

### CARRY 1 — the Windows-dispatch survivors and their `cfg(unix)` twins
A fresh dispatch at this boundary, report-only and outside the ledger: `gh workflow run windows-mutants.yml --ref build/viola-0.1.0` → run **37761947926** on `e304994`. Per job, from its own log:

| job (id) | found | graded | caught | unviable | missed | timeout | harness document |
|---|---|---|---|---|---|---|---|
| mutants (viola-pty) (113260249676) | 89 | 89 | 70 | 11 | 8 | 0 | ok:false |
| mutants (viola-channel) (113260249908) | 148 | 148 | 127 | 17 | 4 | 0 | ok:false |
| mutants (viola-state) (113260250051) | 159 | 159 | 130 | 11 | 18 | 0 | ok:false |
| mutants (viola-agent-claude) (113260250053) | 40 | 40 | 36 | 4 | 0 | 0 | ok:true |
| mutants (viola) (113260249970) | 140 | 125 | 88 | 24 | 13 | 0 | none (cancelled at 120 min) |
| mutants (viola-e2e) (113260250076) | 68 | 0 | 0 | 0 | 0 | 0 | ok:false, mutants-exit-4 |

The 43 MISSED, each classified by two independent readings: the pinned `cover()` recipe under the runner's own cfg set (`rustc --print cfg --target x86_64-pc-windows-msvc`), and this audit's Linux grade of the same mutant (same sha, joined on the tool's name). By class: cfg twin: the Windows build excludes the span **24** · Windows-only body, missed on Windows **11** · excluded on both hosts **4** · shared body: missed on Windows, caught on Linux **3** · shared body: missed on both hosts **1**.

| job | site | mutation | class | Windows cover | Linux reading |
|---|---|---|---|---|---|
| pty | `crates/viola-pty/src/lib.rs:321:9` | replace HostTerminal::enter -> Option<Self> with None | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| pty | `crates/viola-pty/src/lib.rs:321:9` | replace HostTerminal::enter -> Option<Self> with Some(Default::default()) | cfg twin: the Windows build excludes the span | `cfg(unix)` | unviable |
| pty | `crates/viola-pty/src/lib.rs:323:64` | replace != with == in HostTerminal::enter | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| pty | `crates/viola-pty/src/lib.rs:328:71` | replace == with != in HostTerminal::enter | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| pty | `crates/viola-pty/src/lib.rs:434:9` | replace console::is_console -> bool with true | Windows-only body, missed on Windows | — | not measured on Linux: cfg(windows) |
| pty | `crates/viola-pty/src/lib.rs:434:9` | replace console::is_console -> bool with false | Windows-only body, missed on Windows | — | not measured on Linux: cfg(windows) |
| pty | `crates/viola-pty/src/lib.rs:436:76` | replace != with == in console::is_console | Windows-only body, missed on Windows | — | not measured on Linux: cfg(windows) |
| pty | `crates/viola-pty/src/lib.rs:487:76` | replace != with == in host_size | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| channel | `crates/viola-channel/src/client.rs:272:5` | replace open_by -> io::Result<Stream> with Ok(Default::default()) | cfg twin: the Windows build excludes the span | `cfg(unix)` | unviable |
| channel | `crates/viola-channel/src/endpoint.rs:76:5` | replace host_socket_dir -> PathBuf with Default::default() | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| channel | `crates/viola-channel/src/server.rs:91:9` | replace <impl Drop for Guard>::drop with () | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| channel | `crates/viola-channel/src/server.rs:124:5` | replace listen -> Result<(Listener, Guard), ChannelError> with Ok((Default::default(), Default::default())) | cfg twin: the Windows build excludes the span | `cfg(unix)` | unviable |
| state | `crates/viola-state/src/fs.rs:18:5` | replace restrict -> io::Result<()> with Ok(()) | shared body: missed on Windows, caught on Linux | — | caught |
| state | `crates/viola-state/src/fs.rs:161:47` | replace \| with ^ in win::protect | Windows-only body, missed on Windows | — | not measured on Linux: cfg(windows) |
| state | `crates/viola-state/src/fs.rs:187:13` | replace \|\| with && in win::dacl_of | Windows-only body, missed on Windows | — | not measured on Linux: cfg(windows) |
| state | `crates/viola-state/src/fs.rs:186:13` | replace \|\| with && in win::dacl_of | Windows-only body, missed on Windows | — | not measured on Linux: cfg(windows) |
| state | `crates/viola-state/src/strict.rs:30:5` | replace check_stamps -> Result<(), Refused> with Ok(()) | shared body: missed on Windows, caught on Linux | — | caught |
| state | `crates/viola-state/src/strict.rs:34:23` | replace match guard e.kind() == io::ErrorKind::NotFound with true in check_stamps | shared body: missed on both hosts | — | missed |
| state | `crates/viola-state/src/strict.rs:43:5` | replace check_path -> Result<(), Refused> with Ok(()) | shared body: missed on Windows, caught on Linux | — | caught |
| state | `crates/viola-state/src/strict.rs:120:9` | replace unix::reading -> io::Result<(u32, u32)> with Ok((0, 0)) | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| state | `crates/viola-state/src/strict.rs:120:9` | replace unix::reading -> io::Result<(u32, u32)> with Ok((0, 1)) | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| state | `crates/viola-state/src/strict.rs:120:9` | replace unix::reading -> io::Result<(u32, u32)> with Ok((1, 0)) | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| state | `crates/viola-state/src/strict.rs:120:9` | replace unix::reading -> io::Result<(u32, u32)> with Ok((1, 1)) | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| state | `crates/viola-state/src/strict.rs:126:9` | replace unix::euid -> u32 with 0 | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| state | `crates/viola-state/src/strict.rs:126:9` | replace unix::euid -> u32 with 1 | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| state | `crates/viola-state/src/strict.rs:163:9` | replace win::check -> Result<(), Refused> with Ok(()) | Windows-only body, missed on Windows | — | not measured on Linux: cfg(windows) |
| state | `crates/viola-state/src/strict.rs:170:9` | replace win::persistent_acls -> Option<bool> with Some(true) | Windows-only body, missed on Windows | — | not measured on Linux: cfg(windows) |
| state | `crates/viola-state/src/strict.rs:192:37` | replace & with \| in win::persistent_acls | Windows-only body, missed on Windows | — | not measured on Linux: cfg(windows) |
| state | `crates/viola-state/src/strict.rs:192:37` | replace & with ^ in win::persistent_acls | Windows-only body, missed on Windows | — | not measured on Linux: cfg(windows) |
| state | `crates/viola-state/src/strict.rs:207:44` | replace \| with ^ in win::owner_and_dacl | Windows-only body, missed on Windows | — | not measured on Linux: cfg(windows) |
| viola | `src/panic_frames.rs:69:5` | replace raw_frames -> Vec<usize> with vec![] | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| viola | `src/panic_frames.rs:69:5` | replace raw_frames -> Vec<usize> with vec![0] | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| viola | `src/panic_frames.rs:69:5` | replace raw_frames -> Vec<usize> with vec![1] | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| viola | `src/panic_frames.rs:80:5` | replace module_of -> Option<(String, usize)> with None | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| viola | `src/panic_frames.rs:80:5` | replace module_of -> Option<(String, usize)> with Some((String::new(), 0)) | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| viola | `src/panic_frames.rs:80:5` | replace module_of -> Option<(String, usize)> with Some((String::new(), 1)) | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| viola | `src/panic_frames.rs:80:5` | replace module_of -> Option<(String, usize)> with Some(("xyzzy".into(), 0)) | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| viola | `src/panic_frames.rs:80:5` | replace module_of -> Option<(String, usize)> with Some(("xyzzy".into(), 1)) | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| viola | `src/panic_frames.rs:82:77` | replace == with != in module_of | cfg twin: the Windows build excludes the span | `cfg(unix)` | caught |
| viola | `src/cmd/run.rs:385:5` | replace sideload_outcome -> (&'static str, Option<viola_state::pin::HeldCompanions>) with ("", None) | excluded on both hosts | `cfg(all(windows, not(target_arch = "x86_64")))` | not measured on Linux: cfg(all(windows, not(target_arch = "x86_64"))) |
| viola | `src/cmd/run.rs:385:5` | replace sideload_outcome -> (&'static str, Option<viola_state::pin::HeldCompanions>) with ("", Some(Default::default())) | excluded on both hosts | `cfg(all(windows, not(target_arch = "x86_64")))` | not measured on Linux: cfg(all(windows, not(target_arch = "x86_64"))) |
| viola | `src/cmd/run.rs:385:5` | replace sideload_outcome -> (&'static str, Option<viola_state::pin::HeldCompanions>) with ("xyzzy", None) | excluded on both hosts | `cfg(all(windows, not(target_arch = "x86_64")))` | not measured on Linux: cfg(all(windows, not(target_arch = "x86_64"))) |
| viola | `src/cmd/run.rs:385:5` | replace sideload_outcome -> (&'static str, Option<viola_state::pin::HeldCompanions>) with ("xyzzy", Some(Default::default())) | excluded on both hosts | `cfg(all(windows, not(target_arch = "x86_64")))` | not measured on Linux: cfg(all(windows, not(target_arch = "x86_64"))) |

**The answer.** The twins are host-excluded, not survivors: all 24 sit in a span the Windows build removes, and on Linux they grade 21 caught, 3 unviable — none missed. The four `sideload_outcome` mutants (`src/cmd/run.rs:385:5`, `cfg(all(windows, not(target_arch = "x86_64")))`) are excluded on both measured hosts: no host this project builds on compiles that body.
Against the first dispatch's 25 (run 37174673472, `evidence/windows-dispatch.md` of chunk 2026-10-04-windows-boundary-mutation-workflow), matched by mutation text since the lines moved: the 4 not measurable are the four above; of the 19 `cfg(unix)` bodies, the 18 in viola-channel (4), viola-pty (5) and `src/panic_frames.rs` (9) are in the table as twins with their Linux grades (viola-state's six `unix::reading` / `unix::euid` twins are new to that job's scope since then); the Windows-equivalent `restrict → Ok(())` (now `fs.rs:18:5`) is missed on Windows and caught on Linux, as recorded then; the two viola-e2e coordinates (`cleanup.rs` `unconnectable`'s twin and the shared-body `+ → -` on the kill deadline) have no Windows grade at this dispatch — the job died on F1 — and their Linux reading is in the report-only run below.
**What classifying does not do.** The jobs do not turn green by it: beside the twins the run holds 15 survivors in code the runner does build (F3), one job over its ceiling (F2) and one baseline failure (F1). A direction, not a verdict: the harness's Windows arm could subtract a mutant whose covering predicate is false on its host before it counts `missed` — the recipe is `cover.py` in this run dir — so the verdict row reads over the measurable set.

### CARRY 2 — what a mutation run's copied tree does with the `target/e2e-home` link
**The link is carried as a link.** In all 8 invocations of this audit (seven units and the report-only run) cargo-mutants 27.1.0's copy under `TMPDIR` held `target/e2e-home` as a symlink (`lstat` mode `0o120777`) to `/tmp/viola-e2e-home-1000` — never a plain directory, never a copy of the homes. So a mutation run's test homes land on the same owner-only tmpfs as the working tree's, outside the copy; the copy's removal does not remove them.
**What the tmpfs saw.** Entries under `/tmp/viola-e2e-home-1000` sampled every 15 s: between 18 and 56 across the tier; `/tmp` used between 9.9 and 13.8 GiB of 32. No quota error in any unit's log.
**The keeper's mutants** (`Workspace::ensure_e2e_home`, first mutated here; from the report-only run, 6 mutants): `mod.rs:94:9` replace Workspace::ensure_e2e_home -> io::Result<()> with Ok(()) — caught; `mod.rs:102:20` delete ! in Workspace::ensure_e2e_home — caught; `mod.rs:109:20` delete ! in Workspace::ensure_e2e_home — caught; `mod.rs:114:54` replace != with == in Workspace::ensure_e2e_home — caught; `mod.rs:114:46` replace & with \| in Workspace::ensure_e2e_home — caught; `mod.rs:114:46` replace & with ^ in Workspace::ensure_e2e_home — caught.
**`backing/`.** `crates/viola-e2e/backing` appeared in the copied tree at 2026-10-08T15:45:15Z (mode `0o40700`, 0 entries; the tool's last line then: `caught   crates/viola-e2e/src/harness/mod.rs:109:20: delete ! in Workspace::ensure_e2e_home in 1s build + 5s test`). It is the relative-target mutant's (`mod.rs:102:20`, `delete !`): the directory's own mtime reads 15:45:03Z, between the last write of the `:94:9` mutant's log (15:45:01Z) and of the `:102:20` mutant's (15:45:08Z) — with the check inverted, the keeper's refusal test hands it the relative target `backing` and the keeper makes it under the test's working directory, as chunk 2026-10-07-test-homes-off-the-contended-volume measured by hand (`evidence/keeper-control.md`, pair 3). It was still standing, empty, two minutes later, so every later mutant of that invocation ran with it present; it sat inside the copy only — nothing named `backing` is in the repository, and the copy went when the invocation was interrupted.
**Beside it.** `backing/` is not the only thing a mutated harness leaves at its tests' working directory: at 2026-10-08T15:47:06Z the copy's `crates/viola-e2e/` also held `.x`, `a`, `b`, `c`, `junit-coverage.xml`, `junit-playwright.xml`, `run-summary.json`, `s`, `viola-session-D3FTnh` (9 entries made after the copy, `carry-copy-droppings.json`) — all inside the copy. And the copy carries the repository's untracked, gitignored entries with it: the eleven `.viola-verify-*` probe dirs at the root and `viola-0.2.0-incubator/`.
**One thing in the repository itself, not made by this run:** `crates/viola-e2e/.viola-verify-2676638-plan/` (mode 0700, dated 2026-10-05T13:18Z, holding one empty `plans/` dir) — a killed verify's probe dir under a member's directory, where the root-anchored `/.viola-verify-*/` ignore does not reach and `git status` shows nothing because it holds no file. Left where it is.

### The report-only viola-e2e run (the operator's answers; never a ledger score)
`cargo mutants --package viola-e2e … -- -E 'not test(=boot_with_an_unknown_cli_version_is_verify_failed)'`, the same form otherwise; baseline Success (257 passed, 1 skipped). Stopped on the operator's second answer once `harness/mod.rs` and `harness/cleanup.rs` were graded: **165 of 718** tested in 1314 s (about 12 s a mutant; the whole unit would have taken some 2.5 h) — a prefix, not the unit. Of the tested: 139 caught · 0 missed · 0 not measured · 0 timeout · 26 unviable. Files graded whole: `viola-harness.rs`, `boot.rs`, `cleanup.rs`, `mod.rs`. No mutant of the tested prefix read missed.

CARRY 1's two viola-e2e coordinates on Linux, from this run: `crates/viola-e2e/src/harness/cleanup.rs:106:35` replace + with - in cleanup_one — **caught**; `crates/viola-e2e/src/harness/cleanup.rs:140:9` delete ! in unconnectable — **caught**.
The project's own ruling on `harness/run/mutants/scratch.rs` `prepare` (behind `HOST_SCRATCH = cfg!(windows)`, a const the recipe cannot prove false, so it stays `missed` by this audit's letter) is registered at `.andromeda/test-plan.md:1212`.

## Below threshold — no action

- mutation-drop · viola-core: 100.0 → 100.0 (+0.0 pt), both at scope `unit`; the rule fires at −10 pt. Hosts differ (x86_64-pc-windows-msvc → x86_64-unknown-linux-gnu).
- mutation-drop · viola-pty: 94.67 → 100.0 (+5.33 pt), both at scope `unit`; the rule fires at −10 pt. Hosts differ (x86_64-pc-windows-msvc → x86_64-unknown-linux-gnu).
- mutation-drop · viola-state: 97.25 → 93.26 (-3.99 pt), both at scope `unit`; the rule fires at −10 pt. Hosts differ (x86_64-pc-windows-msvc → x86_64-unknown-linux-gnu).
- mutation-drop · viola-channel: 100.0 → 100.0 (+0.0 pt), both at scope `unit`; the rule fires at −10 pt. Hosts differ (x86_64-pc-windows-msvc → x86_64-unknown-linux-gnu).
- mutation-drop · viola-agent-claude: 100.0 → 98.59 (-1.41 pt), both at scope `unit`; the rule fires at −10 pt. Hosts differ (x86_64-pc-windows-msvc → x86_64-unknown-linux-gnu).
- mutation-drop · viola: 99.67 → 99.33 (-0.34 pt), both at scope `unit`; the rule fires at −10 pt. Hosts differ (x86_64-pc-windows-msvc → x86_64-unknown-linux-gnu).
- mutation-drop · viola-e2e: no score at either record (`baseline-test-failure` both times) — passed over, never a zero.
- dead-growth: zero-ref candidates 0 → 0 (raw 1 277 → 42 after the tests filter → 5 read by hand, all false-positive families; `c-dead.json`). Unused deps unchanged: the fuzz workspace's `arbitrary`.
- new-cycle: 0 → 0; cross-unit edges 11 → 11; fan-out unchanged.
- coverage-drop: not evaluable — the baseline's `coverage.line` is null (skip `baseline-test-failure` there).
- monotonic, the other four: `dead.zero_ref_candidates` flat; `sizes.file_max` and `sizes.over_800` improved at the previous diff; `coverage.line` not evaluable this run (null at the baseline record, its skip `baseline-test-failure`).
- count-under-ratio: not applicable — clones and duplicated lines rose and their ratio rose with them (M1).
- complexity percentiles: cyclomatic p50 1.0 → 1.0, p90 4.0 → 5.0; cognitive p50 0.0 → 0.0, p90 2.0 → 2.0.

## Skips

| metric | reason | note |
|---|---|---|
| mutation:viola-e2e | baseline-test-failure | the unmutated tree's test run timed out on one test, viola-e2e::harness_lifecycle boot_with_an_unknown_cli_version_is_verify_failed, killed at the mutants profile's 30 s (package(viola-e2e): 15 s x 2); 257 of 258 passed. Three readings: the 30 s kill here; 34.477 s PASS for the same test in this run's coverage command (profile ci); the Windows runner's job killed it at 30.009 s (run 37761947926). Backing tmpfs, hostwatch QUIET for the window (2026-10-08T11:00:00Z..11:02:10Z). Not re-run. A report-only run with that test deselected (the operator's answer; stopped on a second answer at 165 of 718: 139 caught, 0 missed, 26 unviable) is in the run dir, never in scores. |
| coverage.branch | declined | the project coverage command does not instrument branches (as at the 95c1a9b record) |

Left for the operator's desk, outside this run's write surface: `~/dev/projects/viola-mutants-scratch/e3/` (made for this run, NOCOW) holds what the mutated tests left under `TMPDIR` — 112 entries, 244M — and no cargo-mutants copy. Nothing in it is read by any chunk.

A note on this record's `commands.mutation`: it names that scratch by the host's absolute path, the one path in the record that is not repo-relative. A relative `TMPDIR` resolves differently inside each copied tree, so the as-run form was kept and the command says so itself; whether the ledger should carry `<repo parent>/…` there instead is the founder's to rule, and a ruling would ride the next record's `corrections[]`.

## Appendix — not measured on this host (`x86_64-unknown-linux-gnu`): 142

Each a `missed` outcome whose whole span a covering predicate removes from this host's build (the pinned recipe, `cover.py`); never a survivor, never scored. The project's own union verdict is registered at `.andromeda/test-plan.md:1212` (the Windows leg `windows-mutants.yml` measures these); it is named here, not merged.

| site | mutation | predicate |
|---|---|---|
| `crates/viola-pty/src/lib.rs:240:5` | replace terminate -> bool with true | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:240:5` | replace terminate -> bool with false | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:248:46` | replace != with == in terminate | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:296:9` | replace HostTerminal::enter -> Option<Self> with None | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:296:9` | replace HostTerminal::enter -> Option<Self> with Some(Default::default()) | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:310:59` | replace && with \|\| in HostTerminal::enter | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:310:54` | replace != with == in HostTerminal::enter | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:310:95` | replace != with == in HostTerminal::enter | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:315:10` | delete ! in HostTerminal::enter | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:434:9` | replace console::is_console -> bool with true | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:434:9` | replace console::is_console -> bool with false | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:436:76` | replace != with == in console::is_console | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:444:13` | replace console::<impl super::Utf16Source for Console>::read_units -> io::Result<usize> with Ok(0) | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:444:13` | replace console::<impl super::Utf16Source for Console>::read_units -> io::Result<usize> with Ok(1) | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:457:19` | replace == with != in console::<impl super::Utf16Source for Console>::read_units | `cfg(windows)` |
| `crates/viola-pty/src/lib.rs:475:87` | replace == with != in host_size | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:25:5` | replace restrict_dll_search -> bool with true | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:25:5` | replace restrict_dll_search -> bool with false | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:25:86` | replace != with == in restrict_dll_search | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:34:5` | replace search_restricted -> bool with true | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:34:5` | replace search_restricted -> bool with false | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:41:5` | replace preload -> Result<(), PtyError> with Ok(()) | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:41:8` | delete ! in preload | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:58:5` | replace preloaded -> bool with true | `cfg(windows)` |
| `crates/viola-pty/src/sideload.rs:58:5` | replace preloaded -> bool with false | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:95:9` | replace win::wide -> Vec<u16> with vec![] | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:95:9` | replace win::wide -> Vec<u16> with vec![0] | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:95:9` | replace win::wide -> Vec<u16> with vec![1] | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:101:9` | replace win::protected_sddl -> String with String::new() | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:101:9` | replace win::protected_sddl -> String with "xyzzy".into() | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:106:9` | replace win::profile_dir -> io::Result<PathBuf> with Ok(Default::default()) | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:108:86` | replace == with != in win::profile_dir | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:120:19` | replace == with != in win::profile_dir | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:123:46` | replace == with != in win::profile_dir | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:130:9` | replace win::protect_outside_profile -> io::Result<()> with Ok(()) | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:138:9` | replace win::protect -> io::Result<()> with Ok(()) | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:150:17` | replace == with != in win::protect | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:161:47` | replace \| with & in win::protect | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:161:47` | replace \| with ^ in win::protect | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:168:23` | replace == with != in win::protect | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:182:9` | replace win::dacl_of -> io::Result<*mut ACL> with Ok(Default::default()) | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:187:13` | replace \|\| with && in win::dacl_of | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:186:13` | replace \|\| with && in win::dacl_of | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:185:94` | replace == with != in win::dacl_of | `cfg(windows)` |
| `crates/viola-state/src/fs.rs:186:24` | replace == with != in win::dacl_of | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:126:5` | replace retry_open -> bool with true | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:126:5` | replace retry_open -> bool with false | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:126:33` | replace && with \|\| in retry_open | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:126:14` | replace < with == in retry_open | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:126:14` | replace < with > in retry_open | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:126:14` | replace < with <= in retry_open | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:126:49` | replace == with != in retry_open | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:131:5` | replace open_held -> io::Result<File> with Ok(Default::default()) | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:140:5` | replace open_held_waiting -> io::Result<File> with Ok(Default::default()) | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:142:18` | replace += with -= in open_held_waiting | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:142:18` | replace += with *= in open_held_waiting | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:144:23` | replace match guard retry_open(e.raw_os_error(), attempts) with true in open_held_waiting | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:144:23` | replace match guard retry_open(e.raw_os_error(), attempts) with false in open_held_waiting | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:159:5` | replace pin_companions -> Result<HeldCompanions, PinError> with Ok(Default::default()) | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:171:23` | replace match guard e.kind() == io::ErrorKind::NotFound with true in pin_companions | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:171:23` | replace match guard e.kind() == io::ErrorKind::NotFound with false in pin_companions | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:171:32` | replace == with != in pin_companions | `cfg(windows)` |
| `crates/viola-state/src/pin.rs:178:18` | replace != with == in pin_companions | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:159:9` | replace win::wide -> Vec<u16> with vec![] | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:159:9` | replace win::wide -> Vec<u16> with vec![0] | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:159:9` | replace win::wide -> Vec<u16> with vec![1] | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:163:9` | replace win::check -> Result<(), Refused> with Ok(()) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:170:9` | replace win::persistent_acls -> Option<bool> with None | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:170:9` | replace win::persistent_acls -> Option<bool> with Some(true) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:170:9` | replace win::persistent_acls -> Option<bool> with Some(false) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:174:81` | replace == with != in win::persistent_acls | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:192:15` | replace != with == in win::persistent_acls | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:192:55` | replace != with == in win::persistent_acls | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:192:37` | replace & with \| in win::persistent_acls | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:192:37` | replace & with ^ in win::persistent_acls | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:197:9` | replace win::owner_and_dacl -> Option<(String, Option<Vec<Allow>>)> with None | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:197:9` | replace win::owner_and_dacl -> Option<(String, Option<Vec<Allow>>)> with Some((String::new(), None)) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:197:9` | replace win::owner_and_dacl -> Option<(String, Option<Vec<Allow>>)> with Some((String::new(), Some(vec![]))) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:197:9` | replace win::owner_and_dacl -> Option<(String, Option<Vec<Allow>>)> with Some((String::new(), Some(vec![Default::default()]))) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:197:9` | replace win::owner_and_dacl -> Option<(String, Option<Vec<Allow>>)> with Some(("xyzzy".into(), None)) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:197:9` | replace win::owner_and_dacl -> Option<(String, Option<Vec<Allow>>)> with Some(("xyzzy".into(), Some(vec![]))) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:197:9` | replace win::owner_and_dacl -> Option<(String, Option<Vec<Allow>>)> with Some(("xyzzy".into(), Some(vec![Default::default()]))) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:207:44` | replace \| with & in win::owner_and_dacl | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:207:44` | replace \| with ^ in win::owner_and_dacl | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:215:19` | replace != with == in win::owner_and_dacl | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:222:13` | delete match arm Some((owner, Some(aces))) in win::owner_and_dacl | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:229:9` | replace win::aces -> Option<Option<Vec<Allow>>> with None | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:229:9` | replace win::aces -> Option<Option<Vec<Allow>>> with Some(None) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:229:9` | replace win::aces -> Option<Option<Vec<Allow>>> with Some(Some(vec![])) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:229:9` | replace win::aces -> Option<Option<Vec<Allow>>> with Some(Some(vec![Default::default()])) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:238:53` | replace == with != in win::aces | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:243:31` | replace != with == in win::aces | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:260:9` | replace win::sid_string -> Option<String> with None | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:260:9` | replace win::sid_string -> Option<String> with Some(String::new()) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:260:9` | replace win::sid_string -> Option<String> with Some("xyzzy".into()) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:263:62` | replace == with != in win::sid_string | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:267:65` | replace != with == in win::sid_string | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:283:9` | replace win::user_sid -> Option<String> with None | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:283:9` | replace win::user_sid -> Option<String> with Some(String::new()) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:283:9` | replace win::user_sid -> Option<String> with Some("xyzzy".into()) | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:285:86` | replace == with != in win::user_sid | `cfg(windows)` |
| `crates/viola-state/src/strict.rs:297:27` | replace != with == in win::user_sid | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:204:5` | replace retry_busy -> bool with true | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:204:5` | replace retry_busy -> bool with false | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:204:45` | replace && with \|\| in retry_busy | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:204:26` | replace == with != in retry_busy | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:204:52` | replace < with == in retry_busy | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:204:52` | replace < with > in retry_busy | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:204:52` | replace < with <= in retry_busy | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:218:5` | replace open -> io::Result<Stream> with Ok(Default::default()) | `cfg(all(windows, test))` |
| `crates/viola-channel/src/client.rs:223:5` | replace open_by -> io::Result<Stream> with Ok(Default::default()) | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:252:19` | replace != with == in open_by | `cfg(windows)` |
| `crates/viola-channel/src/client.rs:256:12` | delete ! in open_by | `cfg(windows)` |
| `crates/viola-channel/src/server.rs:99:5` | replace listen -> Result<(Listener, Guard), ChannelError> with Ok((Default::default(), Default::default())) | `cfg(windows)` |
| `crates/viola-channel/src/server.rs:110:25` | replace == with != in listen | `cfg(windows)` |
| `crates/viola-channel/src/test_support.rs:66:9` | replace win::canonical_sddl -> String with String::new() | `cfg(windows)` |
| `crates/viola-channel/src/test_support.rs:66:9` | replace win::canonical_sddl -> String with "xyzzy".into() | `cfg(windows)` |
| `crates/viola-channel/src/test_support.rs:93:9` | replace win::user_sid -> String with String::new() | `cfg(windows)` |
| `crates/viola-channel/src/test_support.rs:93:9` | replace win::user_sid -> String with "xyzzy".into() | `cfg(windows)` |
| `crates/viola-channel/src/test_support.rs:116:9` | replace win::dacl_of -> String with String::new() | `cfg(windows)` |
| `crates/viola-channel/src/test_support.rs:116:9` | replace win::dacl_of -> String with "xyzzy".into() | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:21:5` | replace owner_only_sddl -> String with String::new() | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:21:5` | replace owner_only_sddl -> String with "xyzzy".into() | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:25:5` | replace owner_only_descriptor -> io::Result<SecurityDescriptor> with Ok(Default::default()) | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:30:5` | replace user_sid -> io::Result<String> with Ok(String::new()) | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:30:5` | replace user_sid -> io::Result<String> with Ok("xyzzy".into()) | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:32:82` | replace == with != in user_sid | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:42:5` | replace token_user_sid -> io::Result<String> with Ok(String::new()) | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:42:5` | replace token_user_sid -> io::Result<String> with Ok("xyzzy".into()) | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:50:15` | replace == with != in token_user_sid | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:57:68` | replace == with != in token_user_sid | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:77:5` | replace from_sddl -> io::Result<SecurityDescriptor> with Ok(Default::default()) | `cfg(windows)` |
| `crates/viola-channel/src/server/win.rs:88:18` | replace == with != in from_sddl | `cfg(windows)` |
| `src/panic_frames.rs:37:5` | replace raw_frames -> Vec<usize> with vec![] | `cfg(windows)` |
| `src/panic_frames.rs:37:5` | replace raw_frames -> Vec<usize> with vec![0] | `cfg(windows)` |
| `src/panic_frames.rs:37:5` | replace raw_frames -> Vec<usize> with vec![1] | `cfg(windows)` |
| `src/panic_frames.rs:50:5` | replace module_of -> Option<(String, usize)> with None | `cfg(windows)` |
| `src/panic_frames.rs:50:5` | replace module_of -> Option<(String, usize)> with Some((String::new(), 0)) | `cfg(windows)` |
| `src/panic_frames.rs:50:5` | replace module_of -> Option<(String, usize)> with Some((String::new(), 1)) | `cfg(windows)` |
| `src/panic_frames.rs:50:5` | replace module_of -> Option<(String, usize)> with Some(("xyzzy".into(), 0)) | `cfg(windows)` |
| `src/panic_frames.rs:50:5` | replace module_of -> Option<(String, usize)> with Some(("xyzzy".into(), 1)) | `cfg(windows)` |
| `src/panic_frames.rs:55:79` | replace == with != in module_of | `cfg(windows)` |

