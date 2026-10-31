proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 43
    section: "## 2. Test Strategy"
    change: Extend the sentence on the root package's binaries so it reads "The root package carries numbered Path binaries too (`tests/path1_run_start.rs`, sync, `[[test]]` `required-features = ["fake-agent"]`) and one `cross_` binary (`tests/cross_exit_causes.rs`, `[[test]]` `required-features = ["fake-agent"]`, named on the `ci` profile's kill line); both homes report in the E2E layer", the regex that follows unchanged.
    sidecar: test-plan §2 naming, the root package now carries a `cross_` binary, `tests/cross_exit_causes.rs`, which the unchanged E2E regex takes as it took `path1_` (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Harness / gate surface says `Cargo.toml` gains the `[[test]]` with `required-features = ["fake-agent"]`, the `ci` kill line gains `binary(cross_exit_causes)`, and the new table is a root `cross_` binary; its Expected amendments name §2's naming line for this, and the line today places `cross_` in `crates/viola-e2e` only.
    basis: viola-0.1.0/chunks/2026-10-31-exit-cause-code-catalogue/report.md:46
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 42
    section: "## 2. Test Strategy"
    change: Add `start.rs` to the `tests/support/{...}` helper list and, after the `home.rs` sentence, add "`start.rs` holds the start setups root files share (`first_life`, `stopped_life`, `wait_session_start_hooks`, `made_dir`, `remove_dir_once_free`, `plant_agents`, `listed`, `revive`, `kill_wrapper`, `FIXTURE_SESSION`)."
    sidecar: test-plan §2 naming, the shared helper list gains `tests/support/start.rs`, the home of the start setups a root test may not copy (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Counts row `frozen-guard` says `tests/support/start.rs` takes the start setups, its New text lists the added helpers, and deviation 1 moves `remove_dir_once_free`, `FIXTURE_SESSION` and `kill_wrapper` there under the rule against a root file's own copy of what two share; the file's helper list does not name it.
    basis: viola-0.1.0/chunks/2026-10-31-exit-cause-code-catalogue/report.md:80
  - detector: D-new-detail
    severity: warning
    file: .andromeda/test-plan-detail/test-strategy.md
    near: 63
    section: "## 2. Test Strategy"
    change: In the Filesystem (V7) row, at the six frame verbs' refusal, read "exit 21 with the instance pair or the `instance-unreachable` document, its `detail` `strict-modes-failed`, no frame and no line", the rest of the row unchanged.
    sidecar: test-plan §2 trigger map, the strict-modes refusal of the six frame verbs names its `detail`, `strict-modes-failed`, now that exit 21's document is one per cause (chunk 2026-10-31-exit-cause-code-catalogue).
    rationale: The report's Symbols / APIs moves exit 21 from one document to one per cause, and its Counts row `doc-null` gives a strict-modes site the word `strict-modes-failed`; the row's bare "the `instance-unreachable` document" names the form this chunk split.
    basis: viola-0.1.0/chunks/2026-10-31-exit-cause-code-catalogue/report.md:29
note: The body's §2 File naming line still places `cross_` in `viola-e2e` alone and needs the same root `cross_` word, outside this file so not proposed here; D-tests-coverage, D-tests-framework, D-tests-obs-harness and D-tests-count-literals found no violation in this file.
