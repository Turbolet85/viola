# tests · Test Scope Summary — category read

## Carried
- (.andromeda/test-plan-detail/test-scope-summary.md:27) The planted stripped `CLAUDE*` value must be witnessed against the fake agent with a stub child reporting its env, and never against the real CLI in CI.
  > the env strip (using a stub child that dumps its env)
- (.andromeda/test-plan-detail/test-scope-summary.md:33) The CLI witness must hold `revive`'s five exit-1 codes with a null `detail`, while the document a post-preflight start refusal owes stays with "Exit-cause code catalogue" and outside this chunk.
  > `revive` takes `--json` on both arms (chunk 2026-10-17-cli-machine-contract: an exit-1 refusal of its strict-modes check or its preflight prints `{"v":1,"error":"<code>","detail":null}` on stdout, `<code>` one of `strict-modes-failed`, `already-live`, `no-session`, `session-live`, `cwd-missing`; `--list --json` prints `{"v":1,"ok":{"sessions":[…]}}` in log order, an empty chain the `no-session` document; a refusal `run`'s start makes after a passed preflight prints no document, owed to "Exit-cause code catalogue")
- (.andromeda/test-plan-detail/test-scope-summary.md:36) The enumerated CLI error bodies must keep the instance refusal exactly this document, with no line written.
  > a refusal exits 21 with the instance pair, or under `--json` exactly `{"v":1,"error":"instance-unreachable","detail":null}`, and writes no line (§5 On-disk)
- (.andromeda/test-plan-detail/test-scope-summary.md:43) The CLI surface's witness must drive the verb as a subprocess with a per-test home and `--json`.
  > **Driver:** subprocess + `assert_cmd` / `Command::cargo_bin("viola")`, with `--home <per-test temp>` and `--json`
- (.andromeda/test-plan-detail/test-scope-summary.md:44) The CLI witness must read its verdict from the exit code and the one JSON document on stdout, in these three shapes.
  > **Signal:** exit code (0, 1, 2, 10–14, 20, 21) plus one JSON document on stdout (`{"v":1,"ok":…}` / `{"v":1,"refusal":…,"detail":…}` / `{"v":1,"error":…}`).
- (.andromeda/test-plan-detail/test-scope-summary.md:59) The channel surface's witness must use the language-native client, and a raw local-socket client for malformed frames, with schema assertion.
  > **Driver:** language-native IPC test client (the `viola-channel` sync client, or a raw `interprocess` `local_socket` client for malformed frames) + JSON schema assertion
- (.andromeda/test-plan-detail/test-scope-summary.md:60) Every channel protocol fault must hold this code set and shape, `-32603` exactly `"internal error"` / `data:null`, and the `-32602` `data` object is a value the chunk must match or have amended against the scope's "a code or null".
  > Protocol faults are `{"error":{code,message,data}}` with codes -32700 / -32600 / -32601 / -32602 (`data:{supported, wrapper}`) / -32603 (`"internal error"`, `data:null`).
- (.andromeda/test-plan-detail/test-scope-summary.md:63) The channel error enumeration must include the `link` / `unlink` faults at these codes, `data: null` and no line written on the `-32602` causes.
  > both are served by the driven instance's wrapper; a missing, non-string or invalid name, a `driven` that is not the serving wrapper's own name, or a `driver` equal to `driven` is `-32602` with `data: null` and no line written, and a failed `record` / `remove` is `-32603`
- (.andromeda/test-plan-detail/test-scope-summary.md:66) The MCP surface's witness must spawn `viola mcp` with a temp home and speak JSON-RPC with schema assertion.
  > **Driver:** language-native MCP stdio test client. It spawns `viola mcp --home <tmp>` and speaks JSON-RPC over stdin/stdout, with JSON schema assertion.
- (.andromeda/test-plan-detail/test-scope-summary.md:67) The MCP error bodies the chunk enumerates must be these and no other, with the closed field words and a handler fault exactly `-32603`; `wrapper-fault`'s `detail:{code,message,data}` must carry no free text, since its `message` and `data` are the wrapper's.
  > `isError:true` + `{refusal, detail}` and text `"refused: <reason>"`. `isError:true` + `{"error":"instance-unreachable","detail":null}` | `{"error":"wrapper-fault","detail":{code,message,data}}` | `{"error":"invalid-params","detail":"<field>"}` (the closed field words `target · text · after · timeout_ms · dialog_id · response`, no frame written), each text `refused: <code>`. An internal handler fault (any other channel error, a reply of no known shape, a `wait` / `last` refusal, a panicked call) is the JSON-RPC error `-32603` `internal error`, `data: null`; an unknown tool `-32602`.
- (.andromeda/test-plan-detail/test-scope-summary.md:69) The MCP witness's negative inputs must violate the advertised schema, not stay inside it.
  > Schemas are advisory, so negative inputs must be sent that violate the advertised schema.
- (.andromeda/test-plan-detail/test-scope-summary.md:70) The MCP frame tools must keep this code-only refusal ahead of any frame or line.
  > a refused instance returns `isError:true` `{"error":"instance-unreachable","detail":null}` with no frame and no line
- (.andromeda/test-plan-detail/test-scope-summary.md:71) The PREREQ's pre-`initialize` case sits beside this initialized-session rule, whose WARN line must hold no byte of the refused line.
  > a line past the cap passes no byte, is never truncated or skipped, and ends the session as a stdin close does, with one `parse-rejected{parser:"mcp-stdin", detail:"oversize", count:1}` WARN line holding no byte of it; on an initialized session the earlier lines are answered, and `viola mcp` exits 0 with stderr empty and stdout only the earlier frames
- (.andromeda/test-plan-detail/test-scope-summary.md:110) Each witness must bootstrap its own state in a fresh home.
  > Self-bootstrapping per test: a fresh `--home` temp dir.
- (.andromeda/test-plan-detail/test-scope-summary.md:118) A witness that makes its home ahead of viola, such as one planting a path or a value, must make it private through the product's call.
  > A test that makes a home before viola runs makes it through `viola_state::fs::create_private_dir`, never `create_dir_all`
- (.andromeda/test-plan-detail/test-scope-summary.md:154) This `-32602` cause must remain distinguishable from the unsupported-version and invalid-params causes in the channel's error enumeration, as the folded `fault_of` freight requires.
  > `release` with `from` gives `-32602` `release-from-driver` (exit 20).
- (.andromeda/test-plan.md:24) The chunk's test breadth must be held at this tier.
  > **Test tier:** `Comprehensive (2)`
- (.andromeda/test-plan.md:71) The planted value must be a name under the R8 prefix rule that the strip removes, not a persistent-environment name or an identity-floor name.
  > inherited credentials strip (`CLAUDE_CODE_MESSAGING_TOKEN` / `_SOCKET` and every other inherited `CLAUDE*` name under the R8 prefix rule, except persistent-environment names
- (.andromeda/test-plan.md:126) The floor's log half must hold wherever a witness reads the logs.
  > Logs must never contain the `Cookie` header or stripped `CLAUDE*` values (security excerpt, Error-sanitization).
- (.andromeda/test-plan.md:182) The higher-`v` negative must keep passing as the plan names it, whatever the chunk rules on channel error data.
  > higher `params.v` → `-32602` with `data.supported`/`wrapper`
- (.andromeda/test-plan.md:196) The MCP trigger requires code-only `isError` payloads from schema-violating calls.
  > `isError` payloads carry codes only.
- (.andromeda/test-plan.md:251) The sanitised refusal bodies must stay identical across MCP and CLI for one channel result.
  > MCP and CLI return identical `ok` / refusal payloads for the same channel result
- (.andromeda/test-plan.md:269) This is the chunk's governing trigger: the assertions must cover all three surfaces, the exact `-32603`, the log and diagnostics grep and the 0600 mode.
  > **Required test type:** assertions that CLI `--json`, MCP `isError` and channel `error.data` contain no absolute paths, serde paths or values, anyhow chains or tool `input`. `-32603` must be exactly `"internal error"` / `data:null`. A grep of all logs and `diagnostics/` must show no `Cookie` or stripped `CLAUDE*` values. `diagnostics/` must be 0600.
- (.andromeda/test-plan.md:272) The chunk must add no mutation run to its default harness `run`, pre-push or CI.
  > It is not part of a chunk's default harness `run`, the pre-push or CI.
- (.andromeda/test-plan.md:390) The chunk owns this PREREQ and must drive the case with a test, retiring the owed line when it lands.
  > not driven by any test: a stdin line past `MAX_FRAME` landing before `initialize` completes ends `serve` with `ServeError::Initialize` (exit 11), as a closed stdin before `initialize` does; owner: the route entry "Sanitised error surfaces", as a PREREQ
- (.andromeda/test-plan.md:433) Integration-level witnesses must make their homes here so the secret scan covers them, which the `v1-47` scan leg reads.
  > a fresh tempfile home per test, at a not-yet-existing path under `target/e2e-home` that viola creates, so obs-plan's CI gates, the secret scan and the upload cover integration runs
- (.andromeda/test-plan.md:454) A witness that plants or compares a path must use the resolved home as its oracle and drop the viola variables.
  > A test-started `viola` drops `VIOLA_NAME`, `VIOLA_DIR` and `VIOLA_BIN`, and a path oracle compares `resolved(home)`, never `TestHome::path()`.
- (.andromeda/test-plan.md:605) The E2E suite must hold this after every E2E test, and a hit must report place and class, never the matched bytes.
  > **Error sanitization and secret scan:** after every E2E test, no absolute path, serde path or `Caused by` in any error payload, every `-32603` exactly `"internal error"` / `null`, no `Cookie` header or stripped-variable value in any log, `diagnostics/` files 0600 on Unix, and the fixed canary in no home-level diagnostics line; a hit is reported by place and class, never the matched bytes.
- (.andromeda/test-plan.md:609) This matrix is out of the chunk's scope and must stay with the exit-cause entry; the chunk must not build it here.
  > **Exit-cause matrix** (`cross_exit_causes.rs`, not built): one negative test per cause of exit 21 and of exit 1, each asserting the exit code, the `--json` document and a last `hint:` line that names its cause
- (.andromeda/test-plan.md:616) The parity case must keep holding through any change to the refusal bodies.
  > for the same refusal, the MCP `structured_content` `{refusal, detail}` equals the CLI `--json` `{refusal, detail}`
- (.andromeda/test-plan.md:623) The chunk owns this PREREQ and must add a cap-driving case for each of the five rows, retiring the owed line when they land.
  > Owed: the enumeration's rows for the status-line command's stdout, the child pipe drain shared by `claude --version` and `claude agents --json`, and the `config.json` readers of the thresholds, the dialog hold and `claude_env_keep` name a witness that reads the bounded read without driving past its cap, so their caps are held by the row's needle and the census alone; owner: the route entry "Sanitised error surfaces", as a PREREQ (a cap-driving case for each row).
- (.andromeda/test-plan.md:670) A planted-value witness reading the receipt must keep it names only, the value never recorded there.
  > an `env` receipt holds names only, never a value.
- (.andromeda/test-plan.md:874) Each sanitisation case must sit at the lowest layer that can hold it, the witness per surface asserting only cross-process behaviour at E2E.
  > NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
- (.andromeda/test-plan.md:896) The planted `CLAUDE*` value must reach the child through its own `Command::env`, never the test process's environment.
  > NEVER use `std::env::set_var` in tests. It is `unsafe` in edition 2024 and races across threads; use `Command::env` per child.

## Bindings
- **`-32602` `data:{supported, wrapper}` ↔ the chunk scope (Channel error data) and the arch Wrapper channel contract**: the plan requires an object of version values on this code while the scope requires `error.data` "a code or null"; the chunk must keep the shape and name it in the enumeration, or amend both sides, (.andromeda/test-plan-detail/test-scope-summary.md:60), (.andromeda/test-plan.md:182).
- **`wrapper-fault` `detail:{code,message,data}` ↔ the chunk scope (MCP errors) and the security plan's error-sanitization expectations**: the plan's body relays the wrapper's message and data while the scope and the plan's own trigger require codes only; the enumeration must show the relayed values fixed or the shape must be amended, (.andromeda/test-plan-detail/test-scope-summary.md:67), (.andromeda/test-plan.md:196).
- **`-32602` causes ↔ obs-plan metric `viola.compat.v_rejected`**: the folded freight requires the `channel-response` line to tell apart the causes these lines name, a contract change read against obs-plan, (.andromeda/test-plan-detail/test-scope-summary.md:154), (.andromeda/test-plan-detail/test-scope-summary.md:63).
- **Error sanitization trigger ↔ security plan error-sanitization expectations**: the trigger's forbidden contents are traced to the security excerpt, and `verification-matrix.json#v1-47` claims them, its web clauses out, (.andromeda/test-plan.md:269).
- **Secret scan ↔ obs-plan §9**: the witnesses' homes under `target/e2e-home` are what obs-plan's scan and uploads read, (.andromeda/test-plan.md:433), (.andromeda/test-plan.md:605).
- **`revive` documents and exit-cause matrix ↔ route entry "Exit-cause code catalogue"**: the owed start-refusal document and the hint matrix stay with that entry and move no plan line here, (.andromeda/test-plan-detail/test-scope-summary.md:33), (.andromeda/test-plan.md:609).
- **Owed PREREQ lines ↔ test-plan §4 viola-mcp and §6 Contract suite, `tests/contract_bounded_readers.rs`**: landing the cases must retire both owner lines and rename the five rows' witnesses, (.andromeda/test-plan.md:390), (.andromeda/test-plan.md:623).
- **Stdin line cap ↔ obs-plan `parse-rejected` line**: the WARN line's closed fields and its no-byte rule bind the pre-`initialize` case's reading, (.andromeda/test-plan-detail/test-scope-summary.md:71).
