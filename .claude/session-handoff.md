# Session Handoff

**Last Updated:** 2026-10-13T20:26:00Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-13-interrupted-verify-cleanup — Interrupted verify cleanup: viola verify stopped by Ctrl-C or TERM removes only what that run made

## Position
- Done: 2026-10-13-interrupted-verify-cleanup. `viola verify` stopped by Ctrl-C or TERM now ends cleanly:
  - Unix `SIGINT` / `SIGTERM`; Windows Ctrl-C / Ctrl-Break, through `src/cmd/verify/stop.rs`.
  - It kills its child with no key and removes only the dirs that run made.
  - It prints `unable: verify was stopped before it finished` / `hint: run viola verify again to stamp this CLI`,
    writes no stamp, and exits 1 with `detail:"interrupted"` (12 `process-exit.detail` members).
  - Verify now reads `PROGRAM` and `SETTINGS_FLAG`.
  - CI green on `c22161e791ff` (ci#38453057445, attempt 2).
- Next: **Paste newline ledger row** (the first markerless entry) → `/andromeda-phase`.

## Work done
- The chunk: implement run `.andromeda/runs/2026-10-13T18-48-52-implement/`, wrap run
  `.andromeda/runs/2026-10-13T20-00-28-wrap/`.

## Drift resolved
- 27 amendments over all seven masters and three architecture key files, 0 escalations.
- 16 were proposed by the detectors; 11 were raised for expected amendments the detectors' invariants do not reach
  (security-plan, design-system and a11y-plan read 0).
- 17 leaf findings applied, 2 declined as details. `fanout-results.md` and `cascade-dispositions.md` hold the record.

## Notes
- **Curation:**
  - two Tier-2 entries in `ci.md`: write nothing into the tree between the pre-CI commit and the push; a CI job
    that was never acquired;
  - one extension of `testing.md`'s 2026-10-05 status-line entry (the binary/test space);
  - two in-place corrections, nine → ten `mutants (…)` jobs, in `ci.md` and `verification-harness.md`.

  Log: the wrap run dir's `curation.md`.
- **Plan defect, surfaced (the operator's direction: recorded as a deviation, the guard not edited).** The plan's
  preservation guard reads red on `.github/workflows/windows-mutants.yml` alone.
  - `tests/contract_windows_mutation_scope.rs` (inside `pre-push`) requires every product file with a
    `cfg(windows)` gate to have a matrix line.
  - The operator approved the one line (`viola-cmd-verify-stop`); nothing was dispatched.
  - Pipeline owners: a plan guard over a chunk that adds a Windows-gated file must exclude that workflow (the
    fix-loop's `contract.jointly-contradictory-instructions` record).
- **For whoever dispatches `windows-mutants.yml` next (the Epoch 5 boundary audit):**
  - The matrix now holds ten items, the root package five (`viola-cmd-verify-stop`, its first grading).
  - The root package's green baseline is still not measured.
  - Forecasts, not readings: Linux runs read 29 fewer missed over the earlier chunk's files and 16 fewer over
    `typed.rs`.
  - The `resize` miss's two-job cause is a HYPOTHESIS.
- **Epoch growth:** Epoch 5 holds 11 entries (7 markerless after this flip). The founder ruled no boundary inside
  Epoch 5; a split stays the operator's word.
- **For the founder, his next batch (the operator's word, none ruled by him):**
  - the interrupted pair's wording (a planning form, `inputs#I2`);
  - the Windows event set (Ctrl-C and Ctrl-Break only; close, logoff and shutdown keep the default);
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
  - `v1-33`'s title and `requirements.md:48` say "on Windows";
  - the ledger notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90`;
  - seven sentences of the budget-governor plan word the old hold;
  - a live `claude` session ending in this tree rewrites this file's last section.
- **Route:**
  - A CARRY on "Windows-only live measurements" now owns real Windows console delivery to a running verify (the
    P5 card's row, `inputs#I3`).
  - That entry's `BLOCKED-ON` stands (`uname -s` read Linux).
  - The "Paste newline ledger row" CARRY keeps its `src/cmd/verify.rs:284:9` citation, dated as of `632f6a7`; the
    function `ProbeDir::typed` now sits at `:319`.
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
  - `recurrence-despite-learning: a time written into a record ahead of the clock` did not recur this chunk; every
    record time came from `date -u` inside the writing process.
  - carried:
    - `docs/session-learnings.md` "gate.py hygiene reads `/home/<x>/` in prose…" (a quoted Windows drive path);
    - `~/.claude/CLAUDE.md` Shell (a bounded repetition sent to the embedded grep);
    - `host-linux.md` Transports (one inline python heredoc carrying document text).
  - below the threshold:
    - a new `cfg(windows)` product file owes its `windows-mutants.yml` line (exactly 0.6; architecture's CI/CD key
      now states it);
    - the gate trail's `ts` dates a journal line;
    - `gh run view <run> --log-failed` reads the failed steps in one call;
    - the stamp-ahead hook reads a test's synthetic instants dated today;
    - `create_dir_all` over a dangling link returns `AlreadyExists`.
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
