
## 2026-09-29-sideloaded-conpty — the H2 pair, the sideloaded backend and its boundary tests
**Section:** §5 Integration → Module ↔ PTY; Critical Path 1 (the `run` start sequence); §9 CI → Pipeline structure (Coverage report row), Build failure conditions
**Change:**
- Module ↔ PTY: beside the inbox 13 of 200, the with/without pair from one windows-2025 run (image `windows-2025-vs2026` 20260828.587, ci#36563868040): sideloaded 0 of 200, inbox 14 of 200, no rate. Windows x64 root `viola run` tests host the child on the sideloaded `OpenConsole.exe`; viola-pty's tests, `OuterPty` and the harness `supervise` stay inbox. The boundary: `conpty_sideload` (5 cases) and the two-sided `restrict_dll_search_keeps_planted_conpty_out_of_a_bare_name_load`. The piped driver answers DA1, and the forced-window resize test resizes on the wrapper's `process-start{claude-child}` line.
- Critical Path 1: the start sequence gains the ConPTY sideload after the pinned copy and plugin (fail-open).
- §9: on `windows-2025` the `ConPTY vendor verification` step runs before the coverage run, which covers the `conpty_sideload` binary; a verify or probe not ending its verdict fails the build.
**Why:** the chunk's measured acceptance and its new test surfaces (report Counts, Harness/gate surface, Coverage).
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/
