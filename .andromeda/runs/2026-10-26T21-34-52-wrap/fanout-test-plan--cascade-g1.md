proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 36
    section: '## 1. Test Scope Summary'
    change: 'Replace the As-landed line with: «**As landed (strict-modes before any frame):** all eight CLI frame verbs run `viola_state::strict::check_instance` on the named instance (for `link` / `unlink`, the driven one) before any role file or log opens, through the shared `src/cmd/client.rs` `refuse_untrusted` (`send`, `wait`, `last`, `answer`, `pause` and `release` since chunk 2026-10-26-home-and-code-bearing-file-integrity; `link` / `unlink` moved onto it unchanged): a refusal is exit 21 with the fixed instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, with no line written (§5 On-disk); a `driver` equal to `driven` is refused by the wrapper alone, never client-side (CLI exit 20).»'
    sidecar: 'test-scope-summary#3.1: the six CLI frame verbs join link / unlink in running check_instance before any log (refuse_untrusted), retiring «only link / unlink»'
    rationale: 'Report Changes, Symbols `src/cmd/client.rs`: `refuse_untrusted` is called first by send, wait, last, answer, pause and release, and `link_call` moved onto it; Retired gaps (4)–(7); Counts 8 and 20 (exit 21, the instance pair, no line, by the founder''s word inputs#I3 Q2/Q3).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 115
    section: '## 1. Test Scope Summary'
    change: 'Add a Constraints item after «No developer-seeded state.»: «A test that makes its home before viola runs makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`: since chunk 2026-10-26-home-and-code-bearing-file-integrity the read rule refuses a 0755 home at every checked entry point (`tests/contract_diag_schema.rs`, `tests/cli_machine_contract.rs` and `tests/tui_env_strip.rs` moved onto it; the `tests/cli_wait_last.rs` catch-site cases hold a viola-made home).»'
    sidecar: 'test-scope-summary#7.1: test data constraint, a pre-made test home goes through create_private_dir since the read rule refuses a create_dir_all home'
    rationale: 'Report scope record: the three companion tests made their home with `create_dir_all` (0755), which the read rule now refuses, and now use `create_private_dir`; `cli_wait_last` holds a viola-made home because a 0755 home is refused by the frame verbs'' check.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:108
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 121
    section: '## 1. Test Scope Summary'
    change: 'Path line: put the home-tree check first and note the replayed collision check, «`run` start sequence. The home-tree strict-modes check before the wrapper log opens (since chunk 2026-10-26-home-and-code-bearing-file-integrity, `check_home_tree`: the home under the read rule, the instance set, `ledger/`, `bin/`, `plugin/`, `bin/<key>/` and the pinned copy, `plugin/<key>/`, each that exists), then the collision check (over a replayed snapshot it reads the beat alone), pinned copy and plugin folder, …» with the rest of the line unchanged.'
    sidecar: 'test-scope-summary#8.1: run start sequence gains the home-tree strict-modes check before the log and the replayed collision check'
    rationale: 'Report Symbols `src/cmd/run.rs`: `checked_exe` runs `check_home_tree(home, Some(name), Some(&key))`, which `run` calls before `open_wrapper_log`; `collision_check` over `Replayed` takes liveness from the beat alone.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 127
    section: '## 1. Test Scope Summary'
    change: 'Extend the signal: «A duplicate `run` exits 1. A tampered pinned exe (hash mismatch) exits 1. A refused home tree (a 0770 home, a widened `instances/<name>`, `bin/` or pinned copy) exits 1 before any log, with the home pair (the instance pair for an instance-level refusal), no line and no child (`tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log`).»'
    sidecar: 'test-scope-summary#8.1: run start signal gains the widened-home-tree refusal, exit 1 before any log'
    rationale: 'Report Outcome (security): `run` refuses a 0770 home, a widened `instances/<name>`, `bin/` or pinned copy, exit 1 with its pair before any log; Symbols `refuse_tree` (HomeLevel → home pair, InstanceLevel → instance pair).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:131
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 159
    section: '## 1. Test Scope Summary'
    change: 'Append to the statusline signal: «Over a replayed snapshot (unreadable or of an unsupported `v`) the arm finds no `statusline_command`, so it runs and prints nothing (since chunk 2026-10-26-home-and-code-bearing-file-integrity; unit `statusline_over_a_replayed_snapshot_runs_nothing`).»'
    sidecar: 'test-scope-summary#9.2: the statusline over a replayed snapshot runs and prints nothing'
    rationale: 'Report Symbols `src/cmd/hook/statusline.rs` `serve` (128-132): it reads `read_snapshot_or_replay`, and a replayed snapshot has no `statusline_command`, so nothing runs or prints.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:17
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 173
    section: '## 1. Test Scope Summary'
    change: 'Replace «and `viola revive` is its first product reader (chunk 2026-10-10-viola-revive)» with «`viola revive` is its first product reader (chunk 2026-10-10-viola-revive), and since chunk 2026-10-26-home-and-code-bearing-file-integrity it has six — `revive`, `list`''s `board::read`, the hook''s `connect`, the statusline''s `serve`, `run`''s collision check and `viola_state::liveness::live_endpoint` — with no `read_snapshot(` product caller left under `src/` (E5 steps 1–3, `tests/chaos_snapshot_replay.rs`)».'
    sidecar: 'test-scope-summary#10.1: the replay''s product readers 2 → 6, read_snapshot product callers 3 → 0'
    rationale: 'Report Count 5: replay product readers 2 (revive, list) → 6 (+ hook connect, statusline serve, run collision_check, live_endpoint); `read_snapshot(` product callers under src/ 3 → 0; Outcome (tests) E5 steps 1–3 met by `chaos_snapshot_replay`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:32
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 29
    section: '## 2. Test Strategy'
    change: 'Append to the Mutation row''s depth: «Beside it, per chunk, stands the remove-the-guard rule (`.claude/rules/testing.md`, 2026-09-25): a control whose neutralised guard lets a product process start needs a bounded witness; the G4 / G10 witnesses spawn through `tests/support/cli.rs` `Running`, bounded by `WITHIN` (chunk 2026-10-26-home-and-code-bearing-file-integrity, after a measured 27-minute hang under nextest''s default profile, which has no kill line).»'
    sidecar: 'test-strategy#1.7: remove-the-guard witnesses bounded by WITHIN after a measured 27-minute hang'
    rationale: 'Report Decisions & corrections: the remove-the-guard rule applied, measured a 27-minute hang under nextest''s default profile; Deviation 9: the G4 / G10 witnesses spawn through `support::cli::Running`, bounded by `WITHIN`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:123
    fact: F18
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 19
    section: '## 4. Unit Test Strategy'
    change: 'Replace the sentences from «The three other snapshot readers» to the end of the line with: «Since chunk 2026-10-26-home-and-code-bearing-file-integrity the other four snapshot readers call it too: `run`''s collision check (it takes the caller''s one read, and over a replayed snapshot liveness is the beat alone), `src/cmd/hook.rs` `connect` (a replayed or absent snapshot is `channel-unreachable`, fail open), `src/cmd/hook/statusline.rs` `serve` (a replayed snapshot holds no `statusline_command`) and `viola_state::liveness::live_endpoint` (only a present snapshot proceeds); `revive`''s preflight reads once for both its collision check and its recorded-directory reading. No product reader is left on `read_snapshot` (the rule: `read_snapshot(` under `src/`, outside test modules: 3 → 0); the CLI frame verbs and the MCP frame tools read the snapshot through `live_endpoint`.»'
    sidecar: 'unit-test-strategy#1.2: the four remaining snapshot readers moved onto read_snapshot_or_replay; read_snapshot product callers 3 → 0'
    rationale: 'Report Retired: `read_snapshot` as a product reader under `src/` (3 → 0); Count 5; Symbols liveness (92-94), hook `connect` (224-226), statusline `serve` (128-132), run `collision_check` (425-432); Deviation 4 (revive reads once).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:26
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 19
    section: '## 4. Unit Test Strategy'
    change: 'Add after the snapshot-envelope item: «The replayed readers'' unit cases (chunk 2026-10-26-home-and-code-bearing-file-integrity): `live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line` (`crates/viola-state/src/liveness.rs`), `connect_over_a_replayed_snapshot_is_unreachable` (`src/cmd/hook.rs`), `statusline_over_a_replayed_snapshot_runs_nothing` (`src/cmd/hook/statusline.rs`) and `collision_check_over_a_replayed_snapshot_reads_the_beat_alone` (`src/cmd/run.rs`).»'
    sidecar: 'unit-test-strategy#1.2: the four replayed-reader unit cases'
    rationale: 'Report Symbols: units liveness.rs 255-291, hook.rs 640-694, statusline.rs 524-553, run.rs 1173-1224; Coverage of new surfaces: the four readers on the replay → tests unit.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:12
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 22
    section: '## 4. Unit Test Strategy'
    change: 'Replace «`live_endpoint` returns `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot` call.» with «`live_endpoint` returns `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot_or_replay` call: only a present snapshot (`Recovered::Snapshot`) proceeds, and a replayed or absent snapshot or a read error is `None`, since a replayed snapshot carries no `endpoint`, `pid` or `started_at` (chunk 2026-10-26-home-and-code-bearing-file-integrity).»'
    sidecar: 'unit-test-strategy#1.2: live_endpoint reads read_snapshot_or_replay; a replayed snapshot is None'
    rationale: 'Report Symbols `viola_state::liveness::live_endpoint` (92-94): it reads `replay::read_snapshot_or_replay`; only `Recovered::Snapshot` proceeds; `Replayed`, `Absent` or an error → `None`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 23
    section: '## 4. Unit Test Strategy'
    change: 'Replace «Unix strict-modes checks `mode & 0o077`.» with «Strict-modes reads with two rules. The write rule, and the read rule built at chunk 2026-10-26-home-and-code-bearing-file-integrity for the home, the instance directory and `diagnostics/`: `unix_read_verdict(mode, uid, euid)` is the write rule plus a refusal of `mode & 0o077` (`unix_read_verdict_needs_the_write_rule_and_no_foreign_bit`), and `windows_read_verdict` is the write rule plus a refusal of any allow ACE granting a right in `READ_RIGHTS` (`0x9000_0001`: `FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to an untrusted SID, inherit-only ACEs included (`windows_read_verdict_refuses_any_read_right_to_another_sid`, `windows_read_verdict_takes_the_trusted_sids_and_keeps_the_write_rule`, and the `cfg(windows)` `read_rights_are_the_named_flags`).»'
    sidecar: 'unit-test-strategy#1.2: the read rule built, unix_read_verdict and windows_read_verdict unit tables'
    rationale: 'Report Symbols `viola_state::strict`: `unix_read_verdict` (209-217), `windows_read_verdict` (279-296), `READ_RIGHTS`; unit cases 1111-1189 and the `cfg(windows)` `read_rights_are_the_named_flags`; Count 3 (unit-test-strategy.md:23 re-read for not-yet-in-code wording).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:10
    fact: F3
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 24
    section: '## 4. Unit Test Strategy'
    change: 'Append: «Since chunk 2026-10-26-home-and-code-bearing-file-integrity a Users write grant on an instance tree and a Users read grant on `diagnostics/` are also made for real on `windows-2025` (`cfg(windows)` cases, through `icacls`); the synthetic descriptors stay the only cover for what no hosted runner can provision, a foreign-owner SID and a volume without persistent ACLs.»'
    sidecar: 'unit-test-strategy#1.2: Windows write and read grants now provisioned for real on windows-2025 beside the SDDL stubs'
    rationale: 'Report Symbols strict.rs: `cfg(windows)` cases 708-830 (a tree granted Users write beside one that passes, a `diagnostics/` granted Users read), green on the Windows leg (Outcome, security); Count 3 re-read of unit-test-strategy.md:24.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:10
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 25
    section: '## 4. Unit Test Strategy'
    change: 'Replace «reads the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists and in that order» with «reads, each that exists and a folder before what it holds, the home (read rule), `instances/` (write rule), the instance directory (read rule), its `diagnostics/` (read rule), its `snapshot.json` (write rule) and its `events.ndjson` (the write rule on Unix, the read rule on Windows) — the set widened at chunk 2026-10-26-home-and-code-bearing-file-integrity; its three `src/` call sites are `src/cmd/hook.rs` `trusted`, `src/cmd/client.rs` `refuse_untrusted` (shared by the eight frame verbs) and `src/cmd/revive.rs`».'
    sidecar: 'unit-test-strategy#1.2: check_instance''s set widened (read rule, diagnostics/) and its call sites moved'
    rationale: 'Report Symbols `check_instance` (signature unchanged) now reads the home (READ), `instances/` (write), the instance dir (READ), `diagnostics/` (READ, new), `snapshot.json` (write), `events.ndjson` (Unix write, Windows READ); Count 2 (3 → 3 sites, files moved statusline.rs → hook.rs, link.rs → client.rs).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:21
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 25
    section: '## 4. Unit Test Strategy'
    change: 'Add after the instance-check item: «The home-tree and diagnostics checks (chunk 2026-10-26-home-and-code-bearing-file-integrity): `check_home_tree_*` (a tree viola wrote passes; each widened path is refused), `check_diagnostics_*` (what viola wrote or nothing passes; a widened mode and a symlinked role file or folder are refused; a folder is held to the read rule, a file to the read rule on Unix and the write rule on Windows; an absent path passes) and `strict_instance_holds_the_content_folders_to_the_read_rule`.»'
    sidecar: 'unit-test-strategy#1.2: check_home_tree / check_diagnostics unit cases and the read-rule instance case'
    rationale: 'Report Symbols `check_home_tree` (91-125), `check_diagnostics` (127-145); unit cases 1211-1385 by name in New text, by line.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:10
    fact: F4
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 25
    section: '## 4. Unit Test Strategy'
    change: 'Replace «Not covered: a widened DACL on an instance tree on Windows (the crate''s Windows cases widen the ledger paths only); that case is owed to the route entry "Home and code-bearing file integrity".» with «The Windows instance tree is covered since chunk 2026-10-26-home-and-code-bearing-file-integrity by `cfg(windows)` cases green on `windows-2025` (ci#40403389629): a tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read is refused, and `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry` holds.»'
    sidecar: 'unit-test-strategy#1.2: the Windows instance-tree DACL case covered, the owed entry closed'
    rationale: 'Report Spec claims disproved: unit-test-strategy.md:25 «Not covered: a widened DACL on an instance tree on Windows» is now covered by the `cfg(windows)` cases green on windows-2025 (ci#40403389629); Count 1 (owner-title).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:73
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 25
    section: '## 4. Unit Test Strategy'
    change: 'Add a viola-state item: «Stamps and pin (chunk 2026-10-26-home-and-code-bearing-file-integrity): `update_stamps_refuses_a_widened_ledger_and_leaves_it` (`StampsError::StrictModes`, checked before `create_private_dir(ledger/)` and again under the lock, nothing read or written), `stamps_error_messages_are_fixed`, `pin_key_is_the_version_and_the_content_key` and `pin_companions_refuses_a_path_another_user_may_write` (`PinError::StrictModes`, nothing written).»'
    sidecar: 'unit-test-strategy#1.2: stamps and pin strict-modes unit cases'
    rationale: 'Report Symbols `viola_state::stamps` (`StampsError`, check before `create_private_dir`; units 283-325) and `viola_state::pin` (`pin_key`, `PinError::StrictModes`; units 210-213, 453-473); Deviation 1.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:11
    fact: F7
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 71
    section: '## 4. Unit Test Strategy'
    change: 'Add a root-bin item: «The diagnostics opens (`src/obs.rs`, chunk 2026-10-26-home-and-code-bearing-file-integrity): a `diagnostics/` folder or a role or detail file that fails `strict::check_diagnostics` is a failed open, checked before `ensure_private_dir` narrows anything, so the writer ends at `std::io::sink` and a detail line is dropped (`open_role_file_refuses_a_widened_diagnostics_path_and_leaves_it`, `open_role_file_refuses_a_symlinked_role_file`, `write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it`).»'
    sidecar: 'unit-test-strategy#2.1: obs.rs diagnostics-open strict-modes unit cases'
    rationale: 'Report Symbols `src/obs.rs` `check_diagnostics` (69-76) in `open_role_file` and `write_detail`, a refusal an `io::Error` ending at `std::io::sink`; units 610-673 named in New text, by line.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:20
    fact: F8
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 22
    section: '## 5. Integration Test Strategy'
    change: 'Replace «the root integration binary `conpty_sideload` (5 cases: the default sideload, planted copies in the CWD and on `PATH`, tampered pinned copies falling back to `System32\conhost.exe`)» with «the root integration binary `conpty_sideload` (the default sideload, planted copies in the CWD and on `PATH`, tampered pinned copies falling back to `System32\conhost.exe` and, since chunk 2026-10-26-home-and-code-bearing-file-integrity, `run_on_a_companion_another_user_may_write_falls_back_to_conhost`: a companion its owner/DACL check refuses leaves the child on the inbox ConPTY with `sideload_fallback` `strict-modes-failed`, the exit unchanged)».'
    sidecar: 'integration-test-strategy#1.5: conpty_sideload gains the companion strict-modes fallback case; the bare case count dropped'
    rationale: 'Report New text, by line: `tests/conpty_sideload.rs` 295-326 adds `run_on_a_companion_another_user_may_write_falls_back_to_conhost`; Outcome (security) met on windows-2025; Count 18 and Count 7 (`strict-modes-failed`). The «5 cases» literal no longer counts the binary, and the report states no new count.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:133
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 25
    section: '## 5. Integration Test Strategy'
    change: 'Append to the Test approach: «Since chunk 2026-10-26-home-and-code-bearing-file-integrity the event and dialog paths run the instance check before any `config.json` read or obs init, and a `VIOLA_DIR` naming no existing instance directory canonicalises to no session and writes nothing (a `no_instance_dir` row); so in `tests/hook_fail_open.rs` the cases that expect lines run in a viola-made instance dir (`Env::Made`), and the widened-instance rows are one function over the events (`hook_refused_by_the_instance_check_fails_open_and_writes_nothing`), not an rstest matrix row each.»'
    sidecar: 'integration-test-strategy#1.6: hook cases expecting lines run in a viola-made instance dir; widened-instance rows one function'
    rationale: 'Report Deviation 5 (Env::Made, the `no_instance_dir` row, the widened rows one function); Symbols `src/cmd/hook.rs` `canonical` / `trusted` before obs init; New text `tests/hook_fail_open.rs` 477-519.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:98
    fact: F17
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 32
    section: '## 5. Integration Test Strategy'
    change: 'Replace «so for them the test creates the home before viola runs.» with «so for them the test creates the home before viola runs, through `viola_state::fs::create_private_dir`, and the seed dir `target/conpty-seed/<key>/` the same way (chunk 2026-10-26-home-and-code-bearing-file-integrity), so a seeded home passes the strict-modes check.»'
    sidecar: 'integration-test-strategy#2.0: seed_conpty makes the home and the seed dir with create_private_dir'
    rationale: 'Report Harness / gate surface: `seed_conpty` makes the home and the seed dir with `create_private_dir`; Count 15 (pattern `seed`, the carve-out); Outcome (tests): `seed_conpty` homes pass `check_instance` on windows-2025.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:55
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 33
    section: '## 5. Integration Test Strategy'
    change: 'Replace «(as measured at chunk 2026-10-10-statusline-pass-through, CI run 38080061631, three cases refused; the home''s DACL not read)» with «(as measured at chunk 2026-10-10-statusline-pass-through, CI run 38080061631, three cases refused; read at chunk 2026-10-26-home-and-code-bearing-file-integrity on `windows-2025`, ci#40398601594: a home made with `create_dir_all` before viola is refused `Writable`, the refused entry an inherited `S-1-5-32-545` (Users) write ACE, by `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`; the plain home''s canonical SDDL was not recorded)».'
    sidecar: 'integration-test-strategy#2.0: «the home''s DACL not read» disproved by the windows-2025 reading (inherited Users write ACE)'
    rationale: 'Report Spec claims disproved: integration-test-strategy.md:33 «the refused home''s DACL was not read», measured on windows-2025 (ci#40398601594): refused `Writable`, the entry an inherited `S-1-5-32-545` write ACE; Deviation 6: the SDDL itself not recorded.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:72
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 33
    section: '## 5. Integration Test Strategy'
    change: 'Replace «When viola sets its own home DACL at creation (the Epoch 6 entry "Home and code-bearing file integrity"), a seeded home skips it, so that entry owns making the seed let viola create the home first (route CARRY).» with «Since chunk 2026-10-26-home-and-code-bearing-file-integrity the seed makes the home with viola''s own `create_private_dir` (owner-only modes and DACL), so a seeded home no longer skips viola''s home protection; the CARRY is closed.»'
    sidecar: 'integration-test-strategy#2.0: the seed carve-out''s CARRY to «Home and code-bearing file integrity» closed'
    rationale: 'Report Count 15: integration-test-strategy.md:33 «a seeded home skips it, so that entry owns making the seed let viola create the home first» stale, since viola makes the seeded home (`create_private_dir`) before `bin/<key>/conpty/`; Count 1 (owner-title rows stale).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:42
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 48
    section: '## 5. Integration Test Strategy'
    change: 'Add after the MCP impostor case: «the MCP widened-instance case `handlers_frame_tools_refuse_a_widened_instance_before_any_frame` (`crates/viola-mcp/tests/handlers.rs`, chunk 2026-10-26-home-and-code-bearing-file-integrity) drives the MCP tools over stdio with an `initialize` handshake: each frame tool on a widened instance gets `instance-unreachable` with `detail: null`, and no frame and no line are written.»'
    sidecar: 'integration-test-strategy#3.1: the MCP frame tools'' widened-instance refusal case'
    rationale: 'Report Symbols `viola-mcp` `tools.rs` `frame` (337-339): `check_instance` before `live_endpoint`, refusal `instance-unreachable`, `detail: null`, no frame, no line; Deviation 7(b) (stdio, `initialize` handshake); New text handlers.rs 436-478.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14
    fact: F16
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 58
    section: '## 5. Integration Test Strategy'
    change: 'Rewrite the mapping: «strict-modes refusals, each before any log opens and with no line written: exit 1 for `run` / `mcp`, for `viola revive` (… unchanged …) and, since chunk 2026-10-26-home-and-code-bearing-file-integrity, for `run`''s and `revive`''s start home-tree check (the home pair, or the instance pair for an instance-level refusal; no child) and `viola verify`''s home-tree check (the home pair, no step line, `stamps.json` unchanged); hook 0 with no output on the statusline arm and on the event and dialog paths (a refused or absent instance: no `config.json` read, no role file, no stdin read, no frame, no SessionEnd append); MCP frame tools `instance-unreachable` with `detail: null`; CLI 21 for every other verb (the eight frame verbs: the instance pair, or the `instance-unreachable` document under `--json`)».'
    sidecar: 'integration-test-strategy#3.2: strict-modes exit mapping gains verify exit 1, run/revive tree check, the hook event/dialog paths and the MCP frame tools, all with no line'
    rationale: 'Report Symbols: verify.rs 64-70 (`check_home_tree`, exit 1 home pair), run.rs / revive.rs `checked_exe` / `refuse_tree` (exit 1 before `open_wrapper_log`), hook.rs `canonical` / `trusted`, tools.rs 337-339, client.rs `refuse_untrusted` (exit 21); Count 20 (each new no-line site, integration-test-strategy.md:58 re-read); the line''s «CLI 21 for every other verb» would wrongly cover `verify`.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:19
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 59
    section: '## 5. Integration Test Strategy'
    change: 'Append: «Since chunk 2026-10-26-home-and-code-bearing-file-integrity `send`, `wait`, `last`, `answer`, `pause` and `release` run the same check through the same `src/cmd/client.rs` `refuse_untrusted` that `link_call` now shares, with the same refusal (exit 21, the instance pair or the `instance-unreachable` document, no line, the mode or ACL as found).»'
    sidecar: 'integration-test-strategy#3.2: the six frame verbs share link''s instance check and refusal'
    rationale: 'Report Symbols `src/cmd/client.rs` `refuse_untrusted` (20-41), called first by the six verbs, `link_call` moved onto it unchanged; Count 8 (the six frame verbs print the instance pair, inputs#I3 Q3).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 59
    section: '## 5. Integration Test Strategy'
    change: 'Add an On-disk item: «the start and verify tree checks (chunk 2026-10-26-home-and-code-bearing-file-integrity): `viola run` refuses a 0770 home, a widened `instances/<name>`, a widened `bin/` or a widened pinned copy with exit 1 and its pair before any log, with no child (`tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log`); `viola revive`''s start does the same, its `--json` keeping `strict-modes-failed`; `viola verify` refuses a widened `ledger/` with exit 1, the home pair, no step line and `stamps.json` unchanged (`tests/cli_verify.rs` `verify_refuses_a_widened_ledger_before_any_step`).»'
    sidecar: 'integration-test-strategy#3.2: run / revive / verify home-tree refusal cases'
    rationale: 'Report Outcome (security): `run` refuses a 0770 home, widened `instances/<name>`, `bin/` or pinned copy, exit 1 before any log; `verify` refuses a widened `ledger/`; Symbols run.rs / revive.rs (revive `--json` keeps `strict-modes-failed`); New text run_cli.rs 619-674, cli_verify.rs 251-280.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:131
    fact: F9
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 59
    section: '## 5. Integration Test Strategy'
    change: 'Add an On-disk item: «a pre-existing home another user may read refuses `viola run` on every OS (Unix 0750, Windows a Users read grant; `tests/security_negatives_strict_modes.rs` `a_home_another_user_may_read_refuses_run`, chunk 2026-10-26-home-and-code-bearing-file-integrity).»'
    sidecar: 'integration-test-strategy#3.2: a read-granted pre-existing home refuses run on every OS'
    rationale: 'Report Deviation 7(d): the read-granted pre-existing home refusing `viola run` runs on every OS (Unix 0750, Windows a Users read grant); New text security_negatives_strict_modes.rs 380-404.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:100
    fact: F10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 59
    section: '## 5. Integration Test Strategy'
    change: 'Add an On-disk item: «the Windows `--home` outside the profile (`a_home_viola_makes_outside_the_profile_takes_the_protected_dacl`, chunk 2026-10-26-home-and-code-bearing-file-integrity): on `windows-2025` `Get-Acl` printed nothing for a protected DACL, so the DACL is read back with `icacls <home> /save` and reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`; canonical SDDL spells the runner''s user (RID 500) `LA`, so the oracle takes `LA` for a user SID ending `-500`.»'
    sidecar: 'integration-test-strategy#3.2: the protected DACL read back through icacls /save, LA for a -500 user SID'
    rationale: 'Report Decisions & corrections (measured on windows-2025: `Get-Acl` printed nothing, `icacls /save` read it back; canonical SDDL spells RID 500 `LA`); Cross-project claims (the DACL reading); Deviation 10; New text security_negatives_strict_modes.rs 406-475.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:121
    fact: F12
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 71
    section: '## 5. Integration Test Strategy'
    change: 'Add after the `cli_verify` paragraph: «`tests/cli_verify.rs` `verify_under_a_regular_file_refuses_the_home` (chunk 2026-10-26-home-and-code-bearing-file-integrity) has a per-OS oracle: on Unix it prints the home pair; on Windows a path under a regular file stats `NotFound`, so the catch-site line stands; a separate case forces the catch-site line with a regular file where `ledger/` goes in a viola-made home.»'
    sidecar: 'integration-test-strategy#4.1: verify''s regular-file home case and its per-OS oracle'
    rationale: 'Report Deviation 8 (the catch-site case split in two, the per-OS oracle after CI) and Count 13 (an unstattable home a strict-modes refusal on Unix; on Windows it stats `NotFound`).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:101
    fact: F11
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/integration-test-strategy.md
    near: 81
    section: '## 5. Integration Test Strategy'
    change: 'Append to the table''s history: «chunk 2026-10-26-home-and-code-bearing-file-integrity added the row `setting_does_not_disable_the_home_check`, which runs each `SETTINGS` row against the home check.»'
    sidecar: 'integration-test-strategy#4.1: cli_controls_not_disableable gains setting_does_not_disable_the_home_check'
    rationale: 'Report New text, by line: `tests/cli_controls_not_disableable.rs` 576-641 adds `setting_does_not_disable_the_home_check`, looping `for (setting, _) in SETTINGS`; Coverage of new surfaces lists `cli_controls_not_disableable` for the verify / run / revive tree check.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:88
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 135
    section: '## 1. Test Scope Summary'
    change: 'Broad line: «`run` start sequence — the home-tree strict-modes check before the wrapper log opens, then the collision check (over a replayed snapshot, the beat alone), pinned copy and plugin folder, … all precede the child spawn; a second `run` of the same name, a squatted endpoint, a tampered pinned exe or a refused home tree exits 1.»'
    sidecar: 'test-plan §1 Path 1 broad line: the home-tree check before the log and its exit 1'
    rationale: 'Report Symbols `src/cmd/run.rs` (`checked_exe` / `refuse_tree` before `open_wrapper_log`, exit 1; `collision_check` over `Replayed`); the broad line of test-scope-summary#8.1.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 437
    section: '## 5. Integration Test Strategy'
    change: 'Broad line: «… and strict-modes refusals, each before any log and with no line, give exit 1 for `run` / `mcp` / `viola revive` / `viola verify`, hook 0 with no output, MCP frame tools `instance-unreachable`, and CLI 21 for every other verb.»'
    sidecar: 'test-plan §5 On-disk broad line: verify exit 1, MCP frame tools, no line'
    rationale: 'Report Symbols verify.rs 64-70 (exit 1, home pair), tools.rs 337-339, client.rs `refuse_untrusted`; Count 20 (no-line sites); the broad line of integration-test-strategy#3.2.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:19
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 378
    section: '## 4. Unit Test Strategy'
    change: 'Broad line: «… Unix strict-modes and Windows strict-modes (the write and read rules) over synthetic SDDL descriptors and real grants on `windows-2025`, the instance check, the home-tree and diagnostics checks, and the stamps and pin strict-modes refusals.»'
    sidecar: 'test-plan §4 viola-state broad line: read rule, tree checks, stamps and pin refusals'
    rationale: 'Report Symbols `viola_state::strict` (read verdicts, `check_home_tree`, `check_diagnostics`), `stamps` (`StampsError`), `pin` (`PinError::StrictModes`); the broad line of unit-test-strategy#1.2.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:10
note: F1, F2 and F15 (e2e-test-strategy) and F13 and F14 (test-data-fixtures) are not in this group's categories; F14's hard-link hazard is also owed beside the §5 seed carve-out (integration-test-strategy.md:32–33), and the body's §1 filesystem trigger (test-plan.md:209, «group/world-writable … refused») and §5 Hooks line (test-plan.md:443) belong to entities not picked here.
