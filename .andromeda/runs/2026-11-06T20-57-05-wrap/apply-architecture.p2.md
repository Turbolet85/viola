@apply_row n=1 ref_file=fanout-architecture--cascade-g1.md ref_n=3 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g1.md ref_n=5 v=refused
why: a hand removal · fct-co2wsjkw
@apply_row n=3 ref_file=fanout-architecture--cascade-g1.md ref_n=6 v=applied edit=E2
@apply_row n=4 ref_file=fanout-architecture--cascade-g1.md ref_n=8 v=applied edit=E3
@apply_row n=5 ref_file=fanout-architecture--cascade-g1.md ref_n=9 v=applied edit=E4
@apply_row n=6 ref_file=fanout-architecture--cascade-g1.md ref_n=11 v=applied edit=E5
@apply_row n=7 ref_file=fanout-architecture--cascade-g1.md ref_n=12 v=applied edit=E6
@apply_row n=8 ref_file=fanout-architecture--cascade-g1.md ref_n=13 v=applied edit=E7
@apply_row n=9 ref_file=fanout-architecture--cascade-g1.md ref_n=15 v=applied edit=E8
@apply_row n=10 ref_file=fanout-architecture--cascade-g1.md ref_n=16 v=applied edit=E9
@apply_row n=11 ref_file=fanout-architecture--cascade-g1.md ref_n=18 v=applied edit=E10
@apply_row n=12 ref_file=fanout-architecture--cascade-g2.md ref_n=1 v=applied edit=E11
@apply_row n=13 ref_file=fanout-architecture--cascade-g2.md ref_n=2 v=applied edit=E12
@fact id=fct-ad44nn2e expect_rev=3
old: portable-pty `=0.8.1` pinned behind
new: portable-pty `=0.9.0` pinned behind
old: Rationale: 0.8.1 is spike-proven (S1, O1 render stress, the Pulse live test, and about an hour driving this arch session in Windows Terminal with no PTY defect). 0.9.0's Windows garbage-read bug (wezterm#6783) does not bear on a Linux-only build; moving `portable-pty` off `=0.8.1` is owed to the working-route entry "Version done-check" (RUSTSEC-2026-0441, below). Swap triggers: portable-pty-psmux 0.9.7 if the O1 stray-spaces artifact reproduces only under `viola run` (then test `PASSTHROUGH_MODE`).
new: Rationale: exact, as the workspace pins every third-party version; 0.9.0's graph holds no `pin-utils`, so RUSTSEC-2026-0441 does not reach the build, and `viola-pty` compiles unchanged on it.

@fact id=fct-xbwprhwr expect_rev=1
old: Rationale: exec form skips Git Bash path rewriting and shell startup.
new: Rationale: exec form skips shell startup.

@fact id=fct-vxml3b56 expect_rev=2
old: is first read live at "Version done-check"; the ceiling is PROVISIONAL.
new: has not been read live; the ceiling is PROVISIONAL, owner: 0.2.0's first entry.

@fact id=fct-wpbp4hte expect_rev=1
old: `DIALOG_HOLD_MAX`, PROVISIONAL, beside `SPINE_DEADLINE`)
new: `DIALOG_HOLD_MAX`, PROVISIONAL, owner: 0.2.0's first entry, beside `SPINE_DEADLINE`)

@fact after=fct-ikuc6e4y as=n1
- OPEN — whether the budget gate stays and what it stands on, given that it rests on the user's status-line `rate_limits` payload, which reads the account's plan state (finding F-06i, [CLI Version Compatibility]) · owner: 0.2.0's intent.

@fact after=fct-qwpklckh as=n2
- The `statusline-rate-limits` row reads the account's plan state, not the CLI alone: on an account whose status-line payload carries `five_hour` `unknown` and only `seven_day`, `viola verify` fails the row on an unchanged CLI (`stamped 2.1.287  23 pass  1 fail`, `[21/24] statusline-rate-limits`, on the rig home, as measured on the Linux dev host at `viola-0.1.0/chunks/2026-11-06-version-done-check/evidence/live-round.md` Session 1); the dev home's stamps are unmoved. What the budget gate stands on is OPEN at [Budget Governor] (finding F-06i), owner: 0.2.0's intent.

@fact id=fct-xqkfyxga expect_rev=2
old: (one ratified exception: `viola-pty`'s
new: (six ratified exceptions, five named below: `viola-pty`'s
old: A first open divergence, not an exception: `viola-mcp` holds
new: The five, ratified as named exceptions: `viola-mcp` holds
old: ; the rule stands, and its fold or ratification is open. A second: `viola-state` holds
new: ; `viola-state` holds
old: beside `StateError`, the same. Three more the census below adds, the same: `PinError`, `StampsReadError` and `Refused`. The owner of all five: a CARRY on the working-route entry "Version done-check"; anyhow
new: beside `StateError`; and the three the census below names, `PinError`, `StampsReadError` and `Refused`. anyhow

@fact id=fct-gctodeyz expect_rev=1
old: The product sources hold 9
new: The product sources, outside the test-only `crates/viola-e2e`, hold 9
old: --include=*.rs`).
new: --include=*.rs`, whose one further row, `crates/viola-e2e/src/harness/mod.rs`, is test-only).

@fact id=fct-wtlwpexy expect_rev=1
old: acting on it is owed to the working-route entry "Version done-check".
new: it is recorded as known, with no action in 0.1.0, and the pinned copy's re-hash stays the guard.

@fact id=fct-onw4pywk expect_rev=2
old: leading-slash argument.** viola warns when an argument carries a Git Bash rewritten-path prefix. Rationale: Git Bash rewrites `/skill` arguments into Windows paths (brief §6), and agents need machine-parseable refusals.
new: leading-slash argument.** Rationale: agents need machine-parseable refusals.

@fact id=fct-k6htnrfj expect_rev=2
old: (`std::fs::canonicalize`, a Windows `\\?\C:\…` written back as `C:\…`, every other verbatim form kept)
new: (`std::fs::canonicalize`)

@fact id=fct-wfge3rsz expect_rev=2
old: so `budget-paused` now rests on a stamped row.
new: so `budget-paused` now rests on a stamped row, one that reads the account's plan state, not the CLI alone ([CLI Version Compatibility]).

@pattern id=arch2-01
fixed: 0.8.1 is spike-proven
@pattern id=arch2-02
fixed: Swap triggers: portable-pty-psmux
@pattern id=arch2-03
fixed: skips Git Bash path rewriting
@pattern id=arch2-04
fixed: is first read live at "Version done-check"
@pattern id=arch2-05
fixed: open divergence, not an exception
@pattern id=arch2-06
fixed: its fold or ratification is open
@pattern id=arch2-07
fixed: acting on it is owed to the working-route entry
@pattern id=arch2-08
fixed: viola warns when an argument carries a Git Bash rewritten-path prefix
@pattern id=arch2-09
fixed: written back as `C:\…`
