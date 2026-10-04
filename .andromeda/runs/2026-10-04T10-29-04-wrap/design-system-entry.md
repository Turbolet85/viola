
## 2026-10-04-wait-and-last — cli pattern 3: session-end line and message mode
**Section:** Surface: cli → Component Patterns 3 (`viola wait` / `viola last`)
**Change:** The result lines gain `session-end` (every non-dialog kind takes the `turn-ended` form) and `dialog unknown` for an event without a `dialog_id`; message mode is spelled out: every control character but `\n` and `\t` prints as `\xHH`, two uppercase hex digits, never stripped; `--json` stays serde-escaped.
**Why:** CARRY 1's escaper landed with its live consumer, `last`; the `session-end` result line had no form.
**Ref:** .andromeda/runs/2026-10-04T10-29-04-wrap/
