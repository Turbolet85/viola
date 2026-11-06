@leaf_group group=security-plan
@leaf_read leaf=.claude/docs/security-summary.md lines=77
last: **Full plan:** `.andromeda/security-plan.md`.
@leaf_read leaf=.claude/rules/security.md lines=44
last: _This section is owned by `/wrap-session`. setup-project pre
@leaf_edit n=1 leaf=.claude/docs/security-summary.md line=10 class=stale
leaf-text: OS-user ambient trust enforced per OS (IPC endpoints + `~/.viola/`)
source: fct-qkuk7pih «**Auth approach:** OS-user ambient trust, enforced on Linux, on the IPC endpoints and `~/.viola/`.»
replace-with: OS-user ambient trust, enforced on Linux, on the IPC endpoints and `~/.viola/`
@leaf_edit n=2 leaf=.claude/docs/security-summary.md line=14 class=retired
leaf-text: Windows protected SDDL (user SID + SYSTEM), `accept_remote(false)`; Unix 0700 per-user socket dir
source: fct-dthgjfbs «IPC access control (Unix) — the socket lives in a per-user 0700 directory, checked before bind and before any client connect (owner, mode, not a symlink), and is chmod 0600 right after the bind»
replace-with: 0700 per-user socket dir
@leaf_edit n=3 leaf=.claude/docs/security-summary.md line=15 class=retired
leaf-text: before the first frame; Windows clients open with SQOS Identification. As built, on Linux and Windows every frame-writing client
source: fct-v2lh3e2i «The Windows and macOS code, tests and CI workflows stand recoverable at the git tag `multi-os-final` (`ecbe7ab3`) on origin»
replace-with: before the first frame. As built, every frame-writing client
@leaf_edit n=4 leaf=.claude/docs/security-summary.md line=15 class=retired
leaf-text: ahead of that comparison, on Linux and macOS, each runs
source: fct-v2lh3e2i «The Windows and macOS code, tests and CI workflows stand recoverable at the git tag `multi-os-final` (`ecbe7ab3`) on origin»
replace-with: ahead of that comparison each runs
@leaf_edit n=5 leaf=.claude/docs/security-summary.md line=15 class=retired
leaf-text: no frame); macOS, where the peer reports no pid, compares no pid or start time, the named carve-out, owned by no route entry.
source: fct-6cauxi6n «verifying the server against a `snapshot.json` that passed the strict-modes check in the same process (the socket-directory, pid, start-time and euid checks, none skipped)»
replace-with: no frame).
@leaf_edit n=6 leaf=.claude/docs/security-summary.md line=17 class=retired
leaf-text: strict-modes check (Unix mode/owner; Windows owner + DACL) at every entry point
source: fct-65crhg3m «or a trusted file itself (`snapshot.json`, `ledger/stamps.json`, the pinned `viola`) is group- or world-writable or not owned by euid (ssh StrictModes style).»
replace-with: strict-modes check (mode/owner) at every entry point
@leaf_edit n=7 leaf=.claude/docs/security-summary.md line=17 class=retired
leaf-text: `diagnostics/`; a home created outside `%USERPROFILE%` gets the protected user + SYSTEM DACL at creation; pinned exe
source: fct-v2lh3e2i «The Windows and macOS code, tests and CI workflows stand recoverable at the git tag `multi-os-final` (`ecbe7ab3`) on origin»
replace-with: `diagnostics/`; pinned exe
@leaf_edit n=8 leaf=.claude/docs/security-summary.md line=17 class=retired
leaf-text: a copy another start placed first included; the ConPTY companions (`bin/<key>/conpty/`) held to the owner/DACL strict-modes check (`check_present`, before the held open) and fully re-hashed against compiled pins through `FILE_SHARE_READ`-only handles held until the spawn, a refused check or a mismatch falling back to the inbox ConPTY; plugin files rewritten each start, and on Unix the instance's `settings.json` statusline override (0600, only for a pinned path of plain characters; none on Windows);
source: fct-7aw7emh6 «the plugin files and the settings override are rewritten at every start, the override written only for a pinned path of plain characters, since a shell runs it unquoted»
replace-with: a copy another start placed first included; plugin files rewritten each start, and the instance's `settings.json` statusline override (0600, only for a pinned path of plain characters);
@leaf_edit n=9 leaf=.claude/docs/security-summary.md line=18 class=retired
leaf-text: **Planted `conpty.dll` in the CWD or on PATH** (a bare-name DLL load) → every `viola` process restricts its DLL search to System32 as the second statement of `main`; the sideloaded `conpty.dll` is pre-loaded only by absolute path.
source: fct-v2lh3e2i «The Windows and macOS code, tests and CI workflows stand recoverable at the git tag `multi-os-final` (`ecbe7ab3`) on origin»
replace-with: **Windows and macOS surfaces** → retired: their code, tests and CI workflows stand recoverable at the git tag `multi-os-final` (`ecbe7ab3`).
@leaf_edit n=10 leaf=.claude/docs/security-summary.md line=36 class=stale
leaf-text: (`LedgerRow::relied()`: all twenty-four on Unix; on Windows, where the status line is not wrapped, the first eighteen, `resume-session`, `agents-join` and `local-command-compact`), so a decision flows only when every relied row passes (`20 pass  4 fail` on Unix or `17 pass  4 fail` on Windows,
source: fct-zvkglgpg «a stamp is read over the relied set (`LedgerRow::relied()`, all twenty-four rows on every build, counter `/24`)»
replace-with: (`LedgerRow::relied()`: all twenty-four on every build, counter `/24`), so a decision flows only when every relied row passes (`20 pass  4 fail`,
@leaf_edit n=11 leaf=.claude/docs/security-summary.md line=46 class=retired
leaf-text: **SQOS spike: passed.** `CreateFileW(SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION | FILE_FLAG_OVERLAPPED)` adopted by interprocess 2.4.4 `Stream::try_from` leaves the server at `SecurityIdentification` (measured; `tests/channel_sqos_open.rs` pins it). `FILE_FLAG_OVERLAPPED` is required (non-overlapped adoption hangs); never fall back to the default connect (it reads `SecurityImpersonation`).
source: fct-v2lh3e2i «The Windows and macOS code, tests and CI workflows stand recoverable at the git tag `multi-os-final` (`ecbe7ab3`) on origin»
replace-with: **SQOS spike:** retired with the Windows code, which stands recoverable at the git tag `multi-os-final` (`ecbe7ab3`).
@leaf_edit n=12 leaf=.claude/docs/security-summary.md line=48 class=stale
leaf-text: (`doctest-file`, `recvmsg` via interprocess)
source: fct-32d4ty7r «`0BSD` is admitted only per crate, by one `[[licenses.exceptions]]` for `doctest-file`, the one 0BSD crate interprocess 2.4.4 pulls into the Linux graph»
replace-with: (`doctest-file`, the one 0BSD crate interprocess 2.4.4 pulls into the Linux graph)
@leaf_edit n=13 leaf=.claude/docs/security-summary.md line=60 class=retired
leaf-text: Windows pipe SDDL = user SID + SYSTEM only; the logon SID is rejected — a same-user driver in another logon session (SSH, scheduled task) must still connect.
source: fct-2pxk6g7w «the socket lives in a per-user directory with mode 0700: `$XDG_RUNTIME_DIR/viola/`, and `/tmp/viola-<uid>/` only as a fallback.»
replace-with: The endpoint is a Unix socket in a per-user 0700 directory (`$XDG_RUNTIME_DIR/viola/`, `/tmp/viola-<uid>/` only as a fallback); the Windows pipe code stands recoverable at the git tag `multi-os-final`.
@leaf_edit n=14 leaf=.claude/docs/security-summary.md line=65 class=retired
leaf-text: `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch`
source: fct-l7hy7pzk «`cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`»
replace-with: `stale-heartbeat` · `pinned-hash-mismatch`
@leaf_edit n=15 leaf=.claude/docs/security-summary.md line=66 class=stale
leaf-text: (relied on Unix only, stamped on 2.1.287)
source: fct-m6zdbeoz «relied on every build and stamped on 2.1.287 with the two `StatusLine` fixtures»
replace-with: (relied on every build, stamped on 2.1.287)
@leaf_edit n=16 leaf=.claude/docs/security-summary.md line=67 class=retired
leaf-text: the per-start `settings.json` override passed with `--settings` (Unix only);
source: fct-7aw7emh6 «the plugin files and the settings override are rewritten at every start, the override written only for a pinned path of plain characters, since a shell runs it unquoted»
replace-with: the per-start `settings.json` override passed with `--settings`;
@leaf_edit n=17 leaf=.claude/docs/security-summary.md line=67 class=stale
leaf-text: relied on Unix only and stamped on 2.1.287 with the two `StatusLine` fixtures; the Windows shell-out is owed to "Windows-only live measurements".
source: fct-m6zdbeoz «relied on every build and stamped on 2.1.287 with the two `StatusLine` fixtures»
replace-with: relied on every build and stamped on 2.1.287 with the two `StatusLine` fixtures.
@leaf_edit n=18 leaf=.claude/docs/security-summary.md line=69 class=retired
leaf-text: Supply chain beyond cargo: the vendored Microsoft ConPTY binaries (`vendor/conpty/1.24.260710001/x64/`, MIT, embedded; founder live ratification, Decisions Log `2026-09-29`) have their own audit, `scripts/conpty-vendor.sh --verify` / `--probe` (nupkg + per-file SHA-256, Authenticode signer), CI `windows-2025` step `ConPTY vendor verification`; four pins, one textual home `src/conpty.rs`.
source: fct-v2lh3e2i «The Windows and macOS code, tests and CI workflows stand recoverable at the git tag `multi-os-final` (`ecbe7ab3`) on origin»
replace-with: Supply chain beyond cargo: the vendored ConPTY audit left with the Windows code, which stands recoverable at the git tag `multi-os-final` (`ecbe7ab3`).
@leaf_edit n=19 leaf=.claude/docs/security-summary.md line=70 class=retired
leaf-text: On a Windows host `run --mutants` wipes the host mutation scratch outside the repository only behind a guard: final component exactly `viola-mutants-scratch`, never the repository or an ancestor (`scratch-refused`, no fallback); the wiped scratch is re-created owner-only through `viola_state::fs::create_private_dir` (the protected user + SYSTEM DACL on Windows), never a plain `create_dir_all`, so a test's instance tree inside it passes the strict-modes check, which is never loosened for it.
source: fct-npplilir «`run --mutants` (local, and the epoch-boundary code audit) derives its base from git history, with `AGENT_RUN_CHUNK_BASE` only a local override.»
replace-with: `run --mutants` (local, and the epoch-boundary code audit) derives its base from git history, with `AGENT_RUN_CHUNK_BASE` only a local override.
@leaf_edit n=20 leaf=.claude/docs/security-summary.md line=72 class=retired
leaf-text: SIGKILL, a crash, a failed handler install and a Windows close, logoff or shutdown leave it to be removed by hand), on Unix, and only when the pinned path
source: fct-zvqsbdzq «SIGKILL, a crash or a failed handler install (ignored, fail-open) keep the OS default and leave the dirs to be removed by hand»
replace-with: SIGKILL, a crash and a failed handler install leave it to be removed by hand), only when the pinned path
@leaf_edit n=21 leaf=.claude/docs/security-summary.md line=72 class=retired
leaf-text: `hook statusline --capture <probe dir>/typed/statusline --shell "$0"`, none on Windows, no run or start added
source: fct-uemrwfo5 «only when the pinned path and the captures dir are shell-plain, it also gets exactly one `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair»
replace-with: `hook statusline --capture <probe dir>/typed/statusline --shell "$0"`, no run or start added
@leaf_edit n=22 leaf=.claude/docs/security-summary.md line=72 class=retired
leaf-text: with the user's own status line in Runs C and D and in Run B on Windows (on Unix Run B's status line is the probe capture
source: fct-uz6mlfv5 «with the user's own status line in Runs C and D (Run B's status line is the probe capture its one probe `--settings` names»
replace-with: with the user's own status line in Runs C and D (Run B's status line is the probe capture
@leaf_edit n=23 leaf=.claude/rules/security.md line=10 class=retired
leaf-text: Windows pipe listener: explicit protected SDDL `D:P(A;;GA;;;<user-SID>)(A;;GA;;;SY)`, `accept_remote(false)`, first-instance, non-inheritable. Client opens with `SECURITY_SQOS_PRESENT | SECURITY_IDENTIFICATION` — never interprocess's default connect.
source: fct-v2lh3e2i «The Windows and macOS code, tests and CI workflows stand recoverable at the git tag `multi-os-final` (`ecbe7ab3`) on origin»
replace-with: No Windows pipe listener: the IPC endpoint is a Linux Unix socket; the Windows and macOS code stands recoverable at the git tag `multi-os-final`.
@leaf_edit n=24 leaf=.claude/rules/security.md line=11 class=stale
leaf-text: (`$XDG_RUNTIME_DIR/viola/`, `$TMPDIR/viola/`, `/tmp/viola-<uid>/`)
source: fct-2pxk6g7w «the socket lives in a per-user directory with mode 0700: `$XDG_RUNTIME_DIR/viola/`, and `/tmp/viola-<uid>/` only as a fallback.»
replace-with: (`$XDG_RUNTIME_DIR/viola/`, and `/tmp/viola-<uid>/` only as a fallback)
@leaf_edit n=25 leaf=.claude/rules/security.md line=12 class=retired
leaf-text: (the peer euid checked too on Unix, never skipped; macOS, where the peer reports no pid, compares no pid or start time, the named carve-out, owned by no route entry)
source: fct-6cauxi6n «verifying the server against a `snapshot.json` that passed the strict-modes check in the same process (the socket-directory, pid, start-time and euid checks, none skipped)»
replace-with: (the socket-directory, pid, start-time and euid checks, none skipped)
@leaf_edit n=26 leaf=.claude/rules/security.md line=12 class=retired
leaf-text: On Windows x64 `run`'s `pin_companions` runs `check_present` on `bin/<version>-<hash>/conpty/` and on each companion that exists before the held open and the SHA-256 re-hash (a refusal writes and loads nothing, the child runs on the inbox ConPTY, `sideload_fallback` `strict-modes-failed`). No dated strict-modes gap remains
source: fct-v2lh3e2i «The Windows and macOS code, tests and CI workflows stand recoverable at the git tag `multi-os-final` (`ecbe7ab3`) on origin»
replace-with: No dated strict-modes gap remains
@leaf_edit n=27 leaf=.claude/rules/security.md line=12 class=retired
leaf-text: on Unix the peer euid decision (`peer_admitted`) and, on Linux and Windows, the server check
source: fct-ju3c6nzj «**Linux identity check:** `peer_creds().euid()` must equal `geteuid()`»
replace-with: the peer euid decision (`peer_admitted`) and the server check
@leaf_edit n=28 leaf=.claude/rules/security.md line=12 class=retired
leaf-text: on Unix the peer euid decision and, on Linux and Windows, the peer pid and its start time
source: fct-ju3c6nzj «and `peer_creds().pid()` (the `SO_PEERCRED` peer pid) must equal the snapshot `pid`, with the sysinfo start-time match.»
replace-with: the peer euid decision and the peer pid and its start time
@leaf_edit n=29 leaf=.claude/rules/security.md line=14 class=retired
leaf-text: Strict-modes check (Unix mode/owner; Windows owner + DACL incl. inherit-only and generic bits, NULL DACL, non-persistent-ACL volume)
source: fct-65crhg3m «or a trusted file itself (`snapshot.json`, `ledger/stamps.json`, the pinned `viola`) is group- or world-writable or not owned by euid (ssh StrictModes style).»
replace-with: Strict-modes check (mode/owner, ssh StrictModes style)
@leaf_edit n=30 leaf=.claude/rules/security.md line=14 class=retired
leaf-text: (Unix any group or other bit; Windows a read grant to another SID, inherit-only included)
source: fct-65crhg3m «also refuse when the home, `instances/<name>/` or its `diagnostics/` grants any group or other permission (`mode & 0o077 != 0`; `strict::unix_read_verdict`)»
replace-with: (any group or other bit, `mode & 0o077 != 0`)
@leaf_edit n=31 leaf=.claude/rules/security.md line=15 class=retired
leaf-text: and on Unix the instance's `settings.json` statusline override (`replace_private`, 0600, only for a pinned path of ASCII letters, digits and `_ - . / :`; none on Windows;
source: fct-7aw7emh6 «the plugin files and the settings override are rewritten at every start, the override written only for a pinned path of plain characters, since a shell runs it unquoted»
replace-with: and the instance's `settings.json` statusline override (`replace_private`, 0600, only for a pinned path of ASCII letters, digits and `_ - . / :`;
@leaf_edit n=32 leaf=.claude/rules/security.md line=16 class=retired
leaf-text: Never spawn a `.cmd`/`.bat` PTY child (portable-pty bypasses BatBadBut escaping) — resolve to the real `.exe`.
source: fct-sgmmo3ki «`viola run` resolves its child itself, a given path as given and a bare name on `PATH`, and hands portable-pty an absolute path»
replace-with: `viola run` resolves its child itself, a given path as given and a bare name on `PATH`, and hands portable-pty an absolute path.
@leaf_edit n=33 leaf=.claude/rules/security.md line=17 class=retired
leaf-text: Run B on Unix gets exactly one probe `--settings <probe dir>/typed/statusline.json` (0600, `replace_private`, only when the pinned path and the captures dir are shell-plain; none on Windows; no run or start added;
source: fct-uemrwfo5 «only when the pinned path and the captures dir are shell-plain, it also gets exactly one `--settings <probe dir>/typed/statusline.json` after its `--plugin-dir` pair»
replace-with: Run B gets exactly one probe `--settings <probe dir>/typed/statusline.json` (0600, `replace_private`, only when the pinned path and the captures dir are shell-plain; no run or start added;
@leaf_edit n=34 leaf=.claude/rules/security.md line=17 class=retired
leaf-text: (Unix `SIGINT` / `SIGTERM`, Windows `CTRL_C_EVENT` / `CTRL_BREAK_EVENT`; each guard removes only what that run created); SIGKILL, a crash, a failed handler install and a Windows close, logoff or shutdown leave them to be removed by hand
source: fct-zvqsbdzq «The stop is `SIGINT` / `SIGTERM` (`cmd::verify::stop`, installed by `viola verify` alone; its handler only sets one flag every wait reads).»
replace-with: (`SIGINT` / `SIGTERM`; each guard removes only what that run created); SIGKILL, a crash and a failed handler install leave them to be removed by hand
@leaf_edit n=35 leaf=.claude/rules/security.md line=18 class=retired
leaf-text: Windows DLL loads: `viola_pty::sideload::restrict_dll_search()` (System32-only) is the second statement of `main` in every role, before any DLL load; the sideloaded `conpty.dll` is pre-loaded only by absolute path. A ConPTY companion is loaded or launched only after the owner/DACL strict-modes check of `bin/<version>-<hash>/conpty/` and each companion (`check_present`, before the held open) and its full SHA-256 re-hash against the compiled pin through a `FILE_SHARE_READ`-only handle held until the spawn; a failing one (a refused check recorded as `sideload_fallback` `strict-modes-failed`) is never overwritten or deleted, and the run degrades to the inbox ConPTY silently (never a refusal, exit change or terminal byte).
source: fct-v2lh3e2i «The Windows and macOS code, tests and CI workflows stand recoverable at the git tag `multi-os-final` (`ecbe7ab3`) on origin»
replace-with: No DLL sideload or ConPTY companion: the Windows code (`src/conpty.rs`) stands recoverable at the git tag `multi-os-final` (`ecbe7ab3`).
@leaf_edit n=36 leaf=.claude/rules/security.md line=23 class=retired
leaf-text: canonicalises a home that exists (Windows: the plain drive form) and creates nothing
source: fct-l7hy7pzk «then `canonical_home` applies `std::fs::canonicalize` when the path exists, while a path that does not exist keeps `std::path::absolute`'s spelling and nothing is created»
replace-with: canonicalises a home that exists and creates nothing
@leaf_edit n=37 leaf=.claude/rules/security.md line=26 class=retired
leaf-text: `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch`
source: fct-l7hy7pzk «`cwd-missing` · `stale-heartbeat` · `pinned-hash-mismatch` · `squatted-name` · `socket-dir-refused` · `internal-error`»
replace-with: `stale-heartbeat` · `pinned-hash-mismatch`
@leaf_edit n=38 leaf=.claude/rules/security.md line=27 class=retired
leaf-text: (Unix only; stdout through `take(MAX_FRAME)`
source: fct-m6zdbeoz «as `/bin/sh -c <command>` with the command as one argument (`viola_agent_claude::statusline::shell_argv`)»
replace-with: (stdout through `take(MAX_FRAME)`
@leaf_edit n=39 leaf=.claude/rules/security.md line=31 class=stale
leaf-text: for interprocess's `doctest-file` / `recvmsg`, a new exception
source: fct-32d4ty7r «`0BSD` is admitted only per crate, by one `[[licenses.exceptions]]` for `doctest-file`, the one 0BSD crate interprocess 2.4.4 pulls into the Linux graph»
replace-with: for interprocess's `doctest-file` (the one 0BSD crate in the Linux graph), a new exception
@leaf_edit n=40 leaf=.claude/rules/security.md line=31 class=retired
leaf-text: lockfiles; the vendored Microsoft ConPTY binaries (`vendor/conpty/<version>/x64/`, MIT, outside every lockfile) have their own audit, `scripts/conpty-vendor.sh --verify` / `--probe` (CI `windows-2025` step `ConPTY vendor verification`), their four pins with one home in `src/conpty.rs`, a version move only by re-vendoring; `zizmor` on workflows.
source: fct-v2lh3e2i «The Windows and macOS code, tests and CI workflows stand recoverable at the git tag `multi-os-final` (`ecbe7ab3`) on origin»
replace-with: lockfiles; `zizmor` on workflows.
