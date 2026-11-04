@answer n=1 entity=obs-scope-summary#8.3 v=yes
why: R8-stripped CLAUDE* values are never logged anywhere, which the secret-scan test over every diagnostics file (need 15) names and checks.
@answer n=2 entity=obs-scope-summary#9.1 v=no
why: a source citation (arch excerpt, Stack rmcp and Workspace), no rule over the chunk's code.
@answer n=3 entity=obs-scope-summary#9.2 v=yes
why: the MCP path's instrumentability is what the macOS carve-out (need 6) acts on for MCP tool-call logging.
@answer n=4 entity=obs-scope-summary#9.3 v=yes
why: no telemetry may go to stdout, which the hook contract under the Unix suite (need 19) and the lint print_stdout job (need 17) must hold.
@answer n=5 entity=obs-scope-summary#10.1 v=no
why: a source citation (portable-pty, Project Intent), no rule over the chunk's code.
@answer n=6 entity=obs-scope-summary#10.2 v=yes
why: the claude child is instrumented at the viola-pty seam only, as need 1 and the process-start logging of need 2 state.
@answer n=7 entity=obs-scope-summary#10.3 v=yes
why: spawn and exit on the process handle and never on EOF, plus paste writes, are what the PTY-seam logging (need 2) must record.
@answer n=8 entity=obs-scope-summary#11.1 v=no
why: a source citation (Hook contract, MCP tools), no rule over the chunk's code.
@answer n=9 entity=obs-scope-summary#11.2 v=yes
why: the hook and MCP boundary is what the hook contract under the Unix suite (need 19) and the macOS MCP carve-out (need 6) instrument.
@answer n=10 entity=obs-scope-summary#11.3 v=yes
why: hook-invoked, hook-decision and MCP tool-call logging, which need 6 carves on macOS, and the closed parse-rejected oversize detail (need 5).
@answer n=11 entity=obs-scope-summary#12.1 v=no
why: a source citation (sysinfo and claude agents --json), no rule over the chunk's code.
@answer n=12 entity=obs-scope-summary#12.2 v=no
why: the claude agents CLI is not run by the chunk, which runs everything on the fake agent, and no task need names it.
@answer n=13 entity=obs-scope-summary#12.3 v=no
why: the claude agents output parse path is outside the chunk's named surfaces, so only a re-check every change owes.
@answer n=14 entity=obs-scope-summary#13.1 v=no
why: a source citation (hook statusline, the only shell-out), no rule over the chunk's code.
@answer n=15 entity=obs-scope-summary#13.2 v=yes
why: the statusline shell-out runs inside the Unix hook suite that need 19 names, so the chunk's tests exercise it.
@answer n=16 entity=obs-scope-summary#13.3 v=yes
why: the statusline shell-out's duration and exit status are the only fields its log may carry, and the command string never leaves diagnostics, which the Unix hook suite (need 19) exercises.
@answer n=17 entity=obs-scope-summary#14.1 v=no
why: a source citation (Stack), no rule over the chunk's code.
@answer n=18 entity=obs-scope-summary#14.2 v=yes
why: the socket directory, strict-modes and peer-euid call sites are what the Unix channel telemetry (need 7) and run's pre-child refusals (need 8) instrument.
@answer n=19 entity=obs-scope-summary#14.3 v=yes
why: the server-verify-failed, socket-dir-refused and strict-modes-failed call sites on Unix, and the macOS euid-only carve-out, are named by needs 5, 6, 7 and 8.
@answer n=20 entity=obs-scope-summary#15.1 v=no
why: a source citation (Stack ndjson and snapshots, Standard Contracts), no rule over the chunk's code.
@answer n=21 entity=obs-scope-summary#15.2 v=yes
why: events.ndjson and snapshot writes through the viola-state call sites are what the one diagnostics schema (needs 4 and 5) must carry on every OS.
@answer n=22 entity=obs-scope-summary#15.3 v=yes
why: the scrub of absolute paths from external output and the snapshot-replayed state lines (need 21) rule over these stores.
@answer n=23 entity=obs-scope-summary#16.1 v=no
why: a source citation (Observability Hints, config management), no rule over the chunk's code.
@answer n=24 entity=obs-scope-summary#16.2 v=no
why: config is static data that the chunk neither names nor changes.
@answer n=25 entity=obs-scope-summary#16.3 v=no
why: the load-time config parse and its skipped-key logging are outside the chunk's named surfaces, so only a re-check every change owes.
