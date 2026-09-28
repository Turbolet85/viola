
## 2026-09-28-capability-ledger-and-viola-verify — the version gate placed and split by crate, `src/human.rs` shared with `verify`, raw panic frames' dependencies
**Section:** Established Decisions [Session Liveness], [Agent Coverage]; §Conventions CLI exit codes; §Infrastructure Patterns Build system (lint bullet), Crate dependency direction (viola-agent-claude, root bin), Project directory structure (`human.rs`)
**Change:**
- `run` start order as landed: program resolution → strip plan → collision → pinned copy and plugin → version gate (`run::version_gate`: a `version-probe` start/exit pair, the stamps read, `cli_version`/`cli_verified` into the first snapshot and the `claude-child` `process-start`, nothing printed) → bind → snapshot → heartbeat → start events → spawn (was "only the version gate has none").
- The version gate is split: Claude parsing, rows, stamp merge/verdict and scrub in the pure `viola-agent-claude` (no `viola-state` dependency); the spawn in root `run::version_gate`; stamps I/O in `viola_state::stamps` (was "the CLI version gate live[s] only in `viola-agent-claude`").
- Exit `1` also covers a `verify` failing row, verify's four `unable:`/`hint:` refusals plus run's pinned-copy refusal, and the `cli` role's exact `error: internal error`.
- `src/human.rs` adds `write_internal_error`/`internal_error` and the stdout `write_result`/`result`; its callers are `run`'s start refusals and `viola verify` (was "only caller is the `viola run` start refusals").
- The root bin also takes windows-sys `Win32_System_Diagnostics_Debug` + `Win32_System_LibraryLoader` and libc `=0.2.189` (Unix) for `src/panic_frames.rs`.
**Why:** the report's Symbols and Dependencies (both crates already in §Stack; `cargo deny check` green).
**Kept:** the 2026-09-28-cli-output-tokens entry's clap facts stand; only its "called only by `run`" clause is retired here, so it is not superseded whole.
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/
