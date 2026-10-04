
## 2026-10-04-readiness-gate-and-timing-constants — where a vt100 panic is caught and what it writes
**Section:** §7 Error Capture & Reporting (Panic hooks → `run` worker threads; Error classes captured)
**Change:**
- Error classes captured: was "vt100 panics under `catch_unwind` in `run.readiness_gate` give `parse-rejected{parser:"vt100-feed"}` plus `send-refused{detail:"input-not-ready"}`, not a process panic"; now the panic is caught on `run`'s feed thread (`src/run/gate.rs`) by a `catch_unwind` around each feed and resize, each poisoning gives exactly one `parse-rejected{parser:"vt100-feed", detail:"panicked", count:1}` at `WARN` with no `corr`, the model stays poisoned until the host size changes and `run` continues; `viola_panic_hook` also writes its `event:"panic"` role line and detail line for the same poisoning (recorded, not witnessed at run level); `send-refused{detail:"input-not-ready"}` follows once confirmed `send` reads the verdict, and whether G2 counts the contained panic line is that chunk's question.
- `run` worker threads: the vt100 feed thread (detached) is a contained worker — a caught panic poisons the model and writes one `parse-rejected`, nothing goes to `main`; the "no `process-exit`" ban covers the PTY pump and the handle-wait thread.
**Why:** the readiness-gate chunk landed the feed thread with a per-call catch; the verdict's consumer and the G2 question belong to confirmed `send`.
**Ref:** .andromeda/runs/2026-10-04T05-25-03-wrap/
