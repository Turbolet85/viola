# P5 review — 2026-10-13-epoch-4-cleanup · Epoch 4 cleanup

**Sized against one window (`inputs#I1`): it does not fit whole. The plan carries §2 + §3 + §4; §1 (the 47
survivors) is proposed as its own entry, directly after this one.**

## The size, and the split proposed
- **Precedent:** the Epoch 3 cleanup was split in two on the founder's word. Its second half was one builder window:
  18 survivors, the shared test scaffolding, three functions split and one whole-unit score.
- **This entry:** 47 survivors in 10 files across five packages, plus two file splits (≈1000 production and ≈2600
  test lines moved), plus §2 and §4. Two survivor clusters need a cause read before any test can be written:
  - `PortablePty::resize`: three Linux tests should catch it, and the audit reads it missed;
  - the six `fs.rs` loop mutants: they behave as the original on Linux.
- **The cut:**
  - **here:** §2 the Windows baseline's cause, §3 the send/human split and §4 the Windows-target lint gate, all
    mechanical work with known equalities;
  - **next:** §1 the survivors, as the entry that follows.
- **Why this order:**
  - the survivors' new tests land directly in the split files, never written into `src/run/send.rs` and then moved;
  - the Windows lint guards their `cfg` code;
  - §2's `wipe` edit lands before the survivors' entry reads `scratch.rs`.

## Goal
1. **§2 — the Windows mutation baseline's cause, removed.** Run 38215104120's root shards failed because the
   cargo-mutants scratch (`TMP`/`TEMP`) is re-created by plain `create_dir_all`. On the runner's drive it therefore
   inherits BUILTIN\Users write ACEs (measured at ci#37214281447), and the strict check refuses them as specified.
   `check_instance` is right.
   - **The fix:** `wipe` lays the scratch down owner-only through `viola_state::fs::create_private_dir`.
   - **Why there and not in the test:** the baseline was fail-fast (`425/1224` ran), so the fix covers the 799 root
     tests that never ran, not the one test.
   - **The proof:** a Windows witness test, on CI's ordinary `test (windows-2025)` leg. A tree inside a wiped scratch
     is accepted; the same tree in a plain sibling is refused `Writable`.
   - The diagnosis needed no dispatch, so no fork is raised for one.
2. **§3 — `src/human.rs` (1064 code lines) and `src/run/send.rs` (2088) split by phase**, as a pure move:
   - **human:** root + `mirror`, `escape`, `board`, `wait`, `verbs`, `hints`;
   - **send:** root + `classify`, `slot`, `params`, `budget`, `test_support`;
   - re-exports keep all 18 caller files untouched;
   - every test keeps its name and cases;
   - no file exceeds 800 code lines.
3. **§4 — the Windows-target clippy as a gate entry:**
   `CARGO_TARGET_DIR=target/wincheck cargo clippy … --target x86_64-pc-windows-msvc -- -D warnings`
   (check only, the recorded hand form).

## Top acceptance criteria
- The Windows witness passes on `test (windows-2025)` in the run the CI read reads green for HEAD; the run id is in
  the report.
- `crates/viola-state` is unchanged (no loosening of the check); the scratch guard, no-fallback and no-path rules hold.
- The split-names entry reads no difference against step 1's record (223 test lines at `d9ad9f1f1d96`).
- The sizes entry reads 0 files over 800; orphans, G1, lint probes and the human siblings' print/allow read are
  clean; every existing output test passes with no file under `tests/` edited (the preservation guard + `pre-push`).
- The Windows-target clippy exits 0. `cargo deny` passes, and `Cargo.lock` gains an edge, never a crate.
- No capability is claimed (none of the 37 unclaimed is this chunk's); so there is no concretization preview.

## Touchpoints
- **New:** `src/human/{mirror,escape,board,wait,verbs,hints}.rs` and
  `src/run/send/{classify,slot,params,budget,test_support}.rs`.
- **Modified:** `src/human.rs`, `src/run/send.rs`, `crates/viola-e2e/src/harness/run/mutants/scratch.rs`,
  `crates/viola-e2e/Cargo.toml`, `Cargo.lock`.

## Test commands (24 entries; baselines on the untouched tree)
- **Lint (6):** fmt, host clippy, Windows-target clippy (new, green), G1, lint probes, orphans.
- **Unit (2):** the whole unit run, and the scratch cases (7 selected, green).
- **New probes (5) and their baselines:**
  - split names: red, its record not yet written;
  - sizes: red, the split dirs absent;
  - human print/allow: red, `src/human` absent;
  - preservation guard: green;
  - lockfile read: green.
  - Each was controlled both ways in a scratch clone (`p5-controls.md`).
- **The rest:** `cargo deny`, the harness smoke with G2 and G4, the native `pre-push`, and the operator's hygiene,
  push and CI read.
- No mutation run, witness run, workflow dispatch or live `claude` start (`inputs#I1`).

## Expected amendments (the wrap's)
- **architecture:**
  - the directory structure (the two splits);
  - §Occupied Resources (the scratch laid down owner-only).
- **test-plan:**
  - §3 `run` step 4 (the scratch DACL and its witness);
  - §4 (the moved tests' file citations);
  - §9 Lint row (the Windows-target clippy as a chunk gate).
- **security-plan:** Anti-Patterns › Code Patterns (the scratch re-created owner-only).
- **a11y-plan:** §4 (P6's units now in `src/human/board.rs` and `src/human/escape.rs`).

## Noted, not proposed
- The CARRY on "Paste newline ledger row" quotes `fs.rs:285:20` as "the match guard → `true`"; the Epoch 4 audit's
  survivor list reads `:285:20` → `false` (the Windows run it cites may differ). It stays that entry's to reconcile.

## Beyond the plan
| # | proposal | kind |
|---|---|---|
| 1 | A new working-route entry **"Epoch 4 survivors"**, placed directly after "Epoch 4 cleanup" at the head of `### Epoch 5 — Driver surface`. It carries this entry's CARRY 1 verbatim (the audit's 47 survivors) and research.md §1's survey: the `fs.rs` six reachable only on Windows; `scratch.rs` `prepare`'s two already recorded "not measured here"; `PortablePty::resize`'s miss with its cause unknown; the disposal forms test-plan holds. It inherits `inputs#I1`'s direction: no mutation run, witness run or dispatch; no live `claude` start. | route insertion (the wrap mints it) |
| 2 | Scope §1 of this chunk leaves with the split: recorded in `scope.md` as "leaves this chunk with the split, owed to the entry #1 mints". | deferral of a part, its owner #1 |
