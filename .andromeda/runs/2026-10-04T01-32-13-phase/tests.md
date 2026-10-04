# tests extract

## Relevance
relevant — the chunk is test infrastructure end to end: a mutation-scoring workflow, the grading of 34 owed mutants on the runner that can compile them (Comprehensive tier, per test-plan §1), and a test-temp-dir leak in the mutation run's nextest profile.

## Constraints
- **Cadence, not a gate.** test-plan §2 (Mutation row) and §10 (Mutation gate) require mutation to run only at the epoch boundary and on demand, never as a chunk, pre-push or CI gate. §9's Mutation row ("none in CI since 2026-09-28") and §3 `gate` ("No CI job gates `mutants`") must still hold once the dispatch-only workflow exists. The workflow is new text that §9 has to absorb at wrap: a Mutation-row amendment plus the dispatch recipe. It must not change `ci.yml`'s `--require` lists.
- **Drive the harness's own verb.** The workflow must call `run --mutants --package <member>` [`--file <path>`]…, the boundary tier's whole-member form, and must not hand-roll a second cargo-mutants argv (per test-plan §3 `run` step 4 Package, Command). The tested scope stays cargo-mutants' default, per package (§3 `run` step 4 Test scope), so each mutant has to be run under the package that owns it:
  - `viola-e2e` for the 2 `scratch.rs` coordinates;
  - `viola-pty` for 10;
  - `viola-channel` for 13;
  - the root `viola` for the 9 `src/panic_frames.rs` coordinates.
  The Package arm refuses (`package-refused`, exit 2) any `crates/<member>` whose `Cargo.toml` `[features]` lacks `fake-agent`. Whether `viola-pty` and `viola-channel` already declare it is research's question.
- **Windows host-scratch arm.** On a Windows host, §3 `run` step 4 Command requires the run to:
  - resolve `<repo parent>/viola-mutants-scratch`, refusing with `scratch-refused` / `scratch-wipe-failed` and never falling back to `%TEMP%`;
  - set `TMP`/`TEMP` to the scratch and pass `--output <scratch>`;
  - report `scratch_bytes` in the `mutants` object.
  Whether that arm has ever run on `windows-2025`, and whether the runner's checkout depth keeps the copied paths under the path limit, is research's question.
- **Verdict from counts.** A run's verdict comes from its own `outcomes.json` counts: `missed == 0 && timeout == 0 && unviable <= caught`. Only exit codes 0, 2 and 3 can pass. Exits 1, 4, 5, 6 and 70 are `mutants-exit-<code>` (per test-plan §3 `run` step 4 Verdict; §10 Mutation gate).
- **Grading the 34.** A mutant this host cannot compile or reach is recorded by coordinate as "not measured here; owed to {route entry}", never as "equivalent" and never counted as caught. Each of the 34 is graded only by the windows run. `src/cmd/run.rs:318:5` (non-x86_64 Windows) stays not measured (per test-plan §10 Mutation gate; §11 Test Strategy "NEVER treat a Linux-only local run as proof for Windows").
- **Leak fix: cleanup discipline.** test-plan §3 Test data bootstrap Cleanup sets the rules the fix must keep:
  - Root `TestHome` keeps its `owner.json` record. Any sweep goes through `remove_owned`, never by age or name, and never takes a dir without a record.
  - viola-e2e's `Booted` guard runs `cleanup(.., keep_homes = true)` on drop.
  - A run killed under nextest `terminate = "immediate"` never drops its home.
  - `run --mutants` forces `AGENT_RUN_KEEP_HOMES=0` / `AGENT_RUN_KEEP_FAILED=0`.
  Whether the nested `cargo-mutants-ws-*` copies and the `TempDir` leftovers sit outside these mechanisms is research's question.
- **Leak fix: timing bounds.** These bounds must hold in this order after the fix: the root waits' 7 s `WITHIN`, then the nextest `mutants` profile's 10 s kill, then cargo-mutants' 20 s auto-timeout floor. `[profile.ci]`'s 120 s kill and `retries = 0` stay unchanged (per test-plan §3 `run` step 2; §10 Mutation gate; §10 Zero-flakiness budget).

## Patterns to follow
- **The run document as the result.** Read the run document's top-level `mutants` object (`{"tested":N,"verdict":"package","package":"<member>"[,"files"][,"scratch_bytes"]}`) and `suites[].survived`. Never read a raw `mutants.out` listing (per test-plan §3 `run` Output format).
- **Workflow shape.** Follow `nightly.yml`'s pattern of `schedule` + `workflow_dispatch` jobs, minus the schedule:
  - `permissions: {}` at the top and `contents: read` per job;
  - every `uses:` SHA-pinned;
  - tools from taiki-e/install-action v2.87.19 at the §9 pins (`cargo-nextest@0.9.146`, `cargo-mutants@27.1.0`);
  - `rustup toolchain install` rather than a toolchain action.
  Whether the pin can stay single-sourced (pre-push parses it from ci.yml's `test`-job `tool:` line) is a P4 question (per test-plan §9 paragraph after the stage table).
- **Disposal form for missed mutants.** A missed mutant is killed by a new or strengthened test in the owning crate, named `<subject>_<condition>_<expected>` and kept inline in `#[cfg(test)] mod tests` or in the crate's `tests/`. Otherwise it is recorded as equivalent, with its argument (per test-plan §2 naming conventions; §3 `run` step 4 Test scope).
- **Leak fix model.** Model the fix on the existing owner-recorded cleanup: `owner.json` `{pid, started_at}`, then `remove_owned`, with the record deleted last. Do not bolt on a post-run sweep (per test-plan §3 Test data bootstrap Cleanup).

## Anti-patterns to avoid
- Trusting cargo-mutants' exit code alone, because exit 3 masks exit 2. Also banned: counting a mutant as caught before the windows run measured it, and reshaping `cfg(windows)` code so a Linux test can reach a mutant (per test-plan §11 Quality; §10 Mutation gate).
- Uploading `mutants.out/` raw. Its `outcomes.json` holds absolute argv paths and its `log/` holds test output (per test-plan §9 test report format). Any artifact the workflow uploads must be a scan-gated, path-free summary.
- Adding any cache: rust-cache is in `ci.yml` only. Also banned: referencing an Action by a mutable tag, retries, `#[ignore]`, and `sleep`-based synchronisation in a new kill test (per test-plan §11 CI; §10 Zero-flakiness budget).

## Contract bindings
- **tests ↔ obs.** Every artifact that leaves CI does so only behind obs-plan §9's secret scan. If the workflow keeps a result artifact, a `secret-scan` step precedes the upload, and the upload is gated `steps.secret-scan.outcome == 'success'` (per test-plan §9 test report format; §3 `secret-scan` scope).
- **tests ↔ security.** zizmor over `.github/workflows/` in the CI `supply-chain` job must cover the new file. Every rule below must hold for it, with no zizmor finding silenced (per test-plan §9 Supply-chain row and paragraph after the table; security.md §Dependencies and CI):
  - full-SHA pins;
  - `permissions: {}` at the top and `contents: read` per job;
  - no secret on the runner.
- **tests ↔ arch.** The new workflow file and the scratch dir on the runner are directory-tree and Occupied Resources entries (arch §CI/CD; §Infrastructure Patterns).
- **tests ↔ Decisions Log.** Any new `run` `reason`, `mutants.verdict` or harness flag needs a test-plan Decisions Log entry (per test-plan §3 Closed enums, "A new value needs a Decisions Log entry").

## Acceptance criteria contributions
- **The 34 graded on the runner.** The dispatched `windows-2025` workflow's `run --mutants --package <member>` documents for `viola-e2e`, `viola-pty`, `viola-channel` and `viola` grade every one of the 34 coordinates, each as one of:
  - caught;
  - unviable, with its build-log reason;
  - missed and then killed by a new test;
  - equivalent, with its argument.

  Each verdict reads `missed == 0`, `timeout == 0` and `unviable <= caught` from `outcomes.json` counts, not from the exit code. `run.rs:318:5` stays recorded as not measured (per test-plan §10 Mutation gate; §11 Quality).
- **Workflow file.** The file declares only `workflow_dispatch`, `permissions: {}` and per-job `contents: read`, with every `uses:` SHA-pinned and no cache step. The `supply-chain` job's `zizmor --format=json .github/workflows/` reads no finding on it. `ci.yml`'s `gate --require` lists are unchanged (per test-plan §9; §11 CI; §3 `gate`).
- **Leak.** A before/after count of the mutation run's `TMPDIR` across one boundary `run --mutants --package <member>` run reads 0 leftover test temp dirs and 0 nested `cargo-mutants-ws-*` copies. The mechanism must be fixed inside the owner-record and `Booted` cleanup contract, not by a sweep keyed on age or name (per test-plan §3 Test data bootstrap Cleanup).
- **New kill tests in CI.** Any new kill test runs and passes in the `test (windows-2025)` leg's `run --coverage` and in `gate --require coverage,doctest,playwright`, with no retry, `#[ignore]` or skip. Coverage stays at or above 85 / 95 / 80 per OS (per test-plan §10 coverage thresholds and Zero-flakiness budget; §11 Test Strategy).
