@apply_row n=1 ref_file=fanout-security-plan--cascade-g4.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-security-plan--cascade-g4.md ref_n=2 v=refused
why: a hand removal · fct-uooa7rml
@apply_row n=3 ref_file=fanout-security-plan--cascade-g4.md ref_n=4 v=applied edit=E2
@apply_row n=4 ref_file=fanout-security-plan--cascade-g4.md ref_n=5 v=applied edit=E3
@apply_row n=5 ref_file=fanout-security-plan--cascade-g4.md ref_n=6 v=applied edit=E4
@apply_row n=6 ref_file=fanout-security-plan--cascade-g4.md ref_n=8 v=applied edit=E5
@apply_row n=7 ref_file=fanout-security-plan--cascade-g4.md ref_n=10 v=refused
why: a hand removal · fct-bboseg7m
@apply_row n=8 ref_file=fanout-security-plan--cascade-g4.md ref_n=12 v=applied edit=E6
@apply_row n=9 ref_file=fanout-security-plan--cascade-g4.md ref_n=14 v=applied edit=E7
@apply_row n=10 ref_file=fanout-security-plan--cascade-g4.md ref_n=15 v=applied edit=E8
@apply_row n=11 ref_file=fanout-security-plan--cascade-g4.md ref_n=16 v=applied edit=E9
@apply_row n=12 ref_file=fanout-security-plan--cascade-g4.md ref_n=17 v=applied edit=E10
@apply_row n=13 ref_file=fanout-security-plan--cascade-g4.md ref_n=18 v=applied edit=E11
@apply_row n=14 ref_file=fanout-security-plan--cascade-g4.md ref_n=19 v=refused
why: a hand removal · fct-gezignrj
@apply_row n=15 ref_file=fanout-security-plan--cascade-g4.md ref_n=21 v=applied edit=E12
@apply_row n=16 ref_file=fanout-security-plan--cascade-g4.md ref_n=22 v=refused
why: a hand removal · fct-mqyejp35
@apply_row n=17 ref_file=fanout-security-plan--cascade-g4.md ref_n=24 v=refused
why: a hand removal · fct-glgoi2kc
@apply_row n=18 ref_file=fanout-security-plan--cascade-new-dependency-security.md ref_n=1 v=applied edit=E13
@apply_row n=19 ref_file=fanout-security-plan--cascade-new-dependency-security.md ref_n=2 v=applied edit=E14
@apply_row n=20 ref_file=fanout-security-plan--cascade-new-dependency-security.md ref_n=3 v=applied edit=E15
@fact id=fct-l7hy7pzk expect_rev=1
old: once the home exists; the plain drive form on Windows — the resolution as landed
new: once the home exists; the resolution as landed
old: the connect, on Unix the peer euid decision
new: the connect, the peer euid decision
old: and then, on Linux and Windows, the server check before the frame
new: and then the server check before the frame
old: `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch`
new: `stale-heartbeat` · `pinned-hash-mismatch`
old: applies `std::fs::canonicalize` when the path exists (on Windows a `\\?\C:\…` result is written back as `C:\…`, only the verbatim-drive prefix stripped; `\\?\UNC\…`, `\\?\Volume{…}\…` and `\\.\…` stay as given), while
new: applies `std::fs::canonicalize` when the path exists, while
old: runs `check_read` on the home, on every OS, and `check_path`
new: runs `check_read` on the home and `check_path`

@fact id=fct-lj2i7oks expect_rev=1
old: `TERM` exactly `dumb`, a non-terminal stdout, or a failed Windows VT enable (`GetConsoleMode`, then ` SetConsoleMode(mode | 0x0004) ` on stdout, the saved mode restored on drop; never viola-pty's `0x000c`). They
new: `TERM` exactly `dumb`, or a non-terminal stdout. They
old: `fn decide` / `fn read` / `mod vt`, called
new: `fn decide` / `fn read`, called

@fact id=fct-g433mics expect_rev=1
old: reads the outer terminal: on Unix the child's bytes (`e4 b8 ad`) reach it, and on every OS none of
new: reads the outer terminal: the child's bytes (`e4 b8 ad`) reach it, and none of
old: `hint:` appears on it; the Windows byte reading is owed to "Windows-only live measurements".
new: `hint:` appears on it.

@fact id=fct-q67nxnld expect_rev=1
old: hands portable-pty an absolute path, because portable-pty 0.8.1's own Windows search tries the exact name first (it would hit the extensionless npm sh shim). On Windows the PATH search tries PATHEXT extensions only, never the bare name. A `claude.cmd`/`claude.bat` (case-insensitive) resolves to the fixed sibling `<dir>/node_modules/@anthropic-ai/claude-code/bin/claude.exe` when that file exists; the shim is never read or parsed.
new: hands portable-pty an absolute path: a given path is used as given and a bare name is looked up on `PATH`; a program not found is `ProgramNotFound` (under `viola revive --json` the document `internal-error`, before `spawn_child`).
old: Every other `.cmd`/`.bat`, including a `claude.cmd` without that sibling, is refused with the closed `AgentError::BatchScriptChild`: exit 1, no child spawned, `process-exit{subject:"self", exit_code:1, detail:"batch-script-child"}`, and the two fixed `unable:`/`hint:` stderr lines with no path or pid (under `viola revive --json`, through `refused_with`, the document `{"v":1,"error":"batch-script-child","detail":null}` on stdout in place of the pair, stderr empty, and a `claude` not found prints `internal-error`, each before `spawn_child`) — because portable-pty 0.8.1 builds its own `CreateProcessW` command line without std's BatBadBut escaping. `viola run` sets
new: `viola run` sets

@fact id=fct-m6zdbeoz expect_rev=1
old: (`viola hook statusline`, Unix only)
new: (`viola hook statusline`)
old: (`viola_agent_claude::statusline::shell_argv`); on Windows that function returns `None` for every input and no command runs. The arm's
new: (`viola_agent_claude::statusline::shell_argv`). The arm's
old: relied on Unix only and stamped
new: relied on every build and stamped
old: `StatusLine` fixtures; the Windows shell-out is owed to "Windows-only live measurements".
new: `StatusLine` fixtures.

@fact id=fct-jzj7djty expect_rev=1
old: Unix: 0700 directories
new: 0700 directories
old: access control). On Windows a plain home made before viola (`create_dir_all`) is refused `Writable`, the refused entry an inherited allow ACE that grants a write right to a SID the check does not trust (read by the right, whichever SID holds it), so such a home is one the check refuses (as measured on `windows-2025` at viola-0.1.0/chunks/2026-10-27-pinned-copy-race-and-reproducible-builds/report.md).
new: access control).
old: It is written only on Unix, and only when every character
new: It is written only when every character
old: for any other path, and on Windows, no override is written
new: for any other path no override is written
old: relayed verbatim by the operator). The Windows half is cut by his word the same day and owed to the working-route entry "Windows-only live measurements".
new: relayed verbatim by the operator).
old: Before reusing an existing `bin/<version>-<hash>/viola(.exe)`,
new: Before reusing an existing `bin/<version>-<hash>/viola`,
old: no `cc` on any of the three `[graph]` triples
new: no `cc` on the one `[graph]` triple (`x86_64-unknown-linux-gnu`)
old: `typed/statusline.json` (Unix);
new: `typed/statusline.json`;
old: SIGKILL, a crash, a failed handler install and a Windows close, logoff or shutdown leave it
new: SIGKILL, a crash or a failed handler install leave it
old: The stop is Unix `SIGINT` / `SIGTERM` and Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT` (`cmd::verify::stop`
new: The stop is `SIGINT` / `SIGTERM` (`cmd::verify::stop`
old: SIGKILL, a crash, a failed handler install (ignored, fail-open) and a Windows close, logoff or shutdown keep
new: SIGKILL, a crash or a failed handler install (ignored, fail-open) keep

@fact id=fct-l66ara67 expect_rev=1
old: an absent `bin/<version>-<hash>/viola(.exe)` by
new: an absent `bin/<version>-<hash>/viola` by
old: keeps only the plugin files and the ConPTY companions.
new: keeps only the plugin files.

@fact id=fct-gfsndi6o expect_rev=1
old: a file takes the read rule on Unix and the write rule on Windows;
new: a file takes the read rule;

@fact id=fct-cnkeyzag expect_rev=1
old: both separator spellings and case-folded on Windows, → `~`
new: both separator spellings, matched exactly, → `~`

@fact id=fct-zaeoull5 expect_rev=1
old: `verify` writes `batch-script-child`, `program-not-found`,
new: `verify` writes `program-not-found`,

@fact id=fct-uz6mlfv5 expect_rev=1
old: status line in Runs C and D and in Run B on Windows (on Unix Run B's status line
new: status line in Runs C and D (Run B's status line

@fact id=fct-o63eny7e expect_rev=1
old: for three target triples in `deny-sync.toml`
new: for the one target triple `x86_64-unknown-linux-gnu` in `deny-sync.toml`

@fact after=fct-o63eny7e as=n1
- `cargo deny check advisories` reads red on RUSTSEC-2026-0441 alone (`pin-utils 0.1.0` unmaintained, reached through `portable-pty =0.8.1` → `nix 0.25.1` from `viola-pty`), in CI's `supply-chain` job and locally (as measured at viola-0.1.0/chunks/2026-11-05-linux-only-code-and-ci/report.md). Its owner is the CARRY on "Version done-check", whose remedy moves `portable-pty` off `=0.8.1` to a release whose graph drops `pin-utils`; no ignore is widened, the `[advisories]` ignore list keeping its one entry.

@fact after=@n1 as=n2
- The `portable-pty =0.8.1` pin carries no Windows reason: portable-pty 0.9.0's Windows garbage-read bug reaches no code viola builds, and "Version done-check" measures moving off the pin.

@fact after=fct-pgy5bb7v as=n3
- A `[[licenses.exceptions]]` entry naming a crate absent from the graph is dropped, never kept: cargo-deny warns `license-exception-not-encountered` for it, and such an exception narrows nothing and widens nothing.

@pattern id=sec4-01
fixed: the plain drive form on Windows
@pattern id=sec4-02
fixed: on Linux and Windows, the server check
@pattern id=sec4-03
fixed: `stale-heartbeat` · `batch-script-child`
@pattern id=sec4-04
fixed: only the verbatim-drive prefix stripped
@pattern id=sec4-05
fixed: a failed Windows VT enable
@pattern id=sec4-06
fixed: the Windows byte reading is owed to
@pattern id=sec4-07
fixed: the PATH search tries PATHEXT extensions only
@pattern id=sec4-08
fixed: is refused with the closed `AgentError::BatchScriptChild`
@pattern id=sec4-09
fixed: on Windows that function returns `None`
@pattern id=sec4-10
fixed: relied on Unix only
@pattern id=sec4-11
fixed: the Windows shell-out is owed to
@pattern id=sec4-12
fixed: a Windows close, logoff or shutdown
@pattern id=sec4-13
fixed: Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT`
@pattern id=sec4-14
fixed: and on Windows, no override is written
@pattern id=sec4-15
fixed: The Windows half is cut by his word
@pattern id=sec4-16
fixed: no `cc` on any of the three `[graph]` triples
@pattern id=sec4-17
fixed: On Windows a plain home made before viola
@pattern id=sec4-18
fixed: the inherited `%USERPROFILE%` DACL
@pattern id=sec4-19
fixed: Microsoft's ConPTY companions, embedded in the exe
@pattern id=sec4-20
fixed: only the plugin files and the ConPTY companions
@pattern id=sec4-21
fixed: the write rule on Windows
@pattern id=sec4-22
fixed: case-folded on Windows
@pattern id=sec4-23
fixed: `verify` writes `batch-script-child`
@pattern id=sec4-24
fixed: and in Run B on Windows
@pattern id=sec4-25
fixed: for three target triples
