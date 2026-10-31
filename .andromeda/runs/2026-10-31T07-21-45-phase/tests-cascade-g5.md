# tests · cascade read · group 5 of 7

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:10) The CLI `--json` witness, driving a verb's failure with a path, an upstream text and a planted value in play, is driven this way, its document read from stdout.
  > | cli (verbs + `--json`) | assert_cmd 2.2.2 + predicates 3.1.4;
  (.andromeda/test-plan.md:469)
  > **cli (verbs + `--json`):** assert_cmd with predicates
- (.andromeda/test-plan-detail/e2e-test-strategy.md:11) The owed cap-driving case of the status-line command's stdout row runs through `viola hook statusline`, under this driver, and the five edited rows' owed clause names this chunk its owner.
  > | cli (`viola hook` / `hook statusline`) | assert_cmd 2.2.2 with `Instant` deadline | direct | Fixture stdin from `fixtures/claude/<ver>/`. `.env_clear()` must re-add `LLVM_PROFILE_FILE`. |
  (.andromeda/test-plan.md:471)
  > **cli (`viola hook` / `hook statusline`):** assert_cmd with an `Instant` deadline over recorded fixture stdin; `.env_clear()` must re-add `LLVM_PROFILE_FILE`.
  (.andromeda/test-plan.md:622)
  > `tests/contract_bounded_readers.rs` holds the bounded-reader enumeration: its `BOUNDARIES` table names each external reader's bounded read and a witness test, its `EXCEPTIONS` table the reads that take no external input, and a census over the product sources reads every reader bounded in-statement or named (`boundaries_each_name_a_bounded_read_that_stands`, `product_readers_are_each_bounded_or_named`, the census's self-check `census_finds_an_unbounded_read_and_passes_a_bounded_one`)
  (.andromeda/test-plan.md:623)
  > Owed: the enumeration's rows for the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` name a witness that reads the bounded read without driving past its cap, so their caps are held by the row's needle and the census alone; owner: the route entry "Sanitised error surfaces", as a PREREQ (a cap-driving case for each row).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:13) The channel witness must drive a wrapper's JSON-RPC errors over this driver, validating each error shape with jsonschema and reading its endpoint from nowhere but `snapshot.json`; its error bodies, the planted stripped value and any seam forcing `-32603` stay inside the sanitization and canary lines riding here.
  > | ipc-internal (wrapper channel) | viola-channel sync client + interprocess 2.4.4 raw stream + jsonschema 0.57.0 | direct | The endpoint is read only from `snapshot.json`. |
  (.andromeda/test-plan.md:475)
  > **ipc-internal (wrapper channel):** the sync client, a raw interprocess stream and jsonschema; the endpoint is read only from `snapshot.json`.
  (.andromeda/test-plan.md:605)
  > **Error sanitization and secret scan:** after every E2E test, no absolute path, serde path or `Caused by` in any error payload, every `-32603` exactly `"internal error"` / `null`, no `Cookie` header or stripped-variable value in any log, `diagnostics/` files 0600 on Unix, and the fixed canary in no home-level diagnostics line; a hit is reported by place and class, never the matched bytes.
  (.andromeda/test-plan.md:561)
  > **Verification signal:** no stripped name reaches the child, `process-start{subject:"claude-child"}` carries the strip counts and names only, on Unix a `claude_env_keep` name survives and the child holds no viola socket or home path, and the secret scan finds none of the canary values.
  (.andromeda/test-plan.md:603)
  > the panic seam fires only under the `fake-agent` feature and never reaches the release build.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:14) The MCP stdio witness and the PREREQ's line past `MAX_FRAME` before `initialize` must take this owned-child form, never `TokioChildProcess`, and a changed error body must keep the recorded-exchange replay's shape holding.
  > | ipc-internal (MCP stdio) | rmcp 3.4.1 client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit (never rmcp's `TokioChildProcess`, which kills the server 3 s after a close and left a truncated coverage profile, as measured at chunk 2026-10-17-mcp-server-for-drivers) + jsonschema | direct | Lives in `crates/viola-e2e` (`tests/mcp_tools.rs`), which takes rmcp feature `client` only, as a normal dependency (`viola-e2e` carries no `[dev-dependencies]`), no `transport-child-process`. Beside it, the recorded-exchange replay (`tests/mcp_recorded_exchange.rs`, `binary(mcp_recorded_exchange)`) writes each recorded `Mcp.*.json`'s client frames to a `viola mcp` child, takes the `tools/list` response by its request's id, and holds the served result to the recorded result's key and type shape, its tool names in order (its planted negatives read missing). Beside both, `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` spawns and owns a `viola mcp` child of its own and writes raw stdin lines from its own writer thread (an `initialize`, `notifications/initialized`, then one line past `MAX_FRAME`), its stdout and stderr drained on their own threads and its exit awaited under an 8 s kill deadline. |
  (.andromeda/test-plan.md:477)
  > **ipc-internal (MCP stdio):** an rmcp client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's `TokioChildProcess`; beside it, a replay of the recorded MCP exchange.
  (.andromeda/test-plan.md:603)
  > nextest runs the MCP schema-violation cases (each `invalid-params` with its field, no frame written)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:38) The Path 1 watch must be tallied on this chunk's CI reads against this test and its 20 s kill line, which stays where it stands.
  > - **As landed (chunk 2026-10-17-end-to-end-path-binary-and-harness-legs):** steps 1–6 run as one ordered flow over one stamped home in the root path binary `tests/path1_run_start.rs`, `path1_run_start_one_home_in_order(stamped_home)` (`[[test]]` `required-features = ["fake-agent"]`; in the E2E layer, `nextest-e2e`; on the `ci` profile's verify-driven 20 s kill line), every signal above asserted, the `settings.json` half Unix's, its start-refusal helpers shared with `tests/cli_instance_state.rs` through `tests/support/start.rs`. The flow lives on the root chain, not a `viola_e2e::fixtures` copy (fork 1, `inputs#I1`); its wall read 4.69 s (5.14 s instrumented) on the Linux dev host against the `mutants` profile's 10 s kill. The per-step root cases above stand beside it.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:47) The `send` refusal `not-delivered` / `turn-running` must stand in the enumeration of bodies with a closed code, and the MCP result must keep comparing equal to the CLI's document less `v` under whatever the error form becomes.
  > the MCP `send` step landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path2_path3_send_wait_last_match_the_cli` (a harness session, `binary(mcp_tools)`), its result compared with the CLI's `--json` document less `v`. Since chunk 2026-10-04-running-turn-refusal the test boots over `fixtures/fake-scripts/path3.json` and ends its first send's turn (the gated `PostToolUse` and `Stop` released, `turn-ended` awaited) before its second send, which a running turn would refuse `turn-running`; `send_after_a_confirmed_send_is_turn_running_until_turn_ended` covers the driver's own turn: the next `send` exits 13, `[/ ] unable         builder  not-delivered  turn-running` then `hint: a turn is running; viola wait builder first`, until `turn-ended`, then exits 0.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:62) These `send` refusal documents are bodies of the CLI surface that the enumeration must name, each holding a closed code and a code-shaped detail and nothing else.
  > - `mute`: exit 13, `{"refusal":"not-delivered","detail":"no-prompt-submitted"}`, and a `send-refused{refusal:"not-delivered",detail:"no-prompt-submitted"}` record.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:60)
  > a text whose typed text is empty, the empty text included, is refused at once `not-delivered`/`empty-text`, exit 13, and is never issued: `send_empty_text_is_refused_at_once` (`tests/cli_send.rs`, over the empty text, one LF and one CRLF: the struck mirror line, the hint `the text is empty once its trailing newlines are removed; send a text with content` as the last stderr line, no ESC byte, nothing on stdout; one document under `--json`; no `send-issued`; one client role-file `send-refused` line with `side` `client`) and `send_empty_text_straight_to_the_wrapper_is_refused` (`tests/channel_paste_validation.rs`).
  (.andromeda/test-plan.md:503)
  > an empty typed text is refused `empty-text`; a suppressed prompt is `not-delivered`/`no-prompt-submitted`;
- (.andromeda/test-plan-detail/e2e-test-strategy.md:75) The killed-wrapper case is a named check witnessing `instance-unreachable`, and each MCP `wait` / `last` result must keep comparing equal to the CLI's document less `v`.
  > the vanished-wrapper half is `tests/chaos_wait_vanish.rs` `chaos_wait_parked_wrapper_killed_exits_21`. The MCP `wait` / `last` steps landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path2_path3_send_wait_last_match_the_cli`, each result compared with the CLI's `--json` document less `v`; the MCP `wait` sends `timeout_ms` clamped to 50 000 ms, its default (`handlers_wait_sends_the_clamped_bound_and_maps_a_timeout`).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:84) This step is the drive behind the `instance-unreachable` body the scope names on the MCP surface, and must keep drawing that body.
  > 7. Start a new `wait` and kill the wrapper (the chaos helper); run rmcp `wait` on the dead instance.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:91) The enumeration and the MCP witness must hold exactly this body and text on both surfaces, code and null detail and nothing else.
  > - Step 7: CLI exit 21 with `{"error":"instance-unreachable","detail":null}`; MCP `is_error:true` with the same body, `{"error":"instance-unreachable","detail":null}` (the CLI document less `v`), text `refused: instance-unreachable`.
  (.andromeda/test-plan.md:511)
  > a killed wrapper gives CLI exit 21 and MCP `instance-unreachable`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:100) The `unverified-cli` refusal is a body of the MCP surface the scope's `refused: <code>` text rules over, and for the same refusal the MCP pair must equal the CLI's pair.
  > `path4_unstamped_permission_is_left_to_the_human_and_its_allow_refused` (exit 12 `unverified-cli`, no body). The MCP `answer` step landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path4_path5_answer_by_id_and_a_paused_send`, its result and refusal compared with the CLI's `--json` document less `v`.
  (.andromeda/test-plan.md:543)
  > and `answer` exits 12 `unverified-cli` (MCP alike);
  (.andromeda/test-plan.md:616)
  > for the same refusal, the MCP `structured_content` `{refusal, detail}` equals the CLI `--json` `{refusal, detail}`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:121) The `unknown-dialog` refusal document is a body of the CLI surface the enumeration must name.
  > - The unknown id gives exit 13 `{"refusal":"not-delivered","detail":"unknown-dialog"}`.
  (.andromeda/test-plan.md:519)
  > an unknown id exit 13 `unknown-dialog`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:132) The `human-typing` details and the `release-from-driver` cause are bodies the enumeration must name, and `release-from-driver` is one of the `-32602` causes the chunk's `channel-response` carry must tell apart without moving its exit 20.
  > `tests/cli_wheel.rs` (`pause` / `release`, the `human-typing` refusals with detail `null` | `manual-pause`, hints without `release`, `release-from-driver` exit 20)
  (.andromeda/test-plan-detail/e2e-test-strategy.md:133)
  > The MCP `send` step and its refusal checks landed at chunk 2026-10-17-mcp-server-for-drivers in `crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_path4_path5_answer_by_id_and_a_paused_send` (the refusal compared with the CLI's document less `v`; no MCP result, body or tool description names `release`).
  (.andromeda/test-plan.md:527)
  > a driver `send` exits 10 (`detail:null`, or `manual-pause` after `pause`); `release` returns the wheel and `release` with `from` exits 20; a harness-injected turn flips no wheel and refuses `send` `turn-running` until its `turn-ended`; driver-facing hints never suggest `viola release`.

## Bindings
- **The three surfaces' drivers ↔ the sanitization scan**: each per-surface witness drives its failure over its own row's driver, (.andromeda/test-plan-detail/e2e-test-strategy.md:10), (.andromeda/test-plan-detail/e2e-test-strategy.md:13) and (.andromeda/test-plan-detail/e2e-test-strategy.md:14), and what it reads back answers to the sanitization line (.andromeda/test-plan.md:605) and its §1 trigger (.andromeda/test-plan.md:269).
- **The refusal documents ↔ cross-surface parity**: the `--json` bodies the enumeration names (.andromeda/test-plan-detail/e2e-test-strategy.md:62), (.andromeda/test-plan-detail/e2e-test-strategy.md:121) and (.andromeda/test-plan-detail/e2e-test-strategy.md:91) are the same pairs the MCP surface returns, so a change to one side moves the comparison less `v` and the parity line (.andromeda/test-plan.md:616).
- **The error-body enumeration ↔ the bounded-reader enumeration**: the enumeration a new free-text body must trip has its nearest form in the table and census of (.andromeda/test-plan.md:622), whose owed rows (.andromeda/test-plan.md:623) this chunk also edits.
- **The MCP witness ↔ the recorded fixtures**: a changed error body that meets the replay (.andromeda/test-plan-detail/e2e-test-strategy.md:14) moves a recording only through the tap and its scrub (.andromeda/test-plan.md:649).
- **The pre-`initialize` PREREQ ↔ §4 viola-mcp**: the owned-child form (.andromeda/test-plan-detail/e2e-test-strategy.md:14) is the driver nearest the undriven clause that names this chunk its owner (.andromeda/test-plan.md:390).
- **`release-from-driver` ↔ obs-plan**: the `-32602` causes told apart on the `channel-response` line keep exit 20 (.andromeda/test-plan-detail/e2e-test-strategy.md:132), and the line's change is read against the `viola.compat.v_rejected` row (.andromeda/obs-plan-detail/metric-coverage.md:28).
