@task_prose part=session
An API-error turn end now names its cause from a closed list of nine words: rate-limited, overloaded, auth-failed, billing, invalid-request, model-not-found, server-error, output-limit, api-error; the turn-end cause set grew from one member to nine, additively, with no event-line v bump.
The adapter maps the 13 CLI StopFailure error words onto those nine; an absent error reads api-error, any other string reads api-error and is marked unknown; the CLI's error text is still read only to classify and never crosses into the event log.
A marked StopFailure writes exactly one codes-only parse-rejected line (parser hook-stdin, detail unknown-value, count 1) at WARN and the event is still delivered; the diagnostics schema's parse-rejected detail enum grows from eight to nine values.
The adapter-internal API-error class list and its classifier function were retired, replaced by one classifier returning the cause word; the wrapper's closed-set re-check of the cause follows the grown set unchanged.
Tests added: a 15-row unit map table, a 15-row wrapper re-check table, a booted hook case per CLI error word, an MCP wait case carrying the word unchanged, a scoped-capture unit case for the unknown-value line, and api-error turn-end rows in the send rung and list STATUS cases.
Not measured live: the 13 error words are read from the installed CLI bundles' schema, no recorded StopFailure payload exists; CI green 7/7, no change after implement.
@task_file n=1 path=crates/viola-core/src/lib.rs
what: the shared core types — the closed turn-end cause set with its spelled words and name lookup
@task_file n=2 path=crates/viola-agent-claude/src/hook.rs
what: the claude adapter's hook payload normaliser and hook-to-kind map — the StopFailure arm that builds turn-ended data and its unit map table
@task_file n=3 path=src/cmd/hook.rs
what: the viola hook verb's entry — reads hook stdin, writes parse-rejected lines for rejected input, delivers the event to the wrapper; holds its scoped-capture unit cases
@task_file n=4 path=schemas/diag-line.v1.json
what: the diagnostics line schema — the closed parse-rejected detail enum the schema conformance check validates role files against
@task_file n=5 path=src/cmd/run.rs
what: the wrapper's hook-event revalidation before an event is appended to the event log (unit cases only; product code unchanged)
@task_file n=6 path=src/run/send/slot.rs
what: the send turn-running rung — a turn end clears the running turn and lets the next send past (a new api-error turn-end case row)
@task_file n=7 path=crates/viola-state/tests/state_events.rs
what: the event log integration suite — turn-ended event data round trip through the on-disk log
@task_file n=8 path=tests/hook_events.rs
what: the hook E2E suite — viola hook processes under a booted wrapper fed StopFailure payloads, asserting the turn-ended cause, the role-file parse-rejected line and that no error text lands under the home
@task_file n=9 path=tests/cli_wheel.rs
what: the CLI wheel and send suite — an api-error turn end clears the turn with no pause or release
@task_file n=10 path=tests/cli_list.rs
what: the viola list suite — STATUS reads idle after an api-error turn end
@task_file n=11 path=crates/viola-mcp/tests/handlers.rs
what: the MCP handler suite — the MCP wait tool carries a turn-end cause word unchanged in structuredContent, isError false
@task_item n=1 part=touches
contract/event-data/turn-ended (Event data keys · turn-ended): cause is now one of nine closed words, not api-error alone; the row must name the nine-word set and its booted case per CLI error word.
@task_item n=2 part=touches
contract/viola-wait/turn-ended and contract/viola-wait/wake: the turn-ended wake now carries one of nine cause words through wait --json; any cause value these rows state moves from the one word to the nine.
@task_item n=3 part=touches
contract/mcp-wait-tool and contract/mcp-tool-result/ok rows over the MCP wait result: the cause word crosses unchanged into structuredContent with isError false; decision/cli-mcp-identical-payloads holds over it.
@task_item n=4 part=touches
contract/turn-ended/background-tasks and contract/dialog-data/null-details: the StopFailure arm still writes last_assistant_message and background_tasks as null literals; only cause changed, so a row stating the StopFailure data shape must be checked for its cause value.
@task_item n=5 part=touches
contract/log-event-corr/parse-rejected and the log-format rows over parse-rejected: a new emitter, the hook-stdin parser with detail unknown-value, writes one codes-only line per outside StopFailure error string.
@task_item n=6 part=touches
decision/diagnostics-schema-conformance and decision/diagnostics-schema-validation: the parse-rejected detail enum the role files are validated against grew from eight to nine values (unknown-value); the booted case validates every hook role line against it.
@task_item n=7 part=touches
decision/unknown-events-counted-skipped and decision/hook-unknown-fields-tolerated: an unknown StopFailure error value is now counted by a parse-rejected line and the event still delivered as api-error, an application of the unknown-plus-counter rule to a value rather than a field or kind.
@task_item n=8 part=touches
decision/codes-only-role-logs, decision/logs-carry-no-secrets, decision/secrets-never-in-logs and decision/error-sanitization: the outside error value and the CLI's error text appear in no role, detail or event line; the booted case walks the home for the planted text.
@task_item n=9 part=touches
decision/closed-enums-need-decisions-log: two closed enums moved — the turn-end cause set 1 to 9 and the parse-rejected detail enum 8 to 9 — on the founder's ruling for the cause words and the overseer's technical answer for unknown-value.
@task_item n=10 part=touches
decision/hooks-fail-open and decision/hook-fails-open: a marked StopFailure still delivers its event after the WARN line; the hook verb's path gained a branch that must stay fail-open.
@task_item n=11 part=touches
contract/viola-send/turn-running and decision/send-refusal-order: every cause word clears the running turn; a new api-error turn-end row joins the rung's case table (true as stated, add-to only if the row names the cause).
@task_item n=12 part=touches
contract/list-human/columns and contract/viola-list-json/live-instance STATUS: STATUS reads idle after an api-error turn end, with no human-output change; check only where a row names the turn-end cause.
@task_item n=13 part=touches
decision/scoped-capture-pinned-dispatch: the unknown-value unit case reads the hook's lines through the module's scoped capture, not a role file; the role-file read and schema validation stand in the booted case (an implement departure for the reconcile to judge).
@task_item n=14 part=touches
decision/recorded-hook-payloads-walked-at-run-time, decision/fixture-named-by-hook-event and decision/fixtures-per-cli-version: no recorded StopFailure payload exists; the 13 error words come from the CLI bundles' schema, so the map is held by literal tables and booted fake-agent payloads, not a recorded fixture.
@task_item n=15 part=touches
decision/fake-agent-not-proof-of-real-cli and decision/real-claude-local-only: the nine-word map against real StopFailure payloads is unmeasured; the live reading stays carried on the 2.1.325 recorded fixture set.
@task_item n=16 part=touches
decision/rstest-case-tables, decision/contract-tables-are-literal and decision/literal-test-oracles: the 15-row map table and the 15-row wrapper re-check table are literal case tables; the map table carries the unknown mark as a third column.
@task_item n=17 part=touches
decision/hook-exec-against-live-wrapper, decision/hook-e2e-recorded-stdin and decision/real-hook-in-e2e: the booted case runs real viola hook processes under a booted wrapper for each of the 13 words, the absent case and an outside string.
@task_fact n=1 slug=integration-test-strategy
fact: The turn-ended cause is one of nine closed words mapped by the adapter from the 13 CLI StopFailure error words; an absent error reads api-error, an outside string reads api-error and writes one codes-only parse-rejected hook-stdin unknown-value line, and the wrapper refuses to append any cause outside the nine.
@task_fact n=2 slug=test-data-fixtures
fact: No recorded StopFailure payload exists in the fixture set; the 13 CLI error words are taken from the installed CLI bundles' schema, so the nine-word map is held by literal tables and booted fake-agent payloads and stays unmeasured against a live run.
@task_item n=1 part=untouched
The instance-unreachable cause rows (instance-unreachable-cause, unreachable-log-cause, exit-21-detail, mcp-unreachable-detail): their closed cause words share the token cause but are a different set; nothing in them moved.
@task_item n=2 part=untouched
Dialog rows (hook-decision bodies, dialog events, answer mapping, dialog hold): StopFailure is no dialog hook and no decision body changed.
@task_item n=3 part=untouched
Other parse-rejected parsers and details (control-character, malformed, oversize, unknown-keys and the rest), the MCP stdin line cap and the wrapper channel frame cap: unchanged.
@task_item n=4 part=untouched
CI, coverage, mutation, perf, fuzz and supply-chain gates and the harness contract rows (boot, cleanup, status, gate, pre-push): no harness or gate surface moved, no dependency or tool version changed.
@task_item n=5 part=untouched
Channel and IPC rows (JSON-RPC errors, link and unlink, socket directory, server identity, peer admission): no new method, endpoint, socket, env var or event kind.
@task_item n=6 part=untouched
Wrapper start, revive, verify stamping and PTY passthrough rows: no boot-path change; the smoke was skipped for that reason.
@task_item n=7 part=untouched
Human-mode output rows (CLI human lines, SGR, control escaping, list board layout): the cause word reaches only --json and MCP, no human output changed.
