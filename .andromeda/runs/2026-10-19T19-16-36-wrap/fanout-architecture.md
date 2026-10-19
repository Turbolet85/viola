proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts
    change: "The `turn-ended` row of **Event `data` per kind** reads `{last_assistant_message, background_tasks}`: `last_assistant_message` a string or `null` (still the source `last` rebuilds from); `background_tasks` the Stop payload's `background_tasks` array length, or `null` when the key is absent, `null` or not an array (a non-array also writes the drift entry `{\"path\":\"background_tasks\",\"expected\":\"array\"}`); no element is read or carried, a count only (the founder's \"count only\", `inputs#I14` Q4 of chunk 2026-10-19-overseer-switch-to-viola); additive, no `v` bump."
    sidecar: "§Standard Contracts Event data per kind: `turn-ended` gains `background_tasks` (the Stop payload's array length or `null`, count only) — chunk 2026-10-19-overseer-switch-to-viola."
    rationale: "Report Changes › Symbols / APIs: `viola_agent_claude::hook::data_of` (Stop arm) adds `background_tasks`, and `viola_core::EventKind::TurnEnded` data keys become `[\"last_assistant_message\", \"background_tasks\"]`; Schema / config names it an additive data key on a product event line; Counts / qualifiers moved marks `architecture.md:255` stale. An event data key the chunk lands that the §Standard Contracts registry does not state is D-arch-resources drift."
    basis: ".andromeda/architecture.md:255; crates/viola-agent-claude/src/hook.rs:170-173; crates/viola-agent-claude/src/hook.rs:147-152; crates/viola-core/src/lib.rs:119"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts
    change: "The `question` row of **Event `data` per kind** reads `{dialog_id, questions:[{question, header, options:[<label>], option_details:[{description, preview}], multi_select}]}`: `option_details` holds one `{description, preview}` per option, parallel to `options` (which keeps its `[label]` shape), each field the option's string or `null` (a missing or non-string value gives `null` and never refuses the dialog); the same data reaches `hook.dialog`'s frame, CLI `wait --json` and the MCP `wait` result (human `wait` lines unchanged); the option text is upstream content crossing into `events.ndjson` and the `wait` results, the boundary widening the founder answered himself (`inputs#I14` Q4 of chunk 2026-10-19-overseer-switch-to-viola)."
    sidecar: "§Standard Contracts Event data per kind: each `question` gains `option_details` (`{description, preview}` per option, string or `null`); the row also names the existing `header` field — chunk 2026-10-19-overseer-switch-to-viola."
    rationale: "Report Changes › Symbols / APIs: `viola_agent_claude::dialog::question_data` gives each question `option_details`, reaching `events.ndjson`'s `question` line, `hook.dialog`, `wait --json` and MCP `wait`; Counts / qualifiers moved states the pre-chunk shape as `{question, header, options:[label], multi_select}` and marks `architecture.md:256` (pattern `qdata`) stale; Schema / config names the founder-answered widening (`inputs#I14` Q4). The row as written also omits `header`, which the report's stated shape carries."
    basis: ".andromeda/architecture.md:256; crates/viola-agent-claude/src/dialog.rs:151-156; crates/viola-agent-claude/src/dialog.rs:119-120"
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts
    change: "The example `hook.event` frame for `turn-ended` carries both data keys: `\"data\":{\"last_assistant_message\":null,\"background_tasks\":null}`."
    sidecar: "§Standard Contracts frame examples: the `turn-ended` `hook.event` example's `data` gains `background_tasks` — chunk 2026-10-19-overseer-switch-to-viola."
    rationale: "Report Changes › Counts / qualifiers moved: `architecture.md:230` is an example `hook.event` frame whose `data` holds only `last_assistant_message`; a Stop's frame now carries `background_tasks` too — a stale example of the same retired `{last_assistant_message}` shape."
    basis: ".andromeda/architecture.md:230"
    dependent-of: D-arch-resources
note: "No D-arch-decisions drift (Dependencies: none; `serde::de::IgnoredAny` is in the pinned serde; no runtime or locked decision contradicted — architecture.md:252 and :435 'upstream text is content' stay true). Outside both detectors and so not proposed: architecture.md:77 ('A hold near the 3600 s ceiling is first read live at \"Overseer switch to viola\"') is a stale placement by the report — no hold near the ceiling arose (27 345 ms) and the P5 card's row 2 (`inputs#I15`, the operator's word, not the founder's) re-pins the reading to \"Version done-check\", the ceiling still PROVISIONAL."
