# tests · E2E Test Strategy — category read

## Carried
- (.andromeda/test-plan.md:560) Scenario E3's steps 1 and 3 and both verification signals must be built and pass through the CLI driver once the two verbs are served in the form the founder names at P4.
  > - **Surfaces involved:** cli
  > - **Steps:**
  >   1. Boot `overseer` and `builder`. Run `viola link overseer builder --json` twice.
  >   2. Run `viola send builder` with `VIOLA_NAME=overseer`.
  >   3. Run `viola unlink overseer builder --json`.
  > - **Verification signal:**
  >   - Exactly one `link{driver:"overseer",driven:"builder"}` record exists, because re-linking adds none.
  >   - After unlink: an `unlink` record exists.
- (.andromeda/test-plan.md:570) The body's E3 "As landed" note must be restated when the verbs land, since it names them unbuilt and held to this very entry.
  > - **As landed: owed, built in part.** `viola link` and `viola unlink` are not built (the wrapper answers both `-32601`) and are held to the route entry "Explicit session links"; the implicit link that a `send` or `answer` from another instance records is held by root cases of `tests/cli_links.rs`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:226) The detail's E3 "As landed" record of the hold must be restated against what lands (the `-32601` answers, the unbuilt steps 1 and 3, the hold), quoting the founder's word as given at P4.
  > **As landed (chunk 2026-10-10-session-links): owed, built in part.** Only the implicit link exists, so steps 1 and 3 cannot run: `viola link` and `viola unlink` are not built (the wrapper answers both methods `-32601`) and are held: their boundary widening is not ratified, and the founder's own answer of 2026-10-13, relayed verbatim by the Viola overseer, builds no explicit verb before Epoch 6 (the Epoch 6 entry "Explicit session links", after "Server verification before any frame", carries the hold; first moved from "The board: viola list" by the review card the founder approved).
- (.andromeda/test-plan-detail/e2e-test-strategy.md:226) The four standing implicit-link root cases of `tests/cli_links.rs` must stay green beside the new verbs, and the gap that no case reads a product-written `unlink` record must close with the `unlink` method's product path.
  > What step 2 leaves is held by root cases of `tests/cli_links.rs` (`binary(cli_links)`, each from a stamped home): `links_a_send_from_another_instance_links_once` (one `link{driver:"overseer",driven:"builder"}` record with `source: wrapper` directly before that send's `send-issued`, and a second issued send adds none), `links_an_answer_from_another_instance_links`, `links_a_refused_send_and_an_own_name_send_record_none` and `links_survive_a_wrapper_restart`, green on the three CI runners (run 38098238731). No case reads an `unlink` record a product path wrote, because none writes one.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:298) The raw-channel name negative owed with the verbs must be built: each listed bad `driver` / `driven` gets `-32602` and appends no record.
  > **Raw-channel `link` / `unlink` name validation** (channel frames). Frames written straight to the endpoint with `driver` / `driven` set to `../x`, `BUILDER`, `a/b`, an empty string and a 33-character name each get `-32602`, and no `link` / `unlink` record is appended.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:13) The channel cases (`tests/channel_links.rs` and the raw-frame negative) must drive the wrapper through the sync client or a raw stream with jsonschema, the endpoint read only from `snapshot.json`.
  > | ipc-internal (wrapper channel) | viola-channel sync client + interprocess 2.4.4 raw stream + jsonschema 0.57.0 | direct | The endpoint is read only from `snapshot.json`. |
- (.andromeda/test-plan-detail/e2e-test-strategy.md:10) The verbs' human result lines must be pinned by trycmd, while the `viola --help` row `handoff:  link, unlink` moves the literal in `tests/cli_output_plain.rs`.
  > assert_cmd 2.2.2 + predicates 3.1.4; trycmd 1.2.1 for human layouts other than the `viola list` board and `viola --help`, pinned by literal in `tests/cli_list.rs` and `tests/cli_output_plain.rs`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:305) If the verbs write their frame through the server check, as every other CLI frame verb does, the server-identity negative and its control (today six verbs) must cover `link` and `unlink` in both modes with the same verdicts.
  > **Server identity** (`#![cfg(not(target_os = "macos"))]`, IPC client-side server verification). `tests/security_negatives_server_identity.rs` `security_negative_cli_verb_writes_no_frame_to_a_server_that_is_not_the_recorded_wrapper` runs `send`, `wait`, `last`, `answer`, `pause` and `release`, each under `--json` and in human mode, against a same-user server bound on the recorded endpoint while `snapshot.json` records another live process (the fake agent's `--hold-stdout-internal` holder): exit 21 with the server-verification cause's `--json` document (stderr empty), hint and `process-exit` line (Exit-cause matrix), no `channel-request` line, no frame reaching the server, and no pid, start time, path or endpoint in any output or line (`digit_runs`).
  >     - Its control `security_negative_cli_verb_reaches_the_recorded_wrapper` runs each of the six verbs, in both modes, against the recorded wrapper itself, which receives one frame each.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:271) A server-verification refusal of either verb must give that cause's own hint, log line and exit-21 document.
  > Its hint is its own, cause `server-verify` in `src/human/hints.rs` (the design-system surface cli-send string, byte for byte), apart from a dead instance's not-running hint; its log line is `process-exit{subject:"self", exit_code:21, detail:"server-verify-failed", during:"connect"}` at WARN in `cli-<name>.ndjson`; its `--json` document is every exit-21 cause's `{"v":1,"error":"instance-unreachable","detail":null}`.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:265) Every refusal case the verbs add must assert the exit code, the `--json` document and a last `hint:` line naming its cause, no hint shared across causes.
  > Every case asserts the exit code, the `--json` document, and a last stderr `hint:` line that names that cause. A shared hint across causes fails the table, and exact strings are pinned with trycmd once the product defines them.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:267) If the founder takes the instance check ahead of the snapshot read (the second option), a strict-modes refusal of either verb is an exit-21 cause and must be asserted as one.
  >   - Exit 21 (`instance-unreachable`) causes:
  >     - the home fails strict-modes (Unix 0770 home; Windows synthetic-descriptor stub)
- (.andromeda/test-plan.md:437) A strict-modes refusal of a CLI verb other than `run` / `mcp` / `viola revive` must be exit 21, which binds `link` / `unlink` under any form that checks strict-modes.
  > strict-modes refusals give exit 1 for `run` / `mcp` / `viola revive`, hook 0 with no output, and CLI 21 for every other verb.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:289) No hint the verbs print may contain `release`.
  > - Driver-facing hints never contain `release`: this holds for every refusal row in the table (`human-typing`, `budget-paused`), not only in Paths 5 and 6.
- (.andromeda/test-plan-detail/e2e-test-strategy.md:254) The `-32602` errors and every `--json` document of the new methods and verbs must pass the per-test sanitization: no absolute path, serde path or `Caused by`, and every `-32603` exact.
  > - **Error sanitization and secret scan:** after every E2E test, the `events` support helper checks:
  >   - CLI `--json`, MCP `isError` and channel `error.data` contain no absolute path (drive letter, `/home/`, `/Users/`, the temp home prefix), no serde path and no `Caused by`
  >   - every -32603 is exactly `"internal error"` / `null`
- (.andromeda/test-plan-detail/e2e-test-strategy.md:234) E5's link leg, written for the implicit link alone, must be restated once a product path writes an explicit `link` and an `unlink`, and the replay of a product-written `unlink` read where the leg names it.
  > The link leg landed at chunk 2026-10-10-session-links, on the implicit link alone (no `viola link` verb exists; the explicit verbs are not built): `e5_the_replay_reads_the_link_the_wrapper_wrote` (`tests/cli_links.rs`) replays the link a wrapper recorded for a `send` from another instance, equal to the snapshot's `links`, with all three skip counts 0, beside `state_replay_links_hold_the_first_since_until_an_unlink_and_a_later_one_after_it` and `state_replay_a_snapshot_cut_short_is_replayed_with_the_logged_link`.
- (.andromeda/test-plan.md:106) No `link` / `unlink` MCP tool may be added: `tools/list` stays exactly five, so `v1-13`'s MCP half cannot be met by a new tool.
  > `tools/list` is exactly the five tools, never `release` / `pause` / `link` / `unlink`, and negative inputs must violate the advertised schema, which is advisory.
- (.andromeda/test-plan.md:610) Where `v1-13` reads an MCP result beside the `--json` one, the two must carry the same `{refusal, detail}` for the same refusal.
  > - **Cross-surface parity:**
  >   - for the same refusal, the MCP `structured_content` `{refusal, detail}` equals the CLI `--json` `{refusal, detail}`
- (.andromeda/test-plan.md:868) Name validation, `already_linked` and the `-32602` cases belong at the unit and channel tiers; E3 asserts only the cross-process behaviour, each listed surface with its own driver step.
  > - NEVER assert at E2E a case that a lower layer can hold. Parser, refusal-order, threshold and clock cases belong in §4, contract shapes in §5, and §6 asserts only cross-process behaviour (§2 pyramid).
  > - NEVER assert a multi-surface critical path on one surface only. Every surface a §6 scenario lists needs its own driver step and transition signal.
- (.andromeda/test-plan.md:895) The E3 cases must wait on logged signals, read no child screen, and run the wrapper under std::process with a `Drop` guard.
  > - NEVER use `sleep(N)` for synchronization. Wait for an explicit signal: an `events.ndjson` offset, `viola wait --after` or a status field.
  > - NEVER parse the rendered child screen (the vt100 grid, PTY master output) for content. Verdicts come from hook events, `events.ndjson`, channel payloads and the fake-agent receipt (R7).
  > - NEVER detect child exit by reading master EOF; use `child.wait()` on the handle (ConPTY).
  > - NEVER use `assert_cmd` `.assert()` on the long-lived `viola run` wrapper. It blocks until exit; use std::process + a `Drop` guard.

## Bindings
- **Raw-channel name negative ↔ security-plan §Input validation**: the `-32602` for a bad `driver` / `driven` is the `ViolaName::try_new` rule, and the scope's second CARRY (a logged link name parsed again before it is shown) rides the same validator (.andromeda/test-plan-detail/e2e-test-strategy.md:298).
- **Server-identity negative ↔ security-plan §Authentication & Authorization (server verification before any frame)**: the six-verb case and its control grow to the two verbs if they write through `client::connect`; macOS stays the recorded gap (.andromeda/test-plan-detail/e2e-test-strategy.md:305).
- **Exit-21 hint and log line ↔ design-system cli-send string and obs `process-exit`**: the server-verify hint is byte-for-byte the design string and its line the obs WARN `process-exit` (.andromeda/test-plan-detail/e2e-test-strategy.md:271).
- **`--help` literal ↔ layout-templates §Output structure — `viola --help`**: the `handoff:  link, unlink` row moves the `HELP` literal pinned in `tests/cli_output_plain.rs` (.andromeda/test-plan-detail/e2e-test-strategy.md:10).
- **E3 ↔ body §1 expansion list and §6 E3 summary**: the detail's E3 record and the body's E3 steps and "As landed" note move together (.andromeda/test-plan.md:570).
- **E5 link leg ↔ §5 On-disk / viola-state replay**: the replay units `state_replay_links_*` stand beside the E2E link leg that a product `unlink` now reaches (.andromeda/test-plan-detail/e2e-test-strategy.md:234).
- **MCP five tools ↔ verification-matrix `v1-13`**: the claim's MCP half meets the closed tool list, a question for P3 (.andromeda/test-plan.md:106).
