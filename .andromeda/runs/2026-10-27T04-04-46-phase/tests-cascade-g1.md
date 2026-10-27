# tests · cascade read · group 1 of 8

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:8) The chunk must hold the channel's 16 MiB `MAX_FRAME` cap on both the client and server frame readers, and witness it with same-user framing cases.
  > `MAX_FRAME` (16 MiB) and the result/refusal/error shapes
  (.andromeda/test-plan-detail/test-scope-summary.md:11)
  > same-user behaviour is testable: framing, method dispatch, refusal order, `-32602` on a higher `v`
  (.andromeda/test-plan.md:46)
  > same-user framing, dispatch, refusal order, server verification and the squatted-name refusal are testable
- (.andromeda/test-plan-detail/test-scope-summary.md:19) The hook payload parser, statusline parser and `claude agents --json` parser the chunk bounds must be witnessed against the fake agent and the recorded fixtures, because the real CLI's output is never reachable in CI.
  > hook parsing and dialog mapping (S3/S7/S8)
  (.andromeda/test-plan-detail/test-scope-summary.md:23)
  > screen signatures, statusline parsing, `claude agents --json` parsing
  (.andromeda/test-plan-detail/test-scope-summary.md:27)
  > parsers, mapping, normalisation, the env strip (using a stub child that dumps its env) and the version gate are testable against the fake agent and against `fixtures/claude/<cli-version>/`.
  (.andromeda/test-plan.md:193)
  > fail-open tests on oversize stdin, malformed JSON, clap error, channel failure, a forced panic
- (.andromeda/test-plan-detail/test-scope-summary.md:33) The chunk's CLI witnesses must assert the contract's integer exits: 13 for a `control-character` / `empty-text` refusal, 2 for a clap usage error such as a malformed revive `--id`, and 14 for an unknown refusal.
  > and its exits are 0, 1 for a refusal (the strict-modes check ahead of the log, or a preflight reading) and 2 for a clap usage error, a malformed `--id` included
  (.andromeda/test-plan.md:200)
  > path-traversal names (`../x`, uppercase, `/`) rejected by `parse_viola_name`
  (.andromeda/test-plan.md:202)
  > oversize `answer` JSON
  (.andromeda/test-plan.md:203)
  > out-of-range `Percent` in `config.json`
  (.andromeda/test-plan.md:261)
  > `RefusalReason` `unknown` fallback (exit 14)
- (.andromeda/test-plan-detail/test-scope-summary.md:43) The chunk's client-side refusals (CLI `send`'s paste rule, CLI positional names, oversize `answer` JSON) must be driven as a subprocess with a per-test `--home` and `--json`, and judged by exit code plus one JSON document.
  > subprocess + `assert_cmd` / `Command::cargo_bin("viola")`, with `--home <per-test temp>` and `--json`
  (.andromeda/test-plan-detail/test-scope-summary.md:44)
  > exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":…}`).
  (.andromeda/test-plan.md:94)
  > its signal is the exit code plus one JSON document on stdout
- (.andromeda/test-plan-detail/test-scope-summary.md:56) The re-verified vt100 panic degrade must be witnessed only through the `input-not-ready` refusal and the continuing passthrough, never by reading the child's screen.
  > The vt100 readiness gate is tested through its refusal outputs (`input-not-ready`), not by scraping the screen.
  (.andromeda/test-plan.md:227)
  > forced vt100 panic → `not-delivered`/`input-not-ready` with passthrough continuing
  (.andromeda/test-plan.md:238)
  > vt100 feed under `catch_unwind`
  (.andromeda/test-plan.md:899)
  > NEVER parse the rendered child screen (the vt100 grid, PTY master output) for content.
- (.andromeda/test-plan-detail/test-scope-summary.md:59) The oversize frame case must be sent by a raw `interprocess` client, and its verdict must be the `-32600` protocol fault followed by a close. Raw-channel names must be refused `-32602` with no line written.
  > (the `viola-channel` sync client, or a raw `interprocess` `local_socket` client for malformed frames)
  (.andromeda/test-plan-detail/test-scope-summary.md:60)
  > Protocol faults are `{"error":{code,message,data}}` with codes -32700 / -32600 / -32601 / -32602 (`data:{supported, wrapper}`) / -32603 (`"internal error"`, `data:null`).
  (.andromeda/test-plan-detail/test-scope-summary.md:63)
  > a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven` is `-32602` with `data: null` and no line written
  (.andromeda/test-plan.md:181)
  > frame over `MAX_FRAME` → `-32600` + close
- (.andromeda/test-plan-detail/test-scope-summary.md:67) The chunk's MCP witnesses must send inputs that violate the advertised schema, and each must get `invalid-params` with its closed field word and no frame written.
  > `{"error":"invalid-params","detail":"<field>"}` (the closed field words `target · text · after · timeout_ms · dialog_id · response`, no frame written), each text `refused: <code>`.
  (.andromeda/test-plan-detail/test-scope-summary.md:69)
  > Schemas are advisory, so negative inputs must be sent that violate the advertised schema.
  (.andromeda/test-plan.md:55)
  > server-side re-validation of `target`, `dialog_id`, `response` and `text`
  (.andromeda/test-plan.md:196)
  > schema-violating tool calls (non-`ViolaName` `target`, non-`u64` `dialog_id`, open-string `behavior`, control chars in `text`) are rejected by the handler. `isError` payloads carry codes only.
- (.andromeda/test-plan-detail/test-scope-summary.md:111) The oversize and malformed `claude agents --json` reads must come from a stub on a test-scoped PATH, any `budget.json` must be seeded through `viola hook statusline`, and the property generators must cover the parser surfaces the chunk bounds.
  > `budget.json` is seeded by piping synthetic statusline stdin into `viola hook statusline`, and `claude agents --json` by a stub on PATH.
  (.andromeda/test-plan-detail/test-scope-summary.md:112)
  > Property / fuzz generators cover the parser surfaces the security excerpt lists.
  (.andromeda/test-plan.md:224)
  > oversize / malformed `claude agents --json` → `unknown`
  (.andromeda/test-plan.md:234)
  > `validate_paste_text` (decoded chars vs raw bytes)
  > - channel ndjson framing at the `MAX_FRAME` boundary
  > - the hook stdin parser incl. `hook statusline`
  > - the `claude agents --json` parser
- (.andromeda/test-plan-detail/test-scope-summary.md:123) The bounded reads the `run` start makes (its snapshot for the collision check, its config, stamps and statusline sources) must leave the start sequence in order. The Path 1 test that witnesses that order is tallied on this chunk's settled CI reads, and no retry is the fix.
  > the collision check (over a replayed snapshot, liveness is the beat alone), pinned copy and plugin folder, the ConPTY sideload (Windows x64, fail-open: any failure leaves the child on the inbox ConPTY), version gate
  (.andromeda/test-plan-detail/test-scope-summary.md:126)
  > `events.ndjson` order is `wheel{cause:"start"}` → `budget-gate` → the fake agent's `session-start{source:"hook"}`
  (.andromeda/test-plan.md:135)
  > a second `run` of the same name, a squatted endpoint, a tampered pinned exe or a refused home tree exits 1.
  (.andromeda/test-plan.md:951)
  > NEVER add retry-once policies or any retry budget for flaky tests (§10 Zero-flakiness budget).
- (.andromeda/test-plan-detail/test-scope-summary.md:131) The paste rule must be re-verified at the client before any frame and in the wrapper: `control-character` comes first, then `empty-text` for an empty typed text, and no character is ever stripped.
  > A text whose typed text is empty is refused `not-delivered`/`empty-text` (exit 13) right after `control-character` and before anything is issued, by the client before any frame and by the wrapper.
  (.andromeda/test-plan.md:39)
  > `ViolaName`, `Percent` (0–100), `v` constants, `validate_paste_text`
  (.andromeda/test-plan.md:190)
  > negative tests showing ESC, other C0, DEL and C1 (U+0080–U+009F) in `send.text` and free-text answers are rejected (never stripped) as `not-delivered`/`control-character` before other refusals, at both the client and the wrapper. LF / CR / TAB and multi-byte UTF-8 are accepted.
- (.andromeda/test-plan-detail/test-scope-summary.md:160) The capped `budget.json` read and the capped status line stdout must fail as this path's signal states: a malformed value is read as `"unknown"`, and an oversize or failed stdout prints empty with exit 0.
  > A malformed `resets_at` becomes `"unknown"`.
  (.andromeda/test-plan-detail/test-scope-summary.md:161)
  > Statusline stdout equals the user command's stdout (empty on failure), exit 0.
  (.andromeda/test-plan.md:159)
  > `viola hook statusline` writes `rate_limits` to `budget.json` (only when the payload holds a `rate_limits` object)
- (.andromeda/test-plan-detail/test-scope-summary.md:174) The over-long ndjson line must be counted in `torn_lines` with nothing rewritten, and a snapshot read that fails its cap or its parse must fall to the replay's recovered fields.
  > the reader counts it in `torn_lines` and rewrites nothing, and the next append heals it and logs one `state-recovered` line
  (.andromeda/test-plan-detail/test-scope-summary.md:175)
  > corrupt `snapshot.json` or give it an unsupported `v`: the replay recovers only `links`, `agent_session_id`, `wheel`, `budget_paused`, `budget_override_until`, with `dialog_pending:false`.
  (.andromeda/test-plan.md:216)
  > over-long ndjson line counted in `skipped.torn_lines`
  (.andromeda/test-plan.md:260)
  > unknown event kinds / fields counted in `skipped` rather than failing
- (.andromeda/test-plan.md:908) A unit case that plants an oversize or malformed `budget.json`, `snapshot.json` or `ledger/stamps.json` must write it through the product's writer and then pad, replace or re-version those bytes, and no booted-wrapper test may hand-write one.
  > NEVER hand-write `ledger/stamps.json`, `snapshot.json` or `budget.json`. Stamps come from `viola verify`, snapshots from the wrapper and budget from `viola hook statusline`. The ban binds every booted-wrapper test. A reader's own unit cases may plant a malformed state file only by altering the writer's own bytes: write through the writer, then pad, replace or re-version
- (.andromeda/test-plan.md:871) The scope asks for a unit-level witness per property and per boundary, so each parser, cap and refusal-order case belongs in the unit layer, and E2E asserts only cross-process behaviour.
  > NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).

## Bindings
- **scope-surfaces (tui) ↔ e2e chaos suite**: the readiness gate's `input-not-ready` signal is witnessed by the chaos suite's forced vt100 panic, which refuses `send` while passthrough continues. See (.andromeda/test-plan-detail/test-scope-summary.md:56) and (.andromeda/test-plan.md:616).
- **scope-surfaces (MCP) ↔ e2e security sweep**: the closed `invalid-params` field words are what the sweep's MCP schema-violation cases assert, each with no frame written. See (.andromeda/test-plan-detail/test-scope-summary.md:67) and (.andromeda/test-plan.md:601).
- **scope-entities (parsers) ↔ e2e property suite**: the hook stdin, statusline and `claude agents --json` parsers named in scope run in the property suite at `cases: 512`, and ndjson framing runs there at `MAX_FRAME` ± 1. See (.andromeda/test-plan-detail/test-scope-summary.md:23) and (.andromeda/test-plan.md:618).
- **scope-surfaces (ipc-internal) ↔ e2e security control negatives**: the raw-channel `link` / `unlink` name refusals are held in `tests/channel_links.rs`. See (.andromeda/test-plan-detail/test-scope-summary.md:63) and (.andromeda/test-plan.md:609).
- **paths (run start) ↔ quality gates (zero flakiness)**: the Path 1 WATCH is tallied against an unchanged kill line, and no retry budget may close it. See (.andromeda/test-plan-detail/test-scope-summary.md:126) and (.andromeda/test-plan.md:928).
- **data ↔ unit**: an oversize or malformed state file is planted only through the product's writer and then altered. See (.andromeda/test-plan-detail/test-scope-summary.md:111) and (.andromeda/test-plan.md:908).
