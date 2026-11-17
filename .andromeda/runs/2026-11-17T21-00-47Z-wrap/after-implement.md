## After implement
- **Report:** 2026-11-17T20-10-21Z-implement — the newest implement run of 2026-11-17-trust-record-ledger-row
  (`rec.py runs … --file '*-2026-11-17-trust-record-ledger-row.json'`, its last run line), its outcome `green`
  (`outcome-2026-11-17-trust-record-ledger-row.md`: «Outcome: green»).
- **Commits:** since last_wrap 2026-11-17T19:03:40Z (the `state` view): `f9d75214 chore(route): operator-requested
  adaptation — 0-pending wrap` (the wrap that minted this entry) · `95792f71 chore(2026-11-17-trust-record-ledger-row):
  operator pre-CI commit, for the run this chunk's verdict reads`. The pass's commits from the parent f9d75214:
  95792f71 alone.
- **Operator pass:** run in the implement window (evidence/operator-pass.md): 15 smoke boot `trust-smoke` green ·
  16 status green, `ready` · 17 cleanup green, processes_gone, home_removed · 18 `pre-push` green (linux.run passed
  3519 failed 0; linux.gate ok, breaches []) · 19 `gate.py hygiene` clean (read 223, 0 host paths kept) · the pre-CI
  commit 95792f71 · 20 push green, origin f9d75214→95792f71 · 21 `ci.py conclusion`: «95792f71eb0c verdict: green ·
  checks 7/7 · wall 610 s · runs ci#44172988913 completed/success» (fuzz-replay · lint · msrv · perf · release ·
  supply-chain · test). The pass's end: as-left, green.
- **Changes after implement:** none — no commit after the pre-CI one; the wrap directs no edit.
- **Inputs:** `inputs.py verify` (this run): I1 · ../additional/viola-overseer/phase-trust-row-directive.md ·
  copy no-repo · unchanged (cited by the scope, research, plan and report) · I2 · ../additional/viola-overseer/
  wrap-trust-row-directive.md · copy no-repo · unchanged — the overseer's WRAP DIRECTIVE, snapped `--step wrap` (cited
  here: inputs#I2) · I3 · message (the wrap, extracted from the implement window session log d922876c) · the
  implement window's one `ScheduleWakeup` call and its refusal (cited here: inputs#I3). Summary at the read: «inputs:
  2 entries — unchanged 2 · drifted 0 · vanished 0 · broken 0 · … · uncited 1» (I2 uncited before this close; I3
  snapped after it). No DRIFT. What I2 changes: (1) the live reading S0 is recorded AS READ and does NOT verify the
  row — `trust-record` stands BUILT and proven on the fake agent and UNVERIFIED on the real CLI, its cause unmeasured;
  (2) the answer card's two widenings are already ruled (the founder's R4, his dialog answer read
  2026-11-17T18:59:41Z): verify's child set of five becomes six (Run F), and verify writes the trust record for its
  own temp dir into the config its child reads and removes that one record after — the bounded exception to the rows
  that say viola writes nothing into `~/.claude.json`; the PRODUCT's write (`run` / `revive`) is not amended in here;
  (3) route-resolve asks the overseer in one dialog (the owed live re-read as a carry, the stamps, the 52 s watch).
- **Scope:** none appended — the wrap's scope read: «scope: clean — changed 27 · listed 26 · recorded 1 (companion 1
  · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 222» (base f9d75214).
- **Sweep reach:** `cascade.py sweep --pre-pass` (trail `cascade-2026-11-17-trust-record-ledger-row-pre-pass.json`),
  first fire exit 3: «control: stepctr22, stamped22, fakestarts6, receipts6, ledgerrows never fired over the seven
  masters' pre-pass text» — each of those five a master zero on that refusal's basis (the step counter `[NN/22]`, the
  `22 pass 0 fail` line, the fake agent's six starts, the six `env` receipts and `LEDGER_ROWS`: no master site — test
  and harness text only). Re-fired on the other nine:
  - `rows22` (the relied rows 22 → 23, name token `LedgerRow::ALL`) · standing 1 · architecture 1
    (`decision/permission-request-question-stays-null`, «adding one needs a 22nd row») → stale, owed architecture;
    leaf 3 (viola-agent-claude.md, viola.md, workflow.md) → the cascade.
  - `runs5` (verify's interactive runs five → six, A–F) · standing 4 · architecture 2 (`decision/verify-child-set`,
    the decision row whose text holds «the five runs execute in order A to E») · obs-plan 2
    (`contract/diag-subject/verify-pty-probe`, `contract/verify-spawn-subject/verify-pty-probe`) → stale, owed those
    masters; leaf 12 → the cascade.
  - `pairs8` (spawn pairs eight → nine) · standing 1 · obs-plan 1
    (`contract/verify-process-exit/child-exit-status-and-duration`) → stale, owed obs-plan.
  - `claudejson` (verify's own temp-dir record in `.claude.json`) · standing 2 · security-plan 2
    (`decision/no-writes-under-claude-home` «…or into ~/.claude.json», and the decision that reads the user's
    `~/.claude.json` only after a moved stamp, the rig's private config) → the first stale (the bounded exception), the
    second a true claim sharing the token (no change); curation 3 → no change (true facts); leaf 1 → the cascade.
  - `cfgdir` (`CLAUDE_CONFIG_DIR`'s one value read) · standing 1 · security-plan 1 (the rig's private config row) → a
    true claim sharing the token; the owed sites are the env-read decisions the plan names (architecture
    `decision/env-strip-names-only` · `/no-environment-configuration` · `/env-var-viola-prefix`, security-plan
    `decision/r8-claude-env-strip` · `/credential-strip-by-name-only` · `/new-env-read-needs-log-entry`), which state
    the claim in other words — the detectors' own reach.
  - `claudehome` (viola writes nothing under the Claude home / into Claude's config) · standing 3 · security-plan 3
    (`decision/no-writes-under-claude-home` · `/nothing-written-under-claude-home` · `/viola-writes-nothing-in-claude-dir`)
    → stale against verify's bounded exception, owed security-plan.
  - `fakeargv` (`--trust-config`, the fake agent's argv) · standing 3 · test-plan 3 (`contract/fake-agent-cli/
    argv-only-options`, the `--transcript` contract, `decision/fake-agent-argv-only`) → the first and third add-to, the
    second a true claim sharing the token; leaf 2 → the cascade.
  - `stamphome` (every stamping verify under a test-owned `HOME`) · standing 1 · test-plan 1 (the kill-witness
    decision for a tied stamping verify) → add-to, owed test-plan; leaf 2 → the cascade.
  - `claudeint` · standing 1 · obs-plan 1 (`decision/claude-internals-not-a-contract`) → add-to, owed obs-plan.
  Expected-amendments entries naming no pattern hit (`contract/ledger-stamps/rows` · `/measured`,
  `decision/verify-removes-only-own-dirs`, `decision/claude-shapes-in-one-crate`, security-plan
  `decision/verify-probe-dirs-cleanup` · `decision/probe-artifacts-cleanup`, test-plan
  `contract/fake-agent-screens/trust-dialog` and §3 boot step 4) state the claim in other words — the detectors' reach.
  Sweep reach: 14 patterns · 21 moved facts, 21 with a pattern (the report's own count: 8 Counts facts and 13
  Expected-amendments entries, each named by a pattern above or the five-pattern refusal) · by line 0 lines · 0
  offsets. The re-fire's total: «total (9 patterns) · 41 rows over 17 files».
- **Outcome, final:** implement's Outcome stands, with these lines restated:
  - supersedes «(CI) the pushed HEAD's CI — the operator pass's.»: ci#44172988913 over 95792f71, completed/success,
    checks 7/7 (the overseer read it too: 7 of 7 jobs green, inputs#I2).
  - supersedes «`pre-push`, `gate.py hygiene`, the push and `ci.py conclusion` — the operator pass's.»: each green
    (**Operator pass** above).
  - restates «(live) … met as worded» in the directive's words (inputs#I2 item 1): the live reading S0 is recorded AS
    READ and does NOT verify the row. S0 (ledgered 2026-11-17T20:39:44Z, 2.1.325 by path, one start, the spare S0-2
    unspent, `keys: none`, `stop: none` — verify exited by itself): `verify-exit: 1`, `pass-count: 7/23`,
    `trust-record: fail`, `record-left: absent`, `user-config: changed-by-others`, `needles: 1`. Rows 8–23 failed from
    `input-box-signature` on: no interactive run reached the input box, rows a live verify of 2.1.325 passed before
    among them. The row `trust-record` stands BUILT and proven on the fake agent, and UNVERIFIED on the real CLI; its
    cause is not measured. Measured by the builder after the run (the code's lines, no start): the file was the rig's
    private `<scratch>/config/.claude.json`, never the user's own; runs B–E stood in `.viola-verify-<pid>*` subfolders
    of the one seeded folder `<scratch>/cwd`, Run F in a fresh `viola-verify-XXXX` dir under the OS temp dir; none inside a git
    repository; no child's screen was kept (the rig's frames are verify's own terminal). The owed live re-read goes to
    the overseer at route-resolve (P5).
  - watches: none folded; noted, `cli_verify runs::verify_window_without_screens_fails_every_interactive_row` reads
    52.2–52.3 s (three implement runs) against its 60 s kill — its owner proposed at P5.
  - Outcome basis: Setup 4's commit list (95792f71 alone past f9d75214), the final HEAD's CI run ci#44172988913
    (evidence/operator-pass.md entry 21), the pass record; implement's report for what only it holds.
- **Decisions & corrections after implement:**
  - The wrap directive (inputs#I2): S0 recorded as read and never written as «verified» or «the CLI honours the
    record» as a measured fact; the owed live re-read asked of the overseer at route-resolve; the two widenings ruled
    by the founder (R4) carried on the answer card citing his words, asking nothing new; the nine deviations judged by
    the reconcile.
  - The slip, as it happened (inputs#I3): the implement window's one `ScheduleWakeup` call, at
    2026-11-17T20:31:38Z, carried `{"stop": true}`; its refusal at 2026-11-17T20:31:41Z reads «Permission for this
    action was denied by the Claude Code auto mode classifier. Reason: [Destructive or Unwanted Actions].» — the
    permission classifier, no hook. No wake or timer was set (implement's report: «made in error … refused by the
    permission classifier»).
- **Process hygiene, re-measured:** `gate.py census --comm claude --comm viola --comm viola-fake-agen --comm
  viola-live-rig --comm viola-harness --args projects/viola/target` (this wrap's run dir): «census: 16 running» —
  every row the host's own Claude sessions, the overseer pair's prototype `viola run` wrappers, a `~/.viola` `viola
  mcp` and two `viola wait` processes; none under this repository's target — none this chunk's.
