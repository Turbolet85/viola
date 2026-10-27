# Classified — obs-plan · pii-scrubbing-compliance.b1

## Answers
pii-scrubbing-compliance#1.1 · no · the Cookie-header scan and its diagnostics/ scope are untouched (Untouched §9 names the secret scan), and CI green 15/15 ran it on the changed tree
pii-scrubbing-compliance#1.2 · no · viola-agent-claude, where the R8 strip lives, is unreached and the session adds no log field (Untouched §8), and CI green 15/15 ran the canary test on the changed tree
pii-scrubbing-compliance#1.3 · no · the session adds no credential path, and CI and Section 9 are untouched (Untouched §9)
pii-scrubbing-compliance#1.4 · no · the session adds no log field and each new refusal writes no line (F4, F7), so text_bytes stays the only text logged
pii-scrubbing-compliance#1.5 · no · the hook and answer paths gain only a check that runs before any read or line (F4, F5), and the session adds no log field (Untouched §8)
pii-scrubbing-compliance#1.6 · no · the new check runs before any stdin read (F5), so the drift seam's inputs are unchanged, and the session adds no log field (Untouched §8)
pii-scrubbing-compliance#1.7 · no · a replayed snapshot carries no statusline_command and statusline serve prints nothing over it (F12), and the session adds no log field (Untouched §8)
pii-scrubbing-compliance#1.8 · no · refusals print fixed pairs with no paths (Touches, cli surface) and write no line (F13), and the session adds no log field (Untouched §8)
pii-scrubbing-compliance#1.9 · no · a replay carries no endpoint (F12), the conpty refusal logs only sideload_fallback strict-modes-failed with nothing written (F11), and the version-gate ledger read is unmoved (Touches)
pii-scrubbing-compliance#1.10 · no · a replay carries no pid or started_at (F12) and refusals print no pid (Touches, cli surface), so never-in-detail-codes holds; the FD4 trigger note (obs-plan.md:253) concerns the exit-1 cause code, not pid placement
pii-scrubbing-compliance#1.11 · no · the session adds no log field (Untouched §8), so agents rows and parse-rejected counts are logged as before
pii-scrubbing-compliance#1.12 · no · refusals print the fixed instance pair with no names (Touches, cli surface), and the names test in tests/cli_links.rs is unreached and passed in CI green 15/15
pii-scrubbing-compliance#1.13 · no · the one schema change adds the kebab-case word strict-modes-failed to the closed sideload_fallback enum (Untouched §8), so closed kebab-case enums still holds
pii-scrubbing-compliance#1.14 · no · no fixture file is reached and the session adds no fixture content (Untouched §8), and verify's new refusal writes nothing (F9)
pii-scrubbing-compliance#2.1 · no · the canary scan, its uploads and the harness are untouched (Untouched §9: no harness, CI or gate change), and CI green 15/15 ran the scan on the changed tree
