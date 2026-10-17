
## 2026-10-17-end-to-end-path-binary-and-harness-legs — arch: the harness session guard reads the keep variables; bin/agents-mode
**Section:** §Occupied Resources → Environment variables (`AGENT_RUN_KEEP_HOMES`, `AGENT_RUN_KEEP_FAILED`); §Occupied Resources → Filesystem (`target/agent-run/<session>/`)
**Change:**
- `AGENT_RUN_KEEP_HOMES` and `AGENT_RUN_KEEP_FAILED` gain a reader: viola-e2e's `harness::session` guard, whose `Drop` (after its `cleanup`) keeps the home under `AGENT_RUN_KEEP_HOMES=1`, or `AGENT_RUN_KEEP_FAILED=1` while the thread is panicking. No variable is new.
- `target/agent-run/<session>/` holds `bin/agents-mode` beside the session's `bin/claude[.exe]` copy: `boot --agents-mode`'s value (0600), a stale one removed before the stamp and the new one written after it, before `supervise.json`; `session.json` and `supervise.json` carry `agents_mode` (serde default).
**Why:** The registry lists each variable's readers and each harness-written file, so the new reader and file are named where they stand.
**Ref:** .andromeda/runs/2026-10-17T01-57-16-wrap/
