@apply_row n=1 ref_file=fanout-architecture--cascade-g5.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-architecture--cascade-g5.md ref_n=3 v=applied edit=E2
@apply_row n=3 ref_file=fanout-architecture--cascade-g5.md ref_n=4 v=applied edit=E3
@apply_row n=4 ref_file=fanout-architecture--cascade-g5.md ref_n=5 v=applied edit=E4
@apply_row n=5 ref_file=fanout-architecture--cascade-g5.md ref_n=6 v=applied edit=E5
@apply_row n=6 ref_file=fanout-architecture--cascade-g5.md ref_n=7 v=applied edit=E6
@apply_row n=7 ref_file=fanout-architecture--cascade-g5.md ref_n=8 v=applied edit=E7
@apply_row n=8 ref_file=fanout-architecture--cascade-g5.md ref_n=9 v=applied edit=E8
@apply_row n=9 ref_file=fanout-architecture--cascade-g5.md ref_n=11 v=applied edit=E9
@apply_row n=10 ref_file=fanout-architecture--cascade-g5.md ref_n=13 v=applied edit=E10
@apply_row n=11 ref_file=fanout-architecture--cascade-g5.md ref_n=14 v=applied edit=E11
@apply_row n=12 ref_file=fanout-architecture--cascade-g5.md ref_n=16 v=applied edit=E12
@apply_row n=13 ref_file=fanout-architecture--cascade-g5.md ref_n=18 v=applied edit=E13
@apply_row n=14 ref_file=fanout-architecture--cascade-g5.md ref_n=19 v=applied edit=E14
@apply_row n=15 ref_file=fanout-architecture--cascade-g5.md ref_n=21 v=applied edit=E15
@apply_row n=16 ref_file=fanout-architecture--cascade-g5.md ref_n=23 v=applied edit=E16
@apply_row n=17 ref_file=fanout-architecture--cascade-g5.md ref_n=24 v=applied edit=E17
@apply_row n=18 ref_file=fanout-architecture--cascade-g6.md ref_n=1 v=applied edit=E18
@apply_row n=19 ref_file=fanout-architecture--cascade-g6.md ref_n=2 v=applied edit=E19
@apply_row n=20 ref_file=fanout-architecture--cascade-g6.md ref_n=3 v=applied edit=E20
@apply_row n=21 ref_file=fanout-architecture--cascade-g6.md ref_n=4 v=applied edit=E21
@apply_row n=22 ref_file=fanout-architecture--cascade-g6.md ref_n=5 v=applied edit=E22
@apply_row n=23 ref_file=fanout-architecture--cascade-g6.md ref_n=6 v=applied edit=E23
@apply_row n=24 ref_file=fanout-architecture--cascade-g6.md ref_n=7 v=applied edit=E24
@apply_row n=25 ref_file=fanout-architecture--cascade-g6.md ref_n=8 v=applied edit=E25
@apply_row n=26 ref_file=fanout-architecture--cascade-g6.md ref_n=9 v=applied edit=E26
@apply_row n=27 ref_file=fanout-architecture--cascade-g6.md ref_n=10 v=applied edit=E27
@apply_row n=28 ref_file=fanout-architecture--cascade-g6.md ref_n=11 v=applied edit=E28
@apply_row n=29 ref_file=fanout-architecture--cascade-g6.md ref_n=12 v=applied edit=E29
@apply_row n=30 ref_file=fanout-architecture--cascade-g6.md ref_n=13 v=refused
why: a hand removal · fct-snmldjr6
@apply_row n=31 ref_file=fanout-architecture--cascade-g6.md ref_n=14 v=refused
why: a hand removal · fct-jyuckxyp
@apply_row n=32 ref_file=fanout-architecture--cascade-g6.md ref_n=15 v=applied edit=E30
@apply_row n=33 ref_file=fanout-architecture--cascade-g6.md ref_n=16 v=refused
why: a hand removal · fct-ymvbuev2
@apply_row n=34 ref_file=fanout-architecture--cascade-g6.md ref_n=17 v=refused
why: a hand removal · fct-znlazsvj
@fact id=fct-nm6raycf expect_rev=1
old: - **Linux / macOS:** Unix domain socket file
new: - **Linux:** Unix domain socket file
old: `$XDG_RUNTIME_DIR/viola/` on Linux, `$TMPDIR/viola/` on macOS, and `/tmp/viola-<uid>/` only as a fallback
new: `$XDG_RUNTIME_DIR/viola/`, and `/tmp/viola-<uid>/` only as a fallback
old: never the `/tmp` or `$TMPDIR` root.
new: never the `/tmp` root.

@fact id=fct-uqyfijk4 expect_rev=1
old: and then, on Unix, the connected peer's euid
new: and then the connected peer's euid
old: the hook failing open. macOS reports no peer pid and stays on the recorded gap, owned by no route entry: there the pid and start-time comparison is absent and `server-verify-failed` means an euid mismatch alone.
new: the hook failing open.

@fact id=fct-bzckmn3n expect_rev=1
old: behind `scripts/agent-run.{sh,ps1}`; its command contract
new: behind `scripts/agent-run.sh`; its command contract

@fact id=fct-mnor3ks3 expect_rev=1
old: (one of the eleven codes, §Standard Contracts)
new: (one of the ten codes, §Standard Contracts)

@fact id=fct-l6ysfimx expect_rev=1
old: How a home under a regular file reads on each OS: Filesystem
new: How a home under a regular file reads: Filesystem

@fact id=fct-p6lni4fy expect_rev=1
old: (none on Windows, and none for a pinned path the override rule refuses;
new: (none for a pinned path the override rule refuses;

@fact id=fct-nbf2wdvs expect_rev=1
old: and, on Unix for shell-plain paths, `--settings
new: and, for shell-plain paths, `--settings
old: after its `--plugin-dir` pair (no flag on Windows); Run C
new: after its `--plugin-dir` pair; Run C

@fact id=fct-ehpssyzz expect_rev=1
old: It is written only on Unix, and only when every character
new: It is written only when every character
old: for any other path, and on Windows, no override is written
new: for any other path no override is written
old: relied on Unix only, since chunk
new: relied on every build (`LedgerRow::relied()`: all twenty-four rows), since chunk
old: relayed verbatim by the operator). The Windows override is owed to the working-route entry "Windows-only live measurements".
new: relayed verbatim by the operator).

@fact id=fct-lb4pv4em expect_rev=1
old: every inherited `CLAUDE*` name (ASCII case-insensitive on Windows, exact on Unix) except a name the user defines persistently — on Windows a value NAME under `HKCU\Environment` or `HKLM\SYSTEM\CurrentControlSet\Control\Session Manager\Environment` (read through windows-sys Registry, names only), on Unix a name in `config.json` `claude_env_keep`.
new: every inherited `CLAUDE*` name, matched exactly, except a name in `config.json` `claude_env_keep`.
old: Names only: the strip, the registry reader and the logs never read a value
new: Names only: the strip and the logs never read a value

@fact id=fct-5lqu6sil expect_rev=1
old: run no mutation job, and the dispatch-only `windows-mutants.yml` runs the `--package <member> --file …` arm, which reads no base.
new: run no mutation job.

@fact id=fct-3jytrhfq expect_rev=1
old: `claude_env_keep`: the Unix R8 pass-list
new: `claude_env_keep`: the R8 pass-list

@fact id=fct-6u55z4cp expect_rev=1
old: `bin/<version>-<hash>/viola(.exe)`, the pinned copy (0700 on Unix):
new: `bin/<version>-<hash>/viola`, the pinned copy (0700):

@fact id=fct-if66544y expect_rev=1
old: in `detail`: `batch-script-child`, `program-not-found`,
new: in `detail`: `program-not-found`,

@fact id=fct-wdtionxy expect_rev=1
old: a folder takes the read rule, a file takes the read rule on Unix and the write rule on Windows, and an absent path passes
new: a folder and a file each take the read rule, and an absent path passes

@fact id=fct-hn66ngvt expect_rev=1
old: wraps the statusline; Unix only, see Claude Code integration names)
new: wraps the statusline; see Claude Code integration names)
old: dir 0700, files 0600 on Unix; one `write` per line
new: dir 0700, files 0600; one `write` per line
old: `check_path` on `instances/`, on every OS, each that exists
new: `check_path` on `instances/`, each that exists
old: its `events.ndjson` (the write rule on Unix, the read rule on Windows)
new: its `events.ndjson` (the write rule)
old: The check is proven on Unix (five widened modes). On Windows `cfg(windows)` cases prove it: an instance tree granted Users write is refused `Writable` beside a `create_private_dir` tree that passes, a `diagnostics/` granted Users read is refused, and a plain home made before viola is refused for its inherited write grant to a SID `trusted_sid` does not trust, read by its right whichever SID holds it, while a home `instance_tree()` makes reads none (as measured at ci#40439188053, `windows-2025`).
new: The check is proven by five widened modes.

@fact id=fct-kekl3p46 expect_rev=1
old: The read rule is the write rule plus, on Unix, a refusal of any group or other bit (`mode & 0o077`) and, on Windows, a refusal of any allow ACE, an inherit-only one included, that grants a right in `READ_RIGHTS` (`0x9000_0001`: `FILE_READ_DATA` | `GENERIC_READ` | `GENERIC_ALL`) to a SID `trusted_sid` rejects (`unix_read_verdict`, `windows_read_verdict`, `check_read`).
new: The read rule is the write rule plus a refusal of any group or other bit (`mode & 0o077`) (`unix_read_verdict`, `check_read`).

@fact id=fct-vs5zumck expect_rev=1
old: the pinned `viola` (`viola.exe` on Windows), `plugin/<key>/`
new: the pinned `viola`, `plugin/<key>/`

@fact id=fct-7uywwkoa expect_rev=1
old: the mode or ACL left as found
new: the mode left as found

@fact id=fct-sbmskf35 expect_rev=1
old: A path under a regular file stats `NotFound` on Windows, so the strict-modes check skips it as absent and the `cli` catch-site line stands there; on Unix the same home is a strict-modes refusal (`verify` exits 1 with the home pair, the frame verbs exit 21 with the instance pair). A test that plants a regular file above a checked path therefore needs a per-OS oracle (as measured at ci#40398601594, `windows-2025`).
new: A path under a regular file is a strict-modes refusal (`verify` exits 1 with the home pair, the frame verbs exit 21 with the instance pair).

@fact id=fct-isxdwj6d expect_rev=1
old: holds the home to the read rule on every OS: `board::scan`
new: holds the home to the read rule: `board::scan`
old: so a 0750 home, or on Windows one granting read to a SID `trusted_sid` rejects, refuses the whole board
new: so a 0750 home refuses the whole board

@fact id=fct-jznqbn54 expect_rev=1
old: `ledger/` is 0700 and both files 0600 on Unix.
new: `ledger/` is 0700 and both files 0600.
old: check first: owner-only modes on Unix, owner + DACL on Windows);
new: check first: owner-only modes);

@fact id=fct-i7o6tqmg expect_rev=1
old: SIGKILL, a crash, a failed handler install and a Windows close, logoff or shutdown leave it to be removed by hand
new: SIGKILL, a crash or a failed handler install leave it to be removed by hand
old: on Unix, when `probe_document` is `Some`
new: when `probe_document` is `Some`

@fact id=fct-c3nhxk2n expect_rev=1
old: stopped by Ctrl-C or TERM: Unix `SIGINT` / `SIGTERM`, Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT` (`cmd::verify::stop`,
new: stopped by Ctrl-C or TERM: `SIGINT` / `SIGTERM` (`cmd::verify::stop`,
old: SIGKILL, a crash, a failed handler install (ignored, fail-open) and a Windows close, logoff or shutdown keep the OS default
new: SIGKILL, a crash and a failed handler install (ignored, fail-open) keep the OS default

@fact id=fct-jv5gqhbl expect_rev=2
old: reads 32 sites in 24 files under `tests/`
new: reads 31 sites in 23 files under `tests/`
old: as measured at chunk 2026-11-04-linux-and-macos-parity's wrap
new: as measured at chunk 2026-11-05-linux-only-code-and-ci's wrap
old: on Unix its `attach` polls
new: its `attach` polls
old: (on Unix a connect to the path is refused or the path is absent, on Windows the pipe is not found)
new: (a connect to the path is refused or the path is absent)
old: its socket file stays on Unix, where
new: its socket file stays, where

@fact id=fct-w6vpbrjt expect_rev=1
old: and its `Drop`, one twin per OS under its own gate: on Unix `attach` sends `kill -STOP` and polls `ps -o stat=` until it reads `T`, and the drop sends `kill -CONT`; on Windows `attach` attaches a debugger with kill-on-exit off and waits for the first debug event, and the drop detaches.
new: and its `Drop`, ungated: `attach` sends `kill -STOP` and polls `ps -o stat=` until it reads `T`, and the drop sends `kill -CONT`.

@fact id=fct-5v4ir6ih expect_rev=1
old: `SOCKET_ROOT_VAR` being `TMPDIR` on macOS and `XDG_RUNTIME_DIR` elsewhere,
new: `SOCKET_ROOT_VAR` reading `XDG_RUNTIME_DIR`,

@fact id=fct-3fjm2cf6 expect_rev=1
old: (`LedgerRow::relied()`: 24 rows on Unix, 21 on Windows)
new: (`LedgerRow::relied()`: all twenty-four rows on every build)
old: `24 pass  0 fail` on Unix);
new: `24 pass  0 fail`);

@fact id=fct-fs7kfa27 expect_rev=1
old: the session's `bin/claude[.exe]` copy
new: the session's `bin/claude` copy

@fact id=fct-xib5fpty expect_rev=1
old: On Unix a base that is a link must have an absolute target
new: A base that is a link must have an absolute target
old: (no reason added). On Windows the keepers are the plain create. On the Linux dev host
new: (no reason added). On the Linux dev host

@fact id=fct-xtqblyzi expect_rev=1
old: (outside the repository, a sibling of it): the host mutation scratch. On a Windows host (`HOST_SCRATCH = cfg!(windows)`) `run --mutants` puts cargo-mutants' temp copies there (`TMP`/`TEMP` on the `cargo mutants` command) and its `--output`, so `mutants.out/` is `<scratch>/mutants.out/`. A guard refuses unless the resolved path's final component is exactly `viola-mutants-scratch` and the repository is neither it nor inside it (`scratch-refused`, never a fallback to `%TEMP%` or the repo). The scratch is measured, removed and recreated before every counted, scoped or package run (`scratch-wipe-failed` when the removal fails); the re-creation goes through `viola_state::fs::create_private_dir`, so on Windows the scratch is the topmost folder it creates outside the profile and takes the protected user + SYSTEM DACL, and every temp copy and test home laid inside it inherits owner-only, as one under `%TEMP%` does (a plain-created scratch on the runner's workspace drive inherits BUILTIN\Users write, which the instance check refuses `Writable`). The witness is `wipe_scratch_takes_an_instance_tree_the_check_accepts_where_a_plain_dir_does_not`, on CI's `test (windows-2025)` leg (as measured at ci#38375200217, chunk 2026-10-13-epoch-4-cleanup); whether the root package's Windows mutation baseline then reads green is not yet measured (the next `windows-mutants.yml` dispatch). Its path is never printed: documents carry only the byte count `scratch_bytes`. The dispatch-only `windows-mutants.yml` runs this same arm on the `windows-2025` runner, so there the scratch sits beside the runner's checkout (its parent directory); each job's document carries only `scratch_bytes` (0 on a fresh runner, as read in run 37174673472). Off Windows the harness arm is inert and `mutants.out/` stays at the repository root (gitignored).
new: (outside the repository, a sibling of it): `run --mutants` has no scratch arm and `mutants.out/` stays at the repository root (gitignored).

@pattern id=arch5-01
fixed: `$TMPDIR/viola/` on macOS
@pattern id=arch5-02
fixed: macOS reports no peer pid
@pattern id=arch5-03
fixed: agent-run.{sh,ps1}
@pattern id=arch5-04
fixed: one of the eleven codes
@pattern id=arch5-05
fixed: home under a regular file reads on each OS
@pattern id=arch5-06
fixed: (none on Windows, and none for a pinned path
@pattern id=arch5-07
fixed: (no flag on Windows)
@pattern id=arch5-08
fixed: It is written only on Unix
@pattern id=arch5-09
fixed: relied on Unix only
@pattern id=arch5-10
fixed: The Windows override is owed to
@pattern id=arch5-11
fixed: ASCII case-insensitive on Windows
@pattern id=arch5-12
fixed: the strip, the registry reader and the logs
@pattern id=arch5-13
fixed: dispatch-only `windows-mutants.yml` runs
@pattern id=arch5-14
fixed: the Unix R8 pass-list
@pattern id=arch5-15
fixed: viola(.exe)
@pattern id=arch5-16
fixed: `batch-script-child`, `program-not-found`
@pattern id=arch5-17
fixed: the read rule on Unix and the write rule on Windows
@pattern id=arch5-18
fixed: the write rule on Unix, the read rule on Windows
@pattern id=arch5-19
fixed: On Windows `cfg(windows)` cases prove it
@pattern id=arch5-20
fixed: `windows_read_verdict`
@pattern id=arch5-21
fixed: (`viola.exe` on Windows)
@pattern id=arch5-22
fixed: the mode or ACL left as found
@pattern id=arch5-23
fixed: stats `NotFound` on Windows
@pattern id=arch5-24
fixed: on Windows one granting read to a SID
@pattern id=arch5-25
fixed: owner + DACL on Windows
@pattern id=arch5-26
fixed: a Windows close, logoff or shutdown
@pattern id=arch5-27
fixed: Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT`
@pattern id=arch5-28
fixed: 32 sites in 24 files
@pattern id=arch5-29
fixed: on Windows the pipe is not found
@pattern id=arch5-30
fixed: on Windows `attach` attaches a debugger
@pattern id=arch5-31
fixed: `TMPDIR` on macOS and `XDG_RUNTIME_DIR` elsewhere
@pattern id=arch5-32
fixed: 24 rows on Unix, 21 on Windows
@pattern id=arch5-33
fixed: `bin/claude[.exe]`
@pattern id=arch5-34
fixed: On Windows the keepers are the plain create
@pattern id=arch5-35
fixed: On a Windows host (`HOST_SCRATCH = cfg!(windows)`)
