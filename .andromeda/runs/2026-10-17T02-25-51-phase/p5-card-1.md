# P5 review — 2026-10-17-cli-machine-contract · CLI machine contract

Plan: `viola-0.1.0/chunks/2026-10-17-cli-machine-contract/plan.md` (17 steps in 5 parts, 21 gate entries, 9 acceptance
criteria) · scope `…/scope.md` · research `…/research.md`. Mechanical checks: all pass (gate fence parses, planlint
0 hits, research lists parsed, `inputs.py verify` 0 unparsed). Validation-1: aligned with the scope and the working
entry.

## Goal
`viola revive` gains its `--json` on both arms. It is the one verb without a machine form; the other seven already
have one. `revive --list` opens the `cli` role log, which retires obs-plan §10's sixth exemption. The home resolves as
`--home`, then the `VIOLA_DIR` grandparent, then the default, canonicalised once it exists. `state-recovered` names
the scanned instance. A completeness witness shows that no flag, `VIOLA_*` variable or `config.json` key switches off
a control, and one per-verb witness reads every verb's document and literal exit side by side. First, Part A removes
the cause of the last wrap's red CI (`ci#38994749835`, `test (ubuntu-latest)`, `resume-session` under the fake agent,
`inputs#I1`), which every `stamped_home` case rests on.

## The two forks, answered (`inputs#I2`, the Viola overseer, technical, founder-delegated)
- revive's exit-1 `--json`: `{"v":1,"error":"<code>","detail":null}`, with the codes strict-modes-failed ·
  already-live · no-session · session-live · cwd-missing.
- `--home` and the `VIOLA_DIR`-derived home are canonicalised now, written in the plain drive form on Windows.

## Leaned, not asked
- `run` and `verify` gain no `--json` (architecture [CLI Conventions] names revive as the one verb without a machine
  form).
- Reading `VIOLA_DIR` in the CLI verbs is not a boundary widening: architecture §Occupied Resources and Config
  management already name every CLI verb as its reader in the target state.
- `schemas/config.v1.json` is created as the "config schema" test-plan §5's completeness case reads; none exists today.

## Top acceptance criteria
1. Part A: the red's cause is named at a `file:line` in `evidence/ci-red.md`. The concurrent-start witness reads red
   with the fix reverted and green over 20 firings with it. CI reads green on `test (ubuntu-latest)`. No bound is
   raised, no retry added and no test serialised.
2. `revive --json`: every exit-1 refusal is the one `error` document with empty stderr; `--list --json` prints the
   sessions document; human pairs and rows are byte-identical.
3. `revive --list` writes its log pair after a passed check, and a refused check writes no line.
4. Resolution order and canonicalisation, proved by the units and the per-verb witness's home cases.
5. The completeness case: every `--help` long flag, every `VIOLA_*` variable and every schema key is in the table,
   and `CONFIG_KEYS` equals the schema's keys.

## Touchpoints
New: `tests/cli_machine_contract.rs`, `schemas/config.v1.json`. Modify: `src/cmd/{mod,hook,run,revive,client}.rs`,
`src/main.rs`, `crates/viola-state/src/replay.rs`, `schemas/diag-line.v1.json`, `src/obs.rs` (its test module only),
the test supports (`cli.rs`, `home.rs`, `start.rs`), `tests/{cli_controls_not_disableable,cli_revive,cli_list,
contract_diag_schema,cli_instance_state,cli_verify}.rs`. The red's site is provisional: one of `src/cmd/verify/typed.rs`,
`src/bin/viola-fake-agent.rs` or `tests/support/home.rs`.

## Test commands (baselines on the untouched tree)
- Lints and `run --unit`: seen green at the last wrap. The new-case filter is green on the 27 standing `role_of_` rows.
- The concurrent witness reads red (0 selected); the per-binary filter reads red (the new binary does not exist yet).
- `run --integration` is green (337 passed, 57 s). `run --e2e` is green (157 passed, 9 s).
- The boot / status / cleanup smoke is green (status `ready`).
- `pre-push`; then, by the operator, hygiene → push → `ci.py conclusion --sha HEAD --wait 1800` asserting
  `verdict: green`.

## Concretization preview — `verification-matrix.json#v1-39`
- **Current acceptance:** Every verb has a machine-readable form mirroring the channel result and a typed exit status
  from the documented set; human output follows the design phraseology (the readback mirror for send, one hint naming
  the next step per cause, results on stdout and context on stderr); a global home option isolates all state; the
  config file sets thresholds, port and diagnostics level, and no setting switches off a security control.
- **Proposed concrete form:** every driver-facing verb (`send`, `wait`, `last`, `answer`, `list`, `pause`, `release`,
  and `revive` with `--list`) has a `--json` form printing one stdout document that mirrors the channel result, or the
  documented `error` form where none exists. Stderr is empty and the exit is typed from {0, 1, 2, 10-14, 20, 21}, read
  side by side per outcome in `tests/cli_machine_contract.rs` and the landed per-verb cases it names. In human mode,
  results go to stdout and context to stderr, each refusal cause ends stderr with its own `hint:`, no hint names
  `release`, and `send` prints its readback mirror (the landed literal pins in `tests/cli_send.rs`,
  `tests/cli_wheel.rs`, `tests/cli_revive.rs`, `tests/cli_list.rs` and the trycmd mirror). A global `--home` isolates
  all state (the two-home case), resolved `--home` → `VIOLA_DIR` grandparent → default and canonicalised.
  `config.json` sets the thresholds and the diagnostics level, and no key, flag or `VIOLA_*` variable switches off a
  control (the completeness case over `schemas/config.v1.json`).
- **Narrowed, not weakened:** the `port` clause left 0.1.0 by the founder's 2026-10-16 ruling, recorded in the cap's
  own notes. Every other clause is kept and given a named witness.
- `v1-18` stays pooled. This chunk mints revive's five exit-1 codes, and "Exit-cause code catalogue" extends the form
  to the other verbs.

## Beyond the plan
none
