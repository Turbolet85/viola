# P5 review — 2026-11-17-trust-record-ledger-row «Trust-record ledger row»

The plan to open in an editor: `.andromeda/cache/docs/2026-11-17-trust-record-ledger-row/plan.md` (rev 5). No plan
stands in a chunk's folder; writing in that file changes no record until a `put`;
`python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-17-trust-record-ledger-row`
is a diff of the chunk's records.

## Goal
The 23rd relied ledger row `trust-record` (the CLI honours a trust record another process wrote), probed by a
sixth `viola verify` run (Run F, last, a Run A twin over a fresh OS-temp dir with its record written first, removed
on every exit path the process reaches). The writer is built once: `crates/viola-agent-claude/src/trust.rs` (pure:
the file the child reads, the CLI's key, the record's form, the closed words) and `src/trust_record.rs` (root bin:
the file checks, the CLI's lock when free, `replace_private` with the file's own mode, the read-back, at most 3
attempts), which «Session folder trusted by viola» calls next from `run` / `revive`. The fake agent reads the record
from a config file named on its argv (`--trust-config`) under its own key rule; every test-started verify runs under
a test-owned `HOME`. One live verify S0 on the rig's new `V` arm. Nothing of `run` / `revive` changes; no capability
is claimed.

## Top acceptance criteria
- `[23/23] trust-record …  pass`, `stamped 2.1.287  23 pass  0 fail` on the fake agent.
- The probe can fail: the record not read → Run F on the trust form, killed with no key, `22 pass  1 fail`.
- A file the writer cannot use writes nothing; the stamp's row reads its closed word (`config-absent` end to end,
  the other words at the unit tier), never `pass`.
- verify removes only its own record (pass, fail, SIGTERM); the sentinel entry value-equal; the file's mode kept;
  the kept `CLAUDE_CONFIG_DIR` file chosen over `HOME`'s.
- Every test-started verify under a test-owned `HOME` (a new census case); the whole integration and E2E layers green.
- The three controls `trust-nowrite` · `trust-noremove` · `trust-cfgdir` read `holds` (`--timeout 900` each).
- S0: once, by path, ledgered first, `keys: none`, `stop:` the standing one, `trust-record:` recorded as read,
  `record-left: absent`, the real config read only by the mark witness.

## Touchpoints
New: `crates/viola-agent-claude/src/trust.rs`, `src/trust_record.rs`, `tests/contract_verify_home.rs`.
Modify (27): the ledger and its test modules, `src/main.rs`, `src/cmd/verify.rs` and `typed.rs` / `typed/runs.rs`,
`src/run/version_gate.rs` (case literals only), the fake agent and its option cases, `tests/support/verify.rs` /
`home.rs`, `tests/cli_verify.rs` and its two modules, `tests/contract_ledger_probes.rs`,
`tests/contract_bounded_readers.rs`, `tests/channel_error_bodies.rs`, `tests/cli_version_gate.rs`, the harness boot
and run, the rig (`mod.rs`, `isolate.rs`, the binary's usage, `rig_rehearsal`).

## Test commands (21 entries)
fmt · clippy · `cargo nextest run -p viola-agent-claude` · `run --unit` · `run --integration` filtered to the changed
binaries · `run --integration` · `run --e2e` · the retired-count probe · release-check · the S0 round (2.1.325 by
path, census, build hash, `live S0`, census) · the operator pass (boot / status / cleanup `trust-smoke`, pre-push,
hygiene, push, `ci.py`). Baselines on the untouched tree: 3 green · 4 green (2651) · 5 red (the filter names the census
binary this chunk creates) · 6 green (518) · 7 green (317) · 8 red (58 hits in 6 files, the counts this chunk moves)
· 9 green.

## Leaned decisions (technical, decided at P4)
Run F last · the step line keeps `pass|fail`, the writer's word stands in the stamp · a held lock writes nothing that
attempt (100 ms, at most 3) · the whole `projects[<own temp dir>]` entry removed · the fake agent learns its config
file by argv, never env · tests isolate by `HOME`, not `claude_env_keep`. No founder fork: every widening the plan
carries is R2 / R4 as worded (`inputs#I1`), riding the wrap's answer card.

## Size (directive item 7)
Larger than the cancelled block's estimate. P3 added two pieces: every test-started verify under a test-owned
`HOME` (a dev-host safety need: without it, each local stamp would write into the user's real `~/.claude.json`), and
a rig `V` arm (the rig had none). My judgment: about one implement window, at its edge. Not measured by a tool. I did
not stop at P3.

## Stale stamps (directive item 5)
Once landed, every 22-id stamp reads unverified to the new binary: the live home's 2.1.296 · .302 · .312 · .319 ·
.323 · .325 stamps, and every rig and scratch home. The overseer's pinned product binary is not rebuilt by this chunk.
The live home's re-stamp stays owed on «The 2.1.325 recorded fixture set». The tests' shared stamp is keyed by the
built binaries' hashes and is re-taken at 23 rows; the integration, E2E and CI entries prove it.

## The P2 / P5 block (directive item 13), verbatim where a tool printed it
- Union, per master (`rec.py rows-union`):
  - `architecture: rows 948 · packs 10 · picked 146 (92 by every draw) · standing 43 · 45024 B`
  - `security-plan: rows 608 · packs 7 · picked 113 (71 by every draw) · standing 17 · 31489 B`
  - `design-system: rows 315 · packs 4 · picked 60 (35 by every draw) · standing 11 · 14920 B`
  - `layout-templates: rows 222 · packs 3 · picked 28 (18 by every draw) · standing 2 · 6614 B`
  - `test-plan: rows 1338 · packs 14 · picked 220 (126 by every draw) · standing 54 · 55747 B`
  - `obs-plan: rows 1024 · packs 11 · picked 248 (139 by every draw) · standing 35 · 57228 B`
  - `a11y-plan: rows 235 · packs 3 · picked 70 (41 by every draw) · standing 6 · 19563 B`
  - `rows-union: 7 master(s) · picked 885 of 4690 · rule any`
- Rows read WHOLE: the union's sum is 230 585 B, under 250 000 B. Every rows file, standing list and history file was
  read once.
- Cited rows: every one is in the union or its standing list. A script check found 28 `({doc} · {handle})` citations
  and 34 bare handles, 0 missing.
- `[RETYPED]` lines (pick-p1's check):
  - `arch-pick-p2.d2 · [RETYPED] line 13: decision/socket-dir-check-verdicts-never-chmods is no row of the pack — read as decision/socket-dir-check-verdicts, the pack's row 13`
  - `arch-pick-p4.d2 · [RETYPED] line 80: decision/fake-agent-test-seam is no row of the pack — read as decision/fake-agent-test-seams, the pack's row 80`
- Re-spawns: none. Dead draws: 0. Every batch read `VERDICT: n of n pass`: task 7/7, pick 75/75 + 75/75 + 6/6,
  history 7/7.
- Non-zero exits and refusals:
  - The Bash guard refused three `cd` calls (into the skill references dir and into the run dir), re-issued as Read
    or absolute paths. It refused one pipe of `rec.py` output into `grep`, re-issued bare.
  - Two `git grep … -A N` calls exited 128 (`-A` read as a revision), re-issued as `sed` / Read.
  - A recursive `grep` over the whole tree outran the 120 s Bash limit and moved to the background. I stopped it
    (TaskStop) and re-ran the search as `git grep`.
  - The stamp-ahead hook refused the plan's estimated `Generated` stamp (20:05:00Z against a 20:00:15Z clock); it was
    corrected from `date -u`. This recurs despite the CLAUDE.md learning, and is recorded as friction.
  - Baseline reds on entries 5 and 8 are this chunk's own subject, as above.
  - planlint read 3 hits once (three leg notes lacked `atom from`), fixed. It has read 0 since.
- `UNPARSED` / `INDETERMINATE` / `NOT DERIVED`: none printed by any tool this run.
- Where a letter was wrong or silent:
  - fanout-contract §The batch, step 2: pick-p2's before snapshot was not taken. I passed p1's after line as its
    before (nothing was written between them; p1's check wrote no trail), and p2's check read PASS. This is a
    deviation from the letter, recorded in the distill evolve record.
  - validation check 4 (2): the test plan declares no boot-smoke path list (`rec.py find` for
    `boot.smoke|smoke trigger|boot-path`: 0 hits). I read the harness boot as a boot-path by shape and added the
    operator smoke trio.
  - inputs-contract §snap: the directive arrived as a file in a sibling directory, not as a message. I snapped it with
    `--source --kind relay` (`I1`, a copy, no repository); the letter names no form for that case.
  - phase SKILL P2 step 5: the letter says a master with no sidecar gets an `@none` history file from the
    orchestrator. All seven held a sidecar and a picked row, so none was written.

## Matrix
No capability claimed (directive item 1). The pool's `v2-33` stays with «Session folder trusted by viola».

## Beyond the plan
none

Apply? (yes / review / cancel)
