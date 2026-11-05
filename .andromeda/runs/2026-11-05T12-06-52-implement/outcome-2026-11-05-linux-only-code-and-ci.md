# Implement outcome — 2026-11-05-linux-only-code-and-ci

**Outcome: green + smoke.** Next: the OPERATOR PASS (entries 38–49, `leg = 'operator'`: the tag entries 38 and 39
first, then the hygiene entry 40, the land and the pre-CI commit, the push 41, the parent check 42, the CI reads
43–49), then `/andromeda-wrap-session`.

## Gates
`python -X utf8 {tools_dir}/gate.py run --chunk 2026-11-05-linux-only-code-and-ci --run-dir .andromeda/runs/2026-11-05T12-06-52-implement`,
one firing, no fix iteration:

`entries 49 · green 35 · red 0 · recorded 2 · timeout 0 · not-run 12`

- 1 `df -h /tmp` — recorded, exit 0: `tmpfs 32G 6.1G 26G 20% /tmp`.
- 8 the carry's pattern over the tracked `.rs` files — green, exit 1, no output (274 lines in 64 files at take-up).
- 9 the workflows' Windows/macOS pattern, 10 the manifests' `windows-sys` / `cfg(windows)` pattern — green, no output.
- 11 the named Windows-only paths gone — green.
- 12 the `e2e_home` cases — green, `9 tests run: 9 passed`, the injected-figure cases and
  `e2e_home_backing_below_the_floor_is_refused_and_left_as_found` among them.
- 13 / 14 / 15 the msrv job's steps on `RUSTUP_TOOLCHAIN=1.96` — green.
- 16 `cargo deny check bans licenses sources` — green.
- 17 `cargo deny check advisories` — recorded, exit 1: `advisories FAILED` on one error alone,
  `error[unmaintained]: pin-utils is unmaintained` (RUSTSEC-2026-0441, `pin-utils 0.1.0`), the CARRY on «Version
  done-check» (`inputs#I3`); no other advisory.
- 21 `deny-probes: 14/14 banned, control clean` (14, as predicted).
- 22 zizmor: `No findings to report. Good job! (2 suppressed)` over `ci.yml` and `nightly.yml` (3 → 2).
- 23 `orphans-check probes: 2/2 fired, control clean`; 24 `orphans-check: 11/11 targets clean` (one triple).
- 27 `run --unit`, 28 the census/contract `--e2e` filter, 29–30 `run --perf` + its gate — green.
- 31–36 the smoke pairing, G2 and G4 — green.
- 37 `pre-push` — green: `{"ok":true,"stage":"linux-tests"}`, coverage `3383 passed, 0 failed, 0 skipped`,
  `gate {"ok":true,"breaches":[]}`.
- 38–49 — `not run — leg operator (the letter drives it)`.

No deferral: every language changed, so no source-delta deferral was open, and no entry carried `defer`.

## Smoke
P3 fired (boot path changed: `src/main.rs`, `src/cmd/run.rs`, `viola-pty`). A by-hand `agent-run.sh cleanup` →
`boot --session p3-smoke --unstamped --instance builder` → `status` → `cleanup` read `ok:true`, `state:"ready"`,
`builder` live; its `process-start` lines read `os` `linux`, `endpoint_kind` `unix-socket`, `pty_backend`
`openpty`, no `sideload_fallback`; cleanup `processes_gone:true, endpoint_gone:true, home_removed:true`.

## Files changed
`gate.py scope`: `scope: clean — changed 113 · listed 105 · recorded 8 (companion 7 · mechanical 1 · in-intent 0 ·
widening 0) · absorbed 2 · excluded 868`. `git diff HEAD --stat`: 116 files, +738 / −7856.

Deleted (beside research's named deletions): `crates/viola-e2e/src/harness/run/mutants/scratch.rs` (listed as
modify; see deviations).

Scope record (`doc:2026-11-05-linux-only-code-and-ci/scope-record`, rev 4):
- `crates/viola-agent-claude/src/ledger/test_cases.rs` · companion · serves crates/viola-agent-claude/src/ledger.rs · self
- `crates/viola-e2e/src/bin/viola-mcp-tap.rs` · companion · serves crates/viola-agent-claude/src/lib.rs · self
- `src/cmd/hook/statusline.rs` · companion · serves crates/viola-agent-claude/src/statusline.rs · self
- `crates/viola-e2e/src/harness/schema_check.rs` · companion · serves schemas/diag-line.v1.json · self
- `crates/viola-e2e/Cargo.toml` · companion · serves step 2 · self
- `fuzz/Cargo.lock` · mechanical · serves crates/viola-channel/Cargo.toml · self
- `crates/viola-e2e/tests/path6_budget_harness.rs` · companion · serves crates/viola-agent-claude/src/statusline.rs · self
- `tests/path1_run_start.rs` · companion · serves crates/viola-agent-claude/src/statusline.rs · self

Research-listed but unchanged: `tests/contract_bounded_readers.rs` — no `BOUNDARIES` / `EXCEPTIONS` needle stood on a
line the removal edited; its stands-check passed (108/108 with the error-body, diag-schema and exit-cause contracts).

## Deviations from the plan, with their justification
1. **`AgentError::BatchScriptChild`'s second producer.** Step 6 named `is_script` as the resolver's; it also fed
   `ProbeRun.program_is_script`, the `shim-resolution` row's input. The field left with `is_script`, and the row
   reads `run.version_answered` alone. On Linux a `.cmd` name is an ordinary executable, so the row's Linux verdict
   is unchanged for every program the resolver returns; the `shim-resolution` row's master wording is the wrap's.
2. **The ledger scrub's case fold.** Step 6 removed `CASE_INSENSITIVE` for the R8 strip; `ledger::scrub` took it as a
   parameter too (verify's `--record` and the test-only `viola-mcp-tap`). The parameter left and the scrub matches
   the home exactly, as it did on Linux; `scrub_folds_the_home_case_only_when_asked` became
   `scrub_matches_the_home_case_exactly` (its Linux half).
3. **`statusline::shell_argv` returns `[&str; 3]`**, no longer `Option`: with `WRAPPED_HERE` folded it was always
   `Some`. Its one product caller (`src/cmd/hook/statusline.rs`) maps instead of `and_then`.
4. **`scratch.rs` deleted, not edited.** Step 10 removed `HOST_SCRATCH` and the Windows wipe; every item left in the
   module served only them (`prepare` returned `None` off Windows), so the module left whole, with `mutants.rs`'s
   `scratch_bytes` field, its `TMP`/`TEMP`/`--output` arm and the tests' `scratch_or_root` / `wiped_stale`. The
   harness document's `scratch_bytes` key (absent on Linux) is gone.
5. **viola-pty's console helpers.** Beside the plan's named arms, `raw_input_mode`, `vt_output_mode`,
   `size_from_window`, `ConsoleInput`, `Utf16Source` and their unit cases left (the `ReadConsoleW` reader's and the
   Windows `HostTerminal`'s), and `PtyError::Load`, produced only by the sideload; `nonzero_size` stays.
6. **`replace_private` is one attempt.** With the retry gone, `replace_private_with` and `write_snapshot_with` (whose
   only role was the pause between attempts) left too; `write_snapshot` calls `replace_private`.
7. **`Refused::NullDacl` / `NoPersistentAcls` and `PinError::StrictModes`** left with their only producers
   (`windows_verdict`, `pin_companions`); the fixed-message cases dropped those rows. `outside_profile` and the
   Windows `win` modules of `fs.rs` / `strict.rs` / `test_support.rs` left whole.
8. **`human/style.rs`** lost the `vt` fact with the VT enable: rungs 1–6 and the depth rule are unchanged; the
   `vt_enable_failed` row and the Windows depth rows left the tables (`colorterm_on_windows` became
   `colorterm_without_term`, the same reading without the VT fact).
9. **The census's new fault class** (`another OS: …`) caught a multi-line
   `#[cfg_attr(not(target_os = "macos"), case::…)]` pair in `tests/hook_fail_open.rs` that the carry's line grep
   (entry 8) cannot see; both cases now run unconditionally. Its planted Windows cases build the word from a
   `WINDOWS` const so the file itself holds no line the carry's pattern reads.
10. **`host.rs`'s fixtures** were re-planted on `not(unix)` (the `windows` role: false on Linux, true on the
    non-unix test host) and `target_os = "freebsd"` (the macOS role, with a `FREEBSD` test host in `MACOS`'s place);
    every verdict keeps its shape. The `windows` / `WINDOWS` test-host data stay (reader data, not gates).
    `mutants.rs`'s planted `any(unix, windows)` became `any(unix, target_os = "none")`.
11. **Test gates made unconditional beyond the plan's named ones**, each because its only sibling left:
    `run_refuses_a_stale_name`, `cross_list_rows_dim_a_stale_row_end_to_end_on_a_terminal`,
    `a_start_under_umask_000_makes_every_home_path_owner_only`, the hook-fail-open impostor cases, the handlers'
    impostor case, `security_negatives_server_identity.rs`; their `GATES` rows left. Every other `cfg(unix)` gate
    stays (218 lines in 64 files by `cfg!?\(.*(unix|target_os = "linux")`, 327 in 78 at take-up).
12. **Comments naming removed code** were reworded where they named it (`tests/support/piped.rs`,
    `tests/hook_statusline.rs`, `tests/cli_version_gate.rs`, `tests/tui_passthrough.rs`, `tests/tui_wheel.rs`);
    ConPTY mentions that record history (`.config/nextest.toml`, `Cargo.toml`'s portable-pty pin note, pump
    comments) stay.
13. **`scripts/lint-probes.sh`'s `native_root`** is `$root` (the `pwd -W` fallback was MSYS's);
    `scripts/install-ripgrep.sh` also lost its `.zip` handling and the Schannel `--ssl-revoke-best-effort` branch,
    both Windows-only.

Counts re-measured by their rules: `process-exit` `detail` enum 20 → 19 (`schemas/diag-line.v1.json`); revive's
refusal codes lose `batch-script-child` (11 → 10); verify's refusal codes 6 → 5 (`src/cmd/verify.rs`
`human::refuse(` 6 → 5); `human::refuse(` in `src/cmd/run.rs` 8 → 7; the `thiserror::Error` census 10 at HEAD and 10
now (`command grep -rn "thiserror::Error" src crates --include=*.rs`; the plan's "9" was a predicted figure);
the nextest `ci` kill line's `binary(…)` names 27 → 26; root `Instant::now() + WITHIN` waits 32 → 31.

## Surfaced
Nothing: no spec↔reality gap the impl could not meet. The masters' amendments are the wrap's, as the plan lists
them; deviations 1, 2, 4, 7 and 8 touch master wording the wrap re-derives (the shim-resolution row, the scrub, the
harness document's `scratch_bytes`, the closed `Refused` messages, the colour rung 7).

## Capabilities
None claimed (`coverage where chunk = '2026-11-05-linux-only-code-and-ci'`: 0 rows); no matrix write.

## Process hygiene
`gate.py census --comm viola --comm viola-fake-agent --comm viola-harness --comm claude --args projects/viola/target`:
`census: 14 running`, none of this repository's build (no `projects/viola/target` path).

| process | started by | final state |
|---|---|---|
| harness sessions `p-linux-smoke` (entries 31–36) and `p3-smoke` (P3) | this run | terminated — each `cleanup` read `processes_gone:true` |
| test-binary children of entries 12–37 | this run (a leg of the gate tool) | terminated — absent from the census |
| `claude` × 4 overseers, `viola-lab/prototype` wrappers × 4 with their `claude` children, `~/.viola/…/viola mcp`, `~/.local/viola-product/bin/viola wait` | NOT DERIVED — other trees' processes | left running: not this run's — the founder's |

## Decisions & corrections
- Operator directive at invocation: live sessions 0, the fake agent only — held (no entry drove the real `claude`).
- Operator directive: a tool fault stops the run with nothing patched — none met.
- Operator directive: health check 13 reading a missing `agent-run.ps1` is known upstream (F411); no setup Phase 4 for it.
- Operator directive: the operator pass runs its tag entries first, as the plan orders, with the `ci.py` conclusion reads.
- Guard refusals: `bash-guard.sh` refused one `cat` heredoc to a file and one `cd` outside the root; each re-issued
  once in the guard's form (a scratchpad script run by path; absolute paths).
