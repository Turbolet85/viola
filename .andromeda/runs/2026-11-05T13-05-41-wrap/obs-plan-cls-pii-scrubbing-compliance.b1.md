@answer n=1 entity=pii-scrubbing-compliance#1.1 v=no
why: the secret-scan rules are unchanged, only the OS arm is removed, and the cookie scrub claim holds (untouched item 5)
@answer n=2 entity=pii-scrubbing-compliance#1.2 v=no
why: the R8 strip still logs names only and keeps the identity floor and canary test, and the removed case fold is a matching detail the row does not state
@answer n=3 entity=pii-scrubbing-compliance#1.3 v=no
why: the session changes no signing credential or secret handling, and the CI secret rule holds on the trimmed workflows
@answer n=4 entity=pii-scrubbing-compliance#1.4 v=no
why: the prompt and assistant-output handling (skip_all, veil Redact, text_bytes) is untouched (untouched item 5)
@answer n=5 entity=pii-scrubbing-compliance#1.5 v=no
why: the tool input and dialog redaction rules are untouched, and the dialog-answer scenario moved no OS arm (untouched items 5 and 6)
@answer n=6 entity=pii-scrubbing-compliance#1.6 v=no
why: the drift-report seam and its fixed Display are untouched by the removal of Windows and macOS code
@answer n=7 entity=pii-scrubbing-compliance#1.7 v=no
why: the statusline span fields are not touched by the session
@answer n=8 entity=pii-scrubbing-compliance#1.8 v=no
why: the PII classification is untouched (untouched item 5), and the Linux path rules (basenames, workspace-relative location, with_file(false)) are unchanged by the removal of Windows-only arms
@answer n=9 entity=pii-scrubbing-compliance#1.9 v=no
why: endpoint is still never logged and only endpoint_kind is, and the config and version fields are not touched
@answer n=10 entity=pii-scrubbing-compliance#1.10 v=no
why: typed structured fields and the rule against logging them inside detail codes are untouched by the session
@answer n=11 entity=pii-scrubbing-compliance#1.11 v=no
why: the agents JSON row handling and C0/C1 escaping are not touched by the session
@answer n=12 entity=pii-scrubbing-compliance#1.12 v=no
why: the link and unlink pair logging and its asserted test are not touched by the session
@answer n=13 entity=pii-scrubbing-compliance#1.13 v=no
why: the schema edits touch only the os, process-start and process-exit words, and the channel-response detail enum the row cites is unchanged
@answer n=14 entity=pii-scrubbing-compliance#1.14 v=no
why: the row's claim that obs adds no fixture content and fixtures are not a telemetry source still holds after the Windows and macOS fixture removal
@answer n=15 entity=pii-scrubbing-compliance#2.1 v=yes mode=contradict
why: the session retires the Windows owner/DACL and home-DACL strict-modes arm of viola_state::strict, so the row's Windows clauses on the diagnostics folder and file checks, and its Windows and macOS test clauses, no longer hold
