# P5 review card 1 — 2026-10-18-compact-post-condition-ledger-row

Plan: `viola-0.1.0/chunks/2026-10-18-compact-post-condition-ledger-row/plan.md` (14 steps, 12 acceptance criteria, 26
gate entries). Mechanical checks: all pass (fence parses; planlint 0 hits; 10 new entries baselined, 3 guards with
must-pass and must-fail controls in `.andromeda/runs/2026-10-18T03-26-43-phase/p5-controls.md`; inputs I1–I3 cited, none
unparsed). Validation-1: aligned with `scope.md` and the working entry (CARRY 2's exit is the scope's own P4
amendment from the founder's answers, `inputs#I3`).

## Goal
A `send` of `/compact` on a verified CLI is confirmed by the next `session-start` of cause `compact` carrying the same
`agent_session_id` the slot remembered, inside a compiled window of its own, `COMPACT_CONFIRM_WINDOW` = 40 s
PROVISIONAL (the 10 s fallback unchanged); an unverified CLI still reads `unconfirmable` at once. The behaviour is the
24th ledger row, `local-command-compact`, relied on every OS (Unix 23 → 24, Windows 20 → 21), probed by a sixth compiled
Run B paste (`/compact` after the newline paste, before `/clear`) and stamped from one `viola verify --record` round on
2.1.287 (six starts; one re-run after a red; never beyond twelve) — the founder's own answers to forks 1 and 2.

## Decisions taken as leans (each with its artifact)
- Window 40 s: architecture [Delivery Confirmation] — the send's total (8.5 s gate + paste + window) must stay under the
  MCP tool-call timeout, read at 50 012 ms on 2.1.287; one compaction measured 18.8 s; 41.5 s is the arithmetic edge.
- Nothing remembered claims nothing: architecture [Delivery Confirmation] — a send is confirmed, never presumed.
- The row appended last (rows 1–23 keep their slots); `/compact` fourth in `FRAMING_TURNS` so `/clear` stays last.
- The fake agent fires the recorded `SessionStart.compact-1` with `session_id` set to the last SessionStart it fired —
  its `--resume` precedent; a byte-unchanged recorded id could never equal the fake session's.

## Top acceptance criteria
- 24 rows; an all-`pass` stamp of the 23 older ids reads unverified, the 24 read verified.
- The same-id claim on a verified CLI; another cause, another id, no id or nothing remembered claims nothing; unverified
  stays `unconfirmable`; the window expires at its own 40 s bound, clock-driven at the unit tier.
- Run B pastes exactly six compiled texts, none into a dialog; six starts and eight spawn pairs stay.
- One live round prints `stamped 2.1.287  24 pass  0 fail`; `fixtures/claude/2.1.287/` gains only
  `SessionStart.compact-1.json`; the drift contract replays it byte for byte.
- `viola verify` prints `[NN/24]` (Unix) / `[NN/21]` (Windows); a human `viola send /compact` on a stamped home prints
  `[RB] read back`.

## Touchpoints
`crates/viola-agent-claude/src/{screen,ledger}.rs`, `src/run/send/{slot,classify}.rs` (+ `src/run/send.rs` imports),
`src/cmd/verify/typed.rs`, `src/cmd/verify.rs`, `src/bin/viola-fake-agent.rs`, `src/run/version_gate.rs`, seven test
files, `crates/viola-e2e/src/harness/run.rs`; new: the recorded fixture and `evidence/live-round.md`.

## Test commands (26 entries)
fmt · clippy · G1 · lint probes · `run --unit` · the new cases by name · the live round (probe, transcript count,
`verify --record`, census, count) · the fixture guard · the integration binaries · the literal probe (106 → 0) · the
preservation guard · `cargo deny` · the harness smoke with G2 / G4 · `pre-push` · operator: hygiene, push, CI read.

## Capabilities — concretization preview
None claimed. `v1-34` stays pooled (it still needs the harness-prefix and R8 identity-floor rows, owed with no entry
minted); `v1-42` stays pooled (its readings move with CARRY 2). Each gets a dated not-claimed note at step 4 after the
word.

## Beyond the plan
1. **A new working-route entry, "Plugin precedence live readings"**, placed directly after this one in Epoch 5,
   carrying CARRY 2 whole: the two plugin readings on claude 2.1.287 (which `viola` MCP server or servers a wrapped
   session holding both the `--plugin-dir` plugin and the installed `viola@viola` starts and which answers `list`;
   whether a registered `viola mcp` that exits 1 blocks an unwrapped start), the method past the folder-trust dialog,
   and `v1-42` — the founder's own answer to fork 3 («Отдельный новый вход», `inputs#I3`); written by this chunk's wrap.
2. **The tmpfs live home's re-stamp** (the `run --local-live` home) is left to its next firing, which spends starts
   outside this chunk's cap; no entry owns it — carried as a handoff note beside "every home stamped before this chunk
   reads `cli_verified:false` until its next `viola verify`".
