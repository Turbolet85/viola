proposals:
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 158
    section: '## 1. Obs Scope Summary'
    change: 'Append to the cli Exporter item: `viola link` / `viola unlink` log as `cli` into the DRIVEN instance''s `cli-<driven>.ndjson` (the instance named by the `<driven>` argument, never the caller''s `VIOLA_NAME`), and only once their `check_instance` of that instance has passed; a refused check opens none (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'obs-scope-summary cli Exporter: link / unlink log into cli-<driven>.ndjson after their instance check; a refused check opens no file.'
    rationale: 'Report Changes, Symbols / APIs, link_call steps 1 and 2: check_instance on the driven instance before client::start(home, &driven), which opens the cli-<driven> role file and writes process-start; task Touches, the cli-<name> producers.'
    basis: src/cmd/link.rs:98-106
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 397
    section: '## 1. Obs Scope Summary'
    change: 'In the Vector 7 refusal-telemetry item, "Four strict-modes refusals leave no line" becomes five: add `viola link` / `viola unlink`''s instance check of the driven instance (`viola_state::strict::check_instance`, run before any role file or log opens), which refuses with exit 21 and writes no line (chunk 2026-10-26-explicit-session-links); the rest unchanged.'
    sidecar: 'obs-scope-summary Vector 7: the link / unlink instance check joins the strict-modes refusals that leave no line (four to five, the named list).'
    rationale: 'Report Changes, link_call step 1: a refused check_instance is exit 21 with no line in any log; report Decisions: the founder''s form (inputs#I3), no dated gap.'
    basis: .andromeda/obs-plan-detail/obs-scope-summary.md:397
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 488
    section: '## 1. Obs Scope Summary'
    change: 'The server-verify fail item names the eight CLI frame verbs: written as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` in `cli-<name>.ndjson` by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (for `link` / `unlink` the driven instance''s file); the rest unchanged.'
    sidecar: 'obs-scope-summary Founder Direction 4: server-verify-failed writers six to eight (link, unlink).'
    rationale: 'Report Counts, frameverbs: CLI frame verbs six to eight, each writing server-verify-failed on a mismatch; link_call step 5.'
    basis: src/cmd/link.rs:140
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 487
    section: '## 1. Obs Scope Summary'
    change: 'After the `viola revive` strict-mode sentence add: `viola link` / `viola unlink`''s strict-mode fail (their `check_instance` of the driven instance) carries no logged `detail` either (no line): its output is the fixed instance-state `unable` / `hint:` pair, or under `--json` the `{"v":1,"error":"instance-unreachable","detail":null}` document, exit 21 (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'obs-scope-summary Founder Direction 4: the link / unlink strict-mode fail is exit 21 with no logged detail.'
    rationale: 'Report Changes, link_call step 1 (refused: exit 21, human::instance_state_pair, the --json instance-unreachable document, no line in any log); report Decisions: exit 21 as list / plugin install, not revive''s exit 1.'
    basis: src/cmd/link.rs:98-106
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-scope-summary.md
    near: 497
    section: '## 1. Obs Scope Summary'
    change: 'Higher-`v` rejections item: the `-32602` code also carries `send`''s invalid params, `release` carrying `from`, and a served `link` / `unlink` whose `driver` or `driven` is missing, not a string or no valid name, whose `driven` is not the serving wrapper''s own name, or whose `driver` equals `driven`, so it is not a higher-`v` count alone.'
    sidecar: 'obs-scope-summary compat trigger: -32602 gains the link / unlink bad-name and bad-pair causes.'
    rationale: 'Report Changes, Symbols / APIs: driver_of answers ProtocolError::InvalidParams (-32602) on each bad name or pair; Counts, vrejected +2 causes.'
    basis: src/run/links.rs:85-99
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/telemetry-strategy.md
    near: 20
    section: '## 2. Telemetry Strategy'
    change: 'In the parenthesised bounded-exemption list, after "or a refused `viola list` scan, which opens none either" add "or a refused instance check on `viola link` / `viola unlink` (the driven instance, checked before `client::start` opens its role file)".'
    sidecar: 'telemetry-strategy panic exemptions: the refused link / unlink instance check opens no role file.'
    rationale: 'Report Changes, link_call step 1: check_instance runs BEFORE client::start, so before any role file or log opens; task Touches, the refusing-verb enumerations (telemetry-strategy line 20).'
    basis: .andromeda/obs-plan-detail/telemetry-strategy.md:20
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 8
    section: '## 4. Span / Trace Coverage'
    change: 'Add `cli.link` and `cli.unlink` to the `CLIENT` list: the dispatch spans `viola link` / `viola unlink` open (`#[instrument(skip_all, name = "cli.link")]` / `name = "cli.unlink"`) around their one request; a server-verify refusal is logged inside it.'
    sidecar: 'span-trace-coverage span kinds: CLIENT gains cli.link and cli.unlink.'
    rationale: 'Report Changes, link_call step 5: one request through cmd::client::connect under #[instrument(skip_all, name = "cli.link")] / "cli.unlink"; Outcome (obs): refusal inside cli.link / cli.unlink.'
    basis: src/cmd/link.rs:85-136
    fact: F1
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 18
    section: '## 4. Span / Trace Coverage'
    change: 'In the cli row''s Manual instrumentation cell, after the `viola plugin install` sentence add: `viola link` / `viola unlink` (spans `cli.link` / `cli.unlink`, chunk 2026-10-26-explicit-session-links) run `check_instance` on the driven instance before any role file opens, then open the driven instance''s `cli-<driven>.ndjson` and write `process-start` / `process-exit`; a refused check (exit 21) opens no process log and writes no line.'
    sidecar: 'span-trace-coverage cli row: link / unlink log into cli-<driven>.ndjson only after their instance check passes.'
    rationale: 'Report Changes, link_call steps 1 and 2 and Coverage of new surfaces (span cli.link / cli.unlink, process-start / process-exit); task Touches, span naming and the cli-<name> producers.'
    basis: src/cmd/link.rs:98-106
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 28
    section: '## 4. Span / Trace Coverage'
    change: 'The server-check item reads "each of the eight frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`)"; `link` / `unlink` take the check after their `check_instance` of the driven instance and the liveness pre-check; the rest unchanged.'
    sidecar: 'span-trace-coverage cli server check: frame verbs six to eight (link, unlink).'
    rationale: 'Report Counts, frameverbs (span-trace-coverage.md:28 stale) and connect (callers 6 to 8, span-trace-coverage.md:28 stale).'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:28
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 177
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'E3: replace "As landed (chunk 2026-10-10-session-links) only the implicit link exists." with "The implicit link landed first (chunk 2026-10-10-session-links)." and replace the closing sentence ("The `method:"link"|"unlink"` pair above belongs to the explicit methods, which are not built: ... and no product path appends an `unlink` line.") with "The `method:"link"|"unlink"` pair above belongs to the explicit methods, served by the driven wrapper since chunk 2026-10-26-explicit-session-links; the `unlink` method (`LinkSet::remove`, its first product caller) is the product path that appends an `unlink` line."'
    sidecar: 'span-trace-coverage E3: the explicit link / unlink hold retired (both served; a product unlink line is written).'
    rationale: 'Report Retired: the hold, the -32601 answer for link / unlink; Symbols / APIs: link / unlink served, LinkSet::remove has its first product caller.'
    basis: .andromeda/obs-plan-detail/span-trace-coverage.md:177
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 177
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'Add to E3: the driven wrapper serves `link` / `unlink`, reading `driver` / `driven` through `ViolaName::try_new` with the own-name rule (`driven` is the serving wrapper''s name) and the self-pair rule (`driver` is not `driven`); a refusal is `-32602` with `data: null` and no line written, a record or remove error `-32603`; neither method reads `from`.'
    sidecar: 'span-trace-coverage E3: the served methods'' name validation and error codes.'
    rationale: 'Report Changes, Symbols / APIs: LinkSet::link / unlink share driver_of; InvalidParams with data null and no line; record / remove error ProtocolError::Internal; neither reads from.'
    basis: src/run/links.rs:85-99
    fact: F2
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 177
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'Add to E3: a served `link` / `unlink` opens no span and writes no process-log line of its own; its trace is the wrapper''s existing `channel-request` / `channel-response` pair with `method:"link"|"unlink"`, which holds no driver or driven name, then the product line and the snapshot''s `links`.'
    sidecar: 'span-trace-coverage E3: the served methods are traced by the channel pair alone, no names in it.'
    rationale: 'Report Changes, Symbols / APIs (neither opens a span nor writes a process-log line); Outcome (obs): link_verbs_log_the_channel_pair_and_no_names.'
    basis: src/cmd/run.rs:144-145
    fact: F3
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/span-trace-coverage.md
    near: 177
    section: '## 4. Span / Trace Coverage › #### Scenario: Budget governor'
    change: 'Add to E3: `unlink` appends a product `unlink` event line (`source: wrapper`, `data: {driver, driven}`) only when a pair is removed; a second `link` of a held pair and an `unlink` of an absent pair append nothing.'
    sidecar: 'span-trace-coverage E3: when the product unlink line is written, and when neither method appends.'
    rationale: 'Report Changes, Symbols / APIs: a second link of a held pair appends nothing (already_linked true); unlink of an absent pair appends nothing; a product path now appends an unlink line.'
    basis: src/run/links.rs:78-83
    fact: F4
  - detector: D-retired
    severity: warning
    file: .andromeda/obs-plan.md
    near: 392
    section: '## 4. Span / Trace Coverage'
    change: 'E3 broad line: the implicit link (recorded by the driven wrapper inside an issued `send` or a taken `answer`) and the explicit `link` / `unlink` methods (served by the driven wrapper, chunk 2026-10-26-explicit-session-links) have no process-log line, no `event` value and no span of their own; their trace is the `channel-request` / `channel-response` pair, the product `link` / `unlink` line in `events.ndjson` and the snapshot''s `links`.'
    sidecar: 'obs-plan §4 E3 broad line: the explicit methods are built; the hold retired.'
    rationale: 'Report Retired: the hold and every master line stating it; Expected amendments, obs-plan §4 E3 (obs-plan.md:392 stale).'
    basis: .andromeda/obs-plan.md:392
    dependent-of: D-retired
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/metric-coverage.md
    near: 28
    section: '## 5. Metric Coverage'
    change: 'In the over-count note: `-32602` also answers `send`''s invalid params (`ProtocolError::InvalidParams`, `"invalid params"`), `release` carrying `from`, and a served `link` / `unlink` with a missing, non-string or invalid name, a `driven` that is not the serving wrapper or a `driver` equal to `driven`; the rest of the row unchanged.'
    sidecar: 'metric-coverage v_rejected: -32602 over-count gains the link / unlink bad-name and bad-pair causes.'
    rationale: 'Report Counts, vrejected: +2 causes, metric-coverage.md:28 stale.'
    basis: .andromeda/obs-plan-detail/metric-coverage.md:28
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan.md
    near: 418
    section: '## 5. Metric Coverage'
    change: '`viola.compat.v_rejected` over-counts: `-32602` also answers `send`''s invalid params, `release` carrying `from` and a bad `link` / `unlink` name or pair, until the line tells the causes apart.'
    sidecar: 'obs-plan §5 broad line: v_rejected over-count gains link / unlink.'
    rationale: 'Report Counts, vrejected: obs-plan.md:418 stale.'
    basis: .andromeda/obs-plan.md:418
    dependent-of: D-cascade
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 13
    section: '## 6. Log Coverage'
    change: 'In the `channel-response` row the `method` field reads "`method` (only for a name on the channel''s closed method-label list, `link` and `unlink` among them; an unserved method''s conn-less line carries no `method` key)"; the other fields unchanged.'
    sidecar: 'log-coverage channel-response: method logged only for a closed-list name; an unserved method logs none.'
    rationale: 'Report Spec claims disproved: the channel names only the closed method list (method_label), so an unserved method logs no method key; Deviations, implement 2.'
    fact: F7
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 29
    section: '## 6. Log Coverage'
    change: '`server-verify-failed` is written by the eight CLI frame verbs (`send`, `wait`, `last`, `answer`, `pause`, `release`, `link`, `unlink`; `link` / `unlink` into the driven instance''s `cli-<driven>.ndjson`) on a server that is not the recorded wrapper, or a check that cannot be made; the rest unchanged.'
    sidecar: 'log-coverage exit-21 detail: server-verify-failed writers six to eight.'
    rationale: 'Report Counts, frameverbs and srvverify: log-coverage.md:29 stale.'
    basis: .andromeda/obs-plan-detail/log-coverage.md:29
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/log-coverage.md
    near: 35
    section: '## 6. Log Coverage'
    change: 'In the Vector 7 `strict-modes-failed` parenthesis, after the `viola plugin install` clause add: `viola link` / `viola unlink`''s refused `check_instance` of the driven instance exits 21 and writes no line either (no `process-start`, no `process-exit`), the fixed instance-state pair on stderr or under `--json` the `instance-unreachable` document (chunk 2026-10-26-explicit-session-links).'
    sidecar: 'log-coverage Vector 7: the link / unlink refused instance check writes no line.'
    rationale: 'Report Changes, link_call step 1; Outcome (security): link_verbs_refuse_a_widened_instance_before_any_log.'
    basis: src/cmd/link.rs:98-106
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/error-capture-reporting.md
    near: 23
    section: '## 7. Error Capture & Reporting'
    change: 'Extend the pre-init window: and the `check_instance` that `viola link` / `viola unlink` run on the driven instance before `client::start` opens the `cli-<driven>` role file (chunk 2026-10-26-explicit-session-links); a refusal is exit 21 `instance-unreachable` (human the fixed instance-state pair, `--json` `{"v":1,"error":"instance-unreachable","detail":null}`) with no line in any log and the mode as found.'
    sidecar: 'error-capture-reporting pre-init window: the link / unlink instance check joins it.'
    rationale: 'Report Changes, link_call step 1: check_instance BEFORE client::start, refused exit 21, no line in any log, the directory mode as found.'
    basis: src/cmd/link.rs:98-106
    fact: F5
  - detector: D-new-detail
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 11
    section: '## 10. SLO Invariants & Telemetry Budgets'
    change: 'Exemption 1: add to the pre-init window "and the instance check of `viola link` / `viola unlink` on the driven instance, which runs before its `cli` role file opens (chunk 2026-10-26-explicit-session-links)"; no `process-start` precedes a refusal there, so its exit status is its only evidence.'
    sidecar: 'slo exemption 1: the link / unlink instance check joins the pre-init window.'
    rationale: 'Report Changes, link_call step 1 (before client::start, which writes process-start); task Touches, §10 exemption 7 and the §7 pre-init window.'
    basis: src/cmd/link.rs:98-106
    fact: F6
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md
    near: 17
    section: '## 10. SLO Invariants & Telemetry Budgets'
    change: 'Exemption 7: add "or a refused `check_instance` of the driven instance on `viola link` / `viola unlink` (chunk 2026-10-26-explicit-session-links, the founder''s form, inputs#I3)" to the refused checks, and "link and unlink: exit 21, the fixed instance-state pair or under `--json` the `instance-unreachable` document" to the parenthesised outcomes.'
    sidecar: 'slo exemption 7: the refused link / unlink instance check opens no role file and writes no line.'
    rationale: 'Report Changes, link_call step 1; Decisions: the founder''s word inputs#I3, exit 21 as list / plugin install; task Touches, slo-invariants-telemetry-budgets line 17.'
    basis: .andromeda/obs-plan-detail/slo-invariants-telemetry-budgets.md:17
  - detector: D-cascade
    severity: warning
    file: .andromeda/obs-plan-detail/obs-anti-patterns.md
    near: 8
    section: '## 11. Obs Anti-Patterns (NEVER do these) › ### Logs'
    change: 'Add `link`''s and `unlink`''s instance-check refusal to the refusals written through `src/human.rs`, and the census to "callers `run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1 · `link` 1 (the one `link_call` site both verbs share), chunk 2026-10-26-explicit-session-links"; the rest unchanged.'
    sidecar: 'obs-anti-patterns Logs: human::refuse census 14 to 15 (link 1).'
    rationale: 'Report Counts, refuse: human::refuse census 14 to 15 (link.rs 1), by grep -rcE over src; Symbols / APIs, link_call step 1 refuses through human::refuse.'
note: obs-plan.md:311 (§3, no picked entity details it) states the same human::refuse census without link (report Counts, refuse, stale) and is the body pass's; F8 (the link line's since outside the Clock seam) has no picked log-coverage entity to carry it and is the new-detail pass's.
