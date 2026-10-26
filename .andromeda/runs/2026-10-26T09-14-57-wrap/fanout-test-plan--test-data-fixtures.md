proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 10
    section: "## 7. Test Data & Fixtures"
    change: "In the On-disk product state row, after the `seed_conpty` exception, add a third: the server-identity impostor cases (chunk 2026-10-26-server-verification-before-any-frame) — `tests/security_negatives_server_identity.rs` `served(label, recorded)`, `tests/hook_fail_open.rs` `Impostor::serve` and `crates/viola-mcp/tests/handlers.rs`'s `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper` — build the instance snapshot as an `InstanceSnapshot` that records a held live process (the fake agent's holder mode `--hold-stdout-internal`: sleeps 10 s, writes nothing, killed and reaped by a `Drop` guard), while a test `viola_channel::Server` serves the endpoint, because no real wrapper records a pid other than its own. The root-bin hook unit cases record this process with a start time a second early, or one that does not parse (`an_earlier_start`), since a unit test has no portable second process. None of these is a booted-wrapper test."
    sidecar: "test-data-fixtures On-disk row: the server-identity impostor cases' test-built snapshot (a held fake-agent holder's pid served by a test Server) recorded as a third exception to never hand-written (chunk 2026-10-26-server-verification-before-any-frame)."
    rationale: "Report Changes: the new file tests/security_negatives_server_identity.rs builds `let snapshot = InstanceSnapshot {` (104-120) beside `Server::bind` (101-103) in `served(label, recorded: u32)` (85-130); hook_fail_open adds `struct Impostor` / `fn serve(home, label)` (182-248); handlers.rs adds `Held` (142-155) and the frame-tools impostor case (359-403); Deviations 3 and 4 name the held process (`--hold-stdout-internal`) and the hook unit start-time variation. The row still says the snapshot is never hand-written and allows only two exceptions, so as written it is false for these cases."
    basis: tests/security_negatives_server_identity.rs:104
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 14
    section: "## 7. Test Data & Fixtures"
    change: "In the `claude agents --json` output row, add: a test that plants more than one fake-agent copy plants every copy (each recorded set's bin) one after another before any thread starts a child, because the tmpfs `target/e2e-home` defeats the hard link, so each copy is written, and a child forked by one thread inherits another thread's open write handle (ETXTBSY). In `fake_agent_agents_replay_matches_every_recorded_agents_set` (`tests/contract_fake_agent_drift.rs`) this was 6 of 40 standalone runs red before and 0 of 40 after (chunk 2026-10-26-server-verification-before-any-frame, evidence/etxtbsy-drift-test.md)."
    sidecar: "test-data-fixtures agents row: fake-agent copies are planted one after another before any thread spawns (the ETXTBSY fold, 6/40 red to 0/40), chunk 2026-10-26-server-verification-before-any-frame."
    rationale: "Report Deviations 1 and the Scope record (in-intent `tests/contract_fake_agent_drift.rs`, inputs#I5): two threads each copied the fake agent and one forked child inherited another's write handle; planting every set's bin one after another before the threads start brought it from 6/40 red to 0/40. New text adds `let planted: Vec<(String, TestHome, PathBuf, Vec<u8>)> = recorded_versions(&root)` (571-583)."
    basis: tests/contract_fake_agent_drift.rs:571
note: test-plan.md:631 (§7 Seed strategies, same two exceptions) and :905 (§11 Test Data, the ban on hand-written snapshots) state the same rule as the line-10 row; they are body lines outside this file, and the orchestrator should check them alongside the first proposal. No count, harness or retired-gap text in this file moved (patterns readers, matrix11, svf, owner, gaps: 0 hits here).
