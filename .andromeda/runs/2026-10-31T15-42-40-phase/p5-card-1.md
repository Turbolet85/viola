# P5 review — 2026-10-31-exit-cause-code-catalogue

**Plan:** `viola-0.1.0/chunks/2026-10-31-exit-cause-code-catalogue/plan.md` · **Research:** `research.md` beside it · **Controls:**
`.andromeda/runs/2026-10-31T15-42-40-phase/p5-controls.md`. No word has been asked for or given on this plan before this card.

## Goal
Every cause behind exit 21 and exit 1 reaches a machine reader as its own closed code, matching the human hint of that cause, on
every surface it can reach. No exit moves, no refusal moves, and no refused check gains a line.

- **Exit 21** keeps `error` and takes the cause in `detail`: `{"v":1,"error":"instance-unreachable","detail":"<cause>"}`, MCP the same
  less `v`, its text still `refused: instance-unreachable`, `v` still 1. The five words: `strict-modes-failed` · `instance-dead` ·
  `unwrapped-name` · `endpoint-lost` · `server-verify-failed`.
- **Two owed hints land:** the unwrapped name (no live endpoint and no instance directory) and the endpoint lost mid-call.
- **Exit 1:** a stale holder leaves `already-live` for `stale-heartbeat`. `revive --json` prints a document for every refusal of the
  start it shares with `run`. The `cli` catch site under `--json` prints the `internal-error` document. `verify`'s refusals write their code
  on the `process-exit` they already write.
- **The table:** `tests/cross_exit_causes.rs`, one negative per cause, a mode per case, failing on a hint two codes share.

## Top acceptance criteria (12 in all)
1. `verification-matrix.json#v1-18`: each cause reaches its code on every surface it reaches, named apart where a ruling or the surface
   says so.
2. `verification-matrix.json#v1-39`: its gap closed: `revive --json` prints one document, exit 1, stderr empty, for `batch-script-child`
   (Windows), `pinned-hash-mismatch`, `squatted-name` and a `claude` not on `PATH` as `internal-error`.
3. No refused check gains a line or moves: a widened instance or home tree opens no log, writes no line and leaves its mode as found.
4. Every new word passes `is_code_word`, holds no path, pid or upstream text, and is a row of `tests/contract_error_bodies.rs`.
5. `hook` is unmoved: its fail-open matrices stay exit 0, empty streams.
6. The table is green beside `no_two_codes_share_a_hint` and its planted-duplicate control; no hint names `release`.

## Touchpoints
1 new file (`tests/cross_exit_causes.rs`), 40 modified: `viola-core` (`UnreachableCause`), `viola-state` liveness (`without_endpoint`),
`src/cmd/` client, the six no-endpoint arms, `list`, `plugin`, `run`, `revive`, `verify`, `src/main.rs`, `src/human/hints.rs`, `viola-mcp` result and
tools, `schemas/diag-line.v1.json` (six words), `Cargo.toml`, `.config/nextest.toml`, and the tests whose pins move. Nothing in `viola-channel`,
`fixtures`, `plugin`, `scripts` or `.github`; no dependency moves.

## Test commands (23 entries)
`gate.py baseline`: seen green 10 · new 10 · not seen 0 · leg 3. The ten new, each fired through the tool on the untouched tree at
HEAD `404f7496dbd7`:

| # | Entry | Baseline |
|---|---|---|
| 1 | `df -h /tmp` | report-only: tmpfs 32G, 9.3G used, 23G avail, 30 % |
| 8 | the integration binaries, 22 named | green, 348 passed |
| 9 | `--e2e`: MCP, contract, Path 1 | green, 77 passed |
| 10 | `--e2e`: `binary(cross_exit_causes)` | **red, predicted**: nextest refuses a filter naming the binary step 14 creates. Its literal's must-pass reading is entry 9 |
| 12 | `scripts/channel-tokio-free.sh` | green |
| 13 | the preservation guard, base `404f7496` | green, `guard: clean`. Clone controls below |
| 14–16, 19 | the harness smoke: clean, boot unstamped, status, cleanup | green; `processes_gone` and `endpoint_gone` true, nothing killed |

**The guard's clone controls** (HEAD `404f7496dbd7`): unmodified green; all 41 listed paths changed green; four outside paths
(`src/cmd/mcp.rs`, `tests/support/fake.rs`, `Cargo.lock`, an untracked file in `viola-channel`) red, `guard: outside 4 files`.

## What P5's reads changed in the plan
The checks found the plan wrong in places and I corrected it, so this card shows the corrected plan. The whole check set was re-run after the
last edit: fence parsed, `planlint` 0 hits, 41 list items, 0 unparsed citations.

- **A row that could not have passed (step 14).** `list --json` over a regular file at `diagnostics` reads **exit 0** with no `VIOLA_NAME`,
  because `list` opens no log without one (measured). The row now drives `wait --json`, which reaches the catch site (measured).
- **Two pins research had wrong (step 15).** `send_unreachable_name_exits_21_not_running` and `wait_last_unreachable_exit_21` stand on a fresh
  home, so after this chunk they read `unwrapped-name` and its hint, not the not-running hint. Research's sweep had the first one staying.
  Corrected, with a new `is not running` sweep. The not-running hint stays held by the table's stopped instance and
  `link_verbs_against_a_stopped_instance_exit_21`.
- **Step 13** left out `wait_session_start_hooks` (which `first_life` calls) and `listed` (the `session-live` row needs it).
- **Step 11** said `record` returns `Ended`; at HEAD it returns `u8`.
- **Step 9** left out `trusted`'s `?`.
- **Step 17:** the census reads the build pre-push leaves, so it runs after that entry.
- **A note added:** `.claude/rules/` still read `detail: null` until the wrap's cascade; on those clauses the plan and `inputs#I2` Q1 win.

Measured as the plan says: a fresh home holds no `instances/` after a refused `wait`, so `unwrapped-name` is reachable as Q2 reads it.

Validation-1: aligned with `scope.md` and the working entry; no scope amendment. Warnings: none. Check 11: the census's records show no
harm to the host. Check 12 (d): all six claims hold.

## What the classifiers read (`inputs#I1` item 2)
Effort high, model haiku, three draws; nothing read `max` and nothing read fewer draws. The manifest: 198 agents, 66 per draw.
- `classify: 198 agents · 195 pass · failed EVERY_ENTITY 2, STANDING_NAMED 1 · yes 497 of 942 (draw 1 449 · draw 2 443 · draw 3 446 · one draw only 46) · closing tags 4`
- `classify-r1: 3 agents · 3 pass · yes 28 of 49`
- `detail-2026-10-31-exit-cause-code-catalogue.json`'s six pick records each read `draws {asked 3, rendered 3, read 3}`.

The batch's line reads yes 497 and one-draw 46; the census below reads 496 and 45. I did not trace that difference of one.

## Deaths and re-spawns (item 4)
No draw died in any batch (`died: []`, six batches). Four agents took the contract's one re-spawn on a check, each passing it:
`arch-cls-occupied-resources.b1.d2` and `tests-cls-test-data-fixtures.b1.d2` (EVERY_ENTITY), `tests-cls-test-strategy.b2.d2` (STANDING_NAMED),
`design-cascade-g1` (BODY_IN_GROUP, BODY_UNDER_ENTITY; the re-check passing after the tool's `move`). No agent did the orchestrator's work.

## The sampler (forced, item 1)
`sample: yes · phase · FORCED by the operator`. The plan's `(sampler)` items stand named in it.
- `sampler: architecture: E carried 119 · the cascade lost 5 (cross-cutting-patterns#2.1, established-decisions#33.2, occupied-resources#21.1, stack-and-technologies#1.23, stack-and-technologies#1.3)`
- `sampler: security-plan: E carried 94 · the cascade lost 6 (data-protection#3.1, data-protection#8.1, data-protection#9.1, dependency-security#3.1, input-validation#1.21, secret-management#1.1) · listed 5`
- `sampler: design-system: E carried 28 · the cascade lost 1 (color-palette#4.3) · listed 2`
- `sampler: test-plan: E carried 138 · the cascade lost 6 (ci-integration#1.11, ci-integration#1.3, ci-integration#1.7, ci-integration#1.8, ci-integration#1.9, quality-gates-coverage-targets#2.0) · listed 3`
- `sampler: obs-plan: E carried 118 · the cascade lost 1 (ci-integration#2.5) · listed 4`
- `sampler: a11y-plan: E carried 60 · the cascade lost 0`
- layout-templates: whole, no cascade.

Census: entities 942 · classify-yes 496 · one-draw-yes 45 · false-yes 73 · lost-classify 8 · lost-strong 11 · listed 14 · standing 63.

## The store (item 8)
`promote` took one call, exit 0, tx 19. `rec.py check` read `check: clean` each time, re-read at this card: journal uncommitted past HEAD 2, store
fresh, on both families. No render moved but the route's own stamp. One guard refusal at research: a pipe of `rec.py` into `cut`, re-issued as
two calls.

## Slips in this P5 window
- I appended a stray `rm -f /dev/null` to a read-only command. The permission classifier denied the call, nothing was removed, and I re-ran the
  reads without it. It served no purpose.
- One `gate.py scope --lists-only` call lacked `--research`: a usage error, re-fired correctly.
- One read took line 1 of `references/evolve/validate.md`, the header, ahead of its checkpoint. No criterion was read.

## Concretization previews
**`verification-matrix.json#v1-18`** (security)
- **Current:** each cause of the unreachable-instance exit (not running, unwrapped name, strict modes, server verification, endpoint lost
  mid-call) and of the internal-error exit (already live, stale heartbeat, squatted name, pinned-hash mismatch, script child, strict-modes or
  socket-directory refusal, internal error) reaches the machine-readable output, MCP and the process log as its own code, carrying no path or pid
  and matching the human hint for that cause.
- **Proposed:** each cause reaches its own closed code on each surface it reaches, matching its hint, no path or pid.
  Exit 21: the five words above, in `detail`, on `--json` and MCP. Exit 1: `strict-modes-failed`, `already-live`, `no-session`, `session-live`,
  `cwd-missing`, plus `stale-heartbeat`, `batch-script-child`, `pinned-hash-mismatch`, `squatted-name`, `internal-error`, and `verify`'s log codes
  `program-not-found`, `version-unreadable`, `unclean-fixture`, `interrupted`.
- **Named apart, on the founder's own word (`inputs#I2` Q4):** a strict-modes check refused before a log opens writes no line; MCP reaches the
  exit-21 causes alone, writing no line, `isError` its record; `run` and `verify` have no `--json`, so their causes reach the log and the hint,
  `run`'s start also through `revive --json`; `viola mcp`'s exit 1 keeps both streams empty; macOS's server check and the socket-directory refusal
  stay with "Unix endpoint and home hardening".
- **What this leaves unmet:** the socket-directory cause has no code here, because the code holds no such refusal, and macOS has no server check.
  This narrows the requirement by his named carve-outs; it is not a reading of the requirement as met. A dated `matrix.py` note records it.
- **Method** stays integration; witnesses are in the plan's first criterion.

**`verification-matrix.json#v1-39`** (arch)
- **Current:** as it stands, revive's exit-1 document naming the five codes.
- **Proposed:** the acceptance as it stands with its gap closed: revive's exit-1 document also takes `stale-heartbeat`, `batch-script-child`,
  `pinned-hash-mismatch`, `squatted-name` and `internal-error`, and `instance-unreachable`'s `detail` is its cause. Its `ref` and `implemented`
  status stand, and the port clause stays out by its notes. No clause is narrowed.

## Whose word stands where
- `inputs#I2` Q1 and Q4: the founder's own, live. The wrap halts once on the answer card, under the playbook's "Boundary widening the founder
  answered himself to a P4 fork".
- `inputs#I2` Q2 and Q3: `delegate the overseer`, PROVISIONAL. The plan's expected amendments carry that mark.

## Beyond the plan
| Proposal | Owner |
|---|---|
| The socket-directory refusal's code, `v1-18`'s leg, is left out of this chunk | the existing entry "Unix endpoint and home hardening" |
