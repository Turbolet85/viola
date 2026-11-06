@apply_row n=1 ref_file=fanout-security-plan--cascade-g1.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-security-plan--cascade-g1.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-security-plan--cascade-g1.md ref_n=3 v=applied edit=E3
@apply_row n=4 ref_file=fanout-security-plan--cascade-g1.md ref_n=4 v=applied edit=E4
@apply_row n=5 ref_file=fanout-security-plan--cascade-g1.md ref_n=6 v=applied edit=E5
@apply_row n=6 ref_file=fanout-security-plan--cascade-g1.md ref_n=7 v=applied edit=E6
@apply_row n=7 ref_file=fanout-security-plan--cascade-g1.md ref_n=9 v=folded_into to_file=fanout-security-plan--cascade-g1.md to_n=7
@fact id=fct-33o4e7zi expect_rev=1
old: `bin/<version>-<hash>/viola(.exe)` is the pinned executable.
new: `bin/<version>-<hash>/viola` is the pinned executable.

@fact id=fct-t7qeqkiu expect_rev=1
old: One endpoint per `viola run`: named pipe `\\.\pipe\viola-<h12>` on Windows, socket file `<dir>/viola-<h12>.sock` on Unix, `<dir>` the per-user 0700 directory (§
new: One endpoint per `viola run`: socket file `<dir>/viola-<h12>.sock`, `<dir>` the per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/` or `/tmp/viola-<uid>/`) (§
old: How that is enforced per OS (pipe DACL, socket file mode, directory permissions) is
new: How that is enforced (socket file mode, directory permissions) is
old: On Unix the socket sits in the per-user 0700 directory, not in world-writable `/tmp`; a directory another user owns, a loosened one or a symlink is refused by the socket-directory check, with no bind on `run` and no frame on a client.
new: The socket sits in the per-user 0700 directory, not in world-writable `/tmp`.
old: On Windows, the named-pipe namespace is shared across the machine.
new: A socket directory another user owns, a loosened one or a symlink is refused by the socket-directory check, with no bind on `run` and no frame on a client.
old: As built, on Linux and Windows every frame-writing client
new: As built, every frame-writing client
old: ahead of that comparison, on Linux and macOS, each runs
new: ahead of that comparison each runs
old: (a refusal `server-verify-failed`, no frame); macOS, where the peer reports no pid, compares no pid or start time, the named carve-out, owned by no route entry.
new: (a refusal `server-verify-failed`, no frame).
old: `server-verify-failed` (Linux, Windows and macOS, on macOS an euid mismatch alone);
new: `server-verify-failed`;

@fact id=fct-qxr7dgpn expect_rev=1
old: holds the home to the read rule on every OS (`check_read`)
new: holds the home to the read rule (`check_read`)

@fact id=fct-feql22h5 expect_rev=1
old: Code-bearing artefacts: `bin/<version>-<hash>/viola(.exe)` and
new: Code-bearing artefacts: `bin/<version>-<hash>/viola` and
old: the strict-modes check (Unix owner and mode, Windows owner and DACL) before
new: the strict-modes check (owner and mode) before
old: the pinned `viola(.exe)`,
new: the pinned `viola`,
old: its `hooks/` and `.claude-plugin/`), and on Windows `pin_companions` checks `bin/<version>-<hash>/conpty/` and each companion before the held open; `marketplace/`
new: its `hooks/` and `.claude-plugin/`); `marketplace/`

@fact id=fct-uemrwfo5 expect_rev=1
old: resolving `claude` from their own PATH with npm-shim → `claude.exe` resolution.
new: resolving `claude` as a bare name on their own PATH.
old: `<cwd>/.viola-verify-<pid>/`; on a wrapped OS (Unix), and only when the pinned path
new: `<cwd>/.viola-verify-<pid>/`; only when the pinned path
old: (Hook stdin row); Windows passes none, Runs A, C and D keep their arguments
new: (Hook stdin row); Runs A, C and D keep their arguments

@fact id=fct-v2lh3e2i expect_rev=1
old: on a native matrix of `windows-2025`, `macos-latest` and `ubuntu-latest`;
new: on the one-leg matrix `[ubuntu-latest]` (every job and artifact name keeps its `${{ matrix.os }}` form);
old: the per-OS `rmcp features` lint step
new: the `rmcp features` lint step
old: (per OS under coverage)
new: (under coverage)
old: the per-OS `perf` job
new: the `perf` job
old: the per-OS release build
new: the release build
old: mutation testing moved to the epoch-boundary code audit.
new: mutation testing moved to the epoch-boundary code audit. The Windows and macOS code, tests and CI workflows stand recoverable at the git tag `multi-os-final` (`ecbe7ab3`) on origin, which holds `src/conpty.rs` and `.github/workflows/windows-mutants.yml`.

@pattern id=sec2-01
fixed: viola(.exe)` is the pinned executable
@pattern id=sec2-02
fixed: named-pipe namespace is shared
@pattern id=sec2-03
fixed: where the peer reports no pid
@pattern id=sec2-04
fixed: on macOS an euid mismatch alone
@pattern id=sec2-05
fixed: Windows owner and DACL
@pattern id=sec2-06
fixed: `pin_companions` checks
@pattern id=sec2-07
fixed: npm-shim → `claude.exe` resolution
@pattern id=sec2-08
fixed: Windows passes none
@pattern id=sec2-09
fixed: native matrix of `windows-2025`
