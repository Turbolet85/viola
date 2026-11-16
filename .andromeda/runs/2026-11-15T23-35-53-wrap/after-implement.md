## After implement
- **Report:** 2026-11-15T22-51-16-implement — the newest implement run of 2026-11-15-api-error-turn-end (`rec.py runs
  … --file '*-2026-11-15-api-error-turn-end.json'`, its last run line), its outcome `green` (`outcome-2026-11-15-api-error-turn-end.md`:
  «green + smoke»).
- **Commits:** since `last_wrap` 2026-11-15T20:06:27Z (the `state` view): `aa6bb5ab test(2026-11-15-mutation-and-coverage-baselines-green):
  mutation and coverage baselines green` (the prior wrap's, the base) · `7fd86141 chore(2026-11-15-api-error-turn-end):
  operator pre-CI commit, for the run this chunk's verdict reads`. Base `aa6bb5ab`, the parent of the oldest pre-CI
  commit (`git log --format='%h %s' aa6bb5ab..HEAD`: `7fd86141` alone).
- **Operator pass:** ran, in implement's window (`evidence/operator-pass.md`, written 2026-11-15T23:24:20Z). Pre-push
  on the uncommitted tree exit 0 (coverage 3318 passed); entry 19 `gate.py hygiene` → `hygiene: clean — read 229`;
  `rec.py land` → tx 1657; the pre-CI commit `7fd86141`; entry 20 the push → `green · exit 0`
  (`refs/remotes/origin/build/viola-0.2.0 aa6bb5ab→7fd86141`); entry 21 `ci.py conclusion --sha HEAD --wait 1200` →
  `7fd86141a732 verdict: red · checks 7/7 · first-fail +713 s test (ubuntu-latest)` · `runs: ci#43773402286 push
  completed/failure` · `failed 1: test (ubuntu-latest) (failure)`. The one failure: `TIMEOUT [ 20.013s] ( 726/3318)
  viola::path1_run_start path1_run_start_one_home_in_order` (`3318 tests run: 3317 passed (16 slow), 1 timed out`),
  a test outside this chunk's diff. Pass end: **red — unfixed, cause not measured**; no fix commit. The failed job was
  re-run ONCE by the overseer at 2026-11-15 23:35Z for this chunk's verdict only (`inputs#I4`, item 1) — its attempt
  read at P7's gates step.
- **Changes after implement:** none — no commit after the pre-CI one (`operator-pass.md` §Commits after the pre-CI
  commit: «none»); no directed edit by this wrap.
- **Inputs:** `inputs.py verify` (this run dir): `I1 · ../additional/viola-overseer/phase-api-error-turn-end-directive.md ·
  copy no-repo · unchanged` · `I2 · message (the overseer, delegate, on the founder word of 2026-11-15T21:54:53Z) · copy
  message · n/a` · `I3 · message (the overseer, delegate, the P5 word) · copy message · n/a` · `I4 ·
  ../additional/viola-overseer/wrap-api-error-turn-end-directive.md · copy no-repo · unchanged` — snapped `--step wrap`
  by this wrap, cited here (verify printed it `UNCITED` before this close existed). Summary `inputs: 4 entries —
  unchanged 2 · drifted 0 · vanished 0 · broken 0 · altered 0 · unreachable 0 · n/a 2 · uncited 1 · unparsed 0`; no
  DRIFT. `inputs#I4`, the overseer's wrap directive, changes: (1) the CI red on `7fd86141` is ruled NOT this chunk's
  and stays OPEN, pinned as a CARRY on «`wait` wake condition» (its measurement below, **Outcome, final**); (2) F1 and
  F2 stay PROVISIONAL, the founder away — the away form; (3) the carries; (4) the desk list's additions; (5) the last
  card's contents.
- **Scope:** none — `gate.py scope`: `scope: clean — changed 18 · listed 18 · recorded 0 (companion 0 · mechanical 0 ·
  in-intent 0 · widening 0) · absorbed 0 · excluded 229`; no line appended.
- **Sweep reach:** `cascade.py sweep --pre-pass`, `report-patterns.toml` (this run dir), trail
  `cascade-2026-11-15-api-error-turn-end-pre-pass.json`; the first firing refused `witness`, `fakeagent` (`control
  UNFIRED over the seven masters … at 7fd86141`) — that refusal is the basis of each one's master zero; re-fired with
  the other six.
  - registered hook events 9 → 10 — `hookcount`: standing 3 (security-plan 1 · test-plan 1 · obs-plan 1): security-plan
    `dec-pdqlfqgk` «a closed set of nine events» stale · test-plan `ctr-iwnzbqin` «over the nine registered hook events»
    stale · obs-plan `ctr-ahargk56` «one of nine closed hook names» stale; 1 leaf (`services/viola-agent-claude.md:25`
    «9 registered events») stale, the cascade's.
  - `turn-ended` data keys 2 → 3 — `tekeys`: standing 4 (architecture 2 · test-plan 2): architecture `ctr-ydq6fc5q`
    (`turn-ended-last-assistant-message`) and `ctr-yczznxmn` (`turn-ended-background-tasks`) — the contract's home,
    gains `cause` · test-plan `ctr-gyhkfi7n` «data holds the keys last_ass…» stale · test-plan `ctr-ichsmmsx`
    (background-tasks) a true claim sharing the token; 3 leaves (`rules/events.md:20`, `services/viola-agent-claude.md:25`,
    `tests-summary.md:27`) the cascade's.
  - the every-event witness counts (hook receipts 7 → 8, event lines 9 → 10, hook lines 7 → 8) — `witness`: master
    zero, by the control refusal; no master site — a test file's internal count.
  - Expected amendment 1 (turn-ended `cause`, architecture + test-plan): `tekeys` above; owner architecture
    §Standard Contracts, test-plan `ctr-gyhkfi7n` beside it.
  - Expected amendment 2 (`contract/hook-to-kind-map/stop-failure`, `TurnEndCause`'s home): `hkmap` standing 11
    (architecture 11 — the ten-event map's eleven rows, `hook-statusline` … `userpromptsubmit`; no `stop-failure` row:
    an add) · `turncause` standing 17 (architecture 1 · security-plan 7 · design-system 1 · test-plan 6 · obs-plan 2) —
    every one an `UnreachableCause` / exit-21 closed-cause site, a true claim sharing the token; `TurnEndCause` itself
    0 sites: an add. Owner architecture.
  - Expected amendment 3 (security-plan «nine events» → ten; `hook-event-revalidation` gains `cause`): `hookcount`
    (security-plan `dec-pdqlfqgk`) · `revalid` standing 1 (security-plan `ctr-rcfvoza5`, owner). Owner security-plan.
  - Expected amendment 4 (obs-plan «one of nine» → ten): `hookcount` (obs-plan `ctr-ahargk56`); owner obs-plan.
  - Expected amendment 5 (`decision/capability-ledger-gate` named `except`, PROVISIONAL F1): `capgate` standing 3
    (architecture 3): `dec-udlphze4` the owner row · `dec-ucvfrmxd`, `dec-wqz4223i` true claims sharing «no ledger
    row». Owner architecture.
  - Expected amendment 6 (the `cause` word crossing into `events.ndjson`, `wait --json`, MCP `wait`, PROVISIONAL F2):
    `tekeys` (architecture) — owner architecture §Standard Contracts with security-plan §Data Protection; no
    security-plan hit for its pattern (`tekeys` per master: architecture 2 · test-plan 2).
  - Expected amendment 7 (the fake agent's registered events ten): `fakeagent` master zero by the control refusal;
    `hookcount`'s test-plan site `ctr-iwnzbqin` stands nearest. Owner test-plan.
  - `Sweep reach: 6 patterns · 10 moved facts, 10 with a pattern · by line 0 lines · 0 offsets` — `6` from the
    re-fired call's `total (6 patterns)` line; `10` the report's own count (3 counts moved + 7 expected amendments, each
    naming a pattern of the eight written); `by line · 0 lines · 0 offsets`.
- **Outcome, final:**
  - supersedes «the CI half is the operator pass's (`ci.py conclusion`), not yet read»: read — `7fd86141` ci#43773402286
    `verdict: red`, the one failure `path1_run_start_one_home_in_order` TIMEOUT at the 20 s `ci` kill. **red — not
    this chunk's:** basis — the test lies outside this chunk's diff and fires no StopFailure; on CI the same test runs
    in a slow mode on trees without this chunk (`c06d1e5c` 15.963 s, `aa6bb5ab` the base 19.189 s, 0.8 s under the
    kill; `operator-pass.md` two-sided table); measured by the implement window, read-only, in a scratch clone at
    `7fd86141` under `cargo llvm-cov nextest --profile ci` (20 runs): the test's six steps take ~1.35 s in every run,
    and ALL the extra time sits in the `stamped_home` fixture's `viola verify` (`tests/support/verify.rs:330`, `within:
    None`) — ~5 s normally, 10.3 s, 15.4 s and past the 20 s kill as the host's 1-minute load rose from 6.5 to 153 —
    the fixture's unbounded wait runs inside the test's measured time (`inputs#I4`, item 1; why the RUNNER was slow is
    not measured) → owner: a CARRY on «`wait` wake condition», taken at its P1 as an `[inferred]` item (the
    stamped-home fixture's `viola verify` leaves the test's measured time, or the class's `ci` kill is sized to it).
    The red stays OPEN; the re-run's attempt gives the chunk its verdict only.
  - The acceptance «every entry green, pre-push included, and CI on the pushed HEAD `verdict: green`»: the local half
    met; the CI half rests on the re-run attempt read at P7 — a red second attempt on the same test alone is the same
    open red, and the wrap stops before any flip (`inputs#I4`).
  - Every other line of implement's Outcome stands — no fix moved a criterion (no commit after the pre-CI one).
  - watches: none.
  - Outcome basis: Setup 4's commit list (`7fd86141` alone past the base), `evidence/operator-pass.md` (the pass record,
    the final HEAD's CI run ci#43773402286), `inputs#I4`; implement's report for what only it holds.
- **Decisions & corrections after implement:**
  - The overseer's wrap directive (`inputs#I4`, item 1): the CI red on `7fd86141` is not this chunk's — the time is the
    stamped-home fixture's `viola verify` inside the test's measured time, on trees without the chunk too — and stays
    OPEN on a named owner (a CARRY on «`wait` wake condition»); one re-run of the failed job, for this chunk's verdict
    only, never closing the timing red.
  - `inputs#I4`, item 2: F1 and F2 (`founder-forks.md`) stay PROVISIONAL, the founder away (asleep): the away form —
    the escalation written to its file, a halt for his word taking the delegate's answer by relay, PROVISIONAL, the most
    conservative option; nothing ratified.
  - `inputs#I4`, item 4: the operator's desk list gains `target/live-stopfailure-20261115/`,
    `target/live-stopfailure-20261115-s2/`, `~/.cache/viola-path1-measure` (13 GB), and the note that live start 1's CLI
    rewrote `~/.claude.json`.
  - The operator pass (`operator-pass.md`): it stopped at the red with the cause unmeasured, citing «a red stays open
    until its cause is known» and the never-raise-a-bound rule; the measurement was the implement window's, after the
    pass, carried by the directive.
- **Process hygiene, re-measured:** `gate.py census --comm viola --comm claude --comm viola-fake-agent --comm
  viola-harness --args projects/viola/target` (this run dir) → `census: 15 running`, none under this repository's
  `target/` (the overseer sessions, the viola-lab prototype wrappers and their `claude` children, the product's `viola
  wait`, `~/.viola/bin/…/viola mcp` — the operator's).
