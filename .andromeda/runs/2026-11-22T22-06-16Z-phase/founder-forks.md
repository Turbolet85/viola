# Founder forks — 2026-11-22-lineage-registry (P3 end, before P4)

**This block HOLDS forks that are the operator's (F1–F9). The phase stops here, at P3's end, and waits for the word.**

- **Window:** the builder session that ran `/andromeda-phase` on «Lineage registry», run dir
  `.andromeda/runs/2026-11-22T22-06-16Z-phase/`, under the product re-pinned to f274938f; P1 promoted the chunk (tx
  4087), P2 read the masters' rows (7 of 7, picked 1298 of 4816), P3 measured and wrote `doc:2026-11-22-lineage-registry/research`
  (rev 2) and closed the scope (rev 3). Directive: `inputs#I1`.
- **No live start, no rig start, no mutation run, no witness run, no timer** were made. One test run under the fake
  agent (M1 below).

## The measurements (research M1–M8, each a test or a no-start code read)

| # | What | Reading |
|---|---|---|
| M1 | the fake-agent run: `cargo test --features fake-agent --test cli_list --test cli_links --test cli_revive --test cli_output_plain --test chaos_revive` | exit 0 · 67 passed · 0 failed (21 · 12 · 27 · 6 · 1) |
| M2 | a parent/child relation in the tree | **none**: 0 event kinds, 0 snapshot fields, 0 board fields, 0 flags name a parent or a role; 0 lineage rows in the seven masters |
| M3 | the one relation held: observed links | `link`/`unlink` lines + `snapshot.json` `links`, restored at EVERY start (`src/cmd/run.rs:338-343`) |
| M4 | a reused name and its links | **inherited** — a second `run` of a gone name restores the first life's links (`links_survive_a_wrapper_restart`, green in M1) |
| M5 | what `revive` keeps | the log (appended), links, the resumed session, the recorded `cwd`; **no parent** (none exists) |
| M6 | `list` today | piped: `BAY`, caption, rows, **no SGR** (green in M1); terminal: the same characters + SGR (bold NAME, amber DIALOG, dim stale); `--json` items exactly 8 keys, pinned, **no `links`**; no view prints a logged link name |
| M7 | what a launch knows of its parent | a `viola run` typed inside a wrapped session inherits `VIOLA_NAME` · `VIOLA_DIR` · `VIOLA_BIN` (set by that session's wrapper) and reads the pair today **only for its home**; no socket, no record of the caller; it **cannot** know the name is honest (self-reported) or live; in a plain shell it sees **nothing** |
| M8 | where a start's lines land | `start_state` writes one `wheel` start line before the spawn; test-plan pins «line 1 `wheel`, line 2 `session-start`» for a booted instance |

## The design each fork leans to (the mechanism)
- One new event kind **`lineage`**, log-only (wakes no `wait`), written by the start's one writer `start_state` for
  `run` and `revive` alike, before the spawn; data `{parent, role}`, each a name/word or `null`.
- The parent a reader takes is the **last `lineage` line's** (never the link rule, which M4 shows inherits): a `run`
  writes its declared parent or `null` → a reused name starts with none; a `revive` writes the parent of the last
  line before it → kept. The snapshot mirrors it; the replay recovers it (a fourth replayed field).
- A parent edge is a **drawing label only**: no wheel, link, `send` or `answer` path reads it (asserted by test).

## What is written to disk, what outlives a SIGKILL, what of a parent's identity can be read (item 6)
- **Disk:** one `lineage` line per start in `instances/<name>/events.ndjson` (0600 in a 0700 dir, one write, owner-only
  append); one `lineage` field in that instance's `snapshot.json` (atomic replace, 0600). Nothing new under
  `diagnostics/`, no process-log line, no new file, no home-level file.
- **SIGKILL:** the line is written before the child is spawned, so a SIGKILL of the wrapper after the start keeps it;
  a SIGKILL before it leaves no line of that life, and a later `revive` reads the previous life's last line.
- **A parent's identity on disk:** its **name** (a validated `ViolaName`) and the **role word**, nothing else — no
  path, no session id, no socket, no pid. They land only in the child instance's own 0600 files, readable by the same
  OS user (the existing trust boundary: security-plan · decision/same-user-trust-boundary), never in a fixture, a
  diagnostics line or a public tree.

## New test children, fake-agent behaviour and bounds (item 7)
- **No new fake-agent behaviour and no new test child:** the lineage line is the wrapper's, written before the child;
  the existing `boot` helpers of `tests/cli_list.rs` / `tests/cli_revive.rs` start the fake agent, with `VIOLA_NAME`
  set explicitly on the started `viola run` where a case declares a parent through the environment (test-plan ·
  decision/test-started-viola-drops-env). Each wait is the existing boot readiness wait, under the 20 s lint.
- **Boot readiness moves:** the `lineage` line lands right after the `wheel` start line, so `session-start` becomes
  line 3 of a start; the harness's boot readiness read (test-plan · contract/boot-readiness/events) is changed with it,
  proved by the existing boot tests going green.

## Every master row the change reaches (all seven searched: `rec.py find` for `lineage|parent` in each, and the
## union's rows files grepped for link · board · list payload · revive · event kind · VIOLA_NAME · caption · column)

**Reached (amended or extended by the recommended options):**
- architecture · contract/normalised-event-kinds/* (one row added: `lineage`) · decision/event-kinds-agent-neutral
  (held) · contract/event-line/data (held) · contract/instance-snapshot/data (+ `lineage`) ·
  decision/snapshot-replay-log-fields-only (+ lineage: four fields) · contract/list-payload/items (F6) ·
  decision/board-read-model-shape («none of its types holds … links» — F5/F6) · decision/wrapper-events-log-only
  (+ `lineage`) · contract/channel-wait/wake-kinds (held: `lineage` wakes nothing) · decision/revive-in-place
  (+ keeps the parent) · contract/child-environment/viola-name (read by `run` as parent — F1)
- obs-plan · decision/instance-source-per-role («run takes its own name argument, never an inherited VIOLA_NAME» —
  F1's arm b/c must say the inherited name is read as the PARENT, never as the instance) ·
  decision/link-names-never-in-process-log (extended to lineage names) · contract/event-line/shape (held)
- security-plan · decision/from-field-authenticates-nothing / decision/channel-from-not-identity (extended: the
  parent is self-reported, never identity) · decision/instance-path-from-validated-name (held: a parent name is never
  joined into a path) · decision/r8-claude-env-strip (held: `VIOLA_*` is not `CLAUDE*`)
- design-system · decision/list-six-fixed-columns · contract/viola-list/columns · contract/cli-transfer-arrow/ascii-arrow
  («links print only as `link` / `unlink` verb output») — F4, F5
- layout-templates · decision/shared-captions-fixed (held: the tree is drawn inside the NAME cell, the six captions,
  words and slot order unchanged) · decision/no-link-column (F5) · decision/board-ordered-by-name («rows ordered by
  name» — F4) · decision/human-output-is-contract («new fields go into `--json` first») ·
  contract/viola-list-board/single-write · decision/words-carry-every-state (held: piped = terminal less SGR)
- test-plan · contract/boot-readiness/events · contract/event-log/start-wheel-record (line 2 → line 3) ·
  decision/links-survive-wrapper-restart (held: links still inherited, the parent not) ·
  decision/snapshot-replay-fields (+ lineage) · contract/event-line/round-trip (every kind, `lineage` included)
- a11y-plan · decision/terminal-structural-equivalents and decision/cli-structural-landmark-equivalents (a links
  separator is a new landmark — F5 arm b) · decision/state-carried-by-printed-word (held: indentation carries
  structure, no state) · decision/critical-path-cli-board-output (the board's captions and slot order, held)

**Read as not reached:** design-system and layout-templates colour, motion and refusal-line rows (no new colour, no
SGR on a tree or a link line, no refusal line unless F8 arm b); security-plan socket, strict-modes, trust-record and
input-validation rows beyond those named (no new reader of another instance's files); obs-plan's event catalog
(`ObsEvent` unchanged, no new process-log line); test-plan's mutation and live-rig rows (none run); every
`viola verify` / ledger row (no new dependence on a `claude` behaviour — the lineage is viola's own).

**Time bounds:** none new. No list cell is touched.

## The forks that are the operator's (each option priced; the recommendation marked)

**F1 — How a parent is declared at launch.**
- **(a) Both: `viola run <name> --parent <p> [--role <r>]`, and with no `--parent` the inherited `VIOLA_NAME` when it
  passes `ViolaName` (Recommended).** Cost: two new flags on `run`; obs · instance-source-per-role amended to say the
  inherited name is read as the parent only; matches v2-07's «names the calling agent as the parent when none is
  given», so `viola open` reuses it. A human typing `viola run` in a plain shell gets no parent.
- (b) Flag only. Cost: least change, no env read; every agent must pass `--parent` itself, so v2-07's default needs its
  own read later.
- (c) Inherited `VIOLA_NAME` only. Cost: no flag; a human cannot declare a parent at all, and no role can be given.

**F2 — The role.**
- **(a) Optional `--role <word>`, a code word of the existing shape (`viola_core::is_code_word`, 1–64 `[a-z0-9-]`,
  a letter first), absent by default; free, not a closed list (Recommended).** Cost: one flag; a bad word is clap's
  exit 2.
- (b) A closed list of role words (named by you). Cost: a closed word set to keep; a new role is a code change.
- (c) No role in 0.2.0. Cost: v2-09's «declared parent and role» is not met; the cap cannot be claimed whole.

**F3 — The `lineage` event's shape and place.**
- **(a) Kind `lineage`, `source: wrapper`, data `{parent: <name>|null, role: <word>|null}`, written by every start
  right after its `wheel` start line, log-only; the snapshot gains `lineage {parent, role}`; additive, no `v` move
  (Recommended).** Cost: `session-start` moves from line 2 to line 3 of a start (two test-plan contract rows amended,
  the harness boot read changed); an older binary reading a newer log counts the line under `skipped.unknown_kinds`.
- (b) The same data carried inside the start's `wheel` line. Cost: no new kind and no line move; a `wheel` line
  re-meant — a `v` bump by the protocol rule, and v2-09's «as a `lineage` event» unmet.

**F4 — How `list` draws the trees (inside the six fixed captions).**
- **(a) Each child row directly under its parent's row, its NAME cell indented two spaces per depth; roots and
  siblings in name order; a row whose parent has no row on the board (gone, unwrapped, never booted, or in a cycle)
  drawn as a root; the same characters piped and on a terminal, ASCII only (Recommended).** Cost: layout-templates ·
  board-ordered-by-name amended («by name within each parent»); the NAME column widens by the indent; the 19 literal
  tests of `rows.rs` keep passing for a board with no parent.
- (b) As (a) on a terminal, flat name order when piped. Cost: two layouts to keep; a script reading the piped board
  loses the tree.
- (c) No tree on the human board; trees only in `--json`. Cost: v2-09's «the board shows the fleet as trees» unmet
  on the CLI.

**F5 — Observed links on `list` («links drawn apart, name-checked»).**
- **(a) A links block after the board (after the unwrapped rack): one separator line `-- LINKS --`, then one line per
  link `<driver> -> <driven>`, sorted, each name re-parsed through `ViolaName` and a pair with a bad name never shown;
  no line when there is no link; plain in every mode (Recommended).** Cost: amends design-system ·
  list-six-fixed-columns / contract/viola-list/columns / contract/cli-transfer-arrow/ascii-arrow («links print only as
  verb output») and layout-templates · no-link-column — your own rulings (fct-2enypnm7, 2026-11-06 deferral of v1-36's
  board clause) — the six columns themselves stay fixed; a new structural landmark for a11y; v2-10 provable here.
- (b) No links on `list` (your rows stand); links stay verb-only until the fleet console. Cost: v2-10 not provable by
  this chunk — it stays pooled for «Fleet console view»; the entry's «links drawn apart» narrows to the console.
- (c) Links in `--json` only (a top-level `links` array, name-checked), the human board unchanged. Cost: amends
  architecture · board-read-model-shape; v2-10 holds on the one view that shows both.

**F6 — What `list --json` gains.**
- **(a) Additive: each wrapped item gains `parent` and `role` when present (absent otherwise), and with F5 (a) or (c)
  a top-level `links` array of `{driver, driven, since}`, names re-parsed; no `v` move (Recommended).** Cost: the
  pinned item literal and the board-read-model-shape row («no … links») amended; MCP `list` returns the same.
- (b) The same under `v: 2`. Cost: every `--json` reader of `list` (drivers, MCP clients) must change.

**F7 — `revive` with a parent that is gone.**
- **(a) Keep it as recorded, no refusal, no warning; the board draws the child as a root while the parent has no row
  (Recommended).** Cost: none new — a parent is a label only.
- (b) Refuse the revive with a new closed code. Cost: a new closed word and hint; contradicts «a parent edge never
  changes … whether a send is accepted» in spirit (it gates a start).
- (c) Revive with the parent dropped (`null`). Cost: v2-09's «a revived session keeps its parent» fails when the
  parent is down.

**F8 — What «name-checked» refuses.**
- **(a) Nothing new at a start: `--parent` takes `ViolaName`'s parser (clap exit 2, as every name argument); a parent
  equal to the instance's own name is clap's exit 2 too; a parent not booted is accepted (self-reported, as a link
  driver); the name check of «links … name-checked» is a display filter — a link name that is no `ViolaName` is never
  shown, counted nowhere (Recommended).** Cost: no new code word, no new exit, no new hint.
- (b) A self-parent refused at the start with a new exit-1 code `parent-is-self` and its hint. Cost: a new closed word
  in the revive/run code set, its `--json` document and the cause tables.
- (c) A link name that fails the check counted under `skipped` (a new key). Cost: the `skipped` triple becomes four,
  the BAY line's `skipped a - b - c` changes.

**F9 — v2-09's overseer clause, and the split.**
- **(a) One chunk, no split; claim `matrix#v2-10` here (with F5 a or c); v2-09 claimed here with its last clause
  («the overseers run wrapped with their parents declared») moved to «The fleet on the founder's host» / v2-28 by your
  word, the rest of its acceptance unweakened (Recommended).** Cost: an acceptance re-wording only you can give.
- (b) One chunk; v2-09 left pooled (a dated note), claimed later by «The fleet on the founder's host». Cost: this
  chunk builds v2-09's whole CLI half and claims nothing for it.
- (c) Split: «Lineage registry» (F1–F3, F6's parent keys, F7, F8) and a new «Board trees and links» entry (F4, F5,
  F6's links) placed after it. Cost: two takes-up, two wraps; each window lighter.

## One implement window
At the recommended options (F1 a · F2 a · F3 a · F4 a · F5 a · F6 a · F7 a · F8 a · F9 a) the plan touches 23
research list items (14 by design, 9 by the snapshot struct-literal sweep) across `viola-core`, `viola-state`, `src/`
and four root test files: **one implement window holds it**, near its upper half. With F5 (b) it is lighter; F9 (c)
is the split if you want two.

## Answers (appended 2026-11-22, from the overseer's relay `~/dev/projects/additional/viola-overseer/relay-lineage-p3.md`, read by the overseer at 2026-11-22T22:50:04Z; snapped as `inputs#I2`)

### The operator's own words (the overseer's two dialogs, this evening; each option was shown to him whole with its cost)
- **F1:** «Флаг и наследование (Recommended)» — arm (a): `--parent` and `--role` on `run`; with no `--parent`, the
  inherited `VIOLA_NAME` when it passes the name parser, read as the PARENT only, never as the instance.
- **F2:** «Свободное слово (Recommended)» — arm (a): an optional `--role`, a code word of the existing shape, free,
  absent by default.
- **F4:** «Отступ в NAME везде (Recommended)» — arm (a): the child row under its parent's, the NAME cell indented two
  spaces per depth, roots and siblings by name, a row whose parent has no row drawn as a root, the same characters
  piped and on a terminal, ASCII only. The six captions, their words and slot order are unchanged.
- **F5:** «Блок LINKS под доской (Recommended)» — arm (a): the links block after the board and the rack, the
  separator line, one line a link, sorted, each name re-parsed, a pair with a bad name never shown, no line when
  there is no link, plain in every mode. He was shown that this amends his own rows (design-system
  `decision/list-six-fixed-columns`, `contract/viola-list/columns`, `contract/cli-transfer-arrow/ascii-arrow`;
  layout-templates `decision/no-link-column`): the six columns stay fixed, «links only in verb output» is what moves.
- **F9:** «Один чанк, фразу в v2-28 (Recommended)» — arm (a): one chunk, no split; v2-10 and v2-09 claimed here;
  v2-09's last clause («the overseers run wrapped with their parents declared») moves to v2-28's acceptance by his
  word, the rest of v2-09's acceptance unweakened, word for word.

### The overseer's technical answers (no closed word, no exit, no `v` move is added by any of them)
- **F3: arm (a).** Kind `lineage`, `source: wrapper`, data `{parent, role}` each a name/word or `null`, written by
  every start right after its `wheel` start line, log-only; the snapshot gains the field; additive. The kind's name
  is the route entry's own. The plan names every reader of «line 2 is `session-start`» (tests, the harness boot
  read, the product's own replay) and shows each changed.
- **F6: arm (a).** Additive: `parent` and `role` on a wrapped item when present, a top-level `links` array of
  `{driver, driven, since}` with names re-parsed; no `v` move. The plan says what a reader pinned to the eight keys
  meets (the MCP `list` tool's schema among them) and holds it by a test.
- **F7: arm (a).** A revive keeps the recorded parent; no refusal, no warning; the board draws the child as a root
  while the parent has no row.
- **F8: arm (a).** Nothing new at a start: `--parent` takes the name parser (exit 2), a self-parent is exit 2 too, a
  parent not booted is accepted; the links' name check is a display filter, counted nowhere.

### For P4 and the P5 card (the overseer's)
- A cycle (a → b → a by two self-reported starts) and a depth past the terminal's width each get a named case; say
  what the NAME cell does when the indent meets the column's bound.
- «No parent edge changes the wheel, the links or whether a send is accepted» is held by a test that would FAIL if
  a path read the field — name the paths it covers.
- The acceptance move of F9 is shown old beside new for both v2-09 and v2-28 on the card, nothing else of either
  reworded.
- An older binary reading a newer log: the pinned product of TODAY (the build of f274938f) drives this session and
  will read logs a new test binary never writes into its home — say so, or name where it could.
