
## 2026-09-29-fake-agent-drift-contract — Harness prefixes: four, the cross-session tag included
**Section:** §4 viola-agent-claude (`prompt-submitted` normalisation, M2); §6 Critical Path 5 (the harness-injected turn)
**Change:** The M2 prefixes and Path 5's harness-injected turn were `<agent-message from=` / `<task-notification>`; now they are four — `<agent-message from=`, `<task-notification>`, `<\cross-session-message`, `<cross-session-message` — on the raw start with no trim. The escaped cross-session form is the one escaped tag that classifies; the tag mid-prompt or after a leading space stays `human`.
**Why:** the founder-ratified widening of the harness class (security-plan Decisions Log `2026-09-29`); `prompt_origin_files_the_cross_session_tag_as_harness` witnesses it, red on the unchanged two-entry list.
**Ref:** .andromeda/runs/2026-09-29T14-38-17-wrap/
