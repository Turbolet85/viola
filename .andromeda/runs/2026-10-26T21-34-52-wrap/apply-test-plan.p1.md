# Apply — test-plan · part 1 of 4

## Verdicts
- fanout-test-plan--cascade-g1.md 31 · applied · .andromeda/test-plan.md:135#1, .andromeda/test-plan.md:135#2
- fanout-test-plan.md 1 · applied · .andromeda/test-plan.md:586#1
- fanout-test-plan--cascade-g2.md 5 · folded into fanout-test-plan.md 1
- fanout-test-plan.md 2 · refused · a hand removal · .andromeda/test-plan.md:592–593
- fanout-test-plan--cascade-g2.md 7 · folded into fanout-test-plan.md 2
- fanout-test-plan.md 3 · applied · .andromeda/test-plan.md:590#1
- fanout-test-plan--cascade-g2.md 6 · folded into fanout-test-plan.md 3
- fanout-test-plan.md 4 · applied · .andromeda/test-plan.md:588#1
- fanout-test-plan.md 5 · applied · .andromeda/test-plan.md:606#1
- fanout-test-plan--cascade-g2.md 18 · folded into fanout-test-plan.md 5
- fanout-test-plan.md 6 · applied · .andromeda/test-plan.md:613#1
- fanout-test-plan.md 7 · applied · .andromeda/test-plan.md:604#1
- fanout-test-plan--cascade-g2.md 13 · folded into fanout-test-plan.md 7
- fanout-test-plan.md 8 · applied · .andromeda/test-plan.md:437#1
- fanout-test-plan--cascade-g1.md 32 · folded into fanout-test-plan.md 8
- fanout-test-plan.md 9 · applied · .andromeda/test-plan.md:443+#1
- fanout-test-plan.md 10 · applied · .andromeda/test-plan.md:598#1
- fanout-test-plan--cascade-g2.md 9 · folded into fanout-test-plan.md 10
- fanout-test-plan.md 11 · applied · .andromeda/test-plan.md:411#1
- fanout-test-plan.md 13 · applied · .andromeda/test-plan.md:427#1
- fanout-test-plan.md 14 · applied · .andromeda/test-plan.md:631#1
- fanout-test-plan.md 15 · applied · .andromeda/test-plan.md:378#1
- fanout-test-plan--cascade-g1.md 33 · folded into fanout-test-plan.md 15
- fanout-test-plan.md 16 · applied · .andromeda/test-plan.md:386#1
- fanout-test-plan.md 17 · applied · .andromeda/test-plan.md:209+#1

## Edits
- line: .andromeda/test-plan.md:135
  old: «**Path:** `run` start sequence — the collision check,»
  new: «**Path:** `run` start sequence — the home-tree strict-modes check before the wrapper log opens, then the collision check,»
- line: .andromeda/test-plan.md:135
  old: «a squatted endpoint or a tampered pinned exe exits 1.»
  new: «a squatted endpoint, a tampered pinned exe or a refused home tree exits 1.»
- line: .andromeda/test-plan.md:586
  old: «The rebuilt snapshot has `links`, `wheel:"human"`, `budget_paused:true` and `agent_session_id` recovered from the log, with `dialog_pending:false`.»
  new: «The replay reads `links`, `budget_paused:true` and `agent_session_id` from the log, with `dialog_pending:false`, and writes no file, so no snapshot is rebuilt from it; every start appends `wheel{holder:"driver", cause:"start"}`, so after step 3 the wheel is the driver.»
- line: .andromeda/test-plan.md:590
  old: «**As landed: in part.** The replay's product readers (`viola revive`, `viola list`), its budget and link legs and its `list --json` leg are covered; steps 1–3 stay unbuilt while `run`'s collision check calls `read_snapshot`: every snapshot reader stays on `read_snapshot` (option 1), and their move onto `read_snapshot_or_replay` stays a CARRY owed to the route entry "Home and code-bearing file integrity".»
  new: «**As landed: built.** Steps 1–3 run as `tests/chaos_snapshot_replay.rs` (`a_start_over_an_unreadable_snapshot_replays_the_log`, `a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live`); the replay's product readers are `viola revive`, `viola list`, hook `connect`, statusline `serve`, `run`'s collision check and `liveness::live_endpoint`, each after its strict-modes check, and `read_snapshot` has no product caller under `src/`; the budget, link and `list --json` legs are covered.»
- line: .andromeda/test-plan.md:588
  old: «No refusal of start (exit is not 1).»
  new: «No refusal of start (exit is not 1) once the heartbeat is older than `STALE_AFTER`: over a replayed snapshot the collision check reads liveness from the beat alone, so a fresh beat refuses the start `already-live` (exit 1).»
- line: .andromeda/test-plan.md:606
  old: «umask-independent modes, single writers of `stamps.json` and `snapshot.json`, and, on Linux and Windows,»
  new: «umask-independent modes, a pre-existing home another user may read refusing `viola run` exit 1, single writers of `stamps.json` and `snapshot.json` (byte-identical after every other writer-capable entry point, the wrapper stopped), the six CLI frame verbs refusing a widened instance with exit 21, no frame and no line (`tests/security_negatives_strict_modes.rs`, three OSes), a Windows x64 ConPTY companion another user may write leaving the child on the inbox ConPTY (`tests/conpty_sideload.rs`), and, on Linux and Windows,»
- line: .andromeda/test-plan.md:613
  old: «forward-compat lines counted in `skipped`, and a `.cmd` child refused exit 1.»
  new: «forward-compat lines counted in `skipped`, a start over a truncated or `v:99` snapshot replaying the log and a fresh beat over the replayed snapshot refusing the start `already-live` (`chaos_snapshot_replay.rs`), and a `.cmd` child refused exit 1.»
- line: .andromeda/test-plan.md:604
  old: «and `revive`'s refusals are cases of `tests/cli_revive.rs`;»
  new: «the strict-modes cause of exit 21 for the six frame verbs is a case of `tests/security_negatives_strict_modes.rs` on three OSes, the exit-1 home-tree refusals of `run` and `verify` are cases of `tests/run_cli.rs` and `tests/cli_verify.rs`, and `revive`'s refusals are cases of `tests/cli_revive.rs`;»
- line: .andromeda/test-plan.md:437
  old: «strict-modes refusals give exit 1 for `run` / `mcp` / `viola revive`, hook 0 with no output, and CLI 21 for every other verb.»
  new: «strict-modes refusals give exit 1 for `run` / `mcp` / `viola revive` / `viola verify`, hook 0 with no output, `instance-unreachable` from the MCP frame tools, and CLI 21 for every other verb (the six frame verbs printing the instance pair); a refused check at `run`, `viola revive`, `viola verify`, the frame verbs, the hook and the MCP frame tools writes no line.»
- after: .andromeda/test-plan.md:443
  anchor: «  - no `VIOLA_NAME` means an immediate exit 0»
  add: «  - a `VIOLA_DIR` instance the instance check refuses, or one naming no existing instance directory, means an immediate exit 0 with empty streams, no line, no frame and no SessionEnd append»
- line: .andromeda/test-plan.md:598
  old: «a forced panic, a missing `VIOLA_NAME` and,»
  new: «a forced panic, a missing `VIOLA_NAME`, an instance the instance check refuses or a `VIOLA_DIR` naming no instance directory and,»
- line: .andromeda/test-plan.md:411
  old: «**Windows DACL read-back:** windows-sys 0.61.2.»
  new: «**Windows DACL read-back:** windows-sys 0.61.2; the root tests set and read grants through `icacls` with its exit asserted (`tests/support/home.rs`), and read a protected DACL back through `icacls <home> /save <file>` (canonical SDDL, which spells the RID-500 runner user `LA`).»
- line: .andromeda/test-plan.md:427
  old: «Two carve-outs: Windows x64 starts seed the ConPTY companions before viola runs, and `tests/chaos_feed_panic.rs` alone boots outside the scans.»
  new: «Two carve-outs: Windows x64 starts seed the ConPTY companions before viola runs, into a home `seed_conpty` makes private (a test that widens a home tree above `bin/<key>/conpty/` starts unseeded, because an inheritable grant reaches every seeded home through the shared hard-linked companion, as measured on `windows-2025` at `viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/evidence/operator-pass.md`), and `tests/chaos_feed_panic.rs` alone boots outside the scans.»
- line: .andromeda/test-plan.md:631
  old: «a case that needs a home viola created starts from `stamped_home`.»
  new: «a case that needs a home viola created starts from `stamped_home`, and a test that makes a home itself makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`: a plain home is one the product refuses (on Unix by the read rule; on Windows `Writable`, its refused entry an inherited `S-1-5-32-545` write ACE, as measured on `windows-2025` by `strict_instance_refuses_a_plain_home_made_before_viola_and_names_the_entry`).»
- line: .andromeda/test-plan.md:378
  old: «Unix strict-modes and Windows strict-modes over synthetic SDDL descriptors, and the instance check.»
  new: «Unix strict-modes and Windows strict-modes (the write and the read rule) over synthetic SDDL descriptors and, on `windows-2025`, real grants, the instance check, the home-tree and diagnostics checks, `live_endpoint` over a replayed snapshot, and the stamps and pin strict-modes refusals.»
- line: .andromeda/test-plan.md:386
  old: «human-mode C0/C1 escaping, and the clock-driven dialog hold.»
  new: «human-mode C0/C1 escaping, the clock-driven dialog hold, the readers over a replayed snapshot (the collision check reading the beat alone, hook `connect` unreachable, statusline `serve` running nothing), and the diagnostics opens refusing a widened or symlinked path and leaving it as found.»
- after: .andromeda/test-plan.md:209
  anchor: «    - group/world-writable home or instance dir → refused»
  add: «    - a home, instance directory or `diagnostics/` another user may read → refused (Unix a group or other bit; Windows a read grant to another SID)»
  add: «    - a widened `ledger/`, `bin/`, `plugin/` or pinned copy → `run` exit 1 before any log; a widened `ledger/`, `bin/` or `plugin/` → `verify` exit 1 before any step»
  add: «    - a symlinked `diagnostics/` folder or role file → refused, the open failing»
  add: «    - a ConPTY companion another user may write → the child on the inbox ConPTY with `sideload_fallback` `strict-modes-failed` (Windows x64)»

## Patterns
[[pattern]]
id = "tp1-01"
regex = 'rebuilt snapshot (has|holds)'
ignore_case = true

[[pattern]]
id = "tp1-02"
fixed = 'stay unbuilt while'

[[pattern]]
id = "tp1-03"
regex = 'stays? on `read_snapshot`'

[[pattern]]
id = "tp1-04"
regex = 'exit 1 for `run` ?/ ?`mcp`( /| and for) `viola revive`(?! / `viola verify`)'
