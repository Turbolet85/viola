# Report — 2026-09-24-diagnostics-plane

**Chunk:** closed ObsEvent vocabulary + obs_event!, per-role JSON sinks with service identity, diagnostics_level, owner-only detail files, diag-line schema, torn-line-aware logs merge
**Date:** 2026-09-24T10:45:00Z
**Commits:** none since `last_wrap` 2026-09-24T09:52:10Z. HEAD is `4d8be52`, and the whole chunk rides this wrap's commit.

## Changes (structured — detectors read this)
- **Files:**
  - New: `crates/viola-core/src/obs.rs`, `src/obs.rs`, `schemas/diag-line.v1.json`, `schemas/diag-detail.v1.json`,
    `tests/contract_diag_schema.rs`, `scripts/guard-probe.py`.
  - Modified: `crates/viola-core/src/lib.rs`, `src/main.rs`, `src/run/mod.rs`, `src/cmd/run.rs`,
    `crates/viola-e2e/src/harness/logs.rs`, `tests/run_cli.rs`, `Cargo.toml`, `.claude/settings.json`.
  - Bookkeeping: `viola-0.1.0/verification-matrix.json` (3 notes), `viola-0.1.0/working-route.md` (the P1 stamp),
    `.andromeda/master-route.md` (the pending record). Basis: `git status --short` at wrap Setup.
- **Symbols / APIs:**
  - **`viola_core::obs` (new, pub):**
    - `ObsEvent`: a 19-variant closed enum. `as_str`/`Display` are kebab, and `ALL` is a const array. There is no
      `a11y-violation` variant.
    - `ObsProcess` (`run|hook|mcp|ui|cli`, `ALL`); `ProcessCtx { process, instance: Option<ViolaName> }`.
    - `set_ctx` (first call wins, OnceLock), `ctx`, `ctx_process`, `ctx_instance`.
    - `#[macro_export] obs_event!(LEVEL, ObsEvent::X, key = value, …)`. It expands to `::tracing::event!` at the caller
      inside `#[allow(clippy::disallowed_macros)]`, and attaches `event`/`process`/`instance` (absent keys when None).
      The message is the event name. `corr` is an ordinary typed field; the macro has no special arm for it.
  - `viola_core::MAX_FRAME: u64 = 16 MiB` (new pub const). Its first consumer is the `config.json` read.
  - **Root-bin `viola::obs` (new, crate-private):**
    - Helpers: `ensure_private_dir`, `open_private_append`, `role_file_name(process, instance, port)` (all five
      roles), `open_role_file`, `timestamp`, `MillisUtc`. The previous `src/run/mod.rs` helpers moved here.
    - Setup and levels:
      - `targets(level)` (the viola crates at level, everything else OFF) and `subscriber(writer, level)` (the builder
        chain);
      - `viola_obs_init(home, process, instance, level)`, which sets the ctx, panic sink, start instant and global
        subscriber;
      - `duration_ms()`;
      - `read_diagnostics_level(home) -> (Level, Option<ConfigRejection>)` and `log_config_rejection`.
    - The detail and panic side: `panic_exit_line`, `detail_path`, `detail_line`, `write_detail`.
  - **`src/main.rs`:**
    - `PanicSink` now carries `ObsProcess`, `Option<ViolaName>` and the home. `set_panic_sink` gained a 4th argument.
      Its callers are `viola_obs_init` (sole production caller) and one test.
    - The hook writes the unchanged home-level line, then a `panic_detail_line` into the detail file.
    - The main-thread catch site calls `obs::panic_exit_line()` before exit 1.
  - **`src/run/mod.rs`:** `log_self_start()`, `log_child_start(pid)`, `log_child_exit(Option<i32>)` and
    `log_self_exit(code, detail)` lost their `&ViolaName` parameter and are now `obs_event!` calls. The six raw
    `tracing::info!/error!` sites are gone. Their sole caller is `src/cmd/run.rs` (research graph query, 1 caller
    each).
  - **Harness `viola_e2e::harness::logs`:** new `pub fn detail_lines(instances, filter)`. `wrap_lines` keeps its
    signature and delegates to a private `wrap_in` that adds `instance`. `logs()` now appends the detail source.
- **Crates / modules:** new module `viola_core::obs`; new root-bin module `obs`. No crate was added or removed.
- **Dependencies:** none added or bumped. `Cargo.lock` is unchanged (`git status` shows no lockfile line).
  `viola-core` normal deps are still only `nutype` (gate `cargo tree -p viola-core -e normal`: 0 tracing/tokio lines).
- **Schema / config:**
  - **New `config.json` key `diagnostics_level`** (`"info"` default | `"debug"`). Read facts:
    - The read is tolerant, capped with `Read::take(MAX_FRAME)`, and requires `v == 1`.
    - Known keys are `{v, diagnostics_level}`. Any other key is counted and reported as
      `parse-rejected{parser:"config-json", detail:"unknown-keys", count}` at WARN.
    - A bad value, a bad or missing `v`, or a non-object gives `detail:"malformed"` with `count:1`. An unreadable file
      gives `detail:"unreadable"`.
    - An absent file gives INFO with no line.
    - It never reads an env var.
  - **New `schemas/diag-line.v1.json`** (JSON Schema 2020-12):
    - Required keys: `timestamp`, `level`, `target`, `message`, `event`, `process`. `instance` is an optional string
      with the ViolaName pattern.
    - The event enum has 19 values. There is one `message == event` rule per event, and the additive fields are keyed
      per event from obs-plan §6.
    - `corr` is allowed only on channel-*/dialog-*/hook-*/send-*/release-from-driver and is typed `number|string`, so a
      literal `null` is rejected.
    - Default-deny is enforced by a top-level **`unevaluatedProperties: false`**, NOT `additionalProperties: false`
      per event. The per-event `additionalProperties` form cannot see fields declared inside `allOf`/`if-then`;
      measured against the conformance tests.
  - **New `schemas/diag-detail.v1.json`:**
    - Required keys are the home-level set plus `instance`.
    - Allowed on top of those: `corr`/`conn`; `panic_location`/`thread` on `panic` lines only; and the content fields
      `panic_payload`, `backtrace`, `chain`, `drift_report`. `unevaluatedProperties: false`.
    - The 19-value event enum is inlined, because jsonschema is built without default features and has no remote
      `$ref` resolver.
  - **Detail line:** hand-formatted with the home-level keys plus fields, one `write_all`. The panic detail adds
    `panic_payload` (`payload_as_str`, else the fixed `non-string payload`) and `backtrace` (a string array from
    `Backtrace::force_capture()`).
- **Spec-master edits:** none. This chunk made no edits to `.andromeda/` masters; the wrap's P2 owns them.
- **Counts / qualifiers moved:**
  - Harness `logs` sources: home-level diag + instance detail. The events source is still absent.
  - `ObsEvent` / schema event enum = 19. Basis: `obs-plan.md:650-656` + D-01…D-05. The contract test pins the
    literal list.
  - Home-level role file names now implemented for all 5 roles (only `run` has a producer).
  - The arch registry row `diagnostics/: … (run-<name>.ndjson today …)` (architecture.md:369) no longer matches: the
    naming path for `hook-<name>`/`mcp`/`ui-<port>`/`cli-<name>` exists in code.
- **Dev-tool versions:** none. Git Bash re-read at GNU bash 5.2.37 (x86_64-pc-msys). cargo-deny 0.20.2 and zizmor
  1.30.1 were unchanged.
- **Harness / gate surface:**
  - `agent-run logs` now also streams `<home>/instances/*/diagnostics/detail-*.ndjson` as
    `{"src":"diag","file","instance","record"}`, and a torn line as
    `{"src":"diag","file","instance","torn":true,"offset":n}`.
  - Instance dirs pass `ViolaName::try_new`; symlinked dirs and files are skipped via `file_type`/`symlink_metadata`.
  - `--kind`/`--after` remain clap usage errors (exit 2 `reason:"usage"`, `viola-harness.rs:77-85`).
  - New dev probe `scripts/guard-probe.py`: it runs the rendered `.claude/settings.json` PreToolUse write guard under
    bash against six paths and prints the exits; expected last line `2,2,2,2,0,0`.
  - The `.claude/settings.json` write guard was repaired:
    - `path=${path//\\//}` became `path=$(printf "%s" "$path" | tr "\\\\" "/")`.
    - Measured: under Git Bash the old form deleted forward slashes (`a/b\c/d` → `ab\cd`), so the guard blocked nothing
      (0,0,0,0,0,0).
    - The new form gives 2,2,2,2,0,0.
- **Cross-project / external claims:**
  - **CI witness for `4d8be5287b00a00c772e9d0a4264f08d6942f512`** (the previous chunk's push):
    - `gh api …/commits/4d8be52…/check-runs`: 9/9 `success` (test ×3, lint ×3, supply-chain, mutants, advisories).
    - Run `35983992260` (ci, push) is `success`, and run `35984789181` (nightly, workflow_dispatch) is `success`, both
      read via `gh run view` at phase P1.
    - The overseer relayed this, and the phase-P1 re-read confirmed it.
  - **The overseer's one-off probe** `D:\dev\projects\additional\viola-overseer\guard_probe.py` (outside this repo) was
    read and run at phase P3. It asserts the shipped substring before running, so it cannot witness a fixed guard,
    which is why the repo-local probe replaced it.
- **Reverted / negative API facts:**
  - A `corr = …` special arm in `obs_event!` was considered and not written. It is ambiguous with the generic
    `key = value` arm in `macro_rules`.
  - A cross-file `$ref` from `diag-detail` to `diag-line` was written and replaced by an inlined enum, because there
    is no remote resolver.
  - The test `panic_hook_writes_exactly_one_line_without_payload` was removed by merging it into
    `panic_hook_writes_detail_line_with_payload_and_backtrace`. All its assertions were kept; both tests set the
    process-global panic sink.
- **Insufficient fixes:** none.
- **Spec claims disproved by measurement:**
  1. obs-plan §8 Default-deny posture (`obs-plan.md:1201`: "`schemas/diag-line.v1.json` (`additionalProperties: false`
     per event)") and `:1202` ("`schemas/diag-detail.v1.json`, with `additionalProperties: false`"). The committed
     schemas enforce default-deny with `unevaluatedProperties: false`. In JSON Schema 2020-12, `additionalProperties`
     does not see properties declared in `allOf`/`if-then` subschemas, so the keyword as written cannot express
     per-event field lists over shared required fields. Evidence: `tests/contract_diag_schema.rs`
     (`uncatalogued_key` case rejected, real run lines accepted) is green.
  2. architecture §Established Decisions [Hook Contract] (`architecture.md:67`: "Its diagnostics go only to the
     instance's `diagnostics/` directory") and §Cross-cutting Patterns → Diagnostic output channels
     (`architecture.md:492`: `mcp` "diagnostics go to stderr, or to the instance's `diagnostics/` when `VIOLA_DIR` is
     set"). The code now names home-level role files `hook-<name>.ndjson` / `mcp.ndjson` / `ui-<port>.ndjson` /
     `cli-<name>.ndjson` under `<home>/diagnostics/` (`src/obs.rs` `role_file_name`, unit-tested for all five roles)
     per obs D-08/D-09. The arch passages predate that.
  3. a11y-plan §12 Resolved questions (`a11y-plan.md:1348`: "`a11y-violation` event value: accepted as a tests + obs
     enum amendment"). The shipped `ObsEvent` and schema enum have no such value (contract test green), per the later
     Z7 entry in the same plan.
- **Expected amendments (from plan):**
  - arch §Occupied Resources → Filesystem (role files + detail files; `diagnostics_level` on `config.json`): **carried**
    (Schema/config + Counts bullets).
    - Sites: `grep -n -E 'diagnostics/|config\.json' architecture.md` gives 6 and 5 hits. Owning lines are :365
      (config.json row) and :369/:370 (diagnostics rows); :24, :321, :480, :482 and :492 are non-registry mentions,
      and :492 is covered by the next entry.
  - arch §Occupied Resources → Repository + tree (`schemas/diag-*.v1.json`): **carried** (Files bullet).
    - Sites: `grep -n 'schemas/' architecture.md` gives 3 hits: :377 (fixtures row, no change), :378 (schemas
      registry row) and :435 (tree comment).
  - arch §Established Decisions [Hook Contract] + §Cross-cutting Patterns → Diagnostic output channels: **carried**
    (Spec claims disproved #2).
    - Sites: `grep -c 'Hook Contract' architecture.md` gives 2 (:67 decision, :492 cross-reference);
      `grep -c 'Diagnostic output channels'` gives 1 (:492).
  - a11y-plan §12 stale `a11y-violation` bullet: **carried** (Spec claims disproved #3).
    - Sites: `grep -n -E 'a11y-violation.{0,40}(event value|accepted)' a11y-plan.md` gives 1 hit (:1348).
  - test-plan §3 `logs` (detail source landed; events + `--kind`/`--after` pending): **not carried**. test-plan §3
    `logs` (:601-606) already states the target shape, detail source included, and the interim usage-error rule is the
    phase-P4 "grammar that grows per chunk" decision (:1738). Sites: `grep -n -E 'logs.{0,60}detail|detail-\*'
    test-plan.md` gives :604 and :1681, both already describing the detail wrapper. No body change is needed.
  - The Wrap curation directive (not a master): narrow the CLAUDE.md learning "Every viola format carries `v`…" to name
    the diag-line exception. This is owned by P3 curation as an operator correction; see Decisions.
- **Coverage of new surfaces:**
  - `config.json` `diagnostics_level` read → validation take(MAX_FRAME)+tolerant parse+`v`+closed level✓ ·
    instrumentation parse-rejected log✓ · PII n/a · tests unit+integ · a11y n/a · tokens n/a
  - instance detail writer (`write_detail`) → validation ViolaName-built path✓ · instrumentation n/a (it is a sink) ·
    PII content only in the owner-only detail file✓ · tests unit · a11y n/a · tokens n/a
  - panic hook detail routing → validation n/a · instrumentation panic line + detail line✓ · PII payload detail-only,
    home line payload-free✓ · tests unit (+schema) · a11y n/a · tokens n/a
  - run catch-site `process-exit{internal-error}` → validation n/a · instrumentation✓ · PII n/a · tests unit (scoped
    subscriber) · a11y n/a · tokens n/a
  - harness `logs` detail source → validation ViolaName + symlink skip + `take(MAX_FILE)`✓ · instrumentation n/a · PII
    n/a (test-side reader) · tests unit · a11y n/a · tokens n/a
  - `scripts/guard-probe.py` → validation n/a · instrumentation n/a · PII n/a · tests gate entry (probe) · a11y n/a ·
    tokens n/a

## Deviations from intent
1. **Schema default-deny keyword:** `unevaluatedProperties: false` at the top level instead of plan step 10's
   "`additionalProperties: false` per event". The latter cannot express shared required fields plus per-event
   additive fields under `if/then` (JSON Schema 2020-12 semantics). The effect (uncatalogued key rejected) is the one
   the plan and obs §8 require. See Spec claims disproved #1.
2. **Inlined event enum** in `diag-detail.v1.json` instead of a `$ref` into `diag-line.v1.json`: there is no remote
   resolver under `default-features = false`. `diag_line_schema_event_enum_equals_the_log_format_literals` pins both
   copies to one literal list.
3. **Merged panic test** (see Reverted): both tests set the same process-global sink.
4. **Additions beyond plan step 14:**
   - Tests: `run_config_unknown_keys_are_counted`, `diag_line_schema_confines_corr_to_its_events`,
     `diag_detail_schema_allows_content_fields_only_there`, `detail_lines_skip_a_diagnostics_path_that_is_not_a_dir`,
     `targets_enable_viola_crates_at_the_level_and_nothing_else`, `config_rejection_detail_and_count`,
     `log_config_rejection_writes_one_warn_line`, `subscriber_level_gates_debug_lines_only`,
     `open_role_file_without_a_name_creates_nothing`, `write_detail_under_a_file_drops_the_line`.
   - Public consts `ObsEvent::ALL` / `ObsProcess::ALL`. Tests iterate them but compare against literal oracles.
5. **Count semantics:** `ConfigRejection` count is 1 for `malformed`/`unreadable` (the plan named no value), and `{}`
   (no `v`) is `malformed`, per plan step 5's "`v` not equal to `1`".
6. **Gate entry `test -s scripts/sync-crates.txt && while read -r c; … check bans …` needed `env = []`:**
   - At /implement it was reported `not run — env c unset`, because gate.py scavenged the loop's own `$c`.
   - The exact text was run by hand (exit 0, `bans ok`).
   - Post-implement, the overseer (founder-delegated) directed adding `env = []` to plan.md. The edit was made between
     runs and `gate.py run --only 12` was green (`bans ok`).
   - The previous chunk's plan carried `env = []` on the same entry, and phase P4 dropped it when copying.

## Decisions & corrections
- **Operator (P4 fork):** the guard fix is proven by a repo-local probe that tests the rendered guard as-is; the
  overseer's `guard_probe.py` was a one-off measurement.
- **Operator (P5 review):** narrow the CLAUDE.md learning "Every viola format carries `v`…". It should name the
  diag-line exception: process-log lines carry no `v`, the version is in the schema filename, per the obs-plan §6
  field list and §8 default-deny. That way the two rules no longer contradict.
- **Operator (post-implement):** add `env = []` to the sole-root deny loop entry. A `run` that sets its own shell
  variable needs `env = []`, or gate.py reads the variable as an unset handle and skips the entry.
- **Overseer note folded at phase P1:** the write-guard repair plus recording the `4d8be52` CI witness here.
- **Decision:** no product-side panic trigger (env/flag). Panic routing is proven in-process
  (`src/main.rs` test with a temp sink).
- **Decision:** diag lines carry no `v`; the version lives in the schema filename.
- **Decision:** the concurrent-append check and argv role classification are deferred to the hook chunk (first
  shared-writer / exit-0 role).
- **Measured:** `path=${path//\\//}` under Git Bash 5.2.37 deletes forward slashes (`a/b\c/d` → `ab\cd`); a guard
  built on it matched nothing.
- **Sweep hazard:** the CARRY grep `tracing::(info|error|warn|debug|trace)!` deliberately excludes `tracing::event!`,
  because `obs_event!` expands to it. A naive `tracing::` grep over `crates/viola-core/src/obs.rs` would read the
  sanctioned macro as a raw call.
- **Sweep hazard:** tests that set a process-global `OnceLock` (`PANIC_SINK`, `ProcessCtx`) are correct only under
  nextest's process-per-test. The harness and `run --mutants` (`--test-tool=nextest`) both use nextest; a bare
  `cargo test` would interleave them.
- **Measured:** jsonschema 0.57 with `default-features = false` resolves no cross-file `$ref`. Keep each schema
  self-contained.

## Outcome
**Acceptance criteria** (re-asserted against the diff):
- Every `run` role line validates against `diag-line.v1.json`, `message == event`, no literal null, and
  `process-start{self}` carries the identity: **met** (`diag_lines_from_real_runs_validate`: 11 lines over 3 runs).
- `process-exit{subject:"self"}.duration_ms` on both exits: **met** (`run_self_exit_carries_duration_ms`: ≥ 60 ms
  lower bound held).
- The schema and `ObsEvent` enums each equal the literal 19 with no `a11y-violation`; a null corr/instance is rejected
  and absence accepted: **met** (viola-core `obs_event_display_matches_the_log_format_literals`; contract tests).
- The panic routes its payload and backtrace to `detail-run.ndjson` only, the home line has no payload, nothing goes to
  stderr, and on Unix the modes are 0600/0700: **met on the Windows host**. The Unix mode asserts
  (`write_detail_creates_owner_only_dirs_and_one_line`) run on the CI Unix legs after the push.
- The catch-site `process-exit{internal-error}` at ERROR: **met** (`panic_exit_line_writes_run_internal_error`).
- The level comes only from `config.json`: **met**. Debug keeps the key set; malformed and unknown keys give
  `parse-rejected` at WARN; `RUST_LOG=trace` is inert.
- `viola run` writes zero bytes of its own to stdout/stderr on both paths: **met** (`run_is_silent_on_stdout_and_stderr`).
- `logs` detail wrapper, torn marking and invalid-name skip: **met** (`detail_lines_*`); the booted-session `logs`
  entry is green.
- `viola-core` gains no tracing/tokio edge, and deny is green: **met**.
- CARRY closed (0 raw tracing macros): **met**. PREREQ closed (`run --unit`: 135 passed): **met**. `run --mutants`:
  **met** (`verdict:"counted"`, 0 survived).
- Guard probe `2,2,2,2,0,0`: **met**.
- CI on the operator's push: pending. That is the operator entry after this wrap's push.

**Gates** (/implement run `.andromeda/runs/2026-09-24T10-22-27-implement`, logs under
`andromeda-gate/…/implement-2026-09-24T10-22-27`):
- `cargo build --workspace --features fake-agent`: green
- `cargo fmt --all --check`: green
- `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`: green
- `bash scripts/agent-run.sh run --unit`: green (135 passed; PREREQ close)
- `bash scripts/agent-run.sh run --integration`: green (82 passed, 1 leaky: the existing
  `fake_agent_exit_no_eof_exits_while_stdout_is_held`, which holds stdout by design; not this chunk's)
- `… run --unit --filter 'test(/obs::tests::/)'`: green (36 run)
- `… run --unit --filter 'test(/panic_hook_writes_detail_line_with_payload_and_backtrace|detail_lines_/)'`: green
  (4 run)
- `… run --integration --filter 'binary(contract_diag_schema)'`: green (9 run)
- `… run --integration --filter 'test(/run_self_exit_carries_duration_ms|run_config_|run_rust_log_|run_is_silent_/)'`:
  green (6 run)
- `grep -rn -E 'tracing::(info|error|warn|debug|trace)!' src crates --include=*.rs`: green (exit 1, no output)
- `cargo tree -p viola-core -e normal --prefix none | grep -cE …`: green (exit 1, last line 0)
- `test -s scripts/sync-crates.txt && while read -r c; … check bans …`: `not run — env c unset` at /implement, then run
  by hand (exit 0, `bans ok`). After the operator-directed `env = []` edit, green through gate.py (`--only 12`,
  `bans ok`).
- `cargo deny check`: green
- `grep -rnE "(std::)?env::set_var" tests crates/viola-e2e src`: green (exit 1, no output)
- `python -X utf8 scripts/guard-probe.py`: green (last line `2,2,2,2,0,0`)
- `agent-run.sh cleanup/boot/status/logs/cleanup --session gate-smoke`: green (state ready; logs contains
  `"service_name":"viola"`; `processes_gone:true`)
- `AGENT_RUN_CHUNK_BASE=4d8be52… bash scripts/agent-run.sh run --mutants`: green (93 mutants, 65 caught, 28 unviable,
  0 survived, `verdict:"counted"`)
- `git diff --quiet && git diff --cached --quiet && git push origin build/viola-0.1.0`: `leg operator`. This wrap's P7
  push discharges it.
- `gh api …/check-runs …`: `leg operator`. Read after the push; pending.
- **Smoke** (/implement P3, boot path changed): ✓. Session `p3-smoke` booted ready (wrapper 6268, child 21684), status
  was ready, logs showed the migrated lines with identity fields, and cleanup reported `processes_gone:true`.

**Outcome basis:** /implement's P4 report in this session's conversation, plus one operator directive after it (the
gate 12 `env = []` edit and its green re-run).

**Process hygiene:**
- The gate-smoke and p3-smoke wrappers and children, cargo-mutants and nextest: all terminated (implement census,
  `Get-Process`).
- Re-measured at wrap: no process of this repo is running.
- `viola.exe` 12172 and 35652 are the operator's viola-lab prototype (`additional\viola-lab\prototype`), not started by
  this chunk. They are left running for the operator.
