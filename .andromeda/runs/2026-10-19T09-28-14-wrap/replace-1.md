- line: .andromeda/architecture.md:66
old: «A sync dialog hook (`viola hook pre-tool-use|permission-request`) waits on `hook.dialog` for at most viola's dialog deadline, the product constant `viola_core::DIALOG_DEADLINE` = 60 s (PROVISIONAL), beside `SPINE_DEADLINE`; the hook bounds its reply read at `DIALOG_DEADLINE + 5 s` and writes stdout once. The dialog hooks' `timeout` in the embedded `hooks.json` is 75 s, above the deadline by a margin a test pins, so viola, never Claude Code, ends the wait. On expiry the hook exits 0 with no body and the dialog renders for the human.»
new: «A sync dialog hook (`viola hook pre-tool-use|permission-request`) waits on `hook.dialog` for at most the instance's dialog hold, `config.json`'s `dialog_hold_s`: whole seconds from 60 to 3600, default 60 (`viola_core::DIALOG_HOLD_DEFAULT` · `DIALOG_HOLD_MIN` · `DIALOG_HOLD_MAX`, PROVISIONAL, beside `SPINE_DEADLINE`), a `DialogHold` read once at the wrapper's start; the hook bounds its reply read at the ceiling, `DIALOG_HOLD_MAX + 5 s` (3605 s), never at each start's hold, and writes stdout once. The dialog hooks' `timeout` in the embedded `hooks.json` is 3615 s, above the ceiling by a margin a test pins, so viola, never Claude Code, ends the wait, and the rendered plugin stays byte-identical across starts. At the hold's end the wrapper replies `null`: the hook exits 0 with no body and the dialog renders for the human.»
- line: .andromeda/architecture.md:67
old: «without waiting out the dialog deadline, and»
new: «without waiting out the dialog hold, and»
- line: .andromeda/architecture.md:67
old: «never hidden from the human for the rest of the deadline.»
new: «never hidden from the human for the rest of the hold.»
- line: .andromeda/architecture.md:95
old: «each with `"timeout": 75` — nine»
new: «each with `"timeout": 3615` — nine»
- line: .andromeda/architecture.md:49
old: «(`turn-ended`, `question`, `permission`, `plan` or `session-end`)»
new: «(`turn-ended`, `question`, `permission`, `plan`, `session-end` or `dialog-expired`)»
- line: .andromeda/architecture.md:223
old: «Only `turn-ended` · `question` · `permission` · `plan` · `session-end` wake»
new: «Only `turn-ended` · `question` · `permission` · `plan` · `session-end` · `dialog-expired` wake»
- line: .andromeda/architecture.md:223
old: «Only hook lines signal (the wrapper's own `send-*`, `link` and start records never wake).»
new: «Hook lines signal, and of the wrapper's own records only `dialog-expired`, appended through `WaitFeed::appending` (the wrapper's `send-*`, `link` and start records never wake).»
- line: .andromeda/architecture.md:339
old: «and an identity-floor name in it has no effect). It is read»
new: «and an identity-floor name in it has no effect; `dialog_hold_s`: the dialog hold, a JSON whole number of seconds from 60 to 3600, default 60, read by the wrapper once at its start through `run::dialog::read_hold` right after the thresholds, any other value taking 60 with one `parse-rejected{parser:"config-json", detail:"malformed"}` WARN line, so no value makes the hold zero or unbounded). It is read»
- line: .andromeda/architecture.md:366
old: «`five_hour` / `seven_day` (integers 0-100), the five `CONFIG_KEYS`»
new: «`five_hour` / `seven_day` (integers 0-100), `dialog_hold_s` (integer 60-3600), the six `CONFIG_KEYS`»
- line: .andromeda/architecture.md:404
old: «default empty). `diagnostics_level` is the only source»
new: «default empty) and `dialog_hold_s` (the dialog hold as a `DialogHold`, whole seconds 60–3600, default 60). `diagnostics_level` is the only source»
- line: .andromeda/architecture.md:145
old: «`send-confirmed`, `send-refused`. New kinds»
new: «`send-confirmed`, `send-refused`, `dialog-expired`. New kinds»
- line: .andromeda/architecture.md:206
old: «It is removed when the dialog is answered, expires or is answered `null` on a wheel move.»
new: «It is written before the dialog's event line is appended, and written back to none when that append fails, so a reader that sees the line already sees it. It is removed when the dialog is answered, expires or is answered `null` on a wheel move.»
- line: .andromeda/architecture.md:254
old: «and each send's outcome. Since chunk»
new: «and each send's outcome. The wrapper also appends `dialog-expired` (`source: wrapper`, `{dialog_id, dialog_kind}`) once when a held dialog's hold ends; it is the one wrapper-appended kind that wakes `wait`. Since chunk»
- line: .andromeda/architecture.md:25
old: «`Percent` (0–100) |»
new: «`Percent` (0–100), `DialogHold` (whole seconds 60–3600) |»
- line: .andromeda/architecture.md:92
old: «nutype newtypes `ViolaName` and `Percent`.»
new: «nutype newtypes `ViolaName`, `Percent` and `DialogHold`.»
- line: .andromeda/architecture.md:309
old: «`timeout` 75).»
new: «`timeout` 3615).»
