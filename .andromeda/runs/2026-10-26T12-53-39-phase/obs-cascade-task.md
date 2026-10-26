# Task — obs · 2026-10-26-explicit-session-links

## The chunk
Serves the two channel methods `link` and `unlink` in the `viola run` wrapper (today answered `-32601`): `link` records a driver/driven pair and answers `{since, already_linked}`, `unlink` removes it and answers the empty object, an `unlink` of an absent pair appends nothing, and a `driven` that is not the serving wrapper's name or a `driver` equal to `driven` is `-32602`.
Adds the CLI verbs `viola link <driver> <driven>` and `viola unlink <driver> <driven>` (with `--json`; the verb's instance is the driven one), their three human result lines, and the `handoff:  link, unlink` row of `viola --help`.
Gives `LinkSet::remove` its first product caller (the `unlink` method appending an `unlink` line to `events.ndjson`) and drops its dead-code allowance; re-parses every logged link name through `ViolaName::try_new` before a result line shows it.
How the verbs reach the wrapper (the liveness pre-check plus the server identity check through `client::connect`, and whether a strict-modes instance check runs ahead of the snapshot read) is the founder's pick at P4; the plan covers every form up to it.
Proves `v1-13`'s `link` half (each human `link` line derivable from its machine result); whether its MCP half is provable here is P3's to read. No `viola verify` child, ledger row or live `claude` start; P3 re-reads that no `viola-agent-claude` file is touched.

## Reached files
- src/run/links.rs · edits `LinkSet::remove` and its `#[cfg_attr(not(test), allow(dead_code))]`
- tests/channel_links.rs · edits test-plan §6 Scenario E3's steps 1 and 3 and the raw-channel name negative
- src/cmd/mod.rs · edits the `HELP` literal's `handoff:` row
- tests/cli_output_plain.rs · edits `help_is_the_grouped_verb_table`, `help_lists_only_served_verbs`
- src/cmd/client.rs
- viola-0.1.0/verification-matrix.json · edits row `v1-13`

## Needs
- The wrapper's channel dispatch for the two new methods: its `channel-request` / `channel-response` pair on both sides with `method`, `corr`, `conn`, result class and error code, the response written from the `Drop` guard in `channel.dispatch` on every return, and never the `params` or result body (`driver`, `driven`, `since`, `already_linked`).
- The closed `method` value set of channel lines (the server's method label already names `link` and `unlink`): the two methods move from the `-32601` class to served, and they become grouping values of the derived channel metrics.
- The two `-32602` refusals (a `driven` that is not the serving wrapper's name, a `driver` equal to `driven`): a `channel-response` at WARN with `error_code` -32602; the metric `viola.compat.v_rejected` already over-counts `-32602` and these two causes join that over-count.
- The self-reported driver name on a `link` / `unlink` frame is never logged as an authenticated identity (security Vector 1 trigger), and a driver or driven name is no catalog field of a channel line.
- Edge flow E3 in §4 (and its echo in §1's critical-path note): it states only the implicit link exists, with no process-log line, no `event` value and no span, and that the explicit methods are not built; this chunk builds them, so that statement and any `event` value or span the plan adds for them change (a new `event` value needs a Decisions Log entry).
- The `unlink` product path: `LinkSet::remove` appending one `unlink` line to `events.ndjson` (the product audit stream), nothing on an absent pair; no event payload is copied into a process-log line.
- The CLI verbs' process log: `process-start` / `process-exit` with `process:"cli"` to `cli-<name>.ndjson` when an instance resolves (here the driven instance), and no process-log file when none resolves.
- The verbs' connect path: a dead or unreachable instance (liveness pre-check) and, on Linux and Windows, a serving process that is not the recorded wrapper end the verb before any frame with exit 21 `instance-unreachable` and one `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN inside the dispatch span and no `channel-request` line, as `send` / `wait` / `last` log it; macOS keeps the recorded gap.
- Should P4's form put a strict-modes instance check ahead of the snapshot read, its refusal's logging reaches the Vector 7 trigger rule that a strict-modes failure has a line only on `run`'s `ledger-stamps` read, and the `strict-modes-failed` code on the `wait` / `last` scenario note owed to "Home and code-bearing file integrity".
- The `detail` code catalog for each cause behind the verbs' exit 21 and exit 1 (Founder Direction 4): a distinct closed kebab-case code, never a path, pid, endpoint or start time.
- Span names for the two verbs: `<area>` has no `link` / `unlink` today, so either a `cli.link` / `cli.unlink` dispatch span or a new area entry in the naming convention; static names, no instance name in a span name.
- The verbs' human stderr refusals through `human::refuse`: §3 records its call sites per verb as re-derived by `grep -rcE 'human::refuse\(' src`, and each refusal site the verbs add changes that count; the `unable:` / `hint:` pair carries no path, pid or name content beyond its fixed text.
- The verbs' human result lines (`link … since …Z`, `link … already linked`, `unlink …`) and their `--json` document: stdout carries only the `--json` result, human output goes to stderr, and no telemetry reaches either stream.
- A logged link name re-parsed through `ViolaName::try_new` before a result line shows it: a name that fails the parse is a closed refusal or skip, never echoed raw into a line, error or log.
- Error capture on the verbs: an internal fault is `process-exit{subject:"self", exit_code:1, detail:"internal-error"}` to `cli-<name>.ndjson`, the chain only in `detail-cli.ndjson`, then exactly `error: internal error` on stderr; channel `-32603` is `"internal error"` with `data: null`.
- The `since` instant on the `link` result and line follows the RFC 3339 UTC millisecond `Z` timestamp convention.

## Standing conventions
- S1 · every new log line goes through `obs_event!` with explicit typed fields, never a raw tracing level macro or `event!`, `corr` copied unchanged (never `?corr` / `%corr`) · §11 Obs Anti-Patterns › Logs / .andromeda/obs-plan-detail/obs-anti-patterns.md
- S2 · every new span is `#[instrument(skip_all, name = "...", fields(...))]` with skip_all first, no `err` / `ret`, a static `<area>.<operation>` name (gate G1) · §4 Span / Trace Coverage / §11 › Spans / Traces
- S3 · every field a new line carries is named in the §6 field catalog and passes G4 against `schemas/diag-line.v1.json` (default-deny; a new field or `detail` code joins the catalog and the schema) · §6 Log Coverage / .andromeda/obs-plan-detail/log-coverage.md / §8 Default-deny posture
- S4 · every new integration test's viola home lives under `target/e2e-home/` so G2, G4, the secret scan and the `diag-<os>` upload cover it · §9 CI Integration › Pipeline integration / .andromeda/obs-plan-detail/ci-integration.md
- S5 · no `print!` / `eprintln!` / `dbg!` outside the output modules that own `--json` and human CLI stderr · §11 › Logs
- S6 · every typed error crossing a crate seam has a fixed `Display`, ends in one logged closed code at WARN/ERROR, and its full chain goes only to the instance detail file · §7 Error Capture & Reporting / .andromeda/obs-plan-detail/error-capture-reporting.md
- S7 · zero unlogged panics and no retry-once policy in any new test (G2 counts home-level role files) · §10 SLO Invariants / .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
- S8 · `detail` codes are closed kebab-case and never hold a path, pid or user content · §11 › PII Scrubbing / .andromeda/obs-plan-detail/pii-scrubbing-compliance.md

## Not needed
- The Budget governor scenario (`hook statusline` shell-out, `budget.json`, `budget-gate`, `STATUSLINE_DEADLINE`): the chunk adds no budget path; `v1-13`'s `release --budget` half already landed.
- The `viola run` start sequence scenario's version gate, sideload, R8 strip and child spawn, and edge flow E2: no ledger row, no live `claude` start, no spawn change.
- The capability-ledger, `verify` and `plugin install` edge flows and their hidden capture arms: no `viola verify` child and no plugin change.
- The Dialog → `answer` scenario and its `hook-invoked` / `hook-decision` timing: no hook or dialog path changes (the implicit link on a taken `answer` stands as built).
- Edge flow E1 (unwrapped hook no-op) and the hook role file `hook-<name>.ndjson`: no `viola hook` change.
- The hook perf budgets (hyperfine `max` rows for the spine hooks, `pre-tool-use`, `session-end`): no hook command changes.
- The heartbeat liveness signal and its stale/gone flip: the verbs read liveness as other clients do and add no transition.
- The `claude agents --json` boundary (`parse-rejected{parser:"claude-agents-json"}`, unknown rows): `viola list` gets no link column and no new agents read.
- The PTY seam and vt100 feed (`process-start{subject:"claude-child"}`, paste `text_bytes`, `parse-rejected{parser:"vt100-feed"}`): a link is recorded from viola's own parameters, never typed into the child.
- The error-reporting platform (sentry, OTel SDK, OTLP exporter, metrics instruments): the chunk adds no reporter, exporter or listener.
- CI artifact retention, the nightly fuzz job and `windows-mutants.yml`: the chunk adds no CI job.
- The `statusline_command` and `CLAUDE*` data classes: neither reaches the link verbs or methods.
