proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 10
    section: "## 7. Test Data & Fixtures"
    change: "In the On-disk product state row, «Two exceptions» becomes three: after the `seed_conpty` exception add «and the server-identity impostor cases (chunk 2026-10-26-server-verification-before-any-frame; `tests/security_negatives_server_identity.rs`'s `served` and `tests/hook_fail_open.rs`'s `Impostor::serve`, compiled out on macOS), which write an `InstanceSnapshot` recording a held fake-agent pid (`--hold-stdout-internal`) beside an endpoint the test itself binds, so that a server that is not the recorded wrapper answers the frame-writing clients»."
    sidecar: "test-data-fixtures: the on-disk product-state rule gains the server-identity impostors' test-built snapshot as a third named exception."
    rationale: "The report's New text shows `served(label, recorded)` building `let snapshot = InstanceSnapshot {` (tests/security_negatives_server_identity.rs 104-120) beside `Server::bind(&endpoint)` (101-103), and `Impostor::serve(home, label)` (tests/hook_fail_open.rs 194-248); outcome deviation 4 names the fake agent's holder mode as the held recorded-pid process of steps 13 and 14. A snapshot the CLI verbs read from disk that no viola run wrote contradicts the row's closed two-exception list."
    basis: "viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:79"
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 631
    section: "## 7. Test Data & Fixtures"
    change: "The On-disk product state broad line's exceptions read «except the chaos tests' corruption of files the product already wrote, the Windows x64 `seed_conpty` companions and the server-identity impostors' test-built `snapshot.json`»."
    sidecar: "test-plan §7: the on-disk product-state broad line names the server-identity impostors' snapshot as an exception."
    rationale: "Second occurrence of the test-data-fixtures#1.1 amendment: the broad line restates the same closed exception list."
    dependent-of: D-cascade
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 52
    section: "## 7. Test Data & Fixtures"
    change: "After the `--exit-no-eof` clause add «; its holder's internal mode `--hold-stdout-internal` (sleeps 10 s, writes nothing) is also spawned directly by the server-identity negatives (`tests/security_negatives_server_identity.rs`) and the hook fail-open impostor rows (`tests/hook_fail_open.rs`) as the live process whose pid the impostor's snapshot records, killed and reaped by each file's `Held` `Drop` guard (chunk 2026-10-26-server-verification-before-any-frame); no option is added»."
    sidecar: "test-data-fixtures: the fake agent's `--hold-stdout-internal` holder mode named as the server-identity negatives' held recorded-pid process."
    rationale: "Outcome deviation 4: «Steps 13 and 14's held process is the fake agent's holder mode `--hold-stdout-internal` (sleeps 10 s, writes nothing, killed and reaped by a `Drop` guard), not a bare `viola-fake-agent` spawn»; the New text lists `impl Held` / `impl Drop for Held` in tests/security_negatives_server_identity.rs (35-53) and tests/hook_fail_open.rs (161-167). The fake agent itself is not among the 18 changed files, so the argv option count stands."
    basis: "viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:79"
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 18
    section: "## 8. Mocking & Stubbing Discipline"
    change: "In the Process mocking row's rule, after the viola-e2e lookup add «; a held recorded-pid process standing behind an impostor endpoint is a real process, never a double: root tests spawn the fake agent's `--hold-stdout-internal` holder, and `crates/viola-mcp/tests/handlers.rs` spawns its own `Held` child (entered at `handlers_held_child_entry` only when its `HELD` variable is set), each killed and reaped by a `Drop` guard (chunk 2026-10-26-server-verification-before-any-frame)»."
    sidecar: "mocking-stubbing-discipline: the Process mocking row names the held recorded-pid processes of the server-identity impostor cases."
    rationale: "The report's New text lists `fn handlers_held_child_entry` (crates/viola-mcp/tests/handlers.rs 128-136) with `if std::env::var_os(HELD).is_none()` (132-134), `impl Held` / `fn spawn` (142-155) and `impl Drop for Held` (157-163), used by `handlers_frame_tools_write_no_frame_to_a_server_that_is_not_the_recorded_wrapper` (359-403); the row names only `CARGO_BIN_EXE_viola-fake-agent` in root tests and the assert_cmd lookup for viola-e2e, so a member crate's own process stand-in is unnamed. No mockall seam was added (Untouched: §8)."
    basis: "viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:173"
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/ci-integration.md
    near: 10
    section: "## 9. CI Integration"
    change: "After the Windows-target clippy sentence add «Beside it, a macOS-target check-only clippy is a host recipe on the Linux dev host, first used by chunk 2026-10-26-server-verification-before-any-frame's operator pass (`CARGO_TARGET_DIR=target/maccheck cargo clippy … --target aarch64-apple-darwin`, nothing linked, the target std installed as host preparation by `rustup target add aarch64-apple-darwin`, 2026-10-26): it reproduced CI's `lint (macos-latest)` error line for line; it is neither a CI step, nor a `pre-push` stage, nor a plan gate entry»."
    sidecar: "ci-integration: the macOS-target check-only clippy recorded as a dev-host recipe beside the Windows-target one, not a gate."
    rationale: "Report Harness / gate surface: «A host lint recipe used for the first time: the macOS-target clippy (`--target aarch64-apple-darwin`, `target/maccheck`), its known-positive control reproducing CI's `lint (macos-latest)` error line for line — not a plan gate entry»; Dev-tool versions: `rustup target add aarch64-apple-darwin`, 2026-10-26."
    basis: "viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:29"
    fact: F13
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 741
    section: "## 9. CI Integration"
    change: "The Lint broad line ends «the `--acyclic` review and the dev-host Windows- and macOS-target check-only clippy runs are not gates»."
    sidecar: "test-plan §9: the Lint broad line names the macOS-target check-only clippy beside the Windows-target one as not a gate."
    rationale: "Second occurrence of F13: the broad line the ci-integration Lint group details names only the Windows-target clippy as not a gate."
    dependent-of: D-new-detail
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 25
    section: "## 10. Quality Gates & Coverage Targets"
    change: "After «never the no-instance no-op (obs-plan §10 obs-overhead row)» add «, and since chunk 2026-10-26-server-verification-before-any-frame every timed hook also makes its server check (the peer pid and its OS start time against the one snapshot read) before its first frame; with the check in the timed path the hook rows read `max` 2.2–2.4 ms over 30 runs each on the Linux dev host (against 1.23–1.6 ms read at that chunk's P5), the rows and the gate unchanged»."
    sidecar: "quality-gates-coverage-targets: the perf session's timed path includes the hook's server check; Linux dev-host reading max 2.2–2.4 ms."
    rationale: "Report Outcome: «(tests) `run --perf`: every hook row `max < SPINE_DEADLINE`, zero panics, the check in the timed path — MET (hook rows max 2.2–2.4 ms over 30 runs each, against P5's 1.23–1.6 ms; the bound 1 s)»; the shared hook `connect` (`read_snapshot` → `connect_by` → `verify_server`) serves both `deliver` and `ask`."
    basis: "viola-0.1.0/chunks/2026-10-26-server-verification-before-any-frame/report.md:111"
    fact: F12
note: "crates/viola-mcp/tests/handlers.rs's impostor case (booted_recording + Held, 359-403) may also write a test-built snapshot — the report does not show it, so test-data-fixtures#1.1 names only the two impostors it does; the readers count and the matrix11 literal are group 1's (e2e / unit strategy)."
