# Cascade dispositions — 2026-09-29-verify-stamped-test-homes-and-harness

The search. `cascade-patterns.toml` holds 12 patterns, every one derived from this pass's amendments before the
first grep. Each pattern's control fired on the pre-pass masters (baseline `31dd9956`, the pre-CI parent). The
listing is the tool's (`cascade.py sweep`).

| pattern | retired claim (wording and verb) |
|---|---|
| `stamperror` | the second enum `StampError` |
| `fold-await` | "awaits / awaiting its fold" |
| `interim-seam` | "interim (no-stamp) seam" |
| `until-verify` | "Until `viola verify` exists" |
| `root-waits-8` | "8 root waits" |
| `lines-1-2` | "lines 1–2 today" |
| `joins-verify` | "join(s) with "Verify-stamped…"" (future ownership) |
| `stays-210` | "stays 2.1.0" |
| `realcli-owner` | "real-CLI verify entry" (H2 owner) |
| `nothing-stamped` | "nothing is stamped yet" |
| `subj-no-verify` | the subject list `version-probe\|agents-probe` without `verify-probe` |
| `nine-suites` | "nine … values" (the suite count) |

Beyond the patterns, the sections I read: architecture [Error Handling], [PTY], Conventions (Rust error types),
Standard Contracts (Ledger stamps envelope), Occupied Resources (Environment variables, Filesystem, Repository),
Inherited Defaults; obs-plan §4 (Scenario 1, `verify`), §6, §12; test-plan §3 (`boot` readiness, `run` step 2,
`run` flags, Output format, `gate`, Closed enums), §5 Module ↔ PTY row, §7 Fake agent, §10 Perf session, §12.

## Rows (every row the listing printed)
- `.andromeda/architecture.md:97` stamperror `new`: this pass's own history clause ("the second enum `StampError`
  was folded into it"), a true statement sharing the token. No change.
- `.andromeda/test-plan.md:1936` root-waits-8 `standing`: inside §12 Test Decisions Log, a historical entry (the
  2026-09-28 hook-perf decision). History is never modified. No change. The current count lives at §3 `run` step 2
  (amended: 9).
- `.andromeda/test-plan.md:645` nine-suites `standing edited`: this pass's own edit ("the nine closed `suite` values
  other than `local-live`"), true. No change.
- `.claude/docs/conventions.md:15` stamperror / fold-await `leaf`: re-derived from arch §Conventions + [Error
  Handling] (AgentError at the root with `Malformed` + `StampsMalformed`; `Refusal` the open divergence).
- `.claude/docs/stack.md:24` stamperror / fold-await `leaf`: re-derived from arch §Inherited Defaults (Errors).
- `.claude/docs/services/viola-agent-claude.md:24` stamperror `leaf`: re-derived (`verified` →
  `Result<bool, AgentError>`, `StampsMalformed`, `Refusal`).
- `.claude/docs/tests-summary.md:21` interim-seam `leaf`: re-derived from test-plan §3 `run` step 2 and §7 (stamped
  seam at 2.1.283, `StampedHome::unstamped`, `run --local-live` refused under CI).
- `.claude/docs/gotchas.md:109` realcli-owner `leaf`: re-derived from arch [PTY] (owner "First live test and
  self-drive").
- `.claude/docs/services/viola-pty.md:30` realcli-owner `leaf`: re-derived the same way.

## Leaves beyond the rows (step 3, by provenance)
A grep over `.claude/docs/*.md`, `.claude/docs/services/*.md`, `.claude/rules/*.md` and `CLAUDE.md` for the moved
facts: `Still to join|lines 1.3|interim|stamped_home|DEFAULT_CLI_VERSION|2\.1\.0|local-live|live-in-ci|version-probe|verify-probe|Malformed|StampError|real-CLI verify|unstamped|root waits|stop_keep|endpoint_gone`.
- `.claude/rules/verification-harness.md:29`: "Still to join: `events.ndjson` lines 1–3" re-derived from test-plan
  §3 readiness (`start_records`, `<name>:events`). UI readiness still to join.
- `.claude/rules/verification-harness.md:43`: "`stamped_home` interim, no stamps" re-derived from test-plan §3
  `run` step 2.
- `.claude/docs/commands.md:31-32`: already lists `--unstamped` and `--local-live`, consistent. No change.
- `.claude/docs/workflow.md:49`: "real `claude` runs only locally (`agent-run run --local-live` …)", consistent. No
  change.
- `.claude/docs/services/viola.md:20,30`: `:30` was written at implement (verify's spawn pairs), consistent with obs
  §6 and D-35. No change.
- `.claude/docs/security-summary.md:18`, `services/viola-state.md:22`, `rules/api.md:30`: the Epoch-6 strict-modes
  interim, a different fact. No change.
- `.claude/docs/obs-summary.md`, `.claude/rules/observability.md`: carry no subject list and no verify log shape. No
  change.
- `CLAUDE.md` `GENERATED:setup:*` (overview · modules · warnings · pointer-table · workflow · architecture):
  recomputed from arch's structural sections. Its `viola-agent-claude` module line names no error enum; warnings,
  the pointer table (active `viola-0.1.0`) and workflow carry none of the moved facts. No change.

## Curation homes and judgment bases
0 `curation` rows and 0 `base` rows across the 12 patterns, so nothing routes to P3 or to propose→approve.

## Standing, not this chunk's
test-plan §10 Perf session: "no recorded fixture exists yet" (perf payloads). The recorded set `fixtures/claude/2.1.283/`
was landed by an earlier chunk (01f22aa, 2026-09-28), not this one. This pass does not touch it (playbook "Not this
chunk's drift"); it is noted for the owning perf work.
