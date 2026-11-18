## After implement
- **Report:** 2026-11-18T16-00-06Z-implement — the newest implement run of 2026-11-18-browser-tools-prompt-modal
  (`rec.py runs --kind '*-implement' --file '*-2026-11-18-browser-tools-prompt-modal.json'`, its one run line), its
  outcome `green` (`outcome-2026-11-18-browser-tools-prompt-modal.md`: «Outcome: green + smoke.»).
- **Commits:** since last_wrap 2026-11-18T13:43:35Z (the `state` view): `7e6b7fd4 feat(2026-11-18-session-folder-trusted-by-viola): …`
  (the prior wrap's own commit, not this chunk's) · `04890aeb chore(2026-11-18-browser-tools-prompt-modal): operator
  pre-CI commit, for the run this chunk's verdict reads`. The pass's commits from the parent 7e6b7fd4: `04890aeb` alone.
- **Operator pass:** run in the implement window (`evidence/operator-pass.md`). Smoke 15–17 green (boot · status
  `cli_verified:true` at 24 rows · cleanup processes_gone). Entry 18 `bash scripts/agent-run.sh pre-push` green · exit 0
  (`"ok":true`, coverage suite 3642 passed · 0 failed). Entry 19 `gate.py hygiene` `clean — read 239`. Entry 20 the push
  green (`refs/remotes/origin/build/viola-0.2.0 7e6b7fd4→04890aeb`). Entry 21 `ci.py conclusion --sha HEAD --wait 1800`:
  `04890aeb9798 verdict: green · checks 7/7 · wall 650 s · runs ci#44347642563 completed/success` (fuzz-replay · lint ·
  msrv · perf · release · supply-chain · test). The pass's end: as-left — every operator entry green, no fix after the
  pre-CI commit.
- **Changes after implement:** none — no commit after `04890aeb`; the wrap directs no edit (inputs#I7).
- **Inputs:** `inputs.py verify`: `inputs: 8 entries — unchanged 4 · drifted 0 · vanished 0 · broken 0 · altered 0 ·
  unreachable 0 · n/a 4 · uncited 2 · unparsed 0` (the two uncited are this wrap's own, cited here):
  - I1 · ../additional/viola-overseer/phase-browser-tools-directive.md · copy · unchanged
  - I2 · message (phase invocation) · copy · n/a
  - I3 · ../additional/viola-overseer/relay-browser-tools-p3.md · copy · unchanged
  - I4 · message (phase resume) · copy · n/a
  - I5 · ../additional/viola-overseer/implement-browser-tools-directive.md · copy · unchanged
  - I6 · message (implement invocation) · copy · n/a
  - inputs#I7 · ../additional/viola-overseer/wrap-browser-tools-directive.md · copy · unchanged — the overseer's wrap
    directive, snapped `--step wrap`: the operator away, a fork to the overseer as a dialog, an answer card whose rows
    lie inside the founder's recorded answers confirmed by the overseer, a widening outside them its most
    conservative option written to `founder-forks.md`; the live proof as ledgered; the card citing «Три слова
    (Recommended)» (inputs#I3, read 2026-11-18T15:41:31Z) and «Разрешить (Recommended)» (inputs#I5, read
    2026-11-18T15:59:31Z); the bt-absent control left INDETERMINATE and not re-fired; route-resolve in one dialog. It
    changes no fact of implement's report: it orders the wrap.
  - inputs#I8 · message (the wrap invocation, the Viola overseer) · copy · n/a — wrap whole per inputs#I7, push, read
    CI after it, no wake or timer tool.
- **Scope:** none appended — `gate.py scope`: `scope: clean — changed 20 · listed 17 · recorded 3 (companion 1 ·
  mechanical 0 · in-intent 2 · widening 0) · absorbed 0 · excluded 232`, base 7e6b7fd4; the three recorded lines are
  implement's (its Deviations).
- **Sweep reach:** `cascade.py sweep --pre-pass`, baseline 04890aeb, trail `cascade-2026-11-18-browser-tools-prompt-modal-pre-pass.json`,
  listing `{run_dir}/sweep-pre-pass-1.1.txt`. Per moved fact:
  - relied rows 23 → 24, the step counter, the all-pass summary, the row-id arrays, the rig's 24/24 → `rows24`
    (name token `browser-tools-prompt`): master 0 — the control refused (`rows24 · control UNFIRED over the seven
    masters`, the trail's first record), the basis of the zero: no master states the relied-row count (they read
    «every relied row», «MM being the relied row count»); the new row's id stands in no master — added by its owner
    rows below.
  - modal literals 2 → 3 → `modals3`: standing 8 (architecture 4 · security-plan 2 · test-plan 1 · a11y-plan 1) · 0
    changed · 8 no-change (each says «a modal literal» generically, true at three). The rows naming the literals and
    their sources (architecture `decision/screen-signatures-compiled`, security-plan
    `decision/compiled-classification-sources`, `decision/input-box-readiness-from-literals`) carry other words: the
    proposer's by key.
  - first-screen closed words 5 → 6 (`browser-tools`) → `firstscreen`: standing 11 (architecture 6 · security-plan 4 ·
    test-plan 1) · 2 changed (architecture `contract/ledger-stamps/first-screen`; security-plan
    `decision/verify-stamp-first-screen-closed-words`) · 9 no-change (the `--record-first-screens` record and the rig's
    first-screen file, untouched).
  - the verified rule (every relied row `pass` → every relied row counting) → `verifiedrule`: standing 5 (architecture
    2 · design-system 1 · layout-templates 1 · test-plan 1) · 5 changed (architecture `decision/cli-capability-ledger`,
    `decision/version-verified-all-relied-rows`; design-system `contract/viola-verify-output/…` trust-record line
    «like every relied row»; layout-templates exit 0 «when every relied row passes»; test-plan
    `decision/version-gate-needs-all-relied-rows`) · 0 no-change. Beyond the pattern's words, `rec.py find --doc
    architecture --regex 'relied (ledger )?rows?'` read two more of the same claim: `decision/capability-ledger` and
    `contract/ledger-stamps/verified-version` («every relied ledger row reads `pass`») — both changed.
  - verify's step-line verdict `absent` and the summary's `n` counting the rows that count → `summary`: standing 43
    (architecture 6 · security-plan 2 · design-system 7 · layout-templates 3 · test-plan 19 · obs-plan 3 · a11y-plan 3)
    · 12 changed (the step-line contracts: architecture `contract/viola-verify-output/step-line`, design-system,
    layout-templates, test-plan `contract/verify-output/step-line`, obs-plan `contract/verify-stdout/step-line`, a11y-plan
    step line; the summary-line contracts: architecture, design-system ×2, layout-templates, test-plan, obs-plan) · 31
    no-change (a refusal's «no step line, no summary», `passthrough`, a mutation run «passes», the regex
    `^stamped \S+  \d+ pass  0 fail$` and «stamped … 0 fail» endings, still true).
  - fake agent `--start-screen` → `startscreen`: standing 3 (test-plan 3) · 1 changed (`contract/fake-agent-cli/argv-only-options`)
    · 2 no-change (`--transcript`, `decision/fake-agent-argv-only`, true as worded).
  - the rig's `cache-a` key-name read → `cachea`: standing 1 (architecture `decision/claude-first-no-adapter-trait`)
    · 1 changed (its except gains the rig's key-name read, the founder's word inputs#I5).
  - the rig's VP / VK arms, their no-key stop, VP's unseeded key → `rigarms`: standing 2 · 2 changed (security-plan
    `decision/live-rig-seed-from-bundle-logic`; test-plan `decision/standing-live-rig`).
  - the code sweep over `*.rs` (implement's Counts) → no master site — a tree count with its own command, as implement
    states it.
  - Leaves 24 rows / 13 files, curation 3 / 2, base 1 / 1: the cascade's (P2), not dispositioned here.
  Sweep reach: 7 patterns · 12 moved facts, 11 with a pattern · by line 0 lines · 0 offsets (12 and 11 are this
  close's own count: implement's five Counts bullets and seven Expected-amendments entries; the code sweep has none;
  `rows24`, refused, is the eighth pattern of the file and outside the tool's `total (7 patterns)`).
- **Outcome, final:** supersedes «operator pre-push · hygiene · push · `ci.py conclusion` — the operator pass's, not run
  by this report»: green — pre-push `"ok":true`, hygiene clean, the push to 04890aeb, CI ci#44347642563 green 7/7 on
  04890aeb (`evidence/operator-pass.md`). Supersedes «(tests) MET for `run --integration` / `run --e2e` …; CI is the
  operator pass's»: MET, CI green 7/7. Every other line: implement's Outcome stands (no commit after the pre-CI one).
  Watches: none folded. The control `bt-absent` stays `INDETERMINATE: case
  verified_reads_the_browser_tools_row_absent_as_counting not run — the runner stopped at 448/650`
  (`evidence/control-1.txt`), recorded so and not re-fired (inputs#I7 item 5); its demonstration is proposed as a
  carry at route-resolve. The outcome basis: Setup's commit list (04890aeb alone), the final HEAD's CI run
  ci#44347642563 in `evidence/operator-pass.md`, the pass record, and implement's report for what only it holds.
- **Decisions & corrections after implement:**
  - The live proof is read exactly as ledgered (inputs#I7 item 1): two starts on one ledger
    (`evidence/prompt-starts.ndjson`), the spare BP-2 unspent, on 2.1.325 by path. BP 16:19:54Z —
    `browser-tools-prompt: pass`, screens B · C · D · F `browser-tools`, Run A `trust`, Run E not started,
    `pass-count: 8/24`, `verify-exit: 1`, `cache-a: written`, `keys: none`, `stop: none`, teardown whole. BK 16:20:02Z
    — `24/24`, B–F `box`, `browser-tools-prompt: absent`, `cache-a: absent`, `keys: none`, `stop: none`, teardown
    whole. The literal is read live on 2.1.325 only, on a host where the extension is detected and the account's flag
    is on; nothing is claimed for 2.1.326 / 2.1.327 or for a reworded prompt.
  - What `cache-a` shows (inputs#I7 item 2): BP `written` against BK `absent`, with B's first screen the prompt, is
    consistent with the CLI's scan writing the extension cache before Run B under BP's seed and writing none under
    BK's; it cannot place the write between Run A and Run B — the word was read once, after verify ended. Why Run E
    did not start on BP is a guess, unmeasured. The `claude` process count per start was forecast (BP 8, BK 10), not
    measured. How the feature flag is read stays partly static. Each goes to its route home at P5.
  - Two calls were the builder's, named in the rows they touch, not asked (inputs#I7 item 4): verify's summary counts
    an `absent` row in `n pass` while the step line and the stamp say `absent`; `cache-a` is read once, after verify
    ends.
  - The answer card cites the founder's own words (inputs#I7 item 3): «Три слова (Recommended)» (inputs#I3, read
    2026-11-18T15:41:31Z) for the 24th row's `pass` · `absent` · `fail`, `absent` counting toward verified on this
    row only; «Разрешить (Recommended)» (inputs#I5, read 2026-11-18T15:59:31Z) for the rig's read of the key NAME
    `cachedChromeExtensionInstalled` in its own private scratch config, the widening of architecture
    `decision/claude-first-no-adapter-trait` — his own word, on the card, no halt.
  - The report's departures (inputs#I7 item 7) — the stale `[NN/23]` `LIVE_PASS` literal the re-run sweep found after
    the block was green, the re-fire of entries 1, 2 and 4, the report landed twice (rev 2) — are the reconcile's to
    judge, not re-argued here.
- **Process hygiene, re-measured:** `gate.py census --comm viola-fake-agent --comm viola-live-rig --comm viola-harness
  --args projects/viola/target/debug/viola --args projects/viola/target/harness/debug/viola` (this run dir):
  `census: none running`.
