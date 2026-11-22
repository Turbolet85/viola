@task_prose part=session
The session measured whether real claude on Linux loses a key typed right after a resize of its pty, and found no loss: claude 2.1.330 (KR) and 2.1.328 (KR8-2) each kept 12 of 12 keys typed 0, 20 or 100 ms after a resize, said as no loss in 12 rounds at these delays on these versions, never as never loses.
No product change followed: the fix half was not needed, no viola-pty, wrapper, passthrough or fake-agent code moved; only the test-only live-round rig changed.
The rig gained a KR live arm (labels KR, KR-2, KR8, KR8-2) that resizes its own pty and types x then DEL into claude's input box, reading the box row's cell attributes only, never text, under a new resize grant word beside box.
The rig's after-key settle was mended for KR alone (quiet counted from a mark taken after the key), telling a late redraw from a dirty box; the shared settle every other arm and the boundary checks use is unchanged, and HB and the earlier arms read as before.
The rehearse verb gained two rehearse-only options for kr (a quiet window of 100 to 400 ms and a withheld or late DEL seam) that the live verb refuses as usage, exit 2; five KR rehearsals over the fake agent were added inside the existing integration suite.
The live KR reading is recorded evidence for the key-after-a-resize item and gates nothing; CI on the pre-CI commit was green 7 of 7.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the live-round rig, the test-only TUI boundary driver that starts real claude in its own pty and reads its screen by signatures and cell attributes: its arms (now KR), its settle, its summary words and its live readings
@task_file n=2 path=crates/viola-e2e/src/rig/keys.rs
what: the live-round rig's key grant, the closed list of keys and actions a live arm may send into its own pty (now a resize grant beside box)
@task_file n=3 path=crates/viola-e2e/src/rig/screen.rs
what: the live-round rig's screen model, which reads claude's box row by cell attributes and signatures, never content, now kept across a resize
@task_file n=4 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the live-round rig's command line, its live and rehearse verbs, its grant and usage refusals
@task_file n=5 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig's rehearsals over the fake agent, run inside the existing integration test suite under nextest
@task_item n=1 part=touches
decision/no-screen-content-parsing: its except list gains the KR arm, which reads claude's box row by cell attributes after each resize and key, never text.
@task_item n=2 part=touches
decision/boundary-check-never-inspects-claude-content: the new KR arm drives real claude and reads only the box row's attributes and closed words, counts, delays and settle times; a row naming which arms read attributes now has one more.
@task_item n=3 part=touches
decision/live-readings-never-gate: the KR and KR8-2 live readings (kept 12 of 12) are recorded evidence only and gate no build; the KR rehearsals over the fake agent are what CI runs.
@task_item n=4 part=touches
contract/tui-passthrough/focus-mouse-resize-sequences: the measurement bears on keys typed right after a resize; claude itself kept every key at 0, 20 and 100 ms, so viola's resize handling needs no wait of its own and the contract stands unchanged, now with a Linux reading beside it.
@task_item n=5 part=touches
contract/tui-passthrough/human-keystrokes and decision/human-keystrokes-never-blocked: a human key typed right after a resize was measured kept on Linux at the CLI unwrapped; no product change, the measurement is not of viola's own poll lag or the driver's paste.
@task_item n=6 part=touches
decision/wheel-ignores-terminal-events and decision/non-editing-sequences: resize is the terminal event the KR arm exercises before each key; neither rule changed, the arm only measures the child's keeping of the key after it.
@task_item n=7 part=touches
decision/tui-boundary-driver-pin and decision/tui-boundary-assertions-only: the boundary driver (the live-round rig) gained a resize action, a KR-only settle and the KR arm; the pin and the boundary-only scope stand.
@task_item n=8 part=touches
decision/a11y-runs-in-existing-test-suites and decision/a11y-inside-existing-suites: the five KR rehearsals were added inside the existing integration suite, no separate driver or suite.
@task_item n=9 part=touches
decision/zero-flakiness: the KR rehearsal that ran 12.4 s against its 15 s deadline now runs about 5.1 s under a 100 ms quiet window; the watched rehearsal red of the box read empty after x and DEL stayed green in 4 runs, a hypothesis that it shares the mended settle named on its carry, its rule unchanged.
@task_item n=10 part=touches
decision/wrapped-tui-reads-as-unwrapped: the measurement was of claude unwrapped, giving the unwrapped baseline a wrapped run is read against for a key after a resize.
@task_fact n=1 slug=keyboard-test-harness
fact: A key typed 0, 20 or 100 ms after a resize of claude's own pty was measured kept on Linux (claude 2.1.330 and 2.1.328, 12 of 12 rounds each, the CLI unwrapped); no viola-side wait or fix follows, stated as no loss at these delays on these versions, never as never loses, and not a measurement of viola's poll lag or the driver's paste.
@task_item n=1 part=untouched
CLI output discipline: control-character escaping, SGR and colour rules, the list board, captions, NAME truncation and word-beside-SGR rows; no CLI output changed.
@task_item n=2 part=untouched
CLI refusal, exit-code, --json failure, fault and hint contracts, including revive, link, plugin install, send, wait and verify refusals; no refusal, code word or exit code of viola moved.
@task_item n=3 part=untouched
Frame-verb, hook-call and MCP frame tool refusals and the instance-unreachable details, including call-deadline; no socket, strict-modes or deadline rule changed.
@task_item n=4 part=untouched
Time bounds on the human and the driver, dialog hold and config.json keys; no time bound, config key or env var was added or changed.
@task_item n=5 part=untouched
Screen-reader targets and the manual Orca and keyboard passes, CI integration, gate thresholds and SLO rows, and the absences (touch, web or graphical frontend, i18n, rendered page, user time limit); none is reached.
