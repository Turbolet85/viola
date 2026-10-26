proposals:
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 488
    section: '## 1. Obs Scope Summary'
    change: '  - The server-verify fail is written on Linux and Windows as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` in `cli-<name>.ndjson` by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (the two link verbs into `cli-<driven>.ndjson`; chunk 2026-10-26-explicit-session-links), distinct from `instance-dead`, with no pid, start time, path or endpoint; macOS writes none (no peer pid, the recorded gap).'
    sidecar: 'obs-scope-summary Founder Direction 4 trigger: the CLI writers of `server-verify-failed` are eight, `link` / `unlink` added (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The report makes the CLI frame verbs eight (send, wait, last, answer, pause, release, link, unlink), each writing `server-verify-failed` on a mismatch, and step 5 of `link_call` writes `process-exit{… exit_code:21, detail:"server-verify-failed", during:"connect"}`; the line enumerates only the six (pattern `frameverbs`/`srvverify`, a site the sweep did not list because it names the verbs, not "six").'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:77
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 397
    section: '## 1. Obs Scope Summary'
    change: '  - Refusal telemetry for strict-modes failures, pinned-exe hash mismatches and ignored symlinks. As landed a strict-modes failure has a line only on the `ledger-stamps` read in `run`''s version gate. Five strict-modes refusals leave no line: `viola revive`''s instance check, `viola hook statusline`''s instance check, the checked read of `budget.json` (`read_budget_stamped`, which the budget gate takes through `read_budget` and `viola list` takes directly, where a refused file is no reading) (chunk 2026-10-10-budget-governor), `viola list`''s scan of the home, `instances/` and each instance, which refuses the whole board with exit 21 (chunk 2026-10-11-the-board-viola-list), and `viola link` / `viola unlink`''s instance check of the driven instance, made before `client::start` so before any role file or log opens, which refuses with exit 21 and the directory''s mode as found (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'obs-scope-summary Vector 7 trigger: the no-line strict-modes refusals are five, `viola link` / `viola unlink`''s instance check added (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Report Symbols / APIs step 1: `check_instance` runs BEFORE `client::start`, refused → exit 21, no line in any log, the directory''s mode as found; the report names this enumeration (`obs-scope-summary.md:397`) as the obs detector''s to extend under pattern `checkinst`, and its count "Four" is now stale.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:41
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 487
    section: '## 1. Obs Scope Summary'
    change: 'Append to the bullet: `viola link` / `viola unlink`''s strict-mode fail carries no logged `detail` either (the check precedes the log): its output is the fixed instance-state `unable` / `hint:` pair through `human::refuse`, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}` on stdout with stderr empty, exit 21 (chunk 2026-10-26-explicit-session-links; the founder''s word `inputs#I3`, exit 21 per card lean 5).'
    sidecar: 'obs-scope-summary Founder Direction 4 trigger: the link verbs'' strict-mode refusal recorded as unlogged, its exit 21 and its `--json` document (chunk 2026-10-26-explicit-session-links).'
    rationale: 'The bullet carves out revive''s unlogged strict-mode fail; the report adds a second exit-cause with no logged detail: the link verbs'' refused `check_instance` writes no line, prints the instance-state pair or `{"v":1,"error":"instance-unreachable","detail":null}`, exit 21 (Symbols / APIs step 1; Decisions: exit 21 as `list` / `plugin install`, not revive''s exit 1).'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:44
  - detector: D-obs-instrumentation
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 497
    section: '## 1. Obs Scope Summary'
    change: '  - Higher-`v` rejections (-32602; the code also carries `send`''s invalid params, `release` carrying `from`, and a `link` / `unlink` whose `driver` or `driven` name is missing or invalid, whose `driven` is not the serving wrapper, or whose `driver` equals `driven` (chunk 2026-10-26-explicit-session-links), so it is not a higher-`v` count alone).'
    sidecar: 'obs-scope-summary compat trigger: `-32602` over-count gains the served `link` / `unlink` param refusals (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Report Counts: `-32602` causes behind `viola.compat.v_rejected`''s over-count +2 (a bad `link` / `unlink` name or pair), pattern `vrejected`; Symbols / APIs: `driver_of` answers each bad name, a foreign `driven` and a self-pair with `ProtocolError::InvalidParams`. This line restates the over-count with only `send` and `release` (a third site beside the report''s `obs-plan.md:418` and `metric-coverage.md:28`).'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:118
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 158
    section: '## 1. Obs Scope Summary'
    change: 'Append to the Exporter bullet: `viola link` / `viola unlink` log as `cli` into `cli-<driven>.ndjson` (the verb''s instance is the driven one, dispatched with `cli_sink(&home, &args.driven)`), opened by `client::start` only after their instance check has passed (a refused check opens none), their one request under the dispatch span `cli.link` / `cli.unlink` (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'obs-scope-summary short-lived-verb surface: the link verbs'' sink `cli-<driven>.ndjson`, opened after the instance check, and their `cli.link` / `cli.unlink` spans (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Report Symbols / APIs: `Command` gains `Link` / `Unlink` dispatched with `resolved_home` and `cli_sink(&home, &args.driven)`; `link_call` step 2 `client::start(home, &driven)` (the `cli-<driven>` role file, `process-start`) after step 1''s check; step 5 under `#[instrument(skip_all, name = "cli.link")]` / `"cli.unlink"`. The surface''s Exporter records per-verb sink rules (revive --list) and has none for the new verbs.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:69
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 355
    section: '## 1. Obs Scope Summary'
    change: '  - The served `link` / `unlink` methods add nothing beyond this pair (`method:"link"|"unlink"`): no span and no process-log line of their own, and the pair holds neither the `driver` nor the `driven` param (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'obs-scope-summary Vector 1 trigger: the served `link` / `unlink` methods logged by the channel pair alone, no names (chunk 2026-10-26-explicit-session-links).'
    rationale: 'Report Symbols / APIs: neither method opens a span nor writes a process-log line, the wrapper''s `channel-request` / `channel-response` pair carries `method:"link"|"unlink"`; Outcome (obs): channel pair holds no names, met by `link_verbs_log_the_channel_pair_and_no_names`; Coverage: no driver name in the wrapper''s process log, asserted.'
    basis: viola-0.1.0/chunks/2026-10-26-explicit-session-links/report.md:28
note: 'D-obs-stack and D-obs-pii read clean (no dependency, no new logged field, names-only and asserted absent from the wrapper log); the link verbs'' strict-mode refusal shares the `--json` code `instance-unreachable` with a dead instance and logs no `detail`, which the orchestrator may weigh against Founder Direction 4''s distinct-code rule (line 487).'
