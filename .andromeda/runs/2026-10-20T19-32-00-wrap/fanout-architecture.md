proposals:
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Binary, subcommands and exit codes — the test-only binaries bullet)
    change: add `viola-mcp-tap` to the test-only binaries, never in a release build — a `[[bin]]` of `viola-e2e` with two verbs, `record <capture> <viola exe> <args…>` (a direct spawn, no shell and no PATH, each line through `take(MAX_FRAME)`, one append per direction-tagged line, the capture created 0600, exits with the child) and `fixture <capture> <out>` (the client frames up to and including the first `tools/list` request and each request's result keyed by method, through `ledger::scrub` and then `ledger::screen_fault`, refusing with the closed codes `home-path` · `absolute-path` · `username` · `email` and the file name, never content)
    sidecar: arch §Occupied Resources — the test-only binaries list gains `viola-mcp-tap` (viola-e2e `[[bin]]`, `record` / `fixture`), chunk 2026-10-20-epoch-5-cleanup-ii
    rationale: the report's Symbols / APIs bullet "New test-only bin" lands `viola-mcp-tap` and its Dependencies bullet adds the `[[bin]]` to viola-e2e; the tap is absent from every master (report Expected amendments, pattern `tap-name` 0)
    basis: crates/viola-e2e/Cargo.toml:18-22
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Workspace crates — the test-only `viola-e2e` bullet)
    change: "`viola-e2e` (harness library + bins `viola-harness` and `viola-mcp-tap`, `publish = false`; …)" — the crate now carries a second test-only bin
    sidecar: arch §Occupied Resources Workspace crates — viola-e2e's bins are `viola-harness` and `viola-mcp-tap`
    rationale: the report's Dependencies bullet — `crates/viola-e2e/Cargo.toml` gains the `[[bin]]` `viola-mcp-tap`; the workspace-crates bullet names `viola-harness` as viola-e2e's only bin
    basis: crates/viola-e2e/Cargo.toml:18-22
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: the `viola-e2e/` tree comment ("test-only: viola-harness (…)") also names the test-only bin `viola-mcp-tap` (src/bin/viola-mcp-tap.rs, the live MCP exchange tap behind the `Mcp.` fixtures)
    sidecar: Project directory structure — the viola-e2e row gains the `viola-mcp-tap` bin
    rationale: the report's Expected amendments name `project-directory-structure.md:82` "test-only: viola-harness (…)" as the bin list where the tap joins
    basis: .andromeda/registries/contracts/architecture/project-directory-structure.md:82
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Repository — the schemas bullets)
    change: add a bullet `schemas/claude-mcp-fixture.v1.json` — the schema of the `Mcp.` class (test-side): `v` const 1; `client` an array of JSON-RPC frames (each `jsonrpc` "2.0", `method` a string) that contains a `tools/list` request with an `id`; `results` an object keyed by method, `tools/list` required with a `tools` array of named tools, any other key an object; `tests/contract_fixture_hygiene.rs` validates every `fixtures/claude/*/Mcp.*.json` against it plus an email check, and its hook-payload walk (`claude_fixtures`) leaves `Mcp.` files out
    sidecar: arch §Occupied Resources — registers `schemas/claude-mcp-fixture.v1.json` (the `Mcp.` class) and its hygiene walk
    rationale: the report's Schema / config bullet lands the new schema and the walk; the schema bullets list the statusline and agents schemas but not this one (report Expected amendments, `fixture-classes` `architecture.md:379` the schemas)
    basis: .andromeda/architecture.md:379
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: the `schemas/` row's list also names `claude-mcp-fixture.v1.json` (test-side)
    sidecar: Project directory structure — the schemas row gains claude-mcp-fixture.v1.json
    rationale: report Schema / config bullet; Expected amendments name `project-directory-structure.md:53` schemas as a class enumeration the `Mcp.` class joins
    basis: .andromeda/registries/contracts/architecture/project-directory-structure.md:53
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Repository — the `fixtures/claude/<cli-version>/` bullet)
    change: add the fixture class `Mcp.<variant>.json` — a live MCP exchange recorded not by `viola verify --record` but by the test-side `viola-mcp-tap` (`record` then `fixture`, the same `ledger::scrub` and the four closed refusal codes), validated by `schemas/claude-mcp-fixture.v1.json` and hygiene-walked (`mcp_fixtures_pass_hygiene`), replayed by `tests/mcp_recorded_exchange.rs` (viola-e2e); `Mcp.tools-list-1.json` in `2.1.302`, recorded by the founder-capped second live print-mode start (inputs#I2 Q1, inputs#I5 Q1)
    sidecar: arch §Occupied Resources Repository — the `Mcp.` fixture class (tap-recorded, scrubbed, schema-validated, replayed)
    rationale: the report's Schema / config bullet ("New fixture class `fixtures/claude/<cli-version>/Mcp.<variant>.json`") and Decisions (the founder's boundary widening); the bullet enumerates every recorded class and names `viola verify --record` as the only recorder (Expected amendments, `fixture-classes` / `hygiene-walk` `architecture.md:374` @c271 candidate)
    basis: .andromeda/architecture.md:374
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Repository — the `fixtures/claude/<cli-version>/` bullet, the `2.1.302` set)
    change: "`2.1.302` (4 spine payloads + 3 screens + 12 dialog variants + 6 framing variants + 2 status line variants + 1 resume variant + 1 agents variant, recorded whole by one `viola verify --record` round, + 1 MCP exchange `Mcp.tools-list-1.json` recorded by the test-side `viola-mcp-tap`; 30 files, the count's rule `ls fixtures/claude/2.1.302 | wc -l`; …)"
    sidecar: arch §Occupied Resources Repository — the 2.1.302 set is 30 files (was 29), not recorded whole by one --record round (the `Mcp.` file is the tap's)
    rationale: the report's Counts bullet — `fixtures/claude/2.1.302` holds 30 files, was 29; "recorded whole by one" (@c3803) and "29 files" (@c3856) are stale, the rule (@c3887) is kept
    basis: .andromeda/architecture.md:374
  - detector: D-arch-resources
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: the `fixtures/claude/<cli-version>/` row also names the `Mcp.<variant>.json` MCP exchanges recorded by the test-side `viola-mcp-tap`
    sidecar: Project directory structure — the fixtures row gains the `Mcp.<variant>.json` class
    rationale: report Schema / config bullet; Expected amendments name `project-directory-structure.md:109` fixtures as a class enumeration the `Mcp.` class joins
    basis: .andromeda/registries/contracts/architecture/project-directory-structure.md:109
    dependent-of: D-arch-resources
  - detector: D-arch-resources
    severity: warning
    section: §Standard Contracts (the MCP tool contract — the `tools/list` result bullet)
    change: after the `ttlMs` 0 / `cacheScope` `private` sentence add — claude 2.1.302 on protocol `2026-07-28` opens with `server/discover` (a string JSON-RPC id) and then `tools/list`, each with `_meta` (`io.modelcontextprotocol/protocolVersion`, `…/clientInfo`, `…/clientCapabilities`), and sends no `initialize` or `notifications/initialized`; viola's two results each carry `ttlMs` 0 and `cacheScope` `private` (as measured live at chunk 2026-10-20-epoch-5-cleanup-ii, `evidence/live-tap.md` start 2), and the recorded exchange `Mcp.tools-list-1.json` is replayed by `mcp_recorded_exchange_replays_the_client_frames_and_keeps_the_accepted_result_shape`
    sidecar: arch §Standard Contracts — the measured 2.1.302 MCP opening (`server/discover` then `tools/list`, no `initialize`) and the recorded-exchange replay that holds the accepted result shape
    rationale: the report's Cross-project bullet (start 2's measurement) and Spec claims disproved (the `initialize`-flavoured handshake false for 2.1.302; `:152` "the site where the measured frames belong"); Expected amendments `ttl-cache` (`architecture.md:152` "the site where the replay's gate on them belongs")
    basis: .andromeda/architecture.md:152
  - detector: D-arch-resources
    severity: warning
    section: §Occupied Resources (Environment variables)
    change: add a test-only bullet `MCP_SERVE_TEST_CHILD` — a marker set by `Command::env` on a `viola-mcp` unit-test child process alone and read only by that crate's `#[cfg(test)]` child entry `serve_child_entry`; never read by a `viola` build, outside the `VIOLA_*` namespace (the `PTY_SIDELOAD_TEST_CASE` precedent)
    sidecar: arch §Occupied Resources Environment variables — registers the test-only child marker `MCP_SERVE_TEST_CHILD`
    rationale: the report's New tests bullet (the child entry `serve_child_entry`, marker `MCP_SERVE_TEST_CHILD` set by `Command::env` on the child alone) and Reverted bullet (renamed from `VIOLA_MCP_SERVE_TEST_CHILD`); the env-var registry lists no such marker
    basis: crates/viola-mcp/src/lib.rs:82-134
  - detector: D-retired
    severity: warning
    section: §Infrastructure Patterns → Crate dependency direction
    change: the `viola-mcp` bullet drops `serde` — "`viola-mcp` → `viola-core`, `viola-channel[tokio]`, `viola-state`, `viola-agent-claude`, rmcp (…), schemars, tokio (`rt`), serde_json, thiserror, tracing; …"
    sidecar: Crate dependency direction — `viola-mcp` no longer depends on serde (`serde.workspace = true` dropped; `cargo machete crates/viola-mcp` clean)
    rationale: the report's Retired bullet ("the `viola-mcp` → `serde` edge") and Dependencies bullet; Expected amendments `mcp-serde` (`crate-dependency-direction.md:7` — stale)
    basis: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:7
  - detector: D-retired
    severity: warning
    section: §Infrastructure Patterns → Crate dependency direction
    change: the shared-pins bullet reads "serde, serde_json, chrono and thiserror are shared through `[workspace.dependencies]` by the workspace crates that take them; `viola-pty` takes none of them and `viola-mcp` takes no serde" instead of "by every workspace crate except `viola-pty`"
    sidecar: Crate dependency direction — the shared-pins bullet no longer implies viola-mcp takes serde
    rationale: the report's Retired bullet (the `viola-mcp` → `serde` edge); this bullet restates every crate but viola-pty as taking serde
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Infrastructure Patterns → Crate dependency direction
    change: the `viola-agent-claude` bullet's ledger list reads "the per-OS relied set (`relied()`, filtering on `relied_on(WRAPPED_HERE)`)" — `relied_here` removed
    sidecar: Crate dependency direction — `LedgerRow::relied_here` retired from the ledger module's listed API
    rationale: the report's Retired bullet (`LedgerRow::relied_here`) and Symbols / APIs Removed (`relied()` now filters on `relied_on(WRAPPED_HERE)`, `ledger.rs:154-157`); the report's sweep read no master naming it, but this key file still lists `relied_here`
    basis: .andromeda/registries/contracts/architecture/crate-dependency-direction.md:6
  - detector: D-retired
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: the `verify/typed.rs` row reads as a module root (`RunDirs`, `Inputs`, `RunStep`, `measure`, `POLL`) over `verify/typed/`: `runs.rs` (Runs A–E, `trusted_args`, `TrustedDir`), `turns.rs` (the settle and wait helpers, `agents_read`, `framing_turns`, the input-box wait after an added turn), `pty.rs` (`KeyPipe`, `SharedPty`, `Feed`, `Sink`, `settled`, `box_wait`), `run.rs` (`Run`, `ctrl_c_ending`, `ended_by`) and `#[cfg(test)]` `test_support.rs`
    sidecar: Project directory structure — `src/cmd/verify/typed.rs` is a module root over `typed/{runs,turns,pty,run}.rs` (+ cfg(test) test_support)
    rationale: the report's Retired bullet (the single-file `src/cmd/verify/typed.rs`) and Symbols / APIs "Moved, paths kept"; Expected amendments `typed-split` (`project-directory-structure.md:33` the tree — stale)
    basis: .andromeda/registries/contracts/architecture/project-directory-structure.md:33
  - detector: D-retired
    severity: warning
    section: §Infrastructure Patterns → Project directory structure
    change: the `human/` row's `board.rs` reads as a module root re-exporting `BayBudget`, `home_state_pair` / `instance_state_pair` and `write_board` over `human/board/`: `pairs.rs` (the two state refusal pairs), `bay.rs` (`BayBudget`), `cells.rs` (the cells, spans, `board_line`, `cut`), `rows.rs` (`write_board`) and `#[cfg(test)]` `test_support.rs`
    sidecar: Project directory structure — `src/human/board.rs` is a module root over `board/{pairs,bay,cells,rows}.rs` (+ cfg(test) test_support)
    rationale: the report's Retired bullet (the single-file `src/human/board.rs`) and Symbols / APIs "Moved, paths kept"; Expected amendments `board-split` (`project-directory-structure.md:23` the tree — stale; `:70` viola-state's `board.rs` a different file, no change)
    basis: .andromeda/registries/contracts/architecture/project-directory-structure.md:23
  - detector: D-retired
    severity: warning
    section: §Established Decisions (the capability-ledger / `viola verify` decision)
    change: "It then drives five interactive PTY runs (`src/cmd/verify/typed.rs` and its children `typed/runs.rs`, `turns.rs`, `pty.rs`, `run.rs`)" instead of the single file
    sidecar: arch §Established Decisions — verify's interactive runs cited by the `typed/` module tree, not the single typed.rs
    rationale: the report's Retired bullet (the single-file typed.rs); Expected amendments `typed-split` (`architecture.md:101` @c4162 `src/cmd/verify/typed.rs` — stale as a file cite)
    basis: .andromeda/architecture.md:101
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Established Decisions (the capability-ledger / `viola verify` decision)
    change: Run B's agents read cites `typed::turns::agents_read` instead of `typed::agents_read`
    sidecar: arch §Established Decisions — `agents_read` cited at its new path `typed::turns`
    rationale: the report's Symbols / APIs "Moved, paths kept" (`agents_read` in `typed/turns.rs`; `verify.rs` keeps only `typed::measure`, `typed::RunDirs`, `typed::Inputs`); Expected amendments `typed-split` (`architecture.md:101` @c4550 — stale)
    basis: .andromeda/architecture.md:101
    dependent-of: D-retired
  - detector: D-retired
    severity: warning
    section: §Established Decisions (the capability-ledger / `viola verify` decision)
    change: Run B's `--settings` pair cites `typed::runs::trusted_args` instead of `typed::trusted_args`
    sidecar: arch §Established Decisions — `trusted_args` cited at its new path `typed::runs`
    rationale: the report's Symbols / APIs "Moved, paths kept" (`trusted_args` in `typed/runs.rs`); Expected amendments `typed-split` (`architecture.md:101` @c5258 — stale)
    basis: .andromeda/architecture.md:101
    dependent-of: D-retired
note: D-arch-decisions found no drift — no external crate added (`cargo deny check` exit 0), and viola-e2e's new `viola-agent-claude` path edge contradicts no decision (Crate dependency direction leaves viola-e2e's dependencies unlisted; arch:108 and arch:332 stay true). The tree names no viola-agent-claude ledger file, so `ledger/test_cases.rs` needs no tree row unless the orchestrator wants one. PTY_SIDELOAD_TEST_CASE, the precedent the report cites for the child marker, is itself unregistered in architecture; whether a test-child marker belongs in the env-var registry at all is the orchestrator's call.
