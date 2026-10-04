
## 2026-10-04-the-wheel — viola's own Windows console stdin reader
**Section:** Established Decisions → [PTY] · §Stack → PTY layer
**Change:** the host stdin handed to the pump is `viola_pty::host_stdin()`: on Windows with a console stdin, viola's own `ReadConsoleW` reader (no Ctrl-Z wakeup control; UTF-16 → UTF-8, a split surrogate carried, every `0x1A` kept, a 0-unit read read again); elsewhere, and for a redirected Windows stdin, `std::io::stdin()`. Callers: `run`'s pump and the fake agent. windows-sys's roles add the console input read.
**Why:** std's console stdin drops a read's trailing `0x1A` and ends input on a lone `^Z` — source-read at the pinned toolchain and measured on `windows-2025`, where the first red was the fake agent's own std read, not viola's.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/
