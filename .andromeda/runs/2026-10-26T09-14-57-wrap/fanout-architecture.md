proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (IPC endpoints, the Test-only row)
    change: "Add the two new creators of the test-only `viola-test-chan-<pid>-<label>` endpoints beside the channel tests: `tests/security_negatives_server_identity.rs` (`served`, the impostor and control servers of the six-verb server-identity negatives) and `tests/hook_fail_open.rs` (`Impostor::serve`, the hook matrices' impostor rows); both stay outside the `viola-<h12>` namespace, unique per test and never bound by `viola`."
    sidecar: "§Occupied Resources IPC endpoints: the test-only `viola-test-chan-` row names the server-identity negatives and the hook fail-open impostor as creators (chunk 2026-10-26-server-verification-before-any-frame)."
    rationale: "Report Changes › Harness / gate surface names the new test binary `security_negatives_server_identity`; New text, by line lists its `served` (85-130) with `let endpoint = if cfg!(windows)` (91-99) and `Server::bind` (101-103), and `tests/hook_fail_open.rs`'s `Impostor` (182-190) with `fn serve(home: &Path, label: &str)` (194-248) — each binds a test endpoint of the registered `viola-test-chan-` shape, and the registry row lists only the earlier creating test files, so the new binders are unregistered."
    basis: "tests/security_negatives_server_identity.rs 85-130; tests/hook_fail_open.rs 194-248"
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Environment variables)
    change: "Register, beside `MCP_SERVE_TEST_CHILD`, the test-only `MCP_HANDLERS_TEST_HELD`: a marker `crates/viola-mcp/tests/handlers.rs` sets by `Command::env` on the held child it spawns (`Held::spawn`), read only by that test binary's child entry `handlers_held_child_entry`; read by no `viola` build, outside the `VIOLA_*` namespace, which is product plumbing."
    sidecar: "§Occupied Resources Environment variables: test-only `MCP_HANDLERS_TEST_HELD` registered (viola-mcp handlers test's held child, chunk 2026-10-26-server-verification-before-any-frame)."
    rationale: "Report New text, by line: `crates/viola-mcp/tests/handlers.rs` added 125-173, with `fn handlers_held_child_entry()` (128-136) reading `if std::env::var_os(HELD).is_none()` (132-134) and `Held::spawn` (144-154) — a new environment variable read by a test child, unregistered in §Occupied Resources, whose sibling `MCP_SERVE_TEST_CHILD` is registered there."
    basis: "crates/viola-mcp/tests/handlers.rs 128-136"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (Instance snapshot)
    change: "`pid` and `started_at` identify the wrapper process for the liveness check and, on Linux and Windows, for the server check every frame-writing client (the six CLI frame verbs, `viola hook`'s event and dialog paths, `viola mcp`'s four frame tools) makes before its first frame: the connected stream's peer pid and that process's OS start time against these two fields, a mismatch writing no frame (macOS: no peer pid, the recorded gap)."
    sidecar: "§Standard Contracts Instance snapshot: `pid` + `started_at` now also read by the pre-frame server identity check of every frame-writing client (chunk 2026-10-26-server-verification-before-any-frame)."
    rationale: "Report Changes › Symbols / APIs: `viola_state::liveness::server_verdict` / `verify_server` compare the recorded pid and `started_at` with `Client::server_pid` and the observed start time; `src/cmd/client.rs` `connect`, `src/cmd/hook.rs`'s shared `connect` and `crates/viola-mcp/src/tools.rs` `server_verified` call it before any frame. The snapshot contract names only the liveness check as these fields' use."
    basis: "crates/viola-state/src/liveness.rs 113-140"
  - detector: D-retired
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: "viola-state's liveness entry reads: the live-endpoint read (`liveness::live_endpoint` → `LiveEndpoint {endpoint, pid, started_at}`, one snapshot read, shared by the CLI frame verbs and viola-mcp's frame tools) and the pure server identity check (`ServerVerdict`, `server_verdict`, `verify_server`: the recorded pid + start time against the connected peer's, used by the CLI frame verbs, `viola hook` and viola-mcp's frame tools before any frame) — no longer a liveness-only pre-check."
    sidecar: "Project directory structure: viola-state liveness gains `LiveEndpoint` and the server identity check; the frame verbs' liveness-only pre-check retired (chunk 2026-10-26-server-verification-before-any-frame)."
    rationale: "Report Retired: the server-identity half of the first and fourth to eighth dated gaps (`hook.event`; CLI `send`; `wait` / `last`; `answer` + `hook.dialog`; `pause` / `release`; `viola mcp`'s four frame tools) is closed — each now verifies the serving process before its first frame; Symbols / APIs: `live_endpoint` returns `LiveEndpoint` (liveness.rs 72-82), new `ServerVerdict` (102-111), `server_verdict` (113-140), `verify_server`. The key file still calls the shared read the liveness-only endpoint read."
    basis: "crates/viola-state/src/liveness.rs 102-111"
  - detector: D-retired
    severity: warning
    section: §Infrastructure Patterns → Crate dependency direction
    change: "viola-state's sysinfo use reads: the pid + start-time liveness check, shared by CLI `list` and `mcp`, and the server identity check's start-time read (`liveness::verify_server`), used before the first frame by CLI `send` / `wait` / `last` / `answer` / `pause` / `release`, `viola hook` and viola-mcp's frame tools; `liveness::live_endpoint(instance_dir)` returns `LiveEndpoint {endpoint, pid, started_at}` from one snapshot read, so those callers' pre-frame check is no longer liveness-only."
    sidecar: "Crate dependency direction: viola-state's sysinfo use names the pre-frame server identity check; `live_endpoint` returns `LiveEndpoint` (chunk 2026-10-26-server-verification-before-any-frame)."
    rationale: "Same retired claim as the project-directory-structure proposal: report Retired (the frame verbs' and MCP tools' liveness-only pre-check, server-identity half closed on Linux and Windows) and Symbols / APIs (`LiveEndpoint`, `verify_server` reading the start time through `process_start_time`; callers `src/cmd/{send,wait,last,answer,pause}.rs` and `crates/viola-mcp/src/tools.rs`, `release` through `client::connect`). No dependency added (report Dependencies)."
    basis: "crates/viola-state/src/liveness.rs 113-140"
    dependent-of: D-retired
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: "The root bin's `cmd/client.rs` entry reads: a helper, no subcommand — `connect`, the verified connection every CLI frame verb (`send`, `wait`, `last`, `answer`, `pause`, `release`) opens (live endpoint → server identity check → client), the closed `Unreachable` (`instance-dead` · `server-verify-failed`) and `CallError`, and the channel client those verbs share."
    sidecar: "Project directory structure: `src/cmd/client.rs` named as the verified `connect` of the six CLI frame verbs (chunk 2026-10-26-server-verification-before-any-frame)."
    rationale: "Report Changes › Symbols / APIs: `src/cmd/client.rs` adds `Unreachable` (75-82), `CallError` (110-116) and `pub(crate) fn connect(live: &LiveEndpoint)` (124-139), the verified connection every CLI frame verb now uses; the six verbs each route `request` through it. The key file describes client.rs as only the channel client `send` / `wait` / `last` share."
    basis: "src/cmd/client.rs 124-139"
note: "Not proposed (no detector invariant reached): the crate-dependency-direction key's viola-channel line lists `TokioClient`'s API as `connect`, `conn`, `request` and omits the new `server_pid` (both clients) and `SERVER_PID_REPORTED`, read through interprocess 2.4.4's `peer_creds` with no dependency or feature added — an orchestrator call whether to add it. D-arch-decisions: no violation (no dependency added or bumped; tokio `spawn_blocking` stays inside viola-mcp; the hook path stays on std, `channel-tokio-free.sh` green). Pre-existing, not this chunk's diff: `crates/viola-mcp/tests/handlers.rs` binds `viola-test-mcp-<pid>-<label>` endpoints (not in its added ranges), which the Test-only IPC row does not name. svf: 0 architecture body sites, as the report reads."
