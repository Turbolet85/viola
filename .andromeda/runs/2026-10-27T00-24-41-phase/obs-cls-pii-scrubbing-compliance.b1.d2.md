# Classified — obs · pii-scrubbing-compliance.b1

## Answers
pii-scrubbing-compliance#1.1 · no · standing S5 · new test output counts toward the zero budgets, 0 secret-scan hits
pii-scrubbing-compliance#1.2 · no · the chunk reads and passes no env value, and the R8 strip it runs inside the start stays unchanged
pii-scrubbing-compliance#1.3 · no · v1 has no signing credential, and the release measurement is a local build with no CI secret
pii-scrubbing-compliance#1.4 · no · the send path is not reached, and the chunk's new lines carry no prompt or assistant text
pii-scrubbing-compliance#1.5 · no · the dialog and tool-input path is not reached (scope: Dialog scenario not needed)
pii-scrubbing-compliance#1.6 · no · the chunk's changes parse no serde input, so no drift report arises
pii-scrubbing-compliance#1.7 · no · the chunk touches no status line or statusline_command read (scope: statusline shell-out not needed)
pii-scrubbing-compliance#1.8 · yes · the pin write's io-error path stays in detail-<process>.ndjson, and a measured path remap must keep panic_location workspace-relative
pii-scrubbing-compliance#1.9 · yes · pinned_bin (the bin/<key> path) appears in no span or line of the pin step
pii-scrubbing-compliance#1.10 · yes · the new lost-race process-exit detail code carries no pid, which stays a typed field
pii-scrubbing-compliance#1.11 · no · the claude agents --json parse is outside the scope, and verify's agents read is unchanged
pii-scrubbing-compliance#1.12 · no · no link, unlink or channel method is added or changed, and the new lines name no pair
pii-scrubbing-compliance#1.13 · yes · the new lost-race exit detail code and the typed error's WARN/ERROR code are closed kebab-case enums
pii-scrubbing-compliance#1.14 · no · the chunk records no fixture, and its re-read evidence files are measurement notes
pii-scrubbing-compliance#2.1 · yes · the witness reads the merged agent-run logs output the scope names, which carries the canary by design and sits under the scan's target/agent-run rule
