
## 2026-10-04-dialog-answers-by-dialog-id — the dialog deadline bounds the driver, never the human
**Section:** §8 Cognitive Accessibility → Timeout extensions (CLI)
**Change:** was "viola imposes no limit on a human"; now a raised dialog takes a `viola answer` only within `DIALOG_DEADLINE` (60 s PROVISIONAL, under the `hooks.json` `timeout` of 75 s); on expiry the hook exits 0 with no body, the dialog renders for the human in the `claude` TUI, and a later `answer` is refused `not-delivered` / `unknown-dialog` (exit 13). The terminal still carries no conformance claim.
**Why:** the chunk served `answer` with a deadline; the limit falls on the driver's answer path, and architecture's fail-open contract leaves the dialog to the human.
**Ref:** .andromeda/runs/2026-10-04T16-53-44-wrap/
