@answer n=1 entity=pii-scrubbing-compliance#1.1 v=yes
why: the rule is witnessed by the secret-scan test over diagnostics/, the scan the chunk keeps on the Linux jobs and reads as its error budget
@answer n=2 entity=pii-scrubbing-compliance#1.2 v=yes
why: names the secret-scan test with per-name canaries as its witness, the scan the chunk keeps and re-measures on Linux
@answer n=3 entity=pii-scrubbing-compliance#1.3 v=no
why: CI secrets never echoed into artifacts is a re-check every CI change owes, and the chunk adds no secret or artifact echo
@answer n=4 entity=pii-scrubbing-compliance#1.4 v=no
why: prompt and assistant text redaction and skip_all sit in code the chunk does not edit and no OS gate touches
@answer n=5 entity=pii-scrubbing-compliance#1.5 v=no
why: tool input and dialog redaction on HookDialogParams and AnswerParams is untouched by the OS removal
@answer n=6 entity=pii-scrubbing-compliance#1.6 v=no
why: the serde_path_to_error fixed Display at the seam is not OS-gated and the chunk does not edit it
@answer n=7 entity=pii-scrubbing-compliance#1.7 v=no
why: the statusline shell-out span is not touched by the OS removal, as the task states
@answer n=8 entity=pii-scrubbing-compliance#1.8 v=no
why: path-free home-level log fields are shared Linux code the chunk leaves alone, and a test panic message is not an obs line
@answer n=9 entity=pii-scrubbing-compliance#1.9 v=no
why: config thresholds and endpoint non-logging are not OS-gated and the chunk changes no logged config value
@answer n=10 entity=pii-scrubbing-compliance#1.10 v=no
why: typed pid and budget fields are logged outside detail codes, a placement the chunk does not change
@answer n=11 entity=pii-scrubbing-compliance#1.11 v=no
why: claude agents rows and C0 and C1 escaping are not OS-gated and the chunk does not touch that path
@answer n=12 entity=pii-scrubbing-compliance#1.12 v=no
why: link and unlink pair names in the wrapper event lines are outside the chunk's scope and not OS-gated
@answer n=13 entity=pii-scrubbing-compliance#1.13 v=yes
why: the closed channel-response detail enum in schemas/diag-line.v1.json is a Linux value set the chunk must keep, named by scope item 11
@answer n=14 entity=pii-scrubbing-compliance#1.14 v=no
why: scrub rules on recorded fixtures are a re-check every change owes, and the chunk adds no fixture
@answer n=15 entity=pii-scrubbing-compliance#2.1 v=yes
why: its secret-scan-gated diag, harness and junit uploads, the scan-failure hit report and the fuzz and perf uploads are named by scope items 2, 3 and 5
