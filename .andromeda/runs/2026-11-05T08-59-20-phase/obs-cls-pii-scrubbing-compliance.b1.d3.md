@answer n=1 entity=pii-scrubbing-compliance#1.1 v=yes
why: the Cookie header secret-scan test is the Linux scan step the scope keeps and re-measures (zero secret-scan hits)
@answer n=2 entity=pii-scrubbing-compliance#1.2 v=yes
why: names the secret-scan test and its identity-floor canaries, the check the kept Linux scan step runs; the R8 strip itself is untouched
@answer n=3 entity=pii-scrubbing-compliance#1.3 v=no
why: v1.x signing credentials are not in the v1 runtime, and the chunk adds no CI secret or signing step
@answer n=4 entity=pii-scrubbing-compliance#1.4 v=no
why: prompt and assistant-text redaction sits on SendParams, LastResult and HookEventParams, Linux code the chunk does not reach
@answer n=5 entity=pii-scrubbing-compliance#1.5 v=no
why: tool-input and dialog redaction is Linux runtime code the chunk does not reach or change
@answer n=6 entity=pii-scrubbing-compliance#1.6 v=no
why: the fixed Display of drift reports at the seam is Linux runtime code the chunk does not reach
@answer n=7 entity=pii-scrubbing-compliance#1.7 v=no
why: the statusline shell-out is untouched by the OS removal, per the scope's not_needed item 13
@answer n=8 entity=pii-scrubbing-compliance#1.8 v=no
why: path-free home-level lines are Linux logging the chunk does not change, and no named Windows path arm writes them
@answer n=9 entity=pii-scrubbing-compliance#1.9 v=no
why: config and endpoint logging fields are Linux logging the removal leaves unchanged
@answer n=10 entity=pii-scrubbing-compliance#1.10 v=no
why: typed structured fields are Linux logging the chunk leaves as it is
@answer n=11 entity=pii-scrubbing-compliance#1.11 v=no
why: claude agents --json row handling is Linux code the chunk does not reach
@answer n=12 entity=pii-scrubbing-compliance#1.12 v=no
why: link and unlink logging is Linux code the chunk does not reach, and its asserting test is not re-measured by this chunk
@answer n=13 entity=pii-scrubbing-compliance#1.13 v=yes
why: scope item 11 keeps the closed channel-response detail values of schemas/diag-line.v1.json, and this rule states those values
@answer n=14 entity=pii-scrubbing-compliance#1.14 v=no
why: the chunk removes Windows fixtures and adds none, so the fixture scrub rule is a re-check every change owes
@answer n=15 entity=pii-scrubbing-compliance#2.1 v=yes
why: scope items 2, 3, 5 and 6 name the secret-scan gating of the Linux diag, harness, junit, perf and fuzz uploads and the scan-failure hit report, so the S2 upload convention is scope-named here
