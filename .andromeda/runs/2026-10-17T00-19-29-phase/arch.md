# arch extract

## Relevance
partial — test-plan §3 owns the harness command contract and §6 owns the scenarios; arch adds where the code goes (the test-only `viola-e2e`), which resources it may touch (env vars, harness files, endpoints) and the product contracts the new legs assert on (the `list` payload, the budget gate, exit codes).

## Constraints
- New code (the `path_` test binaries, `run --e2e`, the `boot` flags, the `status` change, the cleanup fix) belongs in the test-only member `viola-e2e`. No product crate may depend on it, and the chunk must add no product dependency edge. Its own dependencies are not listed in the plan, so a new `viola-e2e` dependency (for example on `viola-channel` for the squat listener) does not breach the direction rules. Whether `viola-e2e` can already reach the endpoint-name function and the interprocess server API is research's question (per architecture §Established Decisions [Module Boundaries]; §Infrastructure Patterns → Crate dependency direction).
- No new environment variable. The harness reads only the listed test-harness variables (`AGENT_RUN_CHUNK_BASE`, `AGENT_RUN_KEEP_HOMES`, `AGENT_RUN_KEEP_FAILED`, `CI`). The fake agent's `agents` mode takes no option and no env var, only the `agents-mode` file beside its own executable with the closed values `recorded` · `oversize` · `malformed` (absent reads as `recorded`). So `boot --agents-mode` must write that file beside `target/agent-run/<session>/bin/claude[.exe]` and set no env var (per architecture §Occupied Resources → Environment variables / Binary, subcommands and exit codes).
- `statusline-source.json` is never written by viola, and tests plant it. `run` reads it at every start through `take(MAX_FRAME)`, and a `--home` session never falls back to `~/.claude/settings.json`. So `boot --statusline-echo` is a test-side write of the session home's source and must change no product read (per architecture §Occupied Resources → Filesystem; §Cross-cutting Patterns, Config management).
- The budget gate never polls. It evaluates at three points only: at start, when a `send` arrives, and at each `turn-ended`. `release --budget` lifts the pause for one instance only, until the crossed window's `resets_at`, and no override outlives its wrapper. The two-instance leg must therefore expect `overseer` to stay paused, and a snapshot may still read `budget_paused: true` until the next gate point (per architecture §Established Decisions [Budget Governor]).
- Every OS-specific branch compiles and is tested on its own CI runner. On Windows no settings override is written and the statusline arm runs no command. So the `--statusline-echo` marker assertion is Unix-only, and its Windows branch must be stated (per architecture §Cross-cutting Patterns, Cross-platform discipline; §Established Decisions [Budget Governor]).
- The base `target/e2e-home` is prepared only by the two test-side keepers (`prepare_home_base`, `Workspace::ensure_e2e_home`). On Unix a linked base whose target is gone gets one non-recursive 0700 create, never a chmod afterwards and never a parent. The cleanup case's fix must go through that rule, not a bare `create_dir_all` over the link (per architecture §Occupied Resources → Repository, `target/e2e-home/...`).
- The real `claude` never runs in CI. Every new E2E case runs against the fake agent on all three runners (per architecture §Infrastructure Patterns → CI/CD approach).

## Patterns to follow
- Harness output stays under the gitignored `target/agent-run/<session>/` (the session record, the supervisor handshake files and the `bin/claude[.exe]` copy), and its builds and test runs stay under `target/harness/` as `CARGO_TARGET_DIR`. The new `agents-mode` file and the `session.json` / `supervise.json` value sit inside that session dir (per architecture §Occupied Resources → Repository).
- Harness session homes live at `target/e2e-home/viola-session-*/home`, stamped at `boot` step 4 by `viola verify` against the fake agent, and are kept in CI for the obs gates under `AGENT_RUN_KEEP_HOMES=1`. The Path 6 harness-session leg uses such a home, not an outside-scan home (per architecture §Occupied Resources → Repository).
- Test homes take fake-agent control and receipt files under `fake/` (`fake/<name>.control`, `fake/<name>.receipt.ndjson`), and viola never reads them. The statusline marker under `<home>/fake/` follows the same test-only convention (per architecture §Occupied Resources → Filesystem).
- The nextest profiles live in `.config/nextest.toml` (`ci`, `mutants`), and `run --e2e` runs under the existing `ci` profile (per architecture §Infrastructure Patterns → Project directory structure).

## Anti-patterns to avoid
- Do not use a `viola-<h12>` endpoint name for a test-owned endpoint other than the Path 1 squat. The test-only endpoints sit outside that namespace (`viola-test-chan-<pid>-<label>`, `viola-test-sqos-…`) and are unique per test. The squat alone binds the instance's real recorded endpoint, and on Unix it lives in the per-user 0700 socket dir, never in the `/tmp` root or an abstract namespace (per architecture §Occupied Resources → IPC endpoints).
- No test-only binary or feature in a release artifact. `viola-harness`, `viola-fake-agent` and anything built with `fake-agent` / `test-support` stay out of `cargo build --release --bin viola`, which the plan's release job enforces (per architecture §Infrastructure Patterns → CI/CD approach, target job 6).
- No shelling out through `sh`, `bash` or `cmd` from the new harness code. Children are spawned directly, and `hook statusline` running the user's command stays the only shell-out (per architecture §Cross-cutting Patterns, Cross-platform discipline).

## Contract bindings
- arch `list` payload ↔ tests harness `status`: `status` nests the verbatim `viola list --json` document. Its `ready` / `degraded` derivation reads each item's `liveness` (`live` · `stale`; gone instances are omitted) and the `wrapped` flag (unwrapped rows are read-only `claude agents --json` rows, not booted instances) (per architecture §Standard Contracts, `list` payload).
- arch CLI exit codes ↔ tests Path 1 / Path 6 signals and `status`'s `down` / `last_error` (per architecture §Conventions, CLI exit codes):
  - `list` exit 21 refuses the whole board (`list-exit-21`);
  - `send` exit 11 means `budget-paused`;
  - `run` exit 1 reads `already-live` or `squatted-name`.
- arch CI/CD approach ↔ tests CI wiring: `ci.yml`'s `test` job runs `run --coverage`, then the boot → status → logs → cleanup lifecycle under `AGENT_RUN_KEEP_HOMES=1`, and its `Gate verdict` is `gate --require coverage,doctest`. Two things for research:
  - whether the E2E binaries ride the coverage suite or need a `nextest-e2e` step or a `--require` entry;
  - whether the lifecycle's `status` still reads `ok` now that it runs `viola list`.

  (Per architecture §Infrastructure Patterns → CI/CD approach.)
- arch Budget Governor ↔ obs `budget-gate` record ↔ tests Path 6 step 7: `budget-gate` is appended, and the snapshot's `budget_paused` / `budget_override_until` are rewritten, only when the triple (paused, window, override end) changes. That is why exactly one `budget-gate{paused:false, override_until}` is expected, for `builder` only (per architecture §Established Decisions [Budget Governor]).

## Acceptance criteria contributions
- (arch) No new environment variable is read by `viola`, `viola-harness` or `viola-fake-agent`. `--agents-mode` reaches the fake agent only as the `agents-mode` file beside the session's `bin/claude[.exe]` (per architecture §Occupied Resources → Environment variables).
- (arch) The new code lives in `crates/viola-e2e` (its `path_*` test binaries and harness modules), and no product crate gains a dependency on `viola-e2e` (per architecture §Established Decisions [Module Boundaries]).
- (arch) In the two-instance leg, after `viola release builder --budget`, `list --json` reads `builder` `budget_paused:false` and `overseer` `true` while the top-level `budget.paused` stays `true`. `send` to `overseer` exits 11 (per architecture §Standard Contracts, `list` payload; §Conventions, CLI exit codes).
- (arch) The new cases are green on all three CI runners against the fake agent, and each Unix-only assertion is under `cfg(unix)` with its Windows branch stated (per architecture §Cross-cutting Patterns, Cross-platform discipline).
