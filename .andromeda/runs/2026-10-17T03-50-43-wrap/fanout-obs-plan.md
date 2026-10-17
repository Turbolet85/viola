proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage (the cli row of the per-role table)
    change: >-
      Retire the "except `viola revive <name> --list`, which by plan opens no process log and writes no line" clause: `viola revive <name> --list` is a `cli` verb that, after its strict-modes check passes and only under a parsed `VIOLA_NAME`, opens the CALLER's `cli-<VIOLA_NAME>.ndjson` and writes `process-start` and `process-exit` (`exit_code` 0 with no detail; `exit_code` 1 `detail:"no-session"` on an empty chain); with no name no log opens (D-06); a refused check opens no process log and writes no line, as the start arm's does.
    sidecar: >-
      §4 cli row: `revive --list` logs process-start / process-exit into the caller's cli-<VIOLA_NAME>.ndjson after its strict-modes check (chunk 2026-10-17-cli-machine-contract); the no-log exception retired.
    rationale: >-
      Report Changes, Symbols / APIs (`src/cmd/revive.rs`): "`--list`: after its strict-modes check passes, and only under a parsed `VIOLA_NAME`, it opens the CALLER's `cli-<VIOLA_NAME>.ndjson` through `client::start` and logs `process-start` and `process-exit`"; "A refused check still opens nothing and writes no line, on both arms"; Counts / qualifiers moved, pattern `list-nolog` (obs-plan.md:530 stale); Outcome (obs) MET.
    basis: src/cmd/revive.rs:244-246
  - detector: D-obs-instrumentation
    severity: warning
    section: §1 Obs Scope Summary (cli surface, Exporter)
    change: >-
      Replace "`viola revive <name> --list` opens no process-log file and writes no line, by plan, until it gains `--json` (the working-route entry "CLI machine contract")" with: `viola revive <name> --list` logs as `cli` into the caller's `cli-<VIOLA_NAME>.ndjson` once its strict-modes check has passed (none with no `VIOLA_NAME`; a refused check opens none).
    sidecar: >-
      §1 cli Exporter: the revive --list no-log clause and its owed "CLI machine contract" pointer retired; it logs as cli under a VIOLA_NAME.
    rationale: >-
      Report Counts / qualifiers moved: `list-nolog` stale at obs-plan.md:143; `owner-title` stale at obs-plan.md:143 (the owed clause, landed by this chunk).
    basis: obs-plan.md:143
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §2 Telemetry Strategy (the zero-unlogged-panics bullet)
    change: >-
      Drop "`viola revive --list`, which opens no process log" from the parenthesised list of bounded exemptions, and widen "a refused instance check on `viola revive`'s start arm" to a refused instance check on either `viola revive` arm (start or `--list`).
    sidecar: >-
      §2 panic bullet: revive --list leaves the exemption list; its refused check joins the refused-check exemption.
    rationale: >-
      Report Counts / qualifiers moved: `revive-nojson` / `list-nolog` stale at obs-plan.md:462; Symbols / APIs: the `--list` arm logs as `cli` after a passed check, a refused check writes no line on both arms.
    basis: obs-plan.md:462
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §7 Error Capture & Reporting (Panic hooks, Main-thread catch site)
    change: >-
      `role_of` files `revive` followed (after any `--home` pair) by `--list` before a `--` as `cli`, and `revive` otherwise beside `run` (a `--list` after `--` is the child's word); so a `--list` dispatch `Err` or panic reaches the `cli` catch site (`process-exit{detail:"internal-error"}`, exactly `error: internal error`). Replace "`viola revive <name> --list` opens no process log and writes no line, so its two refusals (the strict-modes check, `no-session`) are the stderr pair and exit 1 only" with: its refused strict-modes check opens no log and writes no line (the stderr pair, or under `--json` the `strict-modes-failed` document, exit 1); its `no-session` is `process-exit{exit_code:1, detail:"no-session"}` in the caller's `cli-<VIOLA_NAME>.ndjson` under a name, with the stderr pair or the `no-session` document.
    sidecar: >-
      §7 catch site: role_of files `revive … --list` as cli; --list's no-session is logged under a VIOLA_NAME, its refused check still writes none.
    rationale: >-
      Report Symbols / APIs (`src/main.rs` `role_of`): "`revive` followed (after any `--home` pair) by `--list` before a `--` is `Role::Cli` … So a `--list` dispatch `Err` or panic reaches the `cli` catch site"; Counts / qualifiers moved: `role-of` stale at obs-plan.md:822, `list-nolog` stale at obs-plan.md:822.
    basis: obs-plan.md:822
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §10 SLO Invariants & Telemetry Budgets (Bounded exemptions, item 6)
    change: >-
      Retire exemption 6 (`viola revive <name> --list`, no role file): `--list` now opens the `cli` role log under a `VIOLA_NAME` after its check passes, so a panic there writes its one line; with no `VIOLA_NAME` it falls under exemption 2 (`cli` without a resolved instance), and its refused check under exemption 7. Keep the sentence that the start arm's role file is `run-<name>.ndjson` once its check has passed, moved under exemption 7 or 1.
    sidecar: >-
      §10 exemption 6 retired (revive --list logs as cli, chunk 2026-10-17-cli-machine-contract): no-name falls under exemption 2, a refused check under exemption 7.
    rationale: >-
      Report Counts / qualifiers moved: `list-nolog` stale at obs-plan.md:988 and `owner-title` stale at :988 ("owed to the working-route entry "CLI machine contract""); `exempt-count` — :989 stale with exemption 6's retirement, :987 no-change unless the list renumbers.
    basis: obs-plan.md:988
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §10 SLO Invariants & Telemetry Budgets (Bounded exemptions, item 7)
    change: >-
      Widen "a refused instance check on `viola revive`'s start arm" to a refused instance check on either `viola revive` arm (start or `--list`; the stderr pair or under `--json` the `strict-modes-failed` document, exit 1), and replace "it sits against the founder's direction named in exemption 6, a structured log from every process" with the direction stated in place (the founder's direction is a structured log from every process; the operator's answer at the chunk 2026-10-10-viola-revive wrap), since exemption 6 no longer exists.
    sidecar: >-
      §10 exemption 7: covers a refused check on both revive arms; its reference to exemption 6 restated in place.
    rationale: >-
      Report Counts / qualifiers moved: `list-nolog` stale at obs-plan.md:989 ("the founder's direction named in exemption 6"); `exempt-count` :989 stale with exemption 6's retirement; Symbols / APIs: "A refused check still opens nothing and writes no line, on both arms".
    basis: obs-plan.md:989
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §10 SLO Invariants & Telemetry Budgets (Bounded exemptions, item 1)
    change: >-
      The pre-init window names the instance check of both `viola revive` arms (the start arm and `--list`, whose check runs before its `cli` role file opens), beside `viola hook statusline`'s check and `viola list`'s scan.
    sidecar: >-
      §10 exemption 1: the pre-init window includes revive --list's strict-modes check.
    rationale: >-
      Report Symbols / APIs (`src/cmd/revive.rs`): `--list` opens the caller's log only "after its strict-modes check passes", so its check is now pre-init, no longer covered by the retired exemption 6.
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §7 Error Capture & Reporting (Panic hooks, the pre-init window bullet)
    change: >-
      The window before init step 4 holds the instance check that both `viola revive` arms (start and `--list`) and `viola hook statusline` run before any role file is opened.
    sidecar: >-
      §7 pre-init window: revive --list's check named beside the start arm's.
    rationale: >-
      Same fact as the §10 exemption 1 restatement: the `--list` check runs before the caller's `cli` log opens (report Symbols / APIs, `src/cmd/revive.rs`).
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §3 → OTel SDK init
    change: >-
      In the `run` role bullet, retire "`viola revive <name> --list` opens no process log."; state instead (or in the `cli` instance-source line) that `viola revive <name> --list` is role `cli`, its instance the caller's `VIOLA_NAME` (no name, no log), its log opened only after its strict-modes check passes.
    sidecar: >-
      otel-sdk-init: revive --list is role cli under the caller's VIOLA_NAME; the no-log sentence retired.
    rationale: >-
      Report Counts / qualifiers moved, `list-nolog`: stale at registries/contracts/obs-plan/otel-sdk-init.md:20; Deviations, step 12: "the log is the CALLER's `VIOLA_NAME`, as `list` and `verify` log under a name (obs-plan §4 cli row)".
    basis: .andromeda/registries/contracts/obs-plan/otel-sdk-init.md:20
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage (the `state-recovered` catalogue row)
    change: >-
      Add the field `scanned`: on the two snapshot details written by `read_snapshot_or_replay`, the instance dir's own last component when it parses as a `ViolaName` (schema pattern `^[a-z][a-z0-9-]{0,31}$`), absent otherwise — never a path or a literal `null`; `torn-line-healed` (the events.ndjson heal) carries no `scanned`.
    sidecar: >-
      §6 state-recovered row: `scanned` added (the scanned instance's ViolaName, absent otherwise; never on torn-line-healed), schemas/diag-line.v1.json.
    rationale: >-
      Report Symbols / APIs (`crates/viola-state/src/replay.rs`): "`state-recovered` line gains `scanned` … the other `state-recovered` emitter (events.rs, the torn-line heal) writes no `scanned`"; Schema / config: `diag-line.v1.json` `state-recovered` row `scanned` added; Expected amendments: "The §6 catalogue row for `state-recovered` names its fields without the `scanned` token … its fact is in **Schema / config**".
    basis: crates/viola-state/src/replay.rs:174-181
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage (Error paths, E5 snapshot corruption)
    change: >-
      Replace "the line carries the caller's instance, never the scanned one (owed to "CLI machine contract", the review card the founder approved)" with: the line carries `instance` the caller and `scanned` the scanned instance's name (the dir's last component when it is a `ViolaName`, absent otherwise); on revive's start arm `scanned` names the revived instance.
    sidecar: >-
      §4 E5: state-recovered carries `scanned` beside the caller's `instance`; the owed clause retired.
    rationale: >-
      Report Counts / qualifiers moved, pattern `scanned`: stale at obs-plan.md:656 (and `owner-title` at :656); Outcome (obs): "`state-recovered` under a `VIOLA_NAME` carries `instance` the caller and `scanned` the scanned name, valid against the schema → MET".
    basis: obs-plan.md:656
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage (detail catalog, `process-exit` exit 1)
    change: >-
      Replace "from `viola revive`, which prints `run`'s own collision pair" with: from `viola revive`, which prints `run`'s own collision pair, or under `--json` the `{"v":1,"error":"already-live","detail":null}` document (`collision_check`'s `json` argument); the preflight's `no-session` / `session-live` / `cwd-missing` refusals under `--json` are the same-shaped document with their code, the log line unchanged.
    sidecar: >-
      §6 process-exit exit 1: revive's refusals under --json print the exit-1 error document; the logged process-exit unchanged.
    rationale: >-
      Report Symbols / APIs (`src/cmd/revive.rs`, `src/cmd/run.rs`, `src/cmd/client.rs`): `ReviveArgs` gains `--json` on both arms; `Refusal::write(name, json)` writes `{"v":1,"error":"<detail>","detail":null}`; `collision_check(…, json)` writes the `already-live` document; Counts / qualifiers moved, pattern `revive-nojson`.
    basis: src/cmd/client.rs:120-124
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage (detail catalog, Filesystem refusals)
    change: >-
      "`viola revive`'s start arm and `viola revive --list` both refuse on `viola_state::strict::check_instance` with the stderr pair and exit 1 and no log line" becomes: … with the stderr pair, or under `--json` the `strict-modes-failed` document, exit 1, and no log line.
    sidecar: >-
      §6 Filesystem refusals: revive's refused check under --json is the strict-modes-failed document, still no line.
    rationale: >-
      Report Symbols / APIs (`src/cmd/revive.rs`): the codes are `Refusal::detail` (`strict-modes-failed`, …) under `--json`; "A refused check still opens nothing and writes no line, on both arms".
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §1 Obs Scope Summary (the exit-cause bullet, `viola revive`'s strict-mode fail)
    change: >-
      "`viola revive`'s strict-mode fail carries no `detail` code as landed: it is the stderr pair and exit 1 with no log line" becomes: it carries no logged `detail` (no log line); its output is the stderr pair, or under `--json` the `strict-modes-failed` document, exit 1.
    sidecar: >-
      §1 exit-cause bullet: revive's strict-mode fail names `strict-modes-failed` in its --json document; still no log line.
    rationale: >-
      Report Symbols / APIs (`src/cmd/revive.rs`): `Refusal::write(name, json)` under `--json` writes the code document, the codes including `strict-modes-failed`.
    dependent-of: D-obs-instrumentation
note: >-
  D-obs-stack and D-obs-pii read no drift (Dependencies none; `scanned` is a ViolaName never a path, the revive --list lines carry codes only). The report names no dispatch span for `revive --list` (§4 cli row requires one per verb, `cli.<verb>`); not measurable from the report, so no proposal — worth a check at apply. Retiring exemption 6 either renumbers §10's list (exemption 7 becomes 6; obs-plan.md:987's "as in exemption 3" holds) or leaves item 6 as a retired marker; `.claude/rules/observability.md` cites exemption 7 (the cascade's). The `revive --json` post-preflight gap (run::start refusals print no document) is not stated in obs-plan and draws no proposal here.
