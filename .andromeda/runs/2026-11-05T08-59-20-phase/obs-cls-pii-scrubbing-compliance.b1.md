@answer n=1 entity=pii-scrubbing-compliance#1.1 v=yes
why: the Cookie header row is witnessed by the secret-scan test, which the chunk keeps on the remaining Linux jobs and re-measures as zero hits (task items 2 and 6)
@answer n=2 entity=pii-scrubbing-compliance#1.2 v=yes
why: the CLAUDE* canary row is witnessed by the same secret-scan test the chunk keeps and re-measures on the Linux jobs (task items 2 and 6)
@answer n=3 entity=pii-scrubbing-compliance#1.3 v=no
why: v1 signing credentials are not in the runtime, and the chunk adds no secret or upload; the kept uploads are already bound through entity 2.1
@answer n=4 entity=pii-scrubbing-compliance#1.4 v=no
why: prompt and assistant-output redaction sits in shared Linux code and log lines the chunk does not touch
@answer n=5 entity=pii-scrubbing-compliance#1.5 v=no
why: tool input and dialog redaction sits in shared Linux code and log lines the chunk does not touch
@answer n=6 entity=pii-scrubbing-compliance#1.6 v=no
why: the drift-report fixed Display is at the seam in shared Linux code the chunk does not touch
@answer n=7 entity=pii-scrubbing-compliance#1.7 v=no
why: the statusline shell-out is untouched by the OS removal (task item 13 not_needed)
@answer n=8 entity=pii-scrubbing-compliance#1.8 v=no
why: path-free home-level lines are Linux logging the chunk does not change, and no Windows path arm it removes is named in the scope
@answer n=9 entity=pii-scrubbing-compliance#1.9 v=no
why: the endpoint and pinned_bin logging rule is over config and snapshot logging the chunk does not change
@answer n=10 entity=pii-scrubbing-compliance#1.10 v=no
why: pid and budget fields are typed Linux log fields the chunk does not change (PTY seam unchanged, task item 12)
@answer n=11 entity=pii-scrubbing-compliance#1.11 v=no
why: agents --json row handling is untouched by the chunk, which names no parse or terminal-output path
@answer n=12 entity=pii-scrubbing-compliance#1.12 v=no
why: the link and unlink pair logging is untouched, and its witness test tests/cli_links.rs is outside the chunk's reach
@answer n=13 entity=pii-scrubbing-compliance#1.13 v=yes
why: the closed channel-response detail enum in schemas/diag-line.v1.json is a Linux value the chunk's schema edit must keep and re-measure (task item 11)
@answer n=14 entity=pii-scrubbing-compliance#1.14 v=no
why: the chunk only removes Windows fixtures and adds none, so nothing needs scrubbing
@answer n=15 entity=pii-scrubbing-compliance#2.1 v=yes
why: the scan-before-upload rules, the diag, harness and perf uploads and the hit report are named by the scope (task item 2), and the pre-push no-absolute-path check binds through the pre-push step removal (task item 4)
