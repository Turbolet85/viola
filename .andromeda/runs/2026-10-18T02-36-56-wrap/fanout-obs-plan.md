proposals:
  - detector: D-obs-instrumentation
    severity: warning
    section: §3 Observability Harness Contract (the `run` stderr-refusal bullet)
    change: 'The root bin''s human-stderr refusal writer (`src/human.rs` `refuse` → `write_refusal`) is called by `run` (5 sites), `verify` (5), `list` (2), `revive` (1) and `plugin install` (1), 14 in all, as re-derived by `grep -rcE ''human::refuse\('' src` at chunk 2026-10-17-optional-plugin-install; `plugin install`''s one call prints `human::home_state_pair` on a refused tree check (exit 21, stdout empty).'
    sidecar: 'obs-plan §3: the `human::refuse` caller census re-derived 13 → 14, `plugin install` 1 (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'Report Changes → Counts / qualifiers moved: "The `human::refuse` caller census: 13 → 14, `plugin install` 1 (pattern `refuse-census`: 1 standing master row, `obs-plan.md:505`, which states \"run (5 sites), verify (5), list (2) and revive (1)\", stale)"; Symbols → Remaining callers and the verb''s order step 3; Expected amendment 11.'
    basis: 'obs-plan.md:505'
  - detector: D-obs-instrumentation
    severity: warning
    section: §11 Obs Anti-Patterns → Logs (the print-macro ban''s `src/human.rs` exception)
    change: 'The refusals that write through `src/human.rs` (`refuse` / `write_refusal`) are `run`''s and `revive`''s pre-spawn refusals, `revive --list`''s, `verify`''s, `list`''s and `plugin install`''s; callers `run` 5 · `verify` 5 · `list` 2 · `revive` 1 · `plugin install` 1 (14, `grep -rcE ''human::refuse\('' src`, chunk 2026-10-17-optional-plugin-install).'
    sidecar: 'obs-plan §11 Logs: the `src/human.rs` refusal-writer callers gain `plugin install` 1 (14 in all), the duplicate of §3''s census.'
    rationale: 'The same caller census §3 states is restated in the print-macro ban with the old four callers and no `plugin install`; report Counts / qualifiers moved (refuse census 13 → 14) and Symbols → Remaining callers.'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage → Edge flows → `verify` / `plugin install` (the dispatch-spans bullet)
    change: '`cli.plugin_install` is not a dispatch span: it is declared on `write_tree` (`#[instrument(skip_all, name = "cli.plugin_install")]`) and opened after the `cli` log opens, so the tree check and its refusal run outside it (a span opened before the subscriber exists is a no-op), as `list` puts `cli.list` on its read; `verify`''s `cli.verify` / `cli.verify_step` / `state.ledger_write` stand.'
    sidecar: 'obs-plan §4 verify / plugin install: `cli.plugin_install` written as built, on `write_tree` after the log opens, not on the dispatch (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'Report Symbols → the verb''s order step 5 (`write_tree` carries `#[instrument(skip_all, name = "cli.plugin_install")]`, "the span opened after the log") and Deviations from intent 2 ("The span `cli.plugin_install` sits on `write_tree`, after the log opens, not on the whole dispatch"); Expected amendment 9 (`obs-plan.md:662-664`). The body names it among "Dispatch spans".'
    basis: 'src/cmd/plugin.rs:102-118'
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage → Instrumentation per surface (the `cli (short-lived verbs)` row)
    change: 'Beside the `revive --list` and `list` shapes: `viola plugin install` (span `cli.plugin_install` on `write_tree`, opened after the log, chunk 2026-10-17-optional-plugin-install) opens the caller''s `cli-<name>.ndjson` only after its strict-modes tree check has passed and only under a parsed `VIOLA_NAME`, writing `process-start{subject:"self"}` and `process-exit` (`exit_code` 0, no detail); with no name no log, and a refused tree check (exit 21) opens no process log and writes no line.'
    sidecar: 'obs-plan §4 cli row: `plugin install`''s log shape and its span placement added (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'The row''s "A dispatch span per verb (`cli.<verb>`)" restates the dispatch-span claim the primary retires for `cli.plugin_install`, and the row enumerates each verb''s no-log refusal (`revive`, `list`) without `plugin install`; report Symbols → the verb''s order steps 2-7 ("This runs before anything under the home is created, written or opened"; "Under a parsed `VIOLA_NAME` only, `client::start` opens `cli-<name>.ndjson` and writes `process-start{subject:\"self\"}`"; `run::log_self_exit(0, None)`) and Coverage of new surfaces.'
    basis: 'src/cmd/plugin.rs:48-76'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §4 Span / Trace Coverage → Edge flows → `verify` / `plugin install` (the logging bullet)
    change: 'Add for `plugin install`: under `VIOLA_NAME` its pair is `process-start{subject:"self"}` then `process-exit` exit 0 with no detail, written only once its strict-modes tree check (`check_path` on the home and each existing marketplace dir) has passed; a refused check opens no log and writes no line (exit 21; the stderr pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`); its agent-readable outcome is the exit code (0 / 1 / 2 / 21) plus, under `--json`, `{"v":1,"ok":{"marketplace":"viola","plugin":"viola@viola"}}`, no path in either document or any line.'
    sidecar: 'obs-plan §4 verify / plugin install: `plugin install`''s self pair, its no-line refused check and its exit / `--json` outcome written as built (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'The bullet says "With `VIOLA_NAME` set they log `process-start` / `process-exit{exit_code, detail}`" for both verbs and states only `verify`''s outcome; report Symbols → the verb''s order steps 2-7, Coverage of new surfaces ("PII no path in a `--json` document or a line"), Expected amendments 9 and 10 (`obs-plan.md:662-664`).'
    basis: 'src/cmd/plugin.rs:50-60'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §10 SLO Invariants & Telemetry Budgets → Bounded exemptions (item 7)
    change: 'Exemption 7 also holds `viola plugin install`''s refused strict-modes check (`viola_state::strict::check_path` on the home and each existing `marketplace/` tree dir, chunk 2026-10-17-optional-plugin-install, its plan''s lean, approved by the operator at P5 (`inputs#I3`), not ruled by the founder): it opens no role file and writes no line (exit 21, nothing created; the stderr `home_state_pair`, or under `--json` the `instance-unreachable` document).'
    sidecar: 'obs-plan §10 exemption 7: gains `plugin install`''s refused tree check, no role file and no line (the plan''s lean, operator-approved at P5, not the founder''s ruling).'
    rationale: 'Report Expected amendment 10 ("it gains `plugin install`''s refused strict-modes check, which opens no role file and writes no line. This is the plan''s lean, approved with the plan by the operator at P5 (`inputs#I3`), not ruled by the founder"); Symbols → the verb''s order steps 2-3; Counts / qualifiers moved (`exemption7`: "the exemption''s own list stands at §10 item 7"; `obs-plan.md:988`, the retired sixth''s gloss, no change).'
    basis: 'src/cmd/plugin.rs:78-87'
  - detector: D-obs-instrumentation
    severity: warning
    section: §10 SLO Invariants & Telemetry Budgets → Bounded exemptions (item 1, the pre-init window)
    change: 'The pre-init window also holds `viola plugin install`''s strict-modes tree check, which runs before its `cli` role file opens (chunk 2026-10-17-optional-plugin-install).'
    sidecar: 'obs-plan §10 exemption 1: the pre-init window gains `plugin install`''s tree check, the duplicate of exemption 7''s new entry.'
    rationale: 'Exemption 1 enumerates each check that runs before its role file opens (`revive`, `hook statusline`, `list`, `mcp`) and restates exemption 7''s mechanism; report Symbols → the verb''s order step 2 ("This runs before anything under the home is created, written or opened") and step 4.'
    dependent-of: D-obs-instrumentation
  - detector: D-obs-instrumentation
    severity: warning
    section: §6 Log Coverage → `detail` code catalog (Filesystem refusals)
    change: 'Beside `viola list`''s refused scan: `viola plugin install`''s refused `check_path` of the home or a marketplace tree dir exits 21 and writes no line (no `process-start`, no `process-exit`), its `--json` form the `instance-unreachable` document, never `strict-modes-failed` (chunk 2026-10-17-optional-plugin-install); so on `process-exit` exit 21 no arm of this build writes the word still holds.'
    sidecar: 'obs-plan §6 Filesystem refusals: `plugin install`''s no-line exit-21 refusal added (chunk 2026-10-17-optional-plugin-install).'
    rationale: 'The Filesystem-refusals entry enumerates the strict-modes refusals that write no line (`revive`, `hook statusline`, `list`) and restates exemption 7''s claim; report Symbols → the verb''s order step 3 ("A refusal is exit 21, with nothing created and no log opened ... `--json` prints exactly `{\"v\":1,\"error\":\"instance-unreachable\",\"detail\":null}`").'
    dependent-of: D-obs-instrumentation
note: 'D-obs-stack and D-obs-pii fire nothing: no dependency added (Cargo.lock unchanged), the span is `skip_all`, and the report states no path in a `--json` document or a line. Not a logging drift but worth a look: `plugin install`''s human-mode stdout prints the canonical `<home>/marketplace` path by design (Coverage of new surfaces), which §8''s Medium row ("Absolute paths ... scrub-required in external output") does not carve out; whether a success line on human stdout counts as external output is the orchestrator''s / security-plan''s call, not an obs-plan amendment proposed here.'
