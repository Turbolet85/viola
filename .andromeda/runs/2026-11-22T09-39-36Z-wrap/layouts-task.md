@task_prose part=session
The session measured whether real claude on Linux loses a key typed right after a resize of its own pty, and changed only the test-only live rig crate.
Reading: claude 2.1.330 (KR) and 2.1.328 (KR8-2) each kept 12 of 12 keys typed 0, 20 or 100 ms after a resize; no loss in 12 rounds at these delays on these versions, never a claim that it never loses.
No product change: the fix half was not needed, viola's own CLI, send path, pty layer and fake agent are untouched.
The rig gained the KR live arm (labels KR, KR-2, KR8, KR8-2), a KR-only settle that counts quiet from a byte after the key, a late-redraw versus dirty-box word, and five KR rehearsals over the fake agent.
The rig's rehearse verb gained two test-only options, --quiet-ms and --kr-del-seam, accepted with the kr arm alone and refused as usage with exit 2 elsewhere, including on a live start.
Nothing was retired; no config key, env var, product flag or verb of viola moved.
@task_file n=1 path=crates/viola-e2e/src/rig/mod.rs
what: the test-only live rig's arms, session settle and summary keys, edited to add the KR arm, its after-key settle and its closed-word tally
@task_file n=2 path=crates/viola-e2e/src/rig/keys.rs
what: the test-only live rig's key grants, edited to add the resize grant word and its checks
@task_file n=3 path=crates/viola-e2e/src/rig/screen.rs
what: the test-only live rig's screen model of cell attributes, edited to read at a new width after a resize
@task_file n=4 path=crates/viola-e2e/src/bin/viola-live-rig.rs
what: the test-only live rig's command line, its rehearse and live verbs, edited to add two rehearse-only options
@task_file n=5 path=crates/viola-e2e/tests/rig_rehearsal.rs
what: the rig's rehearsals over the fake agent, edited to add five KR rehearsals
@task_item n=1 part=touches
clap derive for CLI parsing: the test-only rig's rehearse verb takes two new clap options (--quiet-ms 100-400, --kr-del-seam withheld:n or late:n); a row ruling on clap parsing is reached only if it rules beyond viola's own verbs, whose parsing did not change.
@task_item n=2 part=touches
The shared refusal word and refusal line form: the rig refuses a bad option or wrong arm as refused: usage with exit 2; that is the test rig's own word and code, not a viola refusal line or a CLI exit code, and no such row moved.
@task_item n=3 part=touches
Indirect, send and input readiness after a resize: the measurement found no key lost after a resize, so viola's send path, its input-not-ready hint and its send outcomes stand unchanged; a row on send readiness gains a reading beside it, not an edit.
@task_item n=4 part=touches
Human output and --json as a contract: the KR summary keys (control, rounds, kept, lost, unread, kept-0ms, kept-20ms, kept-100ms, lost-at, kr-reading, sizes, dirty-box, late-redraw, key-refusal) are the test rig's ledger words of closed words and counts, not a viola output document.
@task_item n=1 part=untouched
CLI token rendering, SGR styling, colour depth and the BAY line: no rendering, colour or glyph changed.
@task_item n=2 part=untouched
The viola list board, its columns, truncation and --json document: untouched.
@task_item n=3 part=untouched
viola run and viola revive starts, their refusals and --json error codes: untouched.
@task_item n=4 part=untouched
viola send, wait, last and answer output and the readback mirror and vocabulary: no word, line or layout changed.
@task_item n=5 part=untouched
The MCP tools and their results, the hidden mcp verb, and the frame verb exit-21 hints and details: untouched.
@task_item n=6 part=untouched
viola verify output and refusals, plugin install, link and unlink output, wheel verbs, help and usage lines: untouched.
@task_item n=7 part=untouched
The web surface absence and the deep-link absence: no surface was added.
