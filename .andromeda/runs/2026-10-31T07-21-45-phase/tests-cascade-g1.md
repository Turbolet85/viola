# tests · cascade read · group 1 of 7

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:8) Every error shape of the channel crate the chunk edits must hold a fixed message and a code or null, across the framing and `-32602` cases this entity names, with the MCP frame tools' `instance-unreachable` witness still standing.
  > `viola-channel`: JSON-RPC 2.0 over ndjson on `interprocess` local sockets. Covers the sync client/server, the Tokio client feature, the FNV-1a endpoint hash, `MAX_FRAME` (16 MiB) and the result/refusal/error shapes
  (.andromeda/test-plan-detail/test-scope-summary.md:11)
  > same-user behaviour is testable: framing, method dispatch, refusal order, `-32602` on a higher `v`, the pid/start-time server verification, and `run` exiting 1 on a squatted name.
  (.andromeda/test-plan-detail/test-scope-summary.md:12)
  > the MCP frame tools by `crates/viola-mcp/tests/handlers.rs` (`instance-unreachable`, no frame)
  (.andromeda/test-plan.md:262)
  >     - higher `v` rejected with `-32602`
- (.andromeda/test-plan-detail/test-scope-summary.md:18) The `Refusal` carry must leave the npm-shim, `.cmd` / `.bat` and parser behaviour of this crate unchanged whichever way it is answered, the planted stripped value must sit under a name the R8 strip removes, and the child pipe drain's oversize case must read `unknown`.
  > `viola-agent-claude`, covering:
  >   - hook parsing and dialog mapping (S3/S7/S8)
  >   - the R8 `CLAUDE*` strip
  >   - npm-shim → `claude.exe` resolution
  >   - the version gate, the capability ledger and the `verify` probes
  >   - screen signatures, statusline parsing, `claude agents --json` parsing
  (.andromeda/test-plan-detail/test-scope-summary.md:27)
  > parsers, mapping, normalisation, the env strip (using a stub child that dumps its env) and the version gate are testable against the fake agent and against `fixtures/claude/<cli-version>/`.
  (.andromeda/test-plan.md:71)
  > inherited credentials strip (`CLAUDE_CODE_MESSAGING_TOKEN` / `_SOCKET` and every other inherited `CLAUDE*` name under the R8 prefix rule, except persistent-environment names — Windows registry `Environment` value names, Unix `config.json` `claude_env_keep` — and always the 11-name identity floor
  (.andromeda/test-plan.md:74)
  > a stub child that reports its env can prove the strip happened.
  (.andromeda/test-plan.md:126)
  > Logs must never contain the `Cookie` header or stripped `CLAUDE*` values (security excerpt, Error-sanitization).
  (.andromeda/test-plan.md:222)
  > `.cmd` / `.bat` PTY child → `run` exit 1 (windows runner)
  >     - npm-shim fixture resolves to `claude.exe`
  >     - oversize / malformed `claude agents --json` → `unknown`
- (.andromeda/test-plan-detail/test-scope-summary.md:33) Every `--json` error document of every verb must hold a closed code and a code-shaped or null `detail`, these documents standing in the enumeration, with `run`'s start refusals left to the entry that owns them.
  > CLI verbs and the `--json` / exit-code contract (`run · revive · send · wait · last · list · answer · pause · release · link · unlink · verify · plugin install`, global `--home`, exits 0/1/2/10–14/20/21; `revive` takes `--json` on both arms (chunk 2026-10-17-cli-machine-contract: an exit-1 refusal of its strict-modes check or its preflight prints `{"v":1,"error":"<code>","detail":null}` on stdout, `<code>` one of `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`; `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order, an empty chain the `no-session` document; a refusal `run`'s start makes after a passed preflight prints no document, owed to "Exit-cause code catalogue")
  (.andromeda/test-plan-detail/test-scope-summary.md:36)
  > a refusal exits 21 with the instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, and writes no line (§5 On-disk); a `driver` equal to `driven` is refused by the wrapper alone, never client-side (CLI exit 20).
  (.andromeda/test-plan.md:39)
  > `RefusalReason` (kebab-case, `#[serde(other)] unknown`)
  (.andromeda/test-plan.md:261)
  >     - `RefusalReason` `unknown` fallback (exit 14)
  (.andromeda/test-plan.md:269)
  > assertions that CLI `--json`, MCP `isError` and channel `error.data` contain no absolute paths, serde paths or values, anyhow chains or tool `input`. `-32603` must be exactly `"internal error"` / `data:null`. A grep of all logs and `diagnostics/` must show no `Cookie` or stripped `CLAUDE*` values. `diagnostics/` must be 0600.
- (.andromeda/test-plan-detail/test-scope-summary.md:44) The CLI witness must read the exit code and the one JSON document through this driver, and the enumeration must name each of these document forms.
  > exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":…}`).
  (.andromeda/test-plan-detail/test-scope-summary.md:43)
  > subprocess + `assert_cmd` / `Command::cargo_bin("viola")`, with `--home <per-test temp>` and `--json`
  (.andromeda/test-plan-detail/test-scope-summary.md:42)
  > under `--json` the exit-1 `{"v":1,"error":"<code>","detail":null}` document (its five codes) or `--list`'s `{"v":1,"ok":{"sessions":[…]}}`)
- (.andromeda/test-plan-detail/test-scope-summary.md:60) Every channel error the chunk enumerates must keep this shape with `error.data` a code or null and `-32603` exactly this body, driven by this surface's raw client and schema assertion, including the link and unlink causes of `-32602` the telling-apart must not mislabel.
  > Success is `{"result":{"ok":…}}`. Refusal is `{"result":{"refusal":…,"detail":…}}` with the first-applicable ordering. Protocol faults are `{"error":{code,message,data}}` with codes -32700 / -32600 / -32601 / -32602 (`data:{supported, wrapper}`) / -32603 (`"internal error"`, `data:null`). This is cross-checked against the `events.ndjson` byte-offset `cursor`.
  (.andromeda/test-plan-detail/test-scope-summary.md:59)
  > language-native IPC test client (the `viola-channel` sync client, or a raw `interprocess` `local_socket` client for malformed frames) + JSON schema assertion
  (.andromeda/test-plan-detail/test-scope-summary.md:63)
  > a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven` is `-32602` with `data: null` and no line written, and a failed `record` / `remove` is `-32603`
  (.andromeda/test-plan.md:181)
  > frame over `MAX_FRAME` → `-32600` + close
  >     - higher `params.v` → `-32602` with `data.supported`/`wrapper`
  >     - `release` with `from` → `release-from-driver`
  >     - server pid / start-time mismatch vs `snapshot.json` → CLI exit 21 / MCP `instance-unreachable` / hook exit 0 with no body
- (.andromeda/test-plan-detail/test-scope-summary.md:67) Every `isError` result of the five tools must carry only these coded bodies with text `refused: <code>`, a handler fault exactly this `-32603`, and a changed error body that meets the recorded exchange must be read against its replay, while the negative inputs still violate the advertised schema.
  > MCP tool result. `isError:false` + `structuredContent` equal to the `ok` payload. `isError:true` + `{refusal, detail}` and text `"refused: <reason>"`. `isError:true` + `{"error":"instance-unreachable","detail":null}` | `{"error":"wrapper-fault","detail":{code,message,data}}` | `{"error":"invalid-params","detail":"<field>"}` (the closed field words `target · text · after · timeout_ms · dialog_id · response`, no frame written), each text `refused: <code>`. An internal handler fault (any other channel error, a reply of no known shape, a `wait` / `last` refusal, a panicked call) is the JSON-RPC error `-32603` `internal error`, `data: null`; an unknown tool `-32602`. `tools/list` is exactly the five tools, its result carrying `ttlMs` 0 and `cacheScope` `private` (chunk 2026-10-17-mcp-server-for-drivers). A recorded live exchange (`fixtures/claude/<cli-version>/Mcp.<variant>.json`) is replayed against `viola mcp` by `mcp_recorded_exchange`, the served `tools/list` result keeping the recorded result's key and type shape, those two keys included, and its tool names in order.
  (.andromeda/test-plan-detail/test-scope-summary.md:66)
  > language-native MCP stdio test client. It spawns `viola mcp --home <tmp>` and speaks JSON-RPC over stdin/stdout, with JSON schema assertion.
  (.andromeda/test-plan-detail/test-scope-summary.md:69)
  > Schemas are advisory, so negative inputs must be sent that violate the advertised schema.
  (.andromeda/test-plan-detail/test-scope-summary.md:70)
  > the frame tools `send`, `wait`, `last` and `answer` run `check_instance` before the liveness pre-check; a refused instance returns `isError:true` `{"error":"instance-unreachable","detail":null}` with no frame and no line (`crates/viola-mcp/tests/handlers.rs` `handlers_frame_tools_refuse_a_widened_instance_before_any_frame`).
  (.andromeda/test-plan.md:55)
  > `isError` mapping, and server-side re-validation of `target`, `dialog_id`, `response` and `text`
  (.andromeda/test-plan.md:196)
  > `isError` payloads carry codes only.
  (.andromeda/test-plan.md:251)
  >     - MCP and CLI return identical `ok` / refusal payloads for the same channel result
- (.andromeda/test-plan-detail/test-scope-summary.md:71) The owed case must drive a line past the cap landing before `initialize` completes to `ServeError::Initialize` (exit 11), beside this landed clause for an initialized session, which must keep holding.
  > `viola mcp` reads its stdin through a line reader capped at `MAX_FRAME` (`CappedLines`); a line past the cap passes no byte, is never truncated or skipped, and ends the session as a stdin close does, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` WARN line holding no byte of it; on an initialized session the earlier lines are answered, and `viola mcp` exits 0 with stderr empty and stdout only the earlier frames (`crates/viola-e2e/tests/mcp_tools.rs` `mcp_tools_a_line_past_max_frame_ends_the_session_with_one_line`).
  (.andromeda/test-plan.md:196)
  > A `viola mcp` stdin line past `MAX_FRAME` is refused whole and ends the session: the earlier lines answered, exit 0, stderr empty, stdout only the earlier frames, and one `parse-rejected` `mcp-stdin` / `oversize` WARN line holding no byte of it.
- (.andromeda/test-plan-detail/test-scope-summary.md:112) The status-line witness must feed `viola hook statusline` synthetic stdin, and the child pipe drain witness must drive past its cap through a stub on PATH that also answers `--version` as the real CLI does, and a recording a changed error body meets must stay free of absolute paths and usernames.
  > `budget.json` is seeded by piping synthetic statusline stdin into `viola hook statusline`, and `claude agents --json` by a stub on PATH.
  (.andromeda/test-plan-detail/test-scope-summary.md:111)
  > The fake agent is the child (`viola run <name> -- <fake agent>`; it answers `--version` like the real CLI).
  (.andromeda/test-plan-detail/test-scope-summary.md:115)
  > Fixtures must not come from real prompts and must be checked for absolute paths and usernames
- (.andromeda/test-plan-detail/test-scope-summary.md:127) The chunk's CI reads must tally `path1_run_start_one_home_in_order`, the check that witnesses this start order and these exits, as the watch states it.
  > `events.ndjson` order is `wheel{cause:"start"}` → `budget-gate` → the fake agent's `session-start{source:"hook"}` (landed with "Hooks to normalised events").
  (.andromeda/test-plan-detail/test-scope-summary.md:124)
  > A second `run` of the same name, or a squatted endpoint, exits 1.
- (.andromeda/test-plan-detail/test-scope-summary.md:132) The enumeration and the cross-surface parity check must read these closed refusal details as code-shaped bodies, the send flow itself staying as it is.
  > If no match arrives in the window, the result is `not-delivered`/`no-prompt-submitted` and `send-refused` is logged.
  (.andromeda/test-plan-detail/test-scope-summary.md:132)
  > A text whose typed text is empty is refused `not-delivered`/`empty-text` (exit 13) right after `control-character` and before anything is issued, by the client before any frame and by the wrapper.
  (.andromeda/test-plan-detail/test-scope-summary.md:139)
  > With the fake agent suppressing UserPromptSubmit, the CLI exits 13 with detail `no-prompt-submitted`, `send-refused` is logged.
- (.andromeda/test-plan-detail/test-scope-summary.md:154) The `-32602` carry must tell this cause apart from the unsupported-version and `InvalidParams` causes without changing the exit or the code, and the enumeration must read these refusal bodies.
  > `release` with `from` gives `-32602` `release-from-driver` (exit 20).
  (.andromeda/test-plan-detail/test-scope-summary.md:151)
  > `send --json` exits 10 with detail `null`. After `pause` the detail is `manual-pause`.
  (.andromeda/test-plan-detail/test-scope-summary.md:152)
  > MCP `send` gives `isError:true` `"refused: human-typing"`.
  (.andromeda/test-plan.md:183)
  > `release` with `from` → `release-from-driver`
- (.andromeda/test-plan-detail/test-scope-summary.md:158) The owed cap-driving cases must assert what the status-line command's oversize stdout and the oversize thresholds file fall back to against this pass-through and these thresholds, the pass-through being Unix only and never run on a failed strict-modes check, and the enumeration must read the window words as code-shaped details.
  > `viola hook statusline` writes `rate_limits` to `budget.json` (only when the payload holds a `rate_limits` object) and passes the user's statusline stdout through unchanged (on Unix only: on Windows `run` writes no `settings.json` override and the arm runs no command). Crossing `five_hour` ≥ 90 % or `seven_day` ≥ 85 % refuses automated `send` with `budget-paused`, and `release --budget` overrides it.
  (.andromeda/test-plan-detail/test-scope-summary.md:162)
  > Statusline stdout equals the user command's stdout (empty on failure), exit 0. The equality is Unix only; on Windows the arm runs no command and prints nothing. Over a replayed snapshot (unreadable or of an unsupported `v`) the arm finds no `statusline_command`, so it runs and prints nothing (unit `statusline_over_a_replayed_snapshot_runs_nothing`).
  (.andromeda/test-plan-detail/test-scope-summary.md:164)
  > `send` exits 11 with detail `five-hour` / `seven-day`.
  (.andromeda/test-plan.md:58)
  > before passing the statusline through (the pass-through on Unix only; on Windows the arm runs no command)
  (.andromeda/test-plan.md:193)
  > `statusline_command` must not run when the home fails strict-modes.
  (.andromeda/test-plan.md:203)
  >     - out-of-range `Percent` in `config.json`
- (.andromeda/test-plan-detail/test-scope-summary.md:178) The `instance-unreachable` body on a vanished endpoint or killed wrapper must stay true across the chunk's sanitising of the MCP and CLI surfaces.
  > endpoint vanishes during `wait`: `instance-unreachable`
  (.andromeda/test-plan.md:145)
  >     - Killing the wrapper mid-wait gives exit 21 / MCP `instance-unreachable`.
- (.andromeda/test-plan.md:920) The surface witnesses that drive a handler fault to `-32603` must force it through process or file faults or an existing mockall seam, never through a failpoint crate.
  > NEVER introduce a failpoint crate that pulls tokio into the sync crates (for example fail-parallel). Fault seams go through mockall doubles of existing traits plus process and file faults.
  (.andromeda/test-plan.md:955)
  > chaos faults come only from explicit process / file operations or existing mockall seams (§2 Property-based and Chaos rows).

## Bindings
- **Surface error bodies ↔ E2E suites**: the CLI, MCP and channel bodies named here are what the error sanitization and secret scan suite (.andromeda/test-plan.md:605) reads after every E2E test, and the MCP and CLI refusal payloads are what the cross-surface parity row (.andromeda/test-plan.md:616) holds equal.
- **Sanitisation trigger ↔ unit strategy**: the `-32603` body of (.andromeda/test-plan.md:269) rests on the JSON-RPC error mapping unit cases of (.andromeda/test-plan.md:380), and the enumeration must agree with both.
- **MCP stdin cap ↔ unit strategy**: the landed clause of (.andromeda/test-plan-detail/test-scope-summary.md:71) stands beside the owed before-`initialize` clause that names this entry its owner at (.andromeda/test-plan.md:390).
- **Budget and child pipe clauses ↔ Contract suite and seed strategies**: the cap-driving cases of (.andromeda/test-plan-detail/test-scope-summary.md:158) and (.andromeda/test-plan-detail/test-scope-summary.md:112) answer the owed rows at (.andromeda/test-plan.md:623) and must seed as (.andromeda/test-plan.md:643) and (.andromeda/test-plan.md:645) state.
- **Planted stripped value ↔ scenario E2**: the name kept or stripped under (.andromeda/test-plan.md:71) decides whether a planted value proves anything, and the secret scan of (.andromeda/test-plan.md:561) is the reading beside it.
- **`-32602` causes ↔ obs-plan**: the causes at (.andromeda/test-plan-detail/test-scope-summary.md:60), (.andromeda/test-plan-detail/test-scope-summary.md:63) and (.andromeda/test-plan-detail/test-scope-summary.md:154) must be told apart on the `channel-response` line so that the obs-plan `viola.compat.v_rejected` metric counts only the unsupported-version cause, and a changed line validates under (.andromeda/test-plan.md:607).
- **Fault seams ↔ mocking discipline**: the rule of (.andromeda/test-plan.md:920) binds with the process-mocking row at (.andromeda/test-plan.md:728) over how the `-32603` witness forces its fault.
