@task_prose part=session
The session is test-only: it pinned the Hyprland seam's two existing MAX_FRAME size bounds at the exact frame, with no product line changed.
It added one unit test of the hyprctl read: a reply of exactly MAX_FRAME bytes is taken and logs process-start then process-exit with no parse-rejected line, and one byte more is refused as Oversize with a parse-rejected line whose detail is oversize.
It added one unit test of the hyprctl dispatch-reply parser: a reply of exactly MAX_FRAME bytes passes the size check and reads Refused, and one byte more reads Oversize.
Both tests kill mutants of the two bounds that the boundary mutation audit had left alive as survivors. Remove-the-guard controls show each test fails when its bound is removed.
No caller, log event, detail word, field, dependency, schema, gate or threshold changed. The crate's own test count went from 82 to 84, and no master states that count.
@task_file n=1 path=crates/viola-hyprland/src/read.rs
what: the Hyprland seam's hyprctl read, which spawns the hyprctl probe, drains its reply up to the MAX_FRAME cap, refuses a larger reply as oversize, and writes the hyprctl-probe process-start, process-exit and parse-rejected diagnostic lines
@task_file n=2 path=crates/viola-hyprland/src/reply.rs
what: the Hyprland seam's hyprctl reply parser (parser hyprctl-json), which reads the clients and dispatch replies and refuses a reply past MAX_FRAME as oversize before parsing
@task_item n=1 part=touches
The parse-rejected diagnostic line for parser hyprctl-json: a test now pins that a hyprctl reply one byte past MAX_FRAME writes parse-rejected with detail oversize, and that a reply of exactly MAX_FRAME bytes writes none. The row's words stand, and a test now witnesses them.
@task_item n=2 part=touches
The parse-rejected detail word oversize: a unit test of the Hyprland read now emits and asserts it at the exact-frame boundary. The word itself is unchanged.
@task_item n=3 part=touches
The diag-line subject hyprctl-probe and the child-and-probe lifecycle lines: the new read test asserts that a reply at the cap logs exactly process-start then process-exit for the probe.
@task_item n=4 part=touches
Indirect, through the hyprctl parser: the rules that parse failures surface as unknown or a refusal with a per-parser parse-failure count. The dispatch parser's size refusal (Oversize past the cap, a normal Refused at the cap) is now pinned, not changed.
@task_item n=5 part=touches
Indirect, through the mutation audit: the rules that obs and boundary code is mutation-tested and that a surviving mutant is a failure. The two MAX_FRAME bound sites that had survived are now killed by tests in the mutated crate's own package.
@task_item n=6 part=touches
Indirect, through the probe spawn: the rules that OS operations and child spawns are instrumented at viola's call sites by subject, duration and exit. The hyprctl probe's call-site lines are exercised by a planted stub, with no instrumentation change.
@task_item n=7 part=touches
Indirect: the convention that a bounded reader refuses a frame past MAX_FRAME and takes one of exactly MAX_FRAME bytes. The Hyprland seam now follows the same exact-frame test pattern as the viola mcp stdin line within MAX_FRAME.
@task_item n=1 part=untouched
The viola mcp stdin rows (a line within MAX_FRAME, an oversize line before initialize, a line past MAX_FRAME, parser mcp-stdin oversize): that reader and its tests did not change. It served only as the precedent.
@task_item n=2 part=untouched
The statusline shell-out rows (frame cap, deadline, duration and exit, command never logged, decision details): no statusline code or bound changed.
@task_item n=3 part=untouched
Diagnostic line schema, field names, corr rules, event enum and log levels: no field, event, detail word or level was added, renamed or removed.
@task_item n=4 part=untouched
CI integration, perf budgets, hook budgets, the zero-panics, schema-conformance and secret-scan gates, and artifact retention: no gate, threshold or workflow changed.
@task_item n=5 part=untouched
Error capture and panic rows, PII scrubbing and redaction, span and trace coverage, metric coverage, and the absence rows: nothing in those areas moved, and no dependency or exporter was added.
@task_item n=6 part=untouched
Channel, send, dialog, wait, wheel, run-start, verify, revive and hook contract rows: none of those components was reached.
