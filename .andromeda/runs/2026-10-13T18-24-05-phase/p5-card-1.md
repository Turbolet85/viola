# P5 review — 2026-10-13-interrupted-verify-cleanup · Interrupted verify cleanup

Plan: `viola-0.1.0/chunks/2026-10-13-interrupted-verify-cleanup/plan.md`, 453 lines, of which about 150 are the
gate fence. Mechanical checks all pass: 20 gate entries parse, planlint reads 0 hits, research's lists parse
(7 items), and inputs read 0 unparsed. Validation-1 finds the plan aligned with scope.md and the working entry.

## Goal
A `viola verify` stopped by Ctrl-C or TERM ends its run cleanly. It ends the PTY or bounded child it is running with no
key, and removes only the dirs that run created, through their existing drop guards. It writes no stamp, no step line
and no `stamped` line. It prints the pair below, logs `process-exit{self, exit_code:1, detail:"interrupted"}` and exits
1. Older, foreign and SIGKILL or crash leftovers stay by hand. No owner record and no remover of an earlier run's dirs
is built (the founder's narrow entry).

Ctrl-C and TERM mean Unix `SIGINT` and `SIGTERM`, and Windows `CTRL_C_EVENT` and `CTRL_BREAK_EVENT`. A Windows close,
logoff and shutdown keep the OS default.

Also: verify's default program becomes `viola_agent_claude::PROGRAM`, and Runs C and D's `--settings` becomes
`SETTINGS_FLAG`. Both moves are byte-identical (the two CARRYs).

The stderr pair is a planning form on your word, with the founder's wording owed (`inputs#I2`):
```
unable: verify was stopped before it finished
hint: run viola verify again to stamp this CLI
```

## Top acceptance criteria
- On Unix, `SIGINT` and `SIGTERM` (two cases, against the fake agent, held in Run B's turn wait by running without
  `--turn-stop`) make verify exit 1 by itself. Afterwards:
  - none of that run's dirs is left: `<cwd>/.viola-verify-<pid>`, `ledger/probes/<pid>/`, the Run A temp dir;
  - the planted foreign `.viola-verify-0`, `.viola-verify-0-plan`, `viola-verify-foreign` and `ledger/probes/0/` are
    untouched;
  - stdout is empty and stderr is exactly the pair, plain;
  - there is no `stamps.json`;
  - the role file closes every spawn pair, ends with the interrupted self exit, holds no panic line, and every line
    validates against the schema.
- On the stop path no byte reaches a PTY child, and each child is reaped before its dir is removed.
- On Windows, the console routine, called directly on `windows-2025`, stops for Ctrl-C and Ctrl-Break and leaves Close to
  the OS. Event delivery to a running verify on Windows is not claimed.
- The uninterrupted run is unchanged (`binary(cli_verify)`, `binary(cli_version_gate)`). The literal probe reads 0. No
  dependency, feature, seam or environment variable is added (the preservation guard, `cargo deny check`).

## Touchpoints
- New: `src/cmd/verify/stop.rs`.
- Modify:
  - `src/cmd/verify.rs`;
  - `src/cmd/verify/typed.rs`;
  - `src/run/version_gate.rs`: `run_bounded` gains a stop input; `run`'s gate passes a never-stop, so its behaviour is
    unchanged;
  - `schemas/diag-line.v1.json`: `interrupted`, the 12th `process-exit.detail` member;
  - `tests/cli_verify.rs`;
  - `tests/support/verify.rs`.

## Test commands
- Lint: fmt, clippy, the Windows-target clippy (check only), G1, lint probes.
- Tests: `run --unit`, and `run --integration` over `cli_verify | cli_version_gate | contract_diag_schema` (baseline
  green, 70 passed).
- Guards: the literal probe (baseline red, last line 3, the subject this chunk builds; controls both ways), the
  preservation guard against `d9f01b1b8642` (baseline green; controls both ways), `cargo deny check`.
- Smoke: boot/status/cleanup, G2, G4, then `pre-push`.
- Operator pass: hygiene, push, CI read.
- Not listed: no mutation run, witness run, workflow dispatch or live `claude` start. The live-start count is 0.

## Decisive leans (reject any to send the plan back to P4)
1. **Every stopped run ends by kill, with no key.** The basis is security-plan §Threat Model Summary: no byte into any
   dialog, and a modal start is killed with no key. Run B's founder-ruled Ctrl-C ending stays as its normal end; the
   stop path is a new end, not a change to that ruling.
2. **The handler lives in the root package and is local to verify.** `viola-pty` stays agent-agnostic. The root
   already has `libc` and `windows-sys` with `Win32_System_Console` through workspace inheritance, so no manifest
   changes.
3. **The stop is read during the probes and once more before the step lines.** Once `check_rows` begins, the run
   completes, because the stamp write is atomic.
4. **A second Ctrl-C during cleanup changes nothing, and the stop path adds no time bound.**

## Concretization previews
No verification-matrix capability is claimed. No pooled cap names an interrupted verify, and `v1-40`'s other clauses
belong to other entries.

## For your information
- The same-pid pre-clean in `ProbeDir::create` / `TrustedDir::create` removes a leftover with this run's own pid by
  name. It predates the narrow ruling and is left unchanged (research.md §Measured facts 8).
- Expected amendments at the wrap land in all seven masters (plan.md §Implementation notes). The obs `detail` member
  takes a Decisions Log entry.
- Sized against one window (`inputs#I1`): one verb's stop path, about ten sites in one file, one schema member, two
  literal moves, two integration cases and about eight unit cases. No split is proposed.

## Beyond the plan
| # | proposal | kind |
|---|---|---|
| 1 | Event delivery to a running `viola verify` on Windows (a real console's Ctrl-C / Ctrl-Break reaching the routine) is left to the existing working-route entry "Windows-only live measurements", whose BLOCKED-ON (an interactive Windows host) stands | owner for a part left out |
