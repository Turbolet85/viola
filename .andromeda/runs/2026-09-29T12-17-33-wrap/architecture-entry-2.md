
## 2026-09-29-sideloaded-conpty — the ConPTY sideload step in the start order
**Section:** Established Decisions → [Session Liveness] (both start-order sites); Infrastructure Patterns → CI/CD approach
**Change:**
- The prose order now reads collision check, pinned copy and plugin folder, the ConPTY sideload (Windows x64, fail-open), version gate, bind.
- The as-landed chain was "program resolution → strip plan → collision → pinned copy and plugin → version gate"; now "program resolution → collision → pinned copy and plugin → ConPTY sideload (`run.conpty_sideload`: `pin_companions` + the absolute-path pre-load; `outcome` `loaded` · `hash-mismatch` · `unreadable` · `load-failed` · `not-built`, never a refusal; held handles live until `spawn_child` returns) → strip plan → version gate". The strip plan's place was already stale before this chunk; it follows the pinned copy.
- CI/CD: the `test` job's `windows-2025`-only `ConPTY vendor verification` step (`conpty-vendor.sh --verify` then `--probe`) runs before the coverage run.
**Why:** the chunk added one start step between the pinned copy and the strip plan (report Symbols/APIs; the span-order test); the step never changes the exit.
**Ref:** .andromeda/runs/2026-09-29T12-17-33-wrap/
