
## 2026-10-19-newer-claude-cli-rows — the 2.1.302 fixture set and the long-turn screen switch
**Section:** Occupied Resources (Binary, subcommands and exit codes — the test-only binaries; Repository — `fixtures/claude/<cli-version>/`).
**Change:**
- The fake agent has thirteen argv options (the count's rule is the list), nine of them serving verify's interactive runs and their tests. It was twelve and eight. The new one is the test-only `--long-turn-screen <phase>`: with `--framing` and `--turn-stop`, after the long paste's turn alone and after its Stop, `Screen.<phase>` of the set is drawn in place of `Screen.turn`. It sits beside `--tag-turn-screen`.
- The committed sets are `2.1.283`, `2.1.287`, `2.1.288` and `2.1.302`; `2.1.302` is new. `2.1.302` holds 4 spine payloads + 3 screens + 12 dialog variants + 6 framing variants + 2 status line variants + 1 resume variant + 1 agents variant: 29 files, the count's rule `ls fixtures/claude/2.1.302 | wc -l`. It was recorded whole by one `viola verify --record` round. Its `Screen.ready` and `Screen.turn` keep one row holding both input-box literals. It is stamped at the relied set the same way as `2.1.287`; `2.1.288` and `2.1.283` stay drift-only.
**Why:** The fixtures for the measured literal: CI's fake agent replays a recorded 2.1.302 set, and `contract_ledger_probes` stamps it (its `STAMPED` list). The screen switch reproduces the stop that held Run B after the long paste's turn without a live start.
**Ref:** .andromeda/runs/2026-10-20T05-48-20-wrap/
