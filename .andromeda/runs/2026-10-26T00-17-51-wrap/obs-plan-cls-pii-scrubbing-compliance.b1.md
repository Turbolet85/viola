# Classified — obs-plan · pii-scrubbing-compliance.b1

## Answers
pii-scrubbing-compliance#1.1 · no · the secret-scan inputs and scan path are unchanged, and the five new Windows cases assert pipe admission rather than diagnostics content
pii-scrubbing-compliance#1.2 · no · the secret-scan inputs (canaries included) and scan path are unchanged, and no product file changed, so the R8 name-only log fields stand
pii-scrubbing-compliance#1.3 · no · no workflow file was edited, and the Cargo.toml change reaches neither Cargo.lock nor the release graph
pii-scrubbing-compliance#1.4 · no · the send-refused product line and its fields are unmoved (the case_4 fix is test-capture only), and veil Redact stays as it was
pii-scrubbing-compliance#1.5 · no · the dialog module's asserted events, fields and levels are unmoved (only its capture call moved), so codes-only dialog events hold
pii-scrubbing-compliance#1.6 · no · no product file changed, and no reached file or touch names the serde_path_to_error seam or its drift_report detail field
pii-scrubbing-compliance#1.7 · no · the statusline's asserted fields are unmoved (only its capture call moved), so the shell_out span still records shell_exit_status and duration_ms only
pii-scrubbing-compliance#1.8 · no · the logging-stack claims (with_file(false), basename and workspace-relative location fields) are re-read unmoved, and src/main.rs gained only a cfg(test) helper and witness
pii-scrubbing-compliance#1.9 · no · the budget and run-start asserted fields are unmoved (only capture calls moved), so logged thresholds and cli_version stand
pii-scrubbing-compliance#1.10 · no · the budget-gate asserted fields are unmoved (only its capture call moved), so the typed pid, percentage and reset fields stand
pii-scrubbing-compliance#1.11 · no · no agents-read module is among the reached files and the run-start asserted fields are unmoved, so rows stay unlogged verbatim
pii-scrubbing-compliance#1.12 · no · no refusal word, exit code or closed enum moved, so protocol codes and refusal details stand
pii-scrubbing-compliance#1.13 · no · no fixture is among the reached files and obs adds no fixture content
pii-scrubbing-compliance#2.1 · no · the scan, its inputs, the upload set and the gating are unchanged (no workflow edited), and the new cases land in the junit artifact, not a scanned upload
