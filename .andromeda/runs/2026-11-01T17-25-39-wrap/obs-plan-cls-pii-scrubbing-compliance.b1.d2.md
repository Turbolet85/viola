# Classified — obs-plan · pii-scrubbing-compliance.b1

## Answers
pii-scrubbing-compliance#1.1 · no · the session writes no cookie header and the secret-scan test is not among the reached files
pii-scrubbing-compliance#1.2 · no · the R8 strip sits in the viola-agent-claude crate, which the session did not edit
pii-scrubbing-compliance#1.3 · no · no signing credential is added and the CI workflows that hold the secrets are untouched
pii-scrubbing-compliance#1.4 · no · the new hook-decision line carries only the closed word, so no prompt or assistant text reaches a home-level line
pii-scrubbing-compliance#1.5 · no · the new hook refusal writes a codes-only hook-decision line, with no tool input or dialog text
pii-scrubbing-compliance#1.6 · no · no serde drift report is touched; the new refusal is a fixed closed word
pii-scrubbing-compliance#1.7 · no · no statusline code or span is part of the session
pii-scrubbing-compliance#1.8 · no · the new refusal lines hold no path, so home-level lines stay path-free as the row says
pii-scrubbing-compliance#1.9 · no · the new lines log no endpoint or pinned_bin, only a closed word
pii-scrubbing-compliance#1.10 · no · the new socket-dir-refused detail is a closed code and carries no pid
pii-scrubbing-compliance#1.11 · no · claude agents --json parsing is in the viola-agent-claude crate, which the session did not edit
pii-scrubbing-compliance#1.12 · no · the link/unlink refusal writes a process-exit with no pair name, and served-method and channel-pair logging are unchanged
pii-scrubbing-compliance#1.13 · no · the schema moves only process-exit's detail enum at :78 and hook-decision's at :53, so the channel-response three-fault enum still closes its detail as stated
pii-scrubbing-compliance#1.14 · no · the session adds synthetic test scaffolding (SocketRoot), not recorded fixtures
pii-scrubbing-compliance#2.1 · yes · add-to · F10: the new socket-directory refusal lines and documents hold no path, pid or uid, witnessed by digit_runs and assert_names_nothing in tests/security_negatives_socket_dir.rs, beside the server-identity and strict-modes refusal bullets
