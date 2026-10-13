# Implement outcome — 2026-10-13-paste-newline-ledger-row

**Outcome: surfaced.** Every functional gate is green and the live round passed. Two of the plan's own gate
entries read red on the correct tree because of how they are worded (12, the literal probe; 13, the preservation
guard). `plan.md` is immutable here, so they are surfaced for the operator's word before the operator pass and the
wrap.

## Gates (run `.andromeda/runs/2026-10-13T21-22-15-implement/`)
- Green: 1 fmt · 2 clippy · 3 G1 · 4 lint probes · 5 unit (2039 tests at the first firing) · 6 the three
  `check_paste_newline_` cases (`passed 3, failed 0`) · 10 the fixture guard · 11 integration (94 passed in 34.7 s;
  `cli_verify`, `contract_ledger_probes`, `contract_fake_agent_drift`, `contract_fixture_hygiene`,
  `cli_version_gate`) · 14 `cargo deny check` · 15–20 the harness smoke with G2 and G4 · 21 pre-push (`ok:true`,
  coverage 2475 passed 0 failed, playwright 1, gate no breaches).
- Live round (7–9), fired once: `round: COMPLETE · legs fired 1/1`, `stamped 2.1.287  18 pass  0 fail`. The
  recorded `paste-3` prompt equals `PROBE_NEWLINE_PASTE` byte for byte, and the census is clean
  (`evidence/live-round.md`, `evidence/round-213213Z.txt`). The cap is spent, 5 of 5 (`evidence/live-starts.md`).
- **Red — surfaced:**
  - **12, the literal probe** (`last line 0`) reads `1`. Its one hit is `tests/cli_verify.rs:1093`,
    `"stamped 2.1.0  17 pass  1 fail"`: the correct eighteen-row oracle of
    `verify_without_framing_fails_only_the_clear_row` (17 pass + 1 fail). The probe's alternative `17 pass` matches
    it, so this is a false positive. Every stale seventeen-row literal is gone. Rewriting the oracle to dodge the
    token would break testing.md's literal-oracle rule.
  - **13, the preservation guard**, reads exit 1. `git diff --name-only` outside its exclusions lists exactly the
    two companion files recorded in `scope-record.md`: `tests/contract_fixture_hygiene.rs` and
    `tests/support/home.rs`. The guard excludes only research's lists. These two edits are required for 11 and 21
    to pass, and `gate.py scope` reads `clean`.
- No deferral: every gate ran. The integration durations were read as the plan's note asked: 34.69 s for the 94
  tests, against 35.4 s at P5 with seventeen rows; no kill line moved.

## Files changed
- Listed: `crates/viola-agent-claude/src/ledger.rs`, `src/cmd/verify/typed.rs`, `src/cmd/verify.rs`,
  `src/bin/viola-fake-agent.rs`, `src/run/version_gate.rs`, `tests/cli_verify.rs`,
  `tests/contract_ledger_probes.rs`, `tests/support/verify.rs`, `crates/viola-e2e/src/harness/run.rs`.
- New: `fixtures/claude/2.1.287/UserPromptSubmit.paste-3.json` (recorded live, copied alone),
  `evidence/live-starts.md`, `evidence/live-round.md`, `evidence/round-213213Z.txt`, `evidence/carry2-kills.md`.
- Recorded (`scope-record.md`):
  - `` `tests/contract_fixture_hygiene.rs` · companion · serves `fixtures/claude/2.1.287/UserPromptSubmit.paste-3.json` · self ``
  - `` `tests/support/home.rs` · companion · serves `crates/viola-agent-claude/src/ledger.rs` · self ``

## Deviations from the plan, with justifications
- **Five shape-pinned sites research's sweep did not list** were fixed in place. They are three `ledger.rs` cases
  (`merge_stamp_writes_the_envelope_for_a_first_version`'s rows object, `check_typed_rows_fail_without_a_measurement`'s
  verdict vector, `verified_reads_a_fourteen_row_stamp_as_unverified`) and the two companions above
  (`dialogless_home` now reads `14 pass  4 fail`; the framing-set list now holds five). Each pinned the row set by
  shape, not by the swept literals.
- **Count moves beyond step 5's list**, each one forced by the fifth paste: in `cli_verify.rs` the identity-strip
  hook count 35 → 37, the Stop count 8 → 9, the dialog-less summary `14 pass  4 fail`, the screen-less summary
  `6 pass  12 fail`. The tag-turn-modal case now reads `16 pass  2 fail` with `paste-newline` failing, since the
  newline text sits behind the same input-box guard as `/clear`. `verify_record_writes_the_scrubbed_spine_variants_and_screens`
  now lists `paste-3` and asserts its prompt. The no-framing case's oracle reads `17 pass  1 fail`; that is the
  literal behind entry 12.
- **Step 2's unit cases** extend `clean_trusted` with the newline turn, so the clear-row and framing-variant
  cases' indices moved by two. `FRAMING_ROWS` gained `PasteNewline`, and `framing_variants_name_…` was renamed
  `…_the_three_prompts_…`.
- **Step 6, the `ProbeDir::typed` case**, builds `ProbeDir` by struct literal. Under the mutant, `create` would
  resolve `plugin/` against the test's cwd (the repository root) and could overwrite the committed
  `plugin/hooks/hooks.json`. Each mutant run selected only its killing case; `git status -- plugin` read empty
  before and after.
- **Step 6, the `replay_dialogs` mutants**: a third case pins the loop's end
  (`replay_dialogs_replays_nothing_for_a_stem_with_no_call`).
- **The live-start ledger** (`evidence/live-starts.md`, the operator's instruction at this run's invocation): the
  five starts are made by one `viola verify` process, so each carries the one `date -u` time read just before the
  firing.

## CARRY 2 (step 6)
All four hand-applied mutants read red on an assertion under the nextest `mutants` profile and green once
restored. The restore was byte-equal each time (`evidence/carry2-kills.md`): c1 and c2 by two new
`replay_dialogs_…` cases, c3 by the existing `opts_parse_reads_values_flags_and_ignores_unknown`, c4 by the new
`probe_dir_names_each_run_its_own_dir_inside_it`. No mutation run, witness run or workflow dispatch was made.

## Smoke (P3)
Smoke ✓, by hand: session `p3-pnl-smoke`. Boot exited 0 (its stamp runs verify against the fake agent at
eighteen rows), status read `ready`, and cleanup read `processes_gone` and `endpoint_gone` true with the home
removed. The plan's smoke entries 15–20 had also read green as P2 gates.

## Verification matrix
The chunk claims no cap (`matrix.py show --chunk` reads 0; `v1-34` is not claimed, per plan), so no ref was
written.

## Process hygiene

| process | started by | final state |
|---|---|---|
| `claude` 2.1.287 ×5 (print, Runs A–D) | a leg (plan entry 8) | terminated. The round's own termination read no survivors; census entry 9 read empty; hand re-read: no `claude` with a probe-dir cwd |
| `viola verify --record` | a leg (plan entry 8) | terminated (exit 0) |
| harness supervisor, wrapper and fake agent, session `p-pnl-smoke` | this run (entries 16, 20) | terminated: cleanup `processes_gone:true` |
| harness supervisor, wrapper and fake agent, session `p3-pnl-smoke` | this run (P3 by hand) | terminated: cleanup `processes_gone:true` |
| `viola` pids 12692, 13416, 15370, 16532, 627411, 1552695, 1585891 | not this run: `/proc/<pid>/exe` is another tree's build (`viola-lab/prototype`) | left running: not this run's (host-linux.md 2026-10-07) |

The round's home `target/e2e-home/viola-record-20261013T213213Z/` stands on the tmpfs backing.

## Next
Operator decision on entries 12 and 13 (see the report). Then the operator pass: entry 22 (hygiene) before the
pre-CI commit, 23 (push), 24 (the CI read). Then `/andromeda-wrap-session`.

## Decisions & corrections
- The operator's instruction at invocation: the founder's cap of exactly five live starts, one `--record` round on
  2.1.287, fired once, never retried; each start ledgered with its `date -u` time before it is made; a red round
  stops and reports; no mutation run, witness run or workflow dispatch; pipeline output to a file and read whole;
  the operator pass with the `ci.py` read. Kept: one firing, `evidence/live-starts.md` written at 21:32:02Z
  before the 21:32:13Z start.
- Standing rule (`testing.md` 2026-09-25, re-read the site before a neutralising edit): kept. Each mutant was
  applied by exact replacement with its site count checked (1) and the restore checked byte-equal.
- Standing rule (`host-linux.md`, a scripted `.rs` write runs rustfmt in the same call): kept, for the
  `count18.py` replacement and each mutant application.
- Standing rule (`host-linux.md` 2026-10-07, whose `viola` a process is): the census read `/proc/<pid>/exe`;
  seven `viola` processes are another tree's.
- One Bash guard refusal: a `cd` into the chunk's `inputs/` dir, refused because it moves the session cwd. It was
  re-issued once with absolute paths and ran.
