@carried n=1 label=obs-plan/obs-scope-summary fact=fct-7bdi4wod
why: The multi-platform-exporter-compat trigger and its per-OS rows must be named in the chunk's plan as expected amendments of the wrap, and the removal must leave every Linux side of each row (Unix 0600/0700, POSIX O_APPEND, Unix socket, openpty, SO_PEERCRED and peer_creds) as it is.
quote: The same file sink and line schema on every OS. Platform differences must still yield identical fields:
    - permission application: Windows DACL vs Unix 0600/0700;
    - append atomicity with concurrent hook writers: Windows append vs POSIX `O_APPEND`;
    - named pipe vs Unix socket in channel logs;
    - ConPTY vs openpty in `pty` spans.
    - the server check's reads: the peer pid, Linux `SO_PEERCRED` vs Windows `GetNamedPipeServerProcessId` vs macOS none (`SERVER_PID_REPORTED` false, so that verdict is `Matches` with no comparison: the named carve-out, owned by no route entry); the peer euid, Unix `peer_creds` (a mismatch reads `server-verify-failed`, on macOS its one meaning) vs Windows none (`PEER_EUID_REPORTED` false, the DACL and the pid comparison the controls).
@carried n=1 run=2 label=obs-plan fact=fct-dao4nifd
quote: the same file sink and line schema on every OS, with identical fields and no OS-specific exporter.
@carried n=1 run=3 label=obs-plan fact=fct-mjdbrsjn
quote: arch CI/CD (windows-2025 / macos-latest / ubuntu-latest)
@carried n=1 run=4 label=obs-plan fact=fct-t2iihkln
quote: PTY seam (spawn, read, write, resize, wait, kill) over portable-pty `=0.8.1`, plus the windows-sys `TerminateProcess` fallback
@carried n=1 run=5 label=obs-plan fact=fct-m24p7svx
quote: the ConPTY and openpty OS APIs behind it are boundary-only
@carried n=1 run=6 label=obs-plan fact=fct-zce67g4u
quote: OS APIs: ConPTY / openpty, named pipe / Unix socket, filesystem locks and atomic rename, sysinfo, `TerminateProcess`
@carried n=1 run=7 label=obs-plan fact=fct-p4bwzizt
quote: JSON-RPC 2.0 over ndjson on a named pipe or Unix socket
@carried n=2 label=obs-plan/obs-scope-summary fact=fct-khwnpdov
why: The Windows arms of `strict::check_diagnostics` leave with the cfg(windows) removal while the Unix rule (no group or other bit, 0600 files in 0700 folders) must keep holding at every open, and the two harness cases the chunk reworks must keep asserting it.
quote: Every file is 0600 inside a 0700 directory (Windows: home DACL), and the tests secret-scan test enforces it. The product holds it at open too: `open_role_file` and `write_detail` run `strict::check_diagnostics` on the existing `diagnostics/` folder and on the role or detail file before anything is created or narrowed — a symlink refused, a folder held to the read rule (Unix: no group or other bit; Windows: no read grant to another SID), a file to the read rule on Unix and the write rule on Windows — and a refusal is a failed open (no role file opened; a detail line dropped).
@carried n=2 run=2 label=obs-plan fact=fct-igz2gfrn
quote: every file is 0600 inside a 0700 directory; there is no rotation in v1.
@carried n=3 label=obs-plan/obs-scope-summary fact=fct-5ckzuzgq
why: The Windows x64 `conpty/` companion line and its `sideload_fallback` `strict-modes-failed` value name sideload code the chunk removes, so P3 must list them and P4 decide them against the masters' change, while the `ledger-stamps` and `verify` lines must stay on Linux.
quote: `diagnostics/` files are 0600 in 0700 directories (Windows: home DACL), enforced by the tests secret-scan test.
    - Refusal telemetry for strict-modes failures, pinned-exe hash mismatches and ignored symlinks. As landed a strict-modes failure has a line in three places: the `ledger-stamps` read in `run`'s version gate, on Windows x64 a refused `conpty/` companion (`PinError::StrictModes`), which `process-start` carries as `sideload_fallback` `strict-modes-failed`, the exit unchanged; and a refused `update_stamps` in `viola verify` (`StampsError::StrictModes`), which leaves only verify's own `process-exit` (with an instance; exit 1, `detail` `strict-modes-failed`).
@carried n=3 run=2 label=obs-plan fact=fct-wrd7y3z2
quote: a strict-modes failure has a line only on the `ledger-stamps` read in `run`'s version gate, on a `conpty/` companion its owner/DACL check refuses (`process-start`'s `sideload_fallback:"strict-modes-failed"`, the child on the inbox ConPTY, exit unchanged) and on `verify`'s own `process-exit` for a refused stamp write (`detail` `strict-modes-failed`, exit 1)
@carried n=4 label=obs-plan/obs-scope-summary fact=fct-ycxx4ij4
why: The Windows and macOS halves of `server-verify-failed` and the macOS euid-only carve-out leave with the arms the chunk removes, while the Linux `server-verify-failed` and `socket-dir-refused` details with exit 21 on CLI `answer` and fail-open on the dialog hook must stay unchanged.
quote: `server-verify-failed` on Linux, Windows and macOS (on macOS an euid mismatch alone) and `socket-dir-refused`, each exit 21 on CLI `answer` with `during:"connect"` and fail-open on the dialog hook with `corr` null and `decision_emitted:false`, no pid, start time, path or endpoint)
@carried n=5 label=obs-plan/obs-scope-summary fact=fct-mxwnyc6t
why: The hook's macOS carve-out and Windows mention leave with their arms, while the Linux `hook-decision` lines and the hook contract the scope names (exit 0, stderr empty) must stay exactly as they are.
quote: A server-verify mismatch (Linux, Windows and macOS, on macOS an euid mismatch alone) is one `hook-decision{detail:"server-verify-failed"}` at WARN with no frame sent — on a dialog hook `hook-invoked{corr:null}` then `hook-decision{corr:null, decision_emitted:false, detail:"server-verify-failed"}` — and no pid, start time, path or endpoint in the line; a refused socket directory takes the same form with `detail` `socket-dir-refused`; macOS compares no peer pid (the named carve-out).
    - Exit is 0 and stderr stays empty.
@carried n=6 label=obs-plan/obs-scope-summary fact=fct-n7xftqkr
why: The CLI verbs' Windows and macOS server-verify text and the macOS carve-out outside the distinct-code rule leave with the macOS arm, while the Linux `process-exit` line with exit 21 and `socket-dir-refused` must keep its shape.
quote: The server-verify fail is written on Linux, Windows and macOS (on macOS an euid mismatch alone) as `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` in `cli-<name>.ndjson` by `send`, `wait`, `last`, `answer`, `pause`, `release`, `link` and `unlink` (for `link` / `unlink` the driven instance's file), distinct from `instance-dead`, with no pid, start time, path or endpoint; macOS compares no peer pid (the named carve-out).
    - The socket-directory cause has its code, `socket-dir-refused`, whatever failed: a refused check, a failed create or a lost race reads that one word. One carve-out stands outside the distinct-code rule: macOS compares no peer pid, named and owned by no route entry.
@carried n=7 label=obs-plan/obs-scope-summary fact=fct-h2wm2goy
why: The `.cmd`/`.bat` child exit-1 cause is the Windows refusal text the scope has P3 list and P4 decide, and every other Linux exit-1 `detail` code must stay as it is within the typed exit set.
quote: Exit-1 causes use a distinct `detail` code: already-live (a live holder or a fresh replayed beat), stale-heartbeat (a stale holder), squatted-name, sha-256 mismatch, `.cmd`/`.bat` child, `socket-dir-refused` (a refused socket directory, on `run` and on a start made by `viola revive`: one `process-exit{subject:"self", exit_code:1}` before any snapshot, child or bind)
@carried n=7 run=2 label=obs-plan fact=fct-y6qmqcu2
quote: each cause behind exit 21 and exit 1 gets a distinct `detail` code with no path or pid
@carried n=8 label=obs-plan/obs-scope-summary fact=fct-yfoyoh37
why: The child's exit must keep being detected on the process handle without EOF on Linux once the Windows `TerminateProcess` fallback leaves viola-pty, and its ConPTY label names code the chunk removes.
quote: child exit detected on the process handle without EOF (ConPTY);
@carried n=8 run=2 label=obs-plan fact=fct-avrouaxx
quote: its spawn and exit (exit detected on the process handle, never on EOF)
@carried n=9 label=obs-plan/obs-scope-summary fact=fct-3og7ujbn
why: The chunk edits the manifests and `deny.toml`, so `cargo deny` must still pass with the Tokio and C-crate bans after the Windows dependencies leave, and "all three targets" is per-OS wording the OS-target change re-points.
quote: Obs dependencies stay Tokio-free in the sync crates and add no C-building crates, and they pass `cargo deny` on all three targets.
@carried n=9 run=2 label=obs-plan fact=fct-r2s52jjf
quote: obs dependencies stay Tokio-free in the sync crates, add no C-building crates and pass `cargo deny`.
@carried n=10 label=obs-plan/obs-scope-summary fact=fct-rkannnby
why: The scope holds the NEVER-log floor unchanged, so the removal of Windows and macOS tests and fixtures must leave every remaining sink and fixture under it.
quote: A never-log floor for `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET` and every R8-stripped `CLAUDE*` value, across all sinks and fixtures.
@carried n=11 label=obs-plan/obs-scope-summary fact=fct-ogkwmfvl
why: The scope fixes the Linux wire contract, the closed words and the hook contract, so no required field may be renamed or removed and the hook must still exit 0 with nothing on stderr.
quote: obs-plan may add fields but must not rename or remove these. The harness greps on them.
  - **Constraints:**
    - The hook trace is the `hook-<name>.ndjson` file. A failed write there is swallowed, so the hook still exits 0 and never writes to stderr.
@carried n=12 label=obs-plan/obs-scope-summary fact=fct-urji5is2
why: The harness `logs` merge is consumed through `scripts/agent-run.sh` and its `.ps1` twin, whose leaving the scope leaves to P3 and P4, and the sh path must keep merging every role's process logs with `events.ndjson`.
quote: Structured JSON process logs from every process role (`run` / `hook` / `mcp` / `cli`) under `diagnostics/`, merged by harness `logs` with `events.ndjson`.
@carried n=12 run=2 label=obs-plan fact=fct-q2zh72zo
quote: The agent-readable consumption path is the tests harness `logs` command (`scripts/agent-run.sh logs` / `.ps1`, via `viola-harness`).
@binding n=1
**obs-scope-summary multi-platform-exporter-compat ↔ ci-integration platform**: the per-OS rows and the three-OS matrix the §9 platform line names, `windows-mutants.yml` included, shrink together to the Linux target in one wrap amendment the plan names as expected, (obs-plan/obs-scope-summary fct-7bdi4wod) and (obs-plan fct-xyexk37w).
@binding n=2
**obs-scope-summary macOS carve-out ↔ span-trace-coverage wait/last scenario**: the macOS euid-only carve-out leaves at every place it is written, the answer fields, the hook fail-open, the CLI exit-21 line and the wait/last scenario, while the Linux peer-pid and euid comparison stays, (obs-plan/obs-scope-summary fct-n7xftqkr), (obs-plan/obs-scope-summary fct-mxwnyc6t) and (obs-plan fct-q67chfay).
@binding n=3
**obs-scope-summary diagnostics 0600/0700 ↔ ci-integration e2e-home**: the reworked free-space harness cases must keep their homes under `target/e2e-home/` and keep asserting the Unix owner-only modes, (obs-plan/obs-scope-summary fct-khwnpdov) and (obs-plan fct-xtfnzpyu).
@binding n=4
**obs-scope-summary sideload_fallback ↔ log-coverage schema default-deny**: whether `sideload_fallback` leaves the field catalog is decided with `schemas/diag-line.v1.json`, which must keep every Linux value and stay at zero conformance failures, (obs-plan/obs-scope-summary fct-5ckzuzgq) and (obs-plan fct-llmksgvm).
@binding n=5
**obs-scope-summary cargo deny ↔ slo-invariants build failure conditions**: a `cargo deny` failure after the Windows dependencies leave is a build failure, (obs-plan/obs-scope-summary fct-3og7ujbn) and (obs-plan fct-5kn2jpbt).
