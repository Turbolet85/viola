
## 2026-10-04-wait-and-last — wait and last as built; the cli role's one internal-error printer
**Section:** Conventions → CLI exit codes (exit `1`) · Standard Contracts → Channel methods (`from`, `wait`, `last`) · Established Decisions [Database / State Store] · [Message Broker / IPC] · Occupied Resources → `diagnostics/` · Infrastructure Patterns → build-system · project-directory-structure
**Change:**
- Exit `1`: was "a failed or panicked `verify` prints `error: internal error`; `send` prints it from its dispatch arm and a `send` panic prints nothing (only `verify` is `cli`)"; now `role_of` files every first word but `run`, `hook` and a leading `-` flag under `cli` (`send`, `wait`, `last`, `verify`) and the `main` catch site prints the line once for a dispatch `Err` or a caught panic; wait/last exit 1 covers a refusal reply, an unknown reply shape and any channel error but a failed connect or a closed/reset connection.
- `wait` was `{after?, timeout_ms?}`; now `{after?, timeout_ms?, from?}`, `after`/`timeout_ms` absent or `u64` (else `-32602`), start offset `after` or the log end at the call, line-starts rule, `EventKind::WAIT_WAKE`, `checked_add` deadline, and the `WaitFeed` wake: one Mutex + Condvar, the hook line's append inside the lock, only hook lines signal, 20 ms clock re-reads that never re-scan. `last` was `{}`; now `{from?}`, null/null before a turn, the newest turn updated under the append's lock and rebuilt before `server.serve`.
- The envelope types no `from`: each method answers a mistyped one `-32602`, never `-32600`.
- As landed, the one events reader skips (never heals) a torn last line, healing owed to `:85`; an `after`-less `wait` starts at the log end, the pending-dialog return owed to `:78`.
- `cli-<name>.ndjson` gains send/wait/last as producers; the tree gains `cmd/client.rs` and `run/wait.rs`, `human.rs`'s callers gain send/wait/last and the catch site.
**Why:** the chunk built `wait` / `last` and moved the `cli` line to one printer (CARRY 2); the in-lock feed update is the operator pass's fold of a macOS red where `last` read the turn before one already on disk.
**Ref:** .andromeda/runs/2026-10-04T10-29-04-wrap/
