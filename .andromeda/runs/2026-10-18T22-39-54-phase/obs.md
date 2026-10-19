# obs extract

## Relevance
partial — the chunk plans no new instrumentation; its readings are taken from the home-level role files (`diagnostics/mcp.ndjson`, `hook-<name>.ndjson`) and `events.ndjson`, so obs governs what those files can and cannot evidence and how the recorded evidence is scrubbed.

## Constraints
- obs-plan §2 (Telemetry surfaces, the `viola mcp` row) requires every `viola mcp` process to log to `<home>/diagnostics/mcp.ndjson` of the home resolved through `resolved_home`, with `instance` set to the server's own `VIOLA_NAME` when one is present. Each of the two servers therefore logs into its own home's file: the installed plugin's `--home <canonical home>`, and the `--plugin-dir` plugin's home reached through `VIOLA_DIR`. Which home's file holds a server's `process-start{subject:"self"}` is what tells the servers apart; the `instance` field cannot, because both servers may inherit the same `VIOLA_NAME`. Whether the installed server inherits `VIOLA_NAME` inside a wrapped session is research's question.
- obs-plan §6 (Boundary-call wrappers, Child / shell spawns) requires `viola mcp`'s `list` tool to write its one `claude agents --json` read as a `process-start` / `process-exit{subject:"agents-probe"}` pair into that server's `mcp.ndjson`. That pair is the logged trace of a `list` answer. Whether the `list` tool call also writes a line of its own (there is no channel frame for `list`) is research's question.
- obs-plan §7 (the pre-init window) and §10 (Zero unlogged panics, exemption 7) require `viola mcp`'s check of its home and `diagnostics/` to run before any role file opens. A refused check (the exit-1 reading's 0770 home) therefore writes no line in `mcp.ndjson`, and the exit status is the only evidence. The exit-1 reading has to be taken from the CLI side (whether the session reaches `idle`), not from viola's process logs.
- obs-plan §4 (Scenario: `viola run` start sequence to child spawn) requires the wrapped start's `session-start{source:"hook"}` to be verified from `events.ndjson` (record three, after `wheel{cause:"start"}` → `budget-gate`) through `agent-run logs --kind`, not copied into process logs. The "one `session-start`" reading counts that record.
- obs-plan §6 (Additive field catalog, `hook-invoked` / `hook-decision`) requires each hook process to write `hook-invoked` / `hook-decision` with `hook_event` set to `session-start` into `hook-<name>.ndjson`. Exactly one such pair means one hook process ran for the event, which is the reading that the installed plugin carries no hooks.
- obs-plan §8 (Data classification rules) classes absolute paths (the OS username) as Medium, scrub-required in external output, and classes `CLAUDE*` values as Critical, never-log anywhere. Lines lifted from the live homes into the chunk's committed `evidence/` keep the home-level codes-only form: basenames, no absolute home or rig path, no `CLAUDE*` value, and no `detail-*.ndjson` content (that is real, non-synthetic user content here).
- obs-plan §8 (Verification, the high-class and detail-file upload rules) allows detail-file upload only because all CI inputs are synthetic. A live 2.1.287 reading is non-synthetic, so none of its homes' `detail-*.ndjson` or `agent-run` captures go into any upload or into the repository.

## Patterns to follow
- obs-plan §5 (Metric instrument types, the Counter row): count events with `awk 1 <files> | jq -R -n '[inputs|fromjson?|select(.event==…)]'` over home-level role files only (detail files repeat lines, and `fromjson?` skips a torn line). Use the same pattern to count `process-start{subject:"self"}` per `mcp.ndjson`, the `agents-probe` pairs, and `hook-invoked{hook_event:"session-start"}`.
- obs-plan §2 (Snapshot / paste-to-AI integration): read through the harness `logs` command (`scripts/agent-run.sh logs`), which merges `events.ndjson` (`src:"events"`) with role-file lines (`{"src":"diag","file":<basename>,"record":<line>}`). It reads the product order and the process logs as one stream.
- obs-plan §3 Log file location (Path): role files are `<home>/diagnostics/{run-<name>,hook-<name>,mcp,cli-<name>}.ndjson`, and content goes only to `instances/<name>/diagnostics/detail-<process>.ndjson`. The evidence names files by these basenames.

## Anti-patterns to avoid
- Deriving a reading from screen content or from Claude Code transcripts as a contract (obs-plan §11 Telemetry Strategy). The vt100 screen serves readiness and gate outcomes only, so which server answered `list` is read from the role files, not from the session's rendered output.
- Logging any field outside the §6 catalog, or putting a path or pid into a `detail` code (obs-plan §11 PII Scrubbing). This applies if P4 finds a fix that the readings force and that adds a line, for example a server-start discriminator.
- Writing telemetry to stdout on the `mcp` role, or leaving the writer implicit (obs-plan §11 Logs). This applies to any `viola mcp` change a reading forces, because stdout carries the MCP frames.

## Contract bindings
- obs ↔ tests harness: the `logs` / `status` commands and `agent-run logs --kind` are the consumption path for both readings (obs-plan §2 Snapshot / paste-to-AI integration; test-plan §3 Log format, reproduced in obs-plan §3 Log format JSON schema).
- obs ↔ CI gates: a live rig home placed under `target/e2e-home/` falls inside the G2 (panic), G4 (schema) and secret-scan scope (obs-plan §9). A home placed elsewhere falls outside every gate, so its readings are only as clean as the scrub applied to the evidence.
- obs ↔ security: the NEVER-log floor and the path-scrub rule on the committed `evidence/` (obs-plan §8 Data classification rules; security-plan §Logging / NEVER-log floor).

## Acceptance criteria contributions
- The precedence evidence names, for each of the two homes, the count of `process-start{subject:"self", process:"mcp"}` lines in its `diagnostics/mcp.ndjson` and of `agents-probe` pairs written during the `list` call. Both counts are read by a `jq` assertion over the home-level files only, never from screen text (per obs-plan §5 Metric instrument types).
- The wrapped start's evidence shows exactly one `session-start` record in `events.ndjson` and exactly one `hook-invoked{hook_event:"session-start"}` in `hook-<name>.ndjson` (per obs-plan §4 Scenario: `viola run` start sequence to child spawn).
- The exit-1 evidence records that the refused `viola mcp` left no line in its home's `mcp.ndjson`, and takes the start's outcome from the CLI side, not from a viola process log (per obs-plan §10 Zero unlogged panics, exemption 7).
- No file under the chunk's `evidence/` holds an absolute path, a `CLAUDE*` value or a `detail-*.ndjson` line (per obs-plan §8 Data classification rules).
