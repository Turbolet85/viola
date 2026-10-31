proposals:
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 111
    section: "## Standard Contracts"
    change: Replace `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` in the refused strict-modes document of `viola link` and `viola unlink`; the rest of the line stands.
    sidecar: The refused strict-modes document of link and unlink now carries detail strict-modes-failed in place of null.
    rationale: Report Retired names the exit-21 document's `detail null`, and Counts doc-null moves each strict-modes site to `strict-modes-failed`; Expected amendments names this line.
    basis: src/cmd/client.rs:222
  - detector: D-retired
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 117
    section: "## Standard Contracts"
    change: Replace `{"v":1,"error":"instance-unreachable","detail":null}` with `{"v":1,"error":"instance-unreachable","detail":"strict-modes-failed"}` in the document the six frame verbs print for their refused strict-modes check; the rest of the line stands.
    sidecar: The refused strict-modes document of the six frame verbs now carries detail strict-modes-failed in place of null.
    rationale: Report Retired names the exit-21 document's `detail null`, and Symbols / APIs has `refuse_untrusted` write `StrictModes` through `unreachable_document`; Expected amendments names this line.
    basis: src/cmd/client.rs:222
    dependent-of: D-retired
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 119
    section: "## Standard Contracts"
    change: The exit-21 document names its cause since chunk 2026-10-31-exit-cause-code-catalogue (the founder's own word, that chunk's `inputs#I2` Q1), reading `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` with `v` still 1 and `error` still `instance-unreachable`, no exit moved and no `RefusalReason` or exit code added. The five closed causes are `strict-modes-failed` · `instance-dead` · `unwrapped-name` · `endpoint-lost` · `server-verify-failed`, the words of `viola_core::UnreachableCause` (`StrictModes` · `InstanceDead` · `UnwrappedName` · `EndpointLost` · `ServerVerify`, through `as_str`, no serde path). `client::unreachable_document(cause)` is the one writer of the document, with `refuse_untrusted` and `viola plugin install` writing `StrictModes` through it; `viola list`'s `refused_document` (a `Value` for `board_document`) builds `strict-modes-failed`, the board's only refusal; `client::refused_document` stays exit 1's.
    sidecar: Adds the exit-21 document's cause form, its five closed causes, UnreachableCause and the one writer unreachable_document.
    rationale: Report Symbols / APIs, the exit-21 document moved, `viola_core::UnreachableCause` and `src/cmd/client.rs`; Expected amendments carries architecture §Standard Contracts CLI `--json` output.
    basis: crates/viola-core/src/lib.rs:181
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 119
    section: "## Standard Contracts"
    change: Which cause a CLI frame verb reads is `client::Unreachable`'s, now `Dead` · `Unwrapped` · `Lost` · `ServerUnverified` (`cause()`, `detail()`). In `answer_of` a `Connect` failure is `Dead` (`instance-dead`) and `Closed | Io` is `Lost` (`endpoint-lost`, a wrapper lost with the call open); a failed server check is `ServerUnverified` (`server-verify-failed`). The six no-endpoint arms of `send`, `wait`, `last`, `answer`, `pause` and `link` read `Unreachable::without_endpoint(&instance_dir)`, with `release` and `unlink` reaching them through `wheel_call` and `link_call`; the unwrapped reading by the instance directory is the delegate's word, PROVISIONAL (that chunk's `inputs#I2` Q2).
    sidecar: Adds how a CLI frame verb maps a failed connect, a lost call, a failed server check and a missing endpoint to its cause.
    rationale: Report Symbols / APIs, `src/cmd/client.rs` and the six no-endpoint arms; Decisions carries Q2 as the delegate's word, PROVISIONAL.
    basis: src/cmd/client.rs:114
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 119
    section: "## Standard Contracts"
    change: MCP's `isError` result for an unreachable instance is the same body less `v`, its text unchanged and the cause in `detail`, built by `viola_mcp`'s `result::unreachable(cause)`. In `of_reply` a `Connect` failure is `InstanceDead` and `Closed | Io` is `EndpointLost`; in `of_board` the board's refusal is `StrictModes`; in the frame tools a refused `check_instance` is `StrictModes`, no `live_endpoint` is `without_endpoint`'s reading and a failed `server_verified` is `ServerVerify`. None of them writes a frame or a line, `isError` the record; JSON-RPC errors did not move and `-32603` keeps its fixed body.
    sidecar: Adds the MCP unreachable result's cause in detail and the path each of its causes comes from.
    rationale: Report Symbols / APIs, the exit-21 document moved and the MCP bullet; the founder's own word `inputs#I2` Q1.
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 119
    section: "## Standard Contracts"
    change: Since chunk 2026-10-31-exit-cause-code-catalogue `viola revive --json` prints one exit-1 document for every refusal of the start it shares with `run`, its codes `strict-modes-failed` · `already-live` · `no-session` · `session-live` · `cwd-missing` · `stale-heartbeat` · `batch-script-child` · `pinned-hash-mismatch` · `squatted-name` · `internal-error`. `Launch` gains `json` (`run` passes false, `revive` passes `args.json`), and `refused_with(json, detail, human)` prints `refused_document` in place of the human pair for `batch-script-child`, `pinned-hash-mismatch`, `squatted-name` and, for a program not found, `internal-error`, all before `spawn_child`; the human pairs stand byte for byte. An anyhow error from `trusted`, `checked_exe`, `open_wrapper_log`, `preflight` or `start` under `--json` prints `refused_document("internal-error")` and returns the same error, so the catch site still logs its chain and its line. A `pump_child` error prints no document, because the child has held the terminal and viola writes nothing past it.
    sidecar: Adds revive --json's start documents, closing the post-preflight gap, with refused_with and the pump_child exception.
    rationale: Report Symbols / APIs, `run`'s start under `--json` and `viola revive --json`; Retired names the post-preflight gap.
    basis: src/cmd/revive.rs:119
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 119
    section: "## Standard Contracts"
    change: A `stale` holder is refused by `collision_check` (`src/cmd/run.rs`) with the code `stale-heartbeat`, printed and logged, while a live holder and a fresh replayed beat stay `already-live`. Stale heartbeat is exit 1's cause alone, because a frame verb against a stale instance reads `instance-dead`.
    sidecar: Adds stale-heartbeat as a stale holder's own code, separate from already-live.
    rationale: Report Symbols / APIs, a stale holder's code; Retired names `already-live` as a stale holder's refusal.
    basis: tests/cli_instance_state.rs:193
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 119
    section: "## Standard Contracts"
    change: At the `cli` catch site, where `prints_internal_error` is true and `src/main.rs` `json_asked(args)` finds `--json` among the arguments before a `--` (read from argv as `role_of` is), `cmd::internal_error_document()` prints `{"v":1,"error":"internal-error","detail":null}` on stdout in place of the human internal-error line, stderr then empty. Without `--json` the line stands byte for byte; `run`, `revive`'s start, `mcp` and `hook` stay silent there.
    sidecar: Adds the catch site's internal-error document under --json, with json_asked and internal_error_document.
    rationale: Report Symbols / APIs, the `cli` catch site under `--json`; Counts catch-site has a site stating the output under `--json` take the document.
    basis: src/main.rs:154
  - detector: D-new-detail
    severity: warning
    file: .andromeda/architecture-detail/standard-contracts.md
    near: 135
    section: "## Standard Contracts"
    change: Where `live_endpoint` gives none, `viola_state::liveness::without_endpoint(instance_dir) -> UnreachableCause` reads the cause through `symlink_metadata`, a directory reading `InstanceDead` and anything else (absent, a regular file, a symlink) `UnwrappedName`; it opens nothing and writes no line, and the strict-modes check stays first at every caller. Its callers are `client::Unreachable::without_endpoint` and the `frame` of `viola-mcp`'s tools. `live_endpoint` gives none for a stale wrapper, so a frame verb against a stale instance reads `instance-dead`.
    sidecar: Adds without_endpoint, the directory reading behind instance-dead and unwrapped-name, to the session liveness contract.
    rationale: Report Symbols / APIs, `viola_state::liveness::without_endpoint` and a stale holder's code; Decisions carries the unwrapped reading as `inputs#I2` Q2, PROVISIONAL.
    basis: crates/viola-state/src/liveness.rs:107
note: The body's CLI `--json` output lines architecture.md:226 and :228 hold the same retired claims and its plugin install line beneath them still names `client::refused_document` for the exit-21 document, which the report moves to `unreachable_document`; they lie outside this file, so no proposal here, and D-arch-decisions found no drift (report Dependencies none, no crate added).
