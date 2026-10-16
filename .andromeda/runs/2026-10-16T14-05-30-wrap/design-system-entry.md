
## 2026-10-14-resume-and-compact-ledger-rows — the verify step list gains resume-session
**Section:** §Surface: cli, pattern 5 (`verify`)
**Change:**
- The step counter reads `[01/22]`. MM is 22 on Unix and 19 on Windows.
- The ledger order ends with the resume row, `resume-session --resume <id> reopens the session under that id`, relied on every OS.
- The summary reads `stamped 2.1.287  22 pass  0 fail`.
- Pattern 2's `[  ] unconfirmable … local command, no measured post-condition` covers `/compact` unchanged.
**Why:** The 22nd ledger row landed (`resume-session`), relied on every OS.
**Ref:** .andromeda/runs/2026-10-16T14-05-30-wrap/
