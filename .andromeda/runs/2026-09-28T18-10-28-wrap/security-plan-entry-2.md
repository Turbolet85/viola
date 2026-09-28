
## 2026-09-28-capability-ledger-and-viola-verify — the hidden `hook --capture` arm, verify's probe captures and the fixture scrub
**Section:** Input Validation rows Hook stdin (capture arm) and CLI arguments / stdin (hook sentence); Error Handling (hook bullet); Data Protection At rest (Probe captures) and Repository fixtures
**Change:**
- `hook <event> --capture <DIR>` (only `verify`'s probe plugin calls it): an absolute, existing `<DIR>`; stdin through `take(MAX_FRAME + 1)`, written raw and unparsed via `replace_private` (0600) to the first free `<DIR>/<PascalEvent>.<k>.json`, `k` over `1..=n+1`; no `VIOLA_*` read, no obs init, no channel; every failure writes nothing; exit 0 with empty stdout and stderr. The hook's `VIOLA_NAME`/`VIOLA_DIR` shape checks and one-object parse hold for its event path only.
- Probe captures are transient content-bearing 0600 files under the 0700 `ledger/probes/<pid>/`, removed whole by a drop guard on every exit path of `verify`.
- `verify --record` scrubs every string and key (home → `~`, both separator spellings, case-folded on Windows; the user word → `<user>`) and refuses the whole recording, nothing written, when a drive path, `/home/`, `/Users/`, `\Users\` or the user word survives.
**Why:** a boundary widening (a new `hook` crossing: a raw payload written to an argv-named directory), ratified live by the founder at 2026-09-28 20:24:32 after it was shown (relay: the Viola overseer). It avoids a new channel method or an env-var switch, runs as the same user through the pinned exe, and no other caller may register it.
**Ref:** .andromeda/runs/2026-09-28T18-10-28-wrap/
