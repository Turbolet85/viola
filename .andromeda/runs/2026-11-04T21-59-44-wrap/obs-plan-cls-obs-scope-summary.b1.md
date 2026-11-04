@answer n=1 entity=obs-scope-summary#1.1 v=no
why: the session adds no telemetry surface and changes no process role or product part, with no src file moved
@answer n=2 entity=obs-scope-summary#1.2 v=no
why: no crate is added, removed or made instrumentable, and the session touches only tests, scripts and .config
@answer n=3 entity=obs-scope-summary#1.3 v=no
why: no span, field or emitting call site changed, so the six must-trace paths and four edge flows stand
@answer n=4 entity=obs-scope-summary#1.4 v=no
why: the trigger count and the trigger names stand, since the session adds or removes no trigger and changes no exporter
@answer n=5 entity=obs-scope-summary#2.1 v=yes mode=stale
why: the session moved the Frozen guard and rewired its four callers across reached test files, and the report does not show the tests' <home>/diagnostics/ root still holds on those changed files
@answer n=6 entity=obs-scope-summary#2.2 v=no
why: the session changes no mcp code or mcp log destination, and mcp is not among the files it reached
@answer n=7 entity=obs-scope-summary#2.3 v=no
why: no event is added to the closed enum, schemas/diag-line.v1.json did not move, and the chaos witness adds a test rather than an event
@answer n=8 entity=obs-scope-summary#2.4 v=no
why: the session changes no hook invocation or dialog_id assignment, and its fake agent edit moves only the SIGHUP reader
@answer n=9 entity=obs-scope-summary#3.1 v=no
why: the clap, vt100 and anyhow stack entries are unchanged, with no Cargo.toml or Cargo.lock move and no src change
@answer n=10 entity=obs-scope-summary#3.2 v=no
why: the crate's instrumentability holds, since no src file moved and the session changes no instrumented behaviour
@answer n=11 entity=obs-scope-summary#3.3 v=no
why: the anyhow error-capture boundary and the chain-to-diagnostics rule are unchanged, since no src file moved and no error path changed
@answer n=12 entity=obs-scope-summary#4.1 v=no
why: viola_core is not in the session's file list and no src file moved
@answer n=13 entity=obs-scope-summary#4.2 v=no
why: viola_core remains a pure type crate, untouched by the test-only session
@answer n=14 entity=obs-scope-summary#4.3 v=no
why: the UnreachableCause set and its no-log rule are unchanged, since no src file moved
@answer n=15 entity=obs-scope-summary#5.1 v=no
why: portable-pty stays as stated in the stack and Cargo.lock did not move, while the outer-PTY witness only uses the seam
@answer n=16 entity=obs-scope-summary#5.2 v=no
why: the seam stays instrumentable, since the session's PTY use is test-only and changes no product seam
@answer n=17 entity=obs-scope-summary#5.3 v=no
why: the PTY byte stream is still read only inside a test and is never logged, so the never-logged rule holds
@answer n=18 entity=obs-scope-summary#6.1 v=no
why: the IPC method set send, wait, last, answer, pause, release, link, unlink, hook.dialog and hook.event is unchanged
@answer n=19 entity=obs-scope-summary#6.2 v=no
why: the interprocess crate stays instrumentable with no product change
@answer n=20 entity=obs-scope-summary#6.3 v=no
why: the interprocess stack entry and the sync-instrumentation reason hold, since no Tokio or dependency moved
@answer n=21 entity=obs-scope-summary#7.1 v=no
why: the ndjson, atomic-write-file and sysinfo entries are unchanged, and the session's pid and start-time reads are test-side only
@answer n=22 entity=obs-scope-summary#7.2 v=no
why: the crate's instrumentability holds, with no product change
@answer n=23 entity=obs-scope-summary#7.3 v=no
why: Tokio is still banned here and no dependency moved, so the sync reason holds
@answer n=24 entity=obs-scope-summary#8.1 v=no
why: the serde_path_to_error stack entry and its drift-report rule are unchanged, since the session adds no serde path
@answer n=25 entity=obs-scope-summary#8.2 v=no
why: the entity stays instrumentable with no serde or schema change
