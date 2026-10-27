proposals:
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 236
    section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
    change: In line 236 replace «As landed (chunks 2026-10-10-self-healing-state and 2026-10-11-the-board-viola-list): landed in part.** The replay has two product readers, `viola revive` (chunk 2026-10-10-viola-revive) and `viola list`'s `board::read` (chunk 2026-10-11-the-board-viola-list), and stays covered» with «As landed (chunks 2026-10-10-self-healing-state, 2026-10-11-the-board-viola-list and 2026-10-26-home-and-code-bearing-file-integrity): built, two of its signals restated below.** The replay has six product readers, `viola revive` (chunk 2026-10-10-viola-revive), `viola list`'s `board::read` (chunk 2026-10-11-the-board-viola-list) and, since chunk 2026-10-26-home-and-code-bearing-file-integrity, the hook's `connect`, the statusline arm's `serve`, `run`'s collision check and `viola_state::liveness::live_endpoint`, with no product `read_snapshot(` caller left under `src/`, and stays covered»; replace «`run`'s collision check still calls `read_snapshot`, so step 3's `viola run builder` does not take the replay.» with «Steps 1–3 landed at chunk 2026-10-26-home-and-code-bearing-file-integrity in `tests/chaos_snapshot_replay.rs` (`binary(chaos_snapshot_replay)`): `a_start_over_an_unreadable_snapshot_replays_the_log` and `a_start_over_a_replayed_snapshot_and_a_fresh_beat_refuses_already_live`; `run`'s collision check takes its caller's one `read_snapshot_or_replay` read, and over a replayed snapshot reads liveness from the beat alone (a beat no older than `STALE_AFTER` is `already-live`, else the name is free).»; and drop «; steps 1–3 stay unbuilt for the `read_snapshot` reason below» from the link-leg parenthesis.
    sidecar: e2e-test-strategy E5 as-landed: the replay's product readers 2 → 6, `read_snapshot(` product callers 3 → 0, steps 1–3 built in `tests/chaos_snapshot_replay.rs` (chunk 2026-10-26-home-and-code-bearing-file-integrity).
    rationale: Report Counts 5 (pattern `read-snapshot`, e2e-test-strategy.md:236 listed stale) and Symbols (`live_endpoint`, hook `connect`, statusline `serve`, `run`'s `collision_check` on `read_snapshot_or_replay`); Outcome «(tests) E5 steps 1–3 and the replayed readers — MET (`binary(chaos_snapshot_replay)` + unit tier)».
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:32
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 237
    section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
    change: In line 237 replace «Steps 1–3 stay unbuilt while `run`'s collision check calls `read_snapshot`; the three other snapshot readers (`read_snapshot(` under the root `src/` outside test modules: `run`'s collision check, `src/cmd/hook.rs` once, `src/cmd/hook/statusline.rs`) stay on `read_snapshot`, and their move onto the replay (`read_snapshot_or_replay`), with what a client does with an unreadable or newer snapshot, is one CARRY on the route entry "Home and code-bearing file integrity".» with «Since chunk 2026-10-26-home-and-code-bearing-file-integrity no product snapshot reader stays on `read_snapshot`, and what a reader does with a replayed or absent snapshot is held at the unit tier: the hook's `connect` is unreachable and fails open (`connect_over_a_replayed_snapshot_is_unreachable`), the statusline arm runs and prints nothing (`statusline_over_a_replayed_snapshot_runs_nothing`), `live_endpoint` reads no endpoint, so a frame verb exits 21 `instance-unreachable` (`live_endpoint_of_a_replayed_snapshot_is_none_with_its_one_line`), and `run`'s collision check reads the beat alone (`collision_check_over_a_replayed_snapshot_reads_the_beat_alone`).»
    sidecar: e2e-test-strategy E5: the CARRY moving the three `read_snapshot` readers onto the replay closed; each reader's replayed-snapshot behaviour named with its unit case.
    rationale: Report Counts 5 (`read-snapshot`, e2e-test-strategy.md:237 listed stale: «names the 3 callers») and 1 (`owner-title`: rows naming this entry as owner of a CARRY it closed); Symbols liveness/hook/statusline/run units.
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:32
    dependent-of: D-tests-count-literals
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 237
    section: ## 6. E2E Test Strategy › #### Scenario: E5 — snapshot corruption → replay recovery
    change: In line 237 replace «Two of its signals are set against what landed, for the entry that builds it to restate: every start appends `wheel{holder:"driver", cause:"start"}` (as measured at `tests/chaos_torn_append.rs`, the second start's first record), so `wheel:"human"` does not hold after step 3; and the replay writes no file, so no snapshot is rebuilt from it.» with «Two of its signals stand restated by the entry that built it (chunk 2026-10-26-home-and-code-bearing-file-integrity): every start appends `wheel{holder:"driver", cause:"start"}` (as measured at `tests/chaos_torn_append.rs`, the second start's first record), so `wheel:"human"` is no signal after step 3; and the replay writes no file, so no snapshot is rebuilt from it.»
    sidecar: e2e-test-strategy E5: the two signals set aside for the building entry are restated as standing, the entry built.
    rationale: Report Count 19 (`wheel-human`: e2e-test-strategy.md:237 listed stale; «E5's two signals restated (`wheel:"human"` does not hold after step 3 … the replay writes no file)»).
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:46
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 305
    section: ## 6. E2E Test Strategy
    change: In line 305 replace «With the wrapper stopped (SIGSTOP on Unix; on Windows the wrapper is suspended with the fake agent idle), run every other writer-capable entry point» with «With the wrapper stopped (`Wrapper::stop` on every OS, never SIGSTOPped), run every other writer-capable entry point», and append «As landed (chunk 2026-10-26-home-and-code-bearing-file-integrity): `tests/security_negatives_strict_modes.rs` `only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps` (from `stamped_home`), both files byte-identical afterwards, green on the three CI legs (ci#40403389629).»
    sidecar: e2e-test-strategy Security control negatives: single writers witnessed with the wrapper stopped, not SIGSTOPped; landed case named.
    rationale: Report Count 16 (`single-writer`: «e2e-test-strategy.md:305 ("per stopped (SIGSTOP on Unix; on Windows t…" stale)») and Outcome «single writers (byte-identical after every other writer-capable entry point, the wrapper stopped) … MET … three legs».
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:43
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 303
    section: ## 6. E2E Test Strategy
    change: In line 303 replace «the `RUNNER_TEMP` negative case as worded here stays owed to that route entry.» with «the negative case landed at chunk 2026-10-26-home-and-code-bearing-file-integrity: `tests/security_negatives_strict_modes.rs` `a_home_viola_makes_outside_the_profile_takes_the_protected_dacl` (Windows) reads the home's DACL back as `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)` (canonical SDDL spells the runner's user, local Administrator RID 500, `LA`, so the oracle takes `LA` for a user SID ending `-500`), and `a_home_another_user_may_read_refuses_run` refuses `viola run` exit 1 over a pre-existing home another user may read, on every OS (Unix 0750, Windows a Users read grant).»
    sidecar: e2e-test-strategy Security control negatives: the Windows `--home` outside the profile case and the pre-existing read-granted home refusal landed; the owed-to-entry clause retired.
    rationale: Report Count 1 (`owner-title`: rows naming this entry as owner of a not-yet-built case it closed), Outcome «the Windows `--home` outside the profile (protected DACL read back; a pre-existing Users-read dir refused exit 1) — MET … the DACL read `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`», Deviations 7 (d) and 10.
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:134
  - detector: D-tests-framework
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 303
    section: ## 6. E2E Test Strategy
    change: In line 303 replace «(read back with `GetNamedSecurityInfoW`)» with «(read back through `icacls <home> /save <file>`: `Get-Acl` printed nothing for the protected DACL, as measured on `windows-2025`)».
    sidecar: e2e-test-strategy Security control negatives: the protected DACL's read-back instrument is `icacls /save`, as built and measured.
    rationale: Report Decisions «Measured on `windows-2025`: `Get-Acl` printed nothing for the protected DACL (stderr not captured); `icacls <home> /save <file>` read it back» and Deviation 7 «the protected-DACL read-back Windows-only (finally through `icacls /save`)».
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:121
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 304
    section: ## 6. E2E Test Strategy
    change: Append to line 304 «As landed (chunk 2026-10-26-home-and-code-bearing-file-integrity): `tests/security_negatives_strict_modes.rs` `a_start_under_umask_000_makes_every_home_path_owner_only` (Unix).»
    sidecar: e2e-test-strategy Security control negatives: the umask-independent modes case landed.
    rationale: Report Coverage «`security_negatives_strict_modes` (umask 000, …)» and Outcome «umask-independent modes (Unix) … MET».
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:134
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 271
    section: ## 6. E2E Test Strategy
    change: After line 271 add « As landed for the six frame verbs (chunk 2026-10-26-home-and-code-bearing-file-integrity), a `tests/security_negatives_strict_modes.rs` case, not a `cross_exit_causes.rs` row: `frame_verb_refuses_a_widened_instance_before_any_log` runs `send`, `wait`, `last`, `answer`, `pause` and `release`, each in both modes, against a widened instance on the three CI legs; each exits 21 with `link`'s fixed instance-state `unable:` / `hint:` pair, or exactly `{"v":1,"error":"instance-unreachable","detail":null}` with stderr empty, before anything under the home opens: no `cli` line, no frame, the mode or ACL as found, and no pid, start time or path in any output (`digit_runs`). The check now holds the home, the instance directory and its `diagnostics/` to the read rule too (Unix no group or other bit; Windows no read grant to another SID), and on Unix a home under a regular file is this refusal.»
    sidecar: e2e-test-strategy Exit-cause matrix: the six frame verbs' strict-modes exit 21 case landed, with the read rule's reach.
    rationale: Report Symbols (`client::refuse_untrusted` called first by the six verbs; `check_instance`'s set widened), Counts 2, 8, 13, and Outcome «six frame verbs refuse a widened instance, exit 21, the instance pair / the `instance-unreachable` document, no frame, no line, mode or ACL as found, three OSes — MET».
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:128
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 283
    section: ## 6. E2E Test Strategy
    change: After line 283 add « As landed (chunk 2026-10-26-home-and-code-bearing-file-integrity), a `tests/run_cli.rs` case, not a `cross_exit_causes.rs` row: `run_refuses_a_widened_home_tree_before_its_log` widens the home, `instances/<name>`, `bin/` or the pinned copy and reads exit 1 before any log, no line and no child, with the home pair for a home-level refusal and the instance pair for an instance-level one; its home starts unseeded, because an inheritable grant on a seeded home reaches the shared seed companion through `seed_conpty`'s hard links.»
    sidecar: e2e-test-strategy Exit-cause matrix: `run`'s strict-modes exit 1 over the home tree landed in `tests/run_cli.rs`.
    rationale: Report Symbols (`checked_exe` / `refuse_tree` before `open_wrapper_log`; `HomeLevel` → home pair, `InstanceLevel` → instance pair), Deviation 10 (the unseeded home) and Outcome «`run` refuses a 0770 home, widened `instances/<name>`, `bin/` or pinned copy, exit 1 with its pair before any log … MET».
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:131
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 286
    section: ## 6. E2E Test Strategy
    change: In line 286 replace «the home or the instance's state could be written by another user (`cfg(unix)`)» with «the home or the instance's state could be written by another user, or, since chunk 2026-10-26-home-and-code-bearing-file-integrity, the home, the instance directory or its `diagnostics/` read by one (`cfg(unix)`; no `tests/cli_revive.rs` case holds the read leg); the start arm then checks the home tree (`ledger/`, `bin/`, `plugin/`, the pinned copy) before the wrapper log, `--json` keeping `strict-modes-failed`»
    sidecar: e2e-test-strategy Exit-cause matrix: `revive`'s strict-modes cause names the read rule and the start arm's tree check.
    rationale: Report Count 2 (`check-instance` set widened: home, instance dir, `diagnostics/` to the read rule; claims stating what the check reads re-read) and Symbols (`revive`'s start, revive.rs:125-135, calls `checked_exe` / `refuse_tree` before `open_wrapper_log`, «revive `--json` keeps `strict-modes-failed`»); the scope record shows `tests/cli_revive.rs` only ageing the beat.
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:29
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 246
    section: ## 6. E2E Test Strategy
    change: After line 246 add «  As landed (chunk 2026-10-26-home-and-code-bearing-file-integrity), the matrix holds the instance check: `hook_refused_by_the_instance_check_fails_open_and_writes_nothing` (`tests/hook_fail_open.rs`, one function over the events) runs the event and dialog paths against a widened instance, each exit 0 with empty streams under 1.0 s, no frame, no line and no SessionEnd append; a `no_instance_dir` row pins that a `VIOLA_DIR` naming no existing instance directory writes nothing, and the cases that expect lines run in a viola-made instance directory (`Env::Made`); `hook_statusline_a_widened_instance_runs_and_logs_nothing` (`tests/hook_statusline.rs`) holds the statusline arm, both OS forms.»
    sidecar: e2e-test-strategy Security sweep: the hook fail-open matrix gains the instance-check refusal and the absent-instance-dir row.
    rationale: Report Symbols (hook.rs `canonical` / `trusted` before obs init; a `VIOLA_DIR` naming no directory writes nothing), Deviation 5, Count 23, Coverage «`hook_fail_open`, `hook_statusline` (both OS forms)» and Outcome «`viola hook` event/dialog refused by the instance check … MET».
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:129
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/e2e-test-strategy.md
    near: 244
    section: ## 6. E2E Test Strategy
    change: In line 244, after the MCP schema-violation sentence's «— no `mcp_schema_violations.rs` binary (chunk 2026-10-17-mcp-server-for-drivers) —» add «and the frame tools' instance check, `crates/viola-mcp/tests/handlers.rs` `handlers_frame_tools_refuse_a_widened_instance_before_any_frame` (chunk 2026-10-26-home-and-code-bearing-file-integrity: `instance-unreachable` with `detail: null`, no frame, no line) —»
    sidecar: e2e-test-strategy Security sweep: the MCP frame tools' strict-modes refusal case named.
    rationale: Report Symbols («`viola-mcp` `tools.rs` `frame`: `check_instance` before `live_endpoint`; a refusal returns `instance-unreachable`, `detail: null`, no frame, no line»), Coverage («tests integ `handlers`») and Outcome «`viola mcp` frame tools → `instance-unreachable` … MET … `binary(handlers)` green».
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:130
note: the body's own lines carrying the same moved facts (.andromeda/test-plan.md:590 and :592 for E5's CARRY and set-aside signals, :604 and :606 for the exit-cause and single-writer suites) are the body's, not this file's; no coverage, framework or harness drift found (every new surface carries a test at its tier, nextest/rstest/assert_cmd unchanged, no harness change).
