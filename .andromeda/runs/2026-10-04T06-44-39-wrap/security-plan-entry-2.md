## 2026-10-04-confirmed-send-with-cl-1-records — the fourth dated gap: viola send before server verification (F3)
**Section:** §Authentication & Authorization (IPC client-side server verification; `~/.viola/` access control) · §Input Validation (CLI arguments / stdin; Own state files on read) · §Security Anti-Patterns → Authentication
**Change:**
- A fourth dated interim gap, until the Epoch 6 entries "Server verification before any frame" and "Home and code-bearing file integrity" (`:109` / `:111` remove it): CLI `viola send` writes its `send` frame after a liveness-only pre-check — the snapshot's pid + start time alive, the heartbeat live, an `endpoint` present, else exit 21 — and checks neither the serving process's identity nor the strict-modes of the snapshot it reads. The residual is a same-user process squatting a stale endpoint name; the pipe DACL and the 0700 socket directory still apply. It does not borrow the `hook.event` exception.
- The server-verification ban had one dated exception (`hook.event`); now two, and no other frame or process may borrow either.
- `send` is served by the wrapper (it answered `-32601` before this chunk), under the channel row's controls and this gap.
**Why:** a boundary widening, shown at P4 and held; ratified by the founder live on 2026-10-04, relayed by the overseer.
**Ref:** .andromeda/runs/2026-10-04T06-44-39-wrap/
