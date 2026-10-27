proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 36
    section: '## 1. Test Scope Summary'
    change: '  - **As landed (strict-modes, chunk 2026-10-26-home-and-code-bearing-file-integrity):** all eight CLI frame verbs run `viola_state::strict::check_instance` on the named instance (for `link` / `unlink` the driven one) before any role file or log opens: `send`, `wait`, `last`, `answer`, `pause` and `release` through `client::refuse_untrusted`, which `link_call` shares; a refused check exits 21 with the instance pair (`hint:` last), under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, and writes no frame and no line, the mode or ACL left as found (`tests/security_negatives_strict_modes.rs`, three OSes); the check holds the home, the instance directory and its `diagnostics/` to the read rule (on Windows `events.ndjson` too). A `driver` equal to `driven` is refused by the wrapper alone, never client-side (CLI exit 20).'
    sidecar: 'test-scope-summary CLI verbs entity: «only link / unlink run check_instance» retired; all eight frame verbs run it through refuse_untrusted, exit 21 instance pair / instance-unreachable document, no line (chunk 2026-10-26-home-and-code-bearing-file-integrity)'
    rationale: 'The report Retired bullet names the dated strict-modes gaps (4)-(7) of CLI send, wait / last, answer, pause / release as closed; Symbols adds refuse_untrusted called first by the six verbs and shared by link_call; Count 8 has the six frame verbs join the instance-pair list; Outcome criterion 1 MET on three OSes.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 36
    section: '## 1. Test Scope Summary'
    change: '  - **As landed (home tree check):** `viola run` and the start arm of `viola revive` run `check_home_tree` (the home and `instances/<name>/` under the read rule, then `ledger/`, `bin/`, `plugin/`, `bin/<key>/`, the pinned copy and `plugin/<key>/`) before the wrapper log opens: a refusal exits 1 with the home pair, or the instance pair for an instance-level refusal, with no log, no line and no child (`revive --json` keeps `strict-modes-failed`); `viola verify` runs it over the home, `ledger/`, `bin/` and `plugin/` before its `cli` log and refuses exit 1 with the home pair, no step line and no summary, and `update_stamps` refuses before it reads (`tests/run_cli.rs`, `tests/cli_verify.rs`, `tests/security_negatives_strict_modes.rs`). On Unix a home under a regular file is this refusal; on Windows such a path stats `NotFound` and the catch-site line stands.'
    sidecar: 'test-scope-summary CLI verbs entity: the run / revive start / verify home tree check and its exit-1 pairs added (chunk 2026-10-26-home-and-code-bearing-file-integrity)'
    rationale: 'Report Symbols for run.rs (checked_exe, refuse_tree before open_wrapper_log), verify.rs (check_home_tree, refuse_home) and stamps.rs (StampsError), Counts 9 and 13, and Outcome criterion 4 MET; the CLI verbs entity states the exit contract but not this refusal.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 69
    section: '## 1. Test Scope Summary'
    change: '  - **As landed (strict-modes):** the frame tools `send`, `wait`, `last` and `answer` run `check_instance` before the liveness pre-check; a refused instance returns `isError:true` `{"error":"instance-unreachable","detail":null}` with no frame and no line (`crates/viola-mcp/tests/handlers.rs` `handlers_frame_tools_refuse_a_widened_instance_before_any_frame`, chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    sidecar: 'test-scope-summary MCP surface: the frame tools strict-modes refusal as landed (chunk 2026-10-26-home-and-code-bearing-file-integrity)'
    rationale: 'Report Symbols for viola-mcp tools.rs frame (check_instance before live_endpoint, result::unreachable, no frame, no line), Retired gap (8), and Outcome criterion 3 MET by binary(handlers).'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:14
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 127
    section: '## 1. Test Scope Summary'
    change: '    - A widened home tree (a 0770 home, a widened `instances/<name>/`, `bin/` or pinned copy, or a home another user may read) exits 1 with its pair before any log opens (`run_refuses_a_widened_home_tree_before_its_log`, `a_home_another_user_may_read_refuses_run`); the collision check takes the start''s one snapshot read and, over a replayed snapshot, reads liveness from the beat alone (a fresh beat `already-live`, an older one free; chunk 2026-10-26-home-and-code-bearing-file-integrity).'
    sidecar: 'test-scope-summary run start path: the home tree refusal and the collision check over the replay added to its verification signal (chunk 2026-10-26-home-and-code-bearing-file-integrity)'
    rationale: 'Report Symbols for run.rs (checked_exe / refuse_tree before the log; collision_check taking Option<&Recovered>, Replayed read from the beat alone) and Coverage of new surfaces (run_cli, security_negatives_strict_modes); the path signal lists only the duplicate run and the tampered pinned exe as exit-1 cases.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 173
    section: '## 1. Test Scope Summary'
    change: '    - as landed (chunk 2026-10-26-home-and-code-bearing-file-integrity): the `run` start collision check, the hook `connect`, the statusline `serve` and `live_endpoint` read the snapshot through the replay too, and a replayed snapshot carries no `endpoint`, `pid`, `started_at` or `statusline_command`, so no frame is written and no command runs; `tests/chaos_snapshot_replay.rs` drives E5 steps 1-3: a start over an unreadable snapshot replays the log, and a start over a replayed snapshot with a fresh beat refuses `already-live`'
    sidecar: 'test-scope-summary chaos trigger: the four new replay readers and the E5 steps 1-3 binary added (chunk 2026-10-26-home-and-code-bearing-file-integrity)'
    rationale: 'Report Count 5 (replay product readers 2 -> 6, read_snapshot product callers 3 -> 0), Symbols for liveness.rs, hook.rs connect, statusline.rs serve and run.rs collision_check, and Outcome criterion (tests) E5 steps 1-3 MET by binary(chaos_snapshot_replay); line 173 names only revive as the reader.'
    basis: viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md:32
note: 'D-tests-coverage, D-tests-framework, D-tests-obs-harness and D-tests-count-literals hold for this file: every new path has its tests (Coverage of new surfaces), the runner is nextest, no harness change, and no count literal here moved (the report sweep read test-scope-summary.md:128 and :172 as no-change).'
