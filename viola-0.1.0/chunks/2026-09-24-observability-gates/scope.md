# Scope — 2026-09-24-observability-gates

**Working entry (`working-route.md:21`, Epoch 1 — Foundation):** Observability gates: panic hook first, zero-panic, schema, bare-instrument and abort-panic gates, canary secret scan before any upload, print and raw-log lint bans.

This chunk is obs-plan §3 bootstrap phase `obs-ci-gate-wire`. It comes last in the obs route order, after the merged `pii-scrubbing-wire` / `logging-redaction-wire` phase that chunk `2026-09-24-log-redaction-and-never-log-floor` closed. It turns the obs contract into mechanical gates: in clippy, in `ci.yml` and in the local gate set. The gates are the subject. The product changes are only what the gates need in order to pass honestly.

## What it builds

1. **Panic hook first, witnessed as a gate.** The custom panic hook is the first statement of `main`, and a hook panic still exits 0 (CLAUDE.md invariant). At HEAD, `src/main.rs` `main` already opens with `std::panic::set_hook`. What this chunk adds is a gate that fails if it stops being first. (Verified at P3: `src/main.rs:43`. The check's form is P4's call.) `viola hook` does not exist at HEAD (`src/cmd/mod.rs:27`, `enum Command { Run }`), so "a hook panic exits 0" has no subject yet and stays with the hooks chunk.
2. **Print and raw-log lint bans** (obs §3 `obs-ci-gate-wire` bullets 1–2, §11 Logs):
   - a workspace `clippy.toml` with `disallowed-macros` covering raw `tracing::{event,info,warn,error,debug,trace}` outside `viola_core::obs`;
   - `[workspace.lints.clippy]` `print_stdout` / `print_stderr` / `dbg_macro` = `deny`;
   - `[lints] workspace = true` in every product member. `viola-e2e` is the one member without it and carries its own `[lints.clippy]` table.
   - Local `#[allow]` goes only on the output modules that own `--json`, human CLI stderr, the hook decision body and the `ui` launch line, where each exists at HEAD. [premise-corrected: none exists at HEAD. `run` is the only verb, and product code has zero print sites (grep of `print!|println!|eprint!|eprintln!|dbg!|io::stdout|io::stderr` over `src crates/viola-core`: 0 hits). So the "an output module's `#[allow]` passes" half is proven by a probe, not by an in-tree site.]
   - The mandated both-ways proofs: `obs_event!` lints clean while a raw `tracing::info!` fails, and a `println!` in a `hook`/`run` path fails while an output module's `#[allow]` passes.
   - A CI member-list assertion: every product member has `workspace = true`, and only `viola-e2e` lacks it.
   - [wrap intent amendment] Raw `tracing::event!` is banned by a fail-closed grep in `scripts/lint-probes.sh`, not by `clippy.toml`. On clippy 1.98.1 no allow inside `obs_event!` exempts its inner `event!`, as measured at /implement. The operator ratified this at /implement P1. See obs-plan D-33.
3. **CARRY folded from the entry (hypothesis, re-verify at P3):** the fake agent is a `[[bin]]` of the root package, and lints are set per package. So when the print bans land, exempt it with a crate-level `#![allow(clippy::print_stdout, clippy::print_stderr)]` in `src/bin/viola-fake-agent.rs` (obs-plan §3 `obs-ci-gate-wire` as amended 2026-09-24).
4. **CI gates in `.github/workflows/ci.yml`, per obs §9** (gate commands copied verbatim from §9):
   - the SHA-pinned PCRE2 ripgrep install step plus its `rg --pcre2-version` check;
   - G1 (bare `#[instrument]`) and G3 (`panic = "abort"` in any form);
   - G2 (zero `event:"panic"` over the home-level role files, non-empty scope first) and G4 (`id: schema-conformance`: role lines against `schemas/diag-line.v1.json`, detail lines against `schemas/diag-detail.v1.json`, reporting only file + line + keyword);
   - the secret scan (`id: secret-scan`, `if: always()`) over `target/e2e-home/**/diagnostics/*.ndjson`, `target/agent-run/*` and `target/nextest/ci/junit.xml`;
   - the scan-gated `diag-<os>` / `harness-<os>` / `junit-<os>` uploads (SHA-pinned upload-artifact v7.0.1, `retention-days: 7`) and the `secret-scan-<os>` hit-report upload, in the §9 step order;
   - [val-1 intent-incomplete, P5] The G4 and scan check bodies are internal `viola-harness` subcommands (`schema-check`, `secret-scan`; operator P4). Both `agent-run` shims must forward them, because the shims answer any other verb with `reason:"usage"`. Both skip non-regular files: tests create directories at role-file paths. Following obs-plan §10 ("a `diagnostics/` file that is not 0600") and test-plan §6, the scan also checks Unix `0600`. The Windows DACL half waits for the strict-modes chunk.
   - `AGENT_RUN_KEEP_HOMES=1`, so homes survive until the gates read them. [premise-corrected: already set at job level in `ci.yml:19-20`; kept, not added]
   - [premise-corrected: CI runs neither `cargo fmt` nor `cargo clippy` today. The `lint` job (`ci.yml:114-135`) runs only the Tokio-free check, so the lint bans need a clippy step on all three OSes to be enforced at all. The existing `agent-run-<os>` upload (`ci.yml:74-82`) runs under a bare `if: always()` with no scan before it, which obs §11 CI forbids, so it is replaced by the scan-gated `harness-<os>` upload.]
5. **CARRY folded from the entry (hypothesis, re-verify at P3):** obs G3 is written with `rg`, which is absent from the local gate shell's PATH. Chunk 1 ran it as `grep -rEn`. The local gate set needs an honest equivalent or the tool. (Verified at P3: `bash --noprofile --norc -c 'command -v rg'` finds nothing. The Bash tool's `rg` is a shell function around the Claude binary's embedded ripgrep 14.1.1, and the official release is 15.2.0.)
6. **Test-home placement** (verified at P3). G2/G4/the scan cover only homes under `target/e2e-home/`. Every `viola` process home at HEAD is already there:
   - root chain `TestHome` → `target/e2e-home/viola-test-*/home` (`tests/support/home.rs:37-47`);
   - harness `boot` → `viola-session-*/home` (`crates/viola-e2e/src/harness/boot.rs:124-128`).

   The `tempfile::tempdir()` uses in `src/` and `crates/viola-e2e/src/` are in-process unit tests that start no `viola` process. With `AGENT_RUN_KEEP_HOMES=1`, `TestHome` keeps its home (`:60-70`), so a green run leaves role files for G2's non-empty check.
7. **CI verdict fold: the mutants failure on the last wrap's commit.** Verified at P3 against the recorded run's `mutants` artifact; research.md §Measured facts:
   - Both mutants make `viola` exit 0 with no role line.
   - On the 2-thread ubuntu runner, nextest started only the two `Wrapper::wait_ready` consumers. `wait_ready` (`tests/support/home.rs:151-172`) never checks the child, so both spun to `READY_WITHIN = 20 s` and were SIGTERM'd at 19.82 s by cargo-mutants' 20 s auto timeout (1 s baseline).
   - `[profile.mutants]`'s 30 s nextest kill is also ≥ 20 s, so it can never pre-empt cargo-mutants.
   - Windows caught both mutants only because it scheduled `fake_agent_exit_no_eof_exits_while_stdout_is_held` alongside, and that test failed at `:558`.
   - Reachable deadlines: `tests/support/home.rs:16` (20 s), `tests/run_cli.rs:202` (20 s), `tests/support/fake.rs:12` (10 s), `tests/cli_fake_agent.rs:575` (5 s), and the nextest profile's 30 s.

   The coordinates as folded: Run `35995290314`, sha `2834e4d42bb61fe610ed160f7d1a89fb4815cbdf` (`2834e4d`, chunk `2026-09-24-log-redaction-and-never-log-floor`), job `mutants`, step `Mutation gate (chunk diff)`. Verbatim: `5 mutants tested in 2m: 3 caught, 2 timeouts`:
   - `TIMEOUT  src/main.rs:43:5: replace main -> ExitCode with Default::default() in 1s build + 20s test`
   - `TIMEOUT  src/cmd/mod.rs:39:5: replace dispatch -> Result<ExitCode, Failure> with Ok(Default::default()) in 1s build + 20s test`

   The same mutants were caught locally on Windows (5/5). Folded on the overseer's (founder-delegated) direction. The overseer's causal claim, kept verbatim as a hypothesis for P3 to re-verify against that run: "a test waits on something the mutant never produces until the 20 s per-test timeout." Required outcome, as directed: **fix the cause, not the timeout.** Every in-test deadline such a mutant can hit must sit well below the cargo-mutants per-test timeout (auto-set to 20 s in that run), so the mutant fails as caught.

   Witness on Linux: `--in-diff` will not re-mutate those lines, so touch `src/main.rs:43` / `src/cmd/mod.rs:39` only if a real change is needed. Otherwise prove the fix with the CI ubuntu test log plus a written argument, as chunk 5 proved its kill.

   This intersects the chunk's subject: CI gate correctness and the obs panic/exit surface (`main`, `dispatch`).

## Boundaries (not this chunk)
- No new redaction subjects. veil / skip-all / `ChannelError`, `drift_report` and the `cli` error line stay CARRYs on their own entries ("Wrapper channel", "Hooks to normalised events", "CLI output tokens").
- No perf/hyperfine gates. obs §9 puts them in the same per-OS job, but no perf subject exists at HEAD. (Verified at P3: no `hook` verb and no hyperfine in `ci.yml`.)
- No `viola-harness gate --require` step. The subcommand does not exist at HEAD (`crates/viola-e2e/src/bin/viola-harness.rs:22-65`), so the tests extract's "gate stays last" constraint has no subject yet.
- zizmor's pedantic-persona CARRY sits on `working-route.md:23`, not on this entry, and is not folded.
- No spec edits. A gap between §9's letter and HEAD surfaces as a val-1 divergence or a wrap amendment.

## Surfaces and contracts touched
- `Cargo.toml` (workspace lints, per-member `[lints]`), `crates/*/Cargo.toml`, new `clippy.toml`
- `src/main.rs`, `src/cmd/`, output modules (`#[allow]` sites), `src/bin/viola-fake-agent.rs`
- `.github/workflows/ci.yml` (test / lint / mutants jobs), `scripts/agent-run.{sh,ps1}` (the local gate set and the test timeouts)
- `schemas/diag-line.v1.json`, `schemas/diag-detail.v1.json` (read by G4, not changed)
- Contracts: obs-plan §3 `obs-ci-gate-wire`, §9 CI Integration, §10 (G2 as the panic SLO measure), §11 Logs/CI; security-plan NEVER-log floor (the secret-scan subject); test-plan §3 (`cleanup` step 6, `AGENT_RUN_KEEP_HOMES`).
