# Apply — test-plan · part 4 of 4

## Verdicts
- fanout-test-plan--cascade-g2.md 1 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:36+#1
- fanout-test-plan--cascade-g2.md 2 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:174#1
- fanout-test-plan--cascade-g2.md 3 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:236#1, .andromeda/test-plan-detail/e2e-test-strategy.md:236#2, .andromeda/test-plan-detail/e2e-test-strategy.md:236#3
- fanout-test-plan--cascade-g2.md 4 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:237#1, .andromeda/test-plan-detail/e2e-test-strategy.md:237#2
- fanout-test-plan--cascade-g2.md 8 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:246#1
- fanout-test-plan--cascade-g2.md 10 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:270#1, .andromeda/test-plan-detail/e2e-test-strategy.md:271+#1
- fanout-test-plan--cascade-g2.md 11 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:283#1
- fanout-test-plan--cascade-g2.md 12 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:286#1
- fanout-test-plan--cascade-g2.md 14 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:303#1, .andromeda/test-plan-detail/e2e-test-strategy.md:303#2, .andromeda/test-plan-detail/e2e-test-strategy.md:303#3
- fanout-test-plan--cascade-g2.md 15 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:305#1, .andromeda/test-plan-detail/e2e-test-strategy.md:305#2
- fanout-test-plan--cascade-g2.md 16 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:310+#1
- fanout-test-plan--cascade-g2.md 17 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:310+#2
- fanout-test-plan--cascade-g2.md 19 · applied · .andromeda/test-plan-detail/e2e-test-strategy.md:338+#1
- fanout-test-plan--cascade-g2.md 20 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:10#1
- fanout-test-plan--cascade-g2.md 21 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:10+#1
- fanout-test-plan--cascade-g2.md 22 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:13+#1
- cascade-compare-test-plan.md 4 · applied · .andromeda/test-plan-detail/test-data-fixtures.md:27+#1
- fanout-test-plan--cascade-g2.md 23 · applied · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:26#1, .andromeda/test-plan-detail/quality-gates-coverage-targets.md:26#2
- fanout-test-plan.md 12 · applied · .andromeda/registries/contracts/test-plan/test-data-bootstrap.md:11#1
- cascade-compare-test-plan.md 1 · applied · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:19+#1

## Edits
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:36
  anchor: «  - The `settings.json` half of step 5 is Unix's. On Unix the restart rewrites the instance's `settings.json` whole at 0600 as `{"statusLine":{"type":"command","command":"<pinned path> hook statusline"}}` with the absolute pinned path, and only when every character of that path is an ASCII letter, a digit or one of `_ - . / :`. On Windows no `settings.json` is written. Landed as `run_rewrites_the_settings_override_each_start` (`tests/cli_instance_state.rs`, with a `cfg!(windows)` branch) and the unit `statusline_override_of_a_start_is_rewritten_whole_at_owner_only` (`src/cmd/run.rs`).»
  add: «  - The start runs the home-tree strict-modes check (`check_home_tree`: the home, `check_instance`'s set, `ledger/`, `bin/` and `plugin/`, then `bin/<key>/`, the pinned copy, `plugin/<key>/`, its `hooks/` and `.claude-plugin/`, each that exists) before the wrapper log opens. A refused tree exits 1 with the home-state `unable:` / `hint:` pair, or the instance-state pair when the refusal is at instance level, and opens no log, writes no line and starts no child (`tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log`; `tests/security_negatives_strict_modes.rs` `a_home_another_user_may_read_refuses_run`).»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:174
  old: «`statusline_command` never runs when strict-modes fails.»
  new: «`statusline_command` never runs when strict-modes fails, nor over a replayed snapshot (an unreadable or newer `snapshot.json`, read through `read_snapshot_or_replay`), which carries no `statusline_command`: the arm runs and prints nothing (the unit `statusline_over_a_replayed_snapshot_runs_nothing`).»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:236
  old: «**As landed (chunks 2026-10-10-self-healing-state and 2026-10-11-the-board-viola-list): landed in part.**»
  new: «**As landed: built.**»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:236
  old: «The replay has two product readers, `viola revive` (chunk 2026-10-10-viola-revive) and `viola list`'s `board::read` (chunk 2026-10-11-the-board-viola-list), and stays covered below the surface by `crates/viola-state/tests/state_replay.rs`. `run`'s collision check still calls `read_snapshot`, so step 3's `viola run builder` does not take the replay.»
  new: «The replay has six product readers: `viola revive`, `viola list`'s `board::read`, hook `connect`, `hook statusline`'s `serve`, `run`'s `collision_check` and `viola_state::liveness::live_endpoint`; `read_snapshot` has no product caller under `src/`, and the replay stays covered below the surface by `crates/viola-state/tests/state_replay.rs`.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:236
  old: «step 1's `link`; steps 1–3 stay unbuilt for the `read_snapshot` reason below)»
  new: «step 1's `link`)»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:237
  old: «Steps 1–3 stay unbuilt while `run`'s collision check calls `read_snapshot`; the three other snapshot readers (`read_snapshot(` under the root `src/` outside test modules: `run`'s collision check, `src/cmd/hook.rs` once, `src/cmd/hook/statusline.rs`) stay on `read_snapshot`, and their move onto the replay (`read_snapshot_or_replay`), with what a client does with an unreadable or newer snapshot, is one CARRY on the route entry "Home and code-bearing file integrity".»
  new: «Steps 1–3 run as `tests/chaos_snapshot_replay.rs`: `a_start_over_an_unreadable_snapshot_replays_the_log` and `a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live`. A replayed snapshot carries no `endpoint`, `pid` or `started_at`, so `live_endpoint` returns none (`live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line`), hook `connect` reads the instance unreachable and fails open (`connect_over_a_replayed_snapshot_is_unreachable`), the statusline runs nothing (Path 6 step 6), and `run`'s collision check reads the beat alone: a beat no older than `STALE_AFTER` is `already-live`, an older one leaves the name free (`collision_check_over_a_replayed_snapshot_reads_the_beat_alone`).»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:237
  old: «Two of its signals are set against what landed, for the entry that builds it to restate:»
  new: «Two of its signals read as built:»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:246
  old: «missing `VIOLA_NAME` and, on Linux and Windows,»
  new: «missing `VIOLA_NAME`, a `VIOLA_DIR` naming no existing instance directory (the `no_instance_dir` row: nothing written), an instance its strict-modes instance check refuses (`hook_refused_by_the_instance_check_fails_open_and_writes_nothing`: the event and dialog paths take the check before any `config.json` read or obs init, so exit 0, empty streams, no frame, no line and no SessionEnd append; the two `--capture` arms stay ahead of it, unchanged) and, on Linux and Windows,»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:270
  old: «the home fails strict-modes (Unix 0770 home; Windows synthetic-descriptor stub)»
  new: «the home fails strict-modes (Unix a widened mode; Windows a Users grant through `icacls`, on windows-2025)»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:271
  anchor: « As landed for `viola link` / `viola unlink`, a `tests/cli_links.rs` case, not a `cross_exit_causes.rs` row: `link_verbs_refuse_a_widened_instance_before_any_log` (`cfg(unix)`) makes the driven instance's directory group-writable while its wrapper runs; both verbs, in both modes, exit 21 with the fixed instance-state `unable:` / `hint:` pair or the `instance-unreachable` document with stderr empty, the verb's `cli` log gains no line, the wrapper gets no frame, and the directory's mode reads back as found. A check placed after the role file opens would pass it, because opening a role file narrows an existing home to 0700; remove-the-guard control A, the check moved, read red 1 of 1 (as measured at `viola-0.1.0/chunks/2026-10-26-explicit-session-links/evidence/remove-the-guard.md`).»
  add: « As landed for `send`, `wait`, `last`, `answer`, `pause` and `release`, the Security control negatives' strict-modes case (`tests/security_negatives_strict_modes.rs`, the three CI OSes), not `cross_exit_causes.rs` rows: a widened instance is refused first, exit 21 with `link`'s instance-state pair or the `instance-unreachable` document, no frame and no line.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:283
  old: «the `--home` fails strict-modes at `run`»
  new: «the `--home` fails strict-modes at `run`: the home-tree check before the wrapper log (Path 1), exit 1 with the home-state pair or, at instance level, the instance-state pair, no log, no line, no child, a read-granted pre-existing home included on every OS (`a_home_another_user_may_read_refuses_run`); beside it, `viola verify` over a refused home tree (the home, `ledger/`, `bin/`, `plugin/`) exits 1 with the home-state pair, no step line and no summary (`tests/cli_verify.rs` `verify_refuses_a_widened_ledger_before_any_step`)»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:286
  old: «the home or the instance's state could be written by another user (`cfg(unix)`)»
  new: «the home or the instance's state could be written by another user, or the home, the instance directory or its `diagnostics/` read by one (`cfg(unix)`); the start arm also runs `run`'s home-tree check (Path 1) before the wrapper log: exit 1, no log opened, no line, no child, and under `--json` the `strict-modes-failed` document»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:303
  old: «(read back with `GetNamedSecurityInfoW`)»
  new: «(read back through `icacls <home> /save`)»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:303
  old: «the `RUNNER_TEMP` negative case as worded here stays owed to that route entry.»
  new: «the negative case is `tests/security_negatives_strict_modes.rs` `a_home_viola_makes_outside_the_profile_takes_the_protected_dacl` (windows-2025), whose read-back reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`, canonical SDDL spelling the runner's user (local Administrator, RID 500) `LA`, as measured at ci#40400887151.»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:303
  old: «(for example `BUILTIN\Users` read) is refused with exit 1.»
  new: «(for example `BUILTIN\Users` read) is refused with exit 1 (`a_home_another_user_may_read_refuses_run`, on every OS: a Unix 0750 home, a Windows Users read grant).»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:305
  old: «With the wrapper stopped (SIGSTOP on Unix; on Windows the wrapper is suspended with the fake agent idle), run»
  new: «With the wrapper stopped (`Wrapper::stop`, never SIGSTOPped), run»
- line: .andromeda/test-plan-detail/e2e-test-strategy.md:305
  old: «`stamps.json` changes only when `viola verify` runs.»
  new: «`stamps.json` changes only when `viola verify` runs. The case is `tests/security_negatives_strict_modes.rs` `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps`, on the three CI OSes: both files read byte-identical after every other writer-capable entry point.»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:310
  anchor: «    - Its control `security_negative_cli_verb_reaches_the_recorded_wrapper` runs each of the eight verbs, in both modes, against the recorded wrapper itself, which receives one frame each.»
  add: «  - **Strict-modes before the first read or open** (`tests/security_negatives_strict_modes.rs`, the three CI OSes). `frame_verb_refuses_a_widened_instance_before_any_log`: `send`, `wait`, `last`, `answer`, `pause` and `release`, in both modes, each refuse a widened instance with exit 21, the instance-state `unable:` / `hint:` pair or `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty, no frame and no line, the mode or ACL read back as found. The same binary holds the umask-independent modes, single-writer and Windows `--home` outside the profile cases above.»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:310
  anchor: «    - Its control `security_negative_cli_verb_reaches_the_recorded_wrapper` runs each of the eight verbs, in both modes, against the recorded wrapper itself, which receives one frame each.»
  add: «  - **ConPTY companion owner/DACL** (`#[cfg(windows)]` x64, windows-2025). `tests/conpty_sideload.rs` `run_on_a_companion_another_user_may_write_falls_back_to_conhost`: a companion another user may write leaves the child on the inbox ConPTY, its `process-start.sideload_fallback` `strict-modes-failed`, the exit unchanged; the kept homes of the Windows leg hold exactly one `strict-modes-failed` fallback, that test's (as measured at ci#40403389629).»
- after: .andromeda/test-plan-detail/e2e-test-strategy.md:338
  anchor: «  - `.cmd` child (`#[cfg(windows)]`): `run` exits 1 — landed in `tests/cli_program_resolution.rs` (a `.cmd` and a `.bat` child each exit 1 with no child spawned, `process-exit{subject:"self", exit_code:1, detail:"batch-script-child"}`, exactly the two fixed `unable:`/`hint:` stderr lines; the same binary proves `claude.cmd` → sibling `claude.exe` resolution)»
  add: «  - snapshot replay (`tests/chaos_snapshot_replay.rs`): E5 steps 1–3, a start over an unreadable or newer `snapshot.json` that `run`'s collision check reads through `read_snapshot_or_replay` (Scenario E5)»
- line: .andromeda/test-plan-detail/test-data-fixtures.md:10
  old: «A home that seed made is not one viola created, and a first boot and stop does not change that: `hook statusline`'s instance check refuses it (as measured at chunk 2026-10-10-statusline-pass-through, CI run 38080061631, three cases; the refused home's DACL was not read, and that reading is owed to the working-route entry "Home and code-bearing file integrity").»
  new: «`seed_conpty` makes the home and the seed dir `target/conpty-seed/<key>/` through `viola_state::fs::create_private_dir` before `bin/<key>/conpty/`, so a seeded home passes `check_instance` and, on windows-2025, the three `hook_statusline` cases pass. A home made with `create_dir_all` before viola is refused: on Unix (0755) by the read rule; on windows-2025 `Writable`, the refused entry an inherited `S-1-5-32-545` (Users) write ACE (`strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`, as measured at ci#40398601594); that home's canonical SDDL is not recorded.»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:10
  anchor: «| On-disk product state (`events.ndjson`, `snapshot.json`, `heartbeat`, `plugin/`, `settings.json`) | Produced only by running real `viola run` / `hook` / CLI verbs against `viola-fake-agent` in a fresh home, never hand-written. Three exceptions: the chaos tests, which corrupt files the product already wrote; the server-identity impostor cases (below); and on Windows x64 the root tests' `seed_conpty`, which hard-links (or copies) the ConPTY companions into `<home>/bin/<key>/conpty/`, byte-identical to what viola would write (§5 carve-out; `conpty_sideload` never seeds, so it covers the product's write). A home that seed made is not one viola created, and a first boot and stop does not change that: `hook statusline`'s instance check refuses it (as measured at chunk 2026-10-10-statusline-pass-through, CI run 38080061631, three cases; the refused home's DACL was not read, and that reading is owed to the working-route entry "Home and code-bearing file integrity"). A case that needs a home viola created on every OS starts from `stamped_home`, as the every-OS `hook_statusline` cases that run the arm's whole course do. | per-test fresh (TempDir) |»
  add: «| A widened seeded home (Windows x64) | An inheritable `icacls` grant (`(OI)(CI)`) on a seeded home or its `bin/` propagates through `seed_conpty`'s hard links into the shared `target/conpty-seed/<key>/` companion and widens every seeded home until removed (as measured at ci#40398601594 and ci#40400887151, on windows-2025), so a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded (`run_refuses_a_widened_home_tree_before_its_log`). | per-test |»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:13
  anchor: «| The statusline source (`<home>/statusline-source.json`, or the user's `.claude/settings.json` for a default home) | Written by the test, never by viola, which only reads it: `tests/support/home.rs` `plant_statusline_source(home, command)`, with the command from `statusline_echo_command(marker, extra)` and the marker at `statusline_marker(home)`. A default-home case takes `TestHome::default_of_user` and `Wrapper::boot_as_user`, which sets `HOME` (on Windows `USERPROFILE`) on that one child. | per-test |»
  add: «| Widened test homes (the strict-modes negatives) | Set by the test on the real file, never by viola: `tests/support/home.rs` `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted). | per-test |»
- after: .andromeda/test-plan-detail/test-data-fixtures.md:27
  anchor: «- The server-identity impostor cases (Linux and Windows: `served` in `tests/security_negatives_server_identity.rs`, `Impostor::serve` in `tests/hook_fail_open.rs`) write an `InstanceSnapshot` themselves, recording a held live process's pid (the fake agent's holder, below) beside an endpoint the test binds, so that a server that is not the recorded wrapper answers the frame-writing clients.»
  add: «- A home a test makes before viola, where the case needs it to pass the strict-modes read rule, is made through `viola_state::fs::create_private_dir`, never `create_dir_all` (a `create_dir_all` home is refused, On-disk product state above), as `tests/contract_diag_schema.rs`, `tests/cli_machine_contract.rs` and `tests/tui_env_strip.rs` do. A catch-site case holds a viola-made home with a regular file where the checked entry goes (`tests/cli_wait_last.rs`, `diagnostics/`; `tests/cli_verify.rs`, `ledger/`). A home planted under a regular file takes a per-OS oracle: on Unix it is a strict-modes refusal, while on Windows such a path stats `NotFound`, the check skips it as absent and the catch-site line stands (`verify_under_a_regular_file_refuses_the_home`).»
- line: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:26
  old: «Every timed hook also makes its server check»
  new: «Every timed hook also runs its instance strict-modes check (`check_instance`) before any `config.json` read or obs init, and makes its server check»
- line: .andromeda/test-plan-detail/quality-gates-coverage-targets.md:26
  old: «with the check in the timed path the hook rows read `max` 2.2–2.4 ms over 30 runs each on the Linux dev host, as measured at `viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md`»
  new: «with both checks in the timed path the five hook rows read `max` 2.5 / 2.4 / 2.4 / 2.4 / 3.3 ms on the Linux dev host, under `SPINE_DEADLINE`, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md`»
- line: .andromeda/registries/contracts/test-plan/test-data-bootstrap.md:11
  old: «from the per-run `target/conpty-seed/<key>/` copy before the test's `viola run` (the §5 carve-out);»
  new: «from the per-run `target/conpty-seed/<key>/` copy before the test's `viola run` (the §5 carve-out), making the home and the seed dir through `viola_state::fs::create_private_dir` first; the companions are hard links to one shared seed file, so an inheritable grant on a seeded home or its `bin/` reaches the shared companion and every seeded home, and a test that widens a home tree above `bin/<key>/conpty/` starts that home unseeded;»
- after: .andromeda/test-plan-detail/mocking-stubbing-discipline.md:19
  anchor: «| Fake servers | interprocess 2.4.4 server API (squatter listener; impostor listener); `viola-fake-agent` as the fake CLI; windows-sys 0.61.2 `CreateNamedPipeW` (Windows only: test-owned control pipes) | The squatter binds the computed endpoint before `run`; in the listener-admission negatives the test-owned control pipes (accept-remote, reject-remote, the default descriptor) stand beside the real `viola_channel::Server::bind` listener, and `ImpersonateAnonymousToken` stands in for a foreign principal; in the server-verification negatives (Linux and Windows) an impostor, a server on the recorded endpoint while the snapshot records another held live process (Process mocking), stands in for a server that is not the recorded wrapper (root tests bind a test-owned `viola_channel::Server::bind` listener: `tests/security_negatives_server_identity.rs`, `tests/hook_fail_open.rs`'s `Impostor`; the MCP case is `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper`), and the hook unit impostor varies the recorded start time instead (a second early, or an unparseable `started_at`), a root-bin unit test having no portable second process; no HTTP fake servers |»
  add: «| Filesystem faults | none (a real tempfile home, real modes and ACLs) | A widened or read-granted home tree is made on the real file by the §7 widened-test-home helpers and refused by the product's own check, never a stubbed one: on windows-2025 an instance tree granted Users write, or a `diagnostics/` granted Users read, is refused beside a `create_private_dir` tree that passes; the pure verdicts (`windows_read_verdict` among them) take synthetic descriptors at the unit tier; a test that widens a home tree above `bin/<key>/conpty/` starts its home unseeded (§7, a widened seeded home) |»

## Patterns
[[pattern]]
id = "tp4-01"
fixed = 'The replay has two product readers'

[[pattern]]
id = "tp4-02"
fixed = 'collision check still calls `read_snapshot`'

[[pattern]]
id = "tp4-03"
fixed = 'steps 1–3 stay unbuilt'
ignore_case = true

[[pattern]]
id = "tp4-04"
fixed = 'stay on `read_snapshot`'

[[pattern]]
id = "tp4-05"
fixed = 'for the entry that builds it to restate'

[[pattern]]
id = "tp4-06"
fixed = 'Windows synthetic-descriptor stub'

[[pattern]]
id = "tp4-07"
fixed = 'negative case as worded here stays owed'

[[pattern]]
id = "tp4-08"
fixed = 'read back with `GetNamedSecurityInfoW`'

[[pattern]]
id = "tp4-09"
fixed = 'SIGSTOP on Unix; on Windows the wrapper is suspended'

[[pattern]]
id = "tp4-10"
fixed = 'DACL was not read, and that reading is owed'

[[pattern]]
id = "tp4-11"
fixed = 'A home that seed made is not one viola created'

[[pattern]]
id = "tp4-12"
fixed = '2.2–2.4 ms over 30 runs'
