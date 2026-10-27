# Classified — security · input-validation.b1

## Answers
input-validation#1.1 · yes · the paste rule over decoded chars and the empty-typed-text refusal bind the CLI send, the MCP send and the wrapper's send, which the chunk re-verifies
input-validation#1.2 · yes · the same exact set on dialog free text in viola answer and in the wrapper's answer handler, both named in the chunk's Needs
input-validation#1.3 · yes · the channel frame cap before read_line with -32600 on an oversize frame, serde_json default depth, the closed behavior and dialog_id types and the wrapper's params checks
input-validation#1.4 · yes · the hook stdin cap at MAX_FRAME, the closed HookEvent and decision enums, the option_details and background_tasks reads, and fail-open on every failure
input-validation#1.5 · yes · the MCP tool inputs read raw and checked per handler (target, text, dialog_id, response, after, timeout_ms) with the closed invalid-params field words
input-validation#1.6 · yes · the CLI argument and stdin rules (ViolaName value_parser, the send and answer stdin cap, home resolution, the revive --id shape) in src/cmd/send.rs, answer.rs, mod.rs and revive.rs
input-validation#1.7 · yes · the config.json tolerant parse under take(MAX_FRAME) with the Percent and DialogHold bounds and one parse-rejected line per bad key
input-validation#1.8 · no · src/run/env.rs is not reached, and the Windows registry name read is outside the readers and boundary set the chunk names
input-validation#1.9 · no · standing S5 · a test-only seam stays out of the release build (a new environment read or seam needs its own Decisions Log entry)
input-validation#1.10 · no · standing S5 · a test-only seam stays out of the release build (a new environment read or seam needs its own Decisions Log entry)
input-validation#1.11 · no · src/human/style.rs is not reached, and the colour facts are host environment values only compared, outside the terminal boundary the chunk names (the PTY stream, #1.15)
input-validation#1.12 · yes · the own state-file reads (snapshot, events.ndjson replay and tail, stamps, budget, heartbeat) under the line cap, torn-line counting and the no-write rule
input-validation#1.13 · yes · the notify filter that tails only instances/<name>/events.ndjson for a name that passes ViolaName::try_new, ignoring symlinks
input-validation#1.14 · yes · the claude agents --json read through take(MAX_FRAME + 1) with its 5 s kill, the --version probe cap, and verify's output read only against the signature literals
input-validation#1.15 · yes · the vt100 feed under catch_unwind with its bounded queue, where a panic or dropped copy degrades to input-not-ready while passthrough continues, the catch_unwind re-check at src/run/gate.rs and screen.rs
input-validation#1.16 · yes · the direct spawn of claude for claude agents --json and revive's recorded cwd, whose refusal lines must hold no directory
input-validation#1.17 · yes · the session-id shape on revive --id and the resume_args argv element, with no-session refused before any spawn
input-validation#1.18 · yes · the statusline source read under take(MAX_FRAME) and the statusLine command shape, in src/cmd/run.rs
input-validation#1.19 · yes · the statusline shell-out's stdout capped and printed byte for byte, stderr discarded, and the 5 s bound
