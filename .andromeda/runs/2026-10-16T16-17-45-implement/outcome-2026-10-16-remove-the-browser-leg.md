# Outcome — 2026-10-16-remove-the-browser-leg · implement

**green + smoke.** The block's second whole firing reads `entries 25 · green 22 · red 0 · recorded 0 · timeout 0 ·
not-run 3` (the three `leg = 'operator'` entries 23-25, the operator pass's). Trail:
`.andromeda/runs/2026-10-16T16-17-45-implement/gate-2026-10-16-remove-the-browser-leg.json`.
Next → the OPERATOR PASS (entries 23 hygiene, 24 push, 25 the `ci.py` read; the pre-CI commit between 23 and 24),
then `/andromeda-wrap-session`.

## Files changed
Scope read: `scope: clean — changed 36 · listed 36 · recorded 0 (companion 0 · mechanical 0 · in-intent 0 · widening 0)
· absorbed 6 · excluded 47` (`list e2e-web/ (dir) ×6`).
- Deleted: `e2e-web/` (six tracked files), `scripts/install-node.sh`, `scripts/npm-audit.sh`,
  `crates/viola-e2e/src/harness/run/browser.rs`.
- Modified: `.gitignore`, `.github/workflows/{ci,nightly,windows-mutants}.yml`,
  `crates/viola-e2e/src/harness/{run,gate,pre_push,status,cleanup,mod,boot,secret_scan,logs}.rs`,
  `crates/viola-e2e/src/harness/pre_push/linux.rs`, `crates/viola-e2e/src/bin/viola-harness.rs`,
  `crates/viola-core/src/obs.rs`, `src/obs.rs`, `schemas/diag-{line,detail}.v1.json`, `tests/contract_diag_schema.rs`,
  `scripts/agent-run.{sh,ps1}`, `.config/nextest.toml`, `deny.toml`, `deny-sync.toml`, `Cargo.toml`,
  `scripts/deny-probes.sh`.
- Matrix: `claimed by 2026-10-16-remove-the-browser-leg: 0` — no ref written (the plan claims none).

## Deviations from plan, with justifications
- `e2e-web/` held six tracked files, not five (`stub/pipe.html` the sixth); all six left. Its ignored outputs
  (`node_modules/`, `pw.json`, `pw-junit.xml`, `test-results/`) left with the directory, so the removed `.gitignore`
  block strands nothing untracked.
- `e2e-web/` was removed by the founder, not by this run: the host's deny rule refuses a recursive `rm`, and the
  move into `target/e2e-home.disk/` was refused by the auto-mode classifier. On the overseer's direction the block ran
  with it standing (entry 9 red on its six files alone), and entry 9 was re-fired green after the removal.
- Step 7: `pre_push_native_stages_run_tools_then_linux_tests_and_end_the_pass` asserts the `linux` keys
  `["run", "gate"]`, not the plan's `["gate", "run"]`: the document keeps insertion order (stage order), measured at
  the first firing.
- Step 6: `pre_push_every_native_call_carries_only_home_and_path`'s `fake.calls.len() > 10` (a bound the plan did not
  name) became `assert_eq!(fake.calls.len(), 9, …)`: the node and browser calls left, and the pass now makes nine.
- Step 5: `gate_fails_failed_and_skipped_suites` plants a coverage summary at the floors, so retargeting to `coverage`
  adds no `llvm-cov-summary.json` breach to the two the test asserts.
- Step 9: `url_token_takes_the_t_parameter` left with `url_token` (the plan named the function, not its test).
- Sweeps: `"ui"` over `crates/viola-e2e/src` — 0. `http-request|liveness|sse-` over `tests/` — 2 hits, both
  `tests/cli_list.rs` (`:90`, `:237`), the `viola list` document's own `liveness` field, no change (the plan expected
  0; the word is that contract's, not the obs vocabulary). The count-literal probe over the touched modules — every
  hit is another list (`CONFIG_KEYS` 5, the new `SUITES` 8, the Windows `LEDGER_ROWS` 19), none stale.

## Surfaced / stuck
- Nothing surfaced; nothing stuck.

## Process hygiene
| process | started by | final state |
|---|---|---|
| `viola-harness supervise` + the `builder` wrapper and its fake agent (session `p-rbl-smoke`) | a leg (entry 17, both firings) | terminated — entry 21 `processes_gone:true`, `endpoint_gone:true`; the host's process list holds no process whose executable lies under this repository |
| the `pre-push` children (`run --coverage`, `gate`) | a leg (entry 22, three firings) | terminated — same reading |
| `viola` processes of `~/dev/projects/additional/viola-lab/prototype/` | not this run (another tree's bridge) | left running — not this run's |

## Decisions & corrections
- The operator, as the overseer, 2026-10-16: "deletion is the founder's; the overseer has asked him to run it (6
  tracked files + 19 MB of generated node_modules/pw output, inventoried). Run the block now, then re-fire the
  absence guard once e2e-web/ is gone; journal it." — followed (journal.md).
- Guard refusals: the Bash guard refused a `cd` into a skill directory once (re-issued with absolute paths); the host
  deny rule refused a recursive `rm`, and the classifier refused the move-aside — the removal was left to the founder,
  not routed around.
- The operator's invocation: no live `claude` start in this chunk (none made); the Bash guard (G1, 16:17Z) refuses a
  pipeline tool's stdout sent to a file — every tool ran bare, background outputs read whole with Read.
