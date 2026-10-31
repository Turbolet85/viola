# Task — arch · 2026-10-31-exit-cause-code-catalogue

## The chunk
Gives every cause of exit 21 (`instance-unreachable`) and of exit 1 its own distinct closed code on `--json`, MCP and the process log, carrying no path and no pid, each code matching the human `hint:` line of the same cause.
Builds the new rstest table `tests/cross_exit_causes.rs`, one negative case per cause asserting the exit code, the `--json` document and a last stderr `hint:` naming that cause, a hint shared across causes failing the table.
Closes the owed gap of `v1-39`: the refusals `run`'s start makes after a passed preflight print a document under `viola revive --json`; builds the unwrapped-name hint; brings `viola list` and `viola plugin install` into the table or names them outside it by a reason; moves the architecture, design-system, security-plan and test-plan text naming the document.
Re-reads at HEAD the claim that `revive`'s strict-modes refusal writes no `process-exit` line because its check runs before the wrapper log opens, whether `Refusal::StrictModes.detail()` still stands, and whether `session-live` is built; tallies the `verify_stopped_during_the_resume_run_removes_its_dir_and_exits_1` margin watch on its CI reads.

## Reached files
- tests/cross_exit_causes.rs · edits the new exit-cause table
- crates/viola-core/src/lib.rs
- src/cmd/client.rs · edits `refused_document`, `refuse_untrusted`, the liveness pre-check, the server check
- src/cmd/send.rs
- src/cmd/wait.rs
- src/cmd/last.rs
- src/cmd/answer.rs
- src/cmd/pause.rs
- src/cmd/release.rs
- src/cmd/link.rs
- src/cmd/unlink.rs
- src/cmd/list.rs
- src/cmd/plugin.rs
- src/cmd/mcp.rs
- src/cmd/run.rs · edits its start refusals, `checked_exe` / `refuse_tree`, the collision check
- src/cmd/revive.rs · edits the document after a passed preflight
- src/cmd/verify.rs · edits its refused home tree
- src/human/hints.rs · edits the unwrapped-name hint, each cause's own hint
- src/human/mirror.rs
- crates/viola-mcp/src/result.rs
- crates/viola-mcp/src/tools.rs
- crates/viola-channel/src/client.rs · edits the vanished server read as `instance-unreachable`
- schemas/diag-line.v1.json · edits the `process-exit` `detail` values
- tests/cli_machine_contract.rs
- tests/cli_revive.rs
- tests/cli_list.rs
- tests/cli_links.rs
- tests/cli_verify.rs
- tests/cli_plugin_install.rs
- tests/security_negatives_strict_modes.rs
- tests/security_negatives_server_identity.rs
- tests/contract_error_bodies.rs · edits the census of error bodies and writers
- tests/cli_error_bodies.rs
- crates/viola-mcp/tests/handlers.rs
- crates/viola-e2e/tests/mcp_tools.rs
- .andromeda/architecture.md · edits the text naming the `instance-unreachable` document
- .andromeda/design-system.md · edits the text naming that document, the unwrapped-name cli pattern's hint
- .andromeda/security-plan.md · edits the text naming that document, its error rules
- .andromeda/test-plan.md · edits the Exit-cause matrix, §6's security-control table row for `viola list`
- .claude/rules/security.md · re-reads the strict-modes refusal's `detail: null` rule, its `session-live` line

## Needs
- The CLI `--json` error documents: the exit-21 `instance-unreachable` document, the exit-1 `<code>` document of `viola revive --json`, their writer `client::refused_document`, the empty stderr beside them, and the choice of carrying a cause's code in `error` or in `detail`.
- The document owed after a passed preflight under `viola revive --json`: `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, a `claude` not on `PATH` as `internal-error`.
- The exit-code table: the typed set 0, 1, 2, 10–14, 20, 21, which refusals each of exit 1 and exit 21 holds, a set this chunk neither adds to nor re-maps.
- The strict-modes refusals of `viola list`, `viola plugin install`, `viola link`, `viola unlink` and the six CLI frame verbs as exit 21, and each verb's order of checks ahead of any log: a refused check writes no line, and this chunk must not move a check after a log opens or make it write a line.
- The home-tree refusals of `run`, `viola revive`'s start and `viola verify`: `check_home_tree`, exit 1, the home-state and instance-state `unable:` / `hint:` pairs, no log opened, each now owing a code.
- MCP results: `isError: true` tool results shaped like CLI `--json`, the unreachable instance's text and `structuredContent`, the JSON-RPC errors kept to the connection's own faults, and `viola mcp`'s refused home check with nothing on stdout or stderr.
- The sanitised error surfaces: code words crossing a client read through `viola_core::is_code_word` and `viola_channel::sanitised_fault`, each new code and writer inside the census of `tests/contract_error_bodies.rs`, no code holding a path, a pid or upstream text.
- The closed refusal vocabulary: `RefusalReason` and its details extended only in `viola-core`, an unrecognised code word opaque to a reader, wire values kebab-case; new cause codes must fit the same home and reader rule.
- Mixed-version and protocol versioning: an additive field needs no `v` bump, a changed field meaning does, and a reader of an unknown word tolerates it; the moved document's `v` and a client's reading are held to this.
- The not-running and replayed-snapshot causes: `viola_state::liveness::live_endpoint`, the liveness pre-check, a replayed snapshot carrying no `endpoint` or `pid` so the instance reads as unreachable.
- The unwrapped-name cause: a `wrapped: false` row of the `list` payload is never a valid `target`, and the send to such a row exits 21.
- The server-verification cause: the frame-writing client's check of the connected peer's pid and OS start time against the snapshot's `pid` and `started_at`, a mismatch writing no frame; macOS reporting no peer pid stays the recorded gap, and the Windows connect path is not reshaped.
- The endpoint-lost-mid-call cause: the vanished server read as `instance-unreachable` in `viola-channel`'s client, on both its sync flavour and the Tokio flavour `viola-mcp` enables, kept apart from the wrapper's protocol fault at exit 20.
- The start-refusal causes of `viola run` and `viola revive`: `already-live`, a stale heartbeat, `squatted-name` from the collision check, liveness judged by heartbeat, pid and start time, and the heartbeat-alone judgement over a replayed snapshot.
- The pinned copy's and the child's refusals: `pinned-hash-mismatch` leaving the file as found, `batch-script-child` with the `claude` npm shim carve-out, a `claude` not on `PATH` as `internal-error`.
- `viola revive`'s preflight causes: `strict-modes-failed`, `already-live`, `no-session` with its two stderr pairs, `session-live`, `cwd-missing`, `--list --json`'s `no-session` on an empty chain; the snapshot's `cwd` reaching no log line, error body or stdout.
- The socket directory: the per-user 0700 Unix directory and its verified mode, which stay another route entry's; a code here only if the code holds that refusal today.
- The process log's closed list: `schemas/diag-line.v1.json`'s `process-exit` `detail` values, default-deny and pinned by `tests/contract_diag_schema.rs`, so each new value joins the schema; a cause with no line gets its log form or a named decision for its absence.
- The role files a cause's line lands in: `cli-<name>`, `run-<name>`, `mcp` under `diagnostics/`, and which verbs have opened one by the time of each refusal.
- The human hint surface on stderr: one `hint:` per cause, last on stderr, matched to the code; driver-facing hints never name `viola release`; the short-lived CLI's use of stderr against `--json`'s empty stderr.
- The `viola hook` contract as a bound: exit 0, no body, no stderr, its `hook-decision` detail its only cause record; none of this chunk's codes reaches it.
- The internal-error cause under [Error Handling]: `anyhow` chains serve only the owner's post-mortem and reach no role line, stdout or stderr, so the `internal-error` code carries none; a new error type joins the crate rule and its census.
- The local endpoint trust boundary, read against whether a distinct code tells a same-user process more than `detail: null` does today; a widening is the founder's word.
- The frontend's deferral to 0.2.0, by which `v1-18`'s `ui` start refusal is named out if the code holds none.
- The watch's bound: root test support's `Watch` and `wait_for` at `WITHIN` = 7 s in `tests/support/fake.rs`, no bound moved, its margin tallied on this chunk's CI reads.

## Standing conventions
- S1 · Every new test runs in its own viola home through `--home`, sharing no endpoint, log or `budget.json`, and substitutes the fake agent by naming its executable after `--`, never the real CLI in CI · Established Decisions, [CI/CD]
- S2 · A test home lives under the base of `target/e2e-home`, inside the obs gates and the secret scan, a home outside it being a named carve-out · .andromeda/architecture-detail/occupied-resources.md
- S3 · A new fake-agent scenario under `fixtures/fake-scripts/` holds synthetic text only and validates against `schemas/fake-script.v1.json` · .andromeda/architecture-detail/occupied-resources.md
- S4 · A process-log line is codes only, one complete object per single write, in its home-level role file, with content-bearing detail only in the instance detail files · Cross-cutting Patterns, Diagnostic output channels
- S5 · An OS-specific branch or test case compiles and is tested on its CI runner, and no case leans on a shell or a POSIX-only behaviour · Cross-cutting Patterns, Cross-platform discipline
- S6 · JSON field names are snake_case and wire enum values kebab-case in every viola format · Conventions, Naming patterns

## Not needed
- The PTY seam and the ConPTY companions: a sideload's `strict-modes-failed` falls back to the inbox ConPTY and never refuses a start, so it is no exit cause.
- The screen model, delivery confirmation, the human-takeover wheel and the budget governor's gate: their refusals are exits 10–13 with reasons already distinct, not re-opened here.
- The capability ledger, its rows and probes, and the stamps envelope: the chunk relies on no new `claude` behaviour.
- The event kinds and their `data` in `events.ndjson`: the catalogue reaches the process log, and adds no event kind or field.
- The per-method `params` and `ok` payloads of the wrapper channel: an unreachable cause or a start refusal has no channel `result`, so no frame shape moves.
- The snapshot writer and the state store's lock and append rules: the chunk replaces no mutable file and adds none.
- Hook transport tiers, the registered hook events, the flags passed to the child and the per-session settings override: no refusal cause lives in them.
- The `hook statusline` pass-through and the `budget.json` reading: it always exits 0 and holds no cause of exit 21 or 1.
- The child-environment entries, `VIOLA_BIN`, the `PATH` prefix and the R8 strip: the chunk sets nothing for a child.
- The `fuzz/` workspace, `vendor/conpty` and the tool directories under `target/` other than test homes: the chunk adds no fuzz target, vendored file or tool output.
