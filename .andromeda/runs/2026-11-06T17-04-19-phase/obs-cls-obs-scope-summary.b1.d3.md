@answer n=1 entity=obs-scope-summary#1.1 v=no
why: the count of 5 surface entries across 4 roles and 2 parts is not re-measured or changed by the chunk.
@answer n=2 entity=obs-scope-summary#1.2 v=no
why: the count of 6 instrumentable crates is unchanged, since the chunk adds no crate.
@answer n=3 entity=obs-scope-summary#1.3 v=yes
why: the Linux-only OS target is the chunk's goal, removing the Windows and macOS remnants and keeping CI on ubuntu-latest.
@answer n=4 entity=obs-scope-summary#1.4 v=no
why: the 6 must-trace paths and 4 edge flows are neither re-measured nor touched by any chunk edit or scope item.
@answer n=5 entity=obs-scope-summary#1.5 v=no
why: the 18-trigger count is unchanged, and the Linux compat trigger is named only in passing without its rule.
@answer n=6 entity=obs-scope-summary#2.1 v=yes
why: the live run's evidence path diagnostics/run-<name>.ndjson (item 10) and the home-level diagnostics rule (item 11) depend on which diagnostics/ root applies.
@answer n=7 entity=obs-scope-summary#2.2 v=no
why: the mcp stderr fallback is not touched by any chunk edit, and no scope item names mcp.
@answer n=8 entity=obs-scope-summary#2.3 v=no
why: the chunk adds no event, and the release-from-driver and chaos enum extensions are not named by its live-session scope.
@answer n=9 entity=obs-scope-summary#2.4 v=yes
why: the live dialog read (item 12) reads hook-invoked written after the hook.dialog response, which is this corr timing.
@answer n=10 entity=obs-scope-summary#3.1 v=no
why: a source citation with no rule of its own, and the chunk bumps no version the Stack line names.
@answer n=11 entity=obs-scope-summary#3.2 v=yes
why: the chunk edits plain_drive and the CLI output tests in this crate, so the crate's instrumented surface applies to those units.
@answer n=12 entity=obs-scope-summary#3.3 v=yes
why: the chunk edits CLI code at the anyhow error boundary, and item 3's error-enum fold feeds the fixed-Display scrubbing before stderr that this reason states.
@answer n=13 entity=obs-scope-summary#4.1 v=no
why: a source citation with no rule of its own.
@answer n=14 entity=obs-scope-summary#4.2 v=no
why: the not-instrumentable classification of viola-core states no behaviour the chunk's code or tests must honour.
@answer n=15 entity=obs-scope-summary#4.3 v=yes
why: the closed UnreachableCause set includes socket-dir-refused, the detail code whose wording item 14 ratifies or re-words.
@answer n=16 entity=obs-scope-summary#5.1 v=yes
why: the viola-pty entity row names portable-pty, which the version bump makes stale (item 4).
@answer n=17 entity=obs-scope-summary#5.2 v=yes
why: the PTY seam is named by the scope (item 6 boundary wrapper and process-start and process-exit events), and this classifies it instrumentable.
@answer n=18 entity=obs-scope-summary#5.3 v=yes
why: openpty as boundary-only and the never-logged byte stream are what item 1's openpty-alone pty_backend and item 6's text_bytes-only paste rely on.
@answer n=19 entity=obs-scope-summary#6.1 v=yes
why: the IPC verb list carries answer and hook.dialog, the calls the dialog-hold read (item 12) and the channel-handle no-leak re-measure use.
@answer n=20 entity=obs-scope-summary#6.2 v=yes
why: the channel crate is named by the no-leak re-measure and the dialog scenario, so its instrumented events are what the chunk reads.
@answer n=21 entity=obs-scope-summary#6.3 v=yes
why: Tokio is banned in this graph, the rule item 5's Tokio-free sync-crate check holds the new portable-pty graph to.
@answer n=22 entity=obs-scope-summary#7.1 v=no
why: a source citation only, and the state-store libraries are untouched by the portable-pty bump.
@answer n=23 entity=obs-scope-summary#7.2 v=no
why: the chunk edits no state-store unit, and not_needed item 5 excludes the store's readers and appenders.
@answer n=24 entity=obs-scope-summary#7.3 v=yes
why: Tokio is banned in this graph, the rule item 5's Tokio-free sync-crate check holds the new portable-pty graph to.
@answer n=25 entity=obs-scope-summary#8.1 v=no
why: a source citation for serde_path_to_error drift reports, and the chunk edits no unit or check that it reads.
