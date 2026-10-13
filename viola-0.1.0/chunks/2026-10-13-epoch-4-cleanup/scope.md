# Scope — 2026-10-13-epoch-4-cleanup · Epoch 4 cleanup

**Working entry** (`viola-0.1.0/working-route.md`, the first markerless entry, the head of
`### Epoch 5 — Driver surface`): Epoch 4 cleanup — Epoch 4's mutation survivors disposed, the Windows mutation
baseline green on the root package, send and human output split by phase, Windows-target lint gated — plus four
CARRY blocks (all four folded below; completeness check `route.py pins --run-dir` → this run dir's trail: four rows
on the entry, 1030 · 503 · 338 · 267 chars, no abstention).

**The operator's direction at take-up** (`inputs#I1`, the invocation, verbatim in the snapshot):
- the chunk is sized against one window at P5, and a split is proposed if it does not fit;
- **no mutation run, witness run or workflow dispatch inside the chunk** (the founder's standing direction: the
  mutation workflow is dispatched at the epoch boundary only). If diagnosing the Windows baseline red seems to need a
  `windows-mutants.yml` dispatch, that is raised as a P4 fork for the founder's word;
- **no live `claude` start** without the founder's cap: the plan holds zero.

**CI read at Setup 5a** (the last wrap's flip `dffad3e` through HEAD `d9ad9f1`, `ci.py conclusion`, this run dir's
`ci-no-marker.json`):
- `d9ad9f1` (the 0-pending wrap's route commit): **verdict not yet available** — `ci#38360572551` in progress,
  checks 15/15 seen; not read as green.
- `dffad3e` (Epoch 4's last chunk): **red**, wall 98603 s, first failure at +79661 s. Red by `mutants (viola-run-env)`,
  `mutants (viola-cmd-run)`, `mutants (viola-panic-frames)`, `mutants (viola-main)` = failure, and
  `windows-mutants#38215104120` = failure. Read back at P1 (`gh run view 38215104120 --json jobs`): those four jobs
  are run 38215104120's own root-package shards (06:57:17Z → 07:03–07:05Z, 2026-10-12); its other five jobs
  (`viola-state`, `viola-channel`, `viola-pty`, `viola-agent-claude`, `viola-e2e`) read success. It intersects this
  chunk (§2) and is folded there.

## What this chunk builds

### 1. The Epoch 4 audit's 47 mutation survivors, disposed (CARRY 1)
- **Leaves this chunk with the split, owed to the entry #1 mints** ("Epoch 4 survivors", directly after this one;
  `.andromeda/runs/2026-10-13T07-07-28-phase/p5-card-1.md`, `## Beyond the plan` rows 1 and 2). The bullets below
  stay as the survey that entry carries.
- Source: the Epoch 4 boundary audit, `.andromeda/runs/2026-10-12T06-54-29-code-audit/proposals.md` §Informational,
  the record's `mutation.survivors` and the `c-mutation-{unit}.json` twins (placed by the operator's answer at the
  Epoch 4 boundary's 0-pending wrap, `.andromeda/runs/2026-10-13T06-38-35-wrap/relay-2.md`, question 3).
- 32 in `viola` (the audit's `counts.missed`; the unit score 99.33 → 97.03), by file:
  - `src/cmd/verify/typed.rs` 18: `framing_turns` `:271:84` (`>`→`==`, `>`→`>=`), `:286:29` (`+`→`-`), `:288:40`
    (`>`→`==`, `>`→`>=`); `max_gap` `:304:54` (`&&`→`||`); `<impl Pty for SharedPty>` `resize` `:403:9` (→`Ok(())`),
    `close` `:415:9` (→`()`), `child_pid` `:419:9` (→`None`, `Some(0)`, `Some(1)`); `Feed::take` `:439:23`
    (`>`→`>=`); `Run::end_by_ctrl_c` `:648:12` / `:651:12` (delete `!`), `:648:45` / `:651:45` (`+`→`-`);
    `Run::wait_pump` `:659:9` (→`true`); `Run::finish` `:671:53` (`+`→`-`);
  - `src/run/wheel.rs` 9: `Recorder::record` `:297:18` (`&&`→`||`); `Classifier::abort` `:393:9` (→`true`);
    `Classifier::step` `:427:17` (delete arm ESC), `:434:32` (guard `self.params.len() < BOUND` → `true`), `:434:50`
    (`<`→`<=`), `:458:24` (guard `osc` → `true`), `:467:35` (`<`→`<=`); `is_reply` `:516:49` (`==`→`!=`),
    `:518:47` (`&&`→`||`);
  - `src/run/dialog.rs` 2: `millis` `:144:5` (→`1`); `DialogSlot::hook_dialog` `:208:42` (`!=`→`==`);
  - `src/cmd/hook.rs` 1: `answer` `:403:27` (`||`→`&&`);
  - `src/cmd/send.rs` 1: `Out<'_>::open` `:119:12` (delete `!`);
  - `src/run/send.rs` 1: `close` `:271:5` (→`()`).
- 15 in the other units:
  - `viola-state` 10: `board.rs` `SkippedTotal::add` `:164:29` (`+=`→`*=`, `+=`→`-=`), `check_if_present`
    `:45:19` and `scan` `:67:19` (guard `e.kind() == NotFound` → `true`); `fs.rs` `replace_private_with` `:281:18`
    (`+=`→`*=`), `:285:20` (the guard → `false`), `:285:29` (`<`→`<=`, `==`, `>`), `:286:21` (`&&`→`||`);
  - `viola-agent-claude` 2: `statusline.rs` `shell_argv` `:110:5` (→`Some([""; 3])`, `Some(["xyzzy"; 3])`);
  - `viola-e2e` 2: `harness/run/mutants/scratch.rs` `prepare` `:48:5` (→`Ok(None)`), `:54:8` (delete `!`);
  - `viola-pty` 1: `lib.rs` `<impl Pty for PortablePty>::resize` `:202:9` (→`Ok(())`).
- The entry's words: "each is worded by what the next boundary's audit will read". [premise-corrected: test-plan §3
  `run` step 4 and §10 hold no "accepted equivalent" form for a reachable site (research.md §1, the tests extract)]
  A survivor is disposed by a nextest test in the package that owns the mutated file which fails on an assertion
  under it, or by a restatement that removes the mutant's site with the behaviour unchanged (the Epoch 3 cleanup II
  precedent: no survivor disposed by argument), or — for a site this host cannot reach — by the record "not measured
  here; owed to {route entry}" by coordinate, the form test-plan §10 already holds for `scratch.rs` `prepare`'s two.
- Coordinates are the audit's, at `dffad3e`, and stand at HEAD (`git diff --stat dffad3e HEAD -- '*.rs'`: empty)
  unless §3's split moves them (`src/run/send.rs:271` moves with its file). The function names are the stable key.
- A survivor's disposal cannot be proven by a mutation run inside this chunk (`inputs#I1`; test-plan §2 Mutation
  row): each new test is shown failing under its hand-applied mutant, and the next boundary audit reads the outcome.
- The three `viola-state` `fs.rs:281`/`:285`/`:286` mutants are the ones the route entry "Paste newline ledger row"
  carries a CARRY about; that CARRY stays on its own entry. Research adds (research.md §1): all six `fs.rs`
  survivors behave as the original on Linux (`retry_replace(_, HOST_IS_WINDOWS)` is false there, a const the harness
  never leaves out), so each is reachable only on Windows unless a restatement lets the caller supply the decision.

### 2. The Windows mutation leg's red baseline on the root package (CARRY 2, and Setup 5a's red)
- Run 38215104120 on `windows-2025` at `dffad3e` (dispatched 2026-10-12T06:57:12Z for the Epoch 4 audit; the audit's
  `c-windows-readback.json`): all four root-package shards (`viola-main`, `viola-cmd-run`, `viola-run-env`,
  `viola-panic-frames`) failed the **unmutated baseline** — `ERROR cargo test failed in an unmutated tree, so no
  mutants were tested` — on `viola::bin/viola cmd::hook::statusline::tests::statusline_an_instance_viola_laid_down_passes_its_check`,
  `assertion failed: trusted(&instance)` at `src\cmd\hook\statusline.rs:510:9`.
- The build dir: the cargo-mutants copy under `D:\a\viola\viola-mutants-scratch\`; the test's tempdir and the
  instance check's owner + DACL read on Windows.
- The same test is green in `ci.yml` on `acad7af` (`ci#38124062614`). The cause is not measured (the entry: "the
  cause is not measured").
- VERIFIED against run 38215104120 (its log re-read at P3; research.md §2), the mechanism no longer a hypothesis:
  the test lays its home and `instances/` down with plain `create_dir_all`, and `create_private_dir(&instance.dir)`
  finds nothing missing, so the tree keeps its tempdir's inheritance. Under the mutants run `TMP`/`TEMP` are
  `D:\a\viola\viola-mutants-scratch\`, which `wipe` re-creates with plain `create_dir_all` on the runner's workspace
  drive, whose BUILTIN\Users `0x2`/`0x4` container-inherit ACEs were measured at ci#37214281447 (chunk
  2026-10-04-dialog-answers-by-dialog-id) and are refused `Err(Writable)`; under CI's checkout run the tempdir is in
  the profile's `%TEMP%`, whose inheritance the check accepts. The check behaves as security-plan writes it.
- Research adds: the baseline stopped at the first failure (`[profile.mutants]` `max-fail = 1`; `425/1224 tests run:
  424 passed, 1 failed`), so 799 root tests never ran in the scratch copy and "only this test" is not established —
  the fix belongs where the scratch is laid down, not in the one test.
- The entry's outcome: the root package's Windows baseline green, so the next boundary yields a Windows verdict for
  `viola`. Without a dispatch inside the chunk (`inputs#I1`), it is shown here by the cause removed (`wipe` lays the
  scratch down owner-only, through `viola_state::fs::create_private_dir`), a viola-e2e Windows test on CI's ordinary
  `test (windows-2025)` leg (the statusline test's tree accepted inside a wiped scratch, refused in a plain-created
  sibling on the same drive), and the next boundary's dispatch reads the verdict. The diagnosis needed no dispatch:
  the measurement was already on disk, so no P4 fork is raised for one.
- The failed jobs also carry an install-action `::error::` about a bash startup failure (partner-runner-images#169)
  in its scripted fallback; the audit read it as not the cause (the jobs reached cargo-mutants).

### 3. Audit M1 and M2: `src/run/send.rs` and `src/human.rs` split along their phases (CARRY 3)
- At the audit (tokei code lines): `src/run/send.rs` 2088 (1464 at the Epoch 3 record), `src/human.rs` 1064 (509).
  Re-read at P1 (`wc -l`, physical lines): 2415 and 1241.
- Split along their phases; "CLI output discipline" (a later Epoch 5 entry) lands in `src/human.rs`.
- `crates/viola-agent-claude/src/ledger.rs` stays whole (2709, +31 this epoch), by the operator's directive.
- A pure move: no behaviour change, no public-surface change; the tests and gates that read either file stay green.
  Verified (research.md §3): neither file holds `cfg` code (no `windows-mutants.yml` file list moves); nothing in
  code, scripts, tests or workflows pins either path; nextest overrides key on name regexes; every outside caller (14
  files into `human`, 4 into `run::send`, the graph query) keeps its path through re-exports. The phases are
  research's reading: for `send.rs` the classification, the in-flight slot, the params, the send and its context; for
  `human.rs` the shared writers, the send mirror, the escapers, the board, wait/last, the verb lines, the hints.
- The other M2 names (`src/run/wheel.rs`, `crates/viola-state/src/events.rs`, `crates/viola-agent-claude/src/hook.rs`,
  the three test files) are not this entry's: the entry names send and human only (verified against the entry text).

### 4. A check-only cross-target clippy for `x86_64-pc-windows-msvc` as a gate entry (CARRY 4)
- CI lints a target the dev host does not build; it ran once by hand (friction-log record `2026-10-10T22:01:41Z-a`,
  not re-read at that wrap).
- The gate entry is the recorded hand form (research.md §4): `CARGO_TARGET_DIR=target/wincheck cargo clippy
  --workspace --all-targets --features fake-agent --target x86_64-pc-windows-msvc -- -D warnings`, check only,
  nothing linked; the target's std is installed on the dev host (`rustup target list --installed`), no C toolchain or
  Windows SDK is read (it ran on this host at three chunks, exit 0 in 0.12 s warm, and once caught a Windows dead-code
  red before a push). Its cold time is the P5 baseline run's reading.
- [premise-corrected: no file in this repository is a gate skeleton later chunks inherit (research.md §4)] It is a
  `[[gate]]` entry in this chunk's plan; that later chunks carry it is a test-plan amendment the wrap writes (§9's Lint
  row), not a file this chunk edits, and no `pre-push` stage is added (its closed `stage` enum is a §3 contract).

## Boundaries
- No mutation run, witness run or workflow dispatch (`inputs#I1`); no live `claude` start.
- No behaviour change in §3's split; no new capability; no master amendment is expected beyond what the wrap reads.
- `ledger.rs` is not split. The other M2 files are not split.
- The `fs.rs` Windows-kill-line CARRY on "Paste newline ledger row" stays there.

## Surfaces touched
- `src/cmd/verify/typed.rs`, `src/run/wheel.rs`, `src/run/dialog.rs`, `src/cmd/hook.rs`, `src/cmd/send.rs`,
  `src/run/send.rs` (+ its split siblings), `src/human.rs` (+ its split siblings), `src/cmd/hook/statusline.rs` and
  whatever §2's cause names; `crates/viola-state/src/{board,fs}.rs`, `crates/viola-agent-claude/src/statusline.rs`,
  `crates/viola-e2e/src/harness/run/mutants/scratch.rs`, `crates/viola-pty/src/lib.rs`, and their tests;
  possibly `.github/workflows/windows-mutants.yml` and the harness's gate skeleton.
