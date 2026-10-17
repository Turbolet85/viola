
## 2026-10-17-cli-machine-contract — design: revive's --json documents in the cli surface
**Section:** §Surface: cli → Output structure per verb (`revive`); Exit-code phraseology (exit 1's `--json` cell); the hints-are-human-mode-only bullet (exit 1's document, revive's strict-modes refusal)
**Change:**
- `revive`: under `--json` (both arms) a refused strict-modes check or a preflight refusal prints one stdout document `{"v":1,"error":"<code>","detail":null}` (`strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing`), no hint, stderr empty; `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}` (was: "The verb takes no `--json`"). A post-preflight `run` start refusal prints no document (owed to "Exit-cause code catalogue").
- Exit 1's `--json` cell names revive's document; every other exit 1 has none (was: "no `--json` document until arch amends it"). The hints bullet: arch fixes exit 1's document for revive only; revive's strict-modes refusal is the stderr pair or, under `--json`, the `strict-modes-failed` document.
**Why:** the chunk landed revive's machine form; the plan's expected amendment, raised by the wrap where no detector reached it.
**Ref:** .andromeda/runs/2026-10-17T03-50-43-wrap/
