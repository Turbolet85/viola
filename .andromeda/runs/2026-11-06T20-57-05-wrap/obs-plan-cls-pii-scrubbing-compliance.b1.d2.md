@answer n=1 entity=pii-scrubbing-compliance#1.1 v=no
why: crates/viola-e2e/src/harness/secret_scan.rs is reached but the Cookie-header secret-scan claim still holds on it since the session leaves the PII scrubbing rules unchanged (untouched item 2)
@answer n=2 entity=pii-scrubbing-compliance#1.2 v=no
why: the R8 strip names-only rule and its canary secret-scan still hold on the reached files since the session leaves the PII scrubbing rules unchanged (untouched item 2)
@answer n=3 entity=pii-scrubbing-compliance#1.3 v=no
why: no v1.x signing credential or CI secret echo moved, and the CI matrix re-read moved no rule (touches item 13)
@answer n=4 entity=pii-scrubbing-compliance#1.4 v=no
why: send.rs and hook.rs are reached but the instrument and redact rules still hold, the send change only drops the Git Bash rewritten-path stderr line, and veil 0.3.0 is not among the crates the session moved in Cargo.lock
@answer n=5 entity=pii-scrubbing-compliance#1.5 v=no
why: the new dialog-drift contract test adds coverage and moves no dialog-logging rule, the codes-only dialog events are unchanged (untouched item 5)
@answer n=6 entity=pii-scrubbing-compliance#1.6 v=no
why: contract_fake_agent_drift.rs is reached but the fixed Display at the seam and the detail-only drift_report still hold since the session moves no drift-report rule (untouched items 2 and 5)
@answer n=7 entity=pii-scrubbing-compliance#1.7 v=no
why: the statusline-rate-limits finding changes a ledger reading and not the statusline.shell_out span fields shell_exit_status and duration_ms, which the session leaves unchanged (untouched item 5)
@answer n=8 entity=pii-scrubbing-compliance#1.8 v=no
why: removing the Git Bash rewritten-path warning from send only drops a path line, so the basename, workspace-relative and path-free log rules still hold (untouched item 2)
@answer n=9 entity=pii-scrubbing-compliance#1.9 v=no
why: version_gate.rs is reached but the cli_version and threshold logging and the never-logged endpoint and pinned_bin rule still hold (untouched item 2)
@answer n=10 entity=pii-scrubbing-compliance#1.10 v=no
why: the logging of pid, child_pid, agent_session_id, started_at, budget percentages and resets_at is unchanged, the budget finding alters a ledger reading and no log field (untouched item 1)
@answer n=11 entity=pii-scrubbing-compliance#1.11 v=no
why: claude agents --json rows stay unlogged with parse-rejected counts only, and the live round on claude 2.1.287 moved no parse rule
@answer n=12 entity=pii-scrubbing-compliance#1.12 v=no
why: events.rs is reached but the closed event enum is unchanged with no event kind added or removed, so the link and unlink names stay out of process-log lines (untouched item 5)
@answer n=13 entity=pii-scrubbing-compliance#1.13 v=no
why: the closed kebab-case protocol enums and the schemas/diag-line.v1.json enum are unchanged with no detail code added or removed (untouched item 5)
@answer n=14 entity=pii-scrubbing-compliance#1.14 v=no
why: recorded fixtures are tests-owned and the session adds no obs content or product surface to them (untouched item 1)
@answer n=15 entity=pii-scrubbing-compliance#2.1 v=no
why: the verification rules still hold on the reached mcp_tools.rs, strict.rs, secret_scan.rs and fake-agent files, the CI diag-os artifact naming is a re-read that moved no rule (touches item 13), and no PROVISIONAL mark is named in this row
