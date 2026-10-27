# tests · cascade read · group 6 of 8

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:246) The MCP `send` / tool-argument boundary must keep its schema-violation witnesses, each violation `invalid-params` with no frame and a control-character or empty text `not-delivered`, as the chunk re-verifies the MCP client side of the paste rule and the closed MCP tool arguments.
  > The MCP schema-violating calls live in `crates/viola-mcp/tests/handlers.rs` `handlers_schema_violations_write_no_frame` (no frame reaches the test wrapper), the `crates/viola-mcp/src/tools.rs` unit `tool_inputs_are_revalidated_before_any_frame` and the E2E `mcp_tools_list_is_exactly_the_five_tools_and_list_matches_the_cli` (each violation `invalid-params` with its field and no `send-issued`; a control-character or empty text `not-delivered`) — no `mcp_schema_violations.rs` binary (chunk 2026-10-17-mcp-server-for-drivers) — and the hook fail-open matrix in root `hook_fail_open.rs` (assert_cmd).
  (.andromeda/test-plan.md:601)
  > - **Security sweep:** nextest runs the MCP schema-violation cases (each `invalid-params` with its field, no frame written) and the hook fail-open matrix — oversize stdin, malformed JSON, clap error, channel failure, a forced panic,
- (.andromeda/test-plan-detail/e2e-test-strategy.md:248) The hook stdin cap must be witnessed by the fail-open matrix's oversize-stdin and malformed-JSON rows, each exit 0 with empty streams and no body inside the `Instant` bound.
  > The hook fail-open matrix cases are oversize stdin, malformed JSON, clap error, channel failure, forced panic, missing `VIOLA_NAME`,
  (.andromeda/test-plan.md:469)
  > - **cli (`viola hook` / `hook statusline`):** assert_cmd with an `Instant` deadline over recorded fixture stdin;
- (.andromeda/test-plan-detail/e2e-test-strategy.md:304) Every channel `driver` / `driven` name must be validated by the wrapper before any path join or record, witnessed by frames written straight to the endpoint past every CLI check.
  > **Raw-channel `link` / `unlink` name validation** (channel frames). Frames written straight to the endpoint with `driver` / `driven` set to `../x`, `BUILDER`, `a/b`, an empty string and a 33-character name each get `-32602`, and no `link` / `unlink` record is appended.
  > - As landed: `channel_link_names_are_validated_by_the_wrapper` (`tests/channel_links.rs`, `binary(channel_links)` on the `ci` profile's 20 s kill line) writes its frames through `viola_channel::Client::request`, past every CLI check and the server check. For each of `link` and `unlink`, a `driver` or a `driven` of `../x`, `BUILDER`, `a/b`, an empty string, a 33-character name or a number, a `driven` that is not the serving wrapper's own name, and a `driver` equal to `driven` each get `-32602` with `data: null`, with no `link` / `unlink` record and the snapshot's `links` empty; its control, a valid `link`, answers `already_linked: false` and appends one `link` record.
  (.andromeda/test-plan.md:609)
  > raw-channel `link` / `unlink` name validation (`tests/channel_links.rs`)
- (.andromeda/test-plan-detail/e2e-test-strategy.md:294) A revive `--id` must pass its closed session-id shape before it reaches any lookup or argv, a malformed one refused as clap usage exit 2.
  > a malformed `--id` is the clap usage exit 2
- (.andromeda/test-plan-detail/e2e-test-strategy.md:336) A forced vt100 panic must refuse `send` exit 13 `input-not-ready` while the human passthrough still reaches the receipt, the degrade the chunk re-verifies at its `catch_unwind` sites.
  > forced vt100 panic mode: `send` exits 13 with `input-not-ready`, and human passthrough still reaches the receipt
  (.andromeda/test-plan.md:616)
  > a forced vt100 panic refusing `send` `input-not-ready` while passthrough continues
- (.andromeda/test-plan-detail/e2e-test-strategy.md:332) The `events.ndjson` reader must count a torn or over-long line in `skipped.torn_lines` and never panic, the count the chunk's filesystem witness reads.
  > torn append: `skipped.torn_lines` goes up by 1, and the next append starts on a fresh line.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:337) An unknown event kind or field read from the log must be skipped and counted, never passed into item state as a free string, the closed-type rule the chunk re-verifies for event kinds.
  > forward-compat lines (`chaos_forward_compat.rs`, multi-version-compat trigger). Append two lines to a product-written `events.ndjson`, under the §7 exception for files the product already wrote: one with an unknown `kind`, and one known-kind line with an extra unknown field. Then:
  > - `list --json` exits 0
  > - `skipped.unknown_kinds` and `skipped.unknown_fields` each rise by exactly 1
  (.andromeda/test-plan.md:616)
  > forward-compat lines counted in `skipped`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:350) The per-boundary property witnesses must cover the paste rule, the frame codec at `MAX_FRAME` ± 1, the hook stdin, statusline and `claude agents --json` parsers and the vt100 feed under `catch_unwind`, each at `cases: 512`.
  > - **Property suite** (proptest 1.11.0, `cases: 512`):
  > - `validate_paste_text` over `any::<String>()` plus injected C0/C1 chars (landed at `cases: 512`, chunk 2026-10-04-confirmed-send-with-cl-1-records)
  > - ndjson framing at `MAX_FRAME` ± 1
  > - the hook stdin parser (`viola_agent_claude::hook::normalise` over arbitrary bytes; landed at `cases: 512` with `crates/viola-agent-claude/proptest-regressions/hook.txt` committed); the statusline reader (`viola_agent_claude::statusline::reading` over arbitrary JSON and `resets_at`; landed at `cases: 512` with `crates/viola-agent-claude/proptest-regressions/statusline.txt` committed, chunk 2026-10-10-statusline-pass-through; its strategy draws whole seconds across the date range, because as first written it stayed green on a stub)
  > - the `claude agents --json` parser over arbitrary JSON (landed at `cases: 512`, the `agents.rs` `proptest!`, chunk 2026-10-16-unwrapped-sessions-on-the-board, with `crates/viola-agent-claude/proptest-regressions/agents.txt` committed; its oracle compares a row against the input re-parsed from its bytes, because a float does not round-trip through serde_json text)
  > - the vt100 feed under `catch_unwind` over arbitrary bytes (landed at `cases: 512`: `screen::tests::feed_prop_a_caught_panic_poisons_until_resize`, sizes 1–30 × 1–120 with interleaved resizes; after a caught panic the poisoned model reads `input-not-ready` and drops further bytes)
  (.andromeda/test-plan.md:618)
  > - **Property suite** (proptest, `cases: 512`): `validate_paste_text`, ndjson framing at `MAX_FRAME` ± 1, the hook stdin and statusline parsers, the `claude agents --json` parser, the vt100 feed under `catch_unwind` and `prompt-submitted` normalisation; cargo-fuzz targets in the separate `fuzz/` workspace join as their parsers land.
  (.andromeda/test-plan.md:344)
  > `agents_mode` (`recorded` · `oversize` · `malformed`, serde default `recorded`, by chunk 2026-10-17-end-to-end-path-binary-and-harness-legs) is written by `boot` and stands in `supervise.json` too.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:358) A parser the chunk bounds that is not yet a fuzz target joins the separate `fuzz/` workspace only as its parser lands, the landed targets (`channel_frame`, `hook_stdin`, `vt100_feed`, `paste_text`, `claude_agents_json`) kept with their synthetic seeds.
  > The six parser targets join as their parsers land: `channel_frame` (7 synthetic seeds), `hook_stdin` (the `viola hook` stdin parser and, since chunk 2026-10-10-statusline-pass-through, the statusline reader; 12 synthetic seeds by `ls fuzz/corpus/hook_stdin | wc -l`, the two added `statusline-epoch` and `statusline-absent`) and `vt100_feed` (the vt100 feed under `catch_unwind`, 6 synthetic seeds: three measured panic sizes and three 24×80 lines) and `paste_text` (`validate_paste_text` against an independent per-char oracle over lossy UTF-8, 8 synthetic seeds, kept byte-exact by `.gitattributes` `fuzz/corpus/paste_text/** binary`) and `claude_agents_json` (`agents::parse` over arbitrary bytes, chunk 2026-10-16-unwrapped-sessions-on-the-board; 6 synthetic seeds, `background-row`, `control-session-id`, `deep-array`, `empty-array`, `measured-row` and `non-array`, kept byte-exact by `.gitattributes` `fuzz/corpus/claude_agents_json/** binary`) have joined.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:367) The closed decision types read from hook payloads must stay closed enums under a tolerant schema, the fixture check that witnesses them.
  > jsonschema checks every fixture against the tolerant parser schema: unknown fields allowed, closed enums required.
  (.andromeda/test-plan.md:620)
  > every fixture is checked against the tolerant parser schema
- (.andromeda/test-plan-detail/e2e-test-strategy.md:161) The reader enumeration must name both statusline settings sources and the home rule that chooses between them, each read capped.
  > The settings source is named: `<home>/statusline-source.json` (the Claude settings shape; viola only reads it) whenever it exists, else in the default home the user's `.claude/settings.json`, and in any other home none.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:170) The status line command's run must stay bounded and its stdout capped, and `budget.json` written only from a payload holding a `rate_limits` object.
  > the arm runs the recorded command as `/bin/sh -c <command>` under a 5 s bound (PROVISIONAL). On Windows `run` writes no override and the arm runs no command (`override_document` and `shell_argv` return `None` for every input). Exit 0 with empty stderr holds on every OS. `budget.json` is written only when the payload holds a `rate_limits` object.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:173) A malformed `resets_at` in the statusline payload must be read as the closed `unknown`, never refused or passed through.
  > The malformed value gives `resets_at:"unknown"` in `budget.json`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:238) The reader enumeration must name every product reader of `snapshot.json`, each behind its cap.
  > The replay has six product readers: `viola revive`, `viola list`'s `board::read`, hook `connect`, `hook statusline`'s `serve`, `run`'s `collision_check` and `viola_state::liveness::live_endpoint`; `read_snapshot` has no product caller under `src/`,

## Bindings
- **e2e property / fuzz ↔ the chaos suite**: the vt100 feed property's poisoned model reading `input-not-ready` (.andromeda/test-plan-detail/e2e-test-strategy.md:355) and the chaos forced-panic `send` refusal with passthrough (.andromeda/test-plan-detail/e2e-test-strategy.md:336) witness one degrade at the same `catch_unwind` sites; both must hold.
- **e2e security sweep ↔ unit-test strategy**: the MCP E2E and handler cases (.andromeda/test-plan-detail/e2e-test-strategy.md:246) sit beside the `crates/viola-mcp/src/tools.rs` unit `tool_inputs_are_revalidated_before_any_frame` named in the same run; the client-side paste refusal is witnessed at both layers.
- **e2e property ↔ the §3 harness contract**: the `claude agents --json` parser property (.andromeda/test-plan-detail/e2e-test-strategy.md:354) covers the parse, and the harness `agents_mode` `oversize` · `malformed` (.andromeda/test-plan.md:344) drives the oversize and malformed reads end to end.
- **e2e fuzz ↔ test data & fixtures**: each fuzz corpus seed is synthetic and kept byte-exact by `.gitattributes` (.andromeda/test-plan-detail/e2e-test-strategy.md:358).
