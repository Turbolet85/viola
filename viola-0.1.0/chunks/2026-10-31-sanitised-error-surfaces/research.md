# Codebase Research — 2026-10-31-sanitised-error-surfaces

## Scope
- **Depth:** deep · **Reads:** 22 · **Globs/Greps:** 19
- **Harness rules consulted:** none — no live leg in this chunk (the real `claude` runs in no gate; every witness drives the fake agent or a test-owned server)
- **Platform issues consulted:** none — the one runner-only bullet is the Path 1 WATCH, an observation this chunk plans no instrumentation for
- **External inputs:** `inputs#I1` — the operator directive of this phase

## Files inspected
- `src/cmd/client.rs` (1–300) — `reply_of` copies the wire's `refusal` and `detail` strings; `fault_detail` copies the wire's `message` and any object `data` into `wrapper-fault`'s `detail`.
- `crates/viola-mcp/src/result.rs` (1–135) — a second copy of `fault_detail`, and `of_reply` copies `refusal` and `detail` into `structuredContent` and into the text `refused: <reason>`.
- `crates/viola-channel/src/lib.rs` (1–100, 215–254) — `ProtocolError`: fixed messages, two `data` objects, the literal wire table test.
- `crates/viola-channel/src/client.rs` (177–220) — `reply_class` / `fault_of`.
- `crates/viola-channel/src/server.rs` (423–518) — `Class`, `ResponseLine` and its `Drop`.
- `schemas/diag-line.v1.json` (whole) — `channel-response` admits `detail` as an open string.
- `src/main.rs` (47–148) — `exit_code`, `prints_internal_error`, the catch site.
- `src/obs.rs` (226–286) — `internal_error_exit_line` (only `run` and `cli`), `report_internal_error`.
- `crates/viola-mcp/src/lib.rs` (whole) — `serve`, `ServeError`, the `serve_child_entry` exit map.
- `src/cmd/mcp.rs` (whole) — `viola_mcp::serve(..)?`.
- `crates/viola-mcp/src/tools.rs` (1–300) — the raw-argument handlers, `unknown tool`, `internal_fault`.
- rmcp 3.4.1 `src/transport/async_rw.rs`, `src/handler/server.rs`, `src/service/server.rs` — its own connection faults.
- `tests/contract_bounded_readers.rs` (1–260) — the table form, the five rows.
- `src/cmd/hook/statusline.rs` (165–285), `src/run/budget.rs` (50–95), `src/run/dialog.rs` (158–187), `src/run/env.rs` (31–60), `src/run/version_gate.rs` (41–132, 418–440) — the five bounded reads.
- `src/cmd/send.rs` (181–251), `src/cmd/answer.rs` (85–148) — the exit-2 lines and the refusal arms.
- `crates/viola-e2e/tests/mcp_tools.rs` (165–204, 446–520) — the owned-child form.
- `crates/viola-agent-claude/src/lib.rs` (194–255) — `AgentError`, `Refusal`, `resolve_program`.

## Graph impact (from the code-graph query; "cold-start — empty DB" if early)
- Trace: `tree-query-2026-10-31-sanitised-error-surfaces.json`, one query (the rust plane regenerated first, stale, 6965 nodes in 213 s); lines below are editor lines (the trace's 0-indexed `line` + 1).
- **`fault_detail`** — 2 callers: `of_reply` @ `crates/viola-mcp/src/result.rs:57`, `reply_of` @ `src/cmd/client.rs:70`. Two definitions, replaced by one shared reader.
- **`fault_of`** — `reply_class` @ `crates/viola-channel/src/client.rs:205`, and the unit `fault_of_maps_all_five_codes` @ `:655`, `:657`, which the signature change moves.
- **`reply_class`** — `reply_from` @ `crates/viola-channel/src/client.rs:190`, reached by both clients (`crates/viola-channel/src/tokio_client.rs:18`, `:62`), and the unit `reply_class_reads_the_outcome` @ `:606`.
- **`reply_of`** — `answer_of` @ `src/cmd/client.rs:182`, and its unit @ `:386`. **`of_reply`** — `frame` @ `crates/viola-mcp/src/tools.rs:345`, `:351`, and three units. Neither signature changes.
- **`resolve_program`** (`viola_agent_claude`, the fold's signature) — its one product caller is the root wrapper `resolve_program` @ `src/run/mod.rs:152`, whose callers `start` @ `src/cmd/run.rs:264`, `measure` @ `src/cmd/verify.rs:147` and `read_here` @ `src/run/agents.rs:110` read the fold; plus the crate's own units. `src/cmd/revive.rs:57` is a private enum of the same name, unrelated.

## Patterns detected
- **A fixed wire table pinned by literals** (`crates/viola-channel/src/lib.rs:220`): every `ProtocolError` body is a literal JSON string in an rstest table.
- **A contract binary with literal tables, a census and a planted self-check** (`tests/contract_bounded_readers.rs:29`): the shape the error-body enumeration takes.
- **A child test that owns its `viola mcp`** (`crates/viola-e2e/tests/mcp_tools.rs:446`): own writer thread, drained pipes, an 8 s kill deadline, a canary inside the over-long line.
- **The closed `Unreachable` cause beside one document** (`src/cmd/client.rs:104`): one code on the wire, the cause only in the log.

## Conventions to follow
- **A reply reader judges its input** (`src/cmd/client.rs:80`): a wrong-shaped `detail` already reads as no known reply; the screen this chunk adds sits at the same place.
- **Shared validators live in `viola-core`** (`.claude/rules/security.md`, Input validation): the code-word check goes there.
- **A `channel-response` field is one more `obs_event!` argument in the `Drop` guard** (`crates/viola-channel/src/server.rs:491`), absent when `None`.
- **Three-OS oracles**: a witness home is `stamped_home` under `target/e2e-home`; a path oracle compares both spellings of the home (`resolved(home)`, `tests/support/home.rs`).

## New files to create
- `tests/contract_error_bodies.rs` — the enumeration: literal tables of the three surfaces' error bodies, a census over product sources, and a planted self-check.
- `tests/channel_error_bodies.rs` — the channel witness: raw frames with a path, an upstream text and a planted value in play, against a real wrapper.
- `tests/cli_error_bodies.rs` — the CLI witness: each named failure's document with the same three in play.

## Files to modify
- `crates/viola-core/src/lib.rs` — the code-word check, with its literal unit table.
- `crates/viola-channel/src/lib.rs` — `ProtocolError::cause` and the one sanitising reader of a fault, with their unit tables.
- `crates/viola-channel/src/client.rs` — `reply_class` and `fault_of` read the fault whole, so the client's line tells the causes apart.
- `crates/viola-channel/src/server.rs` — `ResponseLine` writes the cause as `detail`.
- `crates/viola-mcp/src/result.rs` — the shared reader in place of its copy; the refusal screen; new table rows.
- `crates/viola-mcp/src/lib.rs` — the pre-`initialize` `serve` child case.
- `crates/viola-mcp/tests/handlers.rs` — a refusal holding free text, read through `Tools`.
- `src/cmd/client.rs` — the shared reader in place of its copy; the refusal screen; new table rows.
- `crates/viola-agent-claude/src/lib.rs` — `Refusal` folded into `AgentError`.
- `src/run/mod.rs` — the fold's signature and import.
- `src/cmd/run.rs` — the fold's two arms and import.
- `src/cmd/verify.rs` — the fold's three arms and import.
- `src/run/budget.rs` — the thresholds row's cap-driving case.
- `src/run/dialog.rs` — the dialog hold row's cap-driving case.
- `src/run/env.rs` — the `claude_env_keep` row's cap-driving case.
- `src/run/version_gate.rs` — the drain row's cap-driving case.
- `src/cmd/hook/statusline.rs` — the status-line stdout row's cap-driving cases.
- `tests/contract_bounded_readers.rs` — the five rows' witnesses.
- `tests/support/events.rs` — the error-body check test-plan §6 names.
- `crates/viola-e2e/tests/mcp_tools.rs` — the pre-`initialize` process case and the MCP witness.
- `schemas/diag-line.v1.json` — `channel-response`'s `detail` closed to the three cause words.
- `Cargo.toml` — the two wrapper-booting binaries' `[[test]]` tables, `required-features = ["fake-agent"]`.
- `.config/nextest.toml` — the two wrapper-booting binaries on the `ci` profile's 20 s line.

## Sweeps
- `fault_detail` over `src` and `crates`: 2 definitions · 2 changed · 0 no-change (re-derived: `command grep -rn "fault_detail" src crates`).
- `Refusal` over `src` and `crates`: 16 product-line hits · 12 changed · 4 no-change (`src/cmd/revive.rs`'s private enum and its arms) (re-derived: `command grep -rn "Refusal::\|, Refusal\|Refusal {" src crates --include=*.rs`).
- `#\[error(` holding `{` over `src` and `crates`: 0 hits of 36 attributes (re-derived: `command grep -rn '#\[error(' src crates --include=*.rs` and its `{` filter) · no `thiserror` message at HEAD interpolates a field.
- `CoreError` over `crates` and `src`: 0 hits (re-derived: `command grep -n "CoreError" -r crates src --include=*.rs`) · the type the masters list does not exist.
- `thiserror::Error` over product sources, 10 derives: `AgentError`, `Refusal`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError` (re-derived: `command grep -rn "thiserror::Error" src crates --include=*.rs`) · no-change but for `Refusal`.
- `channel-response` over the tree: 7 files name it (`tests/channel_endpoint.rs`, `tests/contract_diag_schema.rs`, `tests/cli_answer.rs`, `tests/cli_wait_last.rs`, `tests/cli_links.rs`, and the two channel sources) · 2 changed · 5 no-change (each reads named fields, an added key moves none; the run decides, at `gate` entries 3 and 4).
- `exit 11` of `viola mcp` over the tests: 1 hit, `crates/viola-mcp/src/lib.rs:162`, the unit child's own map (re-derived: `command grep -rnE "Some\(11\)" crates/viola-e2e/tests/mcp_tools.rs crates/viola-mcp`; `mcp_tools.rs:190`'s `Some(11)` is `budget-paused`) · no test reads the product's exit at that cut.

## Open questions
- none

## Premise closure, and the mechanisms the plan states (re-derived at HEAD)
- **Falsified — "`ServeError::Initialize` (exit 11)" as the product's exit.** `viola mcp` runs `viola_mcp::serve(..)?` (`src/cmd/mcp.rs:36`), so the error reaches `main` as `Outcome::Failed`, and `exit_code(Role::Mcp, Failed)` is 1 (`src/main.rs:137-140`), nothing printed (`src/main.rs:146`). 11 is `serve_child_entry`'s own map for its parent test (`crates/viola-mcp/src/lib.rs:123`). The equality the plan states: one line past `MAX_FRAME` before `initialize` gives `ServeError::Initialize` from `serve` (the cut reads as end of stream, as a closed stdin does, `:80`), one `parse-rejected{mcp-stdin, oversize, count:1}` (`:86`), and a `viola mcp` process exit of 1 with stdout and stderr empty. No `process-exit` line: `internal_error_exit_line` writes only for `run` and `cli` (`src/obs.rs:232`), though `.claude/rules/observability.md` says `mcp` writes one.
- **Falsified — the scope's "`error.data` a code or null".** `ProtocolError::body` builds `{supported, wrapper}` and `{"reason":"release-from-driver"}` (`crates/viola-channel/src/lib.rs:95`), both contract text (architecture §Conventions, Interfaces and versioning; §Standard Contracts, Channel methods) and pinned by literals. The rule the chunk holds: closed fields, no free text.
- **Falsified — the scope's "code-shaped or null `detail`" for `wrapper-fault`.** The contract's `detail` is the fault's `{code, message, data}`. The rule the chunk holds: that object's every field fixed or closed.
- **Falsified — that the bodies already hold, the chunk only proving it.** Three copies carry peer text onto the surfaces: `fault_detail`'s `message` and `data` on both clients, `reply_of`'s `refusal` and `detail` (`src/cmd/client.rs:76-81`), and `of_reply`'s (`crates/viola-mcp/src/result.rs:65-75`), which also reaches the text `refused: <reason>`. A wrapper answering `{"error":{"code":-32603,"message":"/home/x/y","data":{"p":"…"}}}` prints that message and data under `--json` and in MCP `structuredContent`. Only a process that passed the server check, or any process on macOS (no peer pid, the recorded gap), can send it, and every viola wrapper sends fixed bodies, so this is the defence the property needs when the peer is not this build. The equality the plan states: for THESE inputs, a fault of a known code yields the fixed message of `ProtocolError`, `data` only in its two closed shapes else `null`; an unknown code yields the fixed `"wrapper fault"`; a refusal `reason` or `detail` that is no code word yields `unknown` and `null`.
- **Corrected — `Refusal` "the one" divergence.** The masters name three (`Refusal`, `ServeError`, `StampsError`) and one ratified exception. The census reads more: `PinError`, `StampsReadError` and `Refused` also derive `thiserror` beside `StateError`, and `CoreError`, which the masters list, does not exist. All messages are fixed. The entry's freight owns `Refusal` alone.
- **Verified — the `fault_of` mechanism.** `fault_of` finds `UnsupportedVersion` for every `-32602` (`crates/viola-channel/src/client.rs:215`), and the label reaches only `code()` and the line's level (`crates/viola-channel/src/server.rs:499`, `:505`). The server's own line has the exact fault and drops it the same way. The equality the plan states: the three `-32602` faults yield the `detail` words `unsupported-version`, `invalid-params`, `release-from-driver` on both sides; the client reads them from `data` (`supported` · `reason` · neither). The level does not move: all three are WARN (`:507`).
- **Verified — no `Display` reaching a surface holds a path or payload** (§Sweeps): every attribute is a literal; the census makes that hold.
- **Verified — rmcp's own faults hold no caller text**: `Invalid request` (`async_rw.rs:182`), the required-key names (`service/server.rs:523`), a parse failure logged under a target that is OFF and answered nothing. Viola's `unknown tool` holds no name (`crates/viola-mcp/src/tools.rs:232`). The MCP witness drives one wrong-shaped `tools/call` with the canary in it to hold that.
- **Verified — the drift leg's producer**: `write_drift` (`src/cmd/hook.rs:538`) through `write_detail` (`src/obs.rs:323`), held by `handle_writes_a_drift_report_only_to_the_detail_file` (`src/cmd/hook.rs:787`). The owner-only half: `write_detail_refuses_a_widened_diagnostics_folder_and_leaves_it` (`src/obs.rs:660`) and the scan's mode class.
- **Absent — test-plan §6's after-every-test error-body check.** `tests/support/events.rs` holds four functions and no such check (re-derived: `command grep -n "fn " tests/support/events.rs`). The scan half stands: `id: secret-scan`, `if: always()` (`.github/workflows/ci.yml:104`), reading the kept homes, the harness capture and JUnit.
- **Falsified — test-plan §5's "oversize `answer` … exits non-zero with one `--json` error document".** The exit-2 paths of `send` and `answer` write their fixed `UNUSABLE_*` line to stderr whatever `--json` says (`src/cmd/send.rs:187`, `src/cmd/answer.rs:91`), the design table's row 2 giving no document. The chunk changes neither; the enumeration names them.
- **The cap-driving equalities:** a `config.json` read through `take(MAX_FRAME)` keeps the first `MAX_FRAME` bytes, so a key standing past the cap is never read: `{"v":1,"pad":"…","five_hour":50}` with the closing brace past the cap fails the parse and gives the defaults with no rejection (`src/run/budget.rs:69`, `src/run/dialog.rs:171`, `src/run/env.rs:44`), and the same document one byte within the cap gives 50, the control. A status-line command that outlives the cut gets a broken pipe: `dd …; exit 0` exits 0 and prints exactly `MAX_FRAME` bytes; `exec dd …` dies of the signal, a failed command, and prints nothing. `drain` over an endless reader returns exactly `cap` bytes. These read the source; `/andromeda-implement` reads each red first.
- **Runner-only — the Path 1 WATCH** is closed against its recorded runs. New reading at Setup 5a: green at 14.205 s (`SLOW [> 10.000s]`) on `test (ubuntu-latest)`, 6.367 s on `windows-2025`, 6.808 s on `macos-latest`, at `ci#40467224735` over `5273ab8c` (`gh run view --log`).
