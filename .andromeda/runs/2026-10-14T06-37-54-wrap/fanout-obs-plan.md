proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage → Edge flows (the `verify` / `plugin install` bullet, its hidden capture-arm sub-bullet)"
    change: "Beside the `hook <event> --capture <DIR> [--answers <DIR>]` arm, name the second hidden capture arm, `hook statusline --capture <DIR> [--shell <WORD>]` (sole caller: `viola verify`'s Run B probe settings, Unix only), equally uninstrumented by design: no `VIOLA_*`, no obs init, no role or detail file, no `hook-invoked` / `hook-decision`, no channel, no snapshot, no `budget.json`, nothing printed and exit 0 on every path; it files the raw payload (stdin through `take(MAX_FRAME + 1)`, refused past `MAX_FRAME`) 0600 at the first free `StatusLine.<k>.json` under the absolute existing `<DIR>` (the probe dir's 0700 `typed/statusline/`) beside `StatusLine.<k>.shell` holding exactly `bin-sh` or `other`, the shell word itself written nowhere; `hook statusline` without `--capture` keeps its pass-through and its lines unchanged."
    sidecar: "§4 Edge flows: the hidden `hook statusline --capture --shell` arm (chunk 2026-10-13-owed-ledger-rows) added beside the `hook <event> --capture` arm as a second no-obs-init capture arm."
    rationale: "The report's Symbols / APIs add `capture_statusline` as a new arm of `src/cmd/hook.rs` before the statusline pass-through, with no obs init, no `VIOLA_*`, nothing printed and exit 0, and its Coverage of new surfaces marks instrumentation n/a under obs §10 exemption 5; §4 today describes only the `hook <event> --capture` arm filing `<probe>/captures/<Event>.<k>.json`, so the new uninstrumented arm and its files are unstated. The report's Expected amendments list obs-plan.md:740 (capture-arm)."
    basis: "src/cmd/hook.rs:404-435"
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage → Edge flows (the `verify` / `plugin install` bullet, its six-spawn and stdout sub-bullet)"
    change: "verify's stdout carries one `[NN/MM] <row id> <row words>  pass|fail` step line per relied ledger row, MM the relied count with its rule (`LedgerRow::relied()`, `ALL` filtered in `ALL` order: a row is relied unless it is one of the three statusline rows where `statusline::WRAPPED_HERE` is false; 21 on Unix, 18 on Windows), and the last line `stamped <version>  N pass  N fail` sums to MM; the six spawn pairs stay six, Run B's `verify-pty-probe` pair now spawning with one probe `--settings` on Unix, and no line carries the probe command, a payload or a path."
    sidecar: "§4 verify outcome: the step line is per relied row and MM is the relied count (21 Unix / 18 Windows, the `relied()` rule); Run B's one probe `--settings` noted on the unchanged six pairs."
    rationale: "The report's Counts / qualifiers moved: the `viola verify` step counter's MM is now the relied count (21 Unix, 18 Windows), sweep `step-counter` naming obs-plan.md:739; Symbols / APIs: `check_rows` walks `LedgerRow::relied()` and the step line's total is its length; Run B's `--settings` 0 to 1 (Unix only), spawn pairs still six; Outcome (obs): six spawn pairs, no line carries the probe command, a payload or a path. §4's 'one step line per ledger row' is false on Windows (21 rows, 18 lines)."
    basis: "src/cmd/verify/typed.rs:147-157"
  - detector: D-obs-instrumentation
    severity: warning
    section: "§10 SLO Invariants & Telemetry Budgets → Zero unlogged panics → Bounded exemptions (exemption 5)"
    change: "Exemption 5 names both hidden capture arms: `hook <event> --capture` and `hook statusline --capture` (which runs no instance check and no pass-through), each doing no obs init and so having no role file; a panic there writes no line and the hook still exits 0, as in exemption 3."
    sidecar: "§10 exemption 5 widened by name to the `hook statusline --capture` arm (chunk 2026-10-13-owed-ledger-rows)."
    rationale: "The report's Coverage of new surfaces cites 'no obs init by design, obs §10 exemption 5' for `hook statusline --capture`, while exemptions 1 and 7 tie `viola hook statusline` to an instance check the capture arm never runs; the report's Expected amendments list obs-plan.md:1070 (capture-arm)."
    basis: "src/cmd/hook.rs:404-435"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§2 Telemetry Strategy → Agent-readable invariants (the panic-line invariant's exemption list)"
    change: "The exemption list reads 'the no-obs-init hidden capture arms (`hook <event> --capture` and `hook statusline --capture`)' in place of 'the no-obs-init `hook --capture` arm'."
    sidecar: "§2 panic invariant: the exemption list names both no-obs-init capture arms."
    rationale: "Same claim as §10 exemption 5 restated in §2; the report's Expected amendments list obs-plan.md:512 (capture-arm)."
    basis: "src/cmd/hook.rs:404-435"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage → Instrumentation per surface (the cli (`viola hook`) row)"
    change: "`hook-invoked` / `hook-decision` except on the hidden capture arms (`hook <event> --capture` and `hook statusline --capture`: no obs init, no lines, §4 Edge flows)."
    sidecar: "§4 per-surface table, hook row: the line exception names both hidden capture arms."
    rationale: "Same claim restated in the per-surface table; the report's Expected amendments list obs-plan.md:583 (capture-arm)."
    basis: "src/cmd/hook.rs:404-435"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§3 Observability Harness Contract (the harness-pattern bullet on `hook` stdout)"
    change: "`viola hook statusline`'s stdout is the user's status-line command's output byte for byte (nothing when no command is recorded, on a refused instance, on oversize or malformed stdin, or under the hidden `--capture` arm, which prints nothing)."
    sidecar: "§3 hook stdout: the `hook statusline --capture` arm prints nothing."
    rationale: "The report's Symbols / APIs: the `hook statusline --capture` arm prints nothing on every path; the §3 bullet lists the statusline arm's empty-stdout cases without it."
    basis: "src/cmd/hook.rs:404-435"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§1 Obs Scope Summary (Surface: cli `viola hook <event>` and `viola hook statusline`, its Exporter line)"
    change: "stdout carries only the decision body for `viola hook <event>`, and for `viola hook statusline` only the user's status-line command's output; the two hidden `--capture` arms (`verify`'s probe capture) write no sink and print nothing, except `--answers`' probe decision body."
    sidecar: "§1 hook surface Exporter: the hidden capture arms named as writing no sink, `hook statusline --capture` printing nothing."
    rationale: "The report adds `hook statusline --capture` with no obs init (no `hook-<name>.ndjson` sink) and nothing printed; the §1 Exporter line states a file sink and the statusline command's output as the only stdout for every `viola hook statusline` invocation."
    basis: "src/cmd/hook.rs:404-435"
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: "§4 Span / Trace Coverage → Scenario: Budget governor (the statusline arm's required lines)"
    change: "Add to the no-line cases: none under the hidden `hook statusline --capture` arm, which runs no instance check, no shell-out and no `budget.json` write and opens no role file (§4 Edge flows)."
    sidecar: "Scenario Budget governor: the `hook statusline --capture` arm writes no statusline line."
    rationale: "The report's Symbols / APIs: the capture arm runs before the pass-through with no obs init, no `budget.json` and no snapshot; the scenario's statusline line list names only the missing-`VIOLA_NAME` and refused-instance no-line cases."
    basis: "src/cmd/hook.rs:404-435"
    dependent-of: D-obs-instrumentation
note: "D-obs-stack: no drift (report Dependencies: none added or bumped; no telemetry symbol). D-obs-pii: no drift — the chunk adds no log line (the capture arm writes no line; verify's lines carry no command, payload or path); the `workspace.repo.owner` handle kept in the two StatusLine fixtures is a fixture-hygiene matter obs §8 defers to security/tests ('Recorded fixtures: scrub-required (security/tests-owned)'), amended in test-plan §7 per inputs#I3/I4. The two §4 Edge-flows proposals are both primaries of D-obs-instrumentation; the six dependents follow the first (the capture-arm claim). Not proposed: §3 key otel-sdk-init (hook init) and the §7 / §10 exemption-1 pre-init wording, which describe the pass-through's instance check and stay true."
