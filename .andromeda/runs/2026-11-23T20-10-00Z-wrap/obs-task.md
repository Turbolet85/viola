@task_prose part=session
The Hyprland seam's closed request set grew from 6 to 8 requests: a monitors query and one move-to-workspace dispatch, a single Lua dispatcher element holding only a checked window address and an integer workspace.
The monitors reply is parsed into a closed Monitor type of numbers and booleans only, with no text field; a reply past MAX_FRAME is refused as oversize and a non-array as malformed, through the standing hyprctl-json parser word.
The hyprctl read deadline was settled at its unchanged 5 s (no longer provisional), timed by a test, and the drain past the cap gained a witness test that keeps one byte past MAX_FRAME and reads oversize.
The test-only fixture recorder gained two verbs (record-monitors, record-move), 2 to 4 verbs; its string keep-list keeps no monitors string and the move reply only as its success text; two recorded fixtures were added (set 7 to 9 files).
The move was measured live once on Hyprland 0.56.2 and recorded; the seam still has no product caller, no new log event, subject, parser word, detail word, IPC method, env var or crate was added, and no capability is claimed.
@task_file n=1 path=crates/viola-hyprland/src/lib.rs
what: edited — the Hyprland seam crate: its closed request set, each request's word and hyprctl argv, the dispatch marker
@task_file n=2 path=crates/viola-hyprland/src/reply.rs
what: edited — the hyprctl reply parsers (the hyprctl-json parser): closed typed readings of compositor JSON replies, text dropped at the parse
@task_file n=3 path=crates/viola-hyprland/src/read.rs
what: edited — the bounded hyprctl reader: the hyprctl spawn, its read deadline, the MAX_FRAME cap and the drain past it
@task_file n=4 path=crates/viola-e2e/src/bin/viola-hypr-tap.rs
what: edited — the test-only Hyprland fixture recorder (tap) that records hyprctl replies from the live compositor, with its string keep-list and closed stderr failure codes
@task_file n=5 path=crates/viola-e2e/tests/hypr_tap.rs
what: edited — the fixture recorder's rehearsal tests over planted stub hyprctl and terminal programs
@task_file n=6 path=tests/contract_bounded_readers.rs
what: edited — the bounded-readers contract: the Boundary table of every reader with its cap and its witness test (the hyprctl reply row)
@task_file n=7 path=tests/contract_hyprland_seam.rs
what: edited — the Hyprland seam contract: recorded replies replayed through the fake and parsed into their types
@task_file n=8 path=tests/contract_fixture_hygiene.rs
what: edited — the fixture hygiene walk: recorded fixtures checked against the schema and for kept host strings
@task_file n=9 path=schemas/hyprland-fixture.v1.json
what: edited — the recorded Hyprland fixture schema with its closed request-word enum
@task_file n=10 path=fuzz/fuzz_targets/hyprctl_json.rs
what: edited — the fuzz target of the hyprctl-json parser
@task_file n=11 path=fuzz/corpus/hyprctl_json/monitors-row
what: added — a seed in the hyprctl-json fuzz corpus (a monitors reply row)
@task_file n=12 path=fixtures/hyprland/0.56.2/Monitors.recorded-1.json
what: added — a recorded Hyprland fixture: the compositor's monitors reply, numbers and booleans kept, every string the placeholder
@task_file n=13 path=fixtures/hyprland/0.56.2/MoveToWorkspace.recorded-1.json
what: added — a recorded Hyprland fixture: the move-to-workspace dispatch argv and its success reply
@task_item n=1 part=touches
The hyprctl-probe diagnostics subject: it now also covers the monitors query and the move dispatch spawns; no new subject word was added.
@task_item n=2 part=touches
The parse-rejected line with parser hyprctl-json: the monitors parser now emits through it, with the standing malformed and oversize detail words; no new parser or detail word.
@task_item n=3 part=touches
Per-parser parse-failure counts and parse failures surfacing as unknown: the hyprctl-json parser gained a monitors reading, and an element without an id or a key of another type is stepped over or read as absent.
@task_item n=4 part=touches
Recorded fixtures are no telemetry source: two new recorded Hyprland fixtures were added; the monitors fixture keeps numbers (ids, resolution, positions, scale) and booleans, every string replaced.
@task_item n=5 part=touches
Fuzz artifacts uploaded only with synthetic seeds: a new seed was added to the hyprctl-json fuzz corpus and the fuzz target gained a monitors bound.
@task_item n=6 part=touches
Components left uninstrumented and OS operations observed at viola's call sites: the Hyprland seam gained two requests but still has no product caller, so no product process emits a line for them yet.
@task_item n=7 part=touches
Crate-seam errors end in a logged line and typed errors with fixed messages: the seam's error set is unchanged (5 words) though two requests now reach it.
@task_item n=8 part=touches
The MAX_FRAME bound on readers: the hyprctl reply's oversize witness was renamed to the drain test that keeps one byte past the cap; any row naming the old witness or the cap of the hyprctl reply is touched.
@task_item n=9 part=touches
A read time bound: the hyprctl read deadline is settled at 5 s, no longer provisional; any row holding that deadline or its provisional state is touched.
@task_item n=10 part=touches
Content and host strings kept out of telemetry and fixtures (agent and compositor rows never verbatim, default-deny fields): the monitors reply's text is dropped at the parse and the recorder keeps no monitors string.
@task_item n=11 part=touches
Closed protocol codes and fixed failure words: the test-only recorder gained closed stderr codes (no-terminal-window, move-refused, move-unseen, window-left) with no new product word.
@task_fact n=1 slug=log-coverage
fact: Every request of the Hyprland seam's closed set, queries and dispatches alike, is observed by the one hyprctl-probe spawn pair and the one hyprctl-json parser word; adding a request adds no subject, parser or detail word.
@task_fact n=2 slug=pii-scrubbing-compliance
fact: A compositor monitors reply is read as numbers and booleans only; no monitor text (name, description, make, model, serial) reaches any type, so no line or record can carry one.
@task_item n=1 part=untouched
Panic capture, panic lines and per-role panic outcomes: no panic path changed.
@task_item n=2 part=untouched
The channel, send, dialog, wait, wheel and hook paths and their log lines, spans and corr rules.
@task_item n=3 part=untouched
The MCP server, its stdout, its tools and its refusal codes.
@task_item n=4 part=untouched
Log file locations, modes, the logging stack, levels, service identity and diagnostics init order.
@task_item n=5 part=untouched
Metrics derivation, SLO budgets, perf gates and the hook latency budgets.
@task_item n=6 part=untouched
CI gates, secret scan, artifact upload and retention: no CI step or harness command changed.
@task_item n=7 part=untouched
The OTel, exporter, error reporter, network egress and database absences: none added or removed.
@task_item n=8 part=untouched
Exit codes, detail codes of start, revive, verify and the CLI frame verbs, and the events log.
