# Cascade compare — test-plan · wrap · 2026-11-01-unix-endpoint-and-home-hardening
E items 68 · held 24 · lost at classify 8 · lost at strong 1 · beside a placed fact 0 · cascade only 4 · listed - · body 0 · unplaced 0

## Lost — the full read carried these and the cascade did not; the wrap validates each, marked «(sampler)»

### mocking-stubbing-discipline#2.5 · lost at strong · .andromeda/test-plan-detail/mocking-stubbing-discipline.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/mocking-stubbing-discipline.md
    near: 20
    section: "## 8. Mocking & Stubbing Discipline"
    change: In the Fake servers row's Pattern cell add, before the closing clause on HTTP fake servers, the clause; a test server's endpoint directory is made through `viola_state::fs::create_private_dir`, never a bare `tempfile::tempdir()`, which reads 0755 on a default umask (tempfile 3.27) and which a client's socket-directory check refuses before it reaches the server check (`crates/viola-mcp/tests/handlers.rs`, `tests/hook_fail_open.rs`, `tests/security_negatives_server_identity.rs`, `refused_inside` in `src/cmd/client.rs`);
    sidecar: §8 Fake servers gains the rule that a test server's endpoint directory is made private, a bare tempdir meeting the new socket-directory check, chunk 2026-11-01-unix-endpoint-and-home-hardening.
    rationale: The plan's reading of a 0700 tempdir was measured false, the new client check refused nine handlers cases on the first run, and four sites narrowed (report.md:56, report.md:96); the check runs ahead of the euid read and verify_server (report.md:15), so an impostor in a bare tempdir would not exercise the server check this row describes.
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:56

### quality-gates-coverage-targets#3.1 · lost at classify · .andromeda/test-plan-detail/quality-gates-coverage-targets.md:26
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/quality-gates-coverage-targets.md
    near: 26
    section: "## 10. Quality Gates & Coverage Targets"
    change: New continuation line after this one, indented as it is. Since chunk 2026-11-01-unix-endpoint-and-home-hardening each timed hook's `connect` also runs the socket-directory check (`check_endpoint_dir`, before it connects) and the peer euid decision (`peer_admitted`, after it connects), both before its first frame and each a real check on Unix only; with those two in the timed path the five hook rows read a mean of 2.0, 2.0, 2.0, 2.0 and 2.3 ms and a highest `max` of 2.8 ms, under `SPINE_DEADLINE`, as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening (report, Outcome, `run --perf`). The `max` reading above stays the record of its own chunk, and no gate value moves.
    sidecar: test-plan §10 gate-perf names the socket-directory check and the peer euid decision in the timed hook path, with that chunk's reading; no value moved (chunk 2026-11-01-unix-endpoint-and-home-hardening).
    rationale: The report puts both new checks in the timed path and gives the new reading, and states the existing band stays a true record with no value moved, so this adds a line and edits none. This item names only the two earlier checks.
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:38

### test-data-fixtures#1.6 · lost at classify · beside a placed fact F22 (near 14, test-data-fixtures#1.5) · .andromeda/test-plan-detail/test-data-fixtures.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 15
    section: "## 7. Test Data & Fixtures"
    change: "Add a table row after `Widened test homes`: | A test-owned socket root | `tests/support/home.rs` `SocketRoot` (`new` · `path` · `dir` · `made`), under `/tmp`; `SOCKET_ROOT_VAR` names the variable that carries it (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`); `Wrapper::boot_on_socket_root` boots on it, and `first_life_on(stamped, root, open_umask)` stands in `home.rs`, not beside `first_life` in `tests/support/start.rs`, which the guard entry's pattern does not allow. `tests/security_negatives_socket_dir.rs` boots through them. | per-test |"
    sidecar: "§7 gains the test-owned socket root row (`SocketRoot`, `SOCKET_ROOT_VAR`, `Wrapper::boot_on_socket_root`, `first_life_on`), chunk 2026-11-01-unix-endpoint-and-home-hardening."
    rationale: "The report's Symbols / APIs names these new test helpers and Expected amendments names the test-owned socket roots under /tmp; the category's table has no row for this test data (report.md:21, report.md:63, report.md:81)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:21

### test-data-fixtures#1.6 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:15
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 15
    section: "## 7. Test Data & Fixtures"
    change: "Add a table row after `Widened test homes`: | A loosened socket directory (the socket-directory negatives) | Set by the test on the real directory, never by viola, whose check never chmods it: `tests/security_negatives_socket_dir.rs` `Form` (`apply` · `assert_as_found` · `restore`), a directory at 0755 or a symlink to 0700, the refused directory asserted as found. A foreign owner is not driven: `socket_dir_verdict` takes it by an injected uid. | per-test |"
    sidecar: "§7 gains the loosened socket directory row (`Form` in `tests/security_negatives_socket_dir.rs`), chunk 2026-11-01-unix-endpoint-and-home-hardening."
    rationale: "The chunk's new negatives alter a real directory as the strict-modes negatives alter a real home, and the Widened test homes row names only the home helpers (report.md:11, report.md:107, report.md:130)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:107

### test-data-fixtures#1.10 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 19
    section: "## 7. Test Data & Fixtures"
    change: "In the `Human-mode expected output` row, before `are literals in their test files`, add the clause: and the new refusal pair's last `hint:` line (`tests/cross_exit_causes.rs` `SOCKET_DIR`, `tests/security_negatives_socket_dir.rs` `HINT`), a planning form, its wording owed to the founder at Epoch 7's boundary batch,"
    sidecar: "§7 Human-mode expected output names the socket-directory hint literals and their planning-form status, chunk 2026-11-01-unix-endpoint-and-home-hardening."
    rationale: "The chunk adds a closed refusal pair pinned as test literals, its wording a planning form with the founder's word owed (report.md:49, report.md:52, report.md:76, report.md:252, report.md:264)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:76

### test-data-fixtures#2.3 · lost at classify · .andromeda/test-plan-detail/test-data-fixtures.md:32
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-data-fixtures.md
    near: 32
    section: "## 7. Test Data & Fixtures"
    change: "Add a bullet after the home-made-before-viola bullet: - A directory holding a test's endpoint, where the case needs it to pass the socket-directory check, is made through `viola_state::fs::create_private_dir`, never left a bare `tempfile::tempdir()`: `tempfile` 3.27 leaves the mode to the umask (0755 as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening), and such a directory meets `check_endpoint_dir` as a home meets the read rule, as `crates/viola-mcp/tests/handlers.rs`, `tests/hook_fail_open.rs`, `tests/security_negatives_server_identity.rs` and `refused_inside` in `src/cmd/client.rs` do. It binds with §5's own line that a bare temp dir reads 0755 under the umask."
    sidecar: "§7 records that a test's endpoint directory is made through `create_private_dir`, the 0700 temp dir reading disproved at chunk 2026-11-01-unix-endpoint-and-home-hardening."
    rationale: "The report measured the claim that impostor endpoints sit in a 0700 tempdir false, narrowed four sites, and calls a bare tempdir a sweep hazard for any test placing an endpoint in one; the category states the private-dir rule for a home alone (report.md:56, report.md:96)."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:56

### test-scope-summary#7.1 · lost at classify · .andromeda/test-plan-detail/test-scope-summary.md:122
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 122
    section: "## 1. Test Scope Summary"
    change: "Append to this constraint: A test placing an endpoint in a directory of its own makes that directory the same way: `tempfile` 3.27 leaves a `tempfile::tempdir()`'s mode to the umask, so on a default umask it reads 0755 and the socket-directory check refuses it."
    sidecar: "§1 test data strategy: a test-made socket directory goes through `create_private_dir`, as a home does."
    rationale: "Report Spec claims disproved measured the bare temp dir at 0755, the client check refusing nine `handlers` cases on the first run, and four sites narrowed through `viola_state::fs::create_private_dir`; Decisions names it a sweep hazard."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:56

### test-strategy#1.1 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:10
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 10
    section: "## 2. Test Strategy"
    change: "Append to the Unit row's tier depth, after the `server_pid` clause: the socket-directory check's captured-span cases `connect_refuses_a_loosened_socket_directory_inside_its_dispatch_span` and `hook_refuses_a_loosened_socket_directory_inside_hook_handle` run on macOS, reading the span recorder `Seen` from the test module `refused_seen` in `src/cmd/client.rs`, outside the macOS-gated `refused_inside` (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    sidecar: "test-strategy Unit row: the socket-directory captured-span cases run on macOS through `refused_seen`, the server check's cases staying compiled out there."
    rationale: "The report's Deviations (item 4) moves `Seen` out of the gated `refused_inside` so two new cases run on macOS, while this row names `refused_inside` as the macOS-gated helper and no captured-span case running there; the report's New text lists these two as the chunk's captured-span cases, which is the reading that names them."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:82

### test-strategy#1.7 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:29
  - detector: D-tests-coverage
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 29
    section: "## 2. Test Strategy"
    change: "Append to the Mutation row's tier depth: a known survivor stands against the `missed` rule, a mutant replacing a client seam's `peer_admitted` read (`src/cmd/client.rs` `connect`, `src/cmd/hook.rs` `connect`, `crates/viola-mcp/src/tools.rs` `frame`) with an always-admit survives every committed test by construction, because one account can make no foreign peer and no seam may be added; the wiring was read only once, by forced refusals in that chunk's remove-the-guard runs, and the mutation audit reads the survivor at the route entry \"Version done-check\" (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    sidecar: "test-strategy Mutation row: the client seams' euid wiring named as a survivor the boundary audit will read, with no committed test at any tier."
    rationale: "The client euid wiring is a new path with no committed test at the mandated tier; the report's Insufficient fixes rules the remainder to `matrix#v1-44` and test-plan section 1's untestable zone and says the audit reads the same survivor, while this row states any unexcluded `missed` outcome fails the run without naming it."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:54

### test-strategy#2.1 · lost at classify · .andromeda/test-plan-detail/test-strategy.md:43
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 43
    section: "## 2. Test Strategy"
    change: "New bullet following this line, at its indent: **Socket-root helpers:** `tests/support/home.rs` holds the test-owned socket root: `SOCKET_ROOT_VAR` (`TMPDIR` on macOS, else `XDG_RUNTIME_DIR`), `SocketRoot`, `Wrapper::boot_on_socket_root` and `first_life_on`; `first_life_on` stands there, not beside `first_life` in `tests/support/start.rs` (chunk 2026-11-01-unix-endpoint-and-home-hardening)."
    sidecar: "test-strategy naming: the socket-root helpers in `tests/support/home.rs` recorded, `first_life_on` placed apart from `first_life`."
    rationale: "The report's Symbols / APIs adds these test helpers and its Deviations (item 3) places `first_life_on` in `home.rs`; this file describes `home.rs` only as the home-widening helpers and `start.rs` as the shared start helpers."
    basis: .andromeda/cache/docs/2026-11-01-unix-endpoint-and-home-hardening/report.md:21

## Held

- e2e-test-strategy#12.1
- e2e-test-strategy#13.1
- e2e-test-strategy#14.1
- e2e-test-strategy#15.1
- integration-test-strategy#3.0
- integration-test-strategy#4.1
- integration-test-strategy#4.2
- integration-test-strategy#5.1
- mocking-stubbing-discipline#2.6
- mocking-stubbing-discipline#2.7
- quality-gates-coverage-targets#2.0
- test-scope-summary#1.1
- test-scope-summary#3.1
- test-scope-summary#4.1
- test-scope-summary#5.3
- test-scope-summary#6.1
- test-scope-summary#8.1
- test-strategy#3.1
- test-strategy#3.16
- test-strategy#3.6
- unit-test-strategy#1.1
- unit-test-strategy#1.2
- unit-test-strategy#1.3
- unit-test-strategy#2.1

## Cascade only

- e2e-test-strategy#2.2
- integration-test-strategy#1.4
- test-data-fixtures#1.5
- test-data-fixtures#2.1
