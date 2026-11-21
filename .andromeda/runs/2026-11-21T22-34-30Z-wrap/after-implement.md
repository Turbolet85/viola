## After implement
- **Report:** 2026-11-21T21-51-10Z-implement — the newest implement run of 2026-11-21-mcp-calls-bounded (`rec.py runs
  --kind '*-implement' --file '*-2026-11-21-mcp-calls-bounded.json'`, its last run line), its outcome `green`
  (`outcome-2026-11-21-mcp-calls-bounded.md`, «**green** (2 fix iterations)»).
- **Commits:** since `last_wrap` 2026-11-21T18:06:34Z (the `state` view): `53669b68 feat(2026-11-21-cross-session-prefix-ledger-row): …`
  (the prior wrap's own commit) · `261e48d5 chore(2026-11-21-mcp-calls-bounded): operator pre-CI commit, for the run
  this chunk's verdict reads` (the pass's one commit; `git log 53669b68..HEAD`).
- **Operator pass:** run in the implement window (`evidence/operator-pass.md`, `evidence/operator-entries.txt`): 30
  `agent-run.sh boot --session mcpbound-smoke` green · 31 `status` green (`"state":"ready"`, cli 2.1.328 verified) ·
  32 `cleanup` green · 33 `pre-push` green (92.76 s) · 34 `gate.py hygiene` `clean` · the pre-CI commit `261e48d5` ·
  35 the push green (`origin/build/viola-0.2.0 53669b68→261e48d5`) · 36 `ci.py conclusion --sha HEAD`: `261e48d5c13e
  verdict: green · checks 7/7 · wall 610 s · runs ci#44956459404 completed/success` (fuzz-replay · lint · msrv · perf ·
  release · supply-chain · test). The pass's end: as-left — green.
- **Changes after implement:** none — HEAD `261e48d5` is the pre-CI commit; no fix commit followed it.
- **Inputs:** `inputs.py verify` (this run dir): I1 unchanged · copy no-repo · `../additional/viola-overseer/phase-mcp-directive.md` ·
  I2 unchanged · copy no-repo · `../additional/viola-overseer/relay-mcp-p3.md` · I3 unchanged · copy no-repo ·
  `../additional/viola-overseer/relay-mcp-p5.md` · I4 n/a — a message has no live source · copy message · I5 unchanged ·
  copy no-repo · `../additional/viola-overseer/implement-mcp-directive.md` · I6 n/a — a message has no live source ·
  copy message · I7 unchanged · copy no-repo · `../additional/viola-overseer/wrap-mcp-directive.md` — snapped at this
  wrap (`--step wrap`), cited here (`inputs#I7`); `verify` printed it `UNCITED` before this close existed. Summary:
  `inputs: 7 entries — unchanged 5 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 2 · uncited 1
  · unparsed 0`. No DRIFT. What `inputs#I7` changes: the wrap runs whole with the operator away (a fork the overseer's
  dialog, an answer-card row inside the operator's own answers confirmed by the overseer, a widening outside them its
  most conservative option written to `founder-forks.md`); the rows resting on the 2.1.287 reading are re-based to
  what 2.1.328 / 2.1.330 read and what was measured; the paste's missing time bound is a FINDING pinned as a CARRY, its
  route entry a proposal in `founder-forks.md`, never minted; v2-24's flip states the claim with the
  reading/measurement line kept.
- **Scope:** none — `gate.py scope`: `clean — changed 12 · listed 12 · recorded 0`; this read appended no line.
- **Sweep reach:** `cascade.py sweep --pre-pass` (`.andromeda/runs/2026-11-21T22-34-30Z-wrap/report-patterns.toml`,
  trail `cascade-2026-11-21-mcp-calls-bounded-pre-pass.json`, listing `sweep-pre-pass-1.1.txt`), every master row
  `standing`:
  - `CLI287` (the 2.1.287 reading, the directive's item 4): control UNFIRED over the seven masters at 261e48d5 — no
    master holds `2.1.287`; that refusal, its trail record, is the master zero's basis. Its sites are leaves alone
    (`.claude/rules/api.md` §MCP, `.claude/docs/services/viola-mcp.md:24`), owed to the cascade and curation.
  - `CAUSESET` (the unreachable-cause set six → seven, `call-deadline`): standing 48 — architecture 9 ·
    security-plan 8 · design-system 3 · layout-templates 1 · test-plan 13 · obs-plan 13 · a11y-plan 1; leaf 16.
    Disposition by class: the rows naming the MCP set or its whole list (architecture dec-tywniq7h, ctr-wywpxiwa,
    ctr-x7v6ouwh; test-plan ctr-h6ck2jjq; obs-plan ctr-7mzj6yuo, dec-3vz4uby6, and each obs/a11y list row that names
    the MCP detail) → change (gain `call-deadline`, written by the MCP frame tools alone); the CLI exit-21 rows
    (design-system 3, layout-templates 1, the `exit-21`/`cli-wait`/`viola-wait`/`cli-exit-21`/hint rows) → no change
    (a true claim: the CLI set is unchanged, six); the per-member rows `instance-unreachable-cause/{member}`
    (security-plan 6, test-plan 6) → no change, a new `call-deadline` member row owed beside them. Owner masters:
    architecture (cause-set), security-plan, test-plan, obs-plan.
  - `TOKTIMER` («viola-mcp runs no tokio timer»): standing 1 — test-plan 1 (decision/time-faked-through-seams) →
    change (the MCP deadline an injected seam, `DeadlineProvider`, no paused Tokio clock); leaf 1
    (`.claude/rules/testing.md`:28) owed to the cascade. Owner: test-plan.
  - `MBARM` (the rig's `MB` arm beside `AV`): standing 1 — security-plan 1 (decision/live-rig-mcp-arm-keeps-no-output)
    → no change to it; a new row of its own owed beside it (P4-F2). Owner: security-plan. Leaf 2.
  - `WAITCAP` (the wait cap's why): standing 2 — architecture 1 (decision/mcp-wait-cap → change: re-based) · a11y-plan
    1 (decision/time-bounds-fall-on-the-driver → add-to: the MCP call deadline as one added member). Leaf 3
    (`.claude/rules/api.md`:35, `.claude/docs/a11y-summary.md`:53, `.claude/docs/services/viola-mcp.md`:24).
  - `SENDBOUND` (send-time-bound names the paste unbounded): standing 2 — architecture 1 (decision/send-time-bound →
    change) · test-plan 1 (ctr-ixbgkymv «one write_all per line» → no change: a true claim sharing the token). Leaf 4.
  - `DEADLINE` (the new MCP call deadline row): standing 2 — architecture 2 (the same two rows as WAITCAP and
    SENDBOUND) → change; the new decision row owed in architecture. Leaf 2.
  - `TIMEBOUNDS`: standing 2 — a11y-plan 2 (decision/time-bounds-never-on-human, decision/time-bounds-fall-on-the-driver)
    → add-to: «the MCP call deadline» one added member each. Owner: a11y-plan.
  - `MCPISERR`: standing 24 — security-plan 12 · test-plan 7 · obs-plan 5. Change: security-plan
    contract/mcp-iserror/instance-unreachable, obs-plan contract/mcp-iserror/instance-unreachable,
    decision/mcp-unreachable-carries-one-cause-word (the cause list gains `call-deadline`); no change: the other
    `mcp-iserror/*` kinds and the per-member rows (a true claim). Owners: security-plan, test-plan, obs-plan.
  Sweep reach: 8 patterns · 12 moved facts, 12 with a pattern (the report's own count: 3 Counts / qualifiers moved, 8
  Expected amendments, 1 directive re-base; `CLI287` refused unfired, its zero the refusal) · by line 0 lines · 0
  offsets.
- **Outcome, final:** implement's Outcome stands, and: supersedes «CI on the pushed HEAD — the operator pass's»: CI
  ci#44956459404 on `261e48d5` green 7/7. The outcome basis: Setup 4's commit list (`261e48d5`, no fix commit), the
  final HEAD's CI run in `evidence/operator-pass.md`, the pass record. matrix#v2-24 rests on the three live evidence
  lines (`evidence/mcp-starts.ndjson`: `MB` 2.1.330 and `MB8` 2.1.328 — `call-error: false` · `timed-out: true` · 50 s ·
  `at-cap`; `MC` 2.1.330 under a 5000 ms server `timeout` — `call-error: true` · `cut-word: tool-timeout` · 5 s ·
  `at-bound`; each `keys: none`, `census: 0`) and the unit cases of the compiled bound and its expiry green in CI.
  Said exactly (`inputs#I7` item 1): on two versions a real print-mode client let viola's longest call (50 s) return
  whole, and on one it applied a per-server bound as the bundle's strings read it; the DEFAULT bound's value (the hard
  limit, the idle limit, the auto-background figure) stays a READING of the strings per version, never measured; that
  `viola mcp` outlived the cut call is a reading from ONE start. Watches: none folded.
- **Decisions & corrections after implement:**
  - The pass: no correction or ruling (`evidence/operator-pass.md` «none»).
  - The wrap directive (`inputs#I7`, the overseer, the operator away): the answer card's rows inside the operator's
    own answers are cited by his words — «Вшитый дедлайн 55 с (Recommended)» (the compiled MCP-side deadline, its
    expiry a new closed result word) and «3 базовых + 1 запасной (Recommended)» (the rig's `MB` arm in the shape shown,
    the cap, the cut start); P4-F1 (`CallDeadline` one named member, written by the MCP client alone) and P4-F2 (a row
    of its own for the `MB` arm) stand on the overseer's delegate confirm inside those answers, PROVISIONAL; a row
    outside them is not folded but written to `founder-forks.md`, its most conservative option.
  - `inputs#I7` item 4: rows resting on the 2.1.287 reading are re-based, not left — the wait cap's why («below the
    client's smallest default, 60 s») says what 2.1.328 / 2.1.330 read (the 60 s figure floors the HTTP transports
    only) and what was measured; a leaf holds no figure the code states.
  - `inputs#I7` item 6: the long-call predicate needs `timed-out: true` beside `call-error: false`, so a `tool_result`
    with no `is_error` key (read `call-error: false`, Deviation 2) cannot turn a cut call green — said in the `MB`
    arm's row.
  - `inputs#I7` item 7: the paste's missing time bound (`crates/viola-pty/src/pump.rs`:77, one blocking `write_all`)
    is a FINDING of this chunk, not folded: its route entry is the operator's own and he is away — not minted; the
    proposal goes to `founder-forks.md`, the finding pinned as a CARRY. Carry `cry-ujze4atl` not met: re-pinned whole.
- **Process hygiene, re-measured:** `gate.py census --comm viola --comm viola-live-rig --comm viola-fake-agent --comm
  claude --args projects/viola/target/debug/viola` (this run dir): `census: 13 running`, none this repository's
  `target/debug/viola` nor this chunk's — the host's overseer sessions (4 `claude`), the viola-lab prototype's three
  wrappers and their `claude` children (6), the real home's pinned `viola mcp`, the product's `viola wait`, and one
  prototype `viola wait`.
