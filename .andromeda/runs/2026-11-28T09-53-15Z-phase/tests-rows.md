# test-plan · the rows that bind 2026-11-28-fleet-console-view — picked 210 of 1391 (129 by every draw) · rule any · pin 5303

## test-plan-viola · (above the first heading)
decision/agent-driven-test-layers · Agent-driven test layers — Every test layer runs end-to-end by an agent reading exit codes and structured output, with no human-in-the-loop checkpoint, no manual verification and no visual regression with human review. · except: A surface that cannot be agent-driven is flagged untestable and handled through boundary tests and mocks.

## 1-test-scope-summary · ## 1. Test Scope Summary · twin test-scope-summary
decision/agents-json-fallback-unknown · Unreadable agents listing reads unknown — Oversize or malformed claude agents --json output is read as unknown.
decision/chaos-tests-required · Chaos tests required — Chaos tests cover a torn last ndjson line healed by the next append, a corrupt or unsupported snapshot recovered by replay, a child exit without EOF, an endpoint vanishing during wait, and the stale-heartbeat versus live-pid distinction.
decision/cli-mcp-identical-payloads · CLI and MCP return identical payloads — MCP and CLI return identical ok and refusal payloads for the same channel result. · except: call-deadline, written by the MCP frame tools alone, has no CLI document.
decision/cli-tests-as-subprocess · CLI tested as a subprocess — The viola CLI is tested as a subprocess with a per-test --home and --json, its signal being the exit code plus the JSON document on stdout. · except: viola console takes no --json; it is tested under its own outer PTY, its signal its exit code and its diagnostics lines.
decision/human-output-escapes-control · Human-mode output escapes control characters — Human-mode list, wait and last output escapes C0 and C1 characters.
decision/linux-only · Linux only — The product is Linux only: the core on any current distribution, the full workspace and fake-agent E2E on ubuntu-latest, the window features on Hyprland; a test gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/logs-carry-no-secrets · Logs carry no secrets — No log contains a stripped CLAUDE* value, and the fixed canary appears in no home-level diagnostics line. · guard: viola-harness secret-scan
decision/no-child-screen-assertions · No assertions on the child's screen — TTY behaviour is tested through an outer-PTY driver, and assertions never parse the rendered child agent's screen for content. · except: The forced vt100 panic case byte-searches the outer terminal; the test-only live rig reads the child's screen into closed words, by compiled literals and the box row's cell attributes, never text.
decision/no-screen-scraping-in-e2e · E2E asserts on events, never on screen content — E2E assertions use hook events, events.ndjson and channel payloads, never scraped child-screen content.
decision/no-secrets-in-logs · No secrets in logs or diagnostics — No log line and nothing in diagnostics/ holds a stripped CLAUDE* value, and diagnostics/ is mode 0700 with its files 0600. · guard: the secret-scan test
decision/real-claude-local-only · Real claude CLI tested local-only — Claude-facing behaviour is tested against the fake agent and recorded fixtures; the real claude CLI and Anthropic backend are mocked at unit level and reached only locally, through viola verify, the standing live rig's live starts and the gated live-reading scripts, never in CI.
decision/self-bootstrapping-test-data · Self-bootstrapping test data — Each test bootstraps its own state in a fresh --home: the fake agent replays recorded fixtures, stamps come from viola verify against the fake agent, and no state is developer-seeded.
decision/tailing-ignores-symlinks · Tailing ignores symlinks and invalid names — Tailing ignores symlinks and invalid names, and an over-long ndjson line is counted in skipped.torn_lines.
decision/unknown-events-counted-skipped · Unknown events counted, not failed — Unknown event kinds and fields are counted in skipped rather than failing.
decision/waits-key-on-events · Test waits key on logged events — Test waits key on logged events and events.ndjson byte offsets or viola wait --after a cursor, never on sleeps.
contract/event-line/format · the event line · format — Events are ndjson, one JSON object a line: {"v":1,"ts","instance","kind","source","data"}.
contract/instance-liveness/live · instance liveness · live — An instance whose process runs and whose heartbeat is at most 5 s old is live.
contract/instance-liveness/stale · instance liveness · stale — An instance whose process runs but whose heartbeat is older than 5 s or absent is stale.
contract/viola-list-json/items · viola list --json · ok.items — In the {"v":1,"ok":...} document, ok.items is an array of {name, wrapped, liveness live|stale, status idle|busy|unknown, wheel? human|driver, dialog_pending, cli_version?, cli_verified?, parent?, role?, model?, effort?, window? {address, workspace}, origin? cli|agent}.
contract/viola-list-json/mirrors-channel · viola list --json · channel mirror — The CLI --json document mirrors the channel list result.
contract/viola-list-json/skipped · viola list --json · skipped — The ok payload's skipped holds unknown_kinds, unknown_fields and torn_lines.
decision/cargo-modules-orphans-gate · No orphan modules — Every lib and bin target is checked for orphan modules. · guard: cargo-modules orphans gate per lib/bin target
decision/chaos-tests-for-self-healing-state · Chaos tests for self-healing state — The crash-safe self-healing state is covered by chaos tests. · why: A driver of the Comprehensive tier.
decision/hook-deadline-timing · Hook deadlines and heartbeat staleness asserted — Spine hooks exit within the spine deadline, SessionEnd within about one second, and heartbeat staleness flips at more than five seconds, each held by a timing assertion.
decision/private-filesystem-permissions · Private filesystem permissions enforced — A home, instance directory or diagnostics folder that is group or world writable, or that another user may read, is refused, and a symlinked diagnostics folder or role file is refused.
decision/structured-json-process-logs · Structured JSON logs from every process — Every process (run, hook, mcp, cli, console) writes structured JSON-line logs parseable with jq, the harness has a logs command, and hook traces go to diagnostics/. · why: A binding founder direction.
contract/cli-human-output/no-sgr · CLI human output · SGR — Human-mode output carries no SGR sequences under a non-TTY, NO_COLOR or TERM=dumb.
contract/instance-liveness/gone · instance liveness · gone — An instance whose pid and start time no longer name a running process is gone, whatever the heartbeat's age.
contract/test-harness/boot · test harness commands · boot — boot builds viola and starts the wrappers in a fresh home to readiness.

## test-scope-summary · ## 1. Test Scope Summary · twin 1-test-scope-summary
decision/human-tty-through-outer-pty · Human TTY rendering through the outer PTY — The CLI's human TTY rendering is tested under a pseudo-TTY through the outer-PTY driver, not by screen inspection of the child agent. · guard: cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal
decision/never-parse-rendered-child-screen · Rendered child screen never parsed — Test assertions never parse the rendered child screen for content; the readiness gate is tested through its refusal outputs, and terminal output is checked only by byte search. · why: Named a test anti-pattern by the creator. · guard: chaos_feed_panic_refuses_send_and_keeps_passthrough · except: The test-only live rig reads the child's screen only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal asserts on.
decision/no-sgr-without-tty · No SGR without a terminal — CLI output carries no SGR under a non-TTY, NO_COLOR or TERM=dumb.
decision/real-claude-never-in-ci · Real claude CLI only locally — The real claude CLI runs only locally, never in CI; in CI viola verify runs only against the fake agent.
decision/torn-tail-healed-by-append · Torn log tail healed by the next append — A reader counts a torn last line in torn_lines and rewrites nothing; the next append heals it by writing one LF and its line in a single write, and logs one state-recovered line per heal. · guard: torn_tail
decision/cli-json-one-document · One JSON document per verb — Under --json a CLI verb signals by its exit code plus one JSON document on stdout. · guard: NOT_ERROR_BODIES · except: The exit-2 paths of viola answer and viola send write their fixed line to stderr whatever --json says, with no document; viola console takes no --json, its two refusals fixed stderr pairs.

## 2-test-strategy · ## 2. Test Strategy · twin test-strategy
decision/ci-never-real-claude · CI never touches the real agent CLI — CI never touches the real claude or the real hyprctl.
decision/complete-lines-reader · Live-appended files read by complete lines — Every reader of a live-appended file parses only newline-terminated lines, through the one shared complete-lines reader. · why: a Linux reader can observe an append part-way
decision/e2e-for-critical-paths · E2E for every critical path — Every critical path and the expansion scenarios have E2E tests against harness-booted wrappers over the fake agent, through the outer PTY and the rmcp client.
decision/heartbeat-staleness-pure-function · Heartbeat staleness checked without a clock — The heartbeat staleness flip is checked by a pure function fed the beat's age, with no clock.
decision/integration-suite-per-contract · Integration suite for every standard contract — Every architecture standard contract (wrapper channel, on-disk, hooks, CLI, MCP) has a crate-level integration suite over a real interprocess endpoint, a real tempfile home and the built binary run as a process.
decision/no-human-in-loop-verification · No human-in-loop verification — Verification has no human-in-loop step, no visual review service and no manual exploratory step.
decision/no-test-retries-or-sleeps · No test retries or sleep synchronisation — nextest runs with no retries, and no thread or tokio sleep is used for synchronisation, save as the poll interval of a loop that rechecks its condition, which in a test wait also runs under a deadline.

## test-strategy · ## 2. Test Strategy · twin 2-test-strategy
decision/cross-surface-trigger-layer · Cross-surface trigger layer — Cross-surface coordination is tested at E2E, with cross-surface parity checks.
decision/discipline-at-every-layer · Discipline held at every layer — No screen parsing, no blanket approval and no flaky tests hold at every test layer. · except: The test-only live rig reads the child's screen into closed words, by compiled literals and the box row's cell attributes, never text, and passes a read trust dialog under its key grant.
decision/e2e-over-fake-agent · E2E over the fake agent — Every critical path and expansion scenario has an E2E test against a harness-booted viola run over the fake agent, driven through an outer PTY and an rmcp client.
decision/heartbeat-staleness-pure-classifier · Heartbeat staleness by a pure classifier — Liveness is classified live, stale or gone by a pure function fed the beat's age with no clock: a dead pid or one of another start time is gone, and for the same process a beat older than 5 s or absent is stale.
decision/os-gated-tests-registered · OS-gated tests registered with a reason — Every OS-gated test function and gated test file has a gates row with its reason; an unnamed or stale gate fails the run, and a gate whose predicate names windows or macos is a fault even when a row names it. · guard: contract_os_gated_tests
decision/tests-never-read-host-sessions · Tests never read the host's claude sessions — A test's own list child runs with PATH set to one bin dir alone, empty or holding the planted fake claude, and the fake agent is planted as claude only by shared code: two test-support helpers and the e2e harness boot.
decision/e2e-layer-by-binary-prefix · E2E layer selected by binary prefix — The E2E layer is the test binaries whose names match path, tui, mcp, cross, chaos or contract followed by an underscore, in either package.
decision/filesystem-test-layers · Filesystem test layers — The filesystem vector is tested at integration and chaos, including strict-mode refusals.

## 3-test-harness-contract · ## 3. Test Harness Contract
contract/harness-session-record/agents-mode · harness session record · agents_mode — agents_mode is one of recorded, oversize or malformed, defaults to recorded, is written by boot and stands in supervise.json too.

## 4-unit-test-strategy · ## 4. Unit Test Strategy · twin unit-test-strategy
decision/mocks-only-at-product-seams · Mocks only at product seams — Unit tests use mockall doubles only for traits the product already defines at its seams: Pty, the liveness probe and Clock.
decision/rstest-fixtures-for-constructors · rstest fixtures for shared constructors — Shared unit-test constructors, such as a temporary home or a clock, are rstest fixtures.

## unit-test-strategy · ## 4. Unit Test Strategy · twin 4-unit-test-strategy
decision/agents-parser-fails-to-unknown · Agents list parse failure reads unknown — An oversize or malformed claude agents --json payload gives unknown and leaves the board as read, and a row's name is never taken as a viola name.
decision/defensive-log-reader · Event log reader counts and never panics — The event log reader never panics: torn lines are counted and not returned, lines of unknown kind are counted and not returned, and known-kind lines with unknown keys are counted once and returned. · guard: three_counts
decision/human-output-control-escaping · Human output escapes control characters — Human-mode wait and last output escape C0 and C1 control characters, keeping newline and tab.
decision/list-row-control-escaping · list rows show every control character as hex — In list rows every control character, newline, tab, DEL and C1 included, prints as \xHH text. · guard: escape_row_shows_every_control_character_as_hex_text
decision/scoped-capture-pinned-dispatch · Scoped log captures through a pinned dispatch — Every scoped log capture in the root binary's unit tests goes through one shared helper that first registers a pinned, always-interested dispatch, never the global default. · why: so no scoped capture is ever the only registered dispatcher and a line whose callsite another, subscriber-less thread registered first is not lost · guard: scoped_capture_sees_a_line_whose_callsite_another_thread_registered_first · except: The three crate-level scoped captures run on a bare scoped default; never observed red, they have no owner.

## 5-integration-test-strategy · ## 5. Integration Test Strategy · twin integration-test-strategy
decision/cli-outputs-pinned-by-literal · CLI outputs pinned by literal asserts — CLI and hook integration tests run the built viola binary as a process and pin its outputs by literal asserts.
decision/fake-agent-as-claude · Fake agent serves the claude boundary — The only external boundary, child processes, is served in tests by viola-fake-agent as claude on a test-scoped PATH, as hyprctl in a test-scoped bin dir, and as foot, the terminal stand-in.
decision/human-layouts-pinned-by-literal · Human-mode layouts pinned by literal — Human-mode layouts, the viola list board and viola --help, are pinned by literal asserts in their test files. · guard: HELP literal in cli_output_plain; cli_list board tests
decision/module-integration-direct-calls · Module integration by direct calls — Module-to-module integration is tested by direct calls across crate APIs with shared rstest fixtures.
decision/on-disk-state-tested-on-real-files · On-disk state tested on real files — Module-to-disk state is tested over real ndjson, snapshots and lock siblings in a tempfile home, with a concurrent-append check on the hook role and detail files, whose open and write paths every diagnostics file shares, that every line parses as one JSON object.
decision/pty-tested-on-real-openpty · PTY tested on real openpty spawns — PTY integration spawns viola-fake-agent through viola-pty on a real openpty, and a resize is read from the child's own size report alone, with no key written after it.
decision/real-disk-in-integration · Real disk in integration tests — Integration tests use a real tempfile filesystem.
decision/real-pty-in-integration · Real PTY in integration tests — Integration tests use the real portable-pty through viola-pty.
contract/cli-output/no-escape-without-tty · CLI output · escape sequences — Human layouts carry no ESC [ sequence under non-TTY, NO_COLOR or TERM=dumb.
contract/event-line/ts · the event line · ts — ts is RFC 3339 UTC with ms and Z.
contract/event-tailing/ignored-entries · event tailing · symlinks and invalid names — Tailing ignores symlinks and invalid names.
decision/fresh-home-per-test · Fresh home per test — Each test gets a fresh tempfile home at a not-yet-existing path under target/e2e-home that viola creates, a stamped home being a copy of its build's verify stamp, never a link; started wrappers sit in a Drop guard that kills and waits, and nothing is shared between tests. · why: so obs-plan's CI gates, the secret scan and the upload cover integration runs · except: The chaos feed panic test alone boots outside the scans.
contract/strict-modes-refusal/no-line · strict-modes refusal · no line — A refused check at run, viola revive, viola adopt, viola verify, viola focus, viola console, viola up's board scan, the frame verbs, the hook and the MCP frame tools writes no line.

## integration-test-strategy · ## 5. Integration Test Strategy · twin 5-integration-test-strategy
decision/human-output-pinned-by-literals · Human output pinned by literals — Fixed human output, such as the send mirror, the grouped help, the list board and verify's lines, is pinned by a literal in the test file itself.
decision/key-after-resize-waits-for-size · Key after a resize waits for the new size — A test that writes a key after a resize sends it only once the child reports the new size. · guard: spawn_runs_a_raw_child_that_sees_its_size_a_resize_and_its_own_exit_code · except: The live rig's KR arm types its key 0, 20 or 100 ms after a resize of its own pty, in its rehearsals and its live starts, since a key typed right after a resize is what it measures.
decision/module-boundary-integration-tests · Module boundaries tested by direct calls — The module-to-module boundary is tested by direct calls across crate APIs with shared rstest fixtures, run by cargo-nextest.
decision/no-escape-in-plain-output · No escape bytes in plain output — No ESC [ sequence is written when output is not a TTY, under NO_COLOR or under TERM=dumb, and clap's own help and usage errors into a pipe carry no ESC byte even under CLICOLOR_FORCE. · guard: cli_output_plain
decision/on-disk-state-integration-tests · On-disk state integration tests — Module-to-state integration tests use real ndjson logs, snapshots and lock siblings in a tempfile home, with real file-lock contention, in place of database tests. · why: The project has no database.
decision/pty-tests-real-openpty · PTY tests on a real openpty — PTY integration tests spawn viola-fake-agent through a real openpty via the viola-pty seam.
decision/snapshot-replayed-from-log · Broken snapshot replayed from the log — A snapshot cut short or of an unknown version is replayed from the event log, leaving the file's bytes, its modification time and the directory's file names unchanged.
decision/tolerant-event-reader · Tolerant event log reader — The event reader counts lines outside the contract while still returning known kinds, counts and does not return a kind no build knows, and an append after a log cut short starts on a fresh line.
decision/guarded-paths-start-stamped · Check-guarded cases start stamped — A test case that drives a check-guarded path starts from a stamped home.
decision/home-read-rule · Home readable by another user is refused — A home another user may read is refused: viola run exits 1 with the home pair and creates no diagnostics, viola list refuses the whole board, opening nothing, and viola console exits 1 with its home pair, writing no line. · guard: a_home_another_user_may_read_refuses_run
decision/home-resolution-order · Home resolution order — The home resolves from --home, then the VIOLA_NAME + VIOLA_DIR instance home, then <user home>/.viola, canonicalised once it exists. · why: The CLI verbs' VIOLA_DIR step and the canonicalisation are a ratified widening. · guard: home_resolution_takes_the_flag_then_the_session_then_the_default
contract/event-data/dialog-expired · Event data keys · dialog-expired — data holds the keys dialog_id and dialog_kind.
contract/event-line/keys · Event line · keys — Every event kind's line is {"v":1,"ts","instance","kind","source","data"} and round-trips through the state writer and reader.

## 6-e2e-test-strategy · ## 6. E2E Test Strategy · twin e2e-test-strategy
decision/diagnostics-schema-conformance · Diagnostics schema conformance — After the test steps every home-level diagnostics line is validated against diag-line.v1 and every instance detail line against diag-detail.v1; any failing line fails the step. · why: the check body behind obs-plan gate G4; obs owns the schemas, tests own the check
decision/forward-compat-lines-skipped · Forward-compatible lines skipped — Log lines of a newer form are counted in skipped rather than failing the read.
decision/list-names-escaped · List names escaped — Unwrapped session names holding newline, tab or a C1 character print escaped in every mode: no row splits, columns stay aligned to the caption row and the raw bytes never reach a NAME cell. · guard: cross_list_rows
decision/non-json-lines-skipped · Non-JSON diagnostics lines skipped — A diagnostics line that is not JSON is skipped and counted, not failed. · why: chaos cases tear lines by design
decision/tty-asserted-only-on-verb-sgr · TTY rendering asserted only on verb SGR — CLI verb tests pin --json documents and human layouts by literals, and assert TTY rendering only on the SGR sequences in verb output under the outer PTY, never on the child agent's screen.
decision/bounded-readers · Every external reader is bounded — Every product reader of external input is bounded in-statement or named in the enumeration with a witness test. · guard: product_readers_are_each_bounded_or_named · except: Reads of viola's own executable take no external input and are listed as exceptions.
decision/env-clear-keeps-llvm-profile-file · env_clear tests re-add LLVM_PROFILE_FILE — Any test that uses env_clear() must re-add LLVM_PROFILE_FILE.
decision/jsonschema-validator · jsonschema validates diagnostics — jsonschema is what validates diagnostics lines against their schemas.
decision/live-failing-reading-not-verification · Live reading recorded as read, never as verification — A live reading that fails a ledger row is recorded as read and never as verification of the row; a confounded live reading leaves the row unverified on that CLI version and its live re-read owed.
decision/snapshot-replay-from-log · Unreadable snapshot replayed from the log — Over a truncated, unreadable or newer-version snapshot, a start and the product readers replay instance state from the event log, links, the wheel, the agent session id and the launch pair included, with no dialog pending, and write no file. · guard: a_start_over_an_unreadable_snapshot_replays_the_log, the e5_the_replay_reads_ case of cli_links
decision/start-hands-wheel-to-driver · Start hands the wheel to the driver — Every start appends wheel{holder:"driver", cause:"start"}, so after a restart the wheel is the driver's.
decision/torn-append-healed · Torn append healed — A torn log append is counted and the next record starts on a fresh line.
decision/tui-passthrough-never-parses-screen · Passthrough verdicts never parse the rendered screen — viola run passthrough tests detect exit only via child wait, drain the master reader on its own thread, and take verdicts from events.ndjson, the receipt and the exit status, never from a parse of the rendered screen. · except: The forced vt100 panic chaos case byte-searches the drained master reader's stream for the child's own bytes.
contract/list-json/name · list --json · name — list --json carries a session's original name string, escaped as JSON, never the escaped form the table prints.
contract/snapshot/run-fields · the instance snapshot · pinned path, pids and endpoint — The snapshot of a running instance holds the pinned path, the pids and the endpoint, and list --json reads the instance live.
contract/viola-run/stale-heartbeat · viola run · stale-heartbeat — A stopped wrapper is read stale, and a new run of its name is refused stale-heartbeat.

## e2e-test-strategy · ## 6. E2E Test Strategy · twin 6-e2e-test-strategy
decision/cli-e2e-via-process · CLI tested as a process — CLI verbs are tested end to end by running the viola binary as a child process.
decision/cli-fixed-strings-pinned-by-literal · CLI fixed strings pinned by literals — Each fixed CLI string, the human layouts included, is pinned by a literal in its test file.
decision/console-restore-read-on-own-terminal · Console restore read on its own terminal — The console's terminal restore is asserted by reading line mode on the console's own terminal, opened without a controlling tty, raw while the frame stands and cooked after the quit; equality with the mode before the start is not asserted. · why: The PTY seam exposes no master fd, and a read before the start races the console's own raw-mode entry. · guard: console_restores_the_terminal_on_quit
decision/contract-tables-are-literal · Contract tables are literal — Contract tests hold literal tables, never the product's own list.
decision/forward-compatible-log-reading · Forward-compatible event log reading — Lines of an unknown kind and unknown fields on a known kind are counted and skipped; list --json still exits 0 and the known-kind line still counts toward item state. · guard: chaos_lines_of_a_newer_build_are_counted_and_the_board_still_reads
decision/library-surface-at-unit-tier · Library-only surface covered at unit tier — Library-only surfaces are covered by nextest unit tests and doctests, not end to end.
decision/list-rows-escape-control-chars · list rows escape control characters — viola list never prints raw newline, tab or C1 bytes in a NAME cell, so each row stays one line and every column starts at its caption offset in every mode. · guard: cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode
decision/one-negative-per-security-control · One negative test per security control — Every security-plan control has a negative test of its own.
decision/strict-modes-before-any-log · Strict-modes check before any read or open — Frame verbs, link verbs, starts, viola focus, viola console and viola up's board scan take the strict-modes check before opening any log, so a widened state is refused with no frame, no line and its mode read back as found. · why: A check placed after the role file opens would pass, because opening a role file narrows an existing home to 0700. · guard: link_verbs_refuse_a_widened_instance_before_any_log, frame_verb_refuses_a_widened_instance_before_any_log
decision/torn-append-heals · Torn append heals on the next start — A torn last log line is counted in skipped.torn_lines, the next append starts on a fresh line with every earlier byte unchanged, and one state-recovered line is logged. · guard: chaos_torn_append
decision/tty-styling-asserted-on-sgr · Terminal styling asserted on SGR sequences — Terminal styling is checked by running a verb inside the outer PTY and asserting only on the SGR sequences in verb output, never on the child agent screen.
decision/tui-verdicts-from-records · TUI verdicts from records — TUI verdicts come from events.ndjson, the fake-agent receipt and the exit status; the fleet console's verdicts come from its console.ndjson lines and its terminal's line mode, never from the screen. · except: The forced vt100 panic case byte-searches what the drained master reader read.
decision/window-placement-proven-live · Real window placement proven only live — A real terminal window placed on its parent's workspace, or on the workspace its snapshot recorded when viola up re-opens it, on Hyprland is proven only by a live reading, a placing open's kept view only by the gated live reading R7; the fakes prove the placement words and the window records only.
contract/list-human/columns · viola list human board · columns — The caption row is NAME · LIVE · STATUS · WHEEL · DIALOG · CLI, and the table is fixed-width.
contract/list-json/liveness-stale · list --json · liveness stale — A wrapper whose heartbeat is older than 5 s reads liveness "stale"; a killed one is omitted as gone.
contract/viola-list-json/live-instance · viola list --json · running wrapped instance — A running wrapped instance's item reads wrapped:true and liveness "live".
decision/diagnostics-files-0600 · Diagnostics files are owner-only — diagnostics/ files are mode 0600.
decision/list-reads-live-through-replay · list reads a live wrapper through the replay — list reads an instance with an unreadable or newer snapshot and a fresh beat as live, never stale, through the replay, logging one state-recovered line. · guard: e5_list_reads_a_live_wrapper_through_the_replay
decision/outer-pty-reader-drained · Outer PTY reader drained on its own thread — An outer-PTY test drains the master reader on its own thread.
decision/r2-refuses-before-anything-starts · R2 live reading refuses an unfit desktop or scratch — The R2 live reading refuses before anything is made or started, printing r2: red precondition=scratch when its scratch home or its runtime-dir entry already exists, or r2: red precondition=locked when the desktop's locked reading is not exactly false, and exits 1 closing no window.
decision/secrets-never-in-logs · Secrets never in logs or diagnostics — No stripped-variable value appears in any log or diagnostics file.
decision/window-raise-proven-live · Real window raise proven only by the gated live reading R3 — A real raise of a session's window by viola focus is proven only by the gated live reading R3 beside R2; its rehearse form over the fakes is never a live reading, and its verdict words come only from the plan's closed set.
contract/dialog-events/logged-once · dialog events · question, permission, plan — Each dialog event is logged exactly once with a dialog_id, and a parked wait returns it at once.
contract/event-log/start-wheel-record · instance event log · first record of a start — Every start, a revived one included, appends wheel{holder:"driver", cause:"start"} first and the hook's session-start last; where folder-trust keys pass through the wrapper a wheel{holder:"human", cause:"human-input"} record precedes session-start and a later human key adds none.
contract/events/human-input-wheel · events.ndjson · human-input — Human editing input logs a `wheel{holder:"human",cause:"human-input"}` record.
contract/list-json/skipped-counts · list --json · skipped — skipped.torn_lines, skipped.unknown_kinds and skipped.unknown_fields each count the log lines skipped for that reason.
contract/mcp-tools/result-body · MCP tools · result and refusal body — Each MCP tool's result and refusal equal the CLI's --json document less v, save instance-unreachable with detail call-deadline, which the MCP frame tools alone write and which has no CLI document.

## 7-test-data-fixtures · ## 7. Test Data & Fixtures · twin test-data-fixtures
decision/fake-agent-on-test-path · Fake agent on a test-scoped PATH — The fake agent stands as claude in a test-scoped bin dir, or as hyprctl or as foot, the terminal stand-in, by its own file name, answering from files beside its exe with no env var, and every test-started list or revive runs under a test-scoped PATH.
decision/harness-boot-creates-stamps-and-starts · Harness boot makes the home, stamps it and starts wrappers — The harness boot takes a fresh home under a new session folder, stamps it through viola verify unless asked to leave it unstamped, then starts one wrapper per instance under a supervisor and waits until each is ready. · why: E2e tests and agent sessions rely on a stamped, running home before they seed their own data.
decision/human-output-as-literals · Human-mode output as literals — Expected human-mode output, such as help text and the list board, is held as literals in its test files.
decision/injected-clock · injected clock for time-sensitive data — Time-sensitive data is injected per test behind viola_core::Clock, a monotonic clock tests advance, or handed to a pure decision as an instant; the wall clock is read directly, through Utc::now, for record stamps and for the instant a decision takes. · except: A link line's since is stamped from the wall clock and asserted by its shape only.
decision/product-state-from-real-runs · Product state from real runs only — On-disk product state is produced only by real viola runs against the fake agent in a fresh home, never hand-written. · except: chaos tests corrupt files the product already wrote; server-identity impostors build their own snapshot.json; unit tests and viola-state's own tests seed state files for the code under test.
decision/time-as-argument · pure time decisions take their instant — A pure time decision takes its instant as an argument instead of reading a clock.
decision/env-per-child · environment set per child process — Test environment is set per child process with Command::env, never with std::env::set_var. · why: std::env::set_var is unsafe in edition 2024.
decision/per-test-home · isolated home per test — Each test has its own home, so no endpoint or log is shared between tests.
decision/stamps-written-only-by-verify · Stamps written only by verify — ledger/stamps.json is produced by viola verify against the fake agent; no test or harness code writes it. · except: viola-state's own unit tests write it, through the stamps store or as seeded bytes, to test that store and its strict-modes check.
decision/test-homes-made-private · Test-made homes are private directories — A home a test makes before viola runs is made through the product's private-directory creation, never by plain recursive creation. · why: A plain home is one the product refuses by the read rule, its 0755 mode failing the rule.
decision/tests-seed-own-data · self-bootstrapping tests — Tests seed their own data; no developer pre-populates a home.

## test-data-fixtures · ## 7. Test Data & Fixtures · twin 7-test-data-fixtures
decision/agents-stub-from-files-no-env · Agents list answered by a stub on PATH — claude agents --json is answered in tests by the fake agent placed as claude in a test-scoped bin dir, from files beside its exe and with no env var, and every test-started list or revive runs under a test-scoped PATH. · why: The no-env answer was ratified.
decision/human-output-expected-as-literals · Human-mode expected output as literals — Human-mode expected output is held as literals in the test files that check it.
decision/product-state-never-hand-written · On-disk product state produced by viola — On-disk product state in tests is produced only by running real viola verbs against the fake agent in a fresh home, never hand-written. · except: The chaos tests, which corrupt files the product already wrote; the server-identity impostor cases; and the MCP handler suite's own snapshots.
decision/test-clocks-behind-clock-seam · Test clocks behind the clock seam — Time in the sync crates is faked through the core Clock seam with hand-written test clocks; no mocking crate stands behind it.
decision/wrapper-boots-pass-trusted-screens · Test wrapper boots run trusted with screens — Every wrapper boot in the test support and the harness passes the fake agent --screens and a trusted root of its cwd. · except: The untrusted boot passes no root, and a test whose fake agent writes another screen keeps its own verdict.
decision/fake-agent-argv-only · Fake agent configured by argv alone — The fake agent takes its options and mode words from argv alone, no env var, and ignores unknown arguments. · except: Standing as hyprctl by its own file name, the fake agent reads its mode and reply set from files beside its exe; a fake-args file beside its exe goes ahead of argv, argv winning.
decision/fake-agent-copies-planted-first · Fake-agent copies planted before any child — A test that plants more than one fake-agent copy plants every copy one after another before any thread starts a child. · why: On tmpfs each copy is written rather than hard-linked, and a child forked by one thread inherits another thread's open write handle (ETXTBSY).

## 8-mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin mocking-stubbing-discipline
decision/fakes-for-external-services · fakes replace external services — The real claude CLI is replaced by the native viola-fake-agent binary, both as the PTY child and as claude on a test-scoped PATH, the real hyprctl by the same binary as hyprctl, and the real foot terminal by the same binary as foot; tests never reach the real services. · except: viola-hyprland's own unit tests serve hyprctl by a planted shell stub, because cargo-mutants runs only the mutated package's tests, so a root-package test cannot kill that crate's mutants.
decision/filesystem-not-mocked · real filesystem in tests — The filesystem and on-disk state are not mocked; a real tempfile home is used everywhere. · why: Modes, file locking, torn lines and symlink handling are the behaviour under test.
decision/mockall-on-seam-traits-only · mockall doubles on seam traits only — Test doubles are mockall automock, under test only, on traits viola already defines at its seams, with dependency injection preferred over runtime patching of production code; external processes are real stub executables.
decision/no-mocks-in-e2e · nothing mocked in E2E — E2E tests mock none of the wrapper channel, PTY, vt100, serde or viola hook invocations; the fake agent calls the real pinned viola binary, and code paths that must be verified end-to-end go in E2E.
decision/pure-functions-tested-directly · pure functions tested directly — Pure functions in modules under test, such as parsers and refusal ordering, are tested directly and never mocked.

## mocking-stubbing-discipline · ## 8. Mocking & Stubbing Discipline · twin 8-mocking-stubbing-discipline
decision/external-processes-real-stubs · External processes are real stub executables — External processes in tests are real stub executables, never doubles; root tests resolve the fake agent through Cargo's binary path variable, viola-e2e tests from the test executable's own target dir.
decision/real-process-faults · Process faults are real — A stale holder is a real wrapper stopped by SIGSTOP under the Frozen guard, and a wrapper lost under an open call is a real kill; no double stands in for either.
decision/time-faked-through-seams · Time faked through injected clocks — Time is faked only through injected seams, with no mock_instant and no paused Tokio clock: the viola_core Clock, the MCP adapter's injected deadline provider, which tests fire by a signal and never by waiting out the 55 s MCP call deadline, and a pure decision takes its instant as an argument.
decision/utc-now-read-directly · Wall clock read directly, not through the clock seam — chrono Utc::now() is read directly for wall-clock stamps and for the instant a pure decision takes; the clock seam carries monotonic Instants only. · except: The link line's since is stamped directly, so tests assert since by shape only.
decision/real-filesystem-faults · Filesystem faults are real — A widened home tree or a loosened socket directory is made on the real file and refused by the product's own check, never a stubbed one, and read back as found. · except: The pure socket directory verdict takes a foreign owner's uid by injection at the unit tier.

## 9-ci-integration · ## 9. CI Integration · twin ci-integration
decision/no-orphan-modules · No orphan modules — Any cargo-modules orphan in any library or binary target fails the build. · guard: orphans-check.sh with its probe of two planted orphans

## 10-quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin quality-gates-coverage-targets
contract/heartbeat-staleness/age-threshold · Heartbeat staleness classification · heartbeat age — For the same process, a heartbeat aged 5.0 s classifies live and one aged 5.1 s classifies stale, held by an exact boundary test.
contract/heartbeat-staleness/identity-mismatch · Heartbeat staleness classification · pid or start-time mismatch — A pid or start-time mismatch classifies the instance gone at any age.

## quality-gates-coverage-targets · ## 10. Quality Gates & Coverage Targets · twin 10-quality-gates-coverage-targets
decision/stand-in-terminal-tree-teardown · One shared teardown ends a stand-in terminal's tree — A stand-in terminal's process tree is ended by one shared test teardown: it kills first only a leaf whose recorded parent still runs, waits for every other member under one deadline, and at that deadline kills only a pid whose start time still matches. · why: Killing a fake agent already in its own exit left cut-short coverage profiles that turned pre-push red.

## 11-test-anti-patterns-never-do-these · ## 11. Test Anti-Patterns (NEVER do these)
decision/e2e-asserts-only-cross-process · E2E asserts only cross-process behaviour — A case a lower layer can hold is never asserted at E2E: parser, refusal-order, threshold and clock cases belong to unit tests, contract shapes to contract tests, and E2E asserts only cross-process behaviour. · why: The test pyramid.
decision/every-surface-asserted · Every surface of a critical path asserted — A multi-surface critical path is never asserted on one surface only; every surface a scenario lists needs its own driver step and transition signal.
decision/literal-test-oracles · Test oracles written as literals — Tests never import the product's own identity floor, ledger rows or refusal ordering as the oracle; expected values such as the identity-floor names, the refusal order and exit codes are written as literals in the test. · except: The relied-row count is read from the ledger's own count constant at every test site, held by one literal pin beside the relied list.
decision/no-in-process-clock-across-processes · No in-process clock for another process — An in-process mock clock never controls another process's time; cross-process expiry is asserted in a unit case. · why: A global mock clock reaches only the test process's threads, never a spawned viola run.
decision/no-manual-smoke-step · No manual smoke step — No manual smoke step, such as a developer verifying before deploy, is part of the pipeline.
decision/no-over-mocking-integration · No over-mocking at integration level — Integration tests never over-mock. · why: Over-mocking stops testing the integration.
decision/no-over-stubbing · No over-stubbing — Tests never over-stub. · why: Over-stubbing stops testing real behaviour.
decision/no-real-cli-in-ci · No real claude CLI in CI — The real `claude` CLI, and `viola verify` against it, never run in CI; suites reach the real CLI only through a local-live run, and live measurement of a real session is local too. · guard: `agent-run run --local-live` refuses when `CI` is set
decision/no-runtime-monkey-patching · No runtime monkey-patching — Production code is never monkey-patched at runtime; dependency injection is used instead.
decision/no-screen-parsing · Rendered child screen is never parsed — Tests never parse the rendered child screen (the vt100 grid or PTY master output) for content; verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt. · except: The test-only live rig reads the child's screen only into closed words, by compiled literals and the box row's cell attributes, never text, which its rehearsal asserts on.
decision/no-sleep-synchronization · No sleep for synchronization — Tests never synchronize with a sleep; they wait for an explicit signal such as an `events.ndjson` offset, `viola wait --after` or a status field.
decision/no-test-owned-real-time · No test-owned real time — Test-side code uses deterministic, injected clocks: no test-owned timer, sleep or elapsed-time verdict, save a generous bound on a latency no injected clock reaches, in another process or on a real socket. · except: E2E may block on a product deadline it cannot inject when the verdict is the product's own exit code or status field and the same deadline is clock-driven in a unit case.
decision/real-on-disk-state · On-disk state is never mocked — Tests never mock the on-disk state and use a real tempfile home instead. · why: The architecture has no database; this is the database-equivalent rule.
decision/state-files-never-hand-written · State files are never hand-written — Every booted-wrapper test takes `ledger/stamps.json` from `viola verify` and `snapshot.json` from the wrapper, never writing them by hand. · except: A reader's own unit cases may plant a malformed state file by writing through the writer and then padding, replacing or re-versioning its bytes.
decision/chaos-faults-explicit · Chaos faults only from explicit operations — Chaos faults come only from explicit process or file operations or existing mockall seams, so chaos runs replay deterministically.
decision/child-exit-by-wait · Child exit detected by waiting on the handle — Child exit is detected by waiting on the child handle, never by reading master EOF.
decision/no-flaky-test-retries · No retries for flaky tests — No retry-once policy or any retry budget exists for flaky tests. · why: The zero-flakiness budget.
decision/no-mocking-unowned-code · No mocking of what is not owned — Libraries and the language runtime are never mocked without an abstraction layer.
decision/no-production-data-in-tests · No production data in tests — Tests never use production data; fixtures come only from `viola verify` probe text or a test-only recorder's capture, scrubbed of paths and usernames.

## 5-command-implementation · key of ## 3. Test Harness Contract
decision/diagnostics-schema-validation · Schema conformance check of diagnostics — Every diagnostics ndjson file under the test homes is validated with jsonschema 0.57.0: home-level role files against the diag-line v1 schema, detail files against the diag-detail v1 schema; this is obs-plan gate G4.
decision/closed-enums-need-decisions-log · Closed enums change only by decision — A new value of a closed harness enum or of the log event enum needs a Decisions Log entry.
decision/e2e-boot-model-by-cleanup · E2E boot model chosen by cleanup — A scenario whose cleanup is harness cleanup boots through the harness session guard, the same library code as the boot and cleanup commands; one whose cleanup is a temp-dir drop or a test-owned PTY uses the fixture chain.
decision/harness-scoped-path · Harness-scoped PATH — The fake agent is copied as the session's claude and its dir put first on a PATH passed per child, never set on the harness process.
decision/live-verify-never-in-ci · Live verify never in CI — verify against the real claude runs on the operator's host only; with CI set, --local-live refuses live-in-ci, exit 2, before any build, spawn or suite.
decision/log-assertions-via-jq-or-jaq · Log assertions through jq or jaq — Log assertions pipe the agent-run logs stream into jq -e (jq 1.8.2) or jaq 3.1.1.
decision/pre-push-local-linux-gate · pre-push local Linux gate — The operator pass runs pre-push, Linux host only, on the uncommitted working tree before the pre-CI commit, and a red stage stops the pass; pre-push is not an agent command.
decision/test-layer-filtersets · Test layers by nextest filterset — The unit layer is lib and bin targets; the E2E layer is the path, tui, mcp, cross, chaos and contract test binaries; the integration layer is the complement, so a test binary reports in exactly one layer.
decision/verify-sole-stamp-writer · verify is the only stamp writer — viola verify against the fake agent is the only writer of the ledger stamps; boot requires exit 0 and a last line of stamped with zero fail, unless --unstamped.
contract/boot-readiness/events · boot readiness · events — Event lines 1, 2, 3 and 4 are a wheel line with cause start, a lineage line with source wrapper, a launch line with source wrapper, then session-start with source hook; missing reads events.
contract/harness-boot/agents-mode · harness boot · --agents-mode — --agents-mode takes recorded, oversize or malformed, default recorded, an unknown value a usage error; it sets no env var and is recorded as agents_mode in the session records.
contract/secret-scan-class/content-canary · secret-scan hit class · content-canary — A content canary, failing only in a home-level role file.

## bootstrap-phases-derive-for-route-setup-project · key of ## 3. Test Harness Contract
decision/mutants-kill-below-cargo-mutants-timeout · Kill lines below cargo-mutants' timeout — Every in-test wait a root-package mutant can reach, and the 15 s mutants default kill, stay below cargo-mutants' 20 s timeout floor, so a caught mutant ends at its kill line and is never graded Timeout. · why: At or above that floor a hang grades Timeout instead of caught. · guard: test_deadlines_sit_below_the_nextest_kill_line
decision/within-7s-wait-bound · WITHIN bound for every other wait — WITHIN = 12 s is the bound of every test wait not bounded by a per-test kill.
decision/g2-zero-panics · G2 zero panics — CI's log gate G2 fails closed on any event panic role line under the test homes, run as a probe then the check in the test and perf jobs. · except: A panic_location of exactly src/cmd/hook/seam.rs:<digits>.
decision/harness-log-fields-owned-by-test-plan · Ownership of the harness-grepped log fields — The test plan owns the harness-grepped log fields (timestamp, level, target, message, event, process, instance, corr) and the logs wrapper shape; obs-plan may add fields, but renaming or removing one needs a Decisions Log entry in this plan first. · why: The harness greps logs for assertions, so a format break is a harness break.

## log-format · key of ## 3. Test Harness Contract
decision/codes-only-role-logs · Codes-only role logs, detail apart — Role log files carry codes only; content-bearing detail (chains, drift reports, panic payload and backtrace) goes only to the per-instance detail files.
contract/log-event-corr/parse-rejected · log event corr · parse-rejected — corr is null.
contract/log-event-corr/state-recovered · log event corr · state-recovered — corr is null.
contract/log-format/json-per-line · log format · line format — JSON per line, parseable with jq -c select(...) or an equivalent; never a multi-line stack trace.
contract/log-line/corr · process log line · corr — Copied unchanged as a JSON number or string, never renamed (for example to correlation_id).
contract/log-line/event · process log line · event — A closed kebab-case enum; each value fixes what its corr holds.
contract/log-line/level · process log line · level — One of DEBUG, INFO, WARN, ERROR.
contract/log-line/null-encoding · process log line · null encoding — A null corr or instance is written as key absence; the harness and every jq or jaq assertion treat an absent key and null as the same value, and the diag-line schema rejects a literal null.
contract/log-line/process · process log line · process — One of run, hook, mcp, cli, console; cli is a short-lived verb with a resolved instance, writing its own cli role file; console writes console.ndjson with or without an instance.
contract/log-streams/event-stream · log streams · event stream — Product events in each instance's events.ndjson, the arch Event line {"v":1,"ts","instance","kind","source":"hook|wrapper","data"}.
decision/diag-line-schema-requires-corr · Schema requires corr where always defined — The diag-line schema requires corr exactly where the event rules always define it: dialog-raised, dialog-answered, release-from-driver, channel-request unless hook.event, channel-response unless -32700/-32600, wrapper-side send-*, and detail-less pre-tool-use or permission-request hook-decision.
decision/process-logs-via-tracing-subscriber · tracing-subscriber JSON process logs — Process logs are emitted by tracing-subscriber 0.3.23 as flattened JSON without the current span, one role file per run, hook, mcp, cli and console process in the home's diagnostics.
contract/log-files/modes · log files · modes and writes — Files are 0600, dirs 0700, with one write per line.
contract/log-line/instance · process log line · instance — A ViolaName or null.

## test-data-bootstrap · key of ## 3. Test Harness Contract
decision/run-under-test-owned-user-home · Test runs and revives under a test-owned user home — Every test-started viola run and viola revive runs under a test-owned user home, never the runner's real HOME. · why: The start writes into the user's Claude config.

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
