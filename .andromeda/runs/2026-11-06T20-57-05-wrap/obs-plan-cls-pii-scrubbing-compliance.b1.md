@answer n=1 entity=pii-scrubbing-compliance#1.1 v=no
why: the Cookie-header secret-scan test sits in secret_scan.rs, a reached file, but task item 2 records the PII scrubbing rules unchanged and every gate is green on 849aad18, so the scan claim holds.
@answer n=2 entity=pii-scrubbing-compliance#1.2 v=no
why: the R8 names-only env-strip logging and the CLAUDE* canary scan are PII rules left unchanged (task item 2), and no env var was added (task item 1).
@answer n=3 entity=pii-scrubbing-compliance#1.3 v=no
why: v1 signing credentials are absent from the runtime, and the CI section 9 re-read moved no rule (task item 13), so the no-echo claim holds.
@answer n=4 entity=pii-scrubbing-compliance#1.4 v=no
why: the prompt-text redact rule is an unchanged PII rule (task item 2), and the Cargo.lock moves (portable-pty, nix, serial family, pin-utils) do not include the veil 0.3.0 pin.
@answer n=5 entity=pii-scrubbing-compliance#1.5 v=no
why: the dialog and tool-input redact rule is an unchanged PII rule (task item 2), and the session adds no logged field or event kind (task items 1 and 5).
@answer n=6 entity=pii-scrubbing-compliance#1.6 v=no
why: the fixed-Display-at-the-seam and detail-file-only rule for drift reports is unchanged (task item 2), serde_path_to_error is not among the Cargo.lock moves, and the one new drift contract test adds no home-level line.
@answer n=7 entity=pii-scrubbing-compliance#1.7 v=no
why: the statusline_command rule is an unchanged PII rule (task item 2), and the statusline-rate-limits finding (task item 9) concerns the account plan read, not the shell-out span fields.
@answer n=8 entity=pii-scrubbing-compliance#1.8 v=no
why: the path rules (basename file fields, workspace-relative panic_location, path-free home-level lines) are unchanged (task item 2), and the retired Git Bash rewritten-path warning was a stderr human-output line, not a log field or detail-file rule.
@answer n=9 entity=pii-scrubbing-compliance#1.9 v=no
why: config-value logging and the never-logged endpoint and pinned_bin rule are unchanged, and no field, flag or env var was added (task item 1).
@answer n=10 entity=pii-scrubbing-compliance#1.10 v=no
why: pid, child_pid, agent_session_id, started_at, budget and resets_at stay typed structured fields, since the pty_backend stays openpty and the re-read moved no value (task items 4 and 9).
@answer n=11 entity=pii-scrubbing-compliance#1.11 v=no
why: the claude agents --json row rule and its parse-rejected counts are unchanged, and no verb or flag was added (task item 1).
@answer n=12 entity=pii-scrubbing-compliance#1.12 v=no
why: the link and unlink pair-name rule and its cli_links test are outside the change, and the session added no verb (task item 1).
@answer n=13 entity=pii-scrubbing-compliance#1.13 v=no
why: the closed protocol enums and the channel-response detail codes are unchanged, with no detail code added or removed (task item 5).
@answer n=14 entity=pii-scrubbing-compliance#1.14 v=no
why: recorded fixtures stay tests-owned and obs adds no fixture content, and the reached mcp_recorded_exchange.rs carries no telemetry rule the session moved (task item 2).
@answer n=15 entity=pii-scrubbing-compliance#2.1 v=no
why: its per-os artifact names (diag, diag-perf, perf, secret-scan, harness) are templates the ubuntu-latest-only CI re-read confirms with no rule moved (task item 13), and its scan and witness tests in reached files stay green on 849aad18 with the PII rules unchanged (task items 2 and 13).
