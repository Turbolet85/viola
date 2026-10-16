# security extract

## Relevance
relevant — the chunk widens `viola verify`'s interactive child set (a `--resume` probe, a `/compact` paste), adds recorded fixtures, two ledger rows to the stamp's relied set, and a `send` claim change for a local command.

## Constraints
- Any new `viola verify` child (a sixth start resuming a verify-started session) or added Run B paste is a boundary widening of the enumerated child set: security-plan §Threat Model Summary (Vector: Child process spawning and PATH resolution) fixes the four interactive runs, their dirs, arguments and five compiled pastes as founder rulings, so a sixth child or a sixth paste needs his own ruling recorded there before it ships (the scope's P4 gate); per security-plan §Threat Model Summary.
- A `--resume` probe's session id reaches the child's argv only in the closed session-id shape (`viola_agent_claude::is_session_id`), as one argv element of a direct spawn (never a shell), built only by `viola_agent_claude::resume_args` (the one product home of the `--resume` / `--fork-session` literals); the id's source here (a session the verify run itself started) is a new source beside the ratified log/`--id` sources and is itself a crossing to show the founder; per security-plan §Input Validation (Session id as a child argv value row).
- Every verify child stays a direct PTY spawn under the R8 strip, its output capped at `MAX_FRAME` into a `Screen` fed under `catch_unwind` and read only against compiled signature literals; no byte is typed into any dialog, and a modal start is killed with no key; per security-plan §Input Validation (Child process output row).
- Probe dirs and captures for a new run: 0700 dirs / 0600 files under `ledger/probes/<pid>/` (captures claimed by the hidden `hook --capture` arm), any new interactive probe dir created 0700 and removed by its own drop guard on pass, fail, Ctrl-C or TERM, removing only what that run created (never by pattern or listing), and viola writes nothing under `~/.claude`; per security-plan §Data Protection (Probe captures / Interactive probe dirs).
- Each extra live session (a resumed session, a compaction) adds CLI transcripts under `~/.claude/projects/`; the accepted-risk class counts transcripts per verify per founder ruling, so the new count must be shown and accepted, never assumed covered; per security-plan §Data Protection (Accepted risk: verify's trusted runs, B, C and D).
- New recorded fixtures (the SessionStart `resume` and `compact` payloads, any new screen) go through `--record`'s scrub and closed-code refusal (`home-path` · `absolute-path` · `username` · `email`), synthetic probe prompts only; the live `resume` payload's `transcript_path`, `cwd` and `scratchpad_dir` keys carry home paths and must come out scrubbed or refused; per security-plan §Data Protection (Repository fixtures).
- The stamp's relied set gates every non-`null` dialog decision: adding rows means a home stamped before them reads `cli_verified:false` until re-stamped, and the ledger's row count and per-OS relied counts written in the plan (twenty-one, `/21` Unix, `/18` Windows) move with the code; only `viola verify` writes `ledger/stamps.json`; per security-plan §Security Anti-Patterns (Universal).

## Patterns to follow
- Compiled literals only: a `/compact` entry on the local-command list and any new paste are compiled `viola-agent-claude` ledger rows, never built from runtime or upstream text (security-plan §Security Anti-Patterns, Code Patterns).
- The statusline-row precedent of chunk 2026-10-13-owed-ledger-rows: a probe widening recorded in §Threat Model Summary with its ruling source, its per-OS reliance stated, its fixtures kept under the scrub (security-plan §Threat Model Summary; §Data Protection).
- The `revive` spawn shape: fixed program `claude` via `run`'s resolver, `--plugin-dir` first, then `resume_args`, direct spawn (security-plan §Input Validation, Child executable resolution row).
- Fail-open refusals: a new `send` refusal or post-condition path goes to automation only and never blocks a human keystroke (security-plan §Security Anti-Patterns, Universal).

## Anti-patterns to avoid
- Never add a sixth verify child, a sixth Run B paste or a new id source on a decisive lean or a card confirm: each is a widening needing the founder's own word (security-plan §Threat Model Summary).
- Never put a session id, a recorded directory or a pid on a refusal line, an external error or a fixture outside the scrubbed form (security-plan §Input Validation, Session id row; §Error Handling).
- Never shell out for the resume probe: `statusline_command` stays the only shell-out (security-plan §Security Anti-Patterns, Code Patterns).

## Contract bindings
- security §Data Protection (Repository fixtures) ↔ tests: the new `SessionStart` `resume` / `compact` fixtures replace the fake agent's rewritten 5-key payload; no real paths or usernames in committed fixtures.
- security §Security Anti-Patterns (Universal, relied-set count) ↔ arch CLI Version Compatibility / tests: the relied-set count literals and the older-stamp `cli_verified:false` oracles move together (the five shape-readers plus `merge_stamp`'s envelope oracle named in scope).
- security §Error Handling ↔ obs: any new `send` refusal detail for a local command's post-condition carries fixed codes only.

## Acceptance criteria contributions
- A home stamped with the previous relied set (21 ids on Unix, 18 on Windows, all `pass`) reads `cli_verified:false` after the rows land, and `answer` exits 12 `unverified-cli` on it (per security-plan §Security Anti-Patterns, Universal).
- The `--resume` probe's child argv holds the id as one element produced by `resume_args`, of `is_session_id` shape, with no shell in the spawn path (grep: no `sh -c` / `cmd /c` in the verify spawn) (per security-plan §Input Validation, Session id as a child argv value row).
- `viola verify --record` of the new fixtures passes its scrub with no `home-path` / `absolute-path` / `username` / `email` refusal, and the committed `resume` / `compact` payloads hold `~` / `<user>` in place of every home path (per security-plan §Data Protection, Repository fixtures).
- After a passing, failing and Ctrl-C-stopped verify round, no new probe dir remains under the cwd or `ledger/probes/`, and nothing new is written by viola under `~/.claude` beyond the CLI's own transcripts counted in the accepted risk (per security-plan §Data Protection, Interactive probe dirs / Accepted risk).
