# Leak guard and unviable-swamp rule — 2026-09-24-epoch-1-cleanup (overseer decisions 1 and 2)

## 1. The leak, measured locally (controls in a separate `CARGO_TARGET_DIR=target/leak-probe`)

Stub: `cleanup_cmd` in `crates/viola-e2e/src/bin/viola-harness.rs` returned `Default::default()`, the shape of the CI turn mutant
`viola-harness.rs:157:5`. The test is `cargo nextest run --workspace --features viola/fake-agent -E 'test(=status_logs_and_cleanup_drive_a_booted_session)'`.

| control | reading |
|---|---|
| without guard | nextest: `FAIL + LEAK [0.424s] viola-e2e::cli status_logs_and_cleanup_drive_a_booted_session`. Afterwards, alive under the probe target: `viola-harness.exe supervise --session cli-drive-38172` and its `viola.exe … run builder` (plus the fake agent `target\agent-run\cli-drive-38172\bin\claude.exe`) |
| relink with the leak alive | `cargo build -p viola-e2e --bin viola-harness` → exit 101 · `failed to remove file …\target/leak-probe\debug\viola-harness.exe` · `Access is denied. (os error 5)`. This is the **unviable mechanism**: on the CI windows leg every mutant after `:157:5` failed to build |
| run 1's stall (pipe control) | the same test under a parent that captures stdout through a PIPE (as `run_forwarding`'s `Command::output()` did in run 36046091888): `child exited rc=100 after 1.7s` · `pipe NOT at EOF 61.7s after start: a descendant still holds the write end`. The leaked supervisor inherits the write end, so `output()` never returns. This **explains run 1's 2 h 45 m silence**. The harness has streamed cargo-mutants' stdout to stderr since pass 2, which removes that pipe as well |
| with guard (stub still in) | `child exited rc=100 after 1.8s` · `pipe EOF after 1.8s` · `leak-census: 0 processes under target\leak-probe`, 0 under `target\agent-run\cli-*`. The mutant is still CAUGHT (the test still fails); nothing leaks |

The stub was removed afterwards: `git diff` of `viola-harness.rs` is empty against the pre-CI commit.

Process hygiene: the probe processes were stopped by exact image path (`leak-stop.ps1`: roots `target\leak-probe\` and
`target\agent-run\cli-`). One earlier stop matched command lines containing `cli-drive-38172`. It stopped the probe's fake agent
(`claude.exe` under `target\agent-run\cli-drive-38172\bin\`) and a `bash` (51832): this session's own tool shell, whose command line
carried the string, which ended that call with exit 255. A census by image path afterwards showed every real Claude Code / Claude desktop
process and the operator's viola-lab `viola.exe` untouched.

**Guard** (`crates/viola-e2e/tests/cli.rs`): `booted()` returns a `Booted { ws, session }` whose `Drop` calls the library
`cleanup(&ws, Target::Session(..), false)`. That function lives in `harness/cleanup.rs`, outside this chunk's diff, so it is never mutated.
It runs pass or fail, and it is idempotent after a working binary cleanup.

## 2. The swamp rule, picked by measurement

Every leg with an uploaded verdict (`mutants-verdict-*.json`, names + outcomes in run order; `swamp.py`):

| run (sha) | leg | caught | unviable | missed | longest unviable run |
|---|---|---|---|---|---|
| 36019646063 (3f385dd, chunk 7) | ubuntu | 114 | 3 | 1 | 1 |
| 36019646063 (3f385dd, chunk 7) | windows | 113 | 3 | 2 | 1 |
| 36029350628 / 36032014621 (chunk 8) | both | no-rust-delta, 0 mutants | | | |
| 36046091888 (bf87d71) | ubuntu | 139 | 4 | 0 | 2 |
| 36118112104 (74dadd4) | ubuntu | 139 | 4 | 0 | 2 |
| **36118112104 (74dadd4)** | **windows** | **8** | **135** | 0 | **134** |
| local host run 3 (this chunk) | windows host | 137 | 6 | 0 | 2 |

The healthy legs' unviable/caught ratio is at most 0.044 (6/137). The broken leg's is 16.9 (135/8).

**Rule: red if unviable > caught.** It is the simpler of the two candidates and separates the populations by more than two orders of
magnitude. A streak rule ("≥ N consecutive unviable after a caught") also separates them (healthy max 2, broken 134). It needs run order,
though, which the counts path (`mutants_suite`) does not carry. Known edge: a tiny diff whose only mutants are unviable also reads red. No
measured leg has that shape, and such a leg tested nothing viable.

**Implementation:** `mutants_suite` (`crates/viola-e2e/src/harness/run/mutants.rs`) adds the failure `unviable-exceeds-caught` and one to
`failed` (not `survived`). A `--leg` run therefore stays red (`failed != survived`, so it is not deferred to the union). The unmutated
counts path is unchanged.

**Tests (planted fault):**
- `mutants_suite_is_red_when_unviable_outnumbers_caught`: the measured 8/135 shape is red; 3/3 is green.
- `run_mutants_leg_with_an_unviable_swamp_is_red_not_deferred`: a `--leg` run with 1 caught / 2 unviable exits 1.
- Remove-the-guard run: `let swamped = unviable > caught && false;` → both FAIL (`left: (8, 0, 0, 143)` / `right: (8, 1, 0, 143)`;
  `left: 0` / `right: 1`, the false green reproduced). Restored → 3 passed.

## Local gate block with both fixes (run 4, windows host, base 9df9e45)

- `entries 18 · green 15 · red 0 · recorded 0 · timeout 0 · not-run 3` (the 3 are the operator legs).
- Gate 15: `Found 148 mutants to test` · `148 mutants tested in 22m: 144 caught, 4 unviable`, 0 missed, 0 timeout.
  - The 5 new mutants are the swamp rule's lines. All were caught.
  - The rule does not fire on this healthy leg (4 unviable ≤ 144 caught).
- `metrics.py` size / cognitive / clones: `0` / `0` / `0`.
- After the run, a census found 0 processes whose image lies under `D:\dev\projects\viola\target\` or a cargo-mutants temp tree.

**Expected amendment (wrap reconcile):** test-plan §10 Mutation gate wording. A leg is red when its unviable outcomes exceed its caught
outcomes. The `failures` code `unviable-exceeds-caught` joins the closed codes in test-plan §3 if the failure codes are enumerated there.
