@carried n=1 label=obs-plan/error-capture-reporting fact=fct-j5jj3gux
why: The chunk must re-measure the `thiserror::Error` derive census at HEAD (10 measured against the 9 stated) and re-word this count and its list, together with any fold or ratification of the one-enum-per-crate divergences.
quote: The product's `thiserror::Error` derives read 9: `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError`, `ServeError` (as measured at chunk 2026-10-31-sanitised-error-surfaces, a grep of `thiserror::Error` over `src` and `crates`).
@carried n=1 run=2 label=obs-plan/error-capture-reporting fact=fct-wmupioom
quote: `Refused::NotADirectory`'s message is fixed, `a trusted state path is not a directory`, and names no path; it is a variant of `Refused`, no derive of its own, so it adds nothing to the 9.
@carried n=1 run=3 label=obs-plan fact=fct-nwrbbhk2
quote: The mechanism is typed errors with fixed-message `Display` impls — hand-written on `PtyError` (`viola-pty` takes no thiserror), thiserror 2.0.20 on `AgentError`, `ChannelError`, `StateError`, `PinError`, `StampsError`, `StampsReadError`, `Refused`, `McpError` and `ServeError`.
@carried n=2 label=obs-plan/error-capture-reporting fact=fct-iy5crjhq
why: A fold of `ServeError` beside `McpError` must keep the oversize-before-`initialize` path at process exit 1 and re-word the variant this fact names.
quote: process exit 1 when the cut lands before it (`ServeError::Initialize`; `src/main.rs` `exit_code` gives `Role::Mcp` + `Failed` exit 1, and 11 is the code of the unit child alone, `serve_child_entry`)
@carried n=3 label=obs-plan/pii-scrubbing-compliance fact=fct-glcip7ij
why: Any fold of a divergent error enum must keep every resulting error's `Display` fixed, so drift reports stay in the instance detail file and the error-body census stays green.
quote: | serde_path_to_error drift reports | High | instance-scoped `detail-<process>.ndjson` only | Replaced by fixed `Display` at the seam; full report is a `drift_report` field in the detail file only |
@carried n=3 run=2 label=obs-plan fact=fct-i5zqforo
quote: fixed `Display` strings on every thiserror type (no fields holding paths or payloads);
@carried n=3 run=3 label=obs-plan fact=fct-6wyihaqe
quote: **Error layer:** fixed thiserror `Display`, and serde-sourced chains mapped to fixed messages at dispatch.
@carried n=3 run=4 label=obs-plan/pii-scrubbing-compliance fact=fct-ydc7ueix
quote: **Error-body census:** `tests/contract_error_bodies.rs` (`andromeda:walks-tree`) holds the error bodies of CLI `--json`, MCP results and the seven channel faults in tables: `error_bodies_each_name_a_writer_and_a_witness_that_stand` holds each body to a writer and a witness that stand, `product_error_writers_are_each_named` holds every product error writer to a name, and `thiserror_messages_interpolate_nothing` holds `thiserror` messages free of interpolation.
@carried n=4 label=obs-plan/error-capture-reporting fact=fct-vfnyc5f5
why: The chunk edits `Cargo.toml` and must re-run G3 green at the wrap, so every Cargo profile must keep `panic = "unwind"`.
quote: **Unwinding is required:** every Cargo profile keeps `panic = "unwind"`. `panic = "abort"` turns every `catch_unwind` into an abort, so `hook` would die by SIGABRT instead of exiting 0. Gate G3 (§9) enforces this.
@carried n=4 run=2 label=obs-plan/error-capture-reporting fact=fct-vfnyc5f5
quote: **`run` worker threads:** the PTY pump and handle-wait thread bodies each run inside `catch_unwind`. An `Err` is sent to `main`, which takes the `run` exit path below, because the session cannot continue without either thread. A worker panic that leaves `main` running with no `process-exit` is forbidden.
@carried n=4 run=3 label=obs-plan fact=fct-sue3fxzz
quote: the `run` PTY pump and handle-wait threads each run inside `catch_unwind`, and a worker panic that leaves `main` running with no `process-exit` is forbidden; every Cargo profile keeps `panic = "unwind"`, enforced by G3.
@carried n=5 label=obs-plan/error-capture-reporting fact=fct-3qk5e3jo
why: As the socket-directory refusal pair's wording is ratified or re-worded on the founder's word, its `socket-dir-refused` code must stay one of the closed `UnreachableCause` words, carried on MCP, CLI `--json` and start refusals alike.
quote: `detail` is one of the six closed words of `viola_core::UnreachableCause`: `strict-modes-failed`, `instance-dead`, `unwrapped-name`, `endpoint-lost`, `server-verify-failed`, `socket-dir-refused`.
@carried n=5 run=2 label=obs-plan/error-capture-reporting fact=fct-mgjsgjd2
quote: a refusal returns `instance-unreachable` with `detail: "socket-dir-refused"` through `UnreachableCause::SocketDir`, which no `RefusalReason` carries, and writes no frame and no line, the `isError` result its only record.
@carried n=5 run=3 label=obs-plan/error-capture-reporting fact=fct-25dvifnk
quote: in `frame` a refused `check_instance` is `StrictModes`, no `live_endpoint` is `without_endpoint`'s reading, a refused `check_endpoint_dir` is `SocketDir`, and a refused `peer_admitted` or a failed `server_verified` is `ServerVerify`;
@carried n=5 run=4 label=obs-plan/error-capture-reporting fact=fct-vfnyc5f5
quote: **Start refusals under `--json`:** `refused_with` (`src/cmd/run.rs`) prints the exit-1 document in place of the human pair for `pinned-hash-mismatch`, `squatted-name` and `socket-dir-refused`, and the `internal-error` document for a program not found, which has no human pair, all before `spawn_child`;
@carried n=5 run=5 label=obs-plan/pii-scrubbing-compliance fact=fct-exgbnztg
quote: | Protocol codes, refusal details | Low | OK to log structured | Closed kebab-case enums;
@carried n=5 run=6 label=obs-plan/pii-scrubbing-compliance fact=fct-ydc7ueix
quote: A socket-directory refusal carries no path, pid or uid in its lines and documents, and its human pair names only the parsed `ViolaName`: `tests/security_negatives_socket_dir.rs` reads them with `assert_names_nothing` and `digit_runs`, on Linux.
@carried n=6 label=obs-plan/pii-scrubbing-compliance fact=fct-ja2bi6wz
why: The live real-`claude` session is non-synthetic, so no R8-stripped `CLAUDE*` value may reach any file, detail files included, and the strip must log names only.
quote: | `CLAUDE_CODE_MESSAGING_TOKEN`, `CLAUDE_CODE_MESSAGING_SOCKET`, other R8-stripped `CLAUDE*` values | Critical | never-log anywhere, including detail files | R8 strip logs `env_stripped_count` + `env_stripped_known` (stripped names) + `env_kept` (kept names), names only, values never read (D-34); env maps never passed to a span or event; secret-scan test with one canary per identity-floor name plus an unknown name |
@carried n=7 label=obs-plan/pii-scrubbing-compliance fact=fct-a6wgb4sc
why: The live sessions' prompt and assistant text must reach no home-level process-log line, and paste through the bumped PTY seam must log `text_bytes` only.
quote: | Prompt text, assistant output (`prompt-submitted.data.text`, `last_assistant_message`) | High | never in any home-level process-log line; payloads stay in contract payloads and reach an instance `detail-<process>.ndjson` only inside a chain or drift report | `#[instrument(skip_all)]`; veil 0.3.0 `#[derive(Redact)]` with `#[redact]` on `SendParams.text`, `LastResult.last_assistant_message`, `HookEventParams.data`; only `text_bytes` logged |
@carried n=8 label=obs-plan/pii-scrubbing-compliance fact=fct-jv26h5pj
why: The live dialog hold near the 3600 s ceiling takes the dialog → `answer` lines as its evidence, and those lines must carry codes only, never the question, answer or tool input.
quote: | Tool `input`, plan text, dialog questions/answers | High | never in any home-level process-log line; instance `detail-<process>.ndjson` only inside a chain or drift report | `#[redact]` on `HookDialogParams.data`, `AnswerParams.response`; codes-only dialog events (`dialog_kind`) |
@carried n=9 label=obs-plan/pii-scrubbing-compliance fact=fct-ydc7ueix
why: The live real-`claude` sessions are a non-synthetic input source, so any upload of their diagnostics must first drop the detail files and the harness capture.
quote: Any non-synthetic input source must first drop `detail-*.ndjson` and `target/agent-run/` from the upload paths.
@carried n=9 run=2 label=obs-plan fact=fct-tcjwyf2p
quote: a detail-file upload is admissible only because every input is synthetic, so a non-synthetic source first drops `detail-*.ndjson` and `target/agent-run/` from the upload paths;
@carried n=10 label=obs-plan/pii-scrubbing-compliance fact=fct-ydc7ueix
why: The done-check must re-run the secret scan at zero hits before any CI upload at the wrap.
quote: **Verification:** the tests secret-scan test (`Cookie` header, `CLAUDE*` values, the content canary, 0600) over every `diagnostics/` file, run in CI before artifact upload.
@carried n=11 label=obs-plan/pii-scrubbing-compliance fact=fct-ydc7ueix
why: The chunk closes the red `supply-chain` job against its log and re-runs it green on the bumped `portable-pty` graph, so its unscanned artifact must still carry only crate names, versions, advisory ids, registry URLs and workflow-relative locations.
quote: the ci.yml `supply-chain` artifact (`target/supply-chain/deny.json`, `deny-fuzz.json` for the `fuzz/Cargo.lock` audit, and `zizmor.json`; `if: always()`, 7 days) holds cargo-deny and zizmor diagnostics only: crate names and versions, advisory ids, registry URLs and workflow-relative locations. It carries no absolute path, argv, log content or user input (as measured at chunk 2026-09-24-workspace-tree-and-code-graph-planes: 0 absolute paths in all three files; the one raw pattern hit was the `s:/` inside `https://`). The guard stays binding: a member that could carry a host path or log content is scanned before upload, or dropped (operator ratification 2026-09-24, logged for founder review).
@binding n=1
**error-capture-reporting ↔ pii-scrubbing-compliance**: the derive census re-worded and any fold or ratification of `ServeError`, `StampsError`, `PinError`, `StampsReadError` or `Refused` (obs-plan/error-capture-reporting fct-j5jj3gux) (obs-plan/error-capture-reporting fct-iy5crjhq) must land with the fixed-`Display` scrubbing layer intact and the error-body census green (obs-plan/pii-scrubbing-compliance fct-glcip7ij) (obs-plan/pii-scrubbing-compliance fct-ydc7ueix), and with the platform pick's list re-worded to the same count (obs-plan fct-nwrbbhk2).
@binding n=2
**error-capture-reporting ↔ pii-scrubbing-compliance**: the socket-directory refusal's wording, ratified or re-worded, keeps `socket-dir-refused` a closed `UnreachableCause` word on MCP, CLI and start refusals (obs-plan/error-capture-reporting fct-3qk5e3jo) (obs-plan/error-capture-reporting fct-vfnyc5f5) and a closed kebab-case refusal detail naming no path, pid or uid (obs-plan/pii-scrubbing-compliance fct-exgbnztg) (obs-plan/pii-scrubbing-compliance fct-ydc7ueix).
@binding n=3
**pii-scrubbing-compliance ↔ ci-integration**: the live real-`claude` sessions are non-synthetic, so their diagnostics hold the `CLAUDE*`, prompt and dialog floors (obs-plan/pii-scrubbing-compliance fct-ja2bi6wz) (obs-plan/pii-scrubbing-compliance fct-jv26h5pj) and any upload drops `detail-*.ndjson` and `target/agent-run/` first (obs-plan/pii-scrubbing-compliance fct-ydc7ueix), while CI's secret scan reads zero hits before the gated uploads (obs-plan fct-xwehypb2).
@binding n=4
**error-capture-reporting ↔ slo-invariants-telemetry-budgets**: the `Cargo.toml` pin edit keeps every profile at `panic = "unwind"` (obs-plan/error-capture-reporting fct-vfnyc5f5), and G3 must exit with rg exit 1 at the wrap (obs-plan fct-2rxslar4) (obs-plan fct-5kn2jpbt); the bumped `portable-pty` keeps the PTY pump and handle-wait threads inside `catch_unwind` (obs-plan fct-sue3fxzz).
@binding n=5
**pii-scrubbing-compliance ↔ security-plan (Vector 9)**: the `portable-pty` move off `=0.8.1` turns the red `supply-chain` job green with no ignore-list widening, and its `deny.json` artifact stays content-admissible (obs-plan/pii-scrubbing-compliance fct-ydc7ueix) (obs-plan fct-r2s52jjf).
