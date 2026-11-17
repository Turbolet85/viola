@task_prose part=session
The session took the first live readings on the standing live rig, test-only in viola-e2e: a fired scheduled wake's hook source and wake count, the hook-latency window, the external-imports modal, and a no-key group-kill stop.
It added two rig arms (a wake arm and an imports arm), a hidden rig verb that captures the rig's own UserPromptSubmit payload as closed words and key names only, and a mark witness for the user config with the words unchanged, absent, changed-by-others, changed-by-rig and unread; the old changed word is retired.
It measured that a fired /loop wake carries no source key on claude 2.1.325 and 2.1.326, and that the wrapper's prompt-submitted line reads origin human for every fired wake; this bears on the planned entry A fired scheduled wake keeps the wheel (v2-36), whose fate is asked of the overseer at route-resolve.
It gave the test harness's boot a per-instance ready deadline (20 s by default, the harness's CLI boot keeps 20 s) and put three more viola-e2e test binaries on the ci profile's 20 s kill line.
The live viola verify --record and R2's open placeholder-hint question left the chunk, re-pinned on the 2.1.325 recorded fixture set entry.
No product file, no product crate, no viola verb, flag, output line, exit code, IPC method, env var or dependency changed.
@task_file n=1 path=crates/viola-e2e/src/rig/capture.rs
what: new; the live rig's capture of its own UserPromptSubmit hook payload (closed source words, listed key names, no value) — test-only, no viola verb
@task_file n=2 path=crates/viola-e2e/src/rig/wake.rs
what: new; the live rig's scheduled-wake arms, which drive viola send and read the wrapper's event lines — test-only
@task_file n=3 path=crates/viola-e2e/src/rig/isolate.rs
what: edited; the live rig's isolation of its child and the user-config witness (now the mark witness), its scratch seeds — test-only
@task_file n=4 path=crates/viola-e2e/src/rig/mod.rs
what: edited; the live rig's arms, labels, session and stop (adds the no-key group-kill stop) — test-only
@task_file n=5 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: edited; the live rig binary's own command line (hidden capture verb, --wake and --stand options) — a test tool, not the viola CLI
@task_file n=6 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: edited; the rig rehearsal tests run against the fake agent
@task_file n=7 path=crates/viola-e2e/src/harness/boot.rs
what: edited; the test harness's boot of viola instances and its per-instance ready deadline
@task_file n=8 path=crates/viola-e2e/src/harness/run/perf.rs
what: edited; the test harness's performance run, naming the instance deadline
@task_file n=9 path=crates/viola-e2e/tests/cli.rs
what: edited; the end-to-end tests of the viola CLI verbs, naming the instance deadline at their boot literals (no assertion on CLI output changed)
@task_file n=10 path=crates/viola-e2e/tests/harness_lifecycle.rs
what: edited; the test harness lifecycle tests (the never-ready boot case)
@task_file n=11 path=.config/nextest.toml
what: edited; the test runner's ci profile and its 20 s kill line
@task_item n=1 part=touches
The planned entry A fired scheduled wake keeps the wheel (v2-36): a fired wake reached the wrapper as origin human and with no source key on both CLIs; any wheel row (wheel verb output for viola pause and viola release, the spoken phraseology for wheel verbs) that assumes a fired wake is told apart from a human prompt is unmeasured-to-false; no row is changed, the entry's fate goes to the overseer.
@task_item n=2 part=touches
The live viola verify --record reading (the verify output's --record item) left this chunk unmeasured and is re-pinned on the 2.1.325 recorded fixture set entry; the row stays as written.
@task_item n=3 part=touches
The rig drives viola send and reads its reply as closed code words (the send output's read-back, word column and readback vocabulary state words); used as written, not changed — a picker should confirm no word moved.
@task_item n=4 part=touches
The external-imports modal was read live as a CLI dialog (modal imports, box down, closed with no key): bears on rows that rule what the DIALOG attention word and amber mean only through a ledger note on v2-32; no row text changes.
@task_item n=5 part=touches
R2's open question, whether the CLI's placeholder hint is told apart by a cell attribute, stays unmeasured, owned by the 2.1.325 recorded fixture set entry; it bears on the input-not-ready hint of viola send only as an open reading.
@task_item n=6 part=touches
The wrapper started by viola run under the rig (the run verb and its passed start output) was used as written; the scheduled-wake count read through the wrapper's turn-end lines (v2-35) lands as a ledger note, not on a CLI row.
@task_item n=7 part=touches
The hook-latency window (v2-19: 3 ms, 3 ms, 1 ms) lands as a ledger note; no CLI output row holds a latency value.
@task_item n=8 part=untouched
The web surface's absence: no page or web surface was added.
@task_item n=9 part=untouched
The viola list board, its BAY line, columns, colour depth, truncation and --json document: no board or list output changed.
@task_item n=10 part=untouched
The CLI exit codes, refusal lines, hints and the exit-21 --json details: no code, cause, hint or detail was added or moved.
@task_item n=11 part=untouched
The CLI token rendering, styling and plain output modes (SGR module, TTY detection, no spinner, no banner, no pager): untouched.
@task_item n=12 part=untouched
The viola mcp tools and their results, viola link and unlink output, viola plugin install, viola revive and its refusals, viola last, viola wait and viola --help: no verb, flag or output line changed.
@task_item n=13 part=untouched
The CLI verb set and argument parsing (flat verbs, clap derive, ViolaName targets): the new hidden capture verb and the --wake and --stand options belong to a test-only rig binary, not to the viola CLI.
