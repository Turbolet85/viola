@answer n=1 entity=obs-scope-summary#22.3 v=no
why: the file sink and its Unix strict-modes check on diagnostics still hold, and only the Windows owner/DACL arm moved
@answer n=2 entity=obs-scope-summary#22.4 v=no
why: service identity is untouched, as the OTel SDK init and service identity rule holds with no exporter or sink changed
@answer n=3 entity=obs-scope-summary#22.5 v=yes mode=add-to
why: F1 the OS target is Linux only with CI on ubuntu-latest and the Windows and macOS code removed, so it joins this notes list
@answer n=4 entity=obs-scope-summary#23.1 v=no
why: the OTel SDK init, service identity, no-egress rules and the Tokio and C-crate deny bans are untouched, and the init order names no removed arm
@answer n=5 entity=obs-scope-summary#24.1 v=no
why: the tracing-subscriber 0.3.23 pin, the file sink rules and the no RUST_LOG stance are unchanged by the session
@answer n=6 entity=obs-scope-summary#25.0 v=no
why: the event enum and the required process-log fields are unchanged, and the schema edits drop only the os, sideload and batch-script-child words
@answer n=7 entity=obs-scope-summary#26.1 v=yes mode=contradict
why: the Windows home DACL read and write rules named in the diagnostics strict-modes check are retired with the Windows owner/DACL arm
@answer n=8 entity=obs-scope-summary#27.1 v=no
why: the surfaces are cli run, the channel bind and the persistent stores, and all of them remain after the session
@answer n=9 entity=obs-scope-summary#27.2 v=no
why: the must-trace spans, including pty.spawn, and the process-start and process-exit events still exist unchanged
@answer n=10 entity=obs-scope-summary#27.3 v=yes mode=contradict
why: the .cmd/.bat child exit-1 detail is retired with the batch-script-child cause and its run start-refusal rule
@answer n=11 entity=obs-scope-summary#27.4 v=no
why: the sources name the tests, arch and founder directions, and the session changed none of them
@answer n=12 entity=obs-scope-summary#28.1 v=no
why: the send surfaces and the hook confirmations are unchanged, as no send or hook path moved
@answer n=13 entity=obs-scope-summary#28.2 v=no
why: the send must-trace spans, the readiness gate and the paste write are unchanged on Linux
@answer n=14 entity=obs-scope-summary#28.3 v=no
why: the send log fields and detail words are unchanged, and none of them is a removed Windows or sideload word
@answer n=15 entity=obs-scope-summary#28.4 v=no
why: the sources name the tests excerpt, the design and founder directions, and the session moved none of them
@answer n=16 entity=obs-scope-summary#29.1 v=no
why: the wait surfaces are the client, the channel and the run event tail, and all of them are unchanged
@answer n=17 entity=obs-scope-summary#29.2 v=no
why: the wait must-trace spans and the channel-request and channel-response events are unchanged
@answer n=18 entity=obs-scope-summary#29.3 v=no
why: socket-dir-refused stays under the unchanged Unix socket refusals, and endpoint-lost and the exit 21 wait codes are not removed
@answer n=19 entity=obs-scope-summary#29.4 v=no
why: the sources name the tests excerpt, the design and a creator must-work, and the session moved none of them
@answer n=20 entity=obs-scope-summary#30.1 v=no
why: the dialog surfaces and the hook and run paths are unchanged, as the session moves no dialog route
@answer n=21 entity=obs-scope-summary#30.2 v=no
why: the dialog must-trace spans and the hook-decision events are unchanged
@answer n=22 entity=obs-scope-summary#30.3 v=yes mode=contradict
why: the server-verify-failed line names Windows and a macOS euid-only carve-out that the session removes, leaving Linux alone
@answer n=23 entity=obs-scope-summary#30.4 v=no
why: the sources name the tests, arch and a creator must-work, and the session moved none of them
@answer n=24 entity=obs-scope-summary#31.1 v=no
why: the wheel, send-refused and release surfaces are unchanged, and no Windows key or console path is named
@answer n=25 entity=obs-scope-summary#31.2 v=no
why: the pause and release spans and the release-from-driver event stay, and none of them is an OS arm
