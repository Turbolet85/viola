# Apply — test-plan · part 1 of 2

## Verdicts
- fanout-test-plan.md 1 · applied · .andromeda/test-plan.md:794#1
- fanout-test-plan.md 2 · applied · .andromeda/test-plan.md:227#1
- fanout-test-plan.md 3 · applied · .andromeda/test-plan.md:616#1
- fanout-test-plan.md 4 · applied · .andromeda/test-plan.md:471#1
- fanout-test-plan.md 5 · applied · .andromeda/test-plan.md:102#1
- fanout-test-plan.md 6 · applied · .andromeda/test-plan.md:196#1
- fanout-test-plan.md 7 · applied · .andromeda/test-plan.md:388+#1
- fanout-test-plan.md 8 · applied · .andromeda/test-plan.md:388+#2
- fanout-test-plan.md 9 · applied · .andromeda/test-plan.md:382#1
- fanout-test-plan.md 10 · applied · .andromeda/test-plan.md:620#1
- fanout-test-plan.md 11 · applied · .andromeda/test-plan.md:620+#1
- fanout-test-plan--cascade-new-unit-test-strategy.md 1 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:48+#1
- fanout-test-plan--cascade.md 3 · applied · .andromeda/test-plan-detail/unit-test-strategy.md:30+#1
- fanout-test-plan--cascade.md 1 · applied · .andromeda/test-plan-detail/test-scope-summary.md:54#1
- fanout-test-plan--cascade.md 2 · applied · .andromeda/test-plan-detail/test-scope-summary.md:70+#1

## Edits
- line: .andromeda/test-plan.md:794
  old: «`deny-probes: 13/13 banned, control clean`»
  new: «`deny-probes: 14/14 banned, control clean`»
- line: .andromeda/test-plan.md:227
  old: «`input-not-ready` with passthrough continuing»
  new: «`input-not-ready` with passthrough continuing, witnessed on the outer terminal: the child's own bytes reach it on Unix and no viola literal appears on it on every OS (the Windows byte reading owed to the route entry "Windows-only live measurements")»
- line: .andromeda/test-plan.md:616
  old: «a forced vt100 panic refusing `send` `input-not-ready` while passthrough continues,»
  new: «a forced vt100 panic refusing `send` `input-not-ready` while passthrough continues on the outer terminal (§1 security-vector-coverage (child spawn)),»
- line: .andromeda/test-plan.md:471
  old: «verdicts from `events.ndjson`, the receipt and the exit status.»
  new: «verdicts from `events.ndjson`, the receipt and the exit status, and for the forced vt100 panic chaos case a byte search of the drained master reader's stream (the child's own bytes on Unix, no viola literal on every OS), never a parse of the rendered screen.»
- line: .andromeda/test-plan.md:102
  old: «its signal is `events.ndjson`, the fake agent's receipt and exit detection on the process handle;»
  new: «its signal is `events.ndjson`, the fake agent's receipt, exit detection on the process handle and, for the forced vt100 panic case, a byte search of the outer terminal;»
- line: .andromeda/test-plan.md:196
  old: «are rejected by the handler. `isError` payloads carry codes only.»
  new: «are rejected by the handler. `isError` payloads carry codes only. A `viola mcp` stdin line past `MAX_FRAME` is refused whole and ends the session: the earlier lines answered, exit 0, stderr empty, stdout only the earlier frames, and one `parse-rejected` `mcp-stdin` / `oversize` WARN line holding no byte of it.»
- after: .andromeda/test-plan.md:388
  anchor: «  - `tools/list` is exactly `send, wait, last, answer, list`»
  add: «  - the stdin line cap: a line of exactly `MAX_FRAME` bytes, its newline counted, passes whole, and one byte more passes no byte and reads as end of stream across any split of the reads; `serve` ends the session at a line past the cap»
- after: .andromeda/test-plan.md:388
  anchor: «  - `tools/list` is exactly `send, wait, last, answer, list`»
  add: «  - not driven by any test: a stdin line past `MAX_FRAME` landing before `initialize` completes ends `serve` with `ServeError::Initialize` (exit 11), as a closed stdin before `initialize` does; owner: the route entry "Sanitised error surfaces", as a PREREQ»
- line: .andromeda/test-plan.md:382
  old: «and the re-hash of a copy another start placed (`pin::place_absent`).»
  new: «and the re-hash of a copy another start placed (`pin::place_absent`), and the bounded compare read of `replace_private_shared` (at most one byte past the bytes, its outcome a whole-file compare's).»
- line: .andromeda/test-plan.md:620
  old: «schema; and the pinned-exe content hash is pinned to its library by literal vectors.»
  new: «schema; the pinned-exe content hash is pinned to its library by literal vectors; and `tests/contract_bounded_readers.rs` holds the bounded-reader enumeration: its `BOUNDARIES` table names each external reader's bounded read and a witness test, its `EXCEPTIONS` table the reads that take no external input, and a census over the product sources reads every reader bounded in-statement or named (`boundaries_each_name_a_bounded_read_that_stands`, `product_readers_are_each_bounded_or_named`, the census's self-check `census_finds_an_unbounded_read_and_passes_a_bounded_one`), beside `serde_json_unbounded_depth_is_banned_in_deny_toml`.»
- after: .andromeda/test-plan.md:620
  anchor: «- **Contract suite** (`contract_*.rs`): for each recorded fixture set, walked at run time (an empty walk fails), the fake agent's hook sequence and payloads equal the recorded order and bytes, never an insta snapshot; the forwarded `annotations` are asserted; every fixture is checked against the tolerant parser schema; and the pinned-exe content hash is pinned to its library by literal vectors.»
  add: «  - Owed: the enumeration's rows for the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` name a witness that reads the bounded read without driving past its cap, so their caps are held by the row's needle and the census alone; owner: the route entry "Sanitised error surfaces", as a PREREQ (a cap-driving case for each row).»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:48
  anchor: «  - The vt100 feed runs under `catch_unwind`, and a forced panic gives `input-not-ready`.»
  add: «- **viola-mcp:**»
  add: «  - The stdin line cap (`crates/viola-mcp/src/stdin.rs`, `stdin::tests`): `CappedLines` yields whole lines of at most the cap bytes, the `\n` counted; a line of exactly the cap passes whole, and one byte more releases no byte, sets the cut flag and reads as end of stream, at the literal `MAX_FRAME` (`16 * 1024 * 1024`) too; the count restarts after each newline; an over-long line split across reads is cut and a fitting one split across reads is joined (`capped_lines_passes_a_line_within_the_cap_whole_and_cuts_a_longer_one`, `capped_lines_yields_a_fitting_line_then_ends_at_an_over_long_one`, `capped_lines_restarts_the_count_after_each_newline`, `capped_lines_cuts_an_over_long_line_split_across_reads`, `capped_lines_joins_a_fitting_line_split_across_reads`, `capped_lines_at_max_frame_passes_the_cap_and_cuts_one_byte_more`).»
  add: «  - The `serve` child cases (`crates/viola-mcp/src/lib.rs`), each child killed at its 5 s bound: `serve_ends_the_session_at_a_line_past_max_frame` and `serve_on_a_closed_stdin_is_the_initialize_error` (`ServeError::Initialize`).»
- after: .andromeda/test-plan-detail/unit-test-strategy.md:30
  anchor: «  - The pinned copy's placement (`fs::place_private_new`): `place_private_new_places_the_bytes_once_at_the_mode_and_leaves_no_temp` (0700 / 0600 on Unix, the mode set before any byte), `place_private_new_over_a_present_name_is_present_and_leaves_it` (`Placed::Present`, the target byte-identical, no temp left), `place_private_new_under_a_missing_dir_fails`, and the forced-window cases `place_private_new_placed_copy_execs_at_once` (Unix) and `place_private_new_placed_copy_runs_at_once` (Windows) (§5); the lost placement (`pin::place_absent`): `place_absent_over_a_copy_another_start_placed` (the same bytes reused, other bytes `PinError::HashMismatch` with the file byte-identical) and `place_absent_on_a_free_path_writes_the_copy`.»
  add: «  - The bounded compare (`fs::replace_private_shared`'s fallback, through the private `holds_exactly`): `replace_private_shared_reads_no_more_than_one_byte_past_the_bytes` reads the target through `take(bytes.len() + 1)`, never the whole file, its outcome a whole-file compare's on every OS for an equal, a longer, a shorter, a different and an absent target; the refusal of a longer target under a read-only directory runs on Unix only, Windows using a read-only file.»
- line: .andromeda/test-plan-detail/test-scope-summary.md:54
  old: «and process exit code / exit detection on the process handle»
  new: «and process exit code / exit detection on the process handle; for the forced vt100 panic chaos case, a byte search of the outer terminal's drained stream: the child's own bytes (`e4 b8 ad`) arrive on Unix and none of `panic`, `parse-rejected`, `vt100`, `unable`, `hint:` appears on it on every OS (`tests/chaos_feed_panic.rs` `chaos_feed_panic_refuses_send_and_keeps_passthrough`; the Windows byte reading owed to the route entry "Windows-only live measurements"), a byte search, never a parse of the rendered child screen for content»
- after: .andromeda/test-plan-detail/test-scope-summary.md:70
  anchor: «  - **As landed (the instance check):** the frame tools `send`, `wait`, `last` and `answer` run `check_instance` before the liveness pre-check; a refused instance returns `isError:true` `{"error":"instance-unreachable","detail":null}` with no frame and no line (`crates/viola-mcp/tests/handlers.rs` `handlers_frame_tools_refuse_a_widened_instance_before_any_frame`).»
  add: «  - **As landed (the stdin line cap):** `viola mcp` reads its stdin through a line reader capped at `MAX_FRAME` (`CappedLines`); a line past the cap passes no byte, is never truncated or skipped, and ends the session as a stdin close does, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` WARN line holding no byte of it; on an initialized session the earlier lines are answered, and `viola mcp` exits 0 with stderr empty and stdout only the earlier frames (`crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line`).»

## Patterns
[[pattern]]
id = "tp1-01"
fixed = 'deny-probes: 13/13 banned'
