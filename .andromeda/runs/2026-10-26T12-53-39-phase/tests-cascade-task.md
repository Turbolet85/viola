# Task — tests · 2026-10-26-explicit-session-links

## The chunk
Serves the wrapper channel methods `link` and `unlink` (today both answer `-32601`) and adds the CLI verbs `viola link <driver> <driven>` and `viola unlink <driver> <driven>` with `--json`. The verb's instance is the driven one. Which trust-boundary form the verbs take is the founder's to pick at P4, and the scope plans every form up to that point.
`link` records the pair and answers `{since, already_linked}`. `unlink` removes the pair, answers the empty object and appends nothing for an absent pair. A `driven` that is not the serving wrapper's name, or a `driver` equal to `driven`, is `-32602`. The three human result lines (`link … since …Z`, `link … already linked`, `unlink …`) re-parse each logged link name through `ViolaName::try_new`.
Gives `LinkSet::remove` its product caller and drops its `dead_code` allow. Adds the `handoff:  link, unlink` row to `viola --help` and moves the `HELP` literal tests with it.
Builds the owed root cases: test-plan §6 Scenario E3 steps 1 and 3 and the raw-channel `link` / `unlink` name negative, with `tests/channel_links.rs`. It also covers the `link` half of `v1-13`: every human `link` line derivable from its machine result. Whether the MCP half can be proved is a question for P3.

## Reached files
- src/run/links.rs · edits `LinkSet::remove` (its product caller, the `#[cfg_attr(not(test), allow(dead_code))]` removed)
- tests/channel_links.rs · edits the root cases of the two channel methods (new file)
- tests/cli_links.rs
- tests/cli_output_plain.rs · edits the `HELP` literal, `help_is_the_grouped_verb_table`, `help_lists_only_served_verbs`
- viola-0.1.0/verification-matrix.json · edits `v1-13` (its `link` half)

## Needs
- The wrapper channel's `link` and `unlink` methods: served instead of `-32601`, result shapes `{since, already_linked}` and the empty object, schema-validated result, refusal and error shapes, and the `cursor` rule for the integration suites over a real endpoint.
- The `-32602` invalid-params cases on these two methods: a `driven` that is not the serving wrapper's name, and a `driver` equal to `driven`.
- The event kinds `link` and `unlink` in `events.ndjson`. Re-linking adds no `link` record. An `unlink` of an absent pair appends no line. The event-line round-trip of both kinds.
- Test-plan §6 Scenario E3 (`link` / `unlink`): steps 1 and 3 are built, and the scenario's "As landed: owed, built in part" note is restated to match what lands. Step 2's implicit link through `send` with `VIOLA_NAME=overseer` is already held in `tests/cli_links.rs`.
- The security-control negatives suite's raw-channel `link` / `unlink` name validation case, still owed: non-`ViolaName` names sent as raw frames are refused.
- The server identity check before the first frame (§1 IPC coverage trigger; the security negatives suite's `tests/security_negatives_server_identity.rs` CLI frame verb case). On Linux and Windows the two new verbs take it through `client::connect` like `send` does, and a server that is not the recorded wrapper gets no frame and an exit 21 `instance-unreachable`. macOS remains the recorded gap.
- The liveness-only pre-check the frame verbs run before connecting (snapshot pid + start time alive, heartbeat live, `endpoint` present; else exit 21).
- The CLI `--json` / exit-code contract for the two verbs: one `{"v":1,…}` document per exit code, and the CLI `--json` result mirroring the channel `result`.
- On-disk strict-modes refusal mapping for a CLI verb (CLI exit 21), which binds the verbs if the form picked at P4 puts `viola_state::strict::check_instance` ahead of their snapshot read.
- The three human result lines and the `v1-13` derivability check: each human `link` line is derivable from its `--json` machine result, a human layout pinned by literal, and no SGR under non-TTY, `NO_COLOR` or `TERM=dumb`.
- Human-mode C0/C1 escaping of a logged link name shown in a result line, and its re-parse through `ViolaName::try_new`.
- The `viola --help` literal: the `handoff:  link, unlink` row between `wheel:` and `setup:`, pinned by the `HELP` literal in `tests/cli_output_plain.rs` (help shows only served verbs).
- The MCP surface's `tools/list` stays exactly `send, wait, last, answer, list`, never `link` / `unlink`. The scope adds no MCP link tool.
- The wrapper's `LinkSet`, restored from the log at start (the replay's link half, `replay::apply_link_line`), must honour an `unlink` line now that a product path appends one. The snapshot's `links` items `{driver, driven, since}` lose the removed pair.
- Driver-facing hints, including the exit-21 hints of the new verbs, never contain `release`. The exit-cause matrix's one-hint-per-cause rule applies to any cause the verbs add.
- Error sanitization on the new paths: channel `error.data` and CLI `--json` carry codes and fixed messages only, with no path, pid, start time or endpoint. A `-32603` is exactly `"internal error"` / `null`.
- Paths 2 and 4's implicit link (an issued `send` or a taken `answer` from another instance) keeps recording exactly one `link` and stays consistent with an explicit `link` of the same pair (`already_linked`).
- The unit layer for the root bin: refusal ordering and the exit-code mapping of the two verbs, plus `LinkSet::remove`'s unit cases, which now sit beside a product caller.

## Standing conventions
- S1 · Integration binaries named `<surface>_<topic>.rs` (`cli_`, `channel_`, `security_negatives_` in root), so the layer filtersets select them · §2 Test Strategy / .andromeda/test-plan-detail/test-strategy.md
- S2 · Test functions named snake_case `<subject>_<condition>_<expected>`; rstest `#[case::label]` tables carry readable labels · §2 Test Strategy, §4 Unit Test Strategy / .andromeda/test-plan-detail/unit-test-strategy.md
- S3 · Each test gets a fresh tempfile home at a not-yet-existing path under `target/e2e-home`, and started wrappers sit in a `Drop` guard; nothing is shared between tests · §5 Integration Test Strategy / .andromeda/test-plan-detail/integration-test-strategy.md
- S4 · The endpoint is taken only from `snapshot.json`; never interprocess `try_overwrite` or a reused endpoint name · §5, §11 Integration
- S5 · Env goes per child through `Command::env`, never `std::env::set_var`; `.env_clear()` re-adds `LLVM_PROFILE_FILE`; a test-started `viola` drops `VIOLA_NAME` / `VIOLA_DIR` / `VIOLA_BIN`, with a path oracle on `resolved(home)` · §5, §11 Integration
- S6 · Waits key on `events.ndjson` byte offsets through the complete-lines reader `tests/support/ndjson.rs`, never a sleep; root suites share `tests/support/` helpers and copy none · §2 Test Strategy, §11 E2E
- S7 · Channel frames are validated by jsonschema validators built from the product's schemars schemas · §5 Integration Test Strategy / .andromeda/test-plan-detail/integration-test-strategy.md
- S8 · On-disk product state comes only from real `viola` runs, so `snapshot.json` and `stamps.json` are never hand-written; a home viola created starts from `stamped_home` · §7 Test Data & Fixtures, §11 Test Data
- S9 · After every E2E test, the error-sanitization and secret scan and the diagnostics schema conformance run · §6 / .andromeda/test-plan-detail/e2e-test-strategy.md
- S10 · Zero flakiness: nextest `retries = 0`, no `#[ignore]` parking, and coverage gates held per OS · §10 Quality Gates & Coverage Targets / .andromeda/test-plan-detail/quality-gates-coverage-targets.md
- S11 · OS-branch behaviour is verified on its own CI runner, and a Linux-only local run proves nothing for Windows or macOS · §1 multi-os-compat trigger, §11 Test Strategy

## Not needed
- Path 1 (`run` start sequence), Path 3 (`wait` / `last` readback), Path 6 (budget governor) and Path 7 (unverified CLI version gate): the chunk changes no start step, wake set, budget reading or version gate.
- Path 5's wheel and `release` flow: `link` changes no wheel holder, and `release --budget` landed with an earlier chunk.
- Scenario E1 (R5 unwrapped hooks) and Scenario E2 (R8 `CLAUDE*` env strip): no hook or child-env change.
- Scenario E5's steps 1–3 (snapshot corruption → replay recovery): they remain a CARRY owed to the route entry "Home and code-bearing file integrity".
- `viola verify`, the capability-ledger rows, recorded `fixtures/claude/` sets, the contract suite and fixture hygiene: a link is recorded from viola's own parameters, with no ledger row, verify child, re-stamp or live `claude` start.
- The fake agent's behaviour contract (turn scripts, framing replays, exit SIGHUP disposition): the chunk only boots it as it stands.
- The `list` row-integrity suite and the `viola list` board literal: v1 `list` shows no link column.
- The hook fail-open matrix and the hook perf budgets (hyperfine `max` rows): no hook path changes.
- The chaos suite and the property suite's parser surfaces (paste text, framing, statusline, agents, vt100, `prompt-submitted`): no parser or fault seam is added.
- The mutation gate: it runs only at the epoch boundary and on demand, never per chunk.
- The ConPTY vendor verification and the nightly fuzz runs: nothing vendored or fuzzed changes.
