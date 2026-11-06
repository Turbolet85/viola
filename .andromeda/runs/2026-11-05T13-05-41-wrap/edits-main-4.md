@fact id=fct-irhgihut expect_rev=2
old: clap 4.6.7 dispatch (Windows: the System32 DLL-search restriction is its second statement)
new: clap 4.6.7 dispatch
old: `decide`, `read`, the Windows VT enable and its restoring guard);
new: `decide`, `read`);
old: (`write_plugin_installed`, `command_word`: a Windows-gated source, in `windows-mutants.yml`'s `viola-main` job)
new: (`write_plugin_installed`, `command_word`)
old: verify's Ctrl-C / TERM stop (Unix SIGINT / SIGTERM, Windows Ctrl-C / Ctrl-Break)
new: verify's Ctrl-C / TERM stop (SIGINT / SIGTERM)
old: Unix `kill -STOP` polled to `ps` state `T` within `WITHIN`, `kill -CONT` on drop; Windows a debugger attach, detached on drop)
new: `kill -STOP` polled to `ps` state `T` within `WITHIN`, `kill -CONT` on drop)
old: pty seam over portable-pty =0.8.1 (+ windows-sys kill fallback; HostTerminal raw mode: windows-sys Console / libc termios;
new: pty seam over portable-pty =0.8.1 (HostTerminal raw mode: libc termios;
old: host_stdin(): the host stdin (Windows console: viola's own ReadConsoleW reader, every 0x1A kept);
new: host_stdin(): `std::io::stdin()`;
old: (`peer_euid_admitted`, `peer_admitted`, `PEER_EUID_REPORTED`)
new: (`peer_euid_admitted`, `peer_admitted`)
old: (`unix_read_verdict`, `windows_read_verdict`, `READ_RIGHTS`)
new: (`unix_read_verdict`)
old: hook parsing, dialog mapping, R8 strip, shim resolution,
new: hook parsing, dialog mapping, R8 strip, program resolution,
old: agent-run.{sh,ps1}      # identical shims over viola-harness
new: agent-run.sh            # the shim over viola-harness
old: # push + PR: 3-OS test/perf (hyperfine rows + gate --require
new: # push + PR on ubuntu-latest: test/perf (hyperfine rows + gate --require
old: #   perf)/lint (+ module orphans), msrv, fuzz-replay, 3-OS release
new: #   perf)/lint (+ module orphans), msrv, fuzz-replay, release
