
## 2026-10-17-cli-output-discipline — the refusal writer's callers named by verb
**Section:** §3 Observability Harness Contract (intro, the `run` diagnostic-output bullet); §11 Obs Anti-Patterns → Logs
**Change:**
- §3 intro: was "the root bin's only human-stderr writer for a start refusal, called by `run` and by `revive`"; now `refuse` → `write_refusal` is the root bin's human-stderr refusal writer, called by `run` (5 sites), `verify` (5), `list` (2) and `revive` (1), re-derived by `grep -rcE 'human::refuse\(' src`; the `list` colour layer adds no stderr writer and its `unable` / `hint:` pair carries no SGR.
- §11 Logs: the refusals routed through `refuse` / `write_refusal` add `verify`'s and `list`'s to `run`'s, `revive`'s and `revive --list`'s.
**Why:** a carried correction (the 2026-10-09-epoch-3-cleanup-ii wrap): the two sentences named the writer's callers too narrowly; both now name the set by verb with the rule that counts it.
**Ref:** .andromeda/runs/2026-10-17T08-28-14-wrap/
