proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 87
    section: "## 4. Unit Test Strategy"
    change: "Replace the line's last clause with: `unreachable_logs_its_cause` (`src/cmd/client.rs`) logs each closed `Unreachable` cause (`instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`; `endpoint-lost` `during` `call`, the other three `during` `connect`)."
    sidecar: "§4 root bin, the server check: the closed `Unreachable` causes are four, `instance-dead` no longer `during` `call` and `endpoint-lost` the cause lost with the call open."
    rationale: "Report Retired names `instance-dead` · `during` `call`; Counts `dead-call` re-derives this line; Symbols says `Unreachable` is now `Dead` · `Unwrapped` · `Lost` · `ServerUnverified`, `Dead(&'static str)` gone, and that only `endpoint-lost` carries `during:call`. The line's two causes, with only the server cause `during` `connect`, are the retired shape."
    basis: .andromeda/test-plan-detail/unit-test-strategy.md:87
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 9
    section: "## 4. Unit Test Strategy"
    change: "Add after the line, under viola-core: `  - viola_core::UnreachableCause` in backticks, its five variants `StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify`, by `as_str` `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, no serde path, each its own code word, held by `unreachable_cause_is_each_its_own_code_word` (`crates/viola-core/src/lib.rs`)."
    sidecar: "§4 viola-core: `UnreachableCause` and its unit case added."
    rationale: "Report Symbols adds `viola_core::UnreachableCause`, the five closed causes of the exit-21 document; New text lists the unit case in `crates/viola-core/src/lib.rs`."
    basis: crates/viola-core/src/lib.rs:696
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 87
    section: "## 4. Unit Test Strategy"
    change: "Add after the line, under root bin: `  - The no-endpoint reading: viola_state::liveness::without_endpoint(instance_dir)` in backticks reads `symlink_metadata`, a directory `InstanceDead` and anything else (absent, a regular file, a symlink) `UnwrappedName`, opening nothing and writing no line, the strict-modes check first at every caller; the client's mapping is held by `without_endpoint_maps_the_directory_reading` (`src/cmd/client.rs`). The reading is the delegate's word, PROVISIONAL (`inputs#I2` Q2)."
    sidecar: "§4 root bin: the no-endpoint reading and `without_endpoint_maps_the_directory_reading` added, the delegate's word carried PROVISIONAL."
    rationale: "Report Symbols adds `without_endpoint` and its callers; New text lists the case in `src/cmd/client.rs`; Decisions marks `inputs#I2` Q2 as the delegate's word, PROVISIONAL."
    basis: src/cmd/client.rs:536
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 92
    section: "## 4. Unit Test Strategy"
    change: "Add after the line, under root bin: `  - The catch site's --json reading: json_asked` in backticks (`src/main.rs`) takes `--json` among the arguments before a `--`, read from argv as `role_of` is, its rstest cases `json_asked_reads_the_flag_before_a_separator`."
    sidecar: "§4 root bin: `json_asked` and its rstest cases added."
    rationale: "Report Symbols, the `cli` catch site under `--json`, adds `json_asked`; New text lists the rstest case in `src/main.rs`."
    basis: src/main.rs:447
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/unit-test-strategy.md
    near: 92
    section: "## 4. Unit Test Strategy"
    change: "Add after the line, under root bin: `  - The unwrapped-name and endpoint-lost hints` in backticks (`src/human/hints.rs`), each named by `send_hint_is_the_design_string`; the endpoint-lost hint's wording is the delegate's word, PROVISIONAL (`inputs#I2` Q3)."
    sidecar: "§4 root bin: the two new hints' unit case added, the endpoint-lost wording carried PROVISIONAL."
    rationale: "Report Coverage of new surfaces gives these hints `tests unit + e2e (send_hint_is_the_design_string, the table)`; Outcome places the hints at `src/human/hints.rs:50-55`; Decisions marks `inputs#I2` Q3 PROVISIONAL."
    basis: src/human/hints.rs:50
note: "The report does not list the rows of `unreachable_logs_its_cause` or `send_hint_is_the_design_string`, so the first proposal's four logged causes and the last proposal's two hints rest on its Symbols and Coverage text and were not read from source; D-tests-coverage, D-tests-framework, D-tests-obs-harness and D-tests-count-literals found no further drift in this file."
