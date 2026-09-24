# Report — 2026-09-24-observability-gates

**Chunk:** obs CI gates: panic-hook-first, print/dbg + raw-tracing lint bans, G1–G4, canary secret scan before scan-gated uploads; mutants-timeout cause fix for the 2834e4d Linux red
**Date:** 2026-09-24T13:10:00Z
**Commits:** none since `2969198` (docs: prototype field report). This wrap's commit carries the chunk.

## Changes (structured — detectors read this)
- **Files** (`git diff --numstat HEAD` plus untracked, 20 source files):
  - Modified:
    - `.config/nextest.toml` (+9/−2)
    - `.github/workflows/ci.yml` (+83/−3)
    - `Cargo.lock` (+1)
    - `Cargo.toml` (+12)
    - `crates/viola-e2e/Cargo.toml` (+4)
    - `crates/viola-e2e/src/bin/viola-harness.rs` (+17/−1)
    - `crates/viola-e2e/src/harness/mod.rs` (+2)
    - `scripts/agent-run.ps1` (+8/−1)
    - `scripts/agent-run.sh` (+3/−2)
    - `src/bin/viola-fake-agent.rs` (+3)
    - `tests/cli_fake_agent.rs` (+14)
    - `tests/run_cli.rs` (+4/−1)
    - `tests/support/home.rs` (+9/−3)
  - New (line counts from `wc -l`):
    - `clippy.toml` (10)
    - `tests/contract_lints.rs` (105)
    - `scripts/lint-probes.sh` (164)
    - `scripts/install-ripgrep.sh` (73)
    - `crates/viola-e2e/src/harness/schema_check.rs` (271)
    - `crates/viola-e2e/src/harness/secret_scan.rs` (496)
    - `crates/viola-e2e/tests/scan_patterns.rs` (77)
- **Symbols / APIs:**
  - **Two new `viola-harness` subcommands**, `schema-check` and `secret-scan`. They are internal, beside `supervise`, not agent commands; `COMMANDS` goes from 6 to 8 (`crates/viola-e2e/src/bin/viola-harness.rs`). The five-command agent surface is unchanged. Both shims forward them (`scripts/agent-run.sh`: `supervise|ui-restart|gate|schema-check|secret-scan)`; `.ps1`: two new `switch` arms). Each prints one JSON document.
  - **`schema-check`** prints `{"v":1,"cmd":"schema-check","ok","files","lines","torn","skipped_non_file","failures":[{file,line,keyword}]}`. Its reasons are `empty-scope` and `schema-unreadable`.
  - **`secret-scan`** prints `{"v":1,"cmd":"secret-scan","ok","files","skipped_non_file","mode_check":"unix"|"skipped-windows","hits":[{file,line,offset,class}]}`, with `report:"unwritten"` when the hit file cannot be written. It writes `target/secret-scan/hits.json` only when there are hits, and removes a stale one on every run. Its reason is `empty-scope`.
  - **New pub library items in `viola_e2e::harness`:**
    - `schema_check::{DiagKind, DiagFiles, diag_files, relative, schema_check, check_tree}`
    - `secret_scan::{CRITICAL, CONTENT, Roots, secret_scan, scan}`
  - **Scan classes:**
    - `claude-stripped`: the literals `canary-token-value-7f3a`, `canary-socket-value-2b9d`, `canary-entrypoint-value-8e41`;
    - `token-query`: `?t=`;
    - `cookie`: `cookie:` (case-insensitive) and `viola_<digits>=`;
    - `gui-token`: the `t` value read from each home's `ui/*.url`;
    - `content-canary`: `canary-chain-value-5c1e`, which fails only in a home-level role file;
    - `mode`: Unix only, any diagnostics file whose mode is not `0600`;
    - `unreadable`.
  - **Root test fixture** (`tests/support/home.rs`): `Wrapper::wait_ready` now takes `&mut self` and panics `wrapper {name} exited before ready: {status}` when `child.try_wait()` reports an exit. `READY_WITHIN` goes from 20 s to 10 s. The sole caller is `Wrapper::boot` (code-graph `calls`, phase run `tree-query`), whose binding becomes `let mut wrapper`. `boot`'s own callers are unchanged.
  - **`tests/run_cli.rs` `run_self_exit_carries_duration_ms`:** the wait is exit-aware and 10 s (was 20 s).
  - **No env var, port, socket or product symbol was added.** The product crates `src/` and `crates/viola-core` gained no code (grep of `git diff HEAD -- src crates/viola-core`: only `src/bin/viola-fake-agent.rs` +3, a crate-level `#![allow]`).
- **Crates / modules:** `viola-e2e` gains modules `harness::schema_check` and `harness::secret_scan` and an integration test `tests/scan_patterns.rs`. Root gains test target `contract_lints` (`[[test]]`, `required-features = ["fake-agent"]`). No crate was added or removed.
- **Dependencies:** `viola-e2e` gains `jsonschema.workspace = true` (=0.57.0). The crate was already in the graph as a root dev-dependency, so `Cargo.lock` gains only the dependency-edge line `+ "jsonschema",` under `viola-e2e` (`git diff HEAD -- Cargo.lock`: 1 line), with no new package or version. `cargo deny check` is green.
- **Schema / config:**
  - **`[workspace.lints.clippy]`** (new): `print_stdout`, `print_stderr`, `dbg_macro` = `deny`.
  - **`clippy.toml`** (new): `disallowed-macros` for `tracing::{info,warn,error,debug,trace}`. `tracing::event` is deliberately **not** listed; see "Spec claims disproved".
  - **`crates/viola-e2e/Cargo.toml`** gains `[lints.clippy] dbg_macro = "deny"` and keeps no `[lints] workspace = true`.
  - **`src/bin/viola-fake-agent.rs`** gains the crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]`.
  - **`.config/nextest.toml`:**
    - `[profile.mutants]` `slow-timeout` goes from `{15s, ×2}` to `{5s, ×2}`, a 10 s kill.
    - New `[[profile.mutants.overrides]] filter = 'package(viola-e2e)'` with `slow-timeout = {15s, ×2}`, a 30 s kill for the harness's own tests.
  - The schemas `schemas/diag-line.v1.json` and `diag-detail.v1.json` are read by G4 and unchanged.
- **Spec-master edits:** none. Masters were read-only through phase and implement.
- **Counts / qualifiers moved:**
  - Harness subcommand count: 6 → 8. `COMMANDS` holds 5 agent + 3 internal (`supervise`, `schema-check`, `secret-scan`). test-plan §3 lists the internal set as "supervise, ui-restart, gate" (verification-harness rule `:23`).
  - Root-chain readiness bound: 20 s → 10 s. The harness `boot` bound stays 20 s.
  - Mutants nextest kill: 30 s → 10 s for the root package; 30 s kept for `viola-e2e`.
  - `ci.yml` steps:
    - `lint` job gains 8: fmt, clippy, install ripgrep, `rg --pcre2-version`, G1, G3, lint probes (Linux only), and the ripgrep PATH export inside the install step;
    - `test` job gains 10: jq presence, G2, G4, harness capture, secret scan, 4 uploads, and loses 1 upload (`agent-run-<os>`).
- **Dev-tool versions:**
  - **ripgrep (`rg`, the G1/G3 gate tool):** 15.2.0 official release (PCRE2 10.45) is installed by `scripts/install-ripgrep.sh` into `target/tools/ripgrep/bin`, on the dev host 2026-09-24 and in the CI `lint` job. The dev host previously had only the Claude binary's embedded ripgrep 14.1.1, reachable as a Bash-tool shell function; a clean gate shell has no `rg`. The asset sha256 values come from the release's `.sha256` assets.
  - **jq:** runner-provided and presence-checked in the CI `test` job, not installed. The dev host reads it via scoop.
  - **clippy / rustfmt:** from `rust-toolchain.toml` 1.98.1, unchanged.
- **Harness / gate surface:**
  - `lint-probes.sh` proves both ways, in throwaway crates carrying the repo's `clippy.toml` and the extracted `[workspace.lints.clippy]` table:
    - 4 bans fire (`print_stdout`, `print_stderr`, `dbg_macro`, `disallowed_macros` on `tracing::info!`);
    - 2 controls pass (a locally allowed print; the real `viola_core::obs_event!`);
    - a fail-closed raw-`event!` grep (`\bevent!\s*[({[]` in `*.rs` outside `crates/viola-core/src/obs.rs`) finds a planted hit, passes `obs_event!` and is clean over the tree.
  - `install-ripgrep.sh`: pinned, checksum-verified, idempotent, prints `tool-missing: <tool>` on a missing tool.
  - `ci.yml` `test` job follows the obs-plan §9 step order: G2 verbatim → G4 (`id: schema-conformance`) → harness capture (`if: failure()`) → secret scan (`id: secret-scan`, `if: always()`) → uploads:
    - `diag-<os>` and `junit-<os>`: `if: always() && steps.secret-scan.outcome == 'success'`;
    - `harness-<os>`: `if: failure() && … == 'success'`;
    - `secret-scan-<os>`: `if: always() && … == 'failure'`.

    All four use `actions/upload-artifact@043fb46d…` v7.0.1 with `retention-days: 7`.
  - `ci.yml` `lint` job (3 OSes): fmt, clippy `-D warnings --features fake-agent`, ripgrep install, `rg --pcre2-version`, G1 and G3 verbatim, and lint probes on Linux. No new Action; `permissions` are unchanged.
  - New registered-path candidates: `target/secret-scan/`, `target/tools/ripgrep/`, `target/lint-probes/run-<utc>-<pid>/` plus `target/lint-probes/target/`, `clippy.toml`, `scripts/lint-probes.sh`, `scripts/install-ripgrep.sh`. All `target/` paths are gitignored (`git check-ignore`).
- **Cross-project / external claims:**
  - **CI run `35995290314`** (sha `2834e4d42bb61fe610ed160f7d1a89fb4815cbdf`, job `mutants`, conclusion **failure**): 2 TIMEOUT at `src/main.rs:43:5` and `src/cmd/mod.rs:39:5`. Its uploaded `mutants` artifact logs show only 2 of 125 tests started, the two `Wrapper::wait_ready` consumers, SIGTERM'd at 19.82 s under cargo-mutants' 20 s auto timeout. This chunk owns that red.
  - **Sha `2969198b50542fc5cec0a3a242d18a2534bf9abc`:** all checks success (a docs-only diff).
  - **GitHub releases BurntSushi/ripgrep 15.2.0:** asset list and sha256 files.
  - **taiki-e/install-action `7623a79…`:** no `manifests/ripgrep.json` (HTTP 404).
- **Reverted / negative API facts:** `tracing::event` was first placed in `clippy.toml` and then removed. It fired `use of a disallowed macro tracing::event` at all 16 `obs_event!` call sites (`p1-clippy-2.log`).
- **Insufficient fixes (written, kept, not the remedy):** `obs_event!`'s inner `#[allow(clippy::disallowed_macros)]` (`crates/viola-core/src/obs.rs:155`) stays in place but is inert on clippy 1.98.1. The ban on raw `event!` is carried by the grep instead. The file is outside this chunk's modify-set.
- **Spec claims disproved by measurement:**
  1. **obs-plan §3 `logger-stack-install`** (`disallowed-macros` exemption: "`obs_event!` expands to a block whose inner `::tracing::event!` statement carries `#[allow(clippy::disallowed_macros)]`, so caller crates pass `-D warnings`", and "an `obs_event!` call in a caller crate lints clean, and a raw `tracing::info!` in the same crate fails") plus §3 `obs-ci-gate-wire` bullet 1 (ban `tracing::{event,info,warn,error,debug,trace}`).
     - **Measured false on clippy 1.98.1.** A throwaway two-crate workspace tried 8 placements of the allow: inner block, statement, closure, `#[expect]`, the outermost expanded node, the call-site statement, and the calling fn all still report 2 errors per call. Only a crate-level `#![allow]` in the caller crate silences it. In viola, 16 errors appeared at `obs_event!` sites. Evidence: `.andromeda/runs/2026-09-24T12-21-11-implement/clippy-disallowed-macros-measurement.md`.
     - **Operator decision** (overseer, founder-delegated, at /implement P1): ban the level macros by path in `clippy.toml`, and catch raw `event!` with a fail-closed grep in `lint-probes.sh` that is proven both ways. The half of the claim about raw `tracing::info!` failing holds, and is proven by `lint-probes.sh`.
  2. **Plan premise "the viola-e2e harness 20 s deadlines are outside the root mutants run".** It is false for any chunk that touches `viola-e2e`: cargo-mutants then tests that package. `boot_that_never_gets_ready_times_out_and_stops_its_supervisor` waits out the 20 s boot deadline by design, and the new 10 s kill cut it off. The unmutated baseline failed with `mutants-exit-4`. Resolved by the `package(viola-e2e)` override: with the harness in the diff, the baseline test time is 21 s and cargo-mutants auto-set its timeout to 110 s (`30.log`).
  3. **Plan probe `git diff --quiet HEAD -- Cargo.lock`** ("no dependency change: jsonschema is already locked"). Adding an already-locked crate to a new member adds a dependency-edge line, so the probe is red by construction. The substantive claim, no new package or version, holds.
- **Expected amendments (from plan):**
  1. **test-plan §3 Readiness / `run` step 2: root-chain readiness bound 20 s → 10 s, exit-aware rstest wait, `[profile.mutants]` 5 s × 2.**
     - Carried; facts under "Counts / qualifiers moved" and "Schema / config".
     - Sites (`grep -nF` in `test-plan.md`): `20 s` 4 hits (`:530` harness boot, keeps 20 s; `:542` `booted_wrapper` interim wait, changes; `:738` fixture timeout from boot deadlines, keeps; `:1483` boot deadlines not product budgets, keeps); `profile.mutants` 2 hits (`:729` changes to 5 s × 2 plus the `viola-e2e` override; `:1727` keeps).
     - Other masters: 0 hits of `bounded at 20 s` / `profile.mutants`.
  2. **test-plan §3 Harness implementation: internal subcommands `schema-check`, `secret-scan` beside `supervise`.**
     - Carried; fact under "Symbols / APIs".
     - Sites: `grep -n supervise test-plan.md` gives `:519` and `:612-613` (the supervise command block). No master lists the internal set verbatim (`supervise\`, \`ui-restart\``: 0 hits in all 7). The internal list lives in `.claude/rules/verification-harness.md:23` and the shim header comments, which are leaves, not masters.
  3. **test-plan §9 / §3 `ci-tool-install`: G2 uses the runner-provided `jq`, presence-checked; ripgrep 15.2.0 comes from `scripts/install-ripgrep.sh`.**
     - Carried; fact under "Dev-tool versions".
     - Sites: `jaq` 5 hits in test-plan (`:606`, `:682`, `:746`, `:758`, `:1412`) and 1 in obs-plan (`:669`, a null-encoding phrase, no change). `:758` and `:1412` change; the others describe `jq -e` or jaq as assertion forms and do not.
  4. **architecture §Occupied Resources → Repository: `target/secret-scan/`, `target/tools/ripgrep/`, `target/lint-probes/`, `clippy.toml`, `scripts/{lint-probes,install-ripgrep}.sh`.**
     - Carried; fact under "Harness / gate surface".
     - Sites (`grep -nF` in `architecture.md`): the `target/` registry at `:381-385` (4 path lines, including `target/deny-probes/` at `:384`, the shape to mirror) and the `scripts/` tree at `:451`. `target/secret-scan` also has 2 hits in obs-plan (already named there, §8 item 6 and §9).
  5. **obs-plan §3 `logger-stack-install` / `obs-ci-gate-wire`: not in the plan's list, added by /implement's surfaced gap.**
     - Fact under "Spec claims disproved" 1.
     - Sites: `disallowed-macros` 7 hits and `disallowed_macros` 1 hit, all in obs-plan (0 elsewhere).
- **Coverage of new surfaces:**
  - `viola-harness schema-check` → validation: jsonschema 0.57 against the committed schemas ✓ · instrumentation n/a (a test harness) · PII: line content never printed ✓ · tests: unit ×8 (`harness::schema_check::tests`) · a11y n/a · tokens n/a
  - `viola-harness secret-scan` → validation n/a · instrumentation n/a · PII: matched bytes never printed or written ✓ · tests: unit ×15 (`harness::secret_scan::tests`, 1 of them `#[cfg(unix)]`; `grep` of `#[test]` fns) + integ `scan_covers_every_canary_the_root_tests_plant` · a11y n/a · tokens n/a
  - `scripts/lint-probes.sh` → tests: self-proving (4 fired, 2 clean, grep both ways) · the rest n/a
  - `scripts/install-ripgrep.sh` → validation: sha256 pin ✓ · tests: gate run fresh plus idempotent re-run · the rest n/a
  - `ci.yml` new steps → tests: `zizmor .github/workflows/` clean; exercised only on the push · the rest n/a

## Deviations from intent
1. **The raw-tracing ban** has a different shape from plan step 2 and obs-plan §3: the level macros are banned by path, and raw `event!` is caught by a grep inside `lint-probes.sh`. Justification: clippy 1.98.1 cannot exempt `obs_event!`'s inner `event!` (disproved claim 1). The operator chose this at /implement P1 and asked for the grep to be fail-closed with its own positive and negative probe, which it is.
2. **The raw-`event!` grep runs in CI only on the Linux lint leg**, inside `lint-probes.sh`. Source text is OS-independent.
3. **`.config/nextest.toml` gains a `package(viola-e2e)` override** that keeps a 30 s kill for the harness's own tests. The plan named only the 10 s change. Justification: disproved claim 2. The root-package tests, where cargo-mutants' 20 s floor binds, keep the 10 s kill.
4. **`secret_scan.rs` was reshaped for mutation kill**, beyond the plan's wording:
   - the mode check was split into a pure `owner_only` / `mode_hit(…, Option<u32>)` plus a `#[cfg(unix)] file_mode` reader;
   - `find_all`'s redundant length guard was dropped (`windows()` yields nothing when the haystack is shorter);
   - `collect_files` descends into any non-file entry;
   - tests gained count, offset and unwritten-report asserts.

   Missed mutants went 13 → 1 → 3 across three full runs; the final 3 are all in `file_mode` (see Outcome).
5. **The clippy measurement was recorded in the implement run dir, not in `research.md`.** The operator asked for research, but /implement is not permitted to edit it. Wrap folds it (P2).
6. **`lint-probes.sh` and the CI ripgrep PATH step use `pwd -W`** for native paths, because Windows cargo misreads `/d/...` manifest paths.

## Decisions & corrections
- **Operator, P4 (phase):**
  - check bodies as internal harness subcommands;
  - G2 verbatim `jq` with a presence check and no install (ruling F2 covers jq only; the overseer confirmed at the P5 review that a pinned, checksum-verified rg install is right);
  - deadlines fixed by cause AND by value, not through nextest alone.
- **Operator, P5 review:** the unscanned `mutants.out/` upload (the `mutants` job) must not stay ownerless. Pin it as a CARRY on "Quality gates": scan it or stop uploading it.
- **Operator, /implement P1:** level-macro ban plus a fail-closed `event!` grep with its own probes, and the clippy measurement recorded for wrap.
- **Measured tool behaviour:**
  - clippy 1.98.1 `disallowed-macros` reports a macro expanded inside a foreign exported macro at the caller crate's level, and only a crate-level `#![allow]` suppresses it.
  - cargo-mutants 27.1.0 auto timeout is `max(20 s, ~5 × baseline test time)`: 20 s at a 1 s baseline, 110 s at a 21 s baseline. So a nextest kill must sit below the floor only for packages whose baseline is short.
- **Host hazards:**
  - `rg` in the Bash tool is a shell function that wraps the Claude binary; a clean `bash --noprofile --norc` has no `rg`.
  - A PreToolUse hook blocks the Write tool on any path containing `/target/`, including the scratchpad.
  - The permission layer denies `rm -rf` compounds; a single `mv` or a python script by path works.
  - Git Bash `pwd` gives `/d/...`, which Windows cargo reads as `D:\d\...`; `pwd -W` gives the native form.
- **Sweep hazard:** `\bevent!` does not match `obs_event!` (`_` is a word character), which is why the grep separates the sanctioned macro from raw `event!`.
- **Mutation hazard:** a `#[cfg(unix)]`-only function body yields mutants a Windows host can never compile or kill. The local `survived:0` atom is unreachable for them by construction, and only the CI ubuntu `mutants` job witnesses the kill.

## Outcome
**Acceptance criteria, against the diff:**
- **Clippy** `--workspace --all-targets --features fake-agent -D warnings`: green with the bans in force. `lint-probes.sh` prints `lint-probes: 4 bans fired, 2 controls clean`. **MET**, with the raw-`tracing` half re-shaped per deviation 1 (the operator-ratified mechanism).
- **`workspace_members_inherit_the_lint_table_except_the_harness_crate`**: PASS. **MET.**
- **`panic_hook_is_the_first_statement_of_main`**: PASS. **MET.**
- **G1 / G3** exit through `test $? -eq 1` green with ripgrep 15.2.0/PCRE2 locally. **MET locally**; the CI `lint` job on 3 OSes is owed on the push.
- **G2 / G4 / scan over kept homes:** G2 printed `true`; `schema-check` gave `ok:true`, files 26, lines 95, torn 0, skipped_non_file 2; `secret-scan` gave `ok:true`, files 34, hits 0, `mode_check: skipped-windows`. **MET.**
- **Step-15 unit tests:** pass. The filter ran 22 locally (8 `schema_check` + 14 `secret_scan`); the 15th `secret_scan` test is `#[cfg(unix)]`. **MET**, with the Unix test owed to CI.
- **`scan_covers_every_canary_the_root_tests_plant`**: PASS. **MET.**
- **`wrapper_boot_exiting_before_ready_fails_as_exited`**: PASS. No root-test deadline above 10 s (probe exit 1, no output), and the profile line is present. **MET.**
- **zizmor** clean; every added `uses:` is `upload-artifact@043fb46d…`; permissions unchanged. **MET.**
- **Uploads only behind a successful scan:** **MET by construction.** The CI run on the push exercises it.
- **`cargo deny check`** passes. **"`Cargo.lock` unchanged" is UNMET as worded**: the diff adds one edge line. The substance, no new package or version, is met. This is an unlinked criterion, routed to P2 as a plan-probe defect (disproved claim 3).
- **Pushed sha check-runs all `success`, `run --mutants` `survived:0` / `counted`, and the ubuntu PASS line**: **PENDING**, operator-gated (the push follows this wrap's commit). Local `run --mutants`: `survived:3` (below).
- **No CLI output byte changed:** product crates have zero print sites (grep over `src crates/viola-core`); the fake agent's `#![allow]` only. **MET.**

**Gates** (the tool's words; implement's final full-block run `implement-2026-09-24T12-21-11`, 33 entries: green 28 · red 2 · not run 3):
- green: `cargo build --workspace --features fake-agent`
- green: `cargo fmt --all --check`
- green: `cargo clippy --workspace --all-targets --features fake-agent -- -D warnings`
- green: `bash scripts/install-ripgrep.sh`
- green: `… rg --pcre2-version`
- green: G1 (`… rg -n -U --pcre2 … instrument …; test $? -eq 1`)
- green: G3 (`… rg -n --hidden … abort …; test $? -eq 1`)
- green: `bash scripts/lint-probes.sh`
- green: `… run --unit`
- green: `… run --integration`
- green: `… run --unit --filter 'test(/schema_check|secret_scan/)'`
- green: `… run --integration --filter 'test(/workspace_members_inherit…|…scan_covers_every_canary…/)'`
- green: the deadline census grep
- green: the profile grep
- green: `rm -rf target/e2e-home target/agent-run`
- green: `AGENT_RUN_KEEP_HOMES=1 … run --integration`
- green: G2
- green: `… schema-check`
- green: `… secret-scan`
- green: `cargo deny check`
- green: the sync-crates deny loop
- green: `zizmor .github/workflows/`
- **red** · `exit 0`: `git diff --quiet HEAD -- Cargo.lock` (exit 1). This chunk's, by construction (disproved claim 3); dispositioned at P2.
- green: the `set_var` grep
- green: `… cleanup --session gate-smoke`
- green: `… boot …`
- green: `… status …`
- green: `… logs …`
- green: `… cleanup …`
- **red** · `contains "survived":0`: `AGENT_RUN_CHUNK_BASE=2969198b… run --mutants` (78 tested: 69 caught, 6 unviable, 3 missed). All 3 are `crates/viola-e2e/src/harness/secret_scan.rs:224:5` (`replace file_mode -> Option<u32> with None | Some(0) | Some(1)`), the `#[cfg(unix)]` body, uncompiled on this Windows host. On Linux they are killed by `secret_scan_flags_a_diagnostics_file_that_is_not_owner_only` (None) and `secret_scan_passes_a_clean_tree` (Some(0), Some(1): a mode hit on a clean tree). The CI `mutants` job re-generates them (`--in-diff` over this chunk's diff) and is their witness.
- not run — leg operator: `git … && git push origin build/viola-0.1.0`; the `check-runs` read; the ubuntu PASS-line read. Owed after the commit.

**Smoke:** `impl-smoke` cleanup → boot → status `ready` → logs (2 diag lines) → cleanup (`processes_gone:true`, `home_removed:true`), all rc 0.

**Outcome basis:** implement's P4 report in this conversation, plus the operator directives quoted above.

**Process hygiene:** implement's census, re-measured at 13:06 UTC via `Get-Process`:
- The `gate-smoke` and `impl-smoke` sessions are terminated.
- `viola.exe` 12172 and 14064 are the operator's viola-lab prototype, from a different tree, and not this run's.
