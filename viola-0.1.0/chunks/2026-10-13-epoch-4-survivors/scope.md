# Scope — 2026-10-13-epoch-4-survivors · Epoch 4 survivors

**Working entry** (`viola-0.1.0/working-route.md`, the first markerless entry, second under
`### Epoch 5 — Driver surface`, directly after "Epoch 4 cleanup"): Epoch 4 survivors — the Epoch 4 audit's 47
mutation survivors each disposed: an owning-package test, a behaviour-preserving restatement, or a not-measured-here
record by coordinate — plus three CARRY blocks (all three folded below; completeness check `route.py pins --run-dir`
→ this run dir's trail: three rows on the entry, 1030 · 2240 · 603 chars, no abstention).

**The operator's direction at take-up** (`inputs#I1`, the invocation, verbatim in the snapshot):
- **the founder ruled no epoch boundary inside Epoch 5**: the next boundary audit is the one at Epoch 5's end, so
  every record this chunk writes names "the Epoch 5 boundary audit" as its reader;
- **no mutation run, witness run or workflow dispatch inside the chunk** (the founder's standing direction: the
  mutation workflow is dispatched at the epoch boundary only) — each survivor is worded by what the next boundary
  audit reads;
- **no live `claude` start** without the founder's cap: the plan holds zero;
- the chunk is sized against one window at P5 (the Epoch 4 boundary's wording: a split is proposed if it does not
  fit, `.andromeda/runs/2026-10-13T06-38-35-wrap/relay-2.md`, question 3).

**The split — the founder's own answer to the P4 Size fork** (`inputs#I2`, by dialog, «Делить на две», relayed
verbatim by the operator): this chunk takes the 31 survivors disposed by a unit case or a small restatement, plus the
2 `scratch.rs` records; a new entry **"Epoch 4 survivors II"**, directly after this one, takes `typed.rs`'s other 16
(the Run B driver restatement: `framing_turns` 5, `SharedPty` 5, `Run::end_by_ctrl_c` / `wait_pump` / `finish` 6).
The bullets below keep all 47 as the survey; §1a marks the 16 that leave.

**CI read at Setup 5a** (the last wrap's flip `c6e1301` = HEAD, `ci.py conclusion`, this run dir's trail):
- `c6e1301`: **green**, checks 15/15, wall 503 s, `ci#38388446920` (push, completed/success). Nothing to fold.

## What this chunk builds

### 1. The 47 survivors, each disposed by one of three forms (the entry, CARRY 1)
- Source: the Epoch 4 boundary audit, `.andromeda/runs/2026-10-12T06-54-29-code-audit/` — `proposals.md`
  §Informational, `record.json`'s `mutation.survivors` (47 rows, read at P1) and the `c-mutation-{unit}.json` twins;
  host `x86_64-unknown-linux-gnu`; coordinates at `dffad3e`.
- The three forms (the entry; test-plan §3 `run` step 4 and §10, as the Epoch 4 cleanup's research read them):
  - **a nextest test in the package that owns the mutated file**, failing on an assertion under the hand-applied
    mutant and passing on the original;
  - **a behaviour-preserving restatement** that removes the mutant's site;
  - for a site this host cannot reach, **"not measured here; owed to {route entry}"** by coordinate.
  No "accepted equivalent" form exists for a reachable site; no survivor is disposed by argument (the Epoch 3
  cleanup II precedent).
- Each disposal is worded by what the Epoch 5 boundary audit reads (`inputs#I1`): the audit's mutation run is the
  only reader that can confirm a kill, so a new test is shown failing under its hand-applied mutant inside the chunk,
  and the outcome is the audit's to read.
- Coordinates: the audit's, at `dffad3e`. `git diff --stat dffad3e HEAD -- '*.rs'` (read at P1) touches, of the
  survivor files, only `src/run/send.rs` (split by phase into `src/run/send/*.rs`) and
  `crates/viola-e2e/src/harness/run/mutants/scratch.rs` (37 lines, the cleanup's `wipe` edit). The function name is
  the stable key; `close` (`src/run/send.rs:271:5`) now sits in `src/run/send/slot.rs` (CARRY 3).
  The other nine files' coordinates stand at HEAD (`git diff --stat dffad3e HEAD` per file: empty); `scratch.rs`'s
  `:48` and `:54` are the same statements at `dffad3e` and HEAD; `close` is `src/run/send/slot.rs:206`, its body
  `:207:5` (research.md §Measured facts 3, §Files inspected).

#### 1a. `viola` — 32 (the audit's `counts.missed`; unit score 97.03)
- `src/cmd/verify/typed.rs` 18: `framing_turns` `:271:84` (`>`→`==`, `>`→`>=`), `:286:29` (`+`→`-`), `:288:40`
  (`>`→`==`, `>`→`>=`); `max_gap` `:304:54` (`&&`→`||`); `<impl Pty for SharedPty>` `resize` `:403:9` (→`Ok(())`),
  `close` `:415:9` (→`()`), `child_pid` `:419:9` (→`None`, `Some(0)`, `Some(1)`); `Feed::take` `:439:23` (`>`→`>=`);
  `Run::end_by_ctrl_c` `:648:12` / `:651:12` (delete `!`), `:648:45` / `:651:45` (`+`→`-`); `Run::wait_pump`
  `:659:9` (→`true`); `Run::finish` `:671:53` (`+`→`-`).
  [premise-corrected: `typed.rs` has no `Pty` double seam — `SharedPty` wraps the concrete `PortablePty` and `Run`
  holds `Arc<Mutex<PortablePty>>` (research.md §Measured facts 4, 5)] These are `viola verify`'s Run B paste-framing
  driver; no test may start a live `claude` (`inputs#I1`). Two are pure functions killable by a unit case (`max_gap`
  `:304:54`, `Feed::take` `:439:23`). The other 16 (`framing_turns` 5, `SharedPty` 5, `Run::end_by_ctrl_c` /
  `wait_pump` / `finish` 6) need a restatement of the driver that adds a `Pty` double seam and names the framing
  waits; `SharedPty::resize` and `::child_pid` are forwarders no product path calls.
  **With the split (`inputs#I2`): this chunk disposes `max_gap` `:304:54` and `Feed::take` `:439:23`; the other 16
  leave for "Epoch 4 survivors II", which the wrap mints directly after this entry from the P5 card.**
- `src/run/wheel.rs` 9: `Recorder::record` `:297:18` (`&&`→`||`); `Classifier::abort` `:393:9` (→`true`);
  `Classifier::step` `:427:17` (delete arm ESC), `:434:32` (guard `self.params.len() < BOUND` → `true`), `:434:50`
  (`<`→`<=`), `:458:24` (guard `osc` → `true`), `:467:35` (`<`→`<=`); `is_reply` `:516:49` (`==`→`!=`), `:518:47`
  (`&&`→`||`).
- `src/run/dialog.rs` 2: `millis` `:144:5` (→`1`); `DialogSlot::hook_dialog` `:208:42` (`!=`→`==`).
- `src/cmd/hook.rs` 1: `answer` `:403:27` (`||`→`&&`).
- `src/cmd/send.rs` 1: `Out<'_>::open` `:119:12` (delete `!`).
- `src/run/send.rs` 1: `close` `:271:5` (→`()`), now `src/run/send/slot.rs` (CARRY 3; new tests land in the split
  files).
- The entry: "the rest (`src/run/wheel.rs` 9, `src/run/dialog.rs` 2, `src/cmd/hook.rs` 1, `src/cmd/send.rs` 1,
  `src/run/send.rs` 1, …) not read past their coordinates" — P3 reads each.

#### 1b. The other units — 15
- `viola-state` 10:
  - `board.rs` 4: `SkippedTotal::add` `:164:29` (`+=`→`*=`, `+=`→`-=`); `check_if_present` `:45:19` and `scan`
    `:67:19` (guard `e.kind() == io::ErrorKind::NotFound` → `true`). Not read past their coordinates (the entry).
  - `fs.rs` 6: `replace_private_with` `:281:18` (`+=`→`*=`), `:285:20` (the guard → `false`), `:285:29` (`<`→`<=`,
    `==`, `>`), `:286:21` (`&&`→`||`). [premise-corrected: `:286:21` (`&&`→`||`) changes behaviour on Linux — any
    failed `persist` then pauses and retries 99 times (research.md §Measured facts 2)] The retry decision is the
    const `HOST_IS_WINDOWS = cfg!(windows)`, false on Linux. So five of the six (`:281:18`, `:285:20`, the three
    `:285:29`) behave as the original on this host, and the harness never leaves a const-gated site out. They are
    reachable on Linux only if the caller supplies the decision (a restatement). `:286:21` is killable on Linux as it
    stands, by a `persist` that fails. These three coordinates are also the subject of a CARRY on "Paste newline
    ledger row"; that CARRY stays on its own entry.
- `viola-agent-claude` 2: `statusline.rs` `shell_argv` `:110:5` (→`Some([""; 3])`, `Some(["xyzzy"; 3])`). Not read
  past their coordinates (the entry).
- `viola-e2e` 2: `harness/run/mutants/scratch.rs` `prepare` `:48:5` (→`Ok(None)`), `:54:8` (delete `!`).
  The entry: "already recorded 'not measured here; owed to' a route entry (chunk 2026-10-09-epoch-3-cleanup-ii) and
  read caught on `windows-2025` in run 38036448183"; "coordinates unmoved by that chunk's `wipe` edit". Both
  verified: test-plan §10 holds the record (`.andromeda/test-plan.md:1226`), and `:48` / `:54` are the same
  statements at `dffad3e` and HEAD, both inside the `HOST_SCRATCH` (`cfg!(windows)`) branch (research.md §Measured
  facts 3).
- `viola-pty` 1: `lib.rs` `<impl Pty for PortablePty>::resize` `:202:9` (→`Ok(())`). The entry: "three
  Linux-compiled tests resize and wait for `size 120x40`, and why the audit read the mutant missed is not known —
  hand-apply it at take-up". Verified, with one more fact: the three tests are viola-pty's own (`lib.rs:1196`,
  `:1233`, `:1255`), and under the mutant all three PASSED after 5.35–6.11 s. The child reported `size 120x40` about
  5 s after a resize that did nothing (the audit's per-mutant log; research.md §Measured facts 1). What
  changed the size is not measured (the watch reports are gone); P3 is read-only, so the hand-application is
  implement's first step on this item, and the disposal's form follows from it. HYPOTHESIS, read at P4 and not
  measured (research.md §Measured facts 1): the audit ran two cargo-mutants jobs under one `TMPDIR`, and the child's
  report file is named after the test alone, so the other job's unmutated run can write the awaited line.

### 2. Where the records land
- A "not measured here" record and a restatement's note sit where test-plan §10 already holds the `scratch.rs`
  precedent (`.andromeda/test-plan.md:1226`, the Mutation gate paragraph). The plan names the place by its section,
  and any master wording the chunk changes is a wrap amendment, never a phase edit.

### 3. The root package's Windows mutation baseline (CARRY 3, an observation, not this chunk's work)
- watch: the green reading of `windows-mutants.yml` on the root package (run 38215104120 red at `dffad3e`; its cause
  removed by the Epoch 4 cleanup, witnessed on CI at ci#38375200217) is not measured — it is read at the next
  `windows-mutants.yml` dispatch, the Epoch 5 boundary audit (1/1; since chunk 2026-10-13-epoch-4-cleanup).
  No dispatch inside this chunk (`inputs#I1`).

## Boundaries
- No mutation run (`cargo mutants`), witness run or workflow dispatch inside the chunk (`inputs#I1`); a hand-applied
  mutant in the working tree, reverted, is the only mutant this chunk runs.
- No live `claude` start (`inputs#I1`); no new capability-ledger row, no `viola verify` probe change.
- No production behaviour change: a restatement keeps behaviour; a test-only seam stays behind `cfg(test)` or the
  existing test features.
- The "Paste newline ledger row" CARRY on `fs.rs` stays there; this chunk disposes the six `fs.rs` survivors under
  the three forms and names that entry where a record is owed to it.
- Sizing (`inputs#I1`): 47 survivors across 11 files in 5 packages, sized against one window. The founder split it
  at P4 (`inputs#I2`): 31 disposed here plus the 2 `scratch.rs` records; `typed.rs`'s other 16 are owed to "Epoch 4
  survivors II", and no step of this chunk edits `framing_turns`, `SharedPty` or `Run`.
