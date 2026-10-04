
## 2026-10-04-wait-and-last — the fifth dated gap: CLI wait / last after the liveness-only pre-check
**Section:** Authentication & Authorization → IPC client-side server verification · `~/.viola/` access control · Input Validation → Channel frames · CLI arguments / stdin · Own state files on read · Security Anti-Patterns → Authentication
**Change:**
- CLI `viola wait` / `viola last` write their `wait` / `last` frame after `send`'s liveness-only pre-check (snapshot pid + start time alive, heartbeat live, `endpoint` present, else exit 21 `during:"connect"`), with no server identity and no strict-modes, until the Epoch 6 entries `:109` / `:111`; they borrow neither the `hook.event` nor `send`'s exception; their snapshot reads join the home and own-state-file interim lists; the NEVER-write rule's dated exceptions now name three frames; served-as: `wait` / `last` from this chunk.
- Channel frames: the `wait` / `last` params clause (`after`/`timeout_ms` absent or `u64`, `from` the `send` rule, unknown fields ignored, else `-32602` `data: null`); the envelope types no method param.
- CLI arguments: `<name>` through `ViolaName::try_new`, `--after` / `--timeout-ms` clap `u64`.
- Own state files: the events reader `read_from` takes each line through `take(MAX_FRAME + 1)`, skips and counts over-long and non-object lines, never returns or heals an unterminated last line.
**Why:** the founder's ruling, live, 2026-10-04 (F1 at this chunk's P4, given after the widening was shown), relayed by the Viola overseer — the boundary-widening escalation resolves on it. The params and reader clauses record validation the chunk shipped; a mistyped `from` was `-32600` from the envelope until this chunk.
**Ref:** .andromeda/runs/2026-10-04T10-29-04-wrap/
