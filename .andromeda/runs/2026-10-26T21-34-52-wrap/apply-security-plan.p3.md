# Apply — security-plan · part 3 of 4

## Verdicts
- fanout-security-plan--cascade-g2.md 1 · already stands · .andromeda/security-plan-detail/input-validation.md:22–22 · «The event and dialog paths then canonicalise the instance directory and run the instance check (`canonical`, then `trusted`, in `src/cmd/hook.rs`: `viola_state::strict::check_instance`) before the diagnostics level and obs init»
- fanout-security-plan--cascade-g2.md 2 · already stands · .andromeda/security-plan-detail/input-validation.md:30–30 · «on its nine `HookEvent` arms and `hook statusline`, each outside `--capture`, canonicalise the instance directory and run `viola_state::strict::check_instance`»
- fanout-security-plan--cascade-g2.md 3 · already stands · .andromeda/security-plan-detail/input-validation.md:30–30 · «`viola verify` runs `viola_state::strict::check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its `cli` log»
- fanout-security-plan--cascade-g2.md 4 · already stands · .andromeda/security-plan-detail/input-validation.md:31–31 · «`wait`, `last`, `answer`, `pause` and `release` each run `client::refuse_untrusted` first»
- fanout-security-plan--cascade-g2.md 5 · already stands · .andromeda/security-plan-detail/input-validation.md:32–32 · «and the start arm also runs `check_home_tree` (through `run`'s `checked_exe`) before its wrapper log»
- fanout-security-plan--cascade-g2.md 6 · already stands · .andromeda/security-plan-detail/input-validation.md:33–33 · «through `client::refuse_untrusted`, the one check they share with the six frame verbs»
- fanout-security-plan--cascade-g2.md 8 · already stands · .andromeda/security-plan-detail/input-validation.md:53–53 · «They have six product callers, each taken only after its entry point's strict-modes check has passed»
- fanout-security-plan--cascade-g2.md 9 · applied · .andromeda/security-plan-detail/input-validation.md:54#1
- fanout-security-plan--cascade-g2.md 10 · already stands · .andromeda/security-plan-detail/input-validation.md:54–54 · «**Integrity:** read only after the home strict-modes check, at every entry point.»
- fanout-security-plan--cascade-g2.md 11 · already stands · .andromeda/security-plan-detail/input-validation.md:54–54 · «`viola verify`'s `update_stamps` runs `strict::check_stamps` before it creates `ledger/` and again under its lock before the capped read»
- fanout-security-plan--cascade-g2.md 13 · already stands · .andromeda/security-plan-detail/input-validation.md:77–77 · «the snapshot read (`read_snapshot_or_replay`: a replayed snapshot carries no `statusline_command`, so nothing runs and nothing prints)»
- fanout-security-plan--cascade-g2.md 14 · already stands · .andromeda/security-plan-detail/input-validation.md:80–80 · «**Owner/DACL:** before the held open, `pin_companions` runs `viola_state::strict::check_present` on `bin/<key>/conpty/`»
- fanout-security-plan--cascade-g2.md 15 · already stands · .andromeda/security-plan-detail/input-validation.md:80–80 · «`hash-mismatch` · `strict-modes-failed` · `unreadable` · `load-failed` · `not-built`»
- fanout-security-plan--cascade-g2.md 17 · already stands · .andromeda/security-plan-detail/data-protection.md:11–11 · «reads `D:PAI(A;OICI;FA;;;LA)(A;OICI;FA;;;SY)`»
- fanout-security-plan--cascade-g2.md 18 · already stands · .andromeda/security-plan-detail/data-protection.md:12–12 · «On Windows a plain home made before viola (`create_dir_all`) is refused `Writable`»
- fanout-security-plan--cascade-g2.md 19 · already stands · .andromeda/security-plan-detail/data-protection.md:17–17 · «`viola run`'s start pins the bytes it has already checked (`viola_state::pin::pin_bytes`, under the key `pin::pin_key`»
- fanout-security-plan--cascade-g2.md 20 · already stands · .andromeda/security-plan-detail/data-protection.md:18–18 · «Before the held open, `viola_state::strict::check_present` runs on `bin/<key>/conpty/` and on each companion that exists: a refusal is `PinError::StrictModes`»
- fanout-security-plan--cascade-g2.md 21 · already stands · .andromeda/security-plan-detail/data-protection.md:40–40 · «`open_role_file` and `write_detail` run `viola_state::strict::check_diagnostics`»
- fanout-security-plan--cascade-g2.md 22 · already stands · .andromeda/security-plan-detail/bootstrap-phases.md:15–15 · «every client reads the snapshot only after the instance's strict-modes check»
- fanout-security-plan--cascade-g2.md 23 · already stands · .andromeda/security-plan-detail/bootstrap-phases.md:16–16 · «`check_home_tree` (the home tree at `run` and `revive` start and in `verify`)»

## Edits
- line: .andromeda/security-plan-detail/input-validation.md:54
  old: «(the rule: `read_snapshot(` under `src/`, outside test modules, as measured at viola-0.1.0/chunks/2026-10-26-home-and-code-bearing-file-integrity/report.md). Hook, CLI and MCP processes reading `events.ndjson` through the replay is a boundary widening the founder answered himself (P4 fork Q1 of chunk 2026-10-26-home-and-code-bearing-file-integrity, `inputs#I3`). **Integrity:**»
  new: «(the rule: `read_snapshot(` under `src/`, outside test modules). **Integrity:**»

## Patterns
(none)
