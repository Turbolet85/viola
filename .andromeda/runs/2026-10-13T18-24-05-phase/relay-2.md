P4 fork round, chunk 2026-10-13-interrupted-verify-cleanup — the questions as shown, the options in order (the recommendation marked), and the operator's answers verbatim.

Q1 (Outcome): When Ctrl-C or TERM stops `viola verify` mid-run, what should it exit with, print and log? Nothing in architecture, design-system or layout-templates covers this today.
  1. Exit 1 + pair + detail (Recommended) — exit 1; stderr `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI` (planning form on the operator's word, the founder's wording owed); process-exit{subject:self, exit_code:1, detail:"interrupted"}, a twelfth closed schema member and an obs Decisions Log entry at the wrap.
  2. Exit 1 + pair, no detail.
  3. Exit 1, silent.
  4. New exit 130 (an architecture §Conventions amendment the founder rules).
Answer: "Exit 1 + pair + detail (Recommended)" — notes: "The operator: the pair's wording is a planning form, the founder's wording owed, as with the board forms."

Q2 (Win TERM): Windows has no SIGTERM. Which console events should end `viola verify` cleanly, removing what the run made?
  1. Ctrl-C and Ctrl-Break (Recommended) — CTRL_C_EVENT and CTRL_BREAK_EVENT set the stop; CTRL_CLOSE, logoff and shutdown keep the OS default (leftovers by hand, like a kill).
  2. Also hold CTRL_CLOSE.
Answer: "Ctrl-C and Ctrl-Break (Recommended)"

Q3 (Win proof): How should the Windows stop path be proven? The dev host is Linux, so its only Windows reading is a check-only clippy.
  1. Direct handler call (Recommended) — a cfg(windows) unit case on windows-2025 calls the handler routine directly and reads the flag and the return; event delivery on Windows named as not claimed.
  2. Also deliver CTRL_BREAK on CI.
Answer: "Direct handler call (Recommended)"
