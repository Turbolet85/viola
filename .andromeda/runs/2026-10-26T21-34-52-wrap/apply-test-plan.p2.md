# Apply — test-plan · part 2 of 4

## Verdicts
- fanout-test-plan--cascade-g1.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:36#1
- fanout-test-plan--cascade-g1.md 2 · applied · .andromeda/test-plan-detail/test-scope-summary.md:115+#1
- fanout-test-plan--cascade-g1.md 3 · applied · .andromeda/test-plan-detail/test-scope-summary.md:121#1
- fanout-test-plan--cascade-g1.md 4 · applied · .andromeda/test-plan-detail/test-scope-summary.md:127#1
- fanout-test-plan--cascade-g1.md 5 · applied · .andromeda/test-plan-detail/test-scope-summary.md:159#1
- fanout-test-plan--cascade-g1.md 6 · applied · .andromeda/test-plan-detail/test-scope-summary.md:173#1
- cascade-compare-test-plan.md 5 · applied · .andromeda/test-plan-detail/test-scope-summary.md:69+#1
- fanout-test-plan--cascade-g1.md 7 · applied · .andromeda/test-plan-detail/test-strategy.md:29#1
- cascade-compare-test-plan.md 6 · applied · .andromeda/test-plan-detail/test-strategy.md:63#1
- cascade-compare-test-plan.md 7 · applied · .andromeda/test-plan-detail/test-strategy.md:42#1
- fanout-test-plan--cascade-g1.md 8 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:19#1
- fanout-test-plan--cascade-g1.md 9 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:19+#1
- fanout-test-plan--cascade-g1.md 10 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:22#1
- fanout-test-plan--cascade-g1.md 11 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:23#1
- fanout-test-plan--cascade-g1.md 12 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:24#1
- fanout-test-plan--cascade-g1.md 13 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:25#1
- fanout-test-plan--cascade-g1.md 14 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:25+#1
- fanout-test-plan--cascade-g1.md 15 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:25#2
- fanout-test-plan--cascade-g1.md 16 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:25+#2
- fanout-test-plan--cascade-g1.md 17 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:71+#1

## Edits
- line: .andromeda/test-plan-detail/test-scope-summary.md:36
  old: «**As landed (`link` / `unlink`):** of the eight CLI frame verbs only `link` / `unlink` run `viola_state::strict::check_instance` on the driven instance, before any role file or log opens (their refusal: §5 On-disk)»
  new: «**As landed (the instance check):** all eight CLI frame verbs run `viola_state::strict::check_instance` on the named instance (for `link` / `unlink` the driven one) through the shared `src/cmd/client.rs` `refuse_untrusted`, before any role file or log opens: a refusal exits 21 with the instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, and writes no line (§5 On-disk)»
- after: .andromeda/test-plan-detail/test-scope-summary.md:69
  anchor: «  - **Notes:** traced to arch excerpt, MCP and security excerpt, Vector 5. Schemas are advisory, so negative inputs must be sent that violate the advertised schema.»
  add: «  - **As landed (the instance check):** the frame tools `send`, `wait`, `last` and `answer` run `check_instance` before the liveness pre-check; a refused instance returns `isError:true` `{"error":"instance-unreachable","detail":null}` with no frame and no line (`crates/viola-mcp/tests/handlers.rs` `handlers_frame_tools_refuse_a_widened_instance_before_any_frame`).»
- after: .andromeda/test-plan-detail/test-scope-summary.md:115
  anchor: «    - No developer-seeded state.»
  add: «    - A test that makes a home before viola runs makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`: the instance and home-tree checks refuse a `create_dir_all` home (Unix: its 0755 mode fails the read rule; Windows: its inherited Users write ACE).»
- line: .andromeda/test-plan-detail/test-scope-summary.md:121
  old: «`run` start sequence. Collision check, pinned copy and plugin folder,»
  new: «`run` start sequence. The home-tree strict-modes check before the wrapper log opens (`check_home_tree`: the home under the read rule, the instance check's set, then `ledger/`, `bin/`, `plugin/`, `bin/<key>/` and the pinned copy, `plugin/<key>/` and its `hooks/` and `.claude-plugin/`, each that exists), the collision check (over a replayed snapshot, liveness is the beat alone), pinned copy and plugin folder,»
- line: .andromeda/test-plan-detail/test-scope-summary.md:127
  old: «A tampered pinned exe (hash mismatch) exits 1.»
  new: «A tampered pinned exe (hash mismatch) exits 1. A refused home tree (a 0770 home, a widened `instances/<name>`, `bin/` or pinned copy) exits 1 before any log opens, with the home pair (the instance pair for an instance-level refusal), no line and no child (`tests/run_cli.rs` `run_refuses_a_widened_home_tree_before_its_log`).»
- line: .andromeda/test-plan-detail/test-scope-summary.md:159
  old: «The equality is Unix only; on Windows the arm runs no command and prints nothing.»
  new: «The equality is Unix only; on Windows the arm runs no command and prints nothing. Over a replayed snapshot (unreadable or of an unsupported `v`) the arm finds no `statusline_command`, so it runs and prints nothing (unit `statusline_over_a_replayed_snapshot_runs_nothing`).»
- line: .andromeda/test-plan-detail/test-scope-summary.md:173
  old: «and `viola revive` is its first product reader (chunk 2026-10-10-viola-revive)»
  new: «and its product readers are six — `viola revive`, `viola list`'s `board::read`, the hook's `connect`, the statusline's `serve`, `run`'s collision check and `viola_state::liveness::live_endpoint` — with no product caller of `read_snapshot(` left under `src/` (steps 1–3: `tests/chaos_snapshot_replay.rs`)»
- line: .andromeda/test-plan-detail/test-strategy.md:29
  old: «more `unviable` than `caught` outcomes (`unviable-exceeds-caught`), makes that `run` fail.»
  new: «more `unviable` than `caught` outcomes (`unviable-exceeds-caught`), makes that `run` fail. Beside it stands the per-chunk remove-the-guard rule (`.claude/rules/testing.md`): a control whose neutralised guard lets a product process start needs a bounded witness; the G4 / G10 witnesses spawn through `tests/support/cli.rs` `Running`, bounded by `WITHIN`, since nextest's default profile has no kill line (an unbounded witness hung 27 minutes, as measured at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md`).»
- line: .andromeda/test-plan-detail/test-strategy.md:42
  old: «the one place a test plants the fake agent as `claude` (chunk 2026-10-16-unwrapped-sessions-on-the-board).»
  new: «the one place a test plants the fake agent as `claude` (chunk 2026-10-16-unwrapped-sessions-on-the-board). `home.rs` holds the home-widening helpers `widen` · `grant_read` · `unwiden` · `still_widened` (Unix modes; on Windows `icacls`, its exit asserted), through which a root test widens or read-grants a home path, restores it and reads whether it is still widened.»
- line: .andromeda/test-plan-detail/test-strategy.md:63
  old: «§5 Module ↔ disk; §6 Scenario 1, E5»
  new: «§5 Module ↔ disk; §6 Scenario 1, E5 (root `chaos_snapshot_replay.rs`: steps 1–3) + Security control negatives (root `security_negatives_strict_modes.rs`: each of the six frame verbs `send` · `wait` · `last` · `answer` · `pause` · `release` refusing a widened instance before any log, exit 21 with the instance pair or the `instance-unreachable` document, no frame and no line; a home another user may read refusing `run`; a start under umask 000 making every home path owner-only (Unix); a Windows `--home` outside the profile taking the protected DACL, read back through `icacls /save`; only the wrapper writing `snapshot.json` and only `verify` writing `stamps.json`, each byte-identical after every other writer-capable entry point)»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:19
  old: «The three other snapshot readers (`run`'s collision check, `src/cmd/hook.rs` once, its event and dialog paths sharing the one read, and `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules) still call `read_snapshot`; the CLI frame verbs and the MCP frame tools read the snapshot through `viola_state::liveness::live_endpoint`, whose one `read_snapshot` call feeds the server check.»
  new: «The four other snapshot readers call it too: `run`'s collision check (it takes the caller's one read, and over a replayed snapshot liveness is the beat alone), `src/cmd/hook.rs` `connect`, its event and dialog paths sharing the one read (a replayed or absent snapshot is `channel-unreachable`, fail open), `src/cmd/hook/statusline.rs` `serve` (a replayed snapshot holds no `statusline_command`) and `viola_state::liveness::live_endpoint` (its reading below), through which the CLI frame verbs and the MCP frame tools read the snapshot, its one read feeding the server check; `revive`'s preflight reads once for both its collision check and its recorded-directory reading. No product reader is left on `read_snapshot` (the rule: `read_snapshot(` under `src/`, outside test modules).»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:19
  anchor: «  - Snapshot envelope: the classified read tells the snapshot, no file, an unreadable one and an unsupported `v` apart, reading `v` first; an unsupported `v` or a parse failure goes to replay through the read-or-replay function, which logs one `state-recovered` line for each and none for a present or an absent snapshot (unit cases `snapshot_cause`, `replay_recovers`). `viola revive` is the read-or-replay function's first product caller and `viola list`'s `board::read` its second (chunk 2026-10-11-the-board-viola-list, read in `tests/cli_list.rs` `e5_list_reads_a_live_wrapper_through_the_replay`) (revive's in its second preflight reading, the logged session, of four since chunk 2026-10-16-unwrapped-sessions-on-the-board added `session-live` (the strict-modes check ahead of the preflight since chunk 2026-10-10-budget-governor); the one `state-recovered` line on the replay arm is read in `tests/cli_revive.rs`). The three other snapshot readers (`run`'s collision check, `src/cmd/hook.rs` once, its event and dialog paths sharing the one read, and `src/cmd/hook/statusline.rs`; the rule: `read_snapshot(` under `src/`, outside test modules) still call `read_snapshot`; the CLI frame verbs and the MCP frame tools read the snapshot through `viola_state::liveness::live_endpoint`, whose one `read_snapshot` call feeds the server check.»
  add: «  - The replayed readers' unit cases: `live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line` (`crates/viola-state/src/liveness.rs`), `connect_over_a_replayed_snapshot_is_unreachable` (`src/cmd/hook.rs`), `statusline_over_a_replayed_snapshot_runs_nothing` (`src/cmd/hook/statusline.rs`) and `collision_check_over_a_replayed_snapshot_reads_the_beat_alone` (`src/cmd/run.rs`).»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:22
  old: «`live_endpoint` returns `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot` call.»
  new: «`live_endpoint` returns `LiveEndpoint { endpoint, pid, started_at }`, all from its one `read_snapshot_or_replay` call: only a present snapshot proceeds, and a replayed or absent snapshot or a read error is `None`, since a replayed snapshot carries no `endpoint`, `pid` or `started_at`.»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:23
  old: «Unix strict-modes checks `mode & 0o077`.»
  new: «Strict-modes has two rules, the write rule and the read rule, which holds the content-bearing paths the instance check names below: `unix_read_verdict(mode, uid, euid)` is the write rule plus a refusal of `mode & 0o077` (`unix_read_verdict_needs_the_write_rule_and_no_foreign_bit`), and `windows_read_verdict` is the write rule plus a refusal of any allow ACE granting a right in `READ_RIGHTS` (`0x9000_0001`: `FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to an untrusted SID, inherit-only ACEs included (`windows_read_verdict_refuses_any_read_right_to_another_sid`, `windows_read_verdict_takes_the_trusted_sids_and_keeps_the_write_rule`, and the `cfg(windows)` `read_rights_are_the_named_flags`).»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:24
  old: «These are the stubs for the cases hosted runners cannot provision (test-scope Sec 1).»
  new: «These are the stubs for the cases hosted runners cannot provision (test-scope Sec 1). A Users write grant on an instance tree and a Users read grant on `diagnostics/` are also made for real on `windows-2025` (the instance check, below).»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:25
  old: «reads the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists and in that order»
  new: «reads, each that exists and a folder before what it holds, the home (read rule), `instances/` (write rule), the instance directory (read rule), its `diagnostics/` (read rule), its `snapshot.json` (write rule) and its `events.ndjson` (the write rule on Unix, the read rule on Windows); its call sites (`check_instance(` under `src/`) are three, `src/cmd/hook.rs` `trusted`, `src/cmd/client.rs` `refuse_untrusted` (shared by the eight frame verbs) and `src/cmd/revive.rs`»
- line: .andromeda/test-plan-detail/unit-test-strategy.md:25
  old: «Not covered: a widened DACL on an instance tree on Windows (the crate's Windows cases widen the ledger paths only); that case is owed to the route entry "Home and code-bearing file integrity".»
  new: «On Windows the instance tree is covered by `cfg(windows)` cases green on `windows-2025` (as measured at ci#40403389629): a tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read is refused, and `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry` refuses a home made with `create_dir_all` before viola, its refused entry an inherited Users (`S-1-5-32-545`) write ACE.»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:25
  anchor: «  - The instance check `strict::check_instance(home, instance_dir)` reads the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists and in that order. Unit cases `strict_instance_*`: five widened modes (`cfg(unix)`), a tree viola wrote passes, a home with no instance directory passes, and a path that cannot be statted is unreadable. Three `cfg(unix)` root cases in `tests/cli_revive.rs` hold the refusal: two on `viola revive` (a widened instance tree; and, since chunk 2026-10-10-budget-governor, a group-writable home, refused and left as found) and one on `viola revive --list`; one in `tests/cli_links.rs`, `link_verbs_refuse_a_widened_instance_before_any_log`, holds it on `viola link` / `viola unlink` (§5 On-disk). Not covered: a widened DACL on an instance tree on Windows (the crate's Windows cases widen the ledger paths only); that case is owed to the route entry "Home and code-bearing file integrity".»
  add: «  - The home-tree and diagnostics checks: `check_home_tree_of_a_tree_viola_wrote_passes`, `check_home_tree_refuses_each_widened_path`, `check_diagnostics_of_what_viola_wrote_or_of_nothing_passes`, `check_diagnostics_refuses_a_widened_mode` and `check_diagnostics_refuses_a_symlinked_role_file_and_folder` (a symlink is refused, a folder is held to the read rule, a file to the read rule on Unix and the write rule on Windows, and an absent path passes), and `strict_instance_holds_the_content_folders_to_the_read_rule`.»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:25
  anchor: «  - The instance check `strict::check_instance(home, instance_dir)` reads the home, `instances/`, the instance directory, its `snapshot.json` and its `events.ndjson`, each that exists and in that order. Unit cases `strict_instance_*`: five widened modes (`cfg(unix)`), a tree viola wrote passes, a home with no instance directory passes, and a path that cannot be statted is unreadable. Three `cfg(unix)` root cases in `tests/cli_revive.rs` hold the refusal: two on `viola revive` (a widened instance tree; and, since chunk 2026-10-10-budget-governor, a group-writable home, refused and left as found) and one on `viola revive --list`; one in `tests/cli_links.rs`, `link_verbs_refuse_a_widened_instance_before_any_log`, holds it on `viola link` / `viola unlink` (§5 On-disk). Not covered: a widened DACL on an instance tree on Windows (the crate's Windows cases widen the ledger paths only); that case is owed to the route entry "Home and code-bearing file integrity".»
  add: «  - Stamps and pin: `update_stamps_refuses_a_widened_ledger_and_leaves_it` (`StampsError::StrictModes`, checked before `create_private_dir(ledger/)` and again under the lock, nothing read or written), `stamps_error_messages_are_fixed`, `pin_key_is_the_version_and_the_content_key` and `pin_companions_refuses_a_path_another_user_may_write` (`PinError::StrictModes`, nothing written).»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:71
  anchor: «  - The link verbs: `link_reply_is_read_by_its_verbs_shape` and `link_line_is_built_from_the_names_and_the_reply` (`src/cmd/link.rs`), `link_verb_lines_are_the_design_lines_in_one_write` and `link_since_is_the_utc_time_alone` (`src/human/verbs.rs`); the line's time is the reply's `since` as UTC clock time, and a `since` that is no RFC 3339 instant (the restore's `unknown`) prints through `escape_message`.»
  add: «  - The diagnostics opens (`src/obs.rs`): a `diagnostics/` folder or a role or detail file that fails `strict::check_diagnostics` is a failed open, checked before `ensure_private_dir` narrows anything, so the writer ends at `std::io::sink` and a detail line is dropped (`open_role_file_refuses_a_widened_diagnostics_path_and_leaves_it`, `open_role_file_refuses_a_symlinked_role_file`, `write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it`).»

## Patterns
[[pattern]]
id = "tp2-01"
fixed = "only `link` / `unlink` run"

[[pattern]]
id = "tp2-02"
fixed = "is its first product reader"

[[pattern]]
id = "tp2-03"
fixed = "still call `read_snapshot`"

[[pattern]]
id = "tp2-04"
fixed = "all from its one `read_snapshot` call"

[[pattern]]
id = "tp2-05"
fixed = "Not covered: a widened DACL on an instance tree on Windows"
