# Scope — 2026-10-19-newer-claude-cli-rows

**Working entry (Epoch 6 head, `viola-0.1.0/working-route.md`, "Newer claude CLI rows"):** Newer claude CLI rows — the
four ledger rows claude 2.1.302 fails, re-measured and brought back to a full stamp.

**Directive:** the overseer's phase directive, `inputs#I1`; the invocation, `inputs#I2` (the founder is away: his forks
go in the away form).

## What this chunk builds

- `viola verify` on claude 2.1.302 read `stamped 2.1.302  20 pass  4 fail`. The four failing rows are `tag-escaping`
  (16/24), `local-command-clear` (17/24), `paste-newline` (18/24) and `local-command-compact` (24/24). The run is the
  switch chunk's `chunks/2026-10-19-overseer-switch-to-viola/inputs/I20-verify-2.1.302-20261019.out.txt`, which is
  byte-identical (sha256 `e1adf486…`) to the file `inputs#I1` item 3 names. The verdicts give no reason for any row.
  So a 2.1.302 home reads `cli_verified:false` and drives transport-only (the entry's CARRY).
- **Re-measured row by row:** for each of the four rows, what the probe observed on 2.1.302 against its oracle,
  read from the probe's own captures.
- **Brought back to a full stamp:** `viola verify` on 2.1.302 reads 24 pass / 0 fail on the Linux dev host, so a
  2.1.302 home reads `cli_verified:true`. "Full stamp" = all 24 rows of `LedgerRow::ALL` pass, which is the relied set
  on Unix (`LedgerRow::relied()`, research.md); no row is added or removed.
- [premise-corrected: Run B's 2.1.302 transcript holds only the probe prompt and the long paste; its pastes stopped at
  `box_wait`'s 120 s after the long paste's turn (`inputs#I3` R1, `src/cmd/verify/typed.rs:402`)] The CARRY's
  hypothesis read: "hypothesis: the CLI changed its paste and local-command handling (not measured row by row)". The
  four failures are ONE stop, not four:
  - On 2.1.302 Run B never typed the tag-like, newline, `/compact` and `/clear` pastes.
  - `tag-escaping` and `paste-newline` found no capture.
  - `local-command-clear` and `local-command-compact` cannot pass without those captures, being anchored on them
    (`crates/viola-agent-claude/src/ledger.rs:817-849`).
  - The stop's cause, read from the 2.1.302 bundle (not from a live screen, `inputs#I3` R4): the footer hint
    `" for agents"`, the one compiled input-box literal (`SIGNATURES.input_box`), is drawn only while
    `agentsHintExpired` is false, a 10 s value over the session's attacher capabilities.
  - What sets it in Run B is unmeasured. How 2.1.302 handles pastes and local commands is unmeasured too: the probe
    never reached them.
- [premise-corrected: one cause, a screen signature `run`'s gate shares (`src/run/gate.rs:508`)] The resolution is
  way (b), and it reaches the product. The compiled input-box literal set gains the literal 2.1.302's resting input
  box keeps (measured live), so verify's settles and `run`'s pre-send gate both read a 2.1.302 input box.
  - Stamped with today's set, a verified 2.1.302 home would refuse `send` `input-not-ready` once the hint is gone.
  - The `input-box-signature` row, which passed on 2.1.302 only because it reads screens settled within 10 s
    (`inputs#I3` R3, `ledger.rs:1006-1008`), also reads a screen past the expiry.
  - The four rows' oracles change only if the live round shows a paste or local-command shape that differs. Way (c)
    stays the fork for a shape the product can no longer rely on.
- Earlier stamps do not regress (verified achievable: 2.1.296 draws the hint unconditionally, `inputs#I3` R4). 2.1.287 (24/24 on the Linux dev host, the version every row was stamped
  on) and 2.1.296 (24/24 in the overseer's home, `inputs#I3` R3) keep passing the changed signature set and probes.
  The fake agent's committed fixtures keep replaying in CI: 2.1.287's recorded turn row
  `"  ⏸ manual mode on · ← for agents"` must still match (`fixtures/claude/2.1.287/Screen.turn.json`).
- The 2.1.302 shapes are recorded (`viola verify --record` into `fixtures/claude/2.1.302/`). A new compiled literal
  needs a recorded screen holding it for the fake agent to replay in CI, and `--record` writes only at 0 fail, so
  the recording follows the fix in the live round (research.md).
- Surfaces (research.md):
  - `crates/viola-agent-claude/src/screen.rs` (the literal set);
  - `ledger.rs` (the `input-box-signature` row);
  - `src/cmd/verify/typed.rs` (Run B's settles);
  - the gate's reading of the measured row, as `screen.rs` verdict cases. `src/run/gate.rs` and `src/run/send.rs`
    stay unchanged: `src/run/send.rs:666` already pins a verified screen without a literal (amended at P5
    validation-1);
  - the fake agent;
  - `fixtures/claude/`;
  - `tests/cli_verify.rs` and `tests/contract_ledger_probes.rs`.

  Doc wording that names "stamped on 2.1.287" or the input-box literal (`CLAUDE.md` Critical Warnings, the ledger
  notes, the masters) is a wrap cascade, not this chunk's edit.

## Live starts (the founder's cap)

- Live `claude` starts are the founder's cap, given per chunk (`inputs#I1` item 2). He is away tonight. The plan names
  its live round and how many starts it needs; that cap is a P4 fork for his word, written in the away form. No live
  start runs before his word. Everything that needs no live start is planned to complete now.
- One `viola verify` is six starts: the overseer's one verify of 2.1.302 was starts 9–14
  (`chunks/2026-10-19-overseer-switch-to-viola/inputs/I16-relay-switch-step12.md.txt`). Run E's `--resume` is the
  sixth (arch history, chunk 2026-10-14-resume-and-compact-ledger-rows).
- A live verify writes its stamp to `ledger/stamps.json` in its home. Without `--home` that is the `VIOLA_DIR`
  instance's home under a parsed `VIOLA_NAME`, else `~/.viola`, the product's live home. `~/.viola` also holds the
  overseer's builder instance and its 2.1.302 stamp at 20/4 (`inputs#I3` R3). `scripts/agent-run.sh run --local-live`
  stamps its own `target/e2e-home/viola-live-<pid>/home` and never `~/.viola` (test-plan §3 `run`).
- Host facts at the take-up (`inputs#I1` item 3): PATH `claude` is 2.1.302. The builder session runs 2.1.296 through
  the overseer's pin shim `~/.local/viola-claude-pin/bin/claude`, which is first on the builder's PATH. The pin and the
  overseer's scripts are outside this repository, and this chunk does not touch them.

## Folded freight and verdicts

- watch: a `viola` process's truncated `.profraw` refused by `llvm-cov` at `pre-push`'s coverage merge (header
  corrupt), its killer unknown (0/3; since 2026-10-19-overseer-switch-to-viola; that chunk's
  `chunks/2026-10-19-overseer-switch-to-viola/inputs/I24-relay-4.md.txt`).
- **The macOS readiness flake of `b6ddc198`, folded on the overseer's directive (`inputs#I1` item 1), with its own
  acceptance criterion and a witness that cannot pass vacuously.** VERIFIED against the recorded run
  (ci#39321117796 attempt 1, its `diag-macos-latest` artifact, `inputs#I3` R5), coordinates kept, localized:
  - Both failing wrappers (pids 16375 and 16377, started 40 ms apart) wrote `process-start{subject:"self"}` and then
    nothing for ≥ 11 s. Every other run of that minute wrote its `version-probe` start within 33–47 ms.
  - The stall lies inside `run.start` before the version gate (`src/cmd/run.rs:216-251`: `resolve_program`,
    `collision_check`, `pin_and_plugin`, `statusline_command`, `plan_strip`).
  - The phase is unmeasured. `pin_exe`'s whole-exe read, hash and copy is a hypothesis.
  - The runner's tracker holds no matching issue (research.md, Platform issues consulted).
  - Run: ci#39321117796, sha `b6ddc19856e55fd9d4083db4400a8196f24f6488`, job `test (macos-latest)`.
  - Attempt 1 was red and attempt 2 green.
  - Two tests failed, both with `wrapper builder not ready` at `tests/support/watch.rs:54` after ~16.7 s, with the
    watch report `starts w0 c0 r0 snapshot false beat false`:
    - `cross_list_rows_print_no_sgr_under_no_color_dumb_piped_or_json`
    - `cross_list_rows_print_escaped_in_the_captions_columns_in_every_mode`
  - `b6ddc198` changes no source file against `aa965127`, which was green 15/15.
  - Job log: job 118295119002.
  - P3 resolves it against that run, never against HEAD. The coordinates (`watch.rs:54`, the job id) are the
    directive's and are re-verified there.
- **The CI read at Setup** (Setup 5a, the trail `.andromeda/runs/2026-10-19T23-10-05-phase/ci-no-marker.json`):
  - `50e2f76`: verdict not yet available (ci#39339625602 queued).
  - `b6ddc198`: red, 25/25 checks, attempt 2, wall 6445 s, first fail at +2896 s `mutants (viola-e2e)`. It was red by
    run windows-mutants#39326811932 (a `workflow_dispatch` at 2026-10-19T21:21:20Z). Its red jobs were `mutants`
    (`viola-e2e`, `viola-cmd-verify-stop`, `viola-pty`, `viola-main`, `viola-panic-frames`, `viola-run-env`).
  - The red is not this chunk's subject, and it is already owned: "Epoch 5 cleanup"'s first CARRY, item (3), names
    that run ("the Windows mutation leg's baselines pass (run 39326811932: …)"). Leaned per that working-route entry:
    it is not folded and not asked, because its owning chunk is minted.
  - The macOS reading above is the directive's own (ci#39321117796); the tool's row named only the windows-mutants run.

## Boundaries

- No change to the overseer's pin, shim or scripts. No live start before the founder's word. No mutation run or
  workflow dispatch (that is "Epoch 5 cleanup"'s work).
- These rows' Windows readings stay owed to "Windows-only live measurements".
- The other 20 rows and their probes change only where a fix to one of the four reaches a shared path. The shared
  path is the input-box literal set (`SIGNATURES`, 16 references, research.md Graph impact), which Runs A–D's settles
  and the `input-box-signature` / `modal-signature` rows all read. Run B's paste sequence, its literals and its
  six-start count do not change.
