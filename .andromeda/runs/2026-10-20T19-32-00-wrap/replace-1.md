- line: .andromeda/architecture.md:101
old: «interactive PTY runs (`src/cmd/verify/typed.rs`).»
new: «interactive PTY runs (`src/cmd/verify/typed.rs` and its children under `src/cmd/verify/typed/`).»

- line: .andromeda/architecture.md:101
old: «(`typed::agents_read`: Run B's program»
new: «(`typed::turns::agents_read`: Run B's program»

- line: .andromeda/architecture.md:101
old: «(`typed::trusted_args`; the document»
new: «(`typed::runs::trusted_args`; the document»

- line: .andromeda/architecture.md:315
old: «behind `scripts/agent-run.{sh,ps1}`; its command contract is test-plan §3).»
new: «behind `scripts/agent-run.{sh,ps1}`; its command contract is test-plan §3) and `viola-mcp-tap` (a `[[bin]]` of `viola-e2e` that records a live MCP exchange for the `Mcp.` fixtures, Repository below: `record <capture> <viola exe> <args…>` is a direct spawn, no shell and no PATH lookup, each line read through `take(MAX_FRAME)` and appended once, tagged with its direction, to a capture created 0600, exiting with the child; `fixture <capture> <out>` keeps the client frames up to and including the first `tools/list` request and each request's result keyed by method, through `ledger::scrub` (the home → `~`, the user word → `<user>`) and then `ledger::screen_fault` over every string and key, refusing with the file name and one closed code `home-path` · `absolute-path` · `username` · `email`, never the content).»

- line: .andromeda/architecture.md:374
old: «+ 1 agents variant, recorded whole by one `viola verify --record` round; 29 files, the count's rule `ls fixtures/claude/2.1.302 | wc -l`»
new: «+ 1 agents variant, recorded by one `viola verify --record` round, + 1 MCP exchange `Mcp.tools-list-1.json` recorded by the test-only `viola-mcp-tap`; 30 files, the count's rule `ls fixtures/claude/2.1.302 | wc -l`»

- line: .andromeda/architecture.md:374
old: «the fake agent's `agents` mode replays it (`fake_agent_agents_replay_matches_every_recorded_agents_set`).»
new: «the fake agent's `agents` mode replays it (`fake_agent_agents_replay_matches_every_recorded_agents_set`). The `Mcp.<variant>.json` class is recorded not by `--record` but by the test-only `viola-mcp-tap` (Binary, above): a live CLI's MCP exchange through the same scrub and refusal, validated by `schemas/claude-mcp-fixture.v1.json`, hygiene-walked (`mcp_fixtures_pass_hygiene`) and replayed against `viola mcp` by `tests/mcp_recorded_exchange.rs` (`viola-e2e`).»

- line: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6
old: «the per-OS relied set (`relied()`, `relied_here`)»
new: «the per-OS relied set (`relied()`, filtering on `relied_on(WRAPPED_HERE)`)»

- line: .andromeda/security-plan.md:215
old: «`src/cmd/verify/typed.rs` `resume_run` (Run E)»
new: «`src/cmd/verify/typed/runs.rs` `resume_run` (Run E)»

- line: .andromeda/test-plan.md:379
old: «every other `fixtures/claude/*/*.json` hook payload to `schemas/claude-fixture.v1.json`»
new: «every `fixtures/claude/*/Mcp.*.json` MCP exchange to `schemas/claude-mcp-fixture.v1.json`, every other `fixtures/claude/*/*.json` hook payload to `schemas/claude-fixture.v1.json`»

- line: .andromeda/test-plan.md:591
old: «in `src/cmd/verify/stop.rs`, `typed.rs` and `src/run/version_gate.rs` back it»
new: «in `src/cmd/verify/stop.rs`, the `src/cmd/verify/typed/` module and `src/run/version_gate.rs` back it»

- line: .andromeda/test-plan.md:970
old: «`planted_agents_username_is_rejected`; chunk 2026-10-16-unwrapped-sessions-on-the-board);»
new: «`planted_agents_username_is_rejected`; chunk 2026-10-16-unwrapped-sessions-on-the-board); the one MCP exchange of `2.1.302`, `Mcp.tools-list-1.json`, against `schemas/claude-mcp-fixture.v1.json` (`v` const 1; `client` JSON-RPC 2.0 frames that contain a `tools/list` request with an `id`; `results` keyed by method, `tools/list` holding a `tools` array of named tools) with the same path, username and email walk (`mcp_fixtures_pass_hygiene`; planted `planted_clean_mcp_exchange_passes`, `planted_dirty_mcp_exchange_is_rejected`, `planted_mcp_schema_violation_is_rejected`);»

- line: .andromeda/test-plan.md:1027
old: «the probe fires on a planted orphan, last line `orphans-check probes: 1/1 fired, control clean`»
new: «the probe fires on each of its two planted orphans, last line `orphans-check probes: 2/2 fired, control clean`»

- line: .andromeda/test-plan.md:1095
old: «, reachable only on a Windows host, and are recorded not measured here.»
new: «. `prepare` takes its host flag from `mutate()` (`scratch::HOST_SCRATCH`), so both arms are reached on every host, and both mutants (`Ok(None)` and the deleted `!`) read killed on the Linux dev host by its scratch case (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/evidence/survivors.ndjson`).»

- line: .andromeda/test-plan.md:1095
old: «job `mutants (viola-e2e)`). On the Linux dev host they still read missed, so a Linux whole-member run of viola-e2e still reads red by them. The Epoch 4»
new: «job `mutants (viola-e2e)`). The Epoch 4»

- line: .andromeda/test-plan.md:1095
old: «the 2 `prepare` rows above stand as recorded;»
new: «the 2 `prepare` rows above stood as recorded (killed on every host since, above);»

- line: .andromeda/test-plan.md:1095
old: «and the 16 in `src/cmd/verify/typed.rs` (`framing_turns` 5»
new: «and the 16 in `src/cmd/verify/typed.rs` (whose code now sits under its children in `src/cmd/verify/typed/`; `framing_turns` 5»

- line: .andromeda/test-plan.md:1095
old: «restatement inside `typed.rs` (the two framing waits»
new: «restatement inside that module (the two framing waits»

- line: .andromeda/test-plan.md:1095
old: «and a case in `typed.rs`'s `mod tests`, each shown red»
new: «and a case in its unit tests, each shown red»

- line: .andromeda/test-plan.md:1095
old: «and the 16 of chunk 2026-10-13-epoch-4-survivors-ii). The boundary audit»
new: «and the 16 of chunk 2026-10-13-epoch-4-survivors-ii). The Epoch 5 boundary audit's 16 survivors were each disposed by a nextest case in the package that owns the mutated file, 10 killed and 6 after a behaviour-preserving restatement, each red under its hand-applied mutant and green reverted byte-exact, with no mutation run made and none equivalent or recorded (as measured at `viola-0.1.0/chunks/2026-10-20-epoch-5-cleanup-ii/evidence/survivors.ndjson`). The boundary audit»
