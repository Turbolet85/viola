# test-plan · the rows that bind 2026-11-16-wait-wake-condition — picked 229 of 1328 (134 by every draw) · rule any · pin 1668

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/cli-mcp-identical-payloads · CLI and MCP return identical payloads — MCP and CLI return identical ok and refusal payloads for the same channel result.
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout.
decision/fake-agent-and-verify-fixtures-are-contract · Fake agent and verify fixtures are the contract — The fake agent and the viola verify fixtures are the contract the tests hold the agent integration to. · why: A binding founder direction.
decision/fake-agent-matches-verify-fixtures · Fake agent held to recorded verify fixtures — A contract suite compares the fake agent's hook stdin byte for byte, never as snapshots, with the recorded viola verify fixtures across the spine, dialog and framing tiers; live viola verify refreshes those fixtures locally. · why: guards against the fake agent drifting from the real CLI
decision/human-output-escapes-control · Human-mode output escapes control characters — Human-mode list, wait and last output escapes C0 and C1 characters.
decision/ipc-tests-schema-asserted · IPC surfaces tested by schema-asserting clients — The wrapper channel and the MCP stdio server are driven by language-native clients with JSON schema assertion, and channel results are cross-checked against the events.ndjson cursor.
decision/mutation-at-epoch-boundary · Mutation testing at the epoch boundary — cargo-mutants runs at the epoch-boundary code audit and on demand through run --mutants, never in the default run or --all; there is no per-chunk, pre-push or CI mutation gate, and a missed or timed-out mutant fails that run.
decision/mutation-testing · Mutation testing — Mutation testing runs per chunk and at the epoch boundary through /andromeda-code-audit. · why: A mandated driver of the Comprehensive tier.
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify and a route entry's own live measurement, never in CI.
decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, stamps come from viola verify against the fake agent, and no state is developer-seeded.
decision/unknown-events-counted-skipped · Unknown events counted, not failed — Unknown event kinds and fields are counted in skipped rather than failing.
decision/waits-key-on-events · Test waits key on logged events — Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
contract/cli-human-output/no-sgr · CLI human output · SGR — Human-mode output carries no SGR sequences under a non-TTY, NO_COLOR or TERM=dumb.
contract/cli-json-output/stdout-document · CLI --json output · stdout document — Under --json a verb prints one JSON document on stdout.
contract/dialog/event-logged-once · hook dialogs · dialog event — A question, permission or plan event is logged exactly once with a dialog_id, and wait returns it at once.
contract/viola-verify/stamped-line · viola verify · stamped line — viola verify at a stamped version exits 0 and ends with the line "stamped <ver>  <n> pass  0 fail".
contract/wait/already-logged · wait · after cursor — wait returns at once when a waking event is already logged at or after the after cursor.
contract/wait/instance-unreachable · wait · unreachable wrapper — A wrapper killed mid-wait gives CLI exit 21 and MCP instance-unreachable.
contract/wait/timeout · wait · timeout — wait times out with {timed_out:true}.
contract/wait/wake-kinds · wait · wake kinds — wait wakes only on turn-ended, question, permission, plan, session-end or dialog-expired; activity and wheel lines do not wake it.
decision/chaos-tests-required · Chaos tests required — Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction.
decision/error-sanitization · Errors sanitized at every surface — CLI --json, MCP isError and channel error.data contain no absolute paths, serde paths or values, error chains or tool input.
decision/replay-every-cli-version-fixture · Every CLI-version fixture set replayed — Every recorded fixture set of every CLI version is replayed, and stamped and unstamped versions are both gated.
decision/tests-run-on-linux-ubuntu-latest · Tests run on Linux — The test suites run on Linux, with CI on the ubuntu-latest leg.
contract/event-line/format · the event line · format — Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
contract/event-line/source · the event line · source — source is one of hook or wrapper.
contract/mcp-iserror/payload · MCP isError payloads · payload — An isError payload carries codes only.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/cli-json-one-document · One JSON document per verb — Under --json a CLI verb signals by its exit code plus one JSON document on stdout. · guard: NOT_ERROR_BODIES · except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document.
decision/fake-agent-contract-suite · Fake agent held to recorded fixtures — A contract suite asserts the fake agent's hook stdin bytes match the recorded viola verify fixtures byte for byte, for the spine, dialog, framing, resume and compact variants, including forwarded annotations. · why: guards against the fake agent drifting from the real CLI · guard: contract_fake_agent_drift
decision/mutation-testing-on-demand · Mutation testing on demand — Mutation testing scoped to the chunk diff runs only under run --mutants, never in the default run or --all, at the epoch-boundary code audit, and a surviving mutant is red.
decision/per-test-fresh-home · Self-bootstrapping per-test home — Each test bootstraps itself in a fresh temp home passed as --home, with the fake agent as the child, and no developer-seeded state.
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
contract/cli-wait/endpoint-lost · CLI wait · endpoint-lost — An endpoint vanishing during wait gives instance-unreachable with detail endpoint-lost.
contract/instance-unreachable-cause/endpoint-lost · instance-unreachable causes · endpoint-lost — A closed cause word of instance-unreachable's detail.
contract/instance-unreachable-cause/instance-dead · instance-unreachable causes · instance-dead — A closed cause word of instance-unreachable's detail.
contract/mcp-tool-result/ok · MCP tool result · ok — isError false with structuredContent equal to the ok payload.
contract/run-start-events/order · run start events · order — The events log reads wheel with cause start, then the agent's session-start with source hook.
decision/events-never-truncated · Events never truncated — Instance events are never truncated; their retention in tests is the lifetime of the temp home.
decision/frame-verbs-check-instance-first · Frame verbs check the instance first — All eight CLI frame verbs run the instance strict-modes check on the named instance, the driven one for link and unlink, before any role file or log opens; a refusal exits 21 and writes no line.
decision/mcp-frame-tools-check-instance-first · MCP frame tools check the instance first — The MCP frame tools send, wait, last and answer run the instance strict-modes check before the liveness pre-check, a refused instance getting no frame and no line. · guard: handlers_frame_tools_refuse_a_widened_instance_before_any_frame
decision/mcp-negatives-violate-schema · MCP negatives violate the advertised schema — MCP tests send negative inputs that violate the advertised tool schemas. · why: The schemas are advisory.
decision/no-sgr-without-tty · No SGR without a terminal — CLI output carries no SGR under a non-TTY, NO_COLOR or TERM=dumb.
contract/cli-frame-verbs/server-verify-failed · CLI frame verbs · server-verify-failed — A frame verb whose server is not the recorded wrapper sends no frame and exits 21, with detail server-verify-failed under --json.
contract/cli-frame-verbs/strict-modes-failed · CLI frame verbs · strict-modes-failed — A CLI frame verb refuses a widened instance before any role file or log opens: exit 21 with the instance pair, or under --json exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}, with no frame and no line written.
contract/cli-human-lines/unable · CLI human lines · unable — unable  <name>  <reason>  <detail>, followed by a last-line hint:.
contract/mcp-invalid-params-field/after · MCP invalid-params fields · after — A closed field word of invalid-params' detail.
contract/mcp-invalid-params-field/timeout-ms · MCP invalid-params fields · timeout_ms — A closed field word of invalid-params' detail.
contract/mcp-server/internal-fault · MCP server · internal handler fault — Any other channel error, a reply of no known shape, a wait or last refusal or a panicked call is the JSON-RPC error -32603 internal error with data null.
contract/mcp-tool-result/instance-unreachable · MCP tool result · instance-unreachable — isError true with {"error":"instance-unreachable","detail":"<cause>"}, the CLI exit-21 document less v, text refused: <code>.
contract/mcp-tool-result/invalid-params · MCP tool result · invalid-params — isError true with {"error":"invalid-params","detail":"<field>"}, no frame written.
contract/mcp-tool-result/wrapper-fault · MCP tool result · wrapper-fault — isError true with {"error":"wrapper-fault","detail":{code,message,data}}, the detail being the channel fault screen's.
contract/wrapper-channel/success · Wrapper channel · success — {"result":{"ok":…}}

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/complete-lines-reader · Live-appended files read by complete lines — Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader. · why: a Linux reader can observe an append part-way
decision/e2e-for-critical-paths · E2E for every critical path — Every critical path and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
decision/mutation-run-failure-conditions · Mutation run failure conditions — A mutation run fails on a missed mutant not left out as host-excluded, on a timed-out mutant, or when unviable mutants outnumber caught ones.
decision/no-test-retries-or-sleeps · No test retries or sleep synchronisation — nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.
decision/stamps-from-fake-agent-verify · Stamps only from verify against the fake agent — Test stamps come only from viola verify run against the fake agent.
decision/tests-create-own-home · Every test creates its own home — Every test creates its own temporary --home, with no developer-seeded data.
decision/fixture-named-by-hook-event · Fixtures named by hook event — Fixtures are named <Event>.<variant>.json, where <Event> is the CLI's own PascalCase hook event name, and the fake agent looks fixtures up by that name with no event-to-file mapping table on either side.
decision/integration-suite-per-contract · Integration suite for every standard contract — Every architecture standard contract (wrapper channel, on-disk, hooks, CLI, MCP) has a crate-level integration suite over a real interprocess endpoint, a real tempfile home and the built binary run as a process.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/bounded-remove-the-guard-witness · Bounded remove-the-guard witness — A control whose neutralised guard lets a product process start needs a witness bounded by a time limit. · why: nextest's default profile has no kill line, and an unbounded witness hung
decision/cross-surface-trigger-layer · Cross-surface trigger layer — Cross-surface coordination is tested at E2E, with cross-surface parity checks.
decision/discipline-at-every-layer · Discipline held at every layer — No screen parsing, no blanket approval and no flaky tests hold at every test layer.
decision/e2e-over-fake-agent · E2E over the fake agent — Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
decision/mcp-test-layer · MCP test layer — MCP is tested at E2E through an rmcp client.
decision/mutation-run-verdict · Mutation run verdict — A mutation run fails on any missed outcome not left out as host-excluded, on any timeout, or when unviable outcomes exceed caught ones. · except: a mutant replacing a client seam's peer-admitted read with an always-admit survives by construction and stands as the version's known limit
decision/cli-env-test-layers · CLI and env test layers — CLI and environment input is tested at unit and integration.
decision/dialog-bodies-pinned-by-insta · Dialog decision bodies pinned by insta — insta in check mode pins the dialog decision bodies, while hook sequences are compared byte for byte with recorded fixtures, never as snapshots.
decision/hook-stdin-test-layers · Hook stdin test layer — Hook stdin is tested at integration, through the hook as a subprocess.
decision/multi-version-trigger-layers · Multi-version compatibility layers — Multi-version compatibility is tested by unit tests and the contract suite over per-version fixtures.
decision/sandbox-contract-trigger-layer · Sandbox contract trigger layer — The contract-test-against-sandbox trigger is met by the contract suite over recorded fixtures.
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/recorded-hook-payloads-walked-at-run-time · Recorded hook payloads walked by one test — One contract test walks every recorded hook payload fixture at run time, and an empty walk fails.
contract/mcp-tool-arguments/invalid-argument · MCP tool arguments · invalid argument — A non-ViolaName target, a non-u64 dialog_id, an open-string behavior or control characters in text give isError:true with a code-only payload.
decision/proptest-parser-invariants · Proptest for parser invariants — Parser invariants are held by proptest property blocks run at 512 cases.
decision/rstest-fixtures-for-constructors · rstest fixtures for shared constructors — Shared unit-test constructors, such as a temporary home or a clock, are rstest fixtures.
contract/reply-screen/detail · the reply screen · detail — A string detail that is no code word reads null.
contract/reply-screen/peer-fault · the reply screen · peer fault — A peer fault reads through sanitised_fault.
contract/reply-screen/refusal · the reply screen · refusal — A refusal that is no code word reads unknown.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/defensive-log-reader · Event log reader counts and never panics — The event log reader never panics: torn lines are counted and not returned, lines of unknown kind are counted and not returned, and known-kind lines with unknown keys are counted once and returned. · guard: three_counts
decision/hook-unknown-fields-tolerated · Unknown hook payload fields tolerated — Unknown fields in a hook payload are skipped, never fatal; only a known field of the wrong type is recorded as a drift entry.
decision/human-output-control-escaping · Human output escapes control characters — Human-mode wait and last output escape C0 and C1 control characters, keeping newline and tab.
contract/channel-wait/timed-out · the channel wait method · timed_out — wait returns {timed_out:true} only once the clock passes timeout_ms, never before; the timeout runs in the wrapper process.
contract/channel-wait/wake-kinds · the channel wait method · wake kinds — wait wakes only on turn-ended, question, permission, plan, session-end and dialog-expired, and returns at once when a matching record is at or after `after`.
contract/dialog-data/null-details · dialog and Stop event data · description, preview and background_tasks — A missing or non-string question description or preview is null, never a refusal; background_tasks carries only the array's length, or null when absent or not an array.
contract/cli-arguments/json-flag · CLI arguments · --json — The --json flag is read from the arguments only before a `--` separator.
contract/mcp-reply/screened-words · MCP tool replies · refusal and detail words — A refusal that is no code word reads unknown, a string detail that is no code word reads null, and a peer fault reads through the sanitised fault.
contract/unreachable-log-cause/endpoint-lost · the unreachable log cause · endpoint-lost — Logged during call.
contract/unreachable-log-cause/instance-dead · the unreachable log cause · instance-dead — Logged during connect.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, and as hyprctl in a test-scoped bin dir.
decision/mcp-handlers-use-tokio-client · MCP handlers use the Tokio client — MCP handlers use viola-channel's Tokio client, never the sync client, and are tested at adapter level through the transport-free Tools handle against a test wrapper.
contract/cli-output/no-escape-without-tty · CLI output · escape sequences — Human layouts carry no ESC [ sequence under non-TTY, NO_COLOR or TERM=dumb.
contract/event-line/round-trip · the event line · event kind — Every event kind round-trips the event line.
decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans.
decision/hook-exec-against-live-wrapper · Hook exec tested against a live wrapper — viola hook is fed fixture stdin against a live test wrapper, and its event line and hook trace are asserted.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/tolerant-event-reader · Tolerant event log reader — The event reader counts lines outside the contract while still returning known kinds, counts and does not return a kind no build knows, and an append after a log cut short starts on a fresh line.
contract/cli-json/one-document-per-verb · CLI --json · one document per run — Each driver-facing verb prints one --json document and its literal exit code for every exit code but 2; a usage error or an unreadable input exits 2 with text on stderr and no document.
contract/event-data/turn-ended · Event data keys · turn-ended — data holds the keys last_assistant_message, background_tasks and cause; cause is present on an API-error turn end only and absent on a Stop.
contract/mcp-frame-tools/refusal-result · MCP frame tools · refusal result — A refused send, wait, last or answer returns isError with {"error":"instance-unreachable","detail":<cause>} and the text refused: instance-unreachable.
contract/mcp-unreachable-detail/endpoint-lost · MCP frame tools instance-unreachable detail · endpoint-lost — A closed connection or an I/O failure, a wrapper killed under a parked wait included, gives detail endpoint-lost.
contract/mcp-unreachable-detail/instance-dead · MCP frame tools instance-unreachable detail · instance-dead — A failed connect gives detail instance-dead, with no frame and no log line.
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home.
decision/instance-check-before-role-file · Instance check before any role file or frame — viola link, unlink, send, wait, last, answer, pause and release run the instance check on the driven instance before any role file or log opens or any frame is sent, leaving the mode as found. · why: Opening a role file narrows an existing home to 0700, so a check placed after it would pass a 0770 home. · guard: link_verbs_refuse_a_widened_instance_before_any_log
decision/ledger-probe-contract-suite · Ledger probe contract suite — For each stamped fixture set, viola verify against the fake agent lists every relied ledger row id exactly once and ends with zero fails; stamped and drift-only sets are test literals, every fixture dir on exactly one list. · guard: contract_ledger_probes
decision/no-escape-in-plain-output · No escape bytes in plain output — No ESC [ sequence is written when output is not a TTY, under NO_COLOR or under TERM=dumb, and clap's own help and usage errors into a pipe carry no ESC byte even under CLICOLOR_FORCE. · guard: cli_output_plain
contract/cli-json-error/instance-unreachable · CLI --json error document · instance-unreachable strict-modes-failed — An instance refusal under --json prints exactly {"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"} on stdout, stderr empty, exit 21.
contract/event-data/question · Event data keys · question — option_details is a key of each entry in the questions array of a question line's data; the data keys themselves are dialog_id and questions.
contract/event-line/keys · Event line · keys — Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/fake-agent-matches-recordings · Fake agent matches recordings byte for byte — For each recorded fixture set, walked at run time, the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; an empty walk fails.
decision/forward-compat-lines-skipped · Forward-compatible lines skipped — Log lines of a newer form are counted in skipped rather than failing the read.
decision/live-readings-not-a-test-layer · Live readings are not a test layer — A route entry's live readings of a real session are that chunk's own local measurement, kept in its evidence with its own launcher, driver and readers: not a test layer, not harness surface, never in CI.
decision/mcp-stdio-test-owns-child · MCP stdio test owns the child — MCP stdio tests run an rmcp client over a viola mcp child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's TokioChildProcess, beside a replay of the recorded MCP exchange.
decision/single-writer-state-files · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only verify writes stamps.json; every other writer-capable entry point leaves both byte-identical.
contract/dialog-event/logged-once · the event log · dialog event — Each dialog event is logged exactly once with a dialog_id, and wait returns it at once.
contract/viola-wait/timed-out · viola wait · timed_out — A wait past the log end times out with {timed_out:true}.
contract/viola-wait/wake · viola wait · wake on turn-ended — wait stays parked through activity and wheel records and wakes on turn-ended with its cursor; a re-run from that cursor returns the same event at once.
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/hint-names-its-cause · Each exit cause has its own hint — Each cause of exit 21 and of exit 1 has a negative test asserting the exit code, the --json document and a last hint line naming its cause; no two causes share a hint. · guard: no_two_codes_share_a_hint
decision/no-release-in-driver-hints · No release in driver-facing hints — Driver-facing hints, refusal hints included, never suggest or name viola release. · guard: no_hint_names_release
decision/tty-asserted-only-on-verb-sgr · TTY rendering asserted only on verb SGR — CLI verb tests pin --json documents and human layouts by literals, and assert TTY rendering only on the SGR sequences in verb output under the outer PTY, never on the child agent's screen.
contract/mcp-cli-parity/exit-21 · MCP and CLI parity · exit 21 causes — For each cause of exit 21 the MCP isError body {error, detail} equals the CLI --json document less v, its text refused: instance-unreachable.
contract/mcp-tools/invalid-params · MCP tools · invalid-params — A schema-violating call is answered invalid-params naming its field, and no frame is written.
contract/viola-wait/endpoint-lost · viola wait · endpoint-lost — An endpoint vanishing during wait exits 21 with endpoint-lost.
contract/wait-and-last/killed-wrapper · viola wait and last · killed wrapper — A killed wrapper gives CLI exit 21 and MCP instance-unreachable.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/channel-e2e-driver · Channel tested with the sync client — The wrapper channel is tested with the viola-channel sync client, a raw interprocess stream and jsonschema validation.
decision/channel-endpoint-from-snapshot · Channel endpoint read from the snapshot — Channel tests read the wrapper endpoint only from the instance snapshot.
decision/cli-e2e-via-process · CLI tested as a process — CLI verbs are tested end to end by running the viola binary as a child process.
decision/fixture-drift-byte-compare · Fixture drift compared byte for byte — The fake agent's hook sequence and payloads must equal each recorded fixture set byte for byte, never through an insta snapshot, and an empty fixture walk fails. · guard: contract_fake_agent_drift
decision/forward-compatible-log-reading · Forward-compatible event log reading — Lines of an unknown kind and unknown fields on a known kind are counted and skipped; list --json still exits 0 and the known-kind line still counts toward item state. · guard: chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads
decision/single-writers · Single writers of snapshot and stamps — Only the wrapper writes snapshot.json and only viola verify writes ledger/stamps.json. · guard: only_the_wrapper_writes_the_snapshot_and_only_verify_the_stamps
contract/dialog-events/logged-once · dialog events · question, permission, plan — Each dialog event is logged exactly once with a dialog_id, and a parked wait returns it at once.
contract/list-json/skipped-counts · list --json · skipped — skipped.torn_lines, skipped.unknown_kinds and skipped.unknown_fields each count the log lines skipped for that reason.
contract/mcp-tools/result-body · MCP tools · result and refusal body — Each MCP tool's result and refusal equal the CLI's --json document less v.
contract/mcp-wait/timeout-ms · MCP wait · timeout_ms — The bound sent is clamped to 50 000 ms, which is also its default.
contract/turn-ended/background-tasks · turn-ended event data · background_tasks — The Stop array's length, or null.
contract/viola-wait/already-past · viola wait · event already past the cursor — A wait after a cursor whose event is already logged returns that event at once, exit 0, no timed_out.
contract/viola-wait/non-waking-records · viola wait · activity and wheel records — Activity and wheel records never wake a parked wait.
contract/viola-wait/timeout · viola wait · timeout — A wait that times out gives {timed_out:true} with exit 0.
contract/viola-wait/turn-ended · viola wait · turn-ended wake — Exits 0 with event.kind "turn-ended" and a cursor at or past that record's offset.
decision/cli-fixed-strings-pinned-by-literal · CLI fixed strings pinned by literals — Each fixed CLI string, the human layouts included, is pinned by a literal in its test file.
decision/contract-tables-are-literal · Contract tables are literal — Contract tests hold literal tables, never the product's own list.
decision/each-exit-cause-own-hint · Each exit cause has its own hint — Every exit cause ends human output with a hint line naming that cause, no two causes share a hint, and a fault carries none. · guard: no_two_codes_share_a_hint, shared_hints_reads_a_planted_duplicate
decision/hints-never-suggest-release · Driver hints never suggest release — No driver-facing hint, MCP refusal text, structured content, result or tool description names release. · why: Drivers are refused release, so such a hint sends the driver into a dead end.
decision/hook-e2e-recorded-stdin · Hook verbs fed recorded stdin — The hook verbs are tested by running them as processes fed fixture stdin recorded per agent CLI version.
decision/mcp-tests-own-server-child · MCP tests own their server child — MCP tests spawn and own a viola mcp child, closing its stdin and awaiting its exit, never using rmcp's child-process transport. · why: That transport kills the server 3 s after a close, leaving a truncated coverage profile.
contract/cli-exit-codes/21 · CLI exit codes · 21 — Instance unreachable: {"v":1,"error":"instance-unreachable","detail":<cause>}, the cause endpoint-lost, instance-dead or unwrapped-name.
contract/cli-json-failure/document · CLI --json failures · failure document — Each CLI --json failure gives one document, a literal exit code, empty stderr, no ESC byte, and a body holding no path or upstream text.
contract/cli-json/instance-unreachable · CLI --json document · instance-unreachable — Exit 21 prints {"v":1,"error":"instance-unreachable","detail":"<cause>"}; MCP's text is "refused: instance-unreachable".
contract/exit-21-detail/endpoint-lost · exit 21 instance-unreachable detail · endpoint-lost — The wrapper was lost with the call open; its own hint; during call.
contract/mcp-tools/instance-unreachable · MCP tools · instance-unreachable — is_error:true with the CLI document less v, text refused: instance-unreachable.
contract/verify-output/last-line · viola verify output · last line — A passing verify exits 0 with a last line matching ^stamped \S+  \d+ pass  0 fail$.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/control-file-gating · control-file gating of fake agent steps — Each gated step of a fake agent turn script waits for the next line the test appends to the agent's control file, never for a sleep. · why: Deterministic ordering without sleeps.
decision/per-test-home · isolated home per test — Each test has its own home, so no endpoint or log is shared between tests.
decision/product-state-from-real-runs · Product state from real runs only — On-disk product state is produced only by real viola runs against the fake agent in a fresh home, never hand-written. · except: chaos tests corrupt files the product already wrote; server-identity impostors build their own snapshot.json; unit tests and viola-state's own tests seed state files for the code under test.
decision/stamps-written-only-by-verify · Stamps written only by verify — ledger/stamps.json is produced by viola verify against the fake agent; no test or harness code writes it. · except: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check.
contract/fake-agent-cli/argv-only-options · viola-fake-agent command line · modes and options — The fake agent's modes and options are taken from argv only, never from the environment; standing as hyprctl, its mode is read from a file beside its exe.
contract/fake-agent-cli/control · viola-fake-agent command line · --control — The control path is <home>/fake/<name>.control (<name>.revived.control for a revive start), and the fake agent reads it by byte offset.
contract/fake-agent-cli/script · viola-fake-agent command line · --script — The fake agent follows a JSON turn script for PreToolUse, PermissionRequest, PostToolUse and Stop steps; a relative --script is resolved against the workspace root, the same way as --fixtures.
contract/fake-agent-hooks/stdin-payload · viola-fake-agent hook execution · hook stdin payload — The payload on a hook's stdin is the recorded fixture's bytes, save the fields a prompt, a resume or a compact replay sets.
contract/fake-script/format · fake agent turn script · script format — A script is {"v":1,"steps":[{"event","variant","gate"}]} over the ten registered hook events, validated by the fake-script v1 schema.
contract/fake-script/gated-step · fake agent turn script · gated step — A gated step waits for one more newline-terminated control line past the byte offset.
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, or as hyprctl by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/hook-payloads-recorded-locally · Hook payloads recorded locally, replayed in CI — Hook payload fixtures are recorded locally by live viola verify, scrubbed, checked in and replayed in CI.
decision/human-output-as-literals · Human-mode output as literals — Expected human-mode output, such as help text and the list board, is held as literals in its test files.
decision/test-homes-made-private · Test-made homes are private directories — A home a test makes before viola runs is made through the product's private-directory creation, never by plain recursive creation. · why: A plain home is one the product refuses by the read rule, its 0755 mode failing the rule.
decision/tests-seed-own-data · self-bootstrapping tests — Tests seed their own data; no developer pre-populates a home.
contract/fake-script/unreadable-exit · fake agent turn script · unreadable script — An unreadable script makes the fake agent exit 2.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/fake-agent-argv-only · Fake agent configured by argv alone — The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments. · except: Standing as hyprctl by its own file name, the fake agent reads its mode and reply set from files beside its exe.
decision/fake-agent-replays-recorded-bytes · Fake agent replays recorded bytes — The fake agent offers each hook the recorded fixture bytes and builds no payload, save the fields it sets: the typed prompt of a default UserPromptSubmit, the launch SessionStart's source and session_id under --resume whichever variant fires, and the compact replay's session_id. · why: The rewritten resume payload was ratified.
decision/fake-transcripts-compared-to-recordings · Fake-agent transcripts compared byte for byte — Fake-agent hook transcripts have no snapshot of their own; the bytes offered on a hook's stdin are compared byte for byte against the recorded hook payloads.
decision/product-state-never-hand-written · On-disk product state produced by viola — On-disk product state in tests is produced only by running real viola verbs against the fake agent in a fresh home, never hand-written. · except: The chaos tests, which corrupt files the product already wrote; the server-identity impostor cases; and the MCP handler suite's own snapshots.
decision/stamps-only-from-verify · Stamps written only by verify — The stamps ledger of a test home is produced only by running viola verify against the fake agent; no test or harness code writes it. · why: A non-verify process writing the stamps ledger is rejected by the security plan; ratified as the resolution of that conflict.
contract/claude-fixtures/payload-path · Recorded Claude fixtures · payload path — A recorded hook payload is checked in as fixtures/claude/<cli-version>/<Event>.<variant>.json, and the fake agent reads it from that path.
contract/mcp-wait-tool/timeout-bound · MCP wait tool · wait bound — The wait bound is the given timeout capped at 50 000 ms, min(given, 50 000).
decision/fixtures-walked-for-hygiene · Fixture hygiene walk — Every committed fake script, Claude fixture and Hyprland fixture is walked against its schema and for absolute paths, usernames and emails, with planted cases proving each class red. · guard: contract_fixture_hygiene
decision/test-clocks-behind-clock-seam · Test clocks behind the clock seam — Time in the sync crates is faked through the core Clock seam with hand-written test clocks; no mocking crate stands behind it.
decision/test-endpoint-dirs-made-private · Test endpoint directories are private — A directory holding a test's endpoint is made through the private-directory helper, never left a bare temp dir. · why: A bare temp dir takes its mode from the umask, 0755 by default, and the socket-directory check refuses it.
decision/wrapper-boots-pass-trusted-screens · Test wrapper boots run trusted with screens — Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd. · except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.
contract/fake-agent-receipt/hook · Fake-agent receipt · hook — Carries event, command_absolute and ran; when it ran also exit_code, stderr_len, stdout_hex and stdin_hex, the exact bytes offered on stdin; spawn_os_error, the raw OS error as an integer or null, when spawning an absolute command failed.

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, and the real hyprctl by the same binary as hyprctl; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/time-faked-through-seams · Time faked through injected clocks — Time is faked only through the injected viola_core Clock, with no mock_instant; the MCP adapter runs no tokio timer, so no paused test clock, and a pure decision takes its instant as an argument.
decision/wrapper-deadline-injected-clock · Wrapper deadlines driven by the injected clock — The wait timeout deadline runs in the wrapper process and only the injected Clock drives it in tests; a paused tokio clock in a test never reaches a spawned wrapper.
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.

## ci-integration · ## 9. CI Integration · twin 9-ci-integration
decision/ci-stamped-home-kill-line · 20 s kill line for stamped-home tests — On the ci nextest profile the stamped-home test binaries are killed after 20 seconds.
decision/mutation-testing-off-ci · Mutation testing outside CI — Mutation testing has no push or pull-request job and no gate; it runs at the epoch boundary through the code audit and on demand.

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
decision/mutation-gate-cadence · Mutation gate at the epoch boundary only — Mutation testing is judged at the epoch boundary by the code audit and on demand by run --mutants, with no chunk, pre-push or CI mutation gate.
decision/no-ignore-parking · No #[ignore] parking — `#[ignore]` is not allowed as a parking place for a failing or flaky test. · why: Agent-driven development cannot tell a flake from a real bug.
decision/send-and-wait-latency-not-timed · Send confirmation and wait wake latency not timed — The `send` to `prompt-submitted` confirmation latency and the `wait` wake latency are deliberately kept out of the timed quality gates and are asserted as behaviour, not timed. · why: Both are bounded by product constants that are still open.
decision/zero-flakiness · Zero-flakiness budget — Flaky tests are not tolerated: nextest `retries` stays 0, no retry-once policy exists, and a test that flakes once is quarantined at once and fixed at the root cause, the chunk staying red until it is. · why: Agent-driven development cannot tell a flake from a real bug, and retry policies mask actual failures.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/mutation-gate-at-epoch-boundary · Mutation gate at the epoch boundary — Mutation strength is judged at the epoch boundary by the code audit and on demand by run --mutants; no chunk, pre-push or CI mutation gate exists.
decision/mutation-harness-tests-ungated · Real cargo-mutants harness tests ungated — The two real-cargo-mutants harness tests run ungated, never ignored, and the CI profile's 120 s kill and zero retries stay unchanged.
decision/fake-agent-own-process-group · Fake agent spawns in its own process group — Every command the fake agent spawns runs in a process group of its own. · why: A killed wrapper's PTY hangup would reach hooks cut off in their exit-time profile write, leaving profiles llvm-profdata refuses.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/fake-agent-not-proof-of-real-cli · Fake-agent run is not proof of real-CLI behaviour — A green fake-agent run never counts as proof of real-CLI behaviour; a behaviour change requires a local `viola verify` fixture refresh plus the contract suite. · why: The fake agent must not drift from the real CLI.
decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
decision/no-in-process-clock-across-processes · No in-process clock for another process — An in-process mock clock never controls another process's time; cross-process expiry is asserted in a unit case. · why: A global mock clock reaches only the test process's threads, never a spawned viola run.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-shared-mutable-state · No shared mutable state across parallel tests — Parallel tests never share mutable state.
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/real-hook-in-e2e · Real viola hook in E2E — `viola hook` is never stubbed in E2E; the fake agent invokes the real pinned binary through the plugin files `run` wrote.
decision/state-files-never-hand-written · State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify` and `snapshot.json` from the wrapper, never writing them by hand. · except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.
decision/harness-output-json-and-exit-code · Harness commands always give JSON and exit code — Every harness command prints its JSON document and exit code, never a human-only message alone.
decision/no-blanket-approval-tests · No blanket-approval tests — No test auto-approves a dialog.
decision/no-skipping-quality-gates · Quality gates never skipped — Quality gates are never skipped, not even once.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/closed-enums-need-decisions-log · Closed enums change only by decision — A new value of a closed harness enum or of the log event enum needs a Decisions Log entry.
decision/fixture-chain-exists-once · Fixture chain exists once — The home, fake_agent_path, stamped_home and booted_wrapper chain exists once, as the sync root copy in tests/support; viola-e2e has no copy and boots its sessions through its own harness_session. · why: root tests/support cannot be imported by another package, and the root package must not dev-depend on the tokio-based viola-e2e
decision/root-wait-bound · Root waits bounded below the kills — Root waits on a child are bounded at 12 s, below the mutants profile's 15 s kill and cargo-mutants' 20 s timeout floor, and fail at once when the wrapper exits first. · why: a stuck wait fails the test itself, and a mutant that stops the wrapper getting ready grades as caught, not Timeout
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
contract/gate-breach/suite-skipped · gate breach · suite-skipped — A required suite has skipped > 0.
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
decision/e2e-homes-under-target-e2e-home · E2E homes under target/e2e-home — Every booted session home is a fresh unique directory under the workspace's target/e2e-home, which viola itself creates owner-only. · why: CI's zero-panic gate, schema gate, secret scan and diagnostics upload all read that tree
decision/killed-wrapper-endpoint-wait · A killed wrapper's endpoint is gone when unconnectable — After a stop or a kill the endpoint is gone when a connect is refused or the path is absent, never by waiting for the file to vanish; an exit code is not the endpoint gone. · why: a killed wrapper removes nothing, so its socket file stays
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/mutants-tested-by-owning-package · Mutants tested by their own package — Each mutant is tested only by the tests of the package that owns the mutated file, so a crate's mutants must be killed by its own tests.
contract/agent-run-status-polled-fields/skipped · agent-run status polled fields · skipped — An envelope field the agent polls, holding unknown_kinds, unknown_fields and torn_lines.
contract/harness-run-exit/0 · harness run exit codes · 0 — Every selected suite passes, with no timed-out mutants, no missed mutants beyond the host-excluded, and unviable not outnumbering caught.
contract/harness-session/session-id · harness session · session id — The session id is the test name, the label (or main) and the pid; each session has its own home and session dir.
contract/run-reason/live-in-ci · run reason · live-in-ci — Exit 2.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/ci-profile-no-retries · CI test profile without retries — The CI nextest profile runs with retries = 0, fail-fast off, a JUnit report, and a default per-test kill at 120 s (30 s period, four periods).
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 15 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/verify-driven-tests-20s-kill · Per-test kill bounds verify-driven tests — A test that drives viola verify has no test-side bound on that call; its binary sits on the ci override whose 20 s per-test kill is its bound. · why: Ratified; sized from the measured verify floor plus a 3x runner tail.
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 12 s is the bound of every test wait not bounded by a per-test kill.

## log-format · key of ## 3. Test Harness Contract
contract/log-streams/event-stream · log streams · event stream — Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/fake-agent-replay-for-stamps · Fake-agent replay for stamps — Tests replay recorded Claude CLI fixtures per CLI version through the fake agent, and run viola verify against it for stamps.
decision/rstest-fixture-chain · rstest fixture chain — Test homes are built by the rstest 0.27.0 fixture chain home, fake_agent_path, stamped_home, booted_wrapper.

# test-plan · its absences, names and overviews, whole

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
absence/browser-suite — A browser test suite
absence/windows-and-macos-support — Windows and macOS support: code, tests, fixtures and CI jobs

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
absence/database — A database
absence/outbound-http — Outbound HTTP

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
Scenario: E4 — none in 0.1.0 › N/A — viola has no web or graphical frontend.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
absence/deploy-stage — a deploy stage
