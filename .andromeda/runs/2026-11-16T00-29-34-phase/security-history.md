@history_item n=1 marker=2026-10-04-wait-and-last
**2026-10-04-wait-and-last** — (the fifth dated gap entry) The amendment that first put CLI `viola wait` / `last` on the wire. It added the `wait` / `last` params clause (`after` / `timeout_ms` absent or `u64`, `from` follows the `send` rule, unknown fields ignored, else `-32602` `data: null`), `<name>` through `ViolaName::try_new` with `--after` / `--timeout-ms` as clap `u64`, and the events reader `read_from` behind `take(MAX_FRAME + 1)`, which skips and counts over-long and non-object lines. This is the origin of contract/channel-frames/wait-last-params, contract/wrapper-channel-methods/wait and decision/events-reader-bounded-lines. A documented or changed wake condition has to keep these params and reader bounds as they are.

@history_item n=2 marker=2026-10-19-overseer-switch-to-viola
**2026-10-19-overseer-switch-to-viola** — (question option text and `background_tasks`) Stop's `background_tasks` is carried only as the array's length: `null` when absent, `null` or not an array, and a non-array also writes the drift entry `{"path":"background_tasks","expected":"array"}`. A question option's description and preview cross into `events.ndjson`, `hook.dialog`, CLI `wait --json` and MCP `wait` as the founder's own widening. This is the origin of decision/stop-background-tasks-count-only and decision/question-option-text-crosses-boundary. The chunk's wake on a turn end held by a background task (the `turn-ended` line carrying `background_tasks`, or the `question` it raises) rests directly on these. Waking on that count must not read any element or carry more than the length.

@history_item n=3 marker=2026-11-15-api-error-turn-end
**2026-11-15-api-error-turn-end** — (Input Validation, Data Protection and Authentication & Authorization rows) The previous chunk. It changed four bound rows:
- decision/hook-unknown-event-silent-exit now counts ten events (StopFailure added).
- contract/channel-frames/hook-event-revalidation holds a turn-ended `cause` to the closed set (`api-error` alone).
- decision/api-error-turn-end-text-dropped: an API-error turn end carries only null `last_assistant_message` / `background_tasks` and the cause word, which crosses into `events.ndjson`, `wait --json` and MCP `wait`. Guard `hook_stop_failure_error_text_reaches_no_event_line_or_snapshot`, provisional.
- decision/api-error-classes-adapter-only: classes stay inside the adapter.
It also added the StopFailure exception to decision/unverified-cli-gate (no ledger row, provisional, owned by the 2.1.325 fixture set). All of these are PROVISIONAL, delegate overseer, with the founder's word still owed. Any wake rule over `turn-ended` (API-error ends included) must keep the text dropped and the cause closed. The StopFailure live reading is explicitly not this chunk's.

@history_item n=4 marker=2026-09-27-hooks-to-normalised-events
**2026-09-27-hooks-to-normalised-events** — (`hook.event` served) The wrapper re-validates `hook.event`: the five hook kinds, object `data`, `prompt-submitted` `text` a string and `origin` harness|human. An invalid event is not appended. Hook stdin uses `take(MAX_FRAME + 1)` with closed fail-open details. This is the base of contract/wrapper-channel-methods/hook-event, decision/invalid-hook-frames-dropped and contract/hook-stdin/*. The event kinds the wake condition classifies come in through this boundary. A new or reclassified kind must go through this re-validation.

@history_item n=5 marker=2026-09-29-fake-agent-drift-contract
**2026-09-29-fake-agent-drift-contract** — (the cross-session tag filed as harness origin) A prompt is classified `harness` only by four compiled literals matched on its raw start with no trim, `<task-notification>` among them. The ratified side effect: the escaped cross-session form classifies `harness`. This is the origin of decision/harness-prefix-literals. The chunk's "notification gives no false wake" clause is about lines this classifier files. Any wake or no-wake rule that keys on a notification's origin must use these literals and never build prefixes from runtime or upstream text.

@history_item n=6 marker=2026-10-10-self-healing-state
**2026-10-10-self-healing-state** — (own state files on read) The events reader counts `Skipped{unknown_kinds, unknown_fields, torn_lines}` and does not log them. An unknown kind is not returned, a reader never writes, and the next append heals a torn last line under the lock. This is the origin of decision/readers-tolerate-torn-lines, decision/state-readers-never-write and decision/events-reader-bounded-lines. The wait feed reads through this reader, so a wake rule must not wake on an unknown kind or a torn line, and the feed must never write.

@history_item n=7 marker=2026-10-10-session-links
**2026-10-10-session-links** — (the link-set restore reads the log) A start reads `events.ndjson` twice, `WaitFeed::rebuild` and then `LinkSet::restore`, both through the one reader. `link` / `unlink` became known kinds. This is a precedent for how the wait feed's rebuild sits on the shared reader: a change to which kinds the feed returns or parks on must leave the restore pass and the reader's counts unchanged.

@history_item n=8 marker=2026-10-17-mcp-server-for-drivers
**2026-10-17-mcp-server-for-drivers** — (MCP tool input validation and error bodies as landed) Each MCP handler takes the raw argument object, never rmcp's typed deserialisation. `wait`'s `after` / `timeout_ms` are absent, `null` or `u64`, and `timeout_ms` is clamped to `WAIT_CAP` 50 000 ms, which is also its default. A failure is `invalid-params` with a closed field word and no frame. This is the origin of contract/mcp-tool-inputs/timeout-ms, contract/mcp-tool-inputs/invalid-params, decision/mcp-handlers-validate-raw-arguments and contract/mcp-errors/-32603. If the chunk touches the MCP `wait` tool in `crates/viola-mcp/src/tools.rs`, it must keep these.

@history_item n=9 marker=2026-09-26-local-linux-pre-push-gate
**2026-09-26-local-linux-pre-push-gate** — (the `FAKE_AGENT_PUMP_DELAY_MS` carve-out) This ratified the first fake-agent test seam: `fake-agent`-only, capped, compiled out of release, configures nothing, disables no control. It added the NEVER for another seam without its own Decisions Log entry. This is the origin of decision/test-seams-compiled-out, decision/test-seams-fake-agent-only and decision/new-env-read-needs-log-entry. This is the trap if the chunk extends the fake agent's turn scripts (`src/bin/viola-fake-agent.rs`, `fixtures/fake-scripts/`): a new env read or seam to drive a background-task or notification turn is a boundary widening needing its own entry.

@history_item n=10 marker=2026-09-28-hook-perf-gate
**2026-09-28-hook-perf-gate** — (test seam `FAKE_AGENT_HOOK_PANIC`) The second seam, ratified live by the founder. Its precedent is that each further seam, like each G2 exemption, needs its own entry and ratification. Read it together with n=9 when fake-agent scripting for the wake cases is planned.

@history_item n=11 marker=2026-10-11-the-board-viola-list
**2026-10-11-the-board-viola-list** — (the logging floor) Human `wait` / `last` output keeps `\n` and `\t` and escapes the other C0/C1 controls, while `list` rows escape every control as `\xHH`. This is the origin of decision/human-output-escapes-controls and decision/escape-terminal-controls. Any change to `src/human/wait.rs` for the wake behaviour must keep this escaping.

@history_item n=12 marker=2026-09-28-capability-ledger-and-viola-verify
**2026-09-28-capability-ledger-and-viola-verify** — (`MAX_FRAME` consumers extended, verify's CI split) The real CLI runs only locally, and `viola verify` runs in CI only against the fake agent; its real-CLI probe and `--record` run only locally. This is the origin of decision/real-cli-local-only and decision/ci-no-secrets-no-real-claude. The carried CI red sits in the `stamped_home` fixture's `viola verify` (`tests/support/verify.rs`), so whichever option the chunk picks, sharing or copying a stamped home or resizing the `ci` kill, keeps verify on the fake agent in CI.

@history_item n=13 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (`ledger/stamps.json` checked before `ledger/` is made, and `verify`'s home tree before its log) `update_stamps` runs `check_stamps` before `create_private_dir(ledger/)` and again under its lock. `verify` runs `check_home_tree` on the home, `ledger/`, `bin/` and `plugin/` before its log. The ban: never run the check after a create, mode set or log open that narrows the path. This is the origin of decision/verify-home-tree-check, decision/stamps-strict-check-first, decision/strict-check-before-narrowing and decision/state-tree-strict-modes-before-use. The trap for option (a), a stamped home stamped once and shared or copied: the copy must carry owner-only modes (0700 dirs, 0600 files, the pinned `viola` 0700) or every later `run` / `verify` refuses it, and the test must not chmod after the fact to pass the check.

@history_item n=14 marker=2026-10-26-home-and-code-bearing-file-integrity
**2026-10-26-home-and-code-bearing-file-integrity** — (the six CLI frame verbs share `link` / `unlink`'s instance check; `viola mcp`'s frame tools check the instance before `live_endpoint`) CLI `wait` and the MCP `wait` tool run `check_instance` before any role file, log or snapshot value. A refusal sends no frame and writes no line. This is the origin of decision/cli-frame-verbs-strict-modes-first, decision/client-checks-before-frame, decision/mcp-frame-tool-refusals and contract/strict-modes-refusal/mcp-tool. The wake changes in `src/cmd/wait.rs` and `tools.rs` sit behind this order and must not move it.

@history_item n=15 marker=2026-10-07-test-homes-off-the-contended-volume
**2026-10-07-test-homes-off-the-contended-volume** — (test-side removals reach the dev host's tmpfs backing through the home link) A test-side removal under `target/e2e-home` takes only a directory the test side itself created: by its owner record, its own `TempDir` or its session name prefix. The keepers never follow an unsafe link. This is the origin of decision/e2e-home-removal-own-dirs-only and decision/test-removal-own-dirs-only. The trap for option (a): a stamped home shared across tests must be owned and removed only by the side that created it, never swept by pattern.

@history_item n=16 marker=2026-10-20-epoch-5-cleanup
**2026-10-20-epoch-5-cleanup** — (the e2e-home keepers refuse a backing below the headroom floor) The keepers also refuse a linked base under the 9 GiB floor, with a fixed message and a compiled constant, and no env var, flag or config key sets it. A shared or copied stamped home in the test support must still go through these keepers and add no setting.

@history_item n=17 marker=2026-10-31-sanitised-error-surfaces
**2026-10-31-sanitised-error-surfaces** — (the clients' screen of a peer's reply; channel-response's closed detail word) `sanitised_fault` and `is_code_word`: a client reads a refusal or string detail only as a 1–64 byte code word and a fault only through the sanitised reader. The three `-32602` faults log `unsupported-version` / `invalid-params` / `release-from-driver` with no param name or value. This is the origin of decision/client-reads-closed-values-only, decision/client-reply-code-words-only, decision/reply-code-words-validated, decision/channel-faults-sanitised, contract/channel-response-log/detail and contract/channel-response-detail/invalid-params. A `wait` result or refusal changed by this chunk reaches CLI `--json` and MCP only through these screens.

@history_item n=18 marker=2026-10-31-exit-cause-code-catalogue
**2026-10-31-exit-cause-code-catalogue** — (exit 21 names its cause in detail; the catch site's document under --json) Exit 21 under `--json` reads `{"v":1,"error":"instance-unreachable","detail":"<cause>"}` from a closed set, MCP the same without `v`. The catch site prints `{"v":1,"error":"internal-error","detail":null}`. These are the origin of the contract/instance-unreachable-cause/*, contract/cli-instance-unreachable/json-document, contract/mcp-iserror/* and contract/internal-error-output/json rows. `viola wait` is one of the frame verbs these documents cover.

@history_item n=19 marker=2026-11-01-unix-endpoint-and-home-hardening
**2026-11-01-unix-endpoint-and-home-hardening** — (`socket-dir-refused`, the sixth cause; the client's peer euid check and the order at a seam) The order at every client seam is the directory check, the connect, the peer euid decision, then the server verification. A refused euid reads `server-verify-failed`. The ban: never add an env, flag or feature seam to force a refusing peer-euid decision. These are the origin of contract/instance-unreachable-cause/socket-dir-refused, contract/instance-unreachable-detail/server-verify-failed and decision/server-verified-before-any-frame. CLI and MCP `wait` take this order unchanged.

@history_item n=20 marker=2026-10-27-bounded-inputs-at-every-boundary
**2026-10-27-bounded-inputs-at-every-boundary** — (the bounded-reader census) `product_readers_are_each_bounded_or_named` requires every product reader call to be bounded in its statement or named as an exception. serde_json's `unbounded_depth` is banned in `deny.toml`. These are the origin of decision/bounded-readers-census, decision/serde-json-unbounded-depth-banned and decision/serde-json-bounded-depth. Any new or changed read in the wait feed or the test support's product paths must pass the census.

@history_item n=21 marker=2026-11-14-status-line-interception-removed
**2026-11-14-status-line-interception-removed** — (Threat Model Summary: 4 rows) contract/cli-state-root/home-flag and contract/cli-state-root/viola-dir now choose only the state tree, snapshot, endpoint and ledger stamps (`statusline_command` gone). These are the current wording of the two bound rows `--home` / `VIOLA_DIR` that the stamped-home test support passes.

@history_item n=22 marker=2026-10-18-plugin-precedence-live-readings
**2026-10-18-plugin-precedence-live-readings** — (the VIOLA_DIR home step ratified) The CLI verbs' `VIOLA_DIR` step and canonicalisation, `mcp` included, are a founder-ratified widening. This is the origin of decision/cli-home-resolution-order, which every `wait` call over a test home resolves through.

@history_item n=23 marker=2026-09-28-mutation-testing-to-the-epoch-boundary
**2026-09-28-mutation-testing-to-the-epoch-boundary** — (the CI mutation legs retire) CI runs no mutation job; mutation belongs to the epoch-boundary audit. This is the basis of decision/mutation-testing-at-audit, and it matches the scope's "no mutation run": each new guard carries a one-off remove-the-guard control instead.

@history_item n=24 marker=2026-11-05-linux-only-code-and-ci
**2026-11-05-linux-only-code-and-ci** — (CI runs one ubuntu-latest leg) `ci.yml` runs the one-leg `[ubuntu-latest]` matrix. The carried red (`path1_run_start_one_home_in_order` past the `ci` profile's 20 s kill) is on that leg, so sizing the class's `ci` kill or moving the stamping out touches this one runner. No secret or real `claude` enters CI (decision/ci-no-secrets-no-real-claude).

@history_item n=25 marker=2026-11-15-live-round-on-the-trimmed-product
**2026-11-15-live-round-on-the-trimmed-product** — (Authentication & Authorization: 1 row; Data Protection: 1 row) It added decision/live-round-rig-keys-own-pty and decision/live-session-screens-kept-local. If the chunk asks for a live `claude` 2.1.325 start for a wake clause, these rows are the standing precedent: rig keys only into its own pty, screens kept local, and a widening needing the founder's word for the chunk.
