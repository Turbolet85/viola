# tests · cascade read · group 5 of 8

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:11) The hook stdin cap's end-to-end witnesses (oversize stdin and malformed JSON failing open) must run through assert_cmd with an `Instant` deadline over fixture stdin, inside the fail-open matrix.
  > | cli (`viola hook` / `hook statusline`) | assert_cmd 2.2.2 with `Instant` deadline | direct | Fixture stdin from `fixtures/claude/<ver>/`. `.env_clear()` must re-add `LLVM_PROFILE_FILE`. |
  (.andromeda/test-plan.md:469)
  > - **cli (`viola hook` / `hook statusline`):** assert_cmd with an `Instant` deadline over recorded fixture stdin; `.env_clear()` must re-add `LLVM_PROFILE_FILE`.
  (.andromeda/test-plan.md:601)
  > the hook fail-open matrix — oversize stdin, malformed JSON, clap error, channel failure, a forced panic, a missing `VIOLA_NAME`, an instance the instance check refuses or a `VIOLA_DIR` naming no instance directory and, on Linux and Windows, a server that is not the recorded wrapper, each exit 0 with no body
  (.andromeda/test-plan.md:618)
  > the hook stdin and statusline parsers, the `claude agents --json` parser
- (.andromeda/test-plan-detail/e2e-test-strategy.md:12) The re-verified vt100 panic degrade must show passthrough continuing through the outer PTY, exiting only via `child.wait()` with the master reader drained on its own thread, its verdict taken from `events.ndjson`, the receipt and the exit status.
  > | tui (`viola run` passthrough) | portable-pty `=0.8.1` outer PTY (`tests/support/outer_pty.rs` over the `viola-pty` seam) | direct (ConPTY / openpty) | Exit only via `child.wait()`, and the master reader is drained on its own thread. Verdicts come from `events.ndjson`, the fake-agent receipt and the exit status. |
  (.andromeda/test-plan.md:471)
  > - **tui (`viola run` passthrough):** a portable-pty outer PTY over the `viola-pty` seam; exit only via `child.wait()`, the master reader drained on its own thread, verdicts from `events.ndjson`, the receipt and the exit status.
  (.andromeda/test-plan.md:616)
  > a forced vt100 panic refusing `send` `input-not-ready` while passthrough continues
  (.andromeda/test-plan.md:618)
  > the vt100 feed under `catch_unwind`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:13) The over-`MAX_FRAME` frame (`-32600` and a close), the wrapper-side `control-character` / `empty-text` refusal and raw-channel `link` / `unlink` name validation must be driven through the sync client or a raw interprocess stream, with the endpoint read only from `snapshot.json`.
  > | ipc-internal (wrapper channel) | viola-channel sync client + interprocess 2.4.4 raw stream + jsonschema 0.57.0 | direct | The endpoint is read only from `snapshot.json`. |
  (.andromeda/test-plan.md:473)
  > - **ipc-internal (wrapper channel):** the sync client, a raw interprocess stream and jsonschema; the endpoint is read only from `snapshot.json`.
  (.andromeda/test-plan.md:609)
  > raw-channel `link` / `unlink` name validation (`tests/channel_links.rs`)
  (.andromeda/test-plan.md:618)
  > ndjson framing at `MAX_FRAME` ± 1
- (.andromeda/test-plan-detail/e2e-test-strategy.md:14) Any end-to-end witness of the `mcp` stdin cap or of MCP `send`'s client-side paste refusal must drive a `viola mcp` child the test spawns and owns (stdin closed, exit awaited), from `crates/viola-e2e` with rmcp `client` only, never `TokioChildProcess` or `transport-child-process`.
  > | ipc-internal (MCP stdio) | rmcp 3.4.1 client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit (never rmcp's `TokioChildProcess`, which kills the server 3 s after a close and left a truncated coverage profile, as measured at chunk 2026-10-17-mcp-server-for-drivers) + jsonschema | direct | Lives in `crates/viola-e2e` (`tests/mcp_tools.rs`), which takes rmcp feature `client` only, as a normal dependency (`viola-e2e` carries no `[dev-dependencies]`), no `transport-child-process`.
  (.andromeda/test-plan.md:475)
  > - **ipc-internal (MCP stdio):** an rmcp client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit, never rmcp's `TokioChildProcess`; beside it, a replay of the recorded MCP exchange.
  (.andromeda/test-plan.md:601)
  > nextest runs the MCP schema-violation cases (each `invalid-params` with its field, no frame written)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:38) The Path 1 WATCH must be tallied on this one flow's settled CI reads against the `ci` profile's unchanged 20 s kill line, its recorded wall reading the baseline.
  > - **As landed (chunk 2026-10-17-end-to-end-path-binary-and-harness-legs):** steps 1–6 run as one ordered flow over one stamped home in the root path binary `tests/path1_run_start.rs`, `path1_run_start_one_home_in_order(stamped_home)` (`[[test]]` `required-features = ["fake-agent"]`; in the E2E layer, `nextest-e2e`; on the `ci` profile's verify-driven 20 s kill line), every signal above asserted, the `settings.json` half Unix's, its start-refusal helpers shared with `tests/cli_instance_state.rs` through `tests/support/start.rs`. The flow lives on the root chain, not a `viola_e2e::fixtures` copy (fork 1, `inputs#I1`); its wall read 4.69 s (5.14 s instrumented) on the Linux dev host against the `mutants` profile's 10 s kill. The per-step root cases above stand beside it.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:24)
  > 1. rstest `stamped_home` fixture. Spawn `viola run builder --home H -- <viola-fake-agent> …` in an outer PTY, then wait for readiness (the `boot` checks) by tailing `events.ndjson`.
  (.andromeda/test-plan-detail/e2e-test-strategy.md:31)
  > The first three `events.ndjson` records are `wheel{cause:"start"}`, `budget-gate`, `session-start{source:"hook", data.cause:"startup"}`, in that order.
  (.andromeda/test-plan.md:493)
  > - **Verification signal:** the first three records are `wheel{cause:"start"}`, `budget-gate` and the hook's `session-start`, the child spawned only after them;
- (.andromeda/test-plan-detail/e2e-test-strategy.md:60) The re-verified paste rule must keep the empty typed text refused `not-delivered` / `empty-text`, exit 13, never issued, both at the client and straight to the wrapper, and inner CR / CR LF typed as one LF, never stripped beyond the `typed_text` rule.
  > Since chunk 2026-10-09-epoch-3-cleanup a text whose typed text is empty, the empty text included, is refused at once `not-delivered`/`empty-text`, exit 13, and is never issued: `send_empty_text_is_refused_at_once` (`tests/cli_send.rs`, over the empty text, one LF and one CRLF: the struck mirror line, the hint `the text is empty once its trailing newlines are removed; send a text with content` as the last stderr line, no ESC byte, nothing on stdout; one document under `--json`; no `send-issued`; one client role-file `send-refused` line with `side` `client`) and `send_empty_text_straight_to_the_wrapper_is_refused` (`tests/channel_paste_validation.rs`).
  (.andromeda/test-plan-detail/e2e-test-strategy.md:60)
  > Since chunk 2026-10-09-inner-cr-and-crlf-in-a-sent-text a CR or a CR LF inside a text is typed as one LF, so the send is confirmed and the driver keeps the wheel:
  (.andromeda/test-plan-detail/e2e-test-strategy.md:50)
  > 2. Record L = byte length of `instances/builder/events.ndjson`. Run `viola send builder --json --home H` with stdin `"line one\nline two"` and env `VIOLA_NAME=overseer`.
  (.andromeda/test-plan.md:501)
  > an empty typed text is refused `empty-text`
  (.andromeda/test-plan.md:618)
  > `validate_paste_text`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:75) A Stop's `background_tasks` must reach `turn-ended` only as the array's length or `null`, its recorded-fixture witness standing and its non-zero and `null` counts held at the unit tier.
  > A Stop's `turn-ended` data carries `background_tasks`, the Stop array's length or `null`: `tests/hook_events.rs` `hook_a_recorded_stop_carries_its_background_task_count` reads it over the recorded 2.1.287 `Stop.default.json` (an empty array, so 0), with `state_events_background_tasks_and_option_details_are_known_keys` (§5 On-disk); a non-zero and a `null` count are held at the unit tier (§4 viola-agent-claude).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:101) Each question's `option_details` must keep `description` / `preview` only as a string or `null`, its recorded-fixture witness standing and non-null previews held at the unit tier.
  > Each question's `option_details` (one `{description, preview}` per option, a string or `null`) is read by `tests/hook_events.rs` `hook_a_recorded_question_carries_its_option_details` over the recorded 2.1.287 question (it holds no `preview`, so the previews read `null`) and by the `state_events` case of §5 On-disk; non-null previews are held at the unit tier (§4 viola-agent-claude).

## Bindings
- **e2e drivers ↔ integration and security-sweep suites**: the hook fail-open matrix's oversize and malformed stdin cases ride the assert_cmd `Instant`-deadline driver (.andromeda/test-plan-detail/e2e-test-strategy.md:11), and the wrapper-side paste refusal and raw-channel name validation ride the raw-stream driver (.andromeda/test-plan-detail/e2e-test-strategy.md:13), as the body's sweep lists them (.andromeda/test-plan.md:601) (.andromeda/test-plan.md:609).
- **e2e chaos ↔ property suite (unit tier)**: the forced vt100 panic's passthrough witness through the outer PTY (.andromeda/test-plan-detail/e2e-test-strategy.md:12) pairs with the vt100 feed's `catch_unwind` property case (.andromeda/test-plan.md:618), both re-verified by the chunk.
- **e2e paste-rule cases ↔ unit tier**: the client and wrapper `empty-text` cases (.andromeda/test-plan-detail/e2e-test-strategy.md:60) stand beside `validate_paste_text`'s property over decoded chars (.andromeda/test-plan.md:618); the client side and the wrapper side are each witnessed, the wrapper authoritatively.
- **e2e hook payload bounds ↔ unit tier and §5 On-disk**: the recorded-fixture witnesses of `background_tasks` and `option_details` (.andromeda/test-plan-detail/e2e-test-strategy.md:75) (.andromeda/test-plan-detail/e2e-test-strategy.md:101) defer their non-zero, `null` and non-null cases to §4 viola-agent-claude and their known-key case to §5.
- **Path 1 WATCH ↔ quality gates and CI**: the tally reads `path1_run_start_one_home_in_order` against the `ci` profile's 20 s kill line (.andromeda/test-plan-detail/e2e-test-strategy.md:38), the line unmoved and no retry under zero flakiness, the `stamped_home` fixture its dominant measured input (.andromeda/test-plan-detail/e2e-test-strategy.md:24).
