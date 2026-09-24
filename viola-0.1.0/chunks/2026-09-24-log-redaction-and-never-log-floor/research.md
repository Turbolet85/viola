# Codebase Research — 2026-09-24-log-redaction-and-never-log-floor

## Scope
- **Depth:** moderate (small codebase: 2 product crates, 1 423 lines across the 7 product sources — `wc -l src/main.rs src/cmd/*.rs src/run/mod.rs src/obs.rs crates/viola-core/src/*.rs`) · **Reads:** 9 · **Globs/Greps:** 9 · **Graph queries:** 1 (rust plane, 18 rows)
- **Harness rules consulted:** `.claude/rules/verification-harness.md` (read in full; 0 Session Additions) · `.claude/rules/testing.md` (read in full; 4 Session Additions — the observable-effect rule and the one-test-per-`OnceLock` rule both apply below)
- **Platform issues consulted:** Rust std `io::ErrorKind` (fetched doc.rust-lang.org/std/io/enum.ErrorKind.html at P5). It states `NotADirectory` — "A filesystem object is, unexpectedly, not a directory. For example, a filesystem path was specified where one of the intermediate directory components was, in fact, a plain file." — stable since 1.83.0, and distinct from `NotFound` ("An entity was not found, often a file."). The runner-only bullet's signature is a coverage gap in viola's own test, not a hosted-platform failure. The mechanism is re-derived from the test source plus measured open semantics (below), and no instrumentation is planned.

## Files inspected
- `src/main.rs` (full) — the catch site `main()` at :42-52. `Err(_) => ExitCode::from(1)` at :46 drops the anyhow error, writing nothing to stderr, the role file or a detail file. The panic arm at :48-51 calls `obs::panic_exit_line()`. `viola_panic_hook` (:57-89) is the working model of the role-line + detail-line pair: a payload-free role line, then `obs::write_detail` with `panic_payload` + `backtrace` only when an instance is known. `PANIC_SINK` (:26) holds `file · process · instance · home` and is set only inside `viola_obs_init`, after the role file opens.
- `src/obs.rs` (full) — `read_diagnostics_level` :190-200 (the folded mutant at :193). `detail_line` :261-282 and `write_detail` :286-297 are the shipped detail sink. `panic_exit_line` :239-250 is `run`-only. `ConfigRejection` :164-186 carries fixed codes. The only `Unreadable` test is `read_diagnostics_level_unreadable_config_is_reported` :425-433, which makes `config.json` a **directory**.
- `src/cmd/mod.rs` (full) — `dispatch` :29-39 resolves `--home`, else `home_dir()` with `.context("no user home directory")?` (:33). That is the only `.context` in the product: `grep -rn "context(" src crates/viola-core` → 1 hit.
- `src/cmd/run.rs` (full) — `run` :24-49. `viola_obs_init(...)?` at :26 and `child.wait()?` at :39 are the two anyhow `?` sites after the instance name is parsed. A spawn failure (:44-47) is handled in place: role line `process-exit{detail:"internal-error"}`, `Ok(1)`, silent.
- `src/run/mod.rs` (full) — the role-line emitters and `spawn_child` (inherited env, no R8 strip yet).
- `crates/viola-core/src/lib.rs` (full) and `Cargo.toml` — `viola-core` depends only on `nutype`. It has no error enum and no thiserror dependency.
- `Cargo.toml` (root, :1-100) — no `veil` and no `serde_path_to_error` in `[workspace.dependencies]`. The root bin depends on `anyhow`.
- `schemas/diag-detail.v1.json` (full) — admits `chain: [string]` and `drift_report`, with the 19-value inlined event enum including `process-exit`. `unevaluatedProperties: false`. `panic_location` / `thread` are allowed only on `event:"panic"`.
- `tests/run_cli.rs` (:1-25, :100-319) — the `run_viola` helper already plants `CLAUDE_CODE_MESSAGING_TOKEN=canary-token-value-7f3a` (:21), but only the role text is checked against it (:105). `run_is_silent_on_stdout_and_stderr` (:288-303) pins an EMPTY stderr on the spawn-failure exit-1 path.
- `tests/support/{home,fake}.rs` (fn index) — the fake-agent receipt lives at `<home>/fake/<name>.receipt.ndjson` (fake.rs:18-19). Its `env` receipt records env var NAMES only (`src/bin/viola-fake-agent.rs:319-324`), so a canary value never reaches it.
- `crates/viola-e2e/src/harness/run.rs` (:400-459) and `.github/workflows/ci.yml` (:83-110) — the mutation gate runs `cargo mutants --workspace --features fake-agent --in-diff <chunk.diff>`, where the diff is from `merge-base(base, HEAD)` and base is `github.event.before` on push.

## Graph impact (rust plane; trace `tree-query-2026-09-24-log-redaction-and-never-log-floor.json`; editor lines = SCIP line + 1)
- **dispatch** — 1 caller: `main` @ `src/main.rs:44`. The catch site is the single consumer of the anyhow result, so the change is confined to `main` + whatever carries home/instance to it.
- **read_diagnostics_level** — 3 callers: `cmd/run::run` @ `src/cmd/run.rs:25`, and tests `level_of` @ `src/obs.rs:386` and `read_diagnostics_level_unreadable_config_is_reported` @ `src/obs.rs:430`. The fix adds a test and changes no signature.
- **write_detail** — 1 product caller, `viola_panic_hook` @ `src/main.rs:87`, plus 2 tests. **detail_line** — 1 product caller, `panic_detail_line` @ `src/main.rs:111`, plus 1 test. The chain routing is their second product consumer.
- **set_panic_sink** — callers `viola_obs_init` @ `src/obs.rs:151` and a test @ `src/main.rs:249`. **viola_obs_init** — 1 caller, `run` @ `src/cmd/run.rs:26`. **panic_exit_line** — `main` @ `src/main.rs:49` + tests. **log_self_exit** — `run` @ `src/cmd/run.rs:41,45`.

## Patterns detected
- **Role-line + detail-line pair** (`src/main.rs:57-89`): a codes-only role line, then the content-bearing detail line through `obs::detail_line` + `obs::write_detail`, written only when an instance is known. Every failure drops the line silently. The catch-site chain follows this shape.
- **Fixed codes, never text** (`src/obs.rs:171-186`, `src/cmd/run.rs:44-47`): failures surface as closed kebab codes (`unreadable`, `internal-error`). A path, error text or payload never reaches a role line.
- **Env names only** (`src/bin/viola-fake-agent.rs:319-324`): the only env enumeration in the tree records names, never values.

## Conventions to follow
- **Tests inline + rstest tables** (`src/obs.rs:389-423`), `<subject>_<condition>_<expected>` naming, real temp files, never mocks.
- **One test per process-global `OnceLock`** (testing.md Session Addition): `PANIC_SINK` and `ProcessCtx` are already set by one test each (`src/main.rs:238`, `src/obs.rs:547`). A catch-site test that sets either needs nextest's process-per-test isolation and must not add a second setter of the same global.
- **Observable effect or leave it out** (testing.md Session Addition): a function with no consumer is an unkillable mutant under the zero-missed gate.

## Measured facts (the equalities the plan leans on)
- **Open semantics, measured this session.**
  - **Linux 6.6 (WSL2, busybox `cat`):** opening a directory SUCCEEDS and the failure comes at read (`cat: read error: Is a directory`). Opening `<regular file>/config.json` FAILS at open (`can't open …: Not a directory`, ENOTDIR).
  - **Windows 11 (python `open`):** a directory → `PermissionError` errno 13, and a file-as-parent → `FileNotFoundError` errno 2.
  - Rust maps ENOTDIR → `io::ErrorKind::NotADirectory` (≠ `NotFound`) and Windows `ERROR_PATH_NOT_FOUND` → `NotFound`.
- **The missed-mutant mechanism, re-derived.**
  - **On Linux,** the existing directory test reaches `Err(_)` at `src/obs.rs:197` (a read error), never the open-error arm at :194. The mutant `guard → true` at :193 is therefore unobserved there.
  - **On Windows,** the same test fails at open with `PermissionDenied`, which is :194, so the mutant is caught locally. This is the observed split: 0 survived locally, 1 missed on ubuntu.
- **The kill equality.** `read_diagnostics_level(home)` with `home` = a regular file yields `(INFO, Some(Unreadable))` on Linux and macOS through :194, because ENOTDIR ≠ `NotFound`. Under the mutant, every open error matches :193 and the call yields `(INFO, None)`, so a Unix-cfg case asserting `Unreadable` kills it. The same input on Windows yields `(INFO, None)` legitimately (`NotFound`), and the existing directory case keeps covering :194 there.
- **The mutation gate is diff-scoped** (harness `run.rs:426-440`, `--in-diff`). A chunk diff that adds only a test never re-generates the `:193` mutant, so a green `mutants` job on this chunk's push would be vacuous for it. The ubuntu `test` job running the new case green is the direct witness of the equality above.

## Subjects at HEAD (the working entry's four items, measured)
- `#[instrument` in product code: 0 hits (`grep -rn --include=*.rs "instrument" src crates tests`). Skip-all has no site to convert. G1 is "Observability gates"' job.
- `<Crate>Error` thiserror enums in product crates: 0 (`grep -rn -E "thiserror|enum [A-Za-z]*Error" src crates tests` → only `HarnessError` in the test-only `viola-e2e`). There is no `CoreError`: the security extract's "`CoreError` is the one at HEAD" is false. Product `Display` impls are `ObsEvent` / `ObsProcess` (`crates/viola-core/src/obs.rs:82,112`), fixed kebab strings.
- Payload types (send/hook/answer params): 0. They belong to crates that do not exist yet. `veil`: absent from every manifest (`grep -rn veil` → only the `deny.toml` toggle ban).
- serde drift reports: 0 producers (`grep -rn serde_path_to_error` → 0). The one serde parse, `parse_diagnostics_level` (`src/obs.rs:202-226`), maps every error to the fixed `Malformed` code, so no serde source can join an anyhow chain at HEAD.
- anyhow chains: 3 `?` sites (`src/cmd/mod.rs:33`, `src/cmd/run.rs:26,39`). All end silently at `src/main.rs:46`. **This is the chunk's one live leak-shaped surface:** the chain is lost today, not leaked, so the work is to route it (to the detail file) and to give the human line its fixed form.

## New files to create
- none required. The tests land inline (`src/main.rs`, `src/obs.rs`) and in the existing `tests/run_cli.rs`.

## Files to modify
- `src/main.rs` — the catch site: the fixed `error: internal error` stderr line, and the chain → `detail-<process>.ndjson` routing when home + instance resolved. Plus the paired home-level `process-exit{internal-error}` when the role file is open.
- `src/cmd/mod.rs` / `src/cmd/run.rs` — carry the resolved home + instance to the catch site (`dispatch` is `main`'s only callee, graph above). Where it lives is an implement call.
- `src/obs.rs` — a chain detail-line helper beside `detail_line` / `write_detail` if the routing lives here, and the Unix-cfg `read_diagnostics_level` case (the mutant kill).
- `tests/run_cli.rs` — integration: a forced catch-site error with a resolved instance (make `<home>/diagnostics/run-builder.ndjson` a directory, so `viola_obs_init` fails at `src/cmd/run.rs:26` while the instance detail dir stays creatable). Also extend the canary scan from the role text (:105) to every home file incl. detail files, the error path and `diagnostics_level: debug`. Sweep `run_is_silent_on_stdout_and_stderr` (`tests/run_cli.rs:288-303`): **no change** — its exit-1 path is the spawn failure handled in `run` (:44-47), not the catch site.
- `tests/contract_diag_schema.rs` — **no change** expected. `diag_detail_schema_allows_content_fields_only_there` (:141-160) already pins `chain` as detail-only. The new chain line is validated in the new tests against the same schema file.

## Open questions
- How is the `:193` kill WITNESSED on Linux, given the diff-scoped gate cannot re-generate that mutant? (By-construction via the ubuntu `test` job, or a targeted Linux cargo-mutants run?) → blocks: plan-decision
