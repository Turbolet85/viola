# Session Handoff

**Last Updated:** 2026-10-13T22:15:01Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-13-paste-newline-ledger-row — Paste newline ledger row: `paste-newline`, the eighteenth capability-ledger row, stamped live on 2.1.287

## Position
- Done: 2026-10-13-paste-newline-ledger-row.
  - `LedgerRow::ALL` holds eighteen rows; the last is `paste-newline` (an LF inside a paste stays inside the one
    prompt submitted).
  - It is probed by Run B's fifth compiled paste, `PROBE_NEWLINE_PASTE`, and recorded as `paste-3`.
  - One live round on 2.1.287 (five starts, fired once) printed `stamped 2.1.287  18 pass  0 fail`.
  - CARRY 2's four timeout-graded mutants are killed by an assertion (hand-applied).
  - CI green on `16b5f5f4006b` (ci#38472492871).
- Next: **Owed ledger rows** (minted this wrap, the first markerless entry) → `/andromeda-phase`.

## Work done
- The chunk: implement run `.andromeda/runs/2026-10-13T21-22-15-implement/`, wrap run
  `.andromeda/runs/2026-10-13T21-52-39-wrap/`.

## Drift resolved
- 38 amendments over five masters (architecture, security-plan, test-plan, design-system, layout-templates) and four
  key files.
  - 32 were proposed by the detectors and 7 raised by main from the plan's expected amendments; one of the raises
    was folded into a detector's proposal.
  - One escalation: the fifth-paste widening, settled by one answer-card confirm.
- 20 leaf findings applied over nine leaves. `fanout-results.md`, `answer-card.md` and `cascade-dispositions.md` hold
  the record.

## Notes
- **Provisional, for the founder's own word at the Epoch 5 boundary batch.** The answer-card confirm of the
  fifth-paste widening was given by the operator as delegate viola-overseer. The ratification is the founder's
  own answer to P4 fork 1 (`inputs#I2`); his own word on the confirm is owed. Recorded so in the architecture,
  security-plan and test-plan sidecars.
- **Plan defects, recorded as deviations on the operator's word (`inputs#I4`), oracle and guard unchanged:**
  - entry 12, the literal probe, reads red: `17 pass` matches the correct `17 pass  1 fail`;
  - entry 13, the preservation guard, reads red on the two recorded companion test files.

  The light gate carries both as the operator's word. Pipeline owners: the fix-loop's two records
  (`contract.token-proxy-check`, `contract.jointly-contradictory-instructions`).
- **Every home stamped before this chunk reads `cli_verified:false`** until its next `viola verify`. That includes
  the tmpfs `--local-live` home, which reads `row-missing` until its next `run --local-live` (five starts, under a
  cap of its own; the P5 card row 4).
- **For whoever dispatches `windows-mutants.yml` next (the Epoch 5 boundary audit):**
  - CARRY 2's cargo-mutants grading of `src/bin/viola-fake-agent.rs` and `src/cmd/verify.rs` is owed there (P5 card
    row 2). The kills are shown by hand only (`evidence/carry2-kills.md`).
  - CARRY 3, the three `viola-state` retry-loop mutants graded by the 10 s kill on `windows-2025`, is owed there too,
    with no route entry (card row 3). It stays a hypothesis.
  - The matrix holds ten items; the root package's green baseline is still not measured.
- **Epoch growth:** Epoch 5 holds 12 entries (one minted this wrap; 7 markerless). The founder ruled no boundary
  inside Epoch 5; a split stays the operator's word.
- **For the founder, his next batch (the operator's word, none ruled by him):**
  - the interrupted pair's wording (a planning form);
  - the Windows event set (Ctrl-C and Ctrl-Break only);
  - a refused strict-modes check writes no line on `revive`'s start arm;
  - the checked read of `budget.json` writes no line on a refusal;
  - `list`'s exit-1 shapes and per-instance no-row;
  - the five implicit-link shapes;
  - a link line with no string `ts` → `since: "unknown"`;
  - a `--home` session shows no user status line until `statusline-source.json` is planted;
  - two relied-on dependences with no ledger row (now owed on "Owed ledger rows");
  - `viola revive --list` opens no process log;
  - the switch from the prototype not made, `v1-33` unclaimed;
  - one CLI plan file (2026-10-08) in his user directory;
  - the fifth-paste card confirm (above).
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
  - "Owed ledger rows" is minted directly after "Paste newline ledger row". It carries that entry's CARRY 4–8
    verbatim (`--resume`, `/compact`, the statusline override and `/bin/sh`, `rate_limits`, `claude agents --json`).
    Its placement is the founder's own fork-3 answer (`inputs#I2`), minted on the operator's word (`inputs#I4`).
  - A CARRY on "Windows-only live measurements" owns the `paste-newline` row's Windows reading (card row 5); that
    entry's `BLOCKED-ON` stands.
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
- **Curation:** one Tier-2 entry in `testing.md` (a count-literal probe matches the retired shape, never a bare
  `N pass`); one Tier-3 entry (the pattern-file TOML and inline-flag traps). Log: the wrap run dir's `curation.md`.
- **Deferred learnings:**
  - `recurrence-despite-learning: testing.md 2026-10-07 (a moved bound: list the cases by the function it decides,
    never by its literals)`. Research swept the row count's literals and missed five shape-pinned readers of
    `LedgerRow::ALL` and the fixture set.
  - carried:
    - `docs/session-learnings.md` "gate.py hygiene reads `/home/<x>/` in prose…";
    - `~/.claude/CLAUDE.md` Shell (a bounded repetition sent to the embedded grep);
    - `host-linux.md` Transports (one inline python heredoc carrying document text).
  - below the threshold:
    - a hand-applied mutant of a path accessor can write into the repository (build the struct directly);
    - a mutant that makes a loop's exit condition false spins its other case (select only the killing case).
- **Operator desk (the founder's word: leave them):**
  - the earlier chunks' scratch files;
  - gate logs under the OS temp dir's `andromeda-gate/`;
  - `target/wincheck/`, `target/run-archive/`, `target/agent-run/artifacts/`;
  - live homes and rig directories, now including `target/e2e-home/viola-record-20261013T213213Z/` (this chunk's
    record round, on the tmpfs backing);
  - `mutants.out*/`, `target/witness-wmg/`, `crates/viola-e2e/.viola-verify-227786-plan/`;
  - `~/.viola-record-20261004T142325Z`;
  - the CLI's own transcripts, this chunk's record round's Run B, C and D transcripts included.

  Yours to delete: `target/e2e-home.disk` and `target/profraw-census/`.
- **Last failed command:** none.
