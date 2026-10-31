# tests · E2E Test Strategy — category read

## Carried
- (.andromeda/test-plan-detail/e2e-test-strategy.md:259) The chunk's proof must hold this after-every-E2E check on the three surfaces it names, its absolute-path set, its `-32603` exact body and its 0600 rule read as written.
  > - **Error sanitization and secret scan:** after every E2E test, the `events` support helper checks:
  >   - CLI `--json`, MCP `isError` and channel `error.data` contain no absolute path (drive letter, `/home/`, `/Users/`, the temp home prefix), no serde path and no `Caused by`
  >   - every -32603 is exactly `"internal error"` / `null`
  >   - every `diagnostics/*` file and log contains no `Cookie` header and no stripped-var values
  >   - `diagnostics/` files are 0600 on Unix
- (.andromeda/test-plan-detail/e2e-test-strategy.md:264) Any input the chunk's witnesses feed must be synthetic and embed the fixed canary, and a scan hit must report place and class, never the matched bytes.
  > - **Canary** (obs-plan §8, High-class verification): every integration and E2E input is synthetic, and every prompt, send text, tool `input`, dialog answer and statusline payload the tests feed embeds one fixed canary string, a constant in `tests/support`. The scan asserts that the canary never appears in any home-level `<home>/diagnostics/*.ndjson` line. It may appear in `events.ndjson` (contract payloads) and in `instances/<name>/diagnostics/detail-*.ndjson` only. The same scan runs in CI over `target/e2e-home/**`, the harness capture in `target/agent-run/` except `run --mutants`' `target/agent-run/chunk.diff` (repository source text, excluded from the scan and from the `harness-<os>` upload), and `target/nextest/ci/junit.xml` before any upload (obs-plan §9 step order). A hit is reported as file, line, byte offset and pattern class, never the matched bytes.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:213) The planted stripped `CLAUDE*` value in the proof must take this canary form and register in this table, so the secret scan can see it.
  > Values are secret-scan canaries (`canary-<kind>-value-<hex4>`, one per name, registered in the `claude-stripped` table), including `CLAUDE_CODE_MESSAGING_TOKEN` and `_SOCKET`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:219) The floor the proof leans on must stay this: no planted value in any log or diagnostics file.
  > - The secret-scan test finds none of the random values in any log or `diagnostics/` file.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:10) The CLI `--json` witness must drive the verbs through this driver.
  > assert_cmd 2.2.2 + predicates 3.1.4
- (.andromeda/test-plan-detail/e2e-test-strategy.md:13) The channel witness must use this driver and must read the endpoint only from the snapshot.
  > viola-channel sync client + interprocess 2.4.4 raw stream + jsonschema 0.57.0 | direct | The endpoint is read only from `snapshot.json`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:14) An MCP witness over stdio must own its `viola mcp` child this way, never through `TokioChildProcess`.
  > rmcp 3.4.1 client over a `viola mcp` child the test spawns and owns, closing its stdin and awaiting its exit (never rmcp's `TokioChildProcess`, which kills the server 3 s after a close and left a truncated coverage profile, as measured at chunk 2026-10-17-mcp-server-for-drivers) + jsonschema
- (.andromeda/test-plan-detail/e2e-test-strategy.md:14) An MCP E2E witness must live in this crate under this manifest rule, with no dev-dependency added.
  > Lives in `crates/viola-e2e` (`tests/mcp_tools.rs`), which takes rmcp feature `client` only, as a normal dependency (`viola-e2e` carries no `[dev-dependencies]`), no `transport-child-process`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:14) The folded PREREQ case, a stdin line past `MAX_FRAME` before `initialize` completes, must drive raw stdin with this form: its own writer thread, drained streams and a kill deadline.
  > Beside both, `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` spawns and owns a `viola mcp` child of its own and writes raw stdin lines from its own writer thread (an `initialize`, `notifications/initialized`, then one line past `MAX_FRAME`), its stdout and stderr drained on their own threads and its exit awaited under an 8 s kill deadline.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:247) The after-`initialize` clause must stand unchanged beside the new pre-`initialize` case, whose sibling this is.
  > The MCP stdin line past `MAX_FRAME`: `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line` (`crates/viola-e2e/tests/mcp_tools.rs`), from a fresh home of its own, writes an `initialize`, `notifications/initialized` and then one line past `MAX_FRAME` with stdin left open: exit 0, stderr empty, stdout only the `initialize` response, exactly one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` line in `mcp.ndjson`, valid against `schemas/diag-line.v1.json`, and the line's canary in no file under the home (green on the three CI OSes, as measured at ci#40458923095).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:91) This exit-21 pair is a body the enumeration must name, the MCP body equal to the CLI document less `v`.
  > - Step 7: CLI exit 21 with `{"error":"instance-unreachable","detail":null}`; MCP `is_error:true` with the same body, `{"error":"instance-unreachable","detail":null}` (the CLI document less `v`), text `refused: instance-unreachable`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:150) A `wrapper-fault` document carries an object `detail`; the chunk's "code-shaped or null `detail`" must be reconciled with this body in one place, not left in conflict.
  > - `release` with `from` gives exit 20, `{"error":"wrapper-fault","detail":{"code":-32602,…}}` carrying `release-from-driver`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:184) This MCP refusal body must keep its shape and its equality with the CLI document.
  > `isError:true`, `{"refusal":"budget-paused","detail":"five-hour"}` with no cursor, equal to the CLI's document less `v`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:201) The refusal text on MCP must keep the `refused: <code>` form this signal asserts.
  > - `answer` exits 12 `{"refusal":"unverified-cli","detail":null}`. MCP gives `is_error:true` with text `refused: unverified-cli`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:246) An `invalid-params` result names its field, so "codes only" on MCP must keep that field and the enumeration must name it.
  > each violation `invalid-params` with its field and no `send-issued`; a control-character or empty text `not-delivered`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:278) This is the one document every exit-21 cause prints, and the enumeration must hold it as one body.
  > its `--json` document is every exit-21 cause's `{"v":1,"error":"instance-unreachable","detail":null}`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:288) The enumeration of CLI `--json` errors must name these `revive` documents, each with its code and a null detail, and leave their human pair unchanged.
  > The `--json` documents landed with chunk 2026-10-17-cli-machine-contract as `tests/cli_revive.rs` cases, one per code: `revive_json_on_a_live_name_is_the_already_live_document`, `revive_json_of_a_name_never_started_is_the_no_session_document`, `revive_json_with_an_id_the_log_does_not_hold_is_the_no_session_document`, `revive_json_of_a_session_open_in_another_claude_is_the_session_live_document`, `revive_json_of_a_snapshot_with_no_cwd_is_the_cwd_missing_document` and `revive_json_refuses_state_another_user_could_write_on_both_arms`, each exit 1 with the `{"v":1,"error":"<code>","detail":null}` document and empty stderr, the human pair unchanged without `--json`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:289) The chunk must leave this owed document and the rstest table to their own route entry; the boundary is the plan's as well as the scope's.
  > A refusal `run`'s start makes after a passed preflight (`batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`) prints no document under `revive --json` and no case asserts one. Joining the rstest table, and that document, are owed to the route entry "Exit-cause code catalogue".
- (.andromeda/test-plan-detail/e2e-test-strategy.md:306) The channel witness must hold this body of `-32602`, and may write frames past the CLI this way.
  > writes its frames through `viola_channel::Client::request`, past every CLI check and the server check. For each of `link` and `unlink`, a `driver` or a `driven` of `../x`, `BUILDER`, `a/b`, an empty string, a 33-character name or a number, a `driven` that is not the serving wrapper's own name, and a `driver` equal to `driven` each get `-32602` with `data: null`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:313) This standing negative already bars a pid, start time, path or endpoint in any output; the chunk must keep it green and its bar unweakened.
  > exit 21 with the server-verification cause's `--json` document (stderr empty), hint and `process-exit` line (Exit-cause matrix), no `channel-request` line, no frame reaching the server, and no pid, start time, path or endpoint in any output or line (`digit_runs`).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:371) The owed cap-driving cases must be named by their `BOUNDARIES` rows, each row's witness a defined test, the tables literal and never the product's own list.
  > `tests/contract_bounded_readers.rs` (a `contract_` binary in the E2E layer, `run --e2e`) holds two literal tables, never the product's own list: `BOUNDARIES`, one row per external reader naming its bounded read, its cap, depth and type rule and the existing test that witnesses it, and `EXCEPTIONS`, the reads of viola's own executable (`checked_exe`, `pin_exe`), no external input. `boundaries_each_name_a_bounded_read_that_stands` holds each row's read to its file and its witness to a defined test
- (.andromeda/test-plan-detail/e2e-test-strategy.md:38) The chunk's CI reads must tally the Path 1 watch against this kill line, and the line must not move.
  > on the `ci` profile's verify-driven 20 s kill line
- (.andromeda/test-plan.md:269) This is the trigger the chunk's proof answers, on all its named surfaces and contents.
  > - **Required test type:** assertions that CLI `--json`, MCP `isError` and channel `error.data` contain no absolute paths, serde paths or values, anyhow chains or tool `input`. `-32603` must be exactly `"internal error"` / `data:null`. A grep of all logs and `diagnostics/` must show no `Cookie` or stripped `CLAUDE*` values. `diagnostics/` must be 0600.
- (.andromeda/test-plan.md:196) MCP `isError` payloads must carry codes only, which the MCP witness must assert.
  > `isError` payloads carry codes only.
- (.andromeda/test-plan.md:181) The channel's `-32602` on a higher `v` carries `data.supported`/`wrapper`; the chunk's "`error.data` a code or null" must be reconciled with this required negative, and the `-32602` causes must stay told apart as these lines tell them.
  >     - frame over `MAX_FRAME` → `-32600` + close
  >     - higher `params.v` → `-32602` with `data.supported`/`wrapper`
  >     - `release` with `from` → `release-from-driver`
- (.andromeda/test-plan.md:251) A change to any refusal body must keep MCP and CLI identical for the same channel result.
  >     - MCP and CLI return identical `ok` / refusal payloads for the same channel result
- (.andromeda/test-plan.md:616) The parity check below must hold for every refusal the chunk touches.
  >   - for the same refusal, the MCP `structured_content` `{refusal, detail}` equals the CLI `--json` `{refusal, detail}`
- (.andromeda/test-plan.md:390) The chunk owns this undriven clause as a PREREQ and must drive it.
  >   - not driven by any test: a stdin line past `MAX_FRAME` landing before `initialize` completes ends `serve` with `ServeError::Initialize` (exit 11), as a closed stdin before `initialize` does; owner: the route entry "Sanitised error surfaces", as a PREREQ
- (.andromeda/test-plan.md:623) The chunk owns these rows as a PREREQ and must land one cap-driving case for each.
  >   - Owed: the enumeration's rows for the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` name a witness that reads the bounded read without driving past its cap, so their caps are held by the row's needle and the census alone; owner: the route entry "Sanitised error surfaces", as a PREREQ (a cap-driving case for each row).
- (.andromeda/test-plan.md:607) A change to a diagnostics line, such as the folded `channel-response` CARRY, must stay valid under this check, which reports without the line's content.
  > - **Schema conformance** (the check body behind obs-plan gate G4; obs owns the schemas, tests own the check): after the test steps, every home-level `diagnostics/*.ndjson` line under `target/e2e-home/**` is validated with jsonschema 0.57.0 against obs-plan's `schemas/diag-line.v1.json`, and every `instances/*/diagnostics/detail-*.ndjson` line against `schemas/diag-detail.v1.json`.
- (.andromeda/test-plan.md:433) Each witness's home must sit where the scans reach it, or its clean scan proves nothing.
  > a fresh tempfile home per test, at a not-yet-existing path under `target/e2e-home` that viola creates, so obs-plan's CI gates, the secret scan and the upload cover integration runs
- (.andromeda/test-plan.md:454) A witness's absolute-path oracle must compare the resolved home and must drop the inherited instance variables.
  > - A test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, and a path oracle compares `resolved(home)`, never `TestHome::path()`.
- (.andromeda/test-plan.md:324) A new binary must take the prefix of its layer, so the nextest filtersets select it.
  >   - Rust integration binaries use a `<surface>_<topic>.rs` prefix so nextest filtersets can select layers: `cli_`, `hook_`, `tui_`, `channel_`, `chaos_`, `contract_` in root; `path_`, `mcp_`, `cross_` in `viola-e2e`.
- (.andromeda/test-plan.md:874) A case a lower layer can hold, such as a single `Display` message, must sit below E2E; the witness per surface asserts the cross-process body.
  > - NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:887) The enumeration of codes and bodies must be written as literals in the test, never imported from the product.
  > - NEVER import the product's own `IDENTITY_FLOOR`, ledger rows or refusal ordering as the test oracle. The expected values (the 11 identity-floor names, the refusal order, exit codes) are written as literals in the test.
- (.andromeda/test-plan.md:896) The planted `CLAUDE*` value must be set per child, and a witness that clears the environment must keep its coverage profile.
  > - NEVER use `std::env::set_var` in tests. It is `unsafe` in edition 2024 and races across threads; use `Command::env` per child.
  > - NEVER call `.env_clear()` without re-adding `LLVM_PROFILE_FILE`, because that run silently drops out of coverage.
- (.andromeda/test-plan.md:901) A witness that drives a failure over a wrapper must wait on an explicit signal, never a sleep.
  > - NEVER use `sleep(N)` for synchronization. Wait for an explicit signal: an `events.ndjson` offset, `viola wait --after` or a status field.

## Bindings
- **After-every-E2E check ↔ the body's broad line and the security excerpt**: the check at (.andromeda/test-plan-detail/e2e-test-strategy.md:259) details the body's trigger at (.andromeda/test-plan.md:269), whose source is the security excerpt's Error-sanitization expectations; all three must read the same set of barred contents.
- **Canary and scan ↔ obs-plan §8 and §9**: the canary rule and the scan's CI step order at (.andromeda/test-plan-detail/e2e-test-strategy.md:264) bind with obs-plan's High-class verification and its scan-gated uploads; both sides must move together if the scan's reach changes.
- **`channel-response` CARRY ↔ obs-plan metric coverage**: a line telling the `-32602` causes apart must pass schema conformance at (.andromeda/test-plan.md:607) against obs-plan's `schemas/diag-line.v1.json`, and the causes it tells apart are those at (.andromeda/test-plan.md:181); obs owns the schema, tests own the check.
- **`error.data` a code or null ↔ the channel's required negatives**: the scope's channel clause meets `data.supported`/`wrapper` at (.andromeda/test-plan.md:182) and `wrapper-fault`'s object `detail` at (.andromeda/test-plan-detail/e2e-test-strategy.md:150); the architecture's error contract and this plan must state one reading.
- **PREREQ pre-`initialize` case ↔ §4 viola-mcp**: the owed clause at (.andromeda/test-plan.md:390) is driven in the form at (.andromeda/test-plan-detail/e2e-test-strategy.md:14); landing it retires the owner line in §4.
- **PREREQ cap-driving cases ↔ §6 Contract suite**: the owed rows at (.andromeda/test-plan.md:623) change the witness column of the table at (.andromeda/test-plan-detail/e2e-test-strategy.md:371); landing them retires the Owed line.
- **`revive` documents ↔ route entry "Exit-cause code catalogue"**: the enumeration names what stands at (.andromeda/test-plan-detail/e2e-test-strategy.md:288), and the document owed at (.andromeda/test-plan-detail/e2e-test-strategy.md:289) stays with that entry.
- **MCP bodies ↔ cross-surface parity**: the bodies at (.andromeda/test-plan-detail/e2e-test-strategy.md:91) and (.andromeda/test-plan-detail/e2e-test-strategy.md:184) bind with the parity rule at (.andromeda/test-plan.md:616).
