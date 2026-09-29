
## 2026-09-29-sideloaded-conpty — the Windows zero-viola-bytes check names both ConPTY hosts
**Section:** §3 A11y Assertion Harness Contract → Keyboard test harness (Tooling); §6 Visual Design Verification → CLI equivalent
**Change:** was "ConPTY itself emits `ESC[?9001h ESC[?1004h ESC[?25l ESC[2J ESC[m ESC[H`, an OSC 0 title and `ESC[?25h` on every spawn" (the inbox host only); now the inbox host's bytes (fact 4) and the sideloaded `OpenConsole.exe`'s, `ESC[1t ESC[c ESC[?1004h ESC[?9001h` at spawn and `ESC[?1004l ESC[?9001l` at exit, as measured at the chunk's `evidence/da1-stall.md`. Its DA1 query is answered by the terminal (in the tests, the piped driver), never by viola. The Windows check — viola's own literals absent — runs on both backends: the default case on the sideload and `conpty_sideload`'s tampered case on the inbox fallback. §6 credits both hosts' SGR, cursor and query bytes.
**Why:** the plan asked for the measured preamble if the sideloaded one differed; it does, and the zero-viola-literals oracle holds on both. §1 stays verbatim.
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/
