proposals:
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: cli → Output structure — `viola verify`'
    change: 'Add the interrupted form beside exit 0 / 1: a third fenced example `$ viola verify` (stopped) printing on stderr `unable: verify was stopped before it finished` then `hint: run viola verify again to stamp this CLI` (exit 1, stdout empty, no step line, no `stamped` summary, no stamp written), and in the prose after "1 on a failing row or a refusal": "a run stopped by SIGINT / SIGTERM (Windows: Ctrl-C / Ctrl-Break) before its rows are checked prints only that plain ASCII pair, with no name, path or pid, and exits 1 (a planning form on the operator''s word, the founder''s wording owed, inputs#I2; real Windows console delivery owed to "Windows-only live measurements")".'
    sidecar: 'layout-templates §Surface: cli, Output structure — `viola verify`: the interrupted form added (the stderr pair, exit 1, empty stdout, no summary) — chunk 2026-10-13-interrupted-verify-cleanup.'
    rationale: 'The report''s Changes add a new user-facing output region to `viola verify`: `verify` (src/cmd/verify.rs 69-78) prints the pair through `human::refuse` and exits 1, listed under Coverage of new surfaces as "The interrupted stderr pair (CLI)"; the Outcome measures stdout empty, stderr exactly the pair, exit 1 and no `ledger/stamps.json`. The section shows only the exit 0 / exit 1 completed forms; Expected amendment 9 names this section and its `verify-output` site :496.'
    basis: .andromeda/layout-templates.md:496
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: cli → Component — Primary content block 2: refusal lines and the `unable` column'
    change: 'In the Line form bullet, add to `viola verify`''s exit-1 pairs (after the recorded-fixture pair) `verify was stopped before it finished` / `run viola verify again to stamp this CLI` (a run stopped by SIGINT / SIGTERM, Windows Ctrl-C / Ctrl-Break; stdout empty).'
    sidecar: 'layout-templates §Surface: cli, Primary content block 2: the interrupted `verify` pair joins verify''s exit-1 fixed-message pairs — chunk 2026-10-13-interrupted-verify-cleanup.'
    rationale: 'The bullet enumerates `viola verify`''s exit-1 `unable:` / `hint:` pairs with no name, path or pid; the report''s Symbols / APIs add a new one (`verify` prints `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`, exit 1). Expected amendment 9 lists this `unable-pair` site at :588.'
    basis: .andromeda/layout-templates.md:588
    dependent-of: D-layout-surface
  - detector: D-layout-surface
    severity: warning
    section: '§Surface: cli → Component — Footer / terminator'
    change: 'The `verify` bullet should read: "On `verify`, the summary `stamped <version>  <n> pass  <n> fail` closes a completed run; a run stopped before its rows are checked prints no summary, and its `hint:` line is the last line on stderr."'
    sidecar: 'layout-templates §Surface: cli, Footer / terminator: the `stamped` summary closes only a completed `verify`; the interrupted run ends on its hint line — chunk 2026-10-13-interrupted-verify-cleanup.'
    rationale: 'The bullet states the summary always closes a `verify` run; the report''s Outcome measures an interrupted run with stdout empty and stderr exactly the pair, so no summary is printed (measure reads the stop before `check_rows`, src/cmd/verify.rs 242-244). Expected amendment 9 lists this `verify-output` site at :598.'
    basis: .andromeda/layout-templates.md:598
    dependent-of: D-layout-surface
note: 'The `unable-pair` sites :590 (every exit-1 cause has its own hint) and :520 / :535 / :539 / :541–:545 (run / revive) hold unchanged; §Primary screens (commands) `viola verify` stamps a CLI version with a step counter still holds.'
