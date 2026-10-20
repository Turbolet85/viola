proposals:
  - detector: D-tests-count-literals
    severity: warning
    section: "§9 CI Integration (Lint row)"
    change: "The orphans probe's last line reads `orphans-check probes: 2/2 fired, control clean` (two planted `probe_case fire` rows), not `1/1 fired`."
    sidecar: "§9 Lint row: the orphans probe line corrected 1/1 → 2/2 fired, as the gate's own log reads (chunk 2026-10-20-epoch-5-cleanup-ii)."
    rationale: "Report §Changes → Spec claims disproved: the gate's log at this chunk (`7.3.log`) reads `orphans-check probes: 2/2 fired, control clean`; `scripts/orphans-check.sh` is unchanged since 844de976 and already held two `probe_case fire` rows, so the master's 1/1 is a pre-existing stale literal; the report names test-plan.md:1027 as the one master site."
    basis: "test-plan.md:1027; scripts/orphans-check.sh at 844de976 lines 127-133 (as the report states)"
  - detector: D-tests-count-literals
    severity: warning
    section: "§9 CI Integration (the Lint errors bullet under the CI table)"
    change: "The lint-failure bullet's probe clause reads «its probe not ending `2/2 fired, control clean`» in place of `1/1 fired, control clean`."
    sidecar: "§9 Lint errors bullet: the orphans probe's expected last line 1/1 → 2/2 fired, matching the Lint row."
    rationale: "Same retired literal restated in the failure-trigger bullet; the probe prints 2/2 (report §Changes → Spec claims disproved), so a single-site apply at the Lint row would leave this trigger stating a line the gate never prints."
    basis: "test-plan.md (the Lint errors bullet in §9; the report states no line for it)"
    dependent-of: D-tests-count-literals
  - detector: D-tests-coverage
    severity: warning
    section: "§7 Test Data & Fixtures (Fixture hygiene, «Today the walk …» bullet)"
    change: "The walk also covers the MCP exchange class `fixtures/claude/<cli-version>/Mcp.<variant>.json` (today `2.1.302`'s `Mcp.tools-list-1.json`) against `schemas/claude-mcp-fixture.v1.json` (`v` const 1; `client` an array of JSON-RPC 2.0 frames that contains a `tools/list` request with an `id`; `results` keyed by method, `tools/list` required with a `tools` array of named tools) with the path, username and email checks (`mcp_fixtures_pass_hygiene`; planted `planted_clean_mcp_exchange_passes`, `planted_dirty_mcp_exchange_is_rejected`, `planted_mcp_schema_violation_is_rejected`; chunk 2026-10-20-epoch-5-cleanup-ii)."
    sidecar: "§7 Fixture hygiene: the walk's class list gains the `Mcp.` exchange class, its schema and its planted cases (chunk 2026-10-20-epoch-5-cleanup-ii)."
    rationale: "Report §Changes → Schema / config: a new fixture class `Mcp.<variant>.json` walked by tests/contract_fixture_hygiene.rs against the new schema plus an email check; Symbols → New tests names the 11 hygiene cases. The report's expected amendment «test-plan §7 (the `Mcp.` class in the hygiene walk)» names :970 stale by omission."
    basis: "test-plan.md:970; tests/contract_fixture_hygiene.rs:405-419, :439-446, :448-461, :463-479; schemas/claude-mcp-fixture.v1.json:1-47"
  - detector: D-tests-coverage
    severity: warning
    section: "§7 Test Data & Fixtures (Fixture hygiene, the «One run-time directory walk» bullet)"
    change: "The `claude_fixtures` walk leaves `Mcp.` files out (`is_mcp`), and a second run-time walk `mcp_fixtures` in the same file takes them; the walker test `claude_fixtures_walks_every_version_dir_and_only_json_files` holds an `Mcp.` row."
    sidecar: "§7 Fixture hygiene: `claude_fixtures` now excludes `Mcp.` files; the `mcp_fixtures` walk and the walker test's `Mcp.` row named."
    rationale: "Report §Changes → Schema / config: «`claude_fixtures` now leaves `Mcp.` files out (`:63-68`)»; Symbols → New tests: the walker gains an `Mcp.` row (`:334`, `:366-369`). The bullet's «One run-time directory walk … does two checks per fixture» no longer describes every fixtures/claude file."
    basis: "test-plan.md:966; tests/contract_fixture_hygiene.rs:63-65, :76-79, :366-369"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§2 Test Strategy (Contract row)"
    change: "The jsonschema clause adds «every `fixtures/claude/*/Mcp.*.json` MCP exchange to `schemas/claude-mcp-fixture.v1.json`», and «every other `fixtures/claude/*/*.json` hook payload» excludes the `Mcp.` files as it excludes `StatusLine.` and `Agents.`."
    sidecar: "§2 Contract row: the schema enumeration gains the `Mcp.` class and its schema."
    rationale: "Report §Changes → Schema / config and Expected amendments (`fixture-classes` site test-plan.md:379 ×3): as written, «every other fixtures/claude/*/*.json hook payload to claude-fixture.v1.json» would cover `Mcp.tools-list-1.json`, which is checked against its own schema."
    basis: "test-plan.md:379"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§7 Test Data & Fixtures (Seed strategies table)"
    change: "Add a row «Recorded MCP exchange»: `fixtures/claude/<cli-version>/Mcp.<variant>.json` (`Mcp.tools-list-1.json` in `2.1.302`) recorded by the test-only bin `viola-mcp-tap` (crates/viola-e2e): `record <capture> <viola exe> <args…>` sits between one live `claude -p` start (prompt on stdin) and a direct spawn of `viola mcp`, one append per direction-tagged line through `take(MAX_FRAME)` into a 0600 capture; `fixture <capture> <out>` keeps the client frames through the first `tools/list` request and each request's result keyed by method, applies the `--record` scrub (`ledger::scrub`) and refuses through `ledger::screen_fault` with a closed code (`home-path` · `absolute-path` · `username` · `email`) and the file name, never content; a boundary widening the founder answered himself (chunk 2026-10-20-epoch-5-cleanup-ii, inputs#I2 Q1, inputs#I5 Q1) | per-suite, read-only."
    sidecar: "§7 Seed strategies: new row for the `Mcp.` exchange recorded by the test-only `viola-mcp-tap` (record + fixture, the `--record` scrub and four closed codes)."
    rationale: "Report §Changes → Symbols (New test-only bin `viola-mcp-tap`), Schema / config (new fixture class), Counts moved (the 2.1.302 set's `Mcp.` file «is the tap's», not verify's), Decisions (the founder's boundary widening). §7 names only `viola verify --record` as the source of fixtures/claude data."
    basis: "test-plan.md:936; crates/viola-e2e/src/bin/viola-mcp-tap.rs:77-129, :191-224, :46-55"
  - detector: D-tests-coverage
    severity: warning
    section: "§7 Test Data & Fixtures (Fixture library paragraph)"
    change: "The sentence listing what lives in `fixtures/claude/<cli-version>/` adds «and the `viola-mcp-tap`-recorded MCP exchange `Mcp.<variant>.json`»."
    sidecar: "§7 Fixture library: the `Mcp.` exchange named among the fixtures/claude contents."
    rationale: "Same provenance claim restated: the paragraph names only recorded `viola verify` fixtures and RELAYED captures as the contents of fixtures/claude; report §Changes → Counts moved says the 2.1.302 set is no longer «recorded whole by one `viola verify --record` round»."
    basis: "test-plan.md (§7 Fixture library paragraph; the report states no line for it)"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§7 Test Data & Fixtures (Fixture hygiene, the recorder bullet)"
    change: "After the `viola verify` recorder rule, add: the test-only `viola-mcp-tap fixture` applies the same scrub (home → `~`, user word → `<user>`) and refuses a dirty exchange whole with the same four closed codes, naming the file only."
    sidecar: "§7 Fixture hygiene: the recorder rule extended to the tap's `fixture` step (same scrub, same closed refusal codes)."
    rationale: "Report §Changes → Symbols (the tap's `fixture`: `ledger::scrub`, then `ledger::screen_fault`, closed code and file name) and Schema / config («the tap applies the `--record` scrub and its four closed codes»); the bullet names the verify recorder as the only writer bound by the scrub."
    basis: "crates/viola-e2e/src/bin/viola-mcp-tap.rs:181-189, :191-224"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§6 E2E Test Strategy (Drivers per surface, ipc-internal MCP stdio row)"
    change: "Notes add the recorded-exchange replay `crates/viola-e2e/tests/mcp_recorded_exchange.rs` `mcp_recorded_exchange_replays_the_client_frames_and_keeps_the_accepted_result_shape` (`binary(mcp_recorded_exchange)`, chunk 2026-10-20-epoch-5-cleanup-ii): over a harness session it writes each walked `Mcp.*.json`'s recorded client frames raw to a `viola mcp` child, takes the `tools/list` response by its request's id, and requires the served result to keep the recorded result's key and type shape and its tool names in order (planted negatives read missing)."
    sidecar: "§6 MCP stdio driver row: the recorded-exchange replay case named beside `tests/mcp_tools.rs`."
    rationale: "Report §Changes → Symbols → New tests (the replay case), Outcome («run --e2e --filter 'binary(mcp_recorded_exchange)' ok; tool names in order; both planted negatives read missing»), and Expected amendments «test-plan §6 (the replay case in the contract suite)». The row names `tests/mcp_tools.rs` as the MCP stdio suite's only home."
    basis: "test-plan.md:624; crates/viola-e2e/tests/mcp_recorded_exchange.rs:181-226, :105-111, :56-88"
  - detector: D-tests-coverage
    severity: warning
    section: "§1 Test Scope Summary (ipc-internal MCP stdio surface, Signal)"
    change: "After the `ttlMs` 0 / `cacheScope` `private` clause, add: a live claude 2.1.302 exchange on protocol `2026-07-28` (`server/discover`, then `tools/list`, each with `_meta`, no `initialize` or `notifications/initialized`) is committed as `fixtures/claude/2.1.302/Mcp.tools-list-1.json` and replayed by `mcp_recorded_exchange`, the served results keeping the recorded shape, those two keys included."
    sidecar: "§1 MCP stdio Signal: the measured live handshake and its recorded-exchange replay named beside the ttlMs / cacheScope clause."
    rationale: "Report §Changes → Cross-project (start 2 measured `server/discover` then `tools/list`, no `initialize`; viola's results carry `ttlMs` 0 and `cacheScope` `private`) and Expected amendments (`ttl-cache` site test-plan.md:126 «the site where the replay's gate on them belongs»)."
    basis: "test-plan.md:126; fixtures/claude/2.1.302/Mcp.tools-list-1.json:3-56, :57-198"
    dependent-of: D-tests-coverage
  - detector: D-retired
    severity: warning
    section: "§5 Integration Test Strategy (the `tests/cli_list.rs` board bullet, first occurrence)"
    change: "«backed by the `src/human/board.rs` units `bay_line_*` and `board_row_*`» reads «backed by the `src/human/board/` units `bay_line_*` (`bay.rs`) and `board_row_*` (`cells.rs`, `rows.rs`)»."
    sidecar: "§5 board bullet: the board units re-cited under the `src/human/board/` module (the single-file `board.rs` retired)."
    rationale: "Report §Changes → Retired: the single-file `src/human/board.rs` (now a module root, 9 code lines); Moved: the cells in `cells.rs`, the rows in `rows.rs`, `BayBudget` in `bay.rs`. Expected amendments name test-plan.md:590 «the `src/human/board.rs` units — now under `src/human/board/`»."
    basis: "test-plan.md:590; src/human/board/bay.rs:101-111; src/human/board/rows.rs:121-131"
  - detector: D-retired
    severity: warning
    section: "§5 Integration Test Strategy (the `tests/cli_list.rs` board bullet, second occurrence)"
    change: "«backed by the `src/human/board.rs` units `board_row_*` and the `viola-state` units `board_join_*`» reads «backed by the `src/human/board/` units `board_row_*` (`cells.rs`, `rows.rs`) and …»."
    sidecar: "§5 board bullet (unwrapped rack clause): `board_row_*` re-cited under `src/human/board/`."
    rationale: "Same retired single-file cite restated later in the same bullet (report §Changes → Retired / Moved, paths kept)."
    basis: "test-plan.md:590; src/human/board/rows.rs:227-244"
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: "§5 Integration Test Strategy (the `tests/cli_verify.rs` bullet)"
    change: "«The stop's unit cases in `src/cmd/verify/stop.rs`, `typed.rs` and `src/run/version_gate.rs`» reads «… in `src/cmd/verify/stop.rs`, `src/cmd/verify/typed/run.rs` (`ctrl_c_ending_*`, `ended_by_*`), `src/cmd/verify/typed/turns.rs` (`poll_until_*`, `wait_for_and_wait_capture_end_at_the_stop_over_a_held_condition`) and `src/run/version_gate.rs`»."
    sidecar: "§5 cli_verify bullet: the stop's typed cases re-cited under `typed/run.rs` and `typed/turns.rs`."
    rationale: "Report §Changes → Retired (the single-file `src/cmd/verify/typed.rs`, now a 59-line module root) and Moved (`typed/run.rs` holds `Run`, `ctrl_c_ending`, `ended_by`; `typed/turns.rs` holds `poll_until`, `wait_for`, `wait_capture`); Expected amendments name test-plan.md:591 «`typed.rs` — now its children»."
    basis: "test-plan.md:591; src/cmd/verify/typed/run.rs:239-268, :297-310; src/cmd/verify/typed/turns.rs:353-363, :401-418"
  - detector: D-retired
    severity: warning
    section: "§10 Quality Gates & Coverage Targets (Mutation gate paragraph)"
    change: "Where the Epoch 4 history cites `src/cmd/verify/typed.rs`, «inside `typed.rs`» and «a case in `typed.rs`'s `mod tests`», add «(since chunk 2026-10-20-epoch-5-cleanup-ii `typed.rs` is a module root: `framing_turns`, `added_turn_ended`, `local_command_answered` and `probe_until` with their cases in `typed/turns.rs`, `SharedPty` in `typed/pty.rs`, `Run`, `ctrl_c_ending` and `ended_by` in `typed/run.rs`)»."
    sidecar: "§10 Mutation gate: the Epoch 4 survivors' `typed.rs` cites given their post-split homes."
    rationale: "Report §Changes → Retired (single-file typed.rs) and Moved (the children's contents); Expected amendments name test-plan.md:1095 @c4149,4431,4791 `typed.rs` — now its children. The history stays; a reader following the cite needs the new file."
    basis: "test-plan.md:1095; src/cmd/verify/typed/turns.rs:313; src/cmd/verify/typed/run.rs:244"
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets (Mutation gate paragraph)"
    change: "«Its 2 missed are in `prepare` …, reachable only on a Windows host, and are recorded not measured here … On the Linux dev host they still read missed, so a Linux whole-member run of viola-e2e still reads red by them» reads: they were reachable only on a Windows host until chunk 2026-10-20-epoch-5-cleanup-ii, where `prepare(root, host)` takes its host flag from `mutate()` (`scratch::HOST_SCRATCH`) and `prepare_on_the_scratch_host_empties_the_scratch_and_off_it_is_none` reads both arms on every host, killing the Epoch 5 audit's two `prepare` survivors (`Ok(None)`, the deleted `!`) on Linux."
    sidecar: "§10 Mutation gate: the `prepare` misses no longer Windows-only; killed on every host by the host-flag restatement and its case."
    rationale: "Report §Changes → Symbols (Changed signature: `scratch::prepare(root, host: bool)`, its caller passes `HOST_SCRATCH`; new kill case `scratch.rs:190-213`) and Outcome (16 survivors, each red under its hand-applied mutant, green reverted); the chunk's survivors ledger `evidence/survivors.ndjson` holds the two `prepare` rows as `killed` by that case."
    basis: "test-plan.md:1095; crates/viola-e2e/src/harness/run/mutants/scratch.rs:46-49, :190-213; crates/viola-e2e/src/harness/run/mutants.rs:224"
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets (Mutation gate paragraph)"
    change: "«the 2 `prepare` rows above stand as recorded» gains «(until chunk 2026-10-20-epoch-5-cleanup-ii killed them on every host)»."
    sidecar: "§10 Mutation gate: the Epoch 4 note on the 2 `prepare` rows given its end."
    rationale: "Same standing claim (the `prepare` mutants are only recorded, never killed on Linux) restated in the Epoch 4 disposition sentence; report §Changes → Symbols / New tests."
    basis: "test-plan.md:1095"
    dependent-of: D-tests-coverage
  - detector: D-tests-coverage
    severity: warning
    section: "§10 Quality Gates & Coverage Targets (Mutation gate paragraph)"
    change: "After the Epoch 4 dispositions, add: the Epoch 5 boundary audit's 16 survivors were disposed by chunk 2026-10-20-epoch-5-cleanup-ii in its ledger `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/evidence/survivors.ndjson`, 10 killed and 6 after a behaviour-preserving restatement, each by a nextest case in the package that owns the mutated file (`viola-agent-claude` statusline, `viola-state` liveness, the `viola-e2e` harness's scratch, session and status, `viola-mcp`'s tools and lib), each red by its own panic line under its hand-applied mutant and green reverted byte-exact, no mutation run made; none equivalent or recorded."
    sidecar: "§10 Mutation gate: the Epoch 5 audit's 16 survivors' disposition recorded (10 killed, 6 restated)."
    rationale: "Report §Outcome acceptance (tests): 16 survivors, 10 `killed` and 6 `restated`, none equivalent or recorded, none from a cargo-mutants run; Symbols → New tests lists the nine kill cases by file. The paragraph records each epoch boundary's survivor disposition and stops at Epoch 4."
    basis: "test-plan.md:1095; crates/viola-agent-claude/src/statusline.rs:452-461; crates/viola-state/src/liveness.rs:156-171; crates/viola-mcp/src/tools.rs:470-528; crates/viola-mcp/src/lib.rs:109-133"
note: "D-tests-framework and D-tests-obs-harness read no drift (nextest/rstest unchanged; the harness's `capped_document` keeps the 5-command key's `take(MAX_FRAME + 1)` read and the report states no status-shape change). No test-plan site names `relied_here`, a 2.1.302 file count or a test-case count. The §9 Lint errors dependent and the two §7 dependents carry no line because the report states none for them."
