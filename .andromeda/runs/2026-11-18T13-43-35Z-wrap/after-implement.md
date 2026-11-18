## After implement
- **Report:** 2026-11-18T13-24-03Z-implement — the newest implement run of 2026-11-18-session-folder-trusted-by-viola
  (`rec.py runs … --file '*-2026-11-18-session-folder-trusted-by-viola.json'`, its last run line), its outcome `green`;
  it resumed 2026-11-18T12-36-09Z-implement (outcome `stuck`, entry 14 `refused: cap`) under plan revision 1 (phase
  run 2026-11-18T13-18-04Z-phase). Two implement runs and one plan revision, closed from their outcome files, the
  ledger and `evidence/`.
- **Commits:** since last_wrap 2026-11-18T09:48:02Z (the `state` view): `2cbaa752
  feat(2026-11-18-trust-record-row-read-live): the relied trust-record row verified live on 2.1.325 (S0B, 23/23), each
  interactive verify run's first screen named` (the previous wrap's commit, this chunk's base) · `f19b5cd5
  chore(2026-11-18-session-folder-trusted-by-viola): operator pre-CI commit, for the run this chunk's verdict reads`.
  No commit after the pre-CI one.
- **Operator pass:** `evidence/operator-pass.md` (written 2026-11-18T13:41:56Z): 16 `agent-run.sh boot --session
  trust-smoke` green (`"ok":true`, two instances; role lines `trust-record` `written` ×1 and `already-trusted` ×1 —
  the two wrappers share one folder) · 17 `status` green (`"state":"ready"`) · 18 `cleanup` green (`processes_gone` ·
  `endpoint_gone` · `home_removed` true, `killed` empty) · 19 `pre-push` green (`"ok":true`, coverage 3608 passed · 0
  failed, gate 0 breaches) · 20 `gate.py hygiene` `hygiene: clean` · `rec.py land` tx 2727 and the pre-CI commit
  `f19b5cd5` · 21 the push green (`history moved: refs/remotes/origin/build/viola-0.2.0 2cbaa752→f19b5cd5`) · 22
  `ci.py conclusion --sha HEAD --wait 1800` — `f19b5cd5224c verdict: green · checks 7/7 · wall 567 s · runs
  ci#44314599335 completed/success` (fuzz-replay · lint · msrv · perf · release · supply-chain · test). The pass's end:
  as-left green, HEAD `f19b5cd5` = origin.
- **Changes after implement:** none — no commit after the pre-CI one (`git log --format='%h %s' 2cbaa752..HEAD` lists
  `f19b5cd5` alone); no directed edit of the wrap's.
- **Inputs:** `inputs.py verify` (this run's trail `inputs-2026-11-18-session-folder-trusted-by-viola.json`): `inputs:
  7 entries — unchanged 6 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 1 · uncited 1 ·
  unparsed 0`.
  - I1 · ../additional/viola-overseer/phase-session-folder-directive-3.md · copy no-repo · unchanged
  - I2 · ../additional/viola-overseer/relay-session-folder-p3.md · copy no-repo · unchanged (the founder's P3 answers
    relayed by the overseer, 2026-11-18T11:39:23Z: the cap, keys none, F-T1 «Ставит флаг, остальное не трогает»)
  - I3 · ../additional/viola-overseer/implement-session-folder-directive.md · copy no-repo · unchanged
  - I4 · message: the operator (the /andromeda-phase invocation) · copy message · n/a — a message has no live source
  - I5 · ../additional/viola-overseer/relay-session-folder-revision1.md · copy no-repo · unchanged
  - I6 · ../additional/viola-overseer/implement-session-folder-directive.md · copy no-repo · unchanged
  - I7 · ../additional/viola-overseer/wrap-session-folder-directive.md · copy no-repo · unchanged — snapped `--step
    wrap` by this run (printed `UNCITED` before this close; cited here): the overseer's wrap directive. What it changes:
    the operator is away, so a fork goes to the overseer as a dialog; an answer card whose every row lies inside the
    founder's own recorded answers (R2 «Файл ребёнка, ключ как у CLI» and R3 «Блокировка, замена, проверка»,
    2026-11-17T18:59:41Z, `split-trust-route-adaptation.md`; F-T1, inputs#I2) is confirmed by the overseer; a widening
    outside them takes its most conservative option and is written to `founder-forks.md`; route-resolve asks the
    overseer in one dialog; push and read CI.
- **Scope:** none appended by this wrap — `gate.py scope` (this run): `scope: clean — changed 29 · listed 26 · recorded
  3 (companion 3 · mechanical 0 · in-intent 0 · widening 0) · absorbed 0 · excluded 245`; the ten test-home files read
  `record: listed` (in the plan's file list since revision 1).
- **Sweep reach:** `cascade.py sweep --pre-pass` (this run; patterns `.andromeda/runs/2026-11-18T13-43-35Z-wrap/report-patterns.toml`,
  13 patterns). First firing exit 3: `evset`, `startword`, `rootrec`, `trustevent` — `control UNFIRED over the seven
  masters (bodies and sets) at f19b5cd5`; that refusal (trail `cascade-2026-11-18-session-folder-trusted-by-viola-pre-pass.json`)
  is the basis of each one's master zero. Re-fired with the other nine (exit 0, listing
  `sweep-pre-pass-1.1.txt`). Per moved fact:
  - `evset` — the closed obs event set 15 → 16 (`ObsEvent::ALL`, `LOG_FORMAT_EVENTS`, both schema enums): 0 — no master
    states the count (also `rec.py find --doc obs-plan --regex '(15|fifteen|16|sixteen)\b'`: 0 hits).
  - `roleline` — the start's role-line count per `viola run` +1: standing 20 (architecture 6 · security-plan 1 ·
    test-plan 2 · obs-plan 11) · 20 no-change (each a true claim sharing the token: the session-start and
    process-start lines, the panic role line; no master counts a start's lines).
  - `startword` — the new closed set of 16 start words (`StartWord`): 0 — new fact; its homes are P2 adds
    (architecture §Standard Contracts beside `trust-writer-words`, security-plan §Input Validation).
  - `ownrecord` — Retired: «run and revive write no trust record»: standing 3 (architecture 1 · security-plan 2) ·
    `architecture.md:216` dec-scgx6or6 `decision/verify-own-trust-record` stale · `security-plan.md:374` dec-sjrilbtp
    and `security-plan.md:388` dec-pzgvqgc3 («verify's own trust record» as the one write under the Claude config)
    stale-candidate — owner architecture, security-plan.
  - `rootrec` — Spec claims disproved: the fake's oracle «inside a repository only the root's record counts»: 0 — a
    test-only oracle, no master site.
  - `trustwriter` — Expected: security-plan §Data Protection (the start's flag-only write beside verify's): standing 5
    (security-plan 3 · design-system 1 · obs-plan 1) · `security-plan.md:117` dec-qfrs67xc and `:382` dec-tolctq2d
    add-to (the writer now also run's start, flag only) · `:278` dec-cqckpo7v, `design-system.md:184` ctr-g5ib52tk,
    `obs-plan.md:385` dec-r6ir5lzb no-change (verify's stamped row) — owner security-plan.
  - `ccdir` — Expected: §Secret Management, architecture §Occupied Resources / §Cross-cutting / §Conventions
    (`CLAUDE_CONFIG_DIR`'s value now read at the start too): standing 14 (architecture 6 · security-plan 5 · test-plan
    1 · obs-plan 2) · 10 stale («read by verify only»: architecture :323 dec-u6qgqlqe, :699 dec-wjypwipg, :700
    dec-ysr3fsif, :935 dec-u3r3pgro; security-plan :497 dec-rlatwaem, :498 dec-rlsdenig, :507 dec-nussn6nn, :511
    dec-opfbpjfx; obs-plan :171 dec-6q6dunkg, :1041 dec-4ttxubuj) · 2 add-to (architecture :652 ctr-amls3qzn, :776
    ctr-szg56kho) · 2 no-change (security-plan :503 the rig's private config, test-plan :1376 the test home) — owners
    architecture, security-plan, obs-plan.
  - `runorder` — Expected: architecture `run-start-order` (the trust step after the version gate): standing 1 ·
    `architecture.md:179` dec-wx7tk6ur add-to — owner architecture.
  - `keypresent` — Expected: §Standard Contracts (`key-present` verify's only; new start-word rows): standing 1 ·
    `architecture.md:660` ctr-5vzvic7s add-to — owner architecture.
  - `snapcwd` — Expected: `instance-snapshot/cwd`'s second use (the revive's record key): standing 6 (architecture 3 ·
    security-plan 3) · `architecture.md:608` ctr-ywc67zqh and `security-plan.md:261` dec-ot35rizl add-to · architecture
    :65, :300, security-plan :17, :320 no-change (the revive's spawn dir and its refusal) — owners architecture,
    security-plan.
  - `trustevent` — Expected: obs-plan §1 / §4 / §6 (the `trust-record` event, no span): 0 — new fact, owner obs-plan.
  - `fakewalk` — Expected: test-plan §7 (the fake's parent walk): standing 6 (architecture 3 · test-plan 3) ·
    `test-plan.md:841` ctr-kxqcdc74 (the fake's trust-config key rule) stale · architecture :70, :967, :1037 (the
    Claude-shape exception) and test-plan :845, :886 no-change — owner test-plan.
  - `lockpause` — Expected: a11y-plan §8 (the start's lock pauses, at most 2 × 100 ms): standing 10 (architecture 4 ·
    test-plan 4 · obs-plan 1 · a11y-plan 1) · `a11y-plan.md:258` dec-2ghjvk3t add-to-candidate · the other 9 no-change
    (other bounds: the hook's channel wait, send's gate bound, wait, the 100 ms poll, the stamp lock, span attributes)
    — owner a11y-plan.
  - The leaf rows (42 over 16 files) and curation rows (3 over 2) are the cascade's (P2 step 3) and curation's.
  Sweep reach: 9 patterns · 13 moved facts, 13 with a pattern (the report's own count; 4 of them zero by the
  control refusal) · by line 0 lines · 0 offsets.
- **Outcome, final:** implement's Outcome stands, its basis now the final HEAD: `f19b5cd5` = origin, CI ci#44314599335
  green 7/7 on `f19b5cd5224c` (`evidence/operator-pass.md`, `evidence/operator-entries.txt`); the operator entries
  16–22 green (above), which implement's Outcome left «the operator pass's, after this report». The live proof as
  ledgered (`evidence/live-starts.ndjson`): two starts in one ledger, the cap spent exactly — `S1` 2026-11-18T13:13:26Z
  (2.1.325 by path; its own live verify `23/23`, the floor held; `viola run` in a fresh untrusted folder →
  `first-screen: box`, `run-trust: written`, `entry-flag: true`, `sentinel: equal`; `keys: none`, `stop: ctrl-c 4`)
  and `S2` 13:24:46Z (a plain `claude`, the same folder shape → `first-screen: trust`, `entry-flag: absent`; `keys:
  none`, `stop: killed-no-key`, `user-config: unchanged`); the spare `S1-2` unspent. Claimed for 2.1.325 only;
  nothing for 2.1.326 / 2.1.327. The stop between the runs, as it was: entry 14's first firing read `refused: cap`
  before its ledger line — a PLAN defect (entries 12 and 14 on one ledger at `--cap 1` each, `--cap` counting the
  file's base starts), approved at P5, mended by revision 1 (`--cap 2 --spare 0`); no start was spent on it. Entry 15
  fired by hand at 13:25:09Z (`census: 0`, exit 0): the gate tool, asked with `--only 15`, printed `not run — leg
  round (the letter drives it)`. The `needles: 1` line of each summary (`evidence/live-starts.md`) is no config entry:
  read in `crates/viola-e2e/src/rig/mod.rs` (`finish`, `summary.put("needles", needles.len())`), it counts the marks
  the rig searched the user config's moved bytes for — the canonical scratch root, plus the child's session id when
  one was read — so `1` is the path mark alone. The ONE entry of the operator's real config keyed under a test
  scratch root is the separate witness of Deviation 1 (`1 of 53` before the gates, `1 of 53` after both block firings,
  `1 of 54` after `S1`): standing before this chunk's gates and unchanged after. Which earlier path wrote it is
  unmeasured; from the tree alone the one product path that leaves such an entry is a `viola verify` ended by SIGKILL,
  a crash or a failed handler install, which leaves its own trust record in the Claude config (security-plan
  §Authentication & Authorization, the verify residuals); the earlier chunks' live rounds' two `projects[…]` entries
  stand on the operator's desk list. Nothing is removed from his config. The kept failed test home
  `target/e2e-home/viola-test-YKYwsZ/` still stood at this wrap's read (`ls`, before 2026-11-18T13:47:07Z): implement's «the next test home's
  sweep removes it» did not hold by it — on the desk list (handoff). Watches: none folded.
- **Decisions & corrections after implement:**
  - inputs#I7 (the overseer's wrap directive): the operator away; forks to the overseer as a dialog; an answer card
    inside the founder's own recorded answers confirmed by the overseer, PROVISIONAL until his word.
  - Curation candidate (inputs#I7 item 4, for the reconcile and P3 to judge): a test that starts the product never
    runs under the runner's real `HOME` once the product writes into a user config — measured here: the start's trust
    step reached every stamped test start, and ten files beyond the first plan were needed to keep the operator's real
    `~/.claude.json` untouched (Deviation 1).
  - A directive premise corrected from the tree: «the `needles: 1` entry — ONE entry of the operator's real config
    pointing at a test folder» — `needles` is the rig's mark count, not a config entry (Outcome, final).
  - Sweep hazard: the rig's summary word `needles` reads like a count of findings in the config, but it is the count
    of marks searched for (`crates/viola-e2e/src/rig/mod.rs` `finish`).
- **Process hygiene, re-measured:** `gate.py census --comm claude --comm viola --comm viola-live-rig --comm
  viola-fake-agent --args projects/viola/target/debug/viola` (this run's trail
  `census-2026-11-18-session-folder-trusted-by-viola.json`): `census: 16 running` — the overseers, the prototype
  builders and their `claude` children, the product `viola wait`, a `viola mcp`, a `claude --continue`; none of them
  this repository's `target/debug/viola` or a 2.1.325 `claude`.
