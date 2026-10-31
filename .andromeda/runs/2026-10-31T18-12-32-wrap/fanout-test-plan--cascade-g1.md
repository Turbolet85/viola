proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 12
    section: '## 1. Test Scope Summary'
    change: 'In **As landed (server verification)**: the impostor case reads "an impostor server gets no frame and the verb exits 21, `detail` `server-verify-failed` under `--json`"; the MCP frame tools read "`instance-unreachable` with `detail` `server-verify-failed`, no frame"; and the last clause reads "the macOS cases are owed to the route entry "Unix endpoint and home hardening"", no longer to the exit-cause matrix.'
    sidecar: 'test-scope-summary: the server-verification record names `server-verify-failed` and the owner of the macOS cases, "Unix endpoint and home hardening".'
    rationale: 'Report Counts `cross-table` lists test-scope-summary.md:12 as changed; Outcome leaves macOS server check with "Unix endpoint and home hardening", and Symbols gives the cause word.'
    basis: .andromeda/test-plan-detail/test-scope-summary.md:12
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 34
    section: '## 1. Test Scope Summary'
    change: 'In the CLI entity: "an exit-1 refusal of its strict-modes check, its preflight or the start it shares with `run` prints `{"v":1,"error":"<code>","detail":null}` on stdout, `<code>` one of `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`, `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error`"; the clause on a refusal the `run` start makes after a passed preflight, printing no document and owed to the catalogue entry, becomes "a `pump_child` error prints no document, the child having held the terminal"; and exit 1 reads "1 for a refusal (the strict-modes check ahead of the log, a preflight reading or a refusal of the start)".'
    sidecar: 'test-scope-summary: `revive --json` holds ten exit-1 codes and one document per refusal of the start; the post-preflight gap and its owed-to clause are gone.'
    rationale: 'Report Retired names the `revive --json` post-preflight gap; Symbols lists the ten codes and says the no-document sentence is no longer true; Counts `revive-gap`, `start-codes-rev` and `title` move.'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 37
    section: '## 1. Test Scope Summary'
    change: 'In **As landed (the instance check)**: under `--json` a refusal reads exactly `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}`, in place of the `detail` null form; the instance pair, exit 21 and no line stay.'
    sidecar: 'test-scope-summary: the CLI instance check under `--json` carries `detail` `strict-modes-failed`.'
    rationale: 'Report Retired names exit-21 `detail: null`; Counts `doc-null` gives a strict-modes site the cause `strict-modes-failed`.'
  - detector: D-tests-count-literals
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 43
    section: '## 1. Test Scope Summary'
    change: 'In the cli surface: under `--json` the exit-1 `{"v":1,"error":"<code>","detail":null}` document carries "its ten codes" in place of "its five codes".'
    sidecar: 'test-scope-summary: the cli surface counts ten `revive --json` exit-1 codes.'
    rationale: 'Report Symbols lists ten codes for `viola revive --json`, and Counts `start-codes-rev` moves every standing site.'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 69
    section: '## 1. Test Scope Summary'
    change: 'In the MCP **Signal**: `isError:true` + `{"error":"instance-unreachable","detail":"<cause>"}`, `<cause>` one of `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, the CLI exit-21 document less `v`, in place of the `detail` null form; its text stays `refused: instance-unreachable`.'
    sidecar: 'test-scope-summary: the MCP `instance-unreachable` result carries one of five closed causes in `detail`.'
    rationale: 'Report Retired names exit-21 `detail: null`; Symbols says MCP `isError` result is the same document less `v`, its text unchanged, with five closed causes.'
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/test-scope-summary.md
    near: 72
    section: '## 1. Test Scope Summary'
    change: 'In **As landed (the instance check)**: a refused instance returns `isError:true` `{"error":"instance-unreachable","detail":"strict-modes-failed"}`, in place of the `detail` null form, with no frame and no line.'
    sidecar: 'test-scope-summary: the MCP instance check carries `detail` `strict-modes-failed`.'
    rationale: 'Report Symbols, MCP: a refused `check_instance` is `StrictModes`, no frame and no line, `isError` the record; Retired names exit-21 `detail: null`.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 43
    section: '## 2. Test Strategy'
    change: 'After "The root package carries numbered Path binaries too (`tests/path1_run_start.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`)" add "and a `cross_` binary (`tests/cross_exit_causes.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`)"; both homes still report in the E2E layer under the same regex.'
    sidecar: 'test-strategy: the naming group records the root `cross_` binary `tests/cross_exit_causes.rs`.'
    rationale: 'Report Harness / gate surface: `Cargo.toml` gains its `[[test]]` with `fake-agent`, and the new table is a root `cross_` binary the E2E regex takes as it took `path1_`; Expected amendments names the naming line.'
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan.md
    near: 324
    section: '## 2. Test Strategy'
    change: 'The layer-prefix bullet ends "`path_`, `mcp_`, `cross_` in `viola-e2e`; the root package also holds a `cross_` binary, `tests/cross_exit_causes.rs`, which the E2E regex takes as it takes `path1_`".'
    sidecar: 'test-plan §2: the naming line no longer places every `cross_` binary in `viola-e2e`.'
    rationale: 'Report Expected amendments names §2 (the naming line, a root `cross_` binary), and Harness / gate surface states the regex reading.'
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 42
    section: '## 2. Test Strategy'
    change: 'New line after the sync E2E suites: `  - **Shared start helpers:** `tests/support/start.rs` shares `first_life`, `wait_session_start_hooks`, `made_dir`, `stopped_life`, `remove_dir_once_free`, `plant_agents`, `listed`, `revive` and `kill_wrapper` across root test files.`'
    sidecar: 'test-strategy: `tests/support/start.rs` and the start helpers it shares are recorded.'
    rationale: 'Report New text lists these functions in `tests/support/start.rs`; Deviations 1 says shared setups moved there because no root file keeps its own copy of what two share.'
    fact: F22
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 9
    section: '## 4. Unit Test Strategy'
    change: 'New line after `is_code_word`: `  - `viola_core::UnreachableCause` holds `StrictModes`, `InstanceDead`, `UnwrappedName`, `EndpointLost` and `ServerVerify` and has no serde path; `unreachable_cause_is_each_its_own_code_word` holds its words (`crates/viola-core/src/lib.rs`).`'
    sidecar: 'unit-test-strategy: `UnreachableCause` and its code-word case are recorded under viola-core.'
    rationale: 'Report Symbols: `UnreachableCause`, no serde path; New text lists the case in `crates/viola-core/src/lib.rs`.'
    basis: crates/viola-core/src/lib.rs:181-190
    fact: F18
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 28
    section: '## 4. Unit Test Strategy'
    change: 'New line after the server check: `  - `viola_state::liveness::without_endpoint(instance_dir)` reads by `symlink_metadata` a directory as `InstanceDead` and anything else (absent, a regular file, a symlink) as `UnwrappedName`, opening nothing and writing no line; its unit cases sit in `crates/viola-state/src/liveness.rs`, and the client mapping is held by `without_endpoint_maps_the_directory_reading` (`src/cmd/client.rs`).`'
    sidecar: 'unit-test-strategy: `without_endpoint` and its directory reading are recorded under viola-state.'
    rationale: 'Report Symbols gives the signature and reading; New text lists liveness.rs added lines and places `without_endpoint_maps_the_directory_reading` in `src/cmd/client.rs`.'
    basis: crates/viola-state/src/liveness.rs:107-115
    fact: F19
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 69
    section: '## 4. Unit Test Strategy'
    change: 'New line after the code-word item: `  - `json_asked` (`src/main.rs`) reads `--json` from argv only before a `--`, held by `json_asked_reads_the_flag_before_a_separator`.`'
    sidecar: 'unit-test-strategy: the catch-site `json_asked` reading and its case are recorded under the root bin.'
    rationale: 'Report Symbols, the `cli` catch site under `--json`; New text lists the case in `src/main.rs`.'
    basis: src/main.rs:154-161
    fact: F20
  - detector: D-cascade
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 87
    section: '## 4. Unit Test Strategy'
    change: '`unreachable_logs_its_cause` (`src/cmd/client.rs`) logs each closed `Unreachable` cause (`instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; `endpoint-lost` `during` `call`, the other three `during` `connect`), in place of the two-cause list.'
    sidecar: 'unit-test-strategy: the closed `Unreachable` causes are four, and only `endpoint-lost` carries `during` `call`.'
    rationale: 'Report Counts `dead-call` re-derives unit-test-strategy.md:87; Symbols says `Unreachable` is now `Dead`, `Unwrapped`, `Lost`, `ServerUnverified`, with `during` `call` for `Lost` only.'
    basis: .andromeda/test-plan-detail/unit-test-strategy.md:87
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 87
    section: '## 4. Unit Test Strategy'
    change: 'New line after the server check before any frame: `  - The `unwrapped-name` and `endpoint-lost` hints are built in `src/human/hints.rs`, held by `send_hint_is_the_design_string`; the hint cause keeps the key `not-running`, not renamed to the log word `instance-dead`.`'
    sidecar: 'unit-test-strategy: the two built hints, their case and the `not-running` key are recorded under the root bin.'
    rationale: 'Report Coverage of new surfaces names the hints and case; Reverted / negative API facts keeps `not-running`; Retired names the not-built line of the unwrapped-name hint.'
    fact: F21
note: The §4 body lines stay true and take no edit; §3 harness, status and log shapes did not move, so D-tests-obs-harness has nothing to compare; the new viola-agent-claude outputs and the `of_reply` failure mapping are F7, F11 and F14 in e2e-test-strategy and integration-test-strategy.
