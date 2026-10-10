
## 2026-10-10-statusline-pass-through — the statusline-bearing start's zero-viola-bytes reading, and whom the command's 5 s bound falls on
**Section:** §3 → Keyboard test harness (Tooling) · §8 Cognitive Accessibility (Timeout extensions, CLI)
**Change:**
- Keyboard test harness: a statusline-bearing start has its own reading of the zero-viola-bytes clause. `passthrough_with_a_statusline_source_adds_no_viola_bytes` in `tests/tui_passthrough.rs` asserts that a start from a home holding a statusline source adds none of viola's own literals to the outer-PTY stream, green on the three CI runners. On Unix that start writes the settings override and passes `--settings`; on Windows it writes none. `viola hook statusline` hands the user's status line output to the child byte for byte and writes nothing to the terminal itself.
- Timeout extensions: `viola hook statusline` runs the user's status line command under `STATUSLINE_DEADLINE` (5 s, PROVISIONAL) and exits 0 with empty stderr in every case. The bound falls on the user's own command, never on a keystroke and never on the human; no key is blocked, refused or delayed, and the wheel is untouched.
**Why:** the chunk added a start variant and a time bound, and the plan lists every bound viola holds with whom it falls on.
**Ref:** .andromeda/runs/2026-10-10T19-55-42-wrap/
