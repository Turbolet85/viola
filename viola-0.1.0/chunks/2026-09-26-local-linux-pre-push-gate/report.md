# Report — 2026-09-26-local-linux-pre-push-gate

**Chunk:** WSL2 Ubuntu pre-push gate: CI-pinned toolchain, history-carrying synced clone, Unix tests + ubuntu mutation leg before the push
**Date:** 2026-09-27T01:00:00Z
**Commits:** `054ebe4 chore(2026-09-26-local-linux-pre-push-gate): operator pre-CI commit, for the run this chunk's verdict reads` (the only commit since last_wrap 2026-09-26T21:14:00Z; `git log --format='%h %s' a69c5ef..HEAD`)

## Changes (structured — detectors read this)
- **Files:** `crates/viola-e2e/src/harness/pre_push.rs` (new) · `scripts/wsl-provision.sh` (new) ·
  `crates/viola-e2e/src/harness/run/mutants.rs` · `crates/viola-e2e/src/harness/mod.rs` ·
  `crates/viola-e2e/src/bin/viola-harness.rs` · `scripts/agent-run.sh` · `scripts/agent-run.ps1` ·
  `crates/viola-pty/src/lib.rs` · `src/cmd/run.rs` · `tests/tui_passthrough.rs` · route/master (promotion) · chunk
  folder + evidence. (`git diff --name-only a69c5ef` minus `.andromeda/runs/`: 13 non-chunk-folder paths.)
- **Symbols / APIs:**
  - NEW internal harness subcommand `viola-harness pre-push` (`Cmd::PrePush`, `COMMANDS` 9 → 10), forwarded by both
    shims' internal-subcommand arm; not an agent command (the 5-command surface is unchanged). Its document:
    `{"v":1,"cmd":"pre-push","ok",reason?,detail?,"stage",sync{head,tree,files,ms},cache{bytes,cap,cleaned,bytes_after},
    linux{run,gate},legs{"ubuntu-latest","windows-2025"}{ok,reason?,base,verdict,tested},gate}`; exit 0 green · 1 red
    or a stop reason · 2 `pre-push-windows-only`. Stages, fail-fast in order: `host · tools · sync · cache ·
    linux-tests · linux-leg · windows-leg · union`.
  - NEW closed values: `cmd` `pre-push`; `reason` `pre-push-windows-only` (exit 2), `tool-pin-mismatch`,
    `sync-failed`, `sync-mismatch`, `linux-document-unreadable`, `verdict-missing`, `base-mismatch` (`tool-missing`
    reused); `detail` `wsl-distro-ubuntu`, `cc`, `rustc`, `cargo-nextest`, `cargo-mutants`, `cargo-llvm-cov`,
    `pins-unreadable`, `source-path`, `patch`, `clone`, `fetch`, `reset`, `clean`, `apply`, `cache`, and the Linux
    harness command (`run` / `gate`) for `linux-document-unreadable`; `stage` the eight names above.
  - NEW pub fns in `viola_e2e::harness::pre_push`: `pre_push`, `pre_push_with`, `wsl_path`, `ci_pins`,
    `last_document`; consts `PRE_PUSH_HOST_SUPPORTED = cfg!(windows)`, `CACHE_CAP_BYTES` = 40 GiB (42 949 672 960).
    Callers: `viola-harness` `pre_push_cmd` only (leaf crate, no inbound edge — P3 graph q2).
  - CHANGED `resolve_base` behaviour (`run/mutants.rs` `chunk_flip`): the "flip is HEAD → HEAD^" step now applies only
    when the working tree's `.andromeda/master-route.md` holds NO pending record HEAD's copy lacks (new private
    `uncommitted_promotion`). Signature unchanged; callers `mutants` @ mutants.rs + tests (P3 graph q1).
  - CHANGED `viola_pty::pump` signature: `pump(pty, input, output, spawned: Size, host_size)` — the resize baseline
    is the size the PTY was spawned with, never a second `host_size()` read. Callers (graph, implement run dir
    `graph-pump.txt`): `run` @ `src/cmd/run.rs` (passes `spec.size`) + 10 tests in `crates/viola-pty/src/lib.rs`, all
    updated; no other caller.
  - NEW env var `FAKE_AGENT_PUMP_DELAY_MS`: read ONLY by a `viola` binary built with the test-only `fake-agent` cargo
    feature (`#[cfg(feature = "fake-agent")] fn hold_pump_start` in `src/cmd/run.rs`), absent from every release
    build; parsed as u64 milliseconds, capped at 5 000 ms; holds the pump back after the child starts so a test can
    land a host resize in the spawn→pump window. A test seam, never a configuration channel; it disables no control
    and widens no redaction. Not `VIOLA_*` (the no-env-config rule) and not `AGENT_RUN_*` (never read by `viola`).
  - NEW repo path `target/pre-push/` (gitignored under `target/`): the sync's temporary index and binary patch,
    outside `target/agent-run/` because the patch is repository source and `secret-scan` scans `target/agent-run/`.
  - NEW host-side (not repo) state: the WSL clone `~/viola-pre-push` in Ubuntu's Linux filesystem, `~/.rustup`,
    `~/.cargo` there; the temp trail `viola-resize-<pid>.ndjson` (temp dir, test-written, removed on success).
- **Crates / modules:** `viola-e2e` gains module `harness::pre_push`; no crate added or removed; `viola-pty`'s public
  `pump` signature changed (above).
- **Dependencies:** none added or bumped (`Cargo.toml` / `Cargo.lock` untouched — `git diff --name-only a69c5ef --
  Cargo.toml Cargo.lock` → 0).
- **Schema / config:** none (no `schemas/` or `config.json` change); the new closed values above are harness-document
  values under test-plan §3 Closed enums.
- **Spec-master edits:** none by implement (all via this wrap).
- **Counts / qualifiers moved:** harness internal subcommands 5 → 6 (`supervise`, `ui-restart`, `schema-check`,
  `secret-scan`, `gate` + `pre-push` — test-plan §3 :611-659 lists 5 today); `viola-harness` `COMMANDS` 9 → 10;
  Linux test count in the ubuntu `coverage` suite 399 → 401 (the forced-window test + the pump unit test; `pre-push`
  documents); both mutation legs tested 87 mutants on this chunk (CI run 36282518379 job logs; local entry 17).
- **Dev-tool versions:** HOST tools installed this chunk, on the dev host's WSL2 distro `Ubuntu` 26.04.1 LTS (not the CI
  image): C toolchain `build-essential 12.12ubuntu2.26.04.2` / `cc` gcc 15.2.0 (none → installed 2026-09-26);
  **rustup** (the toolchain manager) rustup-init 1.29.1 (none → installed); **rustc** (compiler) 1.98.1 via
  `rust-toolchain.toml` + `llvm-tools-preview`; **cargo-nextest** (test runner) 0.9.146, **cargo-mutants** (mutation
  runner) 27.1.0, **cargo-llvm-cov** (coverage runner) 0.9.1 (none → installed, `cargo install --locked` at ci.yml's
  test-job pins). The Windows host's tools are unchanged (none — re-read: rustup 1.29.1). No lockfile-resolved crate
  is this line's subject.
- **Harness / gate surface:**
  - `pre-push` (above) and `scripts/wsl-provision.sh` (in-distro provisioning: sha256-pinned rustup-init 1.29.1
    `dda72343…cb71`, `rustup toolchain install` from `rust-toolchain.toml`, `cargo install --locked` of the pins parsed
    from ci.yml's test-job `tool:` line; `--check`; `--probe` → `2/2 refused, control clean`; never runs sudo;
    `tool-missing: cc` names the root apt command).
  - `pre-push` launches every WSL call as `wsl.exe -d Ubuntu [--cd D] --exec /usr/bin/env -i HOME=… PATH=<home>/.cargo/
    bin:/usr/local/bin:/usr/bin:/bin …` (never the `--` form; research M2/M4/M5).
  - The sync: clone once (`git clone --no-hardlinks /mnt/<drive>/…`), then per run a Windows temp-index binary patch
    (`GIT_INDEX_FILE=target/pre-push/index`, `read-tree HEAD` → `add -A` → `write-tree` → `diff --cached --binary
    --output=target/pre-push/tree.patch HEAD`) applied after `fetch HEAD` → `reset --hard <sha>` → `clean -fdq`; the
    clone's `add -A` + `write-tree` must equal the Windows tree id (`sync-mismatch` otherwise). Measured ≈1.2–1.4 s per
    run (`evidence/sync-measurement.md`).
  - The Linux verdict = ubuntu `run --coverage` + `gate --require coverage,doctest`; the mutation verdict = the
    `ubuntu-latest` leg in WSL + the `windows-2025` leg on the host + CI's own `gate --require mutants --mutants-legs
    ubuntu-latest,windows-2025`, both legs required to print the same `base`.
  - Operator-pass order (this chunk's plan, dogfooded): stop rust-analyzer → `pre-push` on the uncommitted tree → the
    pre-CI commit → the guarded push → the CI reads; a red `pre-push` stops the pass.
  - The resize tui test: its deadline is 8 s (`RESIZE_WITHIN`), below nextest's 10 s `mutants` kill (5 s × 2,
    `.config/nextest.toml`), and it appends its observations to `viola-resize-<pid>.ndjson` as it goes.
- **Cross-project / external claims:**
  - CI run **36282518379** on **054ebe4** (the pre-CI commit this wrap adds to): conclusion success, 15/15; both
    mutation legs `"base":"a69c5efb082f1d04f067073a5638618fd709be7e"`, tested 87, 0 survived; `mutants-verdict`
    breaches `[]` (read by the session, `evidence/operator-pass.md`, and independently by the overseer — wrap directive).
  - CI run 36272899441 on a69c5ef: success, 15/15 (overseer relay at P5).
  - CI ubuntu runner image `ubuntu-24.04` / `20260920.314.1` (job 108482499487 log) vs local Ubuntu 26.04.1.
  - GitHub REST check-runs doc: `per_page` default 30, `filter` default `latest` (fetched 2026-09-26).
  - The Andromeda `plan-template.md` proposal lives in `evidence/plan-template-proposal.md`, for the overseer to relay
    to overseer1 (a project chunk never edits Andromeda).
- **Reverted / negative API facts:** a TEMPORARY instrument in `src/cmd/run.rs` (`probe_stamp` writing
  `<home>/pump-probe.ndjson` with the spawn read, the pump's baseline read and each changed poll) was written for the
  measurement and removed whole before the pre-CI commit — it never shipped (a product file outside the declared sinks,
  observability.md §Sinks). Nothing else was reverted.
- **Insufficient fixes (written, kept, not the remedy):** the kill-proof trail (experiment B) was written to capture the
  red under the gate; the red was instead established by the forced window (experiment A) — the trail stays as the
  test's failure evidence, it did not capture the original red. Also: `apt-get update` by hand fixed entry 6 once; the
  remedy is the new standing plan entry, not the hand run.
- **Spec claims disproved by measurement:**
  1. test-plan §3 `run` step 4 Base (@553): "When that commit is HEAD itself (the wrap push, whose own commit is the
     flip), the base is `HEAD^`" — assumes HEAD equals the flip only at the wrap push. MEASURED false: HEAD also equals
     the flip right after every wrap, while the next chunk is promoted but uncommitted (/implement and the pre-commit
     operator pass); there the rule derived acd08c7 where CI derives a69c5ef (`git log -1 -G ' · complete · ' HEAD --
     .andromeda/master-route.md` → a69c5ef = HEAD at P4; scope item 4). Fixed in code; the spec sentence needs the
     uncommitted-promotion condition.
  2. test-plan §5 Module ↔ PTY row (@920) "a resize is propagated" held as a claim but the product violated it: a resize
     landing between the spawn sizing and the pump's first look was lost. MEASURED by a forced window: 6/6 red with the
     exact signature before the fix, control 6/6 green; the probe read `spawn` 80×24 → `resize-sent` +23.7 ms →
     `baseline` 100×30 at +990 ms (`evidence/linux-red-investigation.md` §Experiment A). Fixed in code (6/6 green after).
  3. Route CARRY 2 on this chunk's working entry ("The leg's printed `base` should equal the Windows and CI legs' base")
     — false before the fix for a pre-commit run (item 1); true after it (entry 17 and CI both a69c5ef).
  4. Research M3 (plan-level, not a master) "the apt install is an operator step (sudo password)" — disproved by the
     overseer's measurement: `wsl -d Ubuntu -u root` is uid 0, no password (re-read at P5).
- **Expected amendments (from plan):** (site search: fixed-string per master, `scratchpad/amend-sites.py`; hits listed)
  1. test-plan §3 Internal harness subcommands: the `pre-push` entry — carried (Symbols/APIs; Harness surface).
     Site: `**Internal harness subcommands**` test-plan 1 hit @611.
  2. test-plan §3 Closed enums: the new values — carried (Symbols/APIs). Site: `- **Closed enums:**` test-plan 1 @660.
  3. test-plan §3 `run` step 4 Base: the uncommitted-promotion rule — carried (Spec claims disproved 1). Site: `When
     that commit is HEAD itself` test-plan 1 @553.
  4. test-plan §9 tool-install paragraph: WSL provisioning reads ci.yml's pins — carried (Harness surface;
     Dev-tool versions). Site: `exactly one version source` test-plan 1 @1446.
  5. test-plan §10 Mutation gate: the local union before the push — carried (Harness surface). Site: `Mutation gate`
     test-plan 8 hits (@1438 §9 row, @1501 §10 …).
  6. test-plan §12: one Decisions Log entry (the gate, its values, the base rule, the folded red and its measured
     cause) — carried. Site: `Decisions Log` test-plan 13 hits (@666 the Closed-enums rule …).
  7. architecture §Project directory structure: `scripts/wsl-provision.sh`, `harness/pre_push.rs` — carried (Files).
     Site: `install-ripgrep.sh` architecture 4 hits (@37, @398, @488, @522) — the tree's scripts listing.
  8. architecture §Occupied Resources → Repository: `target/pre-push/` — carried (Symbols/APIs). Site:
     `target/release-check` architecture 1 @393.
  9. architecture §Infrastructure Patterns → CI/CD approach: the local pre-push gate precedes the operator push —
     carried (Harness surface). Site: `mutants-verdict` architecture 3 (@386, @522, @523); `rustup toolchain install`
     @522.
  10. security-plan §Dependency Security: the pinned rustup-init + `cargo install --locked` inside WSL — carried
      (Dev-tool versions; Harness surface). Site: `rustup toolchain install` security-plan 2 (@323, @339).
  11. security-plan §Secret Management: `env -i` into WSL, no `CLAUDE*` crossing — carried (Harness surface). Site:
      `AGENT_RUN_CHUNK_BASE` security-plan 1 @340 (the CI-integration paragraph); `WSL` 0 hits in all five (new).
  12. obs-plan §9 Mutation row: the local union is the same verdict — carried. Site: `mutants-verdict` obs-plan 2
      (@1209, @1253); `Mutation gate` obs-plan 1 @1253.
  13. obs-plan §8 item 6: Linux clone paths never in a document — carried (Symbols/APIs, document shape). Site:
      `Unscanned uploads` obs-plan 1 @1208.
  14. test-plan §5 Module ↔ PTY (the resize bullet): the pump's baseline is the spawn size — carried (Spec claims
      disproved 2). Site: `a resize is propagated` test-plan 1 @920.
  15. test-plan §12 names the folded red and its cause — carried (merged into entry 6).
  16. architecture §Occupied Resources → Environment variables: `FAKE_AGENT_PUMP_DELAY_MS` — carried (Symbols/APIs).
      Site: `AGENT_RUN_CHUNK_BASE` architecture 1 @364 (the env registry); `VIOLA_*` architecture 1 @546.
  17. security-plan (the env-var rule) carve-out for the seam — carried; OPERATOR DIRECTIVE at this wrap: record it as
      the architecture + security-plan amendments (cfg(feature="fake-agent"), capped at 5 s, absent from release). Site:
      `VIOLA_*` security-plan 1 @577; the related "env vars are not a configuration channel" rule lives in obs-plan 5
      hits (@39, @604, @606, @1139, @1564) — obs-plan is named too so the carve-out is not contradicted there.
- **Coverage of new surfaces:**
  - `viola-harness pre-push` → validation typed-refusals + pin check + tree-id equality✓ · instrumentation n/a (harness
    JSON document + exit, obs-plan §11 CI) · PII no absolute path / home in the document✓ (unit test; entry 14 grep) ·
    tests unit (25 `pre_push_` tests) + live (gate entries 13/17) · a11y n/a · tokens n/a
  - `scripts/wsl-provision.sh` → validation sha256 + pin comparison✓ · instrumentation n/a · PII n/a · tests `--probe`
    (2 refusals + control) · a11y n/a · tokens n/a
  - `FAKE_AGENT_PUMP_DELAY_MS` seam → validation u64 parse, 5 000 ms cap✓ · instrumentation n/a · PII n/a · tests integ
    (`tui_host_resize_in_the_pump_start_window_reaches_the_child`, latency band 0.9–3 s) · a11y n/a · tokens n/a
  - `viola_pty::pump(…, spawned)` → validation n/a · instrumentation n/a (hot loop, observability.md) · PII n/a · tests
    unit (`pump_forwards_a_resize_that_lands_before_its_first_look` + 10 updated) + integ (forced window) · a11y n/a ·
    tokens n/a

## Deviations from intent
- **Scope widened at implement (operator ruling, "fold, don't split"):** product code changed in `crates/viola-pty`
  and `src/cmd/run.rs`, and `tests/tui_passthrough.rs` changed — the plan's original boundary was test-side only. The
  gate's first real run found a Linux red; its cause was measured (experiment A) before the fix. The widening went
  through the fix-loop's "edit plan + research" path on the operator's word, because `/andromeda-phase` refuses a
  `pending` chunk (scope item 6, plan steps 10–13).
- **A test-only env seam** (`FAKE_AGENT_PUMP_DELAY_MS`) against the no-env-config rule in spirit — compiled only with
  `fake-agent`, capped at 5 s, absent from release; recorded as amendments (operator directive at this wrap).
- **`apt-get update` ran once by hand as root** before its plan entry existed (entry 6 read exit 100 on three 404s);
  the plan then gained it as a standing entry (operator ruling).
- `fuzz_channel` reused for `rust-toolchain.toml` instead of a new `toolchain_channel` parser.
- The window-test gate entry was authored with `test(=…)` (exact full name), which selects nothing for an inline unit
  test (`tests::…`); corrected to the regex form in the same widening.
- A bytecode cache (`evidence/guards/__pycache__/guards.cpython-314.pyc`) rode the pre-CI commit; deleted at this wrap.
- The code-graph query at implement auto-regenerated the stale rust DB (/implement does not rebuild the graph by
  contract; the query tool did it on a stale stamp).

## Decisions & corrections
- The overseer: "fold, don't split — a red found now goes into this chunk"; the cause must be MEASURED, not inferred
  from "it went green"; a variation that does not depend on luck (force the window) before sampling.
- Sweep hazard: an nextest `test(=name)` filter needs the FULL test name — inline unit tests are `tests::name`, so the
  bare name selects nothing (baseline read `nextest-exit-4`).
- Sweep hazard: `wsl.exe -d Ubuntu -- cmd` re-parses the joined argv through the distro's shell (`$t` expanded empty);
  `--exec` passes argv verbatim. From Git Bash a `/mnt/c/…` argument needs `MSYS2_ARG_CONV_EXCL='*'`.
- A test whose own deadline sits at the runner's kill line loses its diagnostic: nextest's `mutants` profile killed the
  resize test at 10.006 s, before its 10 s assertion could print the dump.
- `git reset --hard` already drops files the clone's own `add -A` staged, so a `clean` guard test needs a stray
  clone-side file (the first g8 pair was vacuous).
- The distro's apt lists go stale: a root install needs `apt-get update` first (measured 404s).
- The WSL distro PATH carries the Windows PATH (incl. the Windows cargo); a WSL call hands an explicit PATH.
- The local gate measured: ~850 s full verdict vs CI 1301 s push→last check (overseer1's fast-feedback slot).

## Outcome
- Acceptance criteria, re-asserted against the diff:
  - Base rule: with HEAD at the flip and an uncommitted promotion the base is the flip; the pass and override rules
    are unchanged — MET (`chunk_base_` tests; every `pre-push` and both CI legs print a69c5ef).
  - `pre-push` one document with a typed exit, green only on Linux tests + both legs + union + equal bases — MET
    (unit tests; entry 13 ×3 + entry 17 green; union 0 breaches).
  - Planted `#[cfg(unix)]` red: compiled out here (`nextest-exit-4`), `pre-push` red at `linux-tests` naming it, no leg —
    MET (`evidence/witness-unix-red.md`).
  - Sync proof per run (tree ids equal; ignored files never carried; real index untouched) — MET.
  - Pinned toolchain; refusals typed — MET (entries 10–11; unit tests).
  - No `CLAUDE*` crossing; no absolute path in the document or verdict — MET (entry 12 canary; entry 14 grep; unit tests).
  - Cache bounded and reported — MET (cap 40 GiB, reported 7.0–7.2 GiB).
  - "Only `crates/viola-e2e/**` and `scripts/` change; no product crate" — the ORIGINAL criterion is CONTRADICTED by the
    diff (viola-pty + src/cmd/run.rs changed) and was SUPERSEDED by the operator's scope widening (plan step 11 and the
    amended §Constraints line "any product-crate change beyond the pump baseline of steps 11–12: out of scope"); the
    amended criterion is MET.
  - The Linux red's cause MEASURED before the fix; forced-window test; three consecutive greens — MET.
  - Guards g1–g9 recorded — MET (`evidence/guards/readings.json`, 9 pairs ok; g8's first pair vacuous, its rerun ok).
  - Operator pass: `pre-push` before the pre-CI commit; CI legs' base = local a69c5ef — MET (`evidence/operator-pass.md`).
- Gates (implement, gate tool, final block `fixed-full.out`): `cargo fmt --all --check` green · `cargo clippy --workspace
  --all-targets --features fake-agent -- -D warnings` green · `bash scripts/agent-run.sh run --unit` green · the
  `pre_push_|chunk_base_of_an_uncommitted_promotion` filter green · the `pump_forwards_a_resize_that_lands_before_its_first_look`
  filter green · `AGENT_RUN_KEEP_HOMES=1 … run --integration` green · `… -u root --exec /usr/bin/apt-get update` green ·
  `… apt-get install -y build-essential` green · `… /usr/bin/cc --version` green · `… wsl-provision.sh` green (`pins ok`) ·
  `… wsl-provision.sh --probe` green · the `CLAUDE_CODE_MESSAGING_TOKEN` canary green (last line 0) · `bash
  scripts/agent-run.sh pre-push` green ×3 (846.2 / 850.2 / 852.2 s) · the verdict path-grep green (last line 0) · `grep -c
  pre-push scripts/agent-run.ps1` green · the MSRV check green. `leg = 'operator'` entries run by the session on the
  operator's word: `pre-push` (entry 17) green 851 s · the guarded push exit 0 (`a69c5ef..054ebe4`) · check-runs
  `success` · the mutants read `success,success,success` — all recorded in `evidence/operator-pass.md`, CI run
  36282518379 on 054ebe4.
- Smoke: skipped — no boot-path change; the changed entry points ran live at P2.
- Outcome basis: the implement conversation in this session, the operator's two directives between implement and this
  report (the scope widening + experiments A/B; the operator pass), and the post-implement artifacts named above.
- Process hygiene (implement census, re-measured at the operator pass): nothing this chunk's runs started remains —
  wsl/cargo/nextest/harness 0 on Windows, none in Ubuntu; 4 `viola.exe` are the operator's `viola-lab/prototype`
  sessions (not this chunk's); `vmmemWSL` idle (the WSL VM, self-stops); the 8 kept-on-red trail files deleted.
