
## 2026-09-28-capability-ledger-and-viola-verify — `run`'s and `verify`'s stamps reads before strict-modes, a ratified interim gap until Epoch 6
**Section:** Authentication & Authorization `~/.viola/` access control (Strict-modes check); Input Validation rows CLI arguments / stdin (Home path) and Own state files on read (Integrity); Security Decisions Log (new `2026-09-28` stamps-read + capture-arm entry)
**Change:**
- A second interim gap beside `hook`'s: `viola run`'s version gate reads `ledger/stamps.json` through `read_stamps` (no lock, `take(MAX_FRAME + 1)`, over the cap an error; unreadable or malformed → one `parse-rejected{parser:"ledger-stamps"}` WARN and `cli_verified:false`), and `viola verify`'s `update_stamps` reads the current bytes for its locked read-modify-write, both without the home strict-modes check, until "Home and code-bearing file integrity" (Epoch 6) adds both to the entry-point set.
- The Decisions Log entry records both widenings of this chunk (this gap and the capture arm), their conditions and witness (ci#36460408121 on `6486276`).
**Why:** no process runs the strict-modes check yet (Epoch 6 owns it), and a bad stamps file only degrades `run` to unverified. The founder ratified `run`'s read live at 2026-09-28 12:39:40 and its extension to `verify`'s read at 20:24:32, each after it was shown (relay: the Viola overseer). A non-`null` dialog decision still needs strict-modes on `run`'s stamps read first (PREREQ on "Dialog answers by dialog_id").
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/
