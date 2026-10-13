# Session Handoff

**Last Updated:** 2026-10-13T18:19:13Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-13-epoch-4-survivors-ii — Epoch 4 survivors II: the 16 typed.rs survivors disposed by restatement and named cases

## Position
- Done: 2026-10-13-epoch-4-survivors-ii. The 16 `typed.rs` survivors of the Epoch 4 audit are in
  `evidence/survivors.ndjson`, each restated in `src/cmd/verify/typed.rs`:
  - the framing waits became named predicates;
  - every probe deadline goes through `probe_until`;
  - `SharedPty` is generic over its `Pty`;
  - the Ctrl-C ending and the pump wait are the plain functions `ctrl_c_ending` and `ended_by`.

  Each is killed by a case in `typed.rs`'s `mod tests`, shown red under its hand-applied mutant and green restored:
  14 applications for 16 rows, and no mutation run. CI green on `94f0be9` (ci#38442894573).
- Next: **Interrupted verify cleanup** (the first markerless entry) → `/andromeda-phase`.

## Work done
- The chunk: implement run `.andromeda/runs/2026-10-13T17-47-22-implement/`, wrap run
  `.andromeda/runs/2026-10-13T18-08-14-wrap/`.

## Drift resolved
- 1 amendment, routine, 0 escalations: test-plan §10 Mutation gate. The 16 are disposed, none is owed, and the Epoch 5
  audit confirms 45 kills.
- 2 test-plan proposals were rejected: the §4 and §8 mockall wording bounds where mockall is used and mandates
  nothing.
- 1 leaf finding applied (`tests-summary.md:42`).
- `fanout-results.md` and `cascade-dispositions.md` hold the record.

## Notes
- **Epoch growth:** Epoch 5 holds 11 entries (8 markerless). The founder ruled no boundary inside Epoch 5; a split
  stays the operator's word.
- **For whoever dispatches `windows-mutants.yml` next (the Epoch 5 boundary audit):** the root package's green
  baseline is still not measured. Forecasts, not readings:
  - the audit's Linux runs should read 29 fewer missed over the earlier chunk's files, and 16 fewer over
    `typed.rs` (16 → 0);
  - the new functions' own mutants are predicted caught.

  The `resize` miss's two-job cause is a HYPOTHESIS: the report files now carry the pid.
- **Curation:** one correction in place, `verification-harness.md` 2026-09-27. The kill filter is now
  `test(/(^|::)tests::name(::|$)/)`, because an rstest table's rows are `name::case_<n>_<label>` (the operator's
  word, this chunk's `inputs#I2`). Log: the wrap run dir's `curation.md`.
- **For the founder, his next batch (the operator's word, none ruled by him), carried:**
  - a refused strict-modes check writes no line on `revive`'s start arm;
  - the checked read of `budget.json` writes no line on a refusal;
  - `list`'s exit-1 shapes and per-instance no-row;
  - the five implicit-link shapes;
  - a link line with no string `ts` → `since: "unknown"`;
  - a `--home` session shows no user status line until `statusline-source.json` is planted;
  - three relied-on dependences with no ledger row (owed on "Paste newline ledger row");
  - `viola revive --list` opens no process log;
  - "Paste newline ledger row" holds six shapes and the `claude agents --json` join-field row;
  - the switch from the prototype not made, `v1-33` unclaimed;
  - one CLI plan file (2026-10-08) in his user directory.
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s harness-prefix row and its R8
  identity-floor row.
- **For the operator's word (carried):**
  - the link rule takes a logged `driver` / `driven` as a string (CARRY on "Explicit session links", Epoch 6);
  - `docs/conventions.md` and `docs/stack.md` still list `viola link|unlink`;
  - "Interrupted verify cleanup" has no `requirements.md` line and no ledger entry;
  - `v1-33`'s title and `requirements.md:48` say "on Windows";
  - the ledger notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90`;
  - seven sentences of the budget-governor plan word the old hold;
  - a live `claude` session ending in this tree rewrites this file's last section.
- **Route:** `BLOCKED-ON` on "Windows-only live measurements" stands (`uname -s` read Linux). Epoch 5 has 11 entries
  (8 markerless after this flip), Epoch 6 has 7.
- **Setup:** a setup re-run must keep `.claude/rules/ci.md` and `.claude/rules/testing-src.md`; the host leaf is
  `host-linux.md`.
- **Standing rules (carried):**
  - A removal the permission layer denies is not done another way: move it into `target/e2e-home.disk/`.
  - A stalled-start red is a finding about the backing: read `test -L target/e2e-home && findmnt -n -o FSTYPE -T
    target/e2e-home/`, then stop and report.
  - Read `hyprctl locked` before any step that focuses a window or types a key.
  - Before a census run (`bash scripts/profraw-census.sh 4800 48`), write the operator one line.
  - After a `cargo clean`, re-make `ln -s /tmp/viola-e2e-home-<uid> target/e2e-home`; after a reboot the link dangles
    until a harness call re-makes its target.
  - The Bash guard refuses a pipeline tool piped into `tail` / `head` / `grep` / `cut` / `sed` / `awk`: redirect it to
    a file and read the file whole.
  - A scripted `.rs` write runs rustfmt in the same call.
- **Deferred learnings:**
  - `recurrence-despite-learning: a time written into a record ahead of the clock` did not recur this chunk. Every
    record time was read from `date -u` inside the writing process.
  - carried:
    - `docs/session-learnings.md` "gate.py hygiene reads `/home/<x>/` in prose…" (a quoted Windows drive path);
    - `~/.claude/CLAUDE.md` Shell (a bounded repetition sent to the embedded grep);
    - `host-linux.md` Transports (one inline python heredoc carrying document text).
  - below the threshold:
    - the gate trail's `ts` dates a journal line;
    - `gh run view <run> --log-failed` reads the failed steps in one call;
    - the stamp-ahead hook reads a test's synthetic instants dated today;
    - `create_dir_all` over a dangling link returns `AlreadyExists`;
    - the older list is in git history of this file.
- **Operator desk (the founder's word: leave them):**
  - the earlier chunks' scratch files;
  - gate logs under the OS temp dir's `andromeda-gate/`;
  - `target/wincheck/`, `target/run-archive/`, `target/agent-run/artifacts/`;
  - live homes and rig directories;
  - `mutants.out*/`, `target/witness-wmg/`, `crates/viola-e2e/.viola-verify-227786-plan/`;
  - `~/.viola-record-20261004T142325Z`;
  - the CLI's own transcripts;
  - the earlier chunk's kept mutant-red reports under the OS temp dir's `viola-pty-watch/`.

  Yours to delete: `target/e2e-home.disk` and `target/profraw-census/`.
- **Last failed command:** none.
