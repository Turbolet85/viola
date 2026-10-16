# P5 review card 1 — 2026-10-14-resume-and-compact-ledger-rows · "Resume and compact ledger rows"

Resumed P5 (Setup 3 offer (a): `plan.md` present, no `matrix-2026-10-14-resume-and-compact-ledger-rows.json`), in the
take-up's run dir `.andromeda/runs/2026-10-14T07-11-44-phase/`. Plan:
`viola-0.1.0/chunks/2026-10-14-resume-and-compact-ledger-rows/plan.md`.

## Goal
- **`resume-session`, ledger row 22.** It covers `claude --resume <id>` reopening the session under that id, a SessionStart
  of source `resume` with the same id, which `viola revive` relies on. The row is relied on every OS, so the relied set
  moves 21 → 22 on Unix and 18 → 19 on Windows, and every home stamped before it reads `cli_verified:false` until it is
  re-stamped.
- **Run E, a new sixth interactive start.** After Run D it resumes Run B's first session id, read from Run B's own
  first SessionStart capture. The id passes `is_session_id`, is built by `resume_args`, and is one argv element of a
  direct spawn under the R8 strip. Run E runs in a 0700 `-resume` dir, is killed with no key, and writes one more
  `verify-pty-probe` spawn pair (seven in all).
- **`/compact` on the local-command list, with no post-condition.** A `send` of it reads `ok` `unconfirmable` at once
  instead of a false exit 13. No row, no probe, no live compaction and no new bound. Its post-condition row stays owed.
- **The fake agent's `--resume` builds on the recorded `SessionStart.resume-1.json`**, with the given id written into
  it. The 2.1.288 set, which has no variant, keeps the default-based payload.
- **The forks: the founder's own answers**, relayed verbatim by the operator (`inputs#I1`): «Resume; /compact в список»
  · «Run E, шестой старт» (a boundary widening, ratified by him) · «Двенадцать» (cap: 12 live starts).

## Top acceptance criteria
1. `LedgerRow::ALL` holds 22 rows, the last `resume-session`. `viola verify` prints `MM` = 22 on Unix and 19 on Windows.
   An all-`pass` older stamp (21 rows on Unix, 18 on Windows) reads unverified.
2. The `resume-session` check passes only on Run E's first SessionStart of source `resume` whose id equals Run B's
   first id. It fails on another source, another id, no capture, or a target that isn't session-id shaped. The five
   cases are named by the filter entry, `passed 5`.
3. The Run E id reaches the child's argv only through `resume_args` after `is_session_id`. Its `-resume` dir is removed
   on pass, fail and stop. The integration entry proves this through the stop table and the spawn-pair cases.
4. `SessionStart.resume-1.json` is recorded live, passes the hygiene walk and the schema, and is replayed byte for byte
   by `contract_fake_agent_drift`.
5. A `viola send` of `/compact` on a verified CLI exits 0 with `[  ] unconfirmable` and logs `send-confirmed{confirmed:false}`.
6. The two live rounds (record round, then `run --local-live`) stay within 12 starts. `pre-push` is green, and
   `ci.py conclusion` reads `verdict: green` on the final HEAD (Windows leg at 19).

## Touchpoints
- Modify: `crates/viola-agent-claude/src/ledger.rs` · `src/run/send/classify.rs` (unit rows only) ·
  `src/cmd/verify/typed.rs` · `src/cmd/verify.rs` · `src/bin/viola-fake-agent.rs` · `src/run/version_gate.rs` ·
  `crates/viola-e2e/src/harness/run.rs` · `tests/cli_verify.rs` · `tests/contract_ledger_probes.rs` ·
  `tests/contract_fixture_hygiene.rs` · `tests/contract_fake_agent_drift.rs` · `tests/cli_fake_agent.rs` ·
  `tests/cli_send.rs` · `tests/support/verify.rs` (only if needed).
- New: `fixtures/claude/2.1.287/SessionStart.resume-1.json` (recorded) · the chunk's `evidence/live-round.md`.
- Untouched, guarded: `src/run/send/slot.rs`, `crates/viola-agent-claude/src/screen.rs` (no claim arm, no window), no
  manifest or lockfile change.

## Test commands (26 entries)
- Lint ×4 · `run --unit` · the five `check_resume_session_` cases by name.
- The live round, fired once by `--live-legs`, never retried:
  - probe → the record round (6 starts) → census;
  - probe → `run --local-live` (6 starts) → census.
- Fixture guard · integration (9 binaries) · preservation guard · `cargo deny` · boot/status/cleanup smoke · G2 · G4 ·
  `pre-push`.
- Operator entries: hygiene, push, `ci.py conclusion --wait 1800`.

## What this P5 measured and changed
- **Mechanical checks** (run twice; the second run after the edits below): every check passes.
  - The fence parses (26 entries). `planlint` 0 hits. Its one INDETERMINATE, on entry 15, is resolved by reading: the
    new-file glob lies under the `:!fixtures/claude/2.1.287` exclusion.
  - The scope lists parse (18 items). The snapshot citations are clean (`inputs#I1` cited 10 times).
  - Modify paths exist, and no spec master is in the modify-set. `ledger` is `pub mod`, so it's reachable by path.
- **Baselines**: nine `new` non-leg entries, each run on the untouched tree, `baseline` written into each.
  - Green: 14 (integration, 39.31 s), 15, 16, 17, 18, 19, 22.
  - Red, as predicted on input that doesn't exist yet: 6 (`no tests to run`) and 13 (no addition yet).
- **Controls**, fired as the entry over a scratch clone at `880b056377f4` (`p5-controls.md`):
  - fixture guard: the variant alone green; plus a modified fixture red; plus a second addition red;
  - preservation guard: allowed edits green; `slot.rs` edited red; an untracked `src` file red;
  - entry 6's literal is spelt from a real `run --unit` output.
- **Plan folds**:
  1. **Step 5**: `read_captures` is private, but `typed` is `verify`'s child module, so the plan's `pub(super)` change
     was dropped. Its visibility doesn't change.
  2. **Step 12**: the before-reading is this baseline (39.31 s). The after-reading is taken only after step 13's record
     round, because the integration entry reads the fixture that round lands (check 12).
  3. **Entries 8 and 11's `stop` forms** lose the `/tmp/viola-verify-<suffix>` arm. That dir is shared by every `viola verify`
     on the host, another tree's bridge included, so it isn't narrowed to this round (check 11 (a)). A Run A survivor
     is ended by the round's own process-id census.
  4. **A new implementation note**: Run E resumes from a different project dir than Run B's. In the 2.1.287 CLI's own
     code, `--resume <id>` falls back to scanning every project dir for `<id>.jsonl` and takes a single match. That's a
     code reading, not measured live. The record round is its first live reading, and a red there stops the round with
     no retry.
- **Validation-1: intent-incomplete, `scope.md` amended.** Scope still forecast two rows (23/20). A new section,
  "The cut taken at P4", records the fork answers and the 22/19 count.
- **Matrix**: this chunk claims no capability. The entry's only id, `v1-34`, can't be concretized here: it also needs
  the harness-prefix and R8 identity-floor rows, owed with no entry (the founder, 2026-10-07T09:43Z). After the word,
  step 4 writes the dated "not claimed" note.

## Beyond the plan
| # | proposal | owner / place |
|---|---|---|
| 1 | The `/compact` post-condition row (cause `compact`, same id, a confirm window of its own) stays owed. No entry is minted by this phase. | Its owner is named by the operator at this chunk's wrap (fork 1, `inputs#I1`). |
| 2 | The Windows reading of `resume-session` (a Windows stamp at 19 relied rows) is not measured by this chunk. | The existing entry "Windows-only live measurements" (its `BLOCKED-ON` standing), as a CARRY at this chunk's wrap. |

Apply? (yes / review / cancel)
