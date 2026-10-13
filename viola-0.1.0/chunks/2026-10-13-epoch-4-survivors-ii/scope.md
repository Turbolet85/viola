# Scope — 2026-10-13-epoch-4-survivors-ii · Epoch 4 survivors II

**Working entry** (`viola-0.1.0/working-route.md`, the first markerless entry, third under
`### Epoch 5 — Driver surface`, directly after "Epoch 4 survivors"): Epoch 4 survivors II — the Epoch 4 audit's 16
`typed.rs` survivors in `viola verify`'s Run B driver disposed: an owning-package test after a behaviour-preserving
restatement, or a not-measured-here record — plus one CARRY block (folded below; completeness check `route.py pins
--run-dir` → this run dir's trail: one row on the entry, 2051 chars, no abstention).

**The operator's direction at take-up** (`inputs#I1`, the invocation, verbatim in the snapshot):
- **no mutation run, witness run or workflow dispatch inside the chunk** (the founder's standing direction): a kill is
  shown by applying the mutant by hand, and each survivor is worded by what the next boundary audit reads — the
  Epoch 5 boundary audit (the founder ruled no boundary inside Epoch 5; carried in the handoff and in the CARRY's
  inherited direction);
- **no live `claude` start** without the founder's cap: the plan holds zero;
- the chunk is **sized against one window at P5**.

**CI read at Setup 5a** (the last wrap's flip `64d7e5f` = HEAD, `ci.py conclusion`, this run dir's trail):
- `64d7e5f`: **green**, checks 15/15, wall 524 s, `ci#38437360080` (push, completed/success). Nothing to fold.

## What this chunk builds

### 1. The 16 survivors (the entry, CARRY)
- Source: the Epoch 4 boundary audit, `.andromeda/runs/2026-10-12T06-54-29-code-audit/record.json`; coordinates at
  `dffad3e`. Each row stands `owed` / `by: "Epoch 4 survivors II"` in
  `viola-0.1.0/chunks/2026-10-13-epoch-4-survivors/evidence/survivors.ndjson` (16 `owed` rows of its 18 `typed.rs` rows,
  read at P1).
- Coordinates re-verified at P1 against HEAD: `git diff dffad3e HEAD -- src/cmd/verify/typed.rs` is one hunk,
  `@@ -831,0 +832,18 @@` inside the test module (which opens at `:689`), so every coordinate below stands at HEAD;
  each line was read at its column and holds the operator the mutation names.
- **`framing_turns` 5:** `:271:84` (the second `>` of `count(UserPromptSubmit) > prompts && count(Stop) > stops`)
  ×2 (`>` → `==`, `>` → `>=`); `:286:29` (`clock.now() + PROBE_DEADLINE`, `+` → `-`); `:288:40` (the first `>` of
  `count(SessionStart) > starts || count(UserPromptSubmit) > prompts`) ×2 (`>` → `==`, `>` → `>=`).
- **`<impl Pty for SharedPty>` 5:** `resize` `:403:9` (→ `Ok(())`), `close` `:415:9` (→ `()`), `child_pid` `:419:9`
  ×3 (→ `None`, `Some(0)`, `Some(1)`).
- **`Run`'s ending 6:** `end_by_ctrl_c` `:648:12`, `:651:12` (delete `!`), `:648:45`, `:651:45` (`+` → `-`);
  `wait_pump` `:659:9` (→ `true`); `finish` `:671:53` (`+` → `-`).
- Measured at the earlier chunk's phase and re-read at HEAD by this chunk's P3 (CARRY; that chunk's research.md
  §Measured facts 4 and 5; this chunk's research.md §Measured facts 1, 2): "`typed.rs` has no `Pty` double seam
  (`SharedPty` wraps the concrete `PortablePty`, `Run` holds `Arc<Mutex<PortablePty>>`); the pump calls `resize`
  only on a new size and never `child_pid`, so `SharedPty::resize` and `::child_pid` are forwarders no product path
  calls; no unit test builds a `Run`." Verified. One addition: verify's pump never resizes (its size closure is
  `|| None`), and the pump does call `SharedPty::close` (on the child's exit and in `kill_and_reap`).
- [premise-corrected: research.md §Measured facts 1, 3, 4, 5 — no test can steer a whole `Run` through one clock (the
  pump reads real time); the ending needs no `Pty` double; the root package has no `mockall`] The disposal route the
  founder's split named (the earlier chunk's P5 card row 1, `p5-card-1.md`) was "one restatement of verify's Run B
  driver (a Pty double seam plus named framing waits)". As re-derived, it is three restatements in `typed.rs`, every
  survivor's operator moved into a function its cases call directly:
  - the framing waits as two named predicates over the captures dir, plus one named deadline for
    `clock.now() + PROBE_DEADLINE`;
  - `SharedPty` generic over its `Pty`, killed through a hand-written recording double in the test module;
  - the Ctrl-C ending and the pump wait as plain functions of a clock and closures. `Run` keeps no bool-returning
    wrapper or `+` that only a `Run` reaches.
  No real PTY child and no `claude` are started. Every site is reachable on Linux, so no "not measured here" record is
  expected (research.md §Measured facts 6).

### 2. The three disposal forms (the entry; test-plan §10)
- **a nextest test in the owning package** (the root package `viola`; `typed.rs`'s inline `mod tests`), failing on
  an assertion under the hand-applied mutant and passing on the original;
- **a behaviour-preserving restatement** whose mutants a test then kills;
- **"not measured here; owed to {route entry}"** only for a site this host cannot reach — never "accepted
  equivalent", and no survivor disposed by argument.
- A kill's case is selected by `test(/(^|::)tests::name$/)`, never `test(=tests::name)` (the CARRY;
  verification-harness.md 2026-09-27, corrected at that wrap).
- Each kill is shown red under its hand-applied mutant and green restored, inside the chunk; the confirming reading is
  the Epoch 5 boundary audit's (`inputs#I1`).

### 3. Where the records land
- This chunk's own ledger `evidence/survivors.ndjson` (one row per survivor, the earlier chunk's shape:
  `coordinate` · `mutation` · `file_at_head` · `form` · `by`), its human twin, and test-plan §10's Mutation gate
  paragraph, where the earlier chunk's disposal is recorded. Any master wording the chunk changes is a wrap amendment,
  never a phase edit. The earlier chunk's `owed` rows stand as written (a frozen chunk's evidence); the new ledger
  names them by coordinate.

## Boundaries
- No mutation run (`cargo mutants`), witness run or workflow dispatch inside the chunk (`inputs#I1`); a hand-applied
  mutant in the working tree, reverted, is the only mutant this chunk runs.
- No live `claude` start (`inputs#I1`). No test starts any PTY child: every case runs on a recording double, a tempdir
  of planted captures, a fixed clock or a thread the test owns (research.md §Measured facts 4).
- No behaviour change: `viola verify` measures, types and records exactly what it does now — no new capability-ledger
  row, no probe text, deadline or order change; a test-only seam stays behind `cfg(test)` or an existing test
  feature.
- Only `src/cmd/verify/typed.rs` (and, if a seam requires it, its direct callers in `src/cmd/verify/`) is in reach;
  `viola-pty`'s `Pty` trait is not changed by this chunk unless P3 shows no other route.
- Sizing (`inputs#I1`): 16 survivors in one file, three groups, sized against one window at P5.
