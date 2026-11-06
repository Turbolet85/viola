@apply_row n=1 ref_file=fanout-obs-plan--cascade-g3.md ref_n=1 v=applied edit=E1
@apply_row n=2 ref_file=fanout-obs-plan--cascade-g3.md ref_n=2 v=applied edit=E2
@apply_row n=3 ref_file=fanout-obs-plan--cascade-g3.md ref_n=3 v=applied edit=E3
@apply_row n=4 ref_file=fanout-obs-plan--cascade-g3.md ref_n=4 v=applied edit=E4
@apply_row n=5 ref_file=fanout-obs-plan--cascade-g3.md ref_n=5 v=applied edit=E5
@apply_row n=6 ref_file=fanout-obs-plan--cascade-g3.md ref_n=6 v=applied edit=E6
@apply_row n=7 ref_file=fanout-obs-plan--cascade-g3.md ref_n=7 v=applied edit=E7
@apply_row n=8 ref_file=fanout-obs-plan--cascade-g3.md ref_n=8 v=applied edit=E8
@apply_row n=9 ref_file=fanout-obs-plan--cascade-g3.md ref_n=9 v=applied edit=E9
@apply_row n=10 ref_file=fanout-obs-plan--cascade-g3.md ref_n=10 v=applied edit=E10
@apply_row n=11 ref_file=fanout-obs-plan--cascade-g3.md ref_n=11 v=applied edit=E11
@apply_row n=12 ref_file=fanout-obs-plan--cascade-g3.md ref_n=12 v=applied edit=E12
@apply_row n=13 ref_file=fanout-obs-plan--cascade-g3.md ref_n=13 v=applied edit=E13
@apply_row n=14 ref_file=fanout-obs-plan--cascade-g3.md ref_n=14 v=applied edit=E14
@apply_row n=15 ref_file=fanout-obs-plan--cascade-g3.md ref_n=15 v=applied edit=E15
@apply_row n=16 ref_file=fanout-obs-plan--cascade-g3.md ref_n=16 v=applied edit=E16
@apply_row n=17 ref_file=fanout-obs-plan--cascade-g3.md ref_n=17 v=applied edit=E17
@apply_row n=18 ref_file=fanout-obs-plan--cascade-g3.md ref_n=18 v=applied edit=E18
@apply_row n=19 ref_file=fanout-obs-plan--cascade-g3.md ref_n=19 v=applied edit=E19
@fact id=fct-2tnvi4o3 expect_rev=1
old: The peer pid and start-time comparison runs on Linux and Windows. `server-verify-failed` reaches Linux, Windows and macOS, where on macOS it means an euid mismatch alone: macOS reports no peer pid, its named carve-out.
new: The peer pid and start-time comparison runs on Linux, the one target.

@fact id=fct-jvpve25b expect_rev=1
old: `peer_admitted` refuses a peer of another euid, and a peer it cannot read, where euids are reported (`PEER_EUID_REPORTED`, Unix). On Windows no euid is read, and the DACL and the pid comparison stay the controls.
new: `peer_admitted` refuses a peer of another euid and a peer whose euid it cannot read (`peer_euid_admitted(observed, own)` is `observed == Some(own)`).

@fact id=fct-vivmhmma expect_rev=1
old: of `src/cmd/client.rs`, out of the macOS-gated `refused_inside`.
new: of `src/cmd/client.rs`.

@fact id=fct-kohas2np expect_rev=1
old: The peer pid and start-time comparison runs on Linux and Windows; `server-verify-failed` reaches Linux, Windows and macOS, where on macOS it means an euid mismatch alone (macOS compares no peer pid, its named carve-out).
new: The peer pid and start-time comparison runs on Linux, the one target.

@fact id=fct-ts6ky5ry expect_rev=1
old: The peer pid and start-time comparison runs on Linux and Windows; `server-verify-failed` reaches Linux, Windows and macOS, where on macOS it means an euid mismatch alone (macOS compares no peer pid, its named carve-out).
new: The peer pid and start-time comparison runs on Linux, the one target.

@fact id=fct-e6vtcesf expect_rev=1
old: › `run.pin_copy` › `run.conpty_sideload` (Windows) › `run.version_gate`
new: › `run.pin_copy` › `run.version_gate`
old: through `replace_private`, Unix only)
new: through `replace_private`)

@fact id=fct-smfiay44 expect_rev=1
old: (`ok|pinned-hash-mismatch`); the `.cmd`/`.bat` refusal (`batch-script-child`) is decided earlier, by viola-side program resolution (`resolve_program`, after obs init and before the strip plan and `pty.spawn`), and logged as `process-exit{subject:"self", exit_code:1, detail:"batch-script-child"}` with no child spawned and no `process-start{subject:"claude-child"}`;
new: (`ok|pinned-hash-mismatch`);
old: `endpoint_kind` (`named-pipe|unix-socket`)
new: `endpoint_kind` (`unix-socket`)
old: `pty_backend` (`conpty-sideload|conpty|openpty`, from `viola_pty::pty_backend()`; `conpty-sideload` after a successful pre-load)
new: `pty_backend` (`openpty`, from `viola_pty::pty_backend()`)

@fact id=fct-4g2at3oh expect_rev=1
old: `already-live|stale-heartbeat|squatted-name|pinned-hash-mismatch|batch-script-child|socket-dir-refused`
new: `already-live|stale-heartbeat|squatted-name|pinned-hash-mismatch|socket-dir-refused`
old: (D-34), plus, on Windows when the `run.conpty_sideload` outcome is not `loaded`, `sideload_fallback` (`hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built`, D-36). A degraded sideload adds no `process-exit{exit_code:1}` and no panic line.
new: (D-34).
old: `pty_backend` is `{ "type": "string" }`, while `sideload_fallback` is closed in the `process-start` properties to `hash-mismatch|strict-modes-failed|unreadable|load-failed|not-built` (`diag-detail.v1.json` carries neither). A sideload degrade is recorded on the existing info-level lines, never at `warn`, and no companion path or hash reaches a home-level line (D-36).
new: `pty_backend` is `{ "type": "string" }` and `os` is closed to `linux`.

@fact id=fct-lohdixpq expect_rev=1
old: (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out)
new: (the peer pid and its start time, compared on Linux, the one target)

@fact id=fct-gghzntp4 expect_rev=1
old: and before any frame (Linux, Windows and macOS, on macOS an euid mismatch alone):
new: and before any frame:

@fact id=fct-u2zbdxyd expect_rev=1
old: no frame sent; Linux, Windows and macOS, on macOS an euid mismatch alone)
new: no frame sent)

@fact id=fct-l4ierws2 expect_rev=1
old: opens no `channel.request` (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).
new: opens no `channel.request`.

@fact id=fct-hluu43pb expect_rev=1
old: or the check cannot be made (Linux, Windows and macOS, on macOS an euid mismatch alone), or
new: or the check cannot be made, or

@fact id=fct-u5crvqku expect_rev=1
old: and `answer` sends no frame (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).
new: and `answer` sends no frame.

@fact id=fct-a7vqqg3h expect_rev=1
old: and sends no frame (the peer pid and its start time on Linux and Windows; `server-verify-failed` on Linux, Windows and macOS, on macOS an euid mismatch alone, macOS comparing no peer pid, its named carve-out).
new: and sends no frame.

@fact id=fct-gqw4imf3 expect_rev=1
old: records a `statusline_command` and a shell is built (never on Windows, where `shell_argv` is `None`);
new: records a `statusline_command`;

@fact id=fct-6xa7lf3f expect_rev=1
old: name (ASCII case-insensitive on Windows, exact on Unix) except the persistent set (Windows `HKCU`/`HKLM` `Environment` value names; Unix `config.json` `claude_env_keep`)
new: name, matched exactly, except the persistent set (`config.json` `claude_env_keep`)
old: no value is read by the strip, the registry reader or any log
new: no value is read by the strip or any log

@fact id=fct-f3gi2vhc expect_rev=1
old: (Unix `SIGINT` / `SIGTERM`, Windows Ctrl-C / Ctrl-Break)
new: (`SIGINT` / `SIGTERM`)
old: (`LedgerRow::relied()`, `ALL` filtered in its order: a row is relied unless it is one of the three statusline rows where `statusline::WRAPPED_HERE` is false; 24 on Unix, 21 on Windows, the 22nd row `resume-session`, the 23rd `agents-join` and the 24th `local-command-compact` relied on every OS)
new: (`LedgerRow::relied()`, all 24 rows of `ALL` in its order)
old: with one probe `--settings` on Unix,
new: with one probe `--settings`,
old: SIGKILL, a crash and a Windows close, logoff or shutdown leave it
new: SIGKILL and a crash leave it
old: (sole caller `viola verify`'s Run B probe settings, Unix only)
new: (sole caller `viola verify`'s Run B probe settings)

@fact id=fct-l3f7hrdu expect_rev=1
old: refusal (Linux, Windows and macOS, where it means an euid mismatch alone) is one
new: refusal is one

@pattern id=obs3-01
fixed: an euid mismatch alone
@pattern id=obs3-02
regex: (comparison runs|start time) on Linux and Windows
@pattern id=obs3-03
fixed: where euids are reported
@pattern id=obs3-04
fixed: On Windows no euid is read
@pattern id=obs3-05
fixed: macOS-gated `refused_inside`
@pattern id=obs3-06
fixed: `run.conpty_sideload` (Windows)
@pattern id=obs3-07
fixed: `replace_private`, Unix only
@pattern id=obs3-08
regex: \.cmd`/`\.bat` refusal|batch-script-child
@pattern id=obs3-09
fixed: named-pipe|unix-socket
@pattern id=obs3-10
fixed: conpty-sideload|conpty|openpty
@pattern id=obs3-11
regex: sideload_fallback|degraded sideload|sideload degrade
@pattern id=obs3-12
fixed: where `shell_argv` is `None`
@pattern id=obs3-13
fixed: case-insensitive on Windows
@pattern id=obs3-14
regex: HKCU|the registry reader
@pattern id=obs3-15
fixed: Windows Ctrl-C / Ctrl-Break
@pattern id=obs3-16
regex: 21 on Windows|`statusline::WRAPPED_HERE` is false
@pattern id=obs3-17
fixed: probe `--settings` on Unix
@pattern id=obs3-18
fixed: a Windows close, logoff or shutdown
@pattern id=obs3-19
fixed: probe settings, Unix only
