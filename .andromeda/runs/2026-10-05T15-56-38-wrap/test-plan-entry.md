
## 2026-10-05-permission-end-to-end — Path 4's permission kind end to end; the dialog wake witness complete
**Section:** §6 Path 3 → Surfaces involved; §6 Path 4 → Surfaces involved; §6 Path 4 → Steps (step 1); §7 Test Data & Fixtures (the fake-script hygiene walk)
**Change:**
- Path 4 Surfaces: was "the `permission` kind is unit / insta only (`dialog::tests`), its end-to-end case owed to 'Permission end to end'"; now the `permission` kind runs end to end over its own gated script `fixtures/fake-scripts/path4-permission.json` (the recorded `PermissionRequest.permission-1.json` / `PostToolUse.permission-1.json` under `fixtures/claude/2.1.287/`): `path4_permission_is_logged_once_woken_and_answered_by_id` (`allow` with no suggestion reaching the CLI, `v1-16`; `deny` + `message`; the PostToolUse `activity` line waking no `wait`; a `question` first raised by PermissionRequest logged once, its hook silent, a late `answer` exit 13 `unknown-dialog`) and `path4_unstamped_permission_is_left_to_the_human_and_its_allow_refused` (exit 12, no body).
- Path 3 Surfaces: was "the `permission` wake stays unit-level, its end-to-end witness owed to 'Permission end to end'"; now the `permission` wake witness is end to end in `path4_permission_…`.
- Path 4 Steps: the prompt-2 permission step is, as landed, its own five-step gated script, not prompt 2 of `path4.json`.
- §7: the fake-script walk's committed set is `{gated-turn,path3,path4,path4-permission}.json`.
**Why:** the chunk landed both cases green on all three CI OSes; a separate gated script keeps `path4_dialogs_…`'s exact dialog and `activity` counts uncoupled.
**Ref:** .andromeda/runs/2026-10-05T15-56-38-wrap/
