proposals:
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 44
    section: "## 4. Span / Trace Coverage › #### Scenario: `viola run` start sequence to child spawn"
    change: "Extend the `run.pin_copy` item: the absent copy is written by a no-clobber placement of a closed temp file (`viola_state::fs::place_private_new`, through `pin_bytes`' private `place_absent`); a concurrent first start that loses the placement re-hashes the copy that stands, and `run.pin_copy` reads `ok` when it is equal (the copy reused) or `pinned-hash-mismatch` when it differs (`PinError::HashMismatch`, the file left as found); the placement adds no line, field or span, and the outcome set stays `ok|pinned-hash-mismatch` (chunk 2026-10-27-pinned-copy-race-and-reproducible-builds)."
    sidecar: "span-trace-coverage run-start scenario: `run.pin_copy` names the lost-placement reading of the no-clobber pinned-copy write (F1); outcome set unchanged."
    rationale: "Report Symbols / APIs: `place_private_new` (fs.rs:316-324) and `place_absent` (pin.rs:108-116) replace the arm's `replace_private_shared` write; Schema / config: `run.pin_copy` keeps `ok|pinned-hash-mismatch` and a lost placement reads `ok`; Coverage of new surfaces: instrumentation `run.pin_copy` unchanged."
    basis: crates/viola-state/src/pin.rs:108
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 398
    section: "## 1. Obs Scope Summary"
    change: "Extend the refusal-telemetry item: a pinned-exe hash mismatch is also reached when a concurrent first start loses the copy's no-clobber placement and the copy that stands re-hashes different (`place_absent`'s `PinError::HashMismatch`, `run.pin_copy` `pinned-hash-mismatch`, the file left as found); it writes no new line, field or `detail` code (chunk 2026-10-27-pinned-copy-race-and-reproducible-builds)."
    sidecar: "obs-scope-summary required telemetry: pinned-exe hash-mismatch refusal telemetry names the lost-placement path; no new line."
    rationale: "Report Symbols / APIs: `place_absent`'s `Present` arm re-hashes the standing copy, different is `PinError::HashMismatch`; Schema / config: no `detail` code, log line, field or span added; `src/cmd/run.rs` unchanged."
    basis: crates/viola-state/src/pin.rs:108
note: obs-plan.md:215 and :311 (the broad hash-mismatch telemetry line and the tampered-copy refusal) stay true unchanged; detectors D-obs-instrumentation, D-obs-stack and D-obs-pii read no drift (no new hot-path span, no dependency, `spawn_os_error` a test-only integer receipt field).
