
## 2026-10-04-the-wheel — pause and release join the internal-error verbs
**Section:** §Surface: cli → Component — Primary content block 2 (refusal lines)
**Change:** the failed-or-panicked `cli` verbs: was `send`, `wait`, `last`, `answer`, `verify`; now also `pause` and `release` — each prints exactly `error: internal error` once from the one catch site, no hint.
**Why:** the chunk added the `viola pause` / `viola release` verbs, which exit 1 through the shared catch site on any unexpected reply.
**Ref:** .andromeda/runs/2026-10-04T20-44-01-wrap/
