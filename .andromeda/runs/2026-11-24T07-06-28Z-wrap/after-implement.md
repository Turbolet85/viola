## After implement
- **Report:** 2026-11-24T06-34-55Z-implement — the newest implement run of 2026-11-24-viola-up (`rec.py runs … --file
  '*-2026-11-24-viola-up.json'`, its last run line), its outcome `green` (`outcome-2026-11-24-viola-up.md` of that run:
  «**Word:** green»).
- **Commits:**
  - `863ea534 feat(2026-11-24-viola-focus): `viola focus` — gated on R3` (the last wrap's own commit, not this chunk's);
  - `a64d2e24 chore(2026-11-24-viola-up): operator pre-CI commit, for the run this chunk's verdict reads` (this
    chunk's whole diff, its base `863ea534`).
- **Operator pass:** run in `.andromeda/runs/2026-11-24T06-34-55Z-implement`, recorded in `evidence/operator-pass.md`
  (pass 2), WITHOUT any R4 entry:
  - entry 18 `bash scripts/agent-run.sh pre-push`: GREEN, exit 0, 100.68 s, 4320 passed, 0 failed
    (`evidence/operator-18-pre-push-2.txt`); the host read before it (06:49:35Z, `--from 06:45:38Z`) `QUIET - 5 s
    stalled on IO`, `df -m /tmp` 30960 MB free;
  - entry 19 `gate.py hygiene`: `hygiene: clean — read 333 (runs 274 · evidence 20 · inputs 39) · … 0 host paths kept`;
  - the land (tx 4581), then the pre-CI commit `a64d2e24`;
  - entry 20, the push: `history moved: refs/remotes/origin/build/viola-0.2.0 863ea534→a64d2e24`;
  - entry 21 `ci.py conclusion --sha HEAD --wait 1800`: `a64d2e249fb4 verdict: green · checks 7/7 · wall 770 s · runs
    ci#45343654177 completed/success` (fuzz-replay, lint, msrv, perf, release, supply-chain, test; all on
    ubuntu-latest);
  - entries 22 and 23 (the two builds), 24 (R4a) and 25–26 (R4b's plant and after): NOT fired; entry 17 (the
    `evidence/r4.md` read) stays deferred, gated on R4;
  - pass 1 (run `2026-11-24T05-58-30Z-implement`) was red at pre-push (the corrupt fake-agent profile; 4320 of 4320
    passed) and is superseded: folded as step 22 (plan rev 12);
  - the pass's end: as-left — green, no fix commit.
- **Changes after implement:** none — no commit after the pre-CI one (`git log --format='%h %s' 863ea534..HEAD` lists
  `a64d2e24` alone; `evidence/operator-pass.md` «Commits after the pre-CI one: none»). The wrap directs no tracked edit
  before its scope read.
- **Inputs:** `inputs.py verify` (this run): `inputs: 17 entries — unchanged 9 · drifted 0 · vanished 0 · broken 0 ·
  altered 0 · unreachable 0 · n/a 8 · uncited 2 · unparsed 0`. No DRIFT.
  - `inputs#I16` · `../additional/viola-overseer/wrap-up-directive.md` · copy no-repo · unchanged — the wrap directive,
    snapped `--step wrap`. What it changes: the close states v2-12 GATED and no stronger; U2 arm 1 lands on ONE answer
    card as the operator's own word, every other fork row (U1, U3–U8, T1–T7, every P5 answer, both fold decisions)
    PROVISIONAL; the a11y time-bound lists are NOT grown and NOT split (fork F1 of run `2026-11-24T00-26-24Z-wrap`); the
    named departures and rows below are closed by name; no question dialog for the operator.
  - `inputs#I17` · message · overseer (the wrap invocation) · n/a — proceed under `inputs#I16`, v2-12 stays claimed
    gated, no operator dialog, the host read before the light gate and a stop for a quiet window on a live neighbour.
  - `inputs#I1`–`inputs#I15` stand as implement's report cites them (unchanged 7 · n/a 8, each its own row of this run's
    verify trail `.andromeda/runs/2026-11-24T07-06-28Z-wrap/inputs-2026-11-24-viola-up.json`).
- **Scope:** none appended. `gate.py scope` (this run): `scope: clean — changed 26 · listed 23 · recorded 3 (companion
  2 · mechanical 0 · in-intent 1 · widening 0) · absorbed 0 · excluded 283`, base `863ea534` — the three lines
  implement recorded (`crates/viola-state/src/board.rs` in-intent; `src/human.rs`, `tests/contract_diag_schema.rs`
  companion).
- **Sweep reach:** `cascade.py sweep --pre-pass` over `report-patterns.toml`, baseline `a64d2e24` (HEAD); parts
  `sweep-pre-pass-1.1.txt` (7 patterns) and `sweep-pre-pass-2.1.txt` (`term-window`); trail
  `cascade-2026-11-24-viola-up-pre-pass.json`.
  - the help's `setup:` row (Counts / qualifiers moved) · `setup-row` · standing 8: design-system 2, layout-templates 2,
    test-plan 1, a11y-plan 3. Stale 4: design `ctr-pbkoayxj`, `ctr-njtw4ip3` and layouts `ctr-mt5jspdm`,
    `ctr-m2dq7qek` list `setup:` as `open, run, revive, verify, plugin install` with no `up` (the code: `open, run,
    revive, up, verify, plugin install`). A true claim sharing the token 4: test-plan `ctr-gugpniyn`, a11y
    `ctr-2myx5uyz`, `ctr-2oazfnr2`, `ctr-2oq4dyhr` (the group name, no verb list). Owner: design-system and
    layout-templates; leaves `.claude/rules/frontend.md:25`, `.claude/docs/a11y-summary.md:29` to the cascade.
  - the `boot-time-unread` exit-cause row and the `process-exit` detail enum word (Counts / qualifiers moved; Schema /
    config) · `boot-time` · control UNFIRED: no master holds `boot-time-unread`, «boot time», `btime`,
    `cross_exit_causes` or «exit-cause table» (the refusal, the trail's record, is the zero's basis) · `exit-detail` ·
    standing 91: architecture 35, security-plan 3, test-plan 13, obs-plan 40. Owed: obs-plan
    `contract/process-exit-detail/exit-1` (`ctr-aful5fws`) and architecture's `contract/process-exit-detail/*` set,
    which holds no `boot-time-unread` item. Every other row names another detail word or another verb's exit, a true
    claim sharing the token (87). Owner: obs-plan, architecture.
  - the bounded readers (`BOUNDARIES` +2, `EXCEPTIONS` +1) · `bounded` · standing 2: architecture `dec-udhfflzr`,
    test-plan `dec-fdhigpfh`, each naming the guard `contract_bounded_readers` with no count — true claims sharing the
    token, the readers themselves owed a row at P2 if a master lists the bounded sources (`/proc/stat` stands in no
    master: 0 rows). Curation 2 (`.claude/rules/security.md:45`, `.claude/rules/testing.md:132`): true.
  - `CLI_DOCUMENTS` +5 · `cli-docs` · standing 3: architecture `dec-xxc5ggaf`, test-plan `dec-krugsbrl`,
    `dec-dn4xkoux` — the guard's name, no count: true claims sharing the token.
  - Spec claim disproved — the terminal pair · `term-pair` · standing 14: security-plan 1, obs-plan 13, each a
    `subject:"self"` or another subject's pair, true claims sharing the token, 0 naming `terminal` · `term-window` ·
    standing 3: obs-plan `contract/diag-subject/terminal` (`ctr-yt4sqped`) is STALE: it promises «the process-start and
    process-exit pair of the foot terminal window that viola open spawns»; the code writes the `process-start` alone
    (`src/cmd/open.rs` `spawn_terminal`), and `up` the same through it. layout-templates `ctr-lsboc7mb` and test-plan
    `dec-zysuqw2t` are true claims sharing the token. Owner: obs-plan; leaves `.claude/rules/observability.md:42`
    (the rule the directive names) and `.claude/docs/obs-summary.md:70` stale, to the cascade.
  - revive's hidden `--origin` (Expected amendments) · `origin` · standing 8: architecture 3, security-plan 1,
    test-plan 1, obs-plan 3 — each `viola run`'s `--origin`, true as written; revive's flag is owed beside them
    (architecture `contract/cli-verbs/revive`, `decision/revive-resumes-in-place`; security-plan
    `decision/run-window-flags-checked`; layouts `contract/viola-revive/no-window`). Owner: those masters.
  - the test wrappers' lock files (the directive's departure) · `locks` · standing 8: architecture 6, security-plan 2 —
    the naming and crash-safety rows (`decision/on-disk-file-naming`, `decision/crash-safe-disk-writes`, …) and
    `Cargo.lock`; none says who removes a socket directory's `.lock` sibling, or that a test wrapper removes its own.
    True claims sharing the token; the absence is the fact (below).
  - reach: 9 patterns over 2 sweeps · 137 standing master rows (134 + 3) over the 7 masters · stale 5 (4 `setup:`
    rows, `ctr-yt4sqped`) · owed adds/sets on `boot-time-unread` and revive `--origin` · leaf 84 rows to the cascade.
- **Outcome, final:**
  - supersedes «Next: the operator pass in the same window, WITHOUT any R4 entry»: the pass ran green as-left (above);
    the final HEAD `a64d2e24` read CI ci#45343654177 success, 7 of 7.
  - The two reds of the chunk, and how each ended:
    - the fd case (`tui_channel_fds_the_child_holds_nothing_of_violas`, run `2026-11-24T05-12-02Z-implement`):
      mended by a bounded re-read of the child's fd table inside the case's existing 2 s deadline (step 21); its
      control `fds-held-red` holds (`evidence/control-2.txt`). Closed by its fix, never by a re-fire.
    - the truncated coverage profiles (pass 1's pre-push red): measured by census, nine cases and both `cli_open`
      controls at 192 × 16 (`evidence/profraw-census.md`); mended by ONE shared teardown,
      `tests/support/terminal.rs` `end_trees`, joined by `cli_open`'s `Terminal` and `cli_up`'s `Terminals` guards
      (step 22); re-measured `refused 0` on entries 28–37; the old kill, read by the `old-kill` control with the wait
      removed, `refused 7` of 2227 (`evidence/old-kill.md`). The defect came with chunk `2026-11-23-viola-open-2`'s
      test guard (its `cli_open` `Terminal` kill loop, which this chunk's `cli_up` copied); this chunk's step 22 mended
      both.
  - matrix#v2-12: claimed, GATED on R4, no ref, its gated entry deferred, no flip. R4 has NOT run: no real window, no
    real compositor, no real `claude`, no reboot. `scripts/r4-up.sh` REFUSES R4b as built (`r4: red
    precondition=r4b-not-granted`). Proven over the fakes only: `viola up` and revive's hidden `--origin`.
  - watches: none folded.
  - the outcome basis: Setup 4's commit list (`a64d2e24` on `863ea534`), the final HEAD's CI run ci#45343654177 in
    `evidence/operator-pass.md`, the pass record; implement's three outcome files for what only they hold.
  - Every other line of implement's Outcome stands.
- **Decisions & corrections after implement:**
  - `inputs#I15` (the overseer, delegate for the away operator): the neighbour held from 06:45:38Z until the pre-CI
    commit; the pass ran inside that hold (`evidence/operator-pass.md`).
  - `inputs#I16` (the wrap directive): U2 arm 1 is the operator's own word (his dialog of 2026-11-24T05:21:35Z,
    relayed verbatim by overseer1, the revision's `inputs#I6` item 3); every other fork row lands PROVISIONAL; the a11y
    time-bound lists are not grown and not split (fork F1 of run `2026-11-24T00-26-24Z-wrap`).
  - The rule `.claude/rules/observability.md` (§Panics and errors, last bullet) is CORRECTED to the code, not the code
    named a carry: `viola open` and `viola up` write one `process-start{subject:"terminal"}` per spawned terminal and
    no `process-exit`, since the terminal outlives the verb (the verb returns once the instance is live). Writing a
    `process-exit` the verb cannot observe would be a new log line with no defined exit; the spec follows the built
    behaviour. Its master row obs-plan `contract/diag-subject/terminal` is set at P2 and the rule follows in the
    cascade.
  - The lock files: `$XDG_RUNTIME_DIR/viola/` holds 5496 entries, 5234 of them `*.lock` (`ls | grep -c '\.lock$'`,
    read 2026-11-24T07:08Z). No master row and no rule says a test wrapper removes its socket directory's `.lock`
    sibling (the sweep's `locks` rows; `testing.md`, `verification-harness.md` read for «lock»: only home and
    `Cargo.lock` rules). Owed: a carry on the route entry that owns test hygiene, at P5.
- **Process hygiene, re-measured:** `gate.py census` (this run, comm viola, viola-fake-agent, foot, claude, hyprctl;
  args projects/viola/target): `census: 12 running`, none this chunk's and none under this repository's `target/`: the
  three overseer feet and their claudes, `~/.viola`'s `viola mcp`, escher-builder's foot, wrapper and claude, the
  prototype's `viola wait escher-builder` and the product's `viola wait viola-builder`. Started-by NOT DERIVED; left
  running — the operator's fleet.
