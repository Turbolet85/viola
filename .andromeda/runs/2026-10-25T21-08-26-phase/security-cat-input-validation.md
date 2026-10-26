# security · Input Validation — category read

## Carried
- (.andromeda/security-plan-detail/input-validation.md:17) The chunk touches `viola-channel`'s `server.rs` / `server/win.rs` listener; every frame the hardened Windows listener accepts must still be read under the frame cap, closed on overflow with `-32600`, at serde_json's default depth, so the admission work leaves the framing floor unchanged.
  > **Size cap:** wrap the stream reader in `Read::take(MAX_FRAME)` before `read_line`; an over-limit frame gets `-32600` and the connection is closed. **Depth:** serde_json's default recursion limit of 128 (never `unbounded_depth`). **Version:** the `v` check (`-32602`).
- (.andromeda/security-plan.md:136) The body's broad line for the channel-frame boundary the listener serves: the cap, depth, `v` check and per-method `-32602` checks must hold for every client the Windows pipe admits.
  > Channel frames (IPC ndjson) — a `MAX_FRAME` cap before `read_line` (over it `-32600`, the connection closed), serde_json's default depth limit, never `unbounded_depth`, the `v` check, closed types for `behavior`, `dialog_id` and names, and each method's params checked by the wrapper (`-32602`) before `validate_paste_text`; an invalid `hook.dialog` is not registered, an invalid `hook.event` not appended.
- (.andromeda/security-plan.md:451) An input ban on the channel stream the chunk's listener hands out: no `read_line` on an admitted pipe stream without the cap.
  > NEVER call `read_line` on a channel stream or `read_to_end` on hook stdin or child output without `Read::take(MAX_FRAME)`.
- (.andromeda/security-plan.md:469) The request size limit on the channel stays in force on the Windows listener the chunk hardens.
  > NEVER skip request size limits: `MAX_FRAME` on the channel.

## Bindings
- **Channel frames ↔ body §Authentication & Authorization (IPC access control (Windows))**: the chunk's DACL, remote-client rejection and first-pipe-instance assertions decide who reaches the pipe; the framing floor governs what an admitted client's bytes may do — the admission work sits in front of the frame read and must not replace or bypass it (.andromeda/security-plan.md:108) · (.andromeda/security-plan-detail/input-validation.md:17)
- **Channel frames ↔ body §Security Anti-Patterns § Universal**: no `config.json` key, `VIOLA_*` variable or CLI flag may switch off the IPC DACL or the frame cap the listener enforces, so any listener option the chunk adds or asserts carries no such switch (.andromeda/security-plan.md:516) · (.andromeda/security-plan.md:469)
