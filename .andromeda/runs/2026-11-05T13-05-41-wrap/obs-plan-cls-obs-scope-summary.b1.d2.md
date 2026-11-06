@answer n=1 entity=obs-scope-summary#1.1 v=no
why: the session changes no telemetry surface entry or process role, so the five entries across four roles and two parts still hold
@answer n=2 entity=obs-scope-summary#1.2 v=yes mode=add-to
why: fact F1 (Linux-only OS target with CI on ubuntu-latest, Windows and macOS code recoverable at multi-os-final) joins this crate-scope count, which still reads six
@answer n=3 entity=obs-scope-summary#1.3 v=no
why: the six must-trace paths and four edge flows keep their counts, since the session rewrites scenarios but adds or removes none
@answer n=4 entity=obs-scope-summary#1.4 v=yes mode=stale
why: the multi-OS compat trigger (item 6) loses its windows-2025 and macos-latest legs and keeps only ubuntu-latest, so the multi-OS label moved while the count of 18 holds
@answer n=5 entity=obs-scope-summary#2.1 v=no
why: the diagnostics directory root paths are unchanged, and the session retires only the Windows DACL clause on a diagnostics file's permissions
@answer n=6 entity=obs-scope-summary#2.2 v=no
why: mcp still writes mcp.ndjson with stderr only when that file cannot be opened, and the session changes no mcp log path
@answer n=7 entity=obs-scope-summary#2.3 v=no
why: the event enum extensions are not among the diag-line changes, which touch only the os, sideload_fallback and batch-script-child words
@answer n=8 entity=obs-scope-summary#2.4 v=no
why: no session change names hook-invoked corr timing or the dialog_id assignment it is measured against
@answer n=9 entity=obs-scope-summary#3.1 v=no
why: the CLI crate's Stack line names clap, vt100, thiserror and anyhow, none of which the session removes
@answer n=10 entity=obs-scope-summary#3.2 v=no
why: the CLI crate stays instrumentable, since no retired control sits on its surface as the instrumentability value rests on it
@answer n=11 entity=obs-scope-summary#3.3 v=no
why: anyhow stays the only error-chain crate, and the retired typed errors PtyError Load, PinError StrictModes and AgentError BatchScriptChild are thiserror types in other crates
@answer n=12 entity=obs-scope-summary#4.1 v=no
why: the viola-core Source line names the arch excerpt and Workspace modules, which the session does not change
@answer n=13 entity=obs-scope-summary#4.2 v=no
why: viola-core stays a pure type crate with no I/O, and the session adds no runtime behaviour to it
@answer n=14 entity=obs-scope-summary#4.3 v=no
why: the six exit-21 cause words all still hold on Linux, since strict-modes-failed remains for the Unix mode check and no retired word is among the six
@answer n=15 entity=obs-scope-summary#5.1 v=no
why: the viola-pty Source line names portable-pty, which stays pinned at =0.8.1
@answer n=16 entity=obs-scope-summary#5.2 v=no
why: viola-pty stays instrumentable, and its instrumentability value does not depend on the removed ConPTY or TerminateProcess arms
@answer n=17 entity=obs-scope-summary#5.3 v=yes mode=contradict
why: the session removes the ConPTY arm behind the seam and the TerminateProcess fallback, so openpty becomes the one OS API and the line naming ConPTY and openpty as boundary APIs no longer holds
@answer n=18 entity=obs-scope-summary#6.1 v=no
why: the IPC method list and the interprocess Stack entry stand, since the channel keeps the Unix socket as its transport through the same crate
@answer n=19 entity=obs-scope-summary#6.2 v=no
why: viola-channel stays instrumentable on the Unix socket, and the named pipe removal does not alter that value
@answer n=20 entity=obs-scope-summary#6.3 v=no
why: the Tokio ban stays in the deny gate, so instrumentation in viola-channel must still be sync
@answer n=21 entity=obs-scope-summary#7.1 v=no
why: sysinfo and atomic rename remain on the OS APIs boundary, so the viola-state Source line still holds
@answer n=22 entity=obs-scope-summary#7.2 v=no
why: viola-state stays instrumentable, since only its Windows owner and DACL arm leaves and the value does not rest on it
@answer n=23 entity=obs-scope-summary#7.3 v=no
why: the Tokio ban over viola-state stays, so the sync-instrumentation reason still holds
@answer n=24 entity=obs-scope-summary#8.1 v=no
why: the serde_path_to_error drift reporting is named by no session change
@answer n=25 entity=obs-scope-summary#8.2 v=no
why: the instrumentability of this crate is unchanged by the session, since no retired item names it
