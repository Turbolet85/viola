# Scope — Fake agent and test-data fixtures

**Marker:** `2026-09-24-fake-agent-and-test-data-fixtures` · **Version:** viola-0.1.0 · **Epoch:** Epoch 1 — Foundation (second chunk)

**Working entry (verbatim):** Fake agent and test-data fixtures — scripted modes and control file, receipts,
temp-home fixture chain under target/e2e-home, fixture scrub-and-schema walk, committed property seeds

**Intent anchors:** test-plan §3 Bootstrap phases `test-data-bootstrap-wire` (the fake agent bin + receipt format +
scripted modes, the rstest fixture chain, the fixture scrub-and-schema walk, committed `proptest-regressions/`);
test-plan §7 (Fake agent binding contract behaviour · Fixture hygiene · Self-bootstrapping · Test data lifecycle);
`.claude/rules/verification-harness.md` §Fake agent.

**Annotations:** none stand on the entry (`route.py pins`: 5 freight blocks on the markerless tail, none on line 13).

**CI fold (operator-ratified, Setup 5a):** run `35973118026` on sha `dc01bd9cd426ab3659ad41a5656dff619dee0fd9`,
job `mutants` ("Mutation gate (chunk diff)") — completed `failure`; `test (ubuntu-latest)` success;
`test (windows-2025)` / `test (macos-latest)` were in progress at take-up and completed `success` at P3 (re-read via
`gh api …/commits/dc01bd9…/check-runs`). Folded as item 7 below by the operator's word ("Fold into this chunk"),
with the operator's constraint recorded verbatim there.

_P3 premise closure applied 2026-09-24: of 9 `[inferred]` bullets, 8 verified (tag dropped) and 1 corrected (the
Boundaries boot-wiring bullet); 2 further `[premise-corrected]` bullets fix an untagged statement (item 4's chain
location) and add a finding (item 7's stale-outcomes face); evidence in `research.md`._

## What this chunk builds

1. **Fake agent — scripted turns and the control file.** `src/bin/viola-fake-agent.rs` (root-package `[[bin]]`,
   `required-features = ["fake-agent"]`) grows from the `--version`/Ctrl-C slice to the §7 contract: a JSON turn
   script (`--script <path>`, resolved relative to the workspace root like `--fixtures`) whose gated steps each wait
   for the next line appended to `--control <path>`, read by byte offset (deterministic ordering, no sleeps).
   Checked-in scenario scripts live in `fixtures/fake-scripts/<scenario>.json`, synthetic text only.
2. **Fake agent — receipts.** `--receipt <path>` receives one line per observation: received prompts (text, raw hex,
   `bare_esc`), keystrokes, env names, Unix fds, and hook invocations (exit code, stderr length, stdout bytes). The
   receipt line format is this chunk's to fix (a harness-owned test format, not a product format).
   - Receipt lines are ndjson carrying `v` (project rule: every viola format carries `v`; readers skip unknown
     kinds/fields; architecture §Conventions "Protocol versioning").
3. **Fake agent — modes.** Bracketed paste + Enter accepted as one prompt; `--suppress-prompt-submit`,
   `--local-command-mode`, `--inject-harness-turn`, `--exit-no-eof`, `--vt100-panic-bytes`, `--report-version <ver>`
   (changes only the `--version` answer), `statusline-echo`, `agents --json` with
   `FAKE_CLAUDE_AGENTS_MODE=recorded|oversize|malformed`. Exits on `\x03`.
   - Hook invocation reads the hook commands from the `plugin/` + `settings.json` files `run` wrote, never PATH — but
     no product surface writes those files at HEAD (the plugin folder is Epoch 2 "Instance state and start order";
     hooks are Epoch 2 "Hooks to normalised events"). This chunk builds the reader and the invocation path against
     test-written plugin/settings inputs; the end-to-end hook replay lands with those chunks.
   - Fixture replay of `fixtures/claude/<cli-version>/` has no recorded fixtures to replay at HEAD (`viola verify`
     recording is Epoch 2 "Capability ledger and viola verify"; the drift contract is Epoch 2 "Fake-agent drift
     contract"). This chunk builds the `--fixtures`/`--cli-version` loading seam and exercises it on synthetic test
     inputs only.
4. **Temp-home fixture chain.** rstest `#[fixture]` chain `home → fake_agent_path → stamped_home → booted_wrapper`:
   a not-yet-existing `home` path inside a `tempfile` dir under `<workspace>/target/e2e-home/`; `TempDir::keep()`
   when `AGENT_RUN_KEEP_FAILED=1` on a failing test and always under `AGENT_RUN_KEEP_HOMES=1`; env per child via
   `Command::env`, never `std::env::set_var`.
   - [premise-corrected: test-plan §3 `run` step 2 (test-plan.md:542) — "the root package must not dev-depend on the
     tokio-based `viola-e2e`, so the chain exists twice (the sync root copy and the `viola-e2e` copy) against one
     contract"] The chain is built TWICE against one contract: a sync root copy in `tests/support/` (root tests find
     the fake agent via `CARGO_BIN_EXE_viola-fake-agent`) and a copy in `viola_e2e::fixtures` (fake agent from the
     test executable's own target dir, never `CARGO_BIN_EXE_*`). The root package does NOT dev-depend on `viola-e2e`.
     (The entry's original wording — "in `viola_e2e::fixtures`, used from root `tests/support/`" — is withdrawn.)
   - `stamped_home` cannot stamp through `viola verify` at HEAD (verb absent; "no test or harness code writes
     `ledger/stamps.json`"). The fixture is built with its seam in place and an interim, documented no-stamp
     behaviour; the real stamp arrives with the verify chunk.
   - The existing ad-hoc `scratch()` helper in `tests/run_cli.rs` (6 call sites) migrates onto the root chain.
5. **Fixture scrub-and-schema walk.** One rstest `#[files("fixtures/claude/*/*.json")]` walk failing on absolute
   paths (`^[A-Za-z]:[\\/]`, `/home/`, `/Users/`, `\\Users\\`) and non-placeholder usernames, and validating each file
   with jsonschema.
   - No `fixtures/claude/` file exists at HEAD, so a walk over that glob alone would pass vacuously; the walk must be
     proven non-vacuous (covering `fixtures/fake-scripts/*.json` too, and/or asserting a nonzero match count per
     glob) and its rejection arms proven on planted bad inputs.
6. **Committed property seeds.** proptest strategies with committed `proptest-regressions/` for the randomized
   surfaces that exist at HEAD.
   - At HEAD the only §7-listed surface present is `ViolaName` (`crates/viola-core/src/lib.rs:13`); the other
     listed strategies (`Percent`, event lines, `Last-Event-ID` lists, `rate_limits` JSON) belong to the chunks that
     create those types.
7. **Mutation gate: no vacuous pass on a Rust-free diff (CI fold).** `crates/viola-e2e/src/harness/run.rs::mutants`
   reads a non-empty diff whose files include no `.rs` source (cargo-mutants: `INFO Diff changes no Rust source
   files`, no `outcomes.json`) as `failures:["outcomes-missing"]` — red on every docs-only push, e.g. each wrap's
   witness commit. Mechanism verified against run 35973118026's `mutants` log at sha `dc01bd9` (still `failure`
   there) and reproduced on this host: cargo-mutants 27.1.0 on a 14-line Rust-free diff exits 0 and does not touch
   `mutants.out/`. The `Err(_) if diff.trim().is_empty()` arm at `run.rs:409` covers only the empty diff.
   - [premise-corrected: local reproduction — `mutants.out/outcomes.json` survived the Rust-free run with its
     09:57 +0200 mtime from an earlier run] The defect has a second face: where a prior `mutants.out/` exists (every
     dev host after one run), the harness reads a STALE `outcomes.json` and reports the earlier run's counts — a
     false verdict, not `outcomes-missing`. The fix must never read an `outcomes.json` the current invocation did not
     produce.
   **Operator constraint (verbatim):** "Keep the "no vacuous pass" rule: a diff with no .rs files passes only with an
   explicit no-rust-delta verdict (the diff is checked and named in the output); a diff WITH .rs files and no outcomes
   stays red. Unit-test both."

## P4 operator decisions (val-1: intent-incomplete, amended 2026-09-24)

The operator's P4 word, founder-delegated overseer: "consumer-first, no shapes invented before a recorded fixture;
each deferral pinned as a CARRY on its consumer". It narrows items 3-5 above:
- Item 3: `--vt100-panic-bytes`, `statusline-echo` (with the `settings.json` read) and the `agents --json` modes
  (`FAKE_CLAUDE_AGENTS_MODE`) are deferred to their consumer chunks. Hook `matcher` evaluation is deferred to the
  drift contract. Every other listed mode is built.
- Item 4: only the sync root copy of the chain is built. The `viola_e2e::fixtures` copy lands with its first E2E consumer.
- Item 5: the walk runs over `fixtures/fake-scripts/*.json`. The `fixtures/claude/*/*.json` `#[files]` line and the
  claude-fixture schema land with the first recorded fixture ("Capability ledger and viola verify").
- Item 6: persistence is configured explicitly, and `crates/viola-core/proptest-regressions/lib.txt` is committed
  with proptest's header and no invented seeds. Seeded replay as a gate stays with "Quality gates".

## Boundaries (not this chunk)

- Recording real `viola verify` fixtures, the capability ledger and stamps (Epoch 2 "Capability ledger and viola verify").
- The fake-agent ↔ recorded-fixture drift contract suite (Epoch 2 "Fake-agent drift contract").
- The product `plugin/` folder, `settings.json` writes and hooks (Epoch 2).
- The print-ban lint exemption for the fake agent (CARRY on "Observability gates").
- [premise-corrected: `crates/viola-e2e/src/harness/boot.rs:31-37`] Harness `boot` wiring of `--instance <name>:<fake-agent-args>` already exists (`boot.rs:31-37` parses
  `fake_args`); this chunk adds no boot grammar beyond what the fixture chain needs.
