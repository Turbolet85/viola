
## 2026-10-19-overseer-switch-to-viola — question option text crosses into the event and `wait` payloads; `background_tasks` a count
**Section:** §Input Validation (Hook stdin row, **Question and Stop data**); Threat Model Summary (Data classification, the dialog content's Where); Bootstrap phases → `logging-redaction-wire` (the floor's Scope); Security Anti-Patterns → Code Patterns (R1).
**Change:**
- Hook stdin: each question's `option_details` takes an option's `description` and `preview` only as strings, else `null`, never a refusal (`options` keeps its labels); the Stop's `background_tasks` is read as `serde::de::IgnoredAny` elements and carried only as the array's length, `null` when absent, `null` or not an array, a non-array also writing the drift entry `{"path":"background_tasks","expected":"array"}`. The option text crossing into `events.ndjson`, `hook.dialog`, CLI `wait --json` and the MCP `wait` is recorded as the founder's own widening.
- Data classification: `question` carries questions, options and each option's description and preview text, which also go out over CLI `wait --json` and MCP `wait`. Was "questions and options".
- The floor's Scope: user content is "prompts, `last_assistant_message`, plan, question and option text". Was "plan and question text".
- R1: upstream text lists "plan text, question and option text, tool `input`". Was "plan text, tool `input`".
**Why:** A validated surface admits more upstream content: a boundary widening the founder answered himself at P4 fork Q4 («Строить оба», `inputs#I14`, relayed by the overseer), `background_tasks` count only by the same answer. The wrap's answer-card confirm of these four sections is his own word, relayed by the overseer (`inputs#I21`).
**Ref:** .andromeda/runs/2026-10-19T19-16-36-wrap/
