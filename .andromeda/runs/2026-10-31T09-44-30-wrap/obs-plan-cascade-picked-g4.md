# obs-plan · picked accounting · group 4 of 4

## Picked
pii-scrubbing-compliance#1.2 · not touched · `src/run/env.rs` gained only a test (`env_keep_keeps_no_key_past_the_frame_cap`) and the report moves no strip field or canary, so the names-only rule holds
pii-scrubbing-compliance#1.5 · not touched · `crates/viola-channel/src/lib.rs` gained `cause` and `sanitised_fault` with no param type moved, and the report's `detail` carries no param value or name, so the content rule holds
pii-scrubbing-compliance#1.7 · not touched · `src/cmd/hook/statusline.rs` gained only the cap cases at 652-679, and no span field or line moved, so `statusline_command` stays out of home-level logs
pii-scrubbing-compliance#1.9 · not touched · the report adds no field beside the closed `detail`, so `endpoint` and `pinned_bin` remain unlogged
pii-scrubbing-compliance#1.10 · not touched · `detail`'s new values are three closed words with no pid or time in them, so low-class metadata stays outside `detail` codes
pii-scrubbing-compliance#1.11 · not touched · `src/run/agents.rs` reads `resolve_program` through `let Ok(..) else`, unchanged, so rows are still never logged verbatim
pii-scrubbing-compliance#2.1 · proposed · F12, F13 and F14 sit beside the secret scan's verification items, which the report leaves unchanged
ci-integration#1.3 · not touched · `.config/nextest.toml`'s one added line is the kill-line filter, and the report names no JUnit, nextest version or upload change
ci-integration#4.1 · proposed · F15 reads `v1-47`'s scan after each job's end-to-end run on this step's kept homes and chaos-home ruling, which themselves stand
slo-invariants-telemetry-budgets#1.1 · not touched · `src/cmd/client.rs`, `src/cmd/run.rs` and `src/cmd/verify.rs` moved only reply screening and `AgentError` matches with outputs unchanged, and no panic path or check order moved, so the invariant and its exemptions hold
slo-invariants-telemetry-budgets#2.2 · not touched · no budget or gate moved and the row's values are dated to their own chunk, while the report states means only, no `max` to carry; see the note in the proposals file
obs-anti-patterns#1.1 · not touched · the added lines in `src/cmd/run.rs` and `src/cmd/verify.rs` are imports and `AgentError` matches and the report moves no refusal-writer count, so the print ban and the `human::refuse(` tally hold
