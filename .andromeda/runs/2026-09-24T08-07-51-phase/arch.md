# arch extract

## Relevance
partial. Arch covers where the code lives (test-only bin and crate), the registered repository paths and env-var naming, and the hook-spawn and version rules the fake agent has to mimic. The contract behaviour itself belongs to test-plan §7.

## Constraints
- **Code placement.** The fake agent stays a test-only root-package `[[bin]]` behind feature `fake-agent`, at `src/bin/viola-fake-agent.rs`, and must never be in a release build. The fixture chain belongs in the test-only `viola-e2e` crate, and no product crate may depend on that crate. So root `tests/support/` may reach `viola_e2e::fixtures` only as a dev-dependency. The root bin's "→ all members" normal-dependency edge must not include it. (per architecture §Occupied Resources "Binary, subcommands and exit codes" and "Workspace crates"; §Established Decisions [Module Boundaries]; §Infrastructure Patterns "Project directory structure")
- **Test isolation.** The fake agent is swapped in by naming it after `--` (`viola run <name> -- <fake agent> <args>`). Each test runs in its own viola home through `--home`, so parallel tests never share an endpoint, log or `budget.json`. That home's `ledger/stamps.json` is where the fake agent's reported version can be stamped. The `stamped_home` seam has to fit this. (per architecture §Established Decisions [CI/CD]; §Cross-cutting Patterns "Config management")
- **Test-home location.** Test homes go under the registered path `target/e2e-home/viola-session-*/home`, which is kept in CI for the obs gates. The `tempfile` prefix and layout have to match this pattern. (per architecture §Occupied Resources "Repository")
- **Harness env vars.** Harness-only env vars use the `AGENT_RUN_` prefix, and the `viola` binary never reads them. `AGENT_RUN_KEEP_HOMES` and `AGENT_RUN_CHUNK_BASE` are the only ones registered today. `AGENT_RUN_KEEP_FAILED` would be new. `FAKE_CLAUDE_AGENTS_MODE` fits neither the `VIOLA_` nor the `AGENT_RUN_` convention, so the name needs an explicit decision (it is read only by the fake agent). (per architecture §Conventions "Naming patterns — Environment variables"; §Occupied Resources "Environment variables")
- **How the fake agent runs hooks.** It must copy how the product invokes hooks: exec-form `command` + `args` read from the written plugin files, with no PATH search. Children are spawned directly, never through `sh`, `bash` or `cmd`. Paths inside plugin files use forward slashes. (per architecture §Established Decisions [Hook Transport], [Deployment / Distribution]; §Cross-cutting Patterns "Cross-platform discipline")
- **Version and CLI surfaces.**
  - The fake agent answers `--version` the same way the real CLI does, and output that cannot be parsed counts as an unlisted version. `--report-version` has to keep this so the version gate can be exercised.
  - Replay reads from `fixtures/claude/<cli-version>/`.
  - `agents --json` modes cover parsing that must tolerate bad input: a failed, oversized or malformed result gives status `unknown`, never an error.

  (per architecture §Established Decisions [CLI Version Compatibility], [Validation]; §Standard Contracts "Session liveness"; §Occupied Resources "Repository")
- **Receipt format.** The receipt format is harness-owned, but it should follow viola's own format rules:
  - an integer `v`
  - one complete JSON object plus `\n` per single `write`
  - snake_case fields and kebab-case enum values
  - `ts` in RFC 3339 UTC with milliseconds and a `Z` suffix
  - readers skip unknown kinds and fields

  (per architecture §Conventions "Protocol versioning", "Naming patterns", "Data model conventions — ndjson line discipline / Timestamps"; §Cross-cutting Patterns "Mixed-version tolerance")

## Patterns to follow
- The fake agent's modes should reproduce the product assumptions they test:
  - one bracketed paste `ESC[200~…ESC[201~` + CR equals one prompt
  - harness-turn prefixes `<agent-message from=` / `<task-notification>`
  - local commands that fire no UserPromptSubmit (`--local-command-mode`, `--suppress-prompt-submit`)
  - child exit detected on the process handle, never on EOF (`--exit-no-eof`)

  (per architecture §Established Decisions [Human Takeover / Wheel], [Delivery Confirmation], [PTY])
- New third-party test deps go through `[workspace.dependencies]`: rstest, proptest, jsonschema, and tempfile (arch names 3.27.0). Their versions must be compatible with `rust-version = "1.96"` under resolver 3, and new members set `publish = false`. (per architecture §Infrastructure Patterns "Build system"; §Established Decisions [Snapshot writer])
- `ViolaName` property strategies must generate against the locked definition: ASCII `[a-z0-9-]`, 1–32 chars, starting with a letter, nutype in `viola-core`. (per architecture §Conventions "Data model conventions"; §Stack and Technologies "Domain newtypes")
- The mutation-gate fix stays inside the wired `mutants` job model: ubuntu, chunk diff, base through `env:`, and outputs under `target/agent-run/artifacts/` (`run-summary.json`). (per architecture §Infrastructure Patterns "CI/CD approach"; §Occupied Resources "Repository")
- Fake-agent and harness code uses std threads and blocking I/O. Tokio is confined to `viola-mcp` and `viola-ui`. (per architecture §Cross-cutting Patterns "Tokio containment")

## Anti-patterns to avoid
- Tests that set process-global env with `std::env::set_var`, or touch the default `<user home>/.viola/`. Isolation comes from `--home` and per-child env only. (per architecture §Established Decisions [CI/CD]; §Cross-cutting Patterns "Config management")
- Starting hooks through PATH or a shell (`sh -c`, `cmd /c`) from the fake agent. That would test a path the product forbids. (per architecture §Established Decisions [Deployment / Distribution]; §Cross-cutting Patterns "Cross-platform discipline")
- A product crate, or the `viola` binary's normal dependency graph, taking on `viola-e2e`, the `fake-agent` feature, or reads of `AGENT_RUN_*` / `FAKE_*` variables. (per architecture §Established Decisions [Module Boundaries]; §Conventions "Environment variables")

## Contract bindings
- **arch ↔ tests.** The fake-agent contract (test-plan §7) is the stand-in for everything arch marks as a capability-ledger row. Fixture replay binds to the registered `fixtures/claude/<cli-version>/` path, and to CI target job 5 ("replaying `fixtures/claude/*`"). (§Established Decisions [CLI Version Compatibility]; §Infrastructure Patterns "CI/CD approach")
- **arch ↔ obs.** The homes kept under `target/e2e-home/` (`AGENT_RUN_KEEP_HOMES=1`) feed the obs gates, and the home's `diagnostics/` layout is owned by obs. (§Occupied Resources "Repository", "Filesystem")
- **arch ↔ security.** The fixture scrub (absolute paths, usernames) guards the registered `fixtures/` tree. Hook invocation must stay exec-form, with no shell. (§Occupied Resources "Repository"; §Cross-cutting Patterns "Cross-platform discipline")
- **Registry at wrap.** These new resources must be amended into §Occupied Resources and §Infrastructure Patterns "Project directory structure":
  - `fixtures/fake-scripts/`
  - `proptest-regressions/`
  - `AGENT_RUN_KEEP_FAILED`
  - `FAKE_CLAUDE_AGENTS_MODE`
  - the receipt format, as a test-only format

## Acceptance criteria contributions
- `viola-fake-agent` builds only with `--features fake-agent`, and `viola-e2e` shows up in the root package only under `[dev-dependencies]` (per architecture §Occupied Resources "Binary, subcommands and exit codes"; §Established Decisions [Module Boundaries]).
- Every fixture-chain home resolves under `<workspace>/target/e2e-home/viola-session-*/home` and reaches children only through `--home` or `Command::env`. A grep over `tests/` and `crates/viola-e2e/` finds no `std::env::set_var` (per architecture §Occupied Resources "Repository"; §Established Decisions [CI/CD]).
- The fake agent's hook invocation spawns the `command` path read from the test-written plugin or settings file directly with its `args`. A test with a same-named decoy earlier on PATH proves the decoy never runs (per architecture §Established Decisions [Hook Transport], [Deployment / Distribution]).
- Every receipt line parses as one JSON object carrying integer `v` and snake_case fields, and every new env var the chunk introduces is either `AGENT_RUN_`-prefixed or recorded as an explicit exception at wrap (per architecture §Conventions "Protocol versioning", "Naming patterns — Environment variables").

## Relevant amendment history
- **2026-09-24-three-os-ci-headless-harness-skeleton, "test-only crate, bins, env vars, paths":**
  - Registered `viola-e2e` (with its no-op `fake-agent` feature), the `viola-fake-agent` root `[[bin]]`, `AGENT_RUN_CHUNK_BASE` / `AGENT_RUN_KEEP_HOMES`, the `AGENT_RUN_` prefix, `target/e2e-home/`, and `src/bin/viola-fake-agent.rs` / `tests/` in the tree.
  - Why: that chunk created them.
  - Note: child env the harness sets (`PATH`, `CARGO_TARGET_DIR`, `NEXTEST_PROFILE`) was deliberately not registered, because the arch registry tracks only variables the product reads or sets. Use the same test when deciding whether `FAKE_CLAUDE_AGENTS_MODE` and `AGENT_RUN_KEEP_FAILED` get registered.
- **Same chunk, "CI setup and wired jobs":**
  - Wired the `mutants` job (ubuntu, push + PR, base via `env:`) and the `test` job's `agent-run-<os>` artifact upload.
  - Why: that chunk shipped `ci.yml`. It is the gate that item 7's vacuous-pass fix changes.
- **Same chunk, "toolchain floor and exact pin":**
  - The declared floor is now `rust-version = "1.96"` with 1.98.1 pinned exactly.
  - Why: intent F-19. New test deps (rstest, proptest, jsonschema, tempfile) must resolve at or under this floor.
