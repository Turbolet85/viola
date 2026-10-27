proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 19
    section: '## 4. Unit Test Strategy'
    change: 'Keep the first sentence of the Snapshot envelope item; replace the rest with: Since chunk 2026-10-26-home-and-code-bearing-file-integrity every product snapshot reader goes through the read-or-replay function and `read_snapshot(` has no product caller under `src/` (the rule: `read_snapshot(` under `src/`, outside test modules; 3 → 0). Its six product readers: `viola revive` (one read, handed to both its collision check and its recorded-directory reading; the strict-modes check ahead of the preflight since chunk 2026-10-10-budget-governor; the one `state-recovered` line on the replay arm read in `tests/cli_revive.rs`), `board::read` of `viola list` (chunk 2026-10-11-the-board-viola-list, `tests/cli_list.rs` `e5_list_reads_a_live_wrapper_through_the_replay`), the collision check of `run` (it takes the caller''s one read; a replayed snapshot reads liveness from the beat alone: `collision_check_over_a_replayed_snapshot_reads_the_beat_alone`), the hook `connect` in `src/cmd/hook.rs` (a replayed or absent snapshot is unreachable and fails open: `connect_over_a_replayed_snapshot_is_unreachable`), `serve` in `src/cmd/hook/statusline.rs` (a replayed snapshot holds no `statusline_command`, so nothing runs or prints: `statusline_over_a_replayed_snapshot_runs_nothing`) and `viola_state::liveness::live_endpoint`, which the CLI frame verbs and the MCP frame tools read; E5 steps 1–3 are `tests/chaos_snapshot_replay.rs`.'
    sidecar: 'unit-test-strategy §4 viola-state: the snapshot readers moved onto read_snapshot_or_replay (six product readers, `read_snapshot(` product callers 3 → 0) with their replayed-snapshot unit cases.'
    rationale: 'Report Retired: «`read_snapshot` as a product reader under `src/` (3 → 0)»; Counts 5: replay product readers 2 → 6, unit-test-strategy.md:19 named stale; Symbols (liveness, hook.rs, statusline.rs, run.rs) and Deviation 4 (revive reads once for both readings).'
    basis: .andromeda/test-plan-detail/unit-test-strategy.md:19
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 22
    section: '## 4. Unit Test Strategy'
    change: 'Replace the last sentence with: `live_endpoint` returns `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot_or_replay` call, and only when that read is `Recovered::Snapshot`: a replayed or absent snapshot, or a failed read, gives `None` (a replayed snapshot carries no `endpoint`, `pid` or `started_at`; unit `live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line`).'
    sidecar: 'unit-test-strategy §4 viola-state: live_endpoint reads read_snapshot_or_replay and proceeds only on a snapshot read as the snapshot.'
    rationale: 'Report Symbols: `live_endpoint` reads `replay::read_snapshot_or_replay` (92-94), only `Recovered::Snapshot` proceeds; Retired names `read_snapshot` as a product reader; Counts 5 lists unit-test-strategy.md:22 stale.'
    basis: .andromeda/test-plan-detail/unit-test-strategy.md:22
    dependent-of: D-retired
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 25
    section: '## 4. Unit Test Strategy'
    change: 'Replace the first sentence with: The instance check `strict::check_instance(home, instance_dir)` reads, each that exists and a folder before what it holds, the home (the read rule), `instances/` (the write rule), the instance directory (the read rule), its `diagnostics/` (the read rule), its `snapshot.json` (the write rule) and its `events.ndjson` (the write rule on Unix, the read rule on Windows); and add `strict_instance_holds_the_content_folders_to_the_read_rule` to the `strict_instance_*` unit cases.'
    sidecar: 'unit-test-strategy §4 viola-state: check_instance set widened (diagnostics/ joins; the home, the instance dir and diagnostics/ to the read rule; Windows events.ndjson to the read rule).'
    rationale: 'Report Counts 2: the `check_instance` set widened (the home, the instance dir and `diagnostics/` to the read rule; Windows `events.ndjson` to the read rule); Symbols `viola_state::strict` (check_instance and its unit case at strict.rs 1356-1385).'
    basis: .andromeda/test-plan-detail/unit-test-strategy.md:25
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 25
    section: '## 4. Unit Test Strategy'
    change: 'Replace the sentence «Not covered: a widened DACL on an instance tree on Windows (…); that case is owed to the route entry "Home and code-bearing file integrity".» with: On Windows the `cfg(windows)` cases of `strict.rs` (green on the `windows-2025` leg, ci#40403389629) hold it: an instance tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read is refused, and `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry` refuses a home made with `create_dir_all` before viola as `Writable`, the refused entry an inherited `S-1-5-32-545` (Users) write ACE.'
    sidecar: 'unit-test-strategy §4 viola-state: the Windows widened-instance-tree case built (cfg(windows) strict.rs cases); its owed-to-entry sentence retired.'
    rationale: 'Report Spec claims disproved: «unit-test-strategy.md:25 (Not covered: a widened DACL on an instance tree on Windows) — now covered by the cfg(windows) cases green on windows-2025 (ci#40403389629)»; Counts 1 owner-title stale (the entry that owned it is this chunk); Symbols strict.rs cfg(windows) cases 708-830.'
    basis: .andromeda/test-plan-detail/unit-test-strategy.md:25
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 23
    section: '## 4. Unit Test Strategy'
    change: 'Extend the item: the read rule `unix_read_verdict(mode, uid, euid)` is the write rule plus a refusal of any `mode & 0o077` bit (`Refused::Readable`, a fixed message), pinned by `unix_read_verdict_needs_the_write_rule_and_no_foreign_bit` (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    sidecar: 'unit-test-strategy §4 viola-state: the Unix read rule unix_read_verdict and its unit table.'
    rationale: 'Report Symbols `viola_state::strict`: new `pub fn unix_read_verdict` (209-217), `Refused` gains `Readable`; unit cases `unix_read_verdict_*` (1111-1128); Counts 3 (read-rule, unit-test-strategy.md:23 standing).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 24
    section: '## 4. Unit Test Strategy'
    change: 'Extend the item: the read rule `windows_read_verdict` is the write rule plus a refusal of any allow ACE, inherit-only included, granting a right in `READ_RIGHTS` (`0x9000_0001`: `FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to a SID `trusted_sid` rejects: `windows_read_verdict_refuses_any_read_right_to_another_sid`, `windows_read_verdict_takes_the_trusted_sids_and_keeps_the_write_rule`, and the `cfg(windows)` `read_rights_are_the_named_flags` (chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    sidecar: 'unit-test-strategy §4 viola-state: the Windows read rule windows_read_verdict, READ_RIGHTS and their unit cases.'
    rationale: 'Report Symbols `viola_state::strict`: new `pub fn windows_read_verdict` (279-296) and `pub const READ_RIGHTS: u32 = 0x9000_0001`; unit cases `windows_read_verdict_*` (1134-1189) and cfg(windows) `read_rights_are_the_named_flags`; Counts 3 (read-rule, unit-test-strategy.md:24 standing).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 25
    section: '## 4. Unit Test Strategy'
    change: 'Add after line 25 a viola-state item: The home-tree checks (chunk 2026-10-26-home-and-code-bearing-file-integrity): `check_home_tree(home, name, key)` returns `TreeRefused { HomeLevel, InstanceLevel }` and reads, each that exists, the home (the read rule), with a name the set of `check_instance`, then `ledger/`, `bin/`, `plugin/`, and with a key `bin/<key>/`, the pinned `viola(.exe)`, `plugin/<key>/`, its `hooks/` and `.claude-plugin/` (the write rule): `check_home_tree_of_a_tree_viola_wrote_passes`, `check_home_tree_refuses_each_widened_path`; `check_diagnostics(path)` refuses a symlink (`Refused::Symlink`), holds a folder to the read rule and a file to the read rule on Unix and the write rule on Windows, and passes an absent path: `check_diagnostics_of_what_viola_wrote_or_of_nothing_passes`, `check_diagnostics_refuses_a_widened_mode`, `check_diagnostics_refuses_a_symlinked_role_file_and_folder`; `check_present(path)` is the write rule on one path that exists, an unstattable one refused.'
    sidecar: 'unit-test-strategy §4 viola-state: new item for check_home_tree / TreeRefused, check_diagnostics and check_present with their unit cases.'
    rationale: 'Report Symbols `viola_state::strict`: `TreeRefused` (83-89), `check_home_tree` (91-125), `check_diagnostics` (127-145), `check_present` (160-163), `Refused::Symlink`; unit cases `check_home_tree_*`, `check_diagnostics_*` (1211-1354); Counts 4 (tree-checks: the four new names stand in no master).'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 25
    section: '## 4. Unit Test Strategy'
    change: 'Add after line 25 a viola-state item: The stamps and pin checks (chunk 2026-10-26-home-and-code-bearing-file-integrity): `update_stamps` runs `strict::check_stamps` before `create_private_dir(ledger/)` and again under the lock before the capped read, a refusal `StampsError::StrictModes` that reads and writes nothing (`update_stamps_refuses_a_widened_ledger_and_leaves_it`; `stamps_error_messages_are_fixed` pins the two fixed messages); `pin_key` names the key `<VERSION>-<16 hex>` (`pin_key_is_the_version_and_the_content_key`), and `pin_companions` runs `check_present` on `bin/<key>/conpty/` and each companion that exists before the held open, a refusal `PinError::StrictModes` with nothing written (`pin_companions_refuses_a_path_another_user_may_write`).'
    sidecar: 'unit-test-strategy §4 viola-state: new item for the update_stamps strict-modes refusal (StampsError) and the pin_key / pin_companions checks with their unit cases.'
    rationale: 'Report Symbols `viola_state::stamps` (StampsError 37-44, unit cases 283-325) and `viola_state::pin` (`pin_key` 71-74, `pin_companions` check_present, unit cases 210-213 and 453-473); Counts 17 and 18; Deviation 1.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 69
    section: '## 4. Unit Test Strategy'
    change: 'Add after line 69 a root bin item: The diagnostics opens (`src/obs.rs`, chunk 2026-10-26-home-and-code-bearing-file-integrity): `check_diagnostics(dir, file)` runs `strict::check_diagnostics` on the existing `diagnostics/` folder and the role or detail file before `ensure_private_dir` narrows anything, in `open_role_file` and `write_detail`; a refusal is a failed open (the writer ends at `std::io::sink`, a detail line dropped) and leaves the path as found: `open_role_file_refuses_a_widened_diagnostics_path_and_leaves_it`, `open_role_file_refuses_a_symlinked_role_file`, `write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it`.'
    sidecar: 'unit-test-strategy §4 root bin: new item for the obs.rs diagnostics-open checks and their unit cases.'
    rationale: 'Report Symbols `src/obs.rs`: new private `check_diagnostics(dir, file)` (69-76) before `ensure_private_dir` in `open_role_file` and `write_detail`; Units 610-673; Coverage of new surfaces (diagnostics opens: tests unit obs.rs 610-673).'
note: 'D-tests-framework and D-tests-obs-harness hold (nextest/rstest only; no harness, status or log-format change); the body line test-plan.md:378 («the instance check») and the root-bin broad line test-plan.md:386 are the body side of these amendments and are not this file.'
