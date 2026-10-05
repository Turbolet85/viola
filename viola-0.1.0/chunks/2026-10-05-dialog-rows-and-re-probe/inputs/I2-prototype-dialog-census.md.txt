# Prototype dialog census — structure only (2026-10-05, phase P3 of 2026-10-05-dialog-rows-and-re-probe)

**Source:** the viola-lab prototype's per-session logs, `~/.viola/sessions/*/events.ndjson` (6 sessions:
andromeda-worker, conductor-builder, pulse-builder, viola-builder, viola-live, viola-live2), interactive `claude`
2.1.287 (the same source `fixtures/claude/2.1.287/RELAYED.md` names). The logs hold private content of other
repositories, so they are never copied: only event names, tool names, key sets, decision-body SHAPES and event order
were read, by two scratch scripts that print no prompt, plan, question, answer or path text.

## Readings
- Hook events: PreToolUse AskUserQuestion 24 · PreToolUse ExitPlanMode 5 · PermissionRequest AskUserQuestion 3 ·
  PermissionRequest ExitPlanMode 1 · UserPromptSubmit 403 · Stop 315 · SessionStart 29 · SessionEnd 25 ·
  Notification 62. No PostToolUse was registered by the prototype, and no ordinary-tool PermissionRequest appears.
- Dialog-tool PreToolUse `permission_mode`: AskUserQuestion `auto` (24 of 24), ExitPlanMode `plan` (5 of 5).
- Per dialog-tool PreToolUse, (decision body emitted by the prototype, a PermissionRequest for the same tool followed
  before the next Stop / UserPromptSubmit / PreToolUse):
  - AskUserQuestion: body emitted → no PermissionRequest followed: **21 of 21**; no body → PermissionRequest followed:
    **3 of 3**.
  - ExitPlanMode: body emitted → no PermissionRequest followed: **4 of 4**; no body → PermissionRequest followed:
    **1 of 1**.
- The PreToolUse bodies emitted (shapes): AskUserQuestion `{"hookSpecificOutput":{"hookEventName":"PreToolUse",
  "permissionDecision":"allow","permissionDecisionReason":str,"updatedInput":{"questions","answers","annotations"}}}`
  20×, and the same without `annotations` 1×; ExitPlanMode `{… "permissionDecision":"allow",
  "permissionDecisionReason":str,"updatedInput":{"plan","planFilePath"}}` 4×.
- The 4 PermissionRequests were each answered by the prototype with `{"behavior":"deny","message":str}`:
  - ExitPlanMode (plan mode) → the next hook event was a NEW PreToolUse ExitPlanMode (the model re-planned: a
    revise taking effect);
  - AskUserQuestion ×3 → the next hook event was Stop.
- Dialog concurrency: 0 of 29 dialog PreToolUse hooks arrived while another was unanswered (not exercised); 9 calls
  carried more than one question inside one tool call.

## What it shows (and does not)
- On interactive 2.1.287, a PreToolUse `allow` + `updatedInput` (answers, annotations; or plan approve) answered the
  dialog at the hook layer: the CLI raised no PermissionRequest for it afterwards, in 25 of 25 cases. A
  PermissionRequest `deny` + `message` on ExitPlanMode was followed by a re-plan. That is hook-layer evidence of
  "the decision takes effect", measured live by the prototype, never by this repository's probe.
- Not shown: anything on 2.1.288; whether the dialog was drawn on screen at all; an ordinary-tool PermissionRequest;
  dialog concurrency; a PostToolUse `tool_response` carrying the answer.
