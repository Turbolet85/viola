
## 2026-10-19-overseer-switch-to-viola — `question` gains `option_details`, `turn-ended` gains `background_tasks`
**Section:** §Standard Contracts (Event `data` per kind: `turn-ended`, `question`; the `turn-ended` `hook.event` example frame).
**Change:**
- `turn-ended` is `{last_assistant_message, background_tasks}`. `background_tasks` is the Stop payload's `background_tasks` array length, a count only (no element read or carried), or `null` when the key is absent, `null` or not an array. Was `{last_assistant_message}`.
- `question` is `{dialog_id, questions:[{question, header, options:[<label>], option_details:[{description, preview}], multi_select}]}`. `option_details` is parallel to `options`, each field the option's string or `null`, never a refusal; the option text is upstream content carried into `events.ndjson`, `hook.dialog`, CLI `wait --json` and the MCP `wait`, never into a human line. Was `{dialog_id, questions:[{question, options:[<label>], multi_select}]}` (the row also omitted the `header` the data already carried).
- The `turn-ended` example frame's `data` carries `"background_tasks":null`.
**Why:** The chunk built both gaps research found for the overseer's driving. The option-text crossing is a boundary widening the founder answered himself at P4 fork Q4 («Строить оба», `inputs#I14`, relayed by the overseer), the count-only form of `background_tasks` by the same answer; the wrap's answer-card confirm is his own word, relayed by the overseer (`inputs#I21`).
**Ref:** .andromeda/runs/2026-10-19T19-16-36-wrap/
