
## 2026-09-28-capability-ledger-and-viola-verify — the hidden `hook --capture` arm and verify's probe session, founder-ratified exceptions
**Section:** Established Decisions [Hook Contract], [Plugin Scope]; §Occupied Resources Environment variables (`VIOLA_NAME`)
**Change:**
- [Hook Contract]: `hook` exits 0 at once when `VIOLA_NAME` is absent, except the hidden `hook <event> --capture <DIR>` arm (called only by `verify`'s probe plugin): no `VIOLA_*` read, no obs init, no channel; an absolute existing `<DIR>`, stdin through `take(MAX_FRAME + 1)`, a raw 0600 write to the first free `<DIR>/<PascalEvent>.<k>.json` (`k` across events); every failure writes nothing; exit 0 with empty streams always.
- [Plugin Scope]: unwrapped sessions carry no viola hooks, except `verify`'s transient `claude -p` probe, which loads only `viola-verify-probe` from `ledger/probes/<pid>/plugin/`, removed when `verify` ends.
- `VIOLA_NAME`'s absence makes every hook EVENT path a silent exit 0 (was "every hook").
**Why:** a boundary widening, ratified live by the founder on 2026-09-28 at 20:24:32 after the arm was shown (relay: the Viola overseer); recorded in security-plan's Decisions Log. No other caller may register the arm.
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/
