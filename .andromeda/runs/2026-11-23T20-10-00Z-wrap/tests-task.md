@task_prose part=session
The session widened the Hyprland seam's closed request set from 6 to 8: a monitors query and one move-to-workspace dispatch whose argv is ONE Lua element holding only a checked window address and an integer workspace.
A new closed monitor reply type keeps ids, sizes, positions, focus and active workspace, and no text. The parse routes monitors to its own parser and the move to the dispatch parser.
The compositor read deadline was settled at its unchanged 5 s and timed by a test. A drain witness now shows a reply twice the frame cap keeps one byte past the cap and is oversize. That witness replaces the bounded-readers table's earlier witness for the hyprctl reply row.
The test-only tap gained two recording verbs, record-monitors and record-move, so it now has 4. The recorded Hyprland 0.56.2 fixture set grew from 7 to 9 files, and the fixture schema's request enum grew from 6 to 8 words. Both new replies were recorded live, each by one firing: the move on the operator's own word.
The fuzz target over hyprctl JSON now also runs the monitors parse, with a new corpus entry. No product caller, IPC method, crate, dependency, CI step or harness surface was added, and no capability is claimed.
@task_file n=1 path=crates/viola-hyprland/src/lib.rs
what: edited — the Hyprland seam crate: its closed request set (now 8 words), each request's argv and the dispatch flag
@task_file n=2 path=crates/viola-hyprland/src/reply.rs
what: edited — the hyprctl reply parsers: the closed reply types, a new monitors parser and the per-request routing, with their unit and property tests
@task_file n=3 path=crates/viola-hyprland/src/read.rs
what: edited — the bounded, deadline-timed hyprctl read: its deadline constant settled, a timed-deadline test and a drain-cap witness added
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: edited — the test-only Hyprland recording tap: its verbs, argument parse, string keep-list and the move recording against its own terminal window
@task_file n=5 path=crates/viola-e2e/tests/hypr_tap.rs
what: edited — the tap's rehearsal tests over planted stub hyprctl and stub terminal executables
@task_file n=6 path=tests/contract_bounded_readers.rs
what: edited — the bounded-readers contract table: each external reader's cap, needle and witness test
@task_file n=7 path=tests/contract_hyprland_seam.rs
what: edited — the Hyprland seam contract: the fake replaying every recorded reply byte for byte and the read parsing each into its type
@task_file n=8 path=tests/contract_fixture_hygiene.rs
what: edited — the fixture hygiene walk and its planted violations (unknown request word, monitors string, non-success move reply)
@task_file n=9 path=schemas/hyprland-fixture.v1.json
what: edited — the recorded Hyprland fixture schema: its closed request enum
@task_file n=10 path=fuzz/fuzz_targets/hyprctl_json.rs
what: edited — the hyprctl JSON fuzz target over the reply parsers
@task_file n=11 path=fuzz/corpus/hyprctl_json/monitors-row
what: added — a committed fuzz corpus entry for the hyprctl JSON target
@task_file n=12 path=fixtures/hyprland/0.56.2/Monitors.recorded-1.json
what: added — a recorded, scrubbed Hyprland 0.56.2 fixture: the monitors reply
@task_file n=13 path=fixtures/hyprland/0.56.2/MoveToWorkspace.recorded-1.json
what: added — a recorded Hyprland 0.56.2 fixture: the move dispatch argv and its success text
@task_item n=1 part=touches
Fixture hygiene walk and planted hygiene violations: two new planted cases (a monitors string, a non-success move reply), and the planted unknown request word moved from monitors to exec because the widened enum now admits monitors.
@task_item n=2 part=touches
Recorder scrub and synthetic fixture text rows: the tap's keep-list keeps no monitors string and keeps the move reply only as its success text, and every other string becomes the placeholder.
@task_item n=3 part=touches
Fixtures checked against their schema and against the tolerant schema: the Hyprland fixture schema's request enum grew from 6 to 8 words.
@task_item n=4 part=touches
Bounded readers rows (every external reader bounded, bounded readers keep the frame cap): the hyprctl reply row's witness is now the drain test where a reply twice the cap keeps one byte past it, and its types text names Monitor too.
@task_item n=5 part=touches
Parser surfaces under property and fuzz tests and the fuzz corpus rows: the new monitors parser runs under the property strategy (its key list went from 16 to 18) and the hyprctl JSON fuzz target, and a reproducer corpus entry was committed.
@task_item n=6 part=touches
Time-faked-through-seams, no test-owned real time and no sleep synchronisation: a unit test waits out the real 5 s compositor read deadline (about 5.01 s). The tap's record-move also polls the compositor with a bounded loop.
@task_item n=7 part=touches
Closed enums change only by decision: the seam's closed request set and the schema's request enum each widened by two words on the operator's and overseer's decisions.
@task_item n=8 part=touches
Recorded fixtures replayed and fake matches recordings byte for byte, and fixture drift compared byte for byte: the replay set now holds 9 Hyprland files, including the first recorded dispatch.
@task_item n=9 part=touches
Test-only E2E crate and single test-only crate rows: the test-only tap binary in that crate gained two verbs and a terminal it spawns and ends.
@task_item n=10 part=touches
External processes are real stub executables: the tap's rehearsals plant stub hyprctl and stub terminal scripts that answer the move.
@task_item n=11 part=touches
No human-in-loop verification, live verify never in CI, and live readings recorded as readings: the two recordings were fired live once on a host by the operator pass and are never re-fired by a test. CI only replays them.
@task_item n=12 part=touches
Mocks only at product seams: the Hyprland seam is the product seam its fake replays. It still has no product caller.
@task_item n=13 part=touches
No unbounded JSON depth: the monitors parse reads at serde's default depth after a MAX_FRAME length check.
@task_fact n=1 slug=test-data-fixtures
fact: A recorded dispatch that moves a window acts only on a terminal the recording tap itself started, ends that terminal and confirms its window is gone. It never places a host window it did not open.
@task_fact n=2 slug=7-test-data-fixtures
fact: A Hyprland monitors recording keeps numbers and booleans alone and no string, and a recorded dispatch reply is kept only as its success text. The hygiene walk plants each one and rejects it.
@task_fact n=3 slug=7-test-data-fixtures
fact: A planted unknown-word hygiene case names a word outside the closed set. When the set widens, every planted case that names a new word moves to a word still outside it.
@task_item n=1 part=untouched
CLI verb contracts (exit codes, send, wait, last, release, answer, revive and run refusals): no viola verb changed, and no verb places a window.
@task_item n=2 part=untouched
MCP tools, the MCP server and wrapper channel JSON-RPC contracts: no IPC method, tool or endpoint was added.
@task_item n=3 part=untouched
Test harness contract (agent-run commands, exit codes, status, boot, cleanup, session record, pre-push and gate output): no harness command or status shape changed.
@task_item n=4 part=untouched
CI integration rows (workflows, Actions pinning, artifacts, supply-chain, MSRV, caches): no CI step or workflow changed.
@task_item n=5 part=untouched
Coverage floors, mutation thresholds and performance budgets: no threshold, floor or budget moved.
@task_item n=6 part=untouched
Log format and log event corr contracts: no log event or field was added.
@task_item n=7 part=untouched
Fake agent command line, receipts, screens and recorded claude CLI fixtures: neither the fake agent nor the claude fixtures changed.
@task_item n=8 part=untouched
Heartbeat staleness, snapshot replay, torn append and instance liveness rows: no on-disk product state changed.
@task_item n=9 part=untouched
Dependency and supply-chain bans (tokio, serde_json unbounded depth, rmcp features): no dependency was added or bumped.
