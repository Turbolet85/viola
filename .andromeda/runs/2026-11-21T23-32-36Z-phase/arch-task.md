@task_prose part=chunk
Answers whether real claude on Linux loses a key typed right after a window resize: it reads where that claim comes from, what viola does around a resize today, and what the installed 2.1.328 and 2.1.330 claude bundles do on a resize, as strings only.
It defines what an honest measurement is and prices the cheapest live start that tells it, plus the fix options (a settle wait after a resize, a read-back and one re-type, a refusal), each with the rows it moves, in ONE founder block at P3's end; this phase makes no live start.
A fix lands only AFTER the reading, on the resize path, on what the pump and the driver type after a resize, and in the fake agent's resize and input behaviour; a measuring arm in the live rig and a relied ledger row come only if the answer needs them.
It re-reads the directive's stated facts (installed CLI versions, 29 relied ledger rows, six fixture version folders, MCP_CALL_DEADLINE and the rig's MB arm) and the carries: the fake agent's DEL already erasing, the CHILD_WITHIN comment, and the paste's missing time bound.
@task_file n=1 path=crates/viola-pty/src/lib.rs mark=edits
what: the PTY seam, portable-pty behind viola's own pty seam, where the window-size change is sent to the child's PTY — edits: the resize, and the test-module comment over CHILD_WITHIN naming the mutants profile's kill bound (reworded only if this chunk edits viola-pty)
@task_file n=2 path=crates/viola-pty/src/pump.rs mark=edits
what: the PTY pump and its paste handle, the writes to the child's PTY — edits: the pump's writes after a resize; PasteHandle::paste and its one blocking write, only if this entry's change opens that write path and the P3 block takes it as a fork
@task_file n=3 path=src/run/ mark=edits
what: the run pump and the wheel in the root bin, the path that carries a resize to the child and what is typed after it — edits: the resize path and the keys typed after it
@task_file n=4 path=src/bin/viola-fake-agent/input.rs mark=edits
what: the fake agent's input behaviour, the test double standing for claude — edits: plain() and its DEL handling (read as met), the fake agent's resize and input behaviour if a fix or a rehearsal needs it
@task_file n=5 path=crates/viola-e2e mark=edits
what: the standing live rig, the test-only end-to-end crate that drives a real claude locally — edits: a measuring arm, only if one is added
@task_file n=6 path=crates/viola-agent-claude mark=edits
what: the per-behaviour CLI capability ledger and its relied rows — edits: a new relied ledger row, only if the answer becomes one
@task_item n=1 part=needs
The resize path: who sends the window-size change to the child's PTY, what is written to the PTY after it and when, and every wait or settle bound on that path, read as it stands and changed only if the measurement shows a lost key.
@task_item n=2 part=needs
A human key is never held behind a resize: any fix (a settle wait, a read-back and re-type) must leave the human's passthrough keys unheld and must not make passthrough wait on the screen model.
@task_item n=3 part=needs
What viola types after a resize: the atomic bracketed-paste send, the keys the driver writes and their waits, send's readiness gate and its after-the-fact confirmation, the send time bound and the paste-hint wait; a fix may re-type or refuse only inside those rules, never typing into a dialog.
@task_item n=4 part=needs
The wheel and its byte-source claim: a key typed right after a resize may be the human's or the driver's, and the human always wins the wheel; a fix must not move or hold the wheel.
@task_item n=5 part=needs
The closed word that tells a lost key from a kept one: a compiled screen signature, never a text, and never printed in chat; run never reads screen content, and screen row text is read only by verify.
@task_item n=6 part=needs
The claude CLI's resize behaviour as the installed 2.1.328 and 2.1.330 bundles' strings read (a redraw, an input-mode reset, a debounce): a reading recorded as such, never as a measurement, and never a CLI behaviour relied on without a ledger row.
@task_item n=7 part=needs
The live measurement is priced, not run: real claude probes run only locally and never in CI, under a test-owned home and config, never the operator's real claude home; process, PTY sizes before and after, key bytes and delays, the CLI state typed into (the empty input box, never a dialog), repetitions and what a mixed reading means.
@task_item n=8 part=needs
If the answer becomes a relied row: a version-stamped capability ledger row with its readings, the stamps written only by verify, fixtures recorded only from a clean verify and scrubbed, and a CLI version verified only when every relied row counts.
@task_item n=9 part=needs
The PTY seam depends on no viola crate and knows no agent: a fix in it names no claude behaviour; claude-specific shapes stay in the agent crate.
@task_item n=10 part=needs
The fake agent's resize and input behaviour, reached through its test seam variables (the pump delay seam among them), changed only so it can stand for the measured claude behaviour; its DEL erasing the prompt's last character is read as already met.
@task_item n=11 part=needs
The live rig's arms: a measuring arm is added only if needed, stays test-only, rides the rig's harness knob, and its rehearsal's intermittent red is read at its next red (printed summary and kept frame) before any wait is mended, with no retry rule and no loosened entry.
@task_item n=12 part=needs
The paste write's missing time bound: one blocking write in the paste handle, bounded only if this entry's change opens that write path, and then as a fork of the P3 block; otherwise left whole for its own route entry.
@task_item n=13 part=needs
Every time bound and who waits: a waiting test takes a bound under the 20 s lint, root test waits share one bound, and the CHILD_WITHIN comment names the mutants profile's 15 s kill if the seam's file is edited.
@task_item n=14 part=needs
No mutation run in this chunk, and the pinned product (the release build at its pin) is untouched; the product re-pin step stays outside the chunk.
@task_item n=15 part=needs
The directive's stated facts are re-read and closed at P3: claude 2.1.330 as the bare install and 2.1.328 by path, 29 relied ledger rows, six fixture version folders with one stamped, MCP_CALL_DEADLINE and the rig's MB arm present.
@task_standing n=1
convention: each new test runs in its own viola home, test homes created and removed by their owner process only
home: established-decisions
@task_standing n=2
convention: each new test runs against the fake agent; no live claude in CI, real claude only locally
home: ci-cd-approach
@task_standing n=3
convention: a new OS-gated test names its reason, and no test gate names another OS
home: cross-cutting-patterns
@task_standing n=4
convention: each new log line goes through the one event emitter, codes only in process logs, no print macros
home: build-system
@task_standing n=5
convention: test evidence survives a killed test and a wait report is kept on failure
home: occupied-resources
@task_standing n=6
convention: end-to-end runs carry zero panic lines; insta snapshots run in check mode only
home: project-directory-structure
@task_item n=1 part=not_needed
The MCP server and its frame tools, call deadline, stdin cap and tool results: the chunk changes no MCP surface; the deadline is only re-read as a stated fact.
@task_item n=2 part=not_needed
Hooks, the dialog hold and dialog answers (hook tiers, hook exit codes, hook-to-kind map, permission and plan answers): the chunk types into no dialog and changes no hook.
@task_item n=3 part=not_needed
Revive, the board and list (revive refusals and JSON codes, list payloads, agents listing, session liveness): no resize path runs through them.
@task_item n=4 part=not_needed
Links, link recording and the channel's link, unlink, from and last methods: the chunk adds no channel method and changes no link.
@task_item n=5 part=not_needed
Hyprland window features and the compositor seam errors: the resize measured is the PTY window size, not a compositor window.
@task_item n=6 part=not_needed
Release tooling, dist, crates.io publishing, licences and the cargo-deny dependency policy: the chunk adds no dependency and ships no release.
@task_item n=7 part=not_needed
The trust writer's code words, plugin install and the plugin's compiled contents: no trust record or plugin file is touched.
@task_item n=8 part=not_needed
config.json keys and configuration precedence: the chunk adds no configuration key; any settle bound is compiled.
@task_item n=9 part=not_needed
The fuzz workspace, its nightly toolchain and weekly fuzzing: no fuzz target is added.
@task_item n=10 part=not_needed
Usage-budget and model-API rows: nothing here reads a rate limit or calls a model API.
