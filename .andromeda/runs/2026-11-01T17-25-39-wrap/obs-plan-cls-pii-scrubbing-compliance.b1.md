# Classified — obs-plan · pii-scrubbing-compliance.b1

## Answers
pii-scrubbing-compliance#1.1 · no · the Cookie secret-scan rule is not in a reached file and the session writes no Cookie-bearing line
pii-scrubbing-compliance#1.2 · no · R8 strip and env-map handling are not in the reached files and the session writes no CLAUDE* value
pii-scrubbing-compliance#1.3 · no · no signing credential is involved and the CI workflow files are untouched
pii-scrubbing-compliance#1.4 · no · the new refusal lines carry no prompt or assistant text, and the report leaves the redact and skip_all attributes on those params unchanged
pii-scrubbing-compliance#1.5 · no · the new hook-decision line carries only the socket-dir-refused code, no dialog text, so the redact and codes-only dialog rules hold
pii-scrubbing-compliance#1.6 · no · serde_path_to_error drift reports are not touched by the session
pii-scrubbing-compliance#1.7 · no · statusline_command and its shell_out span are not touched by the session
pii-scrubbing-compliance#1.8 · no · the new refusal lines hold no path (F10 is placed on #2.1), so the rule that home-level lines stay path-free still holds
pii-scrubbing-compliance#1.9 · no · endpoint and pinned_bin are still never logged, and the new refusal lines log only the code
pii-scrubbing-compliance#1.10 · no · pid and uid stay typed fields and the new detail code carries neither, so the Founder Direction 4 rule holds
pii-scrubbing-compliance#1.11 · no · viola-agent-claude is untouched and the claude agents --json parse is unchanged
pii-scrubbing-compliance#1.12 · no · link and unlink refusals write a process-exit line with the code only, the pair names stay out of process-log lines, and the human pair prints only the parsed ViolaName
pii-scrubbing-compliance#1.13 · no · the schema change moves the process-exit and hook-decision detail enums, not the three-code channel-response enum this row names
pii-scrubbing-compliance#1.14 · no · recorded fixtures are not a telemetry source and the session adds no fixture content
pii-scrubbing-compliance#2.1 · yes · add-to · F10: the new socket-dir refusal lines and documents hold no path, pid or uid, witnessed by assert_names_nothing and digit_runs in tests/security_negatives_socket_dir.rs, joining the refusal witnesses listed here
