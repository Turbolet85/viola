@answer n=1 entity=obs-scope-summary#1.1 v=no
why: the session removes no telemetry surface or process role, so run, hook, mcp and cli all stand.
@answer n=2 entity=obs-scope-summary#1.2 v=no
why: the session removes arms and dependencies but no instrumentable crate, so the crate count holds.
@answer n=3 entity=obs-scope-summary#1.3 v=no
why: the must-trace path and edge-flow counts stand, since E2 changes content and not count, and E1, E3 and E5 are untouched.
@answer n=4 entity=obs-scope-summary#1.4 v=yes mode=stale
why: the multi-OS compat trigger is reworded to ubuntu-latest only, so its multi-OS reading has moved.
@answer n=5 entity=obs-scope-summary#2.1 v=no
why: the diagnostics directory root path is unchanged, and only the Windows owner/DACL arm of the home check is retired.
@answer n=6 entity=obs-scope-summary#2.2 v=no
why: the mcp stderr fallback is not among the retired controls, which are Windows and macOS only.
@answer n=7 entity=obs-scope-summary#2.3 v=no
why: the event enum extensions are not among the removed diag-line words or schema members.
@answer n=8 entity=obs-scope-summary#2.4 v=yes mode=stale
why: src/cmd/hook.rs is reached whole and the report does not show the hook-invoked corr timing still holds.
@answer n=9 entity=obs-scope-summary#3.1 v=no
why: clap, vt100, thiserror and anyhow are not among the named Cargo.toml changes (windows-sys removal and libc move).
@answer n=10 entity=obs-scope-summary#3.2 v=no
why: removing Windows arms and the sideload leaves this crate's sync instrumentability unchanged.
@answer n=11 entity=obs-scope-summary#3.3 v=no
why: anyhow use and the dispatch-only context chain are not moved, since the removed items are Windows arms and typed-error variants.
@answer n=12 entity=obs-scope-summary#4.1 v=no
why: viola-core is a pure type crate and its source citation is unchanged by the session.
@answer n=13 entity=obs-scope-summary#4.2 v=no
why: the type crate stays not-instrumentable, since the session adds no I/O or runtime behaviour to it.
@answer n=14 entity=obs-scope-summary#4.3 v=no
why: exit 21's six UnreachableCause words are unchanged, and the removed strict-modes words are diag-line and Windows-arm words.
@answer n=15 entity=obs-scope-summary#5.1 v=no
why: portable-pty stays pinned at =0.8.1, so the Stack source citation holds.
@answer n=16 entity=obs-scope-summary#5.2 v=no
why: the viola-pty seam stays instrumentable after the ConPTY arm is removed.
@answer n=17 entity=obs-scope-summary#5.3 v=yes mode=contradict
why: the ConPTY arm behind the seam is removed, so openpty is the one OS API and ConPTY is no longer a boundary API.
@answer n=18 entity=obs-scope-summary#6.1 v=no
why: interprocess 2.4.4 stays a dependency and the Standard Contracts IPC methods are unchanged.
@answer n=19 entity=obs-scope-summary#6.2 v=no
why: the channel stays instrumentable on the Unix socket, and its sync instrumentation is unchanged.
@answer n=20 entity=obs-scope-summary#6.3 v=no
why: the Tokio ban stays in this crate's graph, so the sync-instrumentation reason holds.
@answer n=21 entity=obs-scope-summary#7.1 v=no
why: ndjson, atomic-write-file and sysinfo 0.39.6 remain in the boundary list.
@answer n=22 entity=obs-scope-summary#7.2 v=no
why: viola-state stays instrumentable, since only the Windows owner/DACL arm and windows_read_verdict are removed.
@answer n=23 entity=obs-scope-summary#7.3 v=no
why: the Tokio ban stays for viola-state, so the sync-instrumentation reason holds.
@answer n=24 entity=obs-scope-summary#8.1 v=no
why: the serde_path_to_error drift-report Stack citation is unchanged by the session.
@answer n=25 entity=obs-scope-summary#8.2 v=no
why: the session changes no logging or sink in this crate, so its instrumentability stands.
