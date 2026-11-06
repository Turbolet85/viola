@answer n=1 entity=obs-scope-summary#17.1 v=no
why: the CI/CD Platform source is an arch excerpt the session does not reach, and this entity names only that source
@answer n=2 entity=obs-scope-summary#17.2 v=no
why: CI is not product runtime, so the not-instrumentable value stands after the change
@answer n=3 entity=obs-scope-summary#17.3 v=no
why: the remaining fixtures stay scrub-required data after the Windows and macOS fixtures are removed, so the reason still holds
@answer n=4 entity=obs-scope-summary#18.1 v=no
why: the tracing facade, JSON logger and per-process panic hook name no retired surface (ConPTY, named pipe, TerminateProcess, macOS or Windows DACL)
@answer n=5 entity=obs-scope-summary#18.2 v=no
why: the frontend is N/A and the terminal CLI surface is unchanged
@answer n=6 entity=obs-scope-summary#18.3 v=no
why: the file sink and the refused-check exit paths hold on Linux, since the removed Windows owner and DACL arm is not named here
@answer n=7 entity=obs-scope-summary#18.4 v=yes mode=add-to
why: F1, the Linux-only OS target with the core on any current distribution and CI on ubuntu-latest, joins the local-only service identity record
@answer n=8 entity=obs-scope-summary#18.5 v=yes mode=contradict
why: the macOS no-peer-pid carve-out and the euid-only server-verify-failed reading are retired, since Linux now compares both the server pid and the euid (task item 11)
@answer n=9 entity=obs-scope-summary#19.1 v=no
why: the tracing spans on the viola-pty seam still hold, since the seam remains with openpty as its one OS API
@answer n=10 entity=obs-scope-summary#19.2 v=no
why: the frontend is N/A and the terminal carries the child's screen as before
@answer n=11 entity=obs-scope-summary#19.3 v=no
why: the run file sink and the no-terminal rule are unchanged, and no retired ConPTY or sideload output is named
@answer n=12 entity=obs-scope-summary#19.4 v=no
why: the process run service identity is unchanged, since no field of it is retired
@answer n=13 entity=obs-scope-summary#19.5 v=no
why: child exit is still detected on the handle wait after the kill-fallback removal (task item 14), so the panic-hook and exit-detection rules hold
@answer n=14 entity=obs-scope-summary#20.1 v=no
why: the hook spans, the hook-invoked and hook-decision lines and the catch_unwind panic line name no retired surface
@answer n=15 entity=obs-scope-summary#20.2 v=no
why: the frontend is N/A
@answer n=16 entity=obs-scope-summary#20.3 v=no
why: the hook file sink and the stdout and stderr rules name no retired surface
@answer n=17 entity=obs-scope-summary#20.4 v=no
why: the hook service identity and its check_instance path hold on Linux, and no Windows arm is named
@answer n=18 entity=obs-scope-summary#20.5 v=yes mode=contradict
why: the Windows owner and DACL reads in the instance check are retired with the Windows owner/DACL arm (task item 25), so the timed-path clause is no longer true
@answer n=19 entity=obs-scope-summary#21.1 v=no
why: the channel tracing spans and corr propagation name no retired surface
@answer n=20 entity=obs-scope-summary#21.2 v=no
why: the frontend is N/A
@answer n=21 entity=obs-scope-summary#21.3 v=no
why: the exporter names per-process log files that the session does not remove
@answer n=22 entity=obs-scope-summary#21.4 v=no
why: the service identity and the params.sender peer version are unchanged
@answer n=23 entity=obs-scope-summary#21.5 v=no
why: the Unix accept loop still drops a refused peer euid before any frame on Linux, and the named-pipe retirement does not reach it
@answer n=24 entity=obs-scope-summary#22.1 v=no
why: the MCP tool spans and their viola-channel client name no retired surface, and the Tokio ban stays
@answer n=25 entity=obs-scope-summary#22.2 v=no
why: the frontend is N/A
