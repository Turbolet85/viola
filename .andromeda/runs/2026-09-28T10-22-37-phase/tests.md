# tests extract

## Relevance
relevant: the chunk builds the stamp source that the test plan's whole fixture chain, harness `boot` and Path 7 depend on (per test-plan §1 Coverage scope `viola-agent-claude`, §7 Seed strategies).

## Constraints
- Only `viola verify --home <home>` run against `viola-fake-agent` (copied onto a test-scoped PATH as `claude`) may produce `ledger/stamps.json` in any test or harness home. No test or harness code writes the file (per test-plan §7 Seed strategies, §12 `2026-09-24` User review 1). Once `verify` exists, the root rstest `stamped_home` must stop being the interim no-stamp seam (`stamped:false`) and stamp through `verify`. Both fixture copies share one contract (per test-plan §3 `run` step 2). Research must check whether any code path writes stamps other than `verify` today.
- The real `claude` never runs in CI. Live `viola verify` (Haiku) is local-only, through `agent-run run --local-live`, which must refuse with exit 2 `reason:"live-in-ci"` when `CI` is set (per test-plan §3 `run` `--local-live`, §11 Test Strategy). Recording real fixtures is therefore an operator/local step, and the plan must name who runs it. Research must check whether `--local-live` is wired at HEAD.
- Recorded fixtures are `fixtures/claude/<cli-version>/<Event>.<variant>.json`, where `<Event>` is the CLI's PascalCase hook event name. Neither the recorder nor the fake agent keeps an event-to-file mapping table (per test-plan §2 File naming, §12 fake-agent chunk entry). The recorder must rewrite home paths to `~` and usernames to `<user>` before writing. Fixture prompts are synthetic probe text only (per test-plan §7 Fixture hygiene).
- The `fixtures/claude/*/*.json` line joins the existing `tests/contract_fixture_hygiene.rs` `#[files]` walk with the first recorded fixture. Each fixture is checked for absolute paths and non-placeholder usernames, then validated by jsonschema against a tolerant schema: unknown fields allowed, closed enums required. `tests/support/hygiene.rs` reports only the class (`absolute-path` / `username` / `schema`), and planted inputs must prove each class red (per test-plan §7 Fixture hygiene, §6 Contract suite).
- Harness `boot` requirements (per test-plan §3 `boot` steps 3–5, Readiness signal, Exit code):
  - Unless `--unstamped` is given, step 4 runs `viola verify --home <home>` against the fake agent. It requires exit 0 and a last stdout line matching `^stamped \S+  \d+ pass  0 fail$`. Otherwise the result is `reason:"verify-failed"`.
  - Readiness adds `events.ndjson` line 3 = `kind:"session-start"`, `source:"hook"`. Today only lines 1–2 are checked.
  - The supervisor passes `--fixtures <workspace root>/fixtures/claude`, the parent dir, so the fake agent joins `<cli-version>` itself.
  - Research must check whether `supervise` passes `--fixtures` today.
- Ledger row ids, and the row count behind `stamped <ver>  <n> pass  0 fail`, are written as literals in tests and never imported from the product's compiled rows (per test-plan §11 Unit, §5 CLI `contract_ledger_probes.rs`).
- The Comprehensive-tier gates apply to the chunk diff (per test-plan §10 Coverage thresholds, Mutation gate):
  - coverage per OS of lines ≥ 85, functions ≥ 95 and regions ≥ 80
  - mutation `missed == 0`, `timeout == 0` and `unviable <= caught`, judged by the ubuntu + windows leg union
  - a mutant in `viola-agent-claude` must be killed by that crate's own tests, because cargo-mutants' default test scope is kept and the `viola-e2e` scenarios never run under mutation (per test-plan §3 `run` step 4 Test scope)

## Patterns to follow
- Root E2E tests use the rstest chain `home → fake_agent_path → stamped_home → booted_wrapper`. Each home is a not-yet-existing path under `target/e2e-home/`, so viola creates it with correct modes/DACL (per test-plan §3 Test data bootstrap, §3 `boot` step 2, §5 Setup / teardown lifecycle).
- `viola-agent-claude` unit tests (per test-plan §4 viola-agent-claude, §4 Fixture pattern at unit level):
  - `#[files("fixtures/claude/*/*.json")]` generates one test per recorded payload, and unknown fields are counted, never fatal.
  - The version gate against stamps is a unit case there, not only an E2E one.
- Unlisted-version tests use the fake agent's `--report-version <ver>`. It changes only the `--version` answer, and fixture replay still follows `--cli-version` / `--fixtures`. The fake agent answers `--version` in the real CLI's format for `--cli-version` (per test-plan §7 Fake agent, §6 Path 7 step 1).
- The human-mode `viola verify` step counter is pinned by a trycmd `tests/cmd/*.toml` case, with snapbox `[..]` redactions for paths and timestamps and no `\x1b[` under non-TTY / `NO_COLOR` / `TERM=dumb` (per test-plan §5 CLI).
- Test binaries follow the naming convention. Contract tests are `contract_<topic>.rs`, which the layer filterset runs in the E2E step, and functions are `<subject>_<condition>_<expected>` (per test-plan §2 File naming / Test function naming, §3 `run` steps 1–2).

## Anti-patterns to avoid
- NEVER hand-write `ledger/stamps.json`. NEVER run `viola verify` against the real CLI in CI. NEVER treat a green fake-agent run as proof of real-CLI behaviour: a behaviour change needs a local `verify` fixture refresh plus the contract suite (per test-plan §11 Test Data, §11 Test Strategy).
- NEVER import the product's ledger rows as the test oracle, and NEVER accept interactive snapshot review. insta/trycmd snapshots change only through the local `viola verify` fixture refresh, and then get committed (per test-plan §11 Unit, §11 Universal).
- NEVER write a test that passes when a decision body is emitted without a verify stamp and wheel `driver` (per test-plan §11 Universal). This applies to the transport-only degrade's withheld-decision path. Research must check whether any hook path emits a non-`null` decision today.

## Contract bindings
- tests ↔ security:
  - The single-writer negative asserts that `stamps.json` changes only when `viola verify` runs, never through a hook, `list`, `wait`/`last`/`send`, `mcp` or `ui` (per test-plan §6 Security control negatives, Single writers).
  - The canary/secret scan over `target/e2e-home/**` and `target/agent-run/` also covers what `verify` writes. Fixtures must carry no token and no R8-stripped `CLAUDE*` value (per test-plan §6 Error sanitization and secret scan).
- tests ↔ obs:
  - `verify`'s diagnostics role lines under `target/e2e-home/**` are validated by `schema-check` (G4) against `schemas/diag-line.v1.json`, and detail lines against `diag-detail.v1.json`.
  - The catch-site line `error: internal error` sends the chain only to the detail file (per test-plan §6 Schema conformance, §3 `logs`).
- tests ↔ arch:
  - `cli_verified` is a polled `status` item field. It must match between `list --json` and `/api/sessions` once those readers land (Epochs 5/8) (per test-plan §3 `status`, Status endpoint shape).
  - Path 7's `answer` exit 12 `unverified-cli` needs Epoch 3 driver verbs. This chunk owns its `verify` step 5 and hook-stdout-empty bullets (per test-plan §6 Path 7).
- tests ↔ design: the human result line `stamped <ver>  <n> pass  <n> fail` is the cli surface's line-match signal (per test-plan §1 Surfaces under test, cli).

## Acceptance criteria contributions
- `contract_ledger_probes.rs` runs `viola verify --home <tmp>` against the fake agent for each `fixtures/claude/<ver>/` set. The pass criteria are (per test-plan §5 CLI, §2 Contract row):
  - every literal ledger row id is listed exactly once, each with its post-condition result
  - the `<n>` in `stamped <ver>  <n> pass  0 fail` equals the literal row count
- Harness `boot` without `--unstamped` runs `viola verify`, which exits 0 with a last line matching `^stamped \S+  \d+ pass  0 fail$`, and readiness checks `events.ndjson` line 3 `session-start{source:"hook"}`. A `boot --unstamped` home has no `ledger/stamps.json` (per test-plan §3 `boot` step 4 and Readiness signal).
- The `tests/contract_fixture_hygiene.rs` walk covers `fixtures/claude/*/*.json`. Every committed fixture passes, and planted inputs turn each class red: `absolute-path`, `username`, `schema` (per test-plan §7 Fixture hygiene).
- `agent-run run` is green on the chunk: nextest unit, integration and e2e plus doctest pass, coverage is ≥ 85/95/80 per OS, and the ubuntu + windows mutation union has zero missed or timed-out mutants and `unviable <= caught` (per test-plan §3 `run`, §10 Coverage thresholds / Mutation gate).
