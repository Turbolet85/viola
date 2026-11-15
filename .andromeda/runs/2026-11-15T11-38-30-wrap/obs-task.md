@task_prose part=session
A live round on claude 2.1.325, run on a test home, read the dialog hold near its 3600 s ceiling: the hook raised a question and the wrapper expired it after 3600013 ms, the hook still waiting; the founder ruled hold: stands.
The only product change is text: the dialog hold ceiling constant keeps its value of 3600 s, and its doc comment no longer calls it PROVISIONAL (measured only to about 660 s). It now says it was measured live at the ceiling.
No IPC method, event kind, log field, detail code, socket, env var, flag, config key, dependency or crate was added, changed or retired.
Measured live: on 2.1.325 the real CLI erases the input box on DEL; the folder-trust dialog in a fresh cwd has unnumbered options with the selector on the non-trust row; the user's status row reads the same, at the same place, wrapped and unwrapped.
Measured live: when trust keys pass through the wrapper, wheel human-input comes before session-start in events.ndjson, and a reused instance name keeps the earlier start's lines at the head of its events.ndjson.
A verify recording on a test home whose directory was named home was refused whole as absolute-path; matrix v2-03 is not verified this round; all the rigs live in evidence, outside the product and the release build.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: viola-core, the pure core crate that emits no telemetry; it holds the dialog hold ceiling (3600 s, the upper bound of config.json dialog_hold_s) and its doc comment
@task_item n=1 part=touches
The dialog hold ceiling of 3600 s: any row whose cell holds dialog_hold_s's upper bound, or calls the 3600 s ceiling provisional or measured only to about 660 s, is now stale. The value stands and is measured live at the ceiling (contract/config-json/dialog-hold-s and its kin).
@task_item n=2 part=touches
The hook's 3605 s backstop past the hold: the reading leaves it standing, because the hook was still waiting when the wrapper expired the dialog at 3600013 ms. A row holding that bound is confirmed, not moved.
@task_item n=3 part=touches
The dialog-expired event in events.ndjson (source wrapper) after a question (source hook): read live at the ceiling; a row on dialog-expired gains a live reading beside it.
@task_item n=4 part=touches
The dialog-answered or hook-decision deadline_hit field: read live as deadline_hit true with duration_ms 3600013. A row on deadline_hit or duration_ms on dialog lines gains a live reading.
@task_item n=5 part=touches
An expired dialog returning to the human: the question dialog stays on the screen after the wrapper's expiry (the hook's reply was null). This bears on the row that an expired dialog renders for the human.
@task_item n=6 part=touches
The rule that viola-core emits no telemetry, which rules over the changed component: the change is a doc comment only and adds no line, field or span, so the rule stands.
@task_item n=7 part=touches
The order of run start events in events.ndjson: on a start whose folder-trust keys pass through the wrapper, wheel human-input comes before session-start, and a later human key adds no wheel line. A row holding the start order as wheel start then session-start does not hold on such a start.
@task_item n=8 part=touches
The wheel product event as the audit record, and wheel or human-typing lines: when the trust keys are typed through the wrapper the wheel turns human before session-start, so a later human key writes no wheel line of its own.
@task_item n=9 part=touches
The event stream is never rotated: a reused instance name keeps the earlier start's lines at the head of its events.ndjson (cursors are byte offsets, never rewritten), so a reader of one start's first lines must read that start's own lines.
@task_item n=10 part=touches
Absolute paths scrubbed and Claude fixtures scrub-required: viola verify --record refused a whole recording as absolute-path because the test home's directory was named home, which put /home/ in the probe's cwd. The standing absolute-path refusal holds; a test-home naming fact belongs beside it.
@task_item n=11 part=touches
Integration-test homes under the e2e-home root: a rig must never resolve the target/e2e-home link, and a test home must not be named home. Rows on where test homes live gain this beside them.
@task_item n=12 part=touches
Send refusal input-not-ready and the confirmed send pair send-issued, prompt-submitted, send-confirmed: read live on 2.1.325 after viola release, with the box empty. Also, input-not-ready is not a residue reading.
@task_item n=13 part=touches
Control characters and paste: the real CLI erases the input box on DEL (measured). Rows naming DEL only as a refused paste or send character gain a live fact beside them.
@task_item n=14 part=touches
No telemetry from screen content, and the statusline_command field: the user's status row was read off a local VT screen model only, the same wrapped and unwrapped, with settings unchanged. No product line carries it.
@task_item n=15 part=touches
Local run outputs never uploaded, and test homes kept: the live frames stay only under a gitignored local path and are never committed or uploaded, and only closed-word summaries land. The kept home and frames are the operator's to delete.
@task_item n=16 part=touches
Capability ledger stamps and verify as the only stamps writer: verify stamped claude 2.1.325 with 21 of 21 relied rows passing, on a test home only; the live home's ledger is not stamped for 2.1.325. This bears on the rows on cli_version, the cli_verified run-log field and the never-logged ledger contents.
@task_item n=17 part=touches
Real-CLI verify never runs in CI: the live round and its verify ran on the operator's host, never in CI; CI on the pre-CI commit was green 7 of 7.
@task_item n=18 part=touches
The compact confirmation window and the settle bounds: measured with 2.1.325 as ready_settle_ms 2339, turn_settle_ms 603 and compact_latency_ms 7038. If a row's cell holds a compact window or settle bound, name it against these readings.
@task_item n=19 part=touches
Wrapped Claude internals are not a contract: 2.1.325's folder-trust dialog has unnumbered options with the selector on the non-trust row. This is a CLI form fact that no product line reads.
@task_fact n=1 slug=1-obs-scope-summary
fact: On a start whose folder-trust keys pass through the wrapper, events.ndjson holds wheel human-input before session-start, and a later human key adds no wheel line, since the wheel is already human.
@task_fact n=2 slug=2-telemetry-strategy
fact: A reused instance name keeps the earlier start's lines at the head of its events.ndjson (cursors are byte offsets, never rewritten), so a start's own lines are read from where that start begins, not from line 1.
@task_item n=1 part=untouched
Panic capture, the panic line and panic outcome by role: no panic path, hook or catch site moved.
@task_item n=2 part=untouched
The MCP stdio server, its tools, frames and stdout: no MCP tap start was made this round, and no tool or description changed.
@task_item n=3 part=untouched
The channel JSON-RPC lines, conn and corr correlation, and span coverage and span attributes: no span, field or join key moved.
@task_item n=4 part=untouched
Metrics derived from logs, the perf budgets, hyperfine gates and SLO invariants: no budget, threshold or gate statistic moved.
@task_item n=5 part=untouched
CI gates, the secret scan, artifact uploads and pinned actions: no workflow, gate or upload rule changed.
@task_item n=6 part=untouched
The logging stack, subscriber setup, log levels, the diag-line schema and its detail codes: no logger, level, field or code was added or changed.
@task_item n=7 part=untouched
Absences such as OTel export, network egress, the error reporter and the database: no capability was added or removed.
@task_item n=8 part=untouched
The rig keys typed into its own pty and the status-row hash: these are evidence rigs, not viola, and no row on viola's typed keys or pty byte stream logging is reached.
