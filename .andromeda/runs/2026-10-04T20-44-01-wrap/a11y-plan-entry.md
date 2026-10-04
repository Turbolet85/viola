
## 2026-10-04-the-wheel — the tui wheel boundary: terminal replies non-editing, the Windows platform fact
**Section:** §3 → Keyboard test harness (Tooling) · §1 → `viola run` TUI passthrough · §1 → Critical path 4 (tui) · §4 → P4 (tui) case (3) · §11 Anti-Patterns → Keyboard
**Change:**
- The non-editing set: was focus, mouse and resize; now the closed list of focus reports, mouse reports (X10, SGR, urxvt) and terminal replies (DA1, DA2, CPR, DECRPM, kitty flags, OSC, DCS), every other byte editing; the replies are proven by the classifier's unit table.
- Case (3) as landed: step-wise (the resize behind its size receipt, then the focus reports with a mouse report as their read barrier), the wheel probed by an `answer` to no pending dialog (exit 13 driver, 10 human). Linux and macOS: the full assertion (no `wheel` record, bytes unchanged). `windows-2025`: was "on all three OS legs … do not move the wheel"; now the platform fact — the inbox ConPTY swallows focus reports, and under win32-input-mode the injected mouse report arrives as win32 key-down records and takes the wheel.
- The Keyboard ban: a mouse report the Windows ConPTY already turned into key-down records is typing.
**Why:** the founder's live rulings F-W2 and F-W3 ("pin the platform fact", the error only ever favouring the human), 2026-10-04, relayed by the overseer; a real Windows terminal's mouse report is measured live at route `:82`.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/
