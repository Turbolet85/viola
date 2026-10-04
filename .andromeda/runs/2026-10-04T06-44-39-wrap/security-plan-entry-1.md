## 2026-10-04-confirmed-send-with-cl-1-records — send's input validation as landed; the feed bound closed
**Section:** §Input Validation (rows: Paste text · Channel frames · CLI arguments / stdin · PTY output bytes (vt100); Constants)
**Change:**
- Paste text: `validate_paste_text(&str)` was `-> Result<(), CoreError>`; now `-> Result<(), NotDelivered>`. It runs in `viola send` (exit 13 before any frame, a client `send-refused` line) and again first in the wrapper's `send`; the paste is one bracketed `write_all` through `viola-pty`'s `PasteHandle` (was "the `run` pump's paste writer").
- Channel frames: `send` params — `text` a string, `from` absent, `null` or a `ViolaName` — else `-32602` `"invalid params"`, `data: null`, before the paste check.
- CLI arguments / stdin and Constants: `viola send`'s stdin / `--file` text joins the `MAX_FRAME` consumers (`take(MAX_FRAME + 1)`; over the cap or non-UTF-8 → exit 2; a second positional is a usage error).
- PTY output bytes: was "Open: the tee → feed queue is an unbounded `std::sync::mpsc`, owed a bound"; now `sync_channel(FEED_CAPACITY)`, 256 messages (≤ 2 MiB), `try_send` after the human's write, a dropped copy poisoning the model (one `parse-rejected{vt100-feed, oversize}` per episode) until a size change; the model behind a mutex shared with the gate, the catch inside the lock.
**Why:** confirmed `send` is the bound's owner (the readiness-gate chunk's routing) and the paste surface's first consumer. Not a widening: every check the rows mandate is present.
**Ref:** .andromeda/runs/2026-10-04T06-44-39-wrap/
