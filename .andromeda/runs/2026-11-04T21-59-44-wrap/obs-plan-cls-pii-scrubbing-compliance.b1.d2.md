@answer n=1 entity=pii-scrubbing-compliance#1.1 v=no
why: the session changed no secret-scan test, and the Cookie-header scan over diagnostics/ is unchanged
@answer n=2 entity=pii-scrubbing-compliance#1.2 v=no
why: the R8 env strip, the CLAUDE* canaries and their scan are product and test code the session did not move
@answer n=3 entity=pii-scrubbing-compliance#1.3 v=no
why: CI secrets are unchanged, and .github/ and the secret-scan step were not touched
@answer n=4 entity=pii-scrubbing-compliance#1.4 v=no
why: prompt and assistant-output redaction lives in src/ and veil attributes, none of which the session moved
@answer n=5 entity=pii-scrubbing-compliance#1.5 v=no
why: tool input and dialog-answer redaction is unchanged src code that the test-only session did not touch
@answer n=6 entity=pii-scrubbing-compliance#1.6 v=no
why: the serde_path_to_error fixed Display seam is product code, unchanged by a test-only session
@answer n=7 entity=pii-scrubbing-compliance#1.7 v=no
why: the statusline.shell_out span and its exit-status fields are product code, unchanged
@answer n=8 entity=pii-scrubbing-compliance#1.8 v=no
why: basename file fields and workspace-relative panic_location are product logging rules the session left alone
@answer n=9 entity=pii-scrubbing-compliance#1.9 v=no
why: product config thresholds and the endpoint and pinned_bin logging rule are unchanged, and .config/nextest.toml is test-runner config with no logged value
@answer n=10 entity=pii-scrubbing-compliance#1.10 v=no
why: the child-end test reads pid and start time in-test without logging them, so the Low logging classification still holds
@answer n=11 entity=pii-scrubbing-compliance#1.11 v=no
why: the stale-row witness asserts only list's SGR sequences, and the C0/C1 escaping of row text is product output the session did not change
@answer n=12 entity=pii-scrubbing-compliance#1.12 v=no
why: the link and unlink pair-name rules and their tests/cli_links.rs witness are untouched by the session
@answer n=13 entity=pii-scrubbing-compliance#1.13 v=no
why: protocol codes and invalid-params details are product protocol that the session did not change
@answer n=14 entity=pii-scrubbing-compliance#1.14 v=no
why: the session adds no recorded fixture, and fixtures/ did not move
@answer n=15 entity=pii-scrubbing-compliance#2.1 v=no
why: the scan, canary, upload and scan-failure rules live in .github/ and product tests the session did not move, and the new census reads only repository source text and writes no diagnostics
