@apply_row n=1 ref_file=fanout-architecture--cascade-g4.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g4.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-architecture--cascade-g4.md ref_n=3 v=applied edit=E3
@apply_row n=4 ref_file=fanout-architecture--cascade-g4.md ref_n=4 v=refused
why: a hand removal · fct-fw4invxz
@apply_row n=5 ref_file=fanout-architecture--cascade-g4.md ref_n=5 v=applied edit=E4
@apply_row n=6 ref_file=fanout-architecture--cascade-g4.md ref_n=6 v=applied edit=E5
@apply_row n=7 ref_file=fanout-architecture--cascade-g4.md ref_n=7 v=applied edit=E6
@apply_row n=8 ref_file=fanout-architecture--cascade-g4.md ref_n=8 v=applied edit=E7
@apply_row n=9 ref_file=fanout-architecture--cascade-g4.md ref_n=9 v=applied edit=E8
@apply_row n=10 ref_file=fanout-architecture--cascade-g4.md ref_n=10 v=applied edit=E9
@apply_row n=11 ref_file=fanout-architecture--cascade-g4.md ref_n=11 v=applied edit=E10
@apply_row n=12 ref_file=fanout-architecture--cascade-g4.md ref_n=12 v=applied edit=E11
@fact id=fct-vuikun3q expect_rev=1
old: integrity check`), `viola run` refusing a `.cmd`/`.bat` child program before any spawn (`process-exit{subject:"self", exit_code:1, detail:"batch-script-child"}`, the two fixed `unable:`/`hint:` stderr lines), and `viola run` losing
new: integrity check`), and `viola run` losing
old: `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch`
new: `stale-heartbeat` · `pinned-hash-mismatch`
old: `install Claude Code or put it on PATH`, `the claude CLI is a .cmd or .bat script` / `pass the real executable, not a .cmd or .bat shim`, `the CLI version
new: `install Claude Code or put it on PATH`, `the CLI version

@fact id=fct-tg3zks37 expect_rev=1
old: `Ended::Refused`): `batch-script-child` · `program-not-found`
new: `Ended::Refused`): `program-not-found`

@fact id=fct-7s455w3m expect_rev=1
old: (on Unix a connected peer whose euid is not this user's or cannot be read, `peer_admitted`; on Linux and Windows the connected peer's pid
new: (a connected peer whose euid is not this user's or cannot be read, `peer_admitted`; or the connected peer's pid

@fact id=fct-gvna3tj2 expect_rev=1
old: (on Unix its peer euid not this user's or unreadable, `peer_admitted`; else its pid
new: (its peer euid not this user's or unreadable, `peer_admitted`; or its pid

@fact id=fct-62qgonvf expect_rev=1
old: reads `pass` (all twenty-four on Unix; on Windows twenty-one, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`), so
new: reads `pass` (all twenty-four, on every build), so
old: ids reads unverified on every OS, so do one of the eighteen and `resume-session` and one of the eighteen, `resume-session` and `agents-join` on Windows, and one of the eighteen, `resume-session`, `agents-join` and `local-command-compact` reads verified on Windows only. A non-`v:1`
new: ids reads unverified. A non-`v:1`

@fact id=fct-2tmeycxu expect_rev=1
old: `TokioClient::server_euid`) and, where it is reported (`PEER_EUID_REPORTED`, Unix), refuses another euid
new: `TokioClient::server_euid`) and refuses another euid
old: writes no frame. macOS reports no peer pid and holds the euid check alone, the named carve-out; Windows reads no euid, its DACL and the pid comparison the controls.
new: writes no frame.

@fact id=fct-zxepdepr expect_rev=1
old: human pair for `batch-script-child`, `pinned-hash-mismatch`
new: human pair for `pinned-hash-mismatch`

@fact id=fct-p4s6aosk expect_rev=1
old: `/bin/sh -c` (`statusline::shell_argv`) with
new: `/bin/sh -c` (`statusline::shell_argv`, the fixed three-element argv `[&str; 3]`) with
old: relied on Unix only, since chunk 2026-10-13-owed-ledger-rows; §Cross-cutting
new: relied on every build: `LedgerRow::relied()` holds all twenty-four rows; §Cross-cutting
old: Capability ledger). On Windows `shell_argv` returns `None` for every input and no command runs; the Windows shell-out is owed to the working-route entry "Windows-only live measurements" (the founder's word on the cut, same day and relay).
new: Capability ledger).

@fact id=fct-a6y4blve expect_rev=1
old: `server_verdict(recorded_pid, started_at, observed_pid, observed_start, peer_pid_reported)`
new: `server_verdict(recorded_pid, started_at, observed_pid, observed_start)`
old: OS start time; with `peer_pid_reported` false, as on macOS, which reports no peer pid, it returns `Matches` with no comparison: the recorded gap. The joined
new: OS start time. The joined
old: `verify_server(recorded_pid, started_at, observed_pid, peer_pid_reported)`
new: `verify_server(recorded_pid, started_at, observed_pid)`

@fact id=fct-ikwybol4 expect_rev=1
old: `peer_euid_admitted(observed, own, reported)` is `!reported || observed == Some(own)`, so a peer it cannot read is refused where reported;
new: `peer_euid_admitted(observed, own)` is `observed == Some(own)`, so a peer it cannot read is refused;
old: `own_euid()` and `PEER_EUID_REPORTED` (`cfg!(unix)`).
new: `own_euid()`.
old: `peer_creds().ok()?.euid()` on Unix, none on Windows)
new: `peer_creds().ok()?.euid()`)

@fact id=fct-yj2flshm expect_rev=1
old: with the same npm-shim → `claude.exe` resolution `run` uses (`run::resolve_program`).
new: with the same resolution `run` uses (`run::resolve_program`: a given path as given, a bare name on `PATH`).
old: ledger row `agents-join`, relied on every OS.
new: ledger row `agents-join`, relied on every build.
@pattern id=arch4-01
fixed: refusing a `.cmd`/`.bat` child program
@pattern id=arch4-02 ignore_case=true
fixed: the claude CLI is a .cmd or .bat script
@pattern id=arch4-03
fixed: on Linux and Windows the connected peer's pid
@pattern id=arch4-04
fixed: on Unix its peer euid not this user's
@pattern id=arch4-05
fixed: on Windows twenty-one
@pattern id=arch4-06
fixed: reads verified on Windows only
@pattern id=arch4-07
fixed: macOS reports no peer pid
@pattern id=arch4-08
fixed: Windows reads no euid
@pattern id=arch4-09
fixed: `shell_argv` returns `None` for every input
@pattern id=arch4-10
fixed: it returns `Matches` with no comparison
@pattern id=arch4-11
fixed: a peer it cannot read is refused where reported
@pattern id=arch4-12
fixed: npm-shim → `claude.exe` resolution
