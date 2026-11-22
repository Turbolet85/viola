# P5 review card 1 — 2026-11-22-lineage-registry «Lineage registry»

**The plan to read:** `.andromeda/cache/docs/2026-11-22-lineage-registry/plan.md` (rev 4, 484 lines with its gate
fence; written by `rec.py render … --out`, byte-equal to `doc:2026-11-22-lineage-registry/plan`). No plan stands in a
chunk's folder; writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-22-lineage-registry` is a diff of the chunk's
records. Rulings: `inputs#I1` (the directive), `inputs#I2` (the P3 block's answers, every arm a).

## Goal
Every start writes one `lineage {parent, role}` line right after its `wheel` start line, mirrored in the snapshot and
recovered by the replay; the parent a reader takes is the LAST line's, so a reused name started without a parent has
none and a revive keeps it. `viola run --parent <name> --role <word>`, else the inherited `VIOLA_NAME` as the parent.
`viola list` draws trees in the NAME cell (two spaces a depth, the six captions fixed, the same characters piped) and a
`-- LINKS --` block after the board; `--json` / MCP `list` gain `parent`, `role` and a top-level `links`, additively. A
parent edge is a label only, held by a behavioural case and a source census.

## Top acceptance criteria (9 in the plan)
- lineage recorded at every start and read back after a wrapper kill and a cut snapshot (`matrix#v2-09`)
- a reused name has no parent; a revive keeps parent and role, a gone parent included (`matrix#v2-09`)
- the tree board, piped and on a terminal, cycles and deep indents named (`matrix#v2-09`)
- no parent edge changes the wheel, the links or a send's acceptance — a behavioural case and a census that FAILS
  when a wheel/link/send/answer path reads the field (`matrix#v2-09`)
- links apart from parent edges, a bad-name pair never shown (`matrix#v2-10`)

## Touchpoints
1 new file (`tests/contract_lineage_label.rs`), 27 modified: `viola-core` (the kind), `viola-state` (snapshot,
replay, board), `src/cmd/{run,revive,list,hook}.rs`, `src/human/board/{rows,cells}.rs`, `src/run/{mod,dialog,wheel}.rs`
and `send/slot.rs` (struct literals only), the harness `boot.rs`, six root test files, two crate test files.

## Test commands
22 entries: fmt, clippy; units of `viola-core`, `viola-state`, the root bins; three filtered new-case entries and the
census binary's own entry (red-vacuous at baseline: 0 selected, or exit 94 for the not-yet-written binary); the seven
touched integration binaries (green 96/96 at baseline); the harness boot read; the host read; the three harness
layers; the operator pass (boot smoke, pre-push, hygiene, push, CI). Three remove-the-guard controls as implement's
one-shots: `last-wins`, `name-check`, `revive-keep`. No live start, no mutation run, no fuzz target, no time bound.

## Concretization preview (the never-weaken checkpoint)

**`matrix#v2-09`** — claimed by this chunk.
- *current:* «A session's declared parent and role are recorded once at launch and read back the same after a crash and a
  replay; the board shows the fleet as trees; a reused name starts with no parent; a revived session keeps its parent;
  no parent edge changes the wheel, the links or whether a send is accepted; the overseers run wrapped with their
  parents declared.»
- *proposed:* «A session's declared parent and role (`viola run --parent`/`--role`, else the inherited `VIOLA_NAME` as
  the parent) are recorded at its launch as one `lineage` event `{parent, role}` in its own `events.ndjson`, right
  after the start's `wheel` line, and in its snapshot, and read back the same by `viola list --json` after a wrapper
  kill and a cut snapshot (`e5_the_replay_reads_the_lineage_the_wrapper_wrote`); the board shows the fleet as trees —
  each child row under its parent's, its NAME indented two spaces per depth, the same characters piped and on a
  terminal (`list_draws_a_declared_child_under_its_parent_piped`, the `board_tree_*` cases); a reused name started with
  no parent has none (`list_a_reused_name_started_without_a_parent_has_none`); a revived session keeps its parent and
  role (`revive_keeps_the_recorded_parent_and_role`); no parent edge changes the wheel, the links or whether a send is
  accepted (`lineage_changes_no_wheel_link_or_send`, `lineage_is_read_by_no_wheel_link_send_or_answer_path`).»
- *what moved:* the last clause, «the overseers run wrapped with their parents declared», moves to `matrix#v2-28` by
  the operator's own word (`inputs#I2` F9: «Один чанк, фразу в v2-28 (Recommended)»). Nothing else of v2-09's
  acceptance is weakened: every other clause stands, made concrete.

**`matrix#v2-10`** — claimed by this chunk.
- *current:* «Observed links appear apart from parent edges on every view that shows both, and a link whose name is not
  a valid viola name is never shown.»
- *proposed:* «On `viola list`, the one view showing both, observed links appear apart from parent edges — in a
  `-- LINKS --` block after the board, one `<driver> -> <driven>` line each, sorted, and in `--json` as a top-level
  `links` array apart from the items' `parent` — and a pair whose driver or driven does not parse as a `ViolaName`
  appears in neither (`list_links_block_shows_a_send_link_apart_from_the_parent_edge`,
  `board_read_links_are_sorted_deduplicated_and_name_checked`).»

**`matrix#v2-28`** — not claimed here; its acceptance gains the clause (see Beyond the plan).
- *current:* «On the founder's Omarchy host the pair's whole fleet is started by viola, restored by viola after a reboot,
  and seen and driven from one console, with the three fleet scripts removed and nothing calling them; the founder
  confirms it.»
- *proposed:* «On the founder's Omarchy host the pair's whole fleet is started by viola, the overseers running wrapped
  with their parents declared, restored by viola after a reboot, and seen and driven from one console, with the three
  fleet scripts removed and nothing calling them; the founder confirms it.»

## The directive's item-12 block (verbatim where a tool printed it)

**The union, per master, and the `rows-union` line:**
```
architecture: rows 983 · packs 10 · picked 282 (169 by every draw) · standing 35 · 75248 B
security-plan: rows 633 · packs 7 · picked 148 (106 by every draw) · standing 25 · 42262 B
design-system: rows 326 · packs 4 · picked 154 (98 by every draw) · standing 8 · 34614 B
layout-templates: rows 225 · packs 3 · picked 129 (99 by every draw) · standing 4 · 27555 B
test-plan: rows 1373 · packs 14 · picked 300 (153 by every draw) · standing 79 · 69129 B
obs-plan: rows 1038 · packs 11 · picked 168 (75 by every draw) · standing 50 · 44240 B
a11y-plan: rows 238 · packs 3 · picked 117 (85 by every draw) · standing 22 · 30435 B
rows-union: 7 master(s) · picked 1298 of 4816 · rule any
```
The sum is about 323 KB, past the 250 000 B line: the rows files were read by search, never whole.

**Every cited row in the union or its standing list?** Yes, after one correction. A script over the plan's cited
handles read three as absent; each was read: `contract/normalised-event-kinds` is a sheet (its 15 rows are in the
architecture union); `contract/viola-run/start-output` is layout-templates' and in its union (the script read it under
obs-plan); `contract/viola-list-board/order` is a11y-plan's, not layout-templates' — the plan had it under the wrong
master, corrected at plan rev 4 (an a11y-plan Expected amendment line). Every other cited handle reads `union`.

**Round pre-flight:** none — the plan holds no `round` or `live` entry.

**`[RETYPED]` lines and re-spawns:**
```
arch-pick-p4.d2 · [RETYPED] line 79: contract/viola-verify-exit-codes/1 is no row of the pack — read as contract/viola-verify-exit-codes/1-relied-row-failed, the pack's row 79
security-pick-p3.d3 · [RETYPED] line 12: decision/unwrapped-agent-name-never-target is no row of the pack — read as decision/unwrapped-name-never-target, the pack's row 12
tests-pick-p7 · [RETYPED] line 67: decision/os-gated-tests-each-named-with-reason is no row of the pack — read as decision/os-gated-tests-each-named, the pack's row 67
tests-pick-p14 · [ABSENT] .andromeda/runs/2026-11-22T22-06-16Z-phase/tests-pick-p14.md
```
One re-spawn: `pick-p2-r1` (tests-pick-p14, `--failed ABSENT`), `1 of 1 pass`.

**Non-zero exits, refusals, `UNPARSED` / `INDETERMINATE` / `NOT DERIVED`:**
- `rec.py put --doc research` (rev 1): `note: … research:87: too broad a glob — its literal prefix names no directory`;
  `gate.py scope --lists-only`: `UNPARSED: research.md:87 — too broad a glob …` → the derived glob replaced by the nine
  named files, `scope lists: parsed`.
- Baseline entry 8 (first form): `red · exit 0 ✗ (exit 94)` — `error: operator didn't match any binary names` on
  `binary(contract_lineage_label)`, which made the whole filter fail and read none of the existing binaries → the entry
  split (rev 2): the seven existing binaries green 96/96; the census binary its own entry, `exit 94` red-vacuous.
- The Bash guard refused four calls, each re-issued once in the form it passes: a `cd` into the skill's references
  dir and a `cd` into the run dir (absolute paths), a `matrix.py --help` piped into `head` (run bare), and a `cat`
  heredoc to a scratchpad file (written with the Write tool).
- No `INDETERMINATE` or `NOT DERIVED` line was printed by any tool this run.

**Where a letter was wrong or silent:**
- `plan-template.md` and `gate-contract.md` are silent on nextest's refusal of a whole filterset when one
  `binary(...)` names a binary that does not exist yet (exit 94) — a chunk's new test binary in a shared filter makes
  the entry read nothing at baseline; met here and split.
- The matrix contract moves a capability's acceptance only by `claim` (this chunk's caps) or `refine --unclaimed` (a
  0-pending wrap); the phase letter is silent on moving a clause from a claimed cap to an unclaimed one, so v2-28's
  gain is a Beyond-the-plan row for the next 0-pending wrap.
- v2-09's REQUIREMENT line also states «every session, the overseers included, runs under viola with its parent
  declared»; the F9 word names the acceptance only, and no letter says whether a claimed cap's requirement line follows
  a moved acceptance clause — named below, not touched.
- `fanout.py check` printed 53 `not a marker` rows for security-history (its items cite `amd-…` ids, as the history
  prompt allows); read, none a defect.

## Beyond the plan
1. **`matrix#v2-28`'s acceptance gains «the overseers running wrapped with their parents declared»** (the proposed text
   above), by the operator's own word (`inputs#I2` F9), written by `matrix.py refine --unclaimed` at the next 0-pending
   wrap — v2-28 is unclaimed and this chunk's phase cannot re-word it. Until then v2-09's moved clause stands owed in
   this row.
2. **v2-09's requirement line keeps its overseer clause** («every session, the overseers included, runs under viola
   with its parent declared»): it is not re-worded here (a claimed cap's text is the wrap's `refine --chunk` only);
   the operator rules whether it follows the acceptance to v2-28 at this chunk's wrap, or stands as the requirement
   v2-28 now proves.

Apply? (yes / review / cancel)
