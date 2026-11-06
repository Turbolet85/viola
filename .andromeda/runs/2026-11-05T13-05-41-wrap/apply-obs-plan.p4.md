@apply_row n=1 ref_file=fanout-obs-plan--cascade-g4.md ref_n=1 v=refused
why: a hand edit · fct-yb5jpaot — the change drops five code-span `|` from the `process-start` table row (`pty_backend`'s `conpty-sideload|conpty|` and `sideload_fallback`'s `hash-mismatch|unreadable|load-failed|not-built`), a cell-boundary change `rec put` refuses (reshaped); by hand the row reads `pty_backend` (`openpty`) and the whole `sideload_fallback` entry goes, every other field kept
@apply_row n=2 ref_file=fanout-obs-plan--cascade-g4.md ref_n=2 v=applied edit=E1
@apply_row n=3 ref_file=fanout-obs-plan--cascade-g4.md ref_n=3 v=applied edit=E2
@apply_row n=4 ref_file=fanout-obs-plan--cascade-g4.md ref_n=4 v=applied edit=E3
@apply_row n=5 ref_file=fanout-obs-plan--cascade-g4.md ref_n=5 v=applied edit=E4
@apply_row n=6 ref_file=fanout-obs-plan--cascade-g4.md ref_n=6 v=applied edit=E5
@apply_row n=7 ref_file=fanout-obs-plan--cascade-g4.md ref_n=7 v=applied edit=E6
@apply_row n=8 ref_file=fanout-obs-plan--cascade-g4.md ref_n=8 v=applied edit=E7
@apply_row n=9 ref_file=fanout-obs-plan--cascade-g4.md ref_n=9 v=applied edit=E8
@apply_row n=10 ref_file=fanout-obs-plan--cascade-g4.md ref_n=10 v=applied edit=E9
@apply_row n=11 ref_file=fanout-obs-plan--cascade-g4.md ref_n=11 v=applied edit=E10
@apply_row n=12 ref_file=fanout-obs-plan--cascade-g4.md ref_n=12 v=folded_into to_file=fanout-obs-plan--cascade-g4.md to_n=11
@apply_row n=13 ref_file=fanout-obs-plan--cascade-g4.md ref_n=13 v=folded_into to_file=fanout-obs-plan--cascade-g4.md to_n=11
@apply_row n=14 ref_file=fanout-obs-plan--cascade-g4.md ref_n=14 v=refused
why: a hand removal · fct-y3ozahco
@apply_row n=15 ref_file=fanout-obs-plan--cascade-g4.md ref_n=15 v=applied edit=E11
@apply_row n=16 ref_file=fanout-obs-plan--cascade-g4.md ref_n=16 v=applied edit=E12
@apply_row n=17 ref_file=fanout-obs-plan--cascade-g4.md ref_n=17 v=applied edit=E13
@apply_row n=18 ref_file=fanout-obs-plan--cascade-g4.md ref_n=18 v=applied edit=E14
@apply_row n=19 ref_file=fanout-obs-plan--cascade-g4.md ref_n=19 v=applied edit=E15
@fact id=fct-dsupowm6 expect_rev=1
old: `pinned-hash-mismatch`, `batch-script-child`, `internal-error`
new: `pinned-hash-mismatch`, `internal-error`
old: (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break)
new: (`SIGINT` / `SIGTERM`)
old: closes the `process-exit` `detail` at 20 values (the enum's members: the words of the exit-1, exit-21 and exit-20 lists here, as measured at chunk 2026-11-01-unix-endpoint-and-home-hardening).
new: closes the `process-exit` `detail` at 19 values (the enum's members: the words of the exit-1, exit-21 and exit-20 lists here, as measured at chunk 2026-11-05-linux-only-code-and-ci).
old: under a parsed `VIOLA_NAME`: `batch-script-child`, `program-not-found`,
new: under a parsed `VIOLA_NAME`: `program-not-found`,

@fact after=fct-dsupowm6 as=n1
- A closed word of `schemas/diag-line.v1.json` that no build can emit leaves the schema with its emitter: `os` is closed at `linux`.

@fact id=fct-qzbqiq2k expect_rev=1
old: on a server of another euid (Unix), on a server
new: on a server of another euid, on a server
old: or on a check that cannot be made (Linux, Windows and macOS, where it means an euid mismatch alone):
new: or on a check that cannot be made:

@fact id=fct-qiql34nd expect_rev=1
old: on a server of another euid (Unix), on a server
new: on a server of another euid, on a server
old: or on a check that cannot be made (Linux, Windows and macOS, where it means an euid mismatch alone):
new: or on a check that cannot be made:

@fact id=fct-c5mu6ru4 expect_rev=1
old: from `src/panic_frames.rs` — `RtlCaptureStackBackTrace` + `GetModuleHandleExW`/`GetModuleFileNameW` on Windows, `libc::backtrace` + `dladdr` on Unix — up to 62 strings
new: from `src/panic_frames.rs` — `libc::backtrace` + `dladdr` — up to 62 strings
old: for 72 frames on the Windows runner and
new: for 72 frames on that chunk's Windows runner and

@fact id=fct-bprhan45 expect_rev=1
old: a symlink is refused, a folder is held to the read rule, a file to the read rule on Unix and the write rule on Windows, and an absent path passes.
new: a symlink is refused, a folder and a file are held to the read rule, and an absent path passes.

@fact id=fct-vfnyc5f5 expect_rev=1
old: **Catch-site reach:** on Unix a home under a regular file
new: **Catch-site reach:** a home under a regular file
old: never reaches the `cli` catch site; on Windows such a path stats `NotFound`, the check skips it as absent, and the catch-site line `process-exit{detail:"internal-error"}` stands.
new: never reaches the `cli` catch site.
old: the human pair for `batch-script-child`, `pinned-hash-mismatch`,
new: the human pair for `pinned-hash-mismatch`,

@fact id=fct-l5yuzjd3 expect_rev=1
old: reaches the outer terminal on any OS, and on Unix the child's bytes `e4 b8 ad` reach it; the Windows byte reading is owed to the route entry "Windows-only live measurements", since ConPTY re-renders a wide character on a 1-column screen.
new: reaches the outer terminal, and the child's bytes `e4 b8 ad` reach it.

@fact id=fct-j5jj3gux expect_rev=1
old: `AgentError`'s variants include `BatchScriptChild` and `ProgramNotFound`, their two messages fixed.
new: `AgentError`'s variants include `ProgramNotFound`, its message fixed.

@fact id=fct-ydc7ueix expect_rev=1
old: On a Windows host it sits in the harness's host mutation scratch `<repo parent>/viola-mutants-scratch/` (`HOST_SCRATCH = cfg!(windows)`, inert elsewhere), outside the repository, whose path is never printed (documents carry only the byte count `scratch_bytes`). On the Linux dev host the same-named directory is only
new: On the Linux dev host a directory named `viola-mutants-scratch/` is only
old: a folder another user may read (Unix: a group or other bit; Windows: a read grant to another SID)
new: a folder another user may read (a group or other bit)
old: a file another user may read (Unix: a group or other bit; Windows: the write rule)
new: a file another user may read (a group or other bit)
old: `digit_runs_reads_whole_numbers_only`), on Linux and Windows (the file compiles out on macOS).
new: `digit_runs_reads_whole_numbers_only`), on Linux.
old: and finds no line, on Linux, macOS and Windows.
new: and finds no line, on Linux.
old: `assert_names_nothing` and `digit_runs`, on Linux and macOS (the file is `cfg(unix)`, with no case on Windows).
new: `assert_names_nothing` and `digit_runs`, on Linux.

@fact id=fct-xpak76xj expect_rev=1
old: in `scripts/lint-probes.sh`, Linux leg (mechanism
new: in `scripts/lint-probes.sh` (mechanism

@fact id=fct-t2ak7i7a expect_rev=1
old: fails on an unnamed gate or a row that no longer stands; its self-check reads eight planted cases
new: fails on an unnamed gate, a row that no longer stands, or a gate whose predicate names `windows` or `target_os = "macos"` (`another OS: …`, a fault even when a row names it); its self-check reads its planted cases and fires on every planted class

@fact id=fct-r4tkrl6z expect_rev=1
old: and on demand. The audit's Windows form is the dispatch-only, report-only `windows-mutants.yml` (`run --mutants --package <member> --file …` on `windows-2025`), whose job `mutants (viola-panic-frames)` scores `src/panic_frames.rs` on its own: 23 tested, 14 caught, 0 unviable, 0 missed, and 9 left out by the harness as `#[cfg(unix)]` twins the Windows build never compiles (that job's `run` document in run 38036448183, the dispatch of 2026-10-10; test-plan §10 Mutation gate). A timed-out
new: and on demand. A timed-out

@fact id=fct-4od5l5b2 expect_rev=1
old: the peer euid decision on Unix, then the server check
new: the peer euid decision, then the server check
old: against the one `snapshot.json` read; macOS the recorded gap, no pid comparison)
new: against the one `snapshot.json` read)

@fact id=fct-hfdtlj6j expect_rev=1
old: callers `run` 8 · `verify` 6 · `list` 2
new: callers `run` 7 · `verify` 5 · `list` 2
old: the rule: the `human::refuse(` call sites under `src/`)
new: the rule: the `human::refuse(` call sites under `src/`, as measured at chunk 2026-11-05-linux-only-code-and-ci)
@pattern id=obs4-01
fixed: closes the `process-exit` `detail` at 20 values
@pattern id=obs4-02
fixed: Windows Ctrl-C / Ctrl-Break
@pattern id=obs4-03
fixed: `pinned-hash-mismatch`, `batch-script-child`
@pattern id=obs4-04
fixed: where it means an euid mismatch alone
@pattern id=obs4-05
fixed: `GetModuleHandleExW`/`GetModuleFileNameW` on Windows
@pattern id=obs4-06
fixed: the read rule on Unix and the write rule on Windows
@pattern id=obs4-07
fixed: on Windows such a path stats `NotFound`
@pattern id=obs4-08
fixed: the human pair for `batch-script-child`
@pattern id=obs4-09
fixed: the Windows byte reading is owed
@pattern id=obs4-10
fixed: include `BatchScriptChild` and `ProgramNotFound`
@pattern id=obs4-11
fixed: On a Windows host it sits in the harness's host mutation scratch
@pattern id=obs4-12
fixed: Windows: a read grant to another SID
@pattern id=obs4-13
fixed: the file compiles out on macOS
@pattern id=obs4-14 ignore_case=true
fixed: on Linux, macOS and Windows
@pattern id=obs4-15
fixed: `scripts/lint-probes.sh`, Linux leg
@pattern id=obs4-16
fixed: The audit's Windows form is the dispatch-only, report-only
@pattern id=obs4-17
fixed: macOS the recorded gap, no pid comparison
@pattern id=obs4-18
fixed: callers `run` 8 · `verify` 6
