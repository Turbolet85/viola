
## 2026-10-14-resume-and-compact-ledger-rows — the verify example block at 22 rows
**Section:** §Output structure — `viola verify` (the example block, the failing-row example, Usage)
**Change:**
- The Unix example block has 22 relied rows (`[NN/22]`). Its new last step line is `[22/22] resume-session --resume <id> reopens the session under that id  pass`, and its summary is `22 pass  0 fail`.
- The failing example reads `20 pass  2 fail`, summing to 22.
- Usage:
  - the ledger order ends `resume-session`;
  - MM is 22 on Unix and 19 on Windows;
  - was "18 on Windows, which stops at `paste-newline`"; now the Windows set skips the three statusline rows and ends at `resume-session`.
- Signature placement's generic local-command wording covers `/compact` unchanged.
**Why:** The 22nd ledger row landed, relied on every OS.
**Ref:** .andromeda/runs/2026-10-16T14-05-30-wrap/
