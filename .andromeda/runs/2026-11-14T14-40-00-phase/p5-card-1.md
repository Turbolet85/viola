# P5 review — 2026-11-14-budget-gate-removed «Budget gate removed» (viola-0.2.0, Epoch 2)

**Open the plan:** `.andromeda/cache/docs/2026-11-14-budget-gate-removed/plan.md` (the render of
`doc:2026-11-14-budget-gate-removed/plan` rev 2). No plan stands in a chunk's folder; writing in that file changes no
record until a `put`. `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-14-budget-gate-removed`
is a diff of the chunk's records.

## Goal
Remove the budget gate whole, on the founder's ruling «Убрать совсем»: no rate-limits reading, no `budget.json`, no
`send` gate, no `budget-paused` refusal (exit 11 retired, never reused), no `budget-gate` event, no budget fields on
the snapshot, the board or `list`, no `release --budget`, no `statusline-rate-limits` ledger row (relied 24 → 23), no
Path 6 suite. The driver `release` guard stays first. Every `v` stays at 1 (Q1, a delegate's answer, PROVISIONAL,
`inputs#I3`); the bump rule's exception is escalated at the wrap for the founder's own word.

## Top acceptance criteria
- matrix#v2-04: the identifier probe (entry 11) exits 1 with no output; `send`'s order has no budget rung; `viola
  release --budget` is a clap usage error and a channel `release` carrying any `budget` key is `-32602` with no wheel
  moved (its control `holds`); `list` is `{items, skipped}`; 23 relied rows, none `statusline-rate-limits`; no refusal
  maps to exit 11; the config keys and the log schema hold no budget key (entry 12).
- Compatibility, the direction the tree can run: a new reader reads a 0.1.0-shaped snapshot and log
  (`snapshot_of_a_budget_era_writer_reads_whole_skipping_its_budget_fields`,
  `replay_over_a_budget_era_log_skips_its_gate_lines`); a written snapshot names no budget key. The old-reader
  direction is measured, not run (no 0.1.0 reader in the tree).
- A stamp holding the retired row reads on the 23 relied rows alone; the board's BAY line reads `BAY  skipped a - b -
  c`, WHEEL 23, no non-budget byte moved; a start writes `wheel{cause:"start"}` then the hook's `session-start`.

## Touchpoints
94 files (research.md «Files to modify», `scope lists: parsed 94 items`): 8 deleted (`src/run/budget.rs`,
`src/run/send/budget.rs`, `crates/viola-state/src/budget.rs`, `crates/viola-agent-claude/proptest-regressions/statusline.txt`,
`crates/viola-e2e/tests/path6_budget_harness.rs`, `tests/cli_budget.rs`, the two `StatusLine.rate-limits-1.json`),
the rest changed across viola-core, viola-state, viola-agent-claude, the wrapper and verbs, MCP, the fake agent, the
e2e harness, the root tests and censuses, three schemas, the fuzz target, two manifests' worth of lockfiles
(`Cargo.lock`, `fuzz/Cargo.lock`), `Cargo.toml` and `.config/nextest.toml`.

## Test commands
34 `[[gate]]` entries: fmt, clippy, G1, lint-probes, orphans-check, check; `cargo deny` on both lockfiles and the fuzz
lock read under `--locked`; two identifier probes; five nextest entries (core, state, agent-claude, the bin, the
censuses); the harness's unit / integration / e2e / fuzz-replay; release-check; the smoke boot / status / G2 / G4 /
cleanup; pre-push; and the operator pass's hygiene, push and `ci.py` read. One `gate.py control` at implement:
`release-budget`, case `release_carrying_a_budget_key_is_invalid_params_and_moves_no_wheel`. No live step, no
mutation run.

Baselines (25 entries fired on the untouched tree, `gate.py baseline` reading 0 NOT SEEN after): green 17, recorded 1
(entry 1), red 7 — each the red the plan predicts (entries 11, 12, 14, 15, 16, 27: the subject this chunk removes or
the cases it adds; read for cause). Entry 9 (green) has its known-positive control in
`.andromeda/runs/2026-11-14T14-40-00-phase/control-9.md`: red, exit 101, with `chrono` dropped and the fuzz lock stale.

## Concretization preview — matrix#v2-04
- **Now:** «An automated send is never refused on budget; no budget reading, budget file, paused state, release
  option, board field, ledger row or log field remains in the product or its tests; the exit status once used for a
  budget pause is retired and not reused.»
- **Proposed (concrete, never weaker):** «No budget identifier stands in `src crates tests schemas fuzz/fuzz_targets
  Cargo.toml .config` (the plan's identifier probe exits 1 with no output); `send` refuses in the order
  control-character, empty-text, human-typing, turn-running, the readiness gate, pinned by literal; `viola release
  --budget` is a clap usage error and a channel `release` carrying any `budget` key is `-32602` with no wheel moved; the
  `list` payload is `{items, skipped}`; the ledger relies on 23 rows, none `statusline-rate-limits`; no refusal maps to
  exit 11 (a literal table); `CONFIG_KEYS`, `schemas/config.v1.json` and `schemas/diag-line.v1.json` hold no budget
  key or field.» Method: integration.

## The directive's item-7 block (inputs#I1), verbatim where a tool printed it
- **The union, per master** (`rec.py rows-union`, printed):
  - `architecture: rows 994 · packs 10 · picked 180 (119 by every draw) · standing 15 · 48172 B`
  - `security-plan: rows 618 · packs 7 · picked 187 (110 by every draw) · standing 5 · 48408 B`
  - `design-system: rows 325 · packs 4 · picked 53 (20 by every draw) · standing 36 · 13617 B`
  - `layout-templates: rows 225 · packs 3 · picked 48 (22 by every draw) · standing 1 · 9863 B`
  - `test-plan: rows 1375 · packs 14 · picked 321 (184 by every draw) · standing 34 · 77228 B`
  - `obs-plan: rows 1060 · packs 11 · picked 152 (81 by every draw) · standing 10 · 38511 B`
  - `a11y-plan: rows 241 · packs 3 · picked 88 (51 by every draw) · standing 15 · 22594 B`
  - `rows-union: 7 master(s) · picked 1029 of 4838 · rule any`
- **Every row the plan cites** (31 handles: 26 by a script over the plan and the seven rows/standing files, 5 more by
  a direct grep of the handles the script's pattern missed across line breaks) stands in its master's rows file; none
  is in a `*-standing.md` alone and none is in neither.
- **`[RETYPED]`** (pick-p2's check, printed): `tests-pick-p13.d3 · [RETYPED] line 98: contract/coverage-failure/llvm-cov-exit-N is no row of the pack — read as contract/coverage-failure/llvm-cov-exit, the pack's row 98`
- **`[EVERY_ROW]` re-spawn** (pick-p1's check, printed): `arch-pick-p4.d3 · [EVERY_ROW] line 1: decision/env-var-viola-prefix — the pack's order holds contract/cli-verbs/unlink here`;
  re-spawned once as `pick-p1-r1`: `VERDICT: 1 of 1 pass · failed: none`. No dead draw.
- **Non-zero exits and refusals:**
  - `rec.py tree --outline` on `doc:viola-0.2.0/census`: exit 2, «`--outline` reads a master's category files;
    viola-0.2.0/census is no master — `rec.py tree doc:viola-0.2.0/census` lists its sections; nothing printed».
  - The Bash guard refused one `cd` («Blocked: cd (…) moves the session cwd …») and one pipe of a pipeline tool
    into `grep` («Blocked: a pipeline tool's output piped into a filter …»); each re-issued in the passing form.
  - `grep` on `viola-0.2.0/working-route.md` for `v2-04`: exit 1 (no line names the cap).
  - The gate's baseline reds above (exit codes as each line prints). No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED`
    line was printed by any tool in this run.
- **Where a letter was wrong or silent:**
  - Silent: the P2 re-run rule keys on «a file or a surface the scope as pinned did not name» and says nothing of a
    named surface's companion file. The closure and P4 added files the pinned scope does not name, without a second
    row read; each, with the row it rests on (the relay's ask, `inputs#I3`):
    - `schemas/config.v1.json` — `architecture · decision/config-keys-match-schema` (picked);
    - `fuzz/fuzz_targets/hook_stdin.rs` — `test-plan · decision/property-tests-on-parsers` (picked);
    - `crates/viola-agent-claude/Cargo.toml` and `Cargo.lock` — `security-plan · decision/release-build-without-test-features`'s Dependency Security area, read by `cargo deny` (no row names a manifest);
    - `fuzz/Cargo.lock` — `architecture · decision/fuzz-replay-ci` (picked) and security-plan's fuzz-lock audit;
    - `src/run/version_gate.rs` — named by scope item 1 already (no addition).
  - Mine, not a letter's: an `Edit` and the `inputs.py snap` that depended on it ran in one parallel batch; the edit
    failed and `inputs#I2` landed without Q1's answer. The answer was appended and snapped again as `inputs#I3`, which
    every citation names; `I2` stands `UNCITED` in the manifest (ids are never reused).
  - Wrong: none found.
- **A new key of a master's keyed section, or a NOT YET MEASURED mark cleared:** none needed.

## The steps a bump would add (Q1's answer asks it, so a revision is cheap)
`SNAPSHOT_V` 2 with a reader accepting 1 and 2 and its `state-recovered{snapshot-unsupported-v}` cases; `PROTOCOL_V`
2 on the client and the wrapper with the `-32602` `unsupported-version` cases both ways; the `list` document's `v`;
the harness's readers of each; the masters' contract rows for each `v`.

## Beyond the plan
| # | proposal | owner |
|---|---|---|
| 1 | Re-record `fixtures/claude/2.1.302/Mcp.tools-list-1.json` so its `list` description drops «and the usage budget» — needs a live `claude` start (`viola-mcp-tap`), so a named operator step that waits | «Live round on the trimmed product» (existing entry) |
| 2 | Remove the live home's stale `~/.viola/budget.json`, read by nothing after this chunk — a write under `~/.viola/`, so a named operator step (`inputs#I1` item 2) | the operator, at the desk (no route entry) |

Apply? (yes / review / cancel)
