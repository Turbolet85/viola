# Cascade dispositions — 2026-11-14-budget-gate-removed

## Search

- **Pass:** `cascade.py sweep` pass 2 (`sweep-pass-2.1`…`.6`, read whole, all six in one message), baseline `e8d74060`
  (the parent of the oldest pre-CI commit `78c0c6de`), over the masters TX 1193–1206.
- **Pattern files:** the nine tool-written files of this wrap's `rows-land` dry runs —
  `cascade-patterns-{architecture,security-plan,design-system,layout-templates,a11y-plan,obs-plan}-1.toml`,
  `cascade-patterns-test-plan-{1,2,3}.toml` — and main's own `cascade-patterns.toml` (untouched: `main-gate`,
  `main-paused`, `main-exit11`, `main-budgetjson`, `main-relbudget`, `main-ratelimits`, `main-thresholds`,
  `main-listbudget`, `main-path6`, `main-anybudget` — the claim's own verbs and phrasings: the gate, the pause, exit 11,
  `budget.json`, `release --budget`, `rate_limits`, the thresholds, `list`'s budget, Path 6, any budget word).
- **The mend (inputs#I9, the pipeline owner's ruling relayed):** pass 1 (`sweep-pass-1.*`, 47 parts, 18 312 rows, ≈ 2.3 MB)
  was NOT read. Its volume was the tool's pattern writer taking a bare number as a distinctive term. Every term that is
  a number alone (digits, with an inner `.` or `,`) was dropped from the nine tool-written files by a script
  (`mend_patterns.py`, session scratchpad); a term holding a letter stayed (`5H`, `7D`, `4m`, `25th`, `10-14`,
  `10/11/12/14`); a pattern left with one term became `fixed`; byte copies of the nine as written stand beside them as
  `.raw-cascade-patterns-*.toml`. 23 patterns trimmed; 8 removed whole, each named with its row:
  - `arch1-1` (`0|100`) — `decision/nutype-domain-newtypes` r1
  - `design1-15` (`89\.6|89`) — `contract/viola-list-board/window-figure` r1
  - `layouts1-7` (`0|3339`) — `contract/release-budget-output/gate-lifted` r1
  - `layouts1-8` (`0`) — `contract/release-budget-output/gate-open` r1
  - `tests1-8` (`3`) — `contract/boot-readiness/events` r1
  - `tests1-32` (`11`) — `contract/cli-send-json/budget-paused` r1
  - `tests1-39` (`90|89`) — `decision/budget-pause-threshold` r1
  - `tests1-53` (`11`) — `contract/budget-paused-refusal/send-exit-11` r1

  Those eight rows' changed numbers were NOT swept: step 3's leaves read each such row's delta (the letter's own arm for
  a change that yields no term). What this stops catching: a leaf line that repeats a retired NUMBER and none of the
  row's names (a threshold «90», an exit «11»); main's own patterns carry the exit-11 and threshold phrasings.
- **Not looked for:** a retired mechanism stated without any term above (prose alone) — the leaves step's recompute
  reads each changed row's delta for it.

## Counts (copied from the pass's `total` and `per class` lines)

total (111 patterns) · 2014 rows over 32 files
per class · new 31/6 · standing 570/7 · leaf 1394/23 · curation 18/3 · base 1/1

Six patterns fired their control and hold 0 rows (`arch1-3`, `layouts1-6`, `obs1-11`, `obs1-23`, `tests1-14`,
`tests1-21`): a statement about each pattern, not the masters.

## Rows

- **new — 31 rows / 6 masters: no change.** Each is a row this wrap's TXs landed stating the removal as it now stands
  (`dec-pbuahe7x` security-plan «the hook reads no rate_limits and writes no file»; `ctr-4hxbzvo4` obs-plan «a retired
  five_hour or seven_day included, is counted as skipped»; `ctr-ruz4fmpt` the span chain; `abs-ywmx3ndp` and
  `dec-zejotuie` architecture; `ctr-pgbggvz3` design-system, `ctr-l3rx35ui` layout-templates, `ctr-jcrenivt` test-plan,
  `ctr-rd2qbhhd` security-plan, `ctr-6j5fsq7z` obs-plan — exit 11 retired, any `budget` key refused).
- **standing — 570 rows / 7 masters: no change**, by class:
  - `config.json` rows (`arch1-63`/`-64`, `security1-15`/`-16`, `a11y1-7`/`-8`, `obs1-19`/`-20`, `tests1-1`; 40 rows
    each, one set): the file stands; every hit is its name beside `claude_env_keep`, `diagnostics_level`,
    `dialog_hold_s`, `v` or the tolerant parse — none states a threshold. The `edited` ones re-read whole for an
    intra-line duplicate of the retired mechanism: `dec-wil5yqdq`, `dec-yq66jxhu` (architecture), `dec-gxrszfan`
    (test-plan), `ctr-4hztdocu`, `ctr-4hxbzvo4` (obs-plan) — each now names the dialog hold or the retired keys as
    skipped; none holds one.
  - `hook.dialog` rows (`arch1-40`/`-44`, 21): the method stands; no row says it reads the gate.
  - `RefusalReason` rows (`arch1-48`, 13): the type stands; `ctr-aagik35y` (obs-plan, `edited`) re-read — its detail
    list no longer holds `five-hour` / `seven-day`.
  - `File::lock` (`arch1-42`, 2), `exit_code` (`obs1-27`, 30), `isError:true` (`tests1-56`, 2), `events.ndjson`
    (`tests1-59`, 65; `ctr-7cycjmhp` and `ctr-z3syrfob` `edited`, re-read — the start order is `wheel` then
    `session-start`, the replay recovers `links`, `agent_session_id`, `wheel`), `statusline.shell_out` (`obs1-24`, 3 —
    the span stands, `src/cmd/hook/statusline.rs:182`; `ctr-5h77vuw2`'s boundary `budget span attributes` is a
    `label`), `\x1b[0m` (`design1-14`, 1 — the SGR reset, not an age): the named things stand, no budget claim.
  - `10-14` (`obs1-2`, `main-exit11`: `ctr-pgbggvz3` design-system, `ctr-k3gcg36a` test-plan): «a code from 10 to 14»
    stays true with 11 unassigned.
  - exit 11 retired (`main-paused`: `ctr-tf6njmtt`, `ctr-ntkdabbl`, `ctr-l4izdlfs`, `ctr-2go66yec`, all `edited`):
    each now states the retirement; re-read, no live use of the code stands.
  - `release` (`main-relbudget` `ctr-zfrhupy4`; `main-anybudget` `ctr-yjjehinc`, `ctr-5ipr6zzx`, all `edited`): each
    states `--budget` a usage error / any `budget` key `-32602` — the landed text.
  - other budget words (`main-anybudget`, `main-gate` `dec-alomvaka`, `main-thresholds` `dec-ath57xoh`): performance,
    latency, error, flakiness and dirty-page budgets and a seven-day artifact retention — another sense of the word;
    `dec-nkessa3q` / `ctr-ndwlucul` (design-system) «never used for errors, warnings or budget» — a prohibition that
    stays true with no budget figure on any surface; section and twin headings (`a11y-plan · 10-slo-invariants-a11y-budgets`,
    obs-plan twins) — `label`.
- **leaf — 1394 rows / 23 files: re-derived** — every leaf they name is in step 3's set (`cascade.py leaves` reads this
  listing); the comparers recompute each against the changed rows.
- **curation — 18 rows / 3 files: no change** — preserve-verbatim homes, each a dated historical learning that stays
  true as history (`2026-10-14` dates hit by `10-14`; `session-learnings.md:46` «exit 11 alone … budget-paused's exit
  11», `:106` `read_budget_stamped`, `:338` CLAUDE.md size budget, `:48` `RefusalReason`; `testing.md:76`
  `events.ndjson`). Nothing routed to P3.
- **base — 1 row: no change** — `rows:rule:51` cites chunk 2026-10-10-budget-governor as a rule's first case: a
  historical citation.

## Leaves

`cascade.py leaves`, the PASS mode (epoch «Epoch 2 — The budget gate and the status line leave» open, markerless 2):
15 groups · 27 leaves · deltas `leaf-delta-{doc}.diff` for all seven masters; one batch `leaf` (15 comparers, `check`
15 of 15 pass, `[PATH_CHECK] PASS`). The tool's line: `total · blocks 94 · applied 93 · held 1`.

- **Mode, every group:** pass. Groups with 0 findings: `architecture+design-system+layout-templates`
  (`.claude/rules/frontend.md`), `architecture+test-plan+obs-plan` (`.claude/docs/commands.md`),
  `security-plan+obs-plan` (`.claude/rules/ci.md`).
- **Applied by the tool (93):** a11y-plan 2 · architecture-1 16 · architecture-2 8 · architecture-3 18 ·
  architecture-4 4 · architecture+security-plan 8 · design-system+layout-templates 4 · masters 3 · obs-plan 7 ·
  security-plan 8 · test-plan 9 · test-plan+obs-plan 6. Source sides checked at their rows: the ledger's 23 rows
  (`crates/viola-agent-claude/src/ledger.rs:47`, `ALL: [Self; 23]`), `EventKind`'s 15 (`crates/viola-core/src/lib.rs:77`),
  `release`'s plain answer and «a `release` on a driver-held wheel changes nothing» (`ctr-tla4wlaw`, `ctr-yh3ke3xb`;
  `src/run/wheel.rs:197-208`). `design-summary.md:7#2` drops «`expired`» with the budget reading's age: the board's
  `expired` word stays named in `.claude/rules/a11y.md:22`.
- **By hand:** none.
- **Declined (1):** `leaf-test-plan.md/.claude/rules/testing.md:24#1` (omission, `dec-n5cm3kti` retired-word fixtures) —
  a detail that stays in its master; no reader of the leaf acts against the source without it.
- **Unmapped (2):** `.claude/rules/host-linux.md` — the host leaf, no source states it; `.claude/rules/testing-src.md` —
  a pointer list into `testing.md`, no statement of its own. Neither changed.
