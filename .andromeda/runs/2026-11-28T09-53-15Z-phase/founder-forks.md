# Founder forks — phase 2026-11-28T09-53-15Z-phase (chunk 2026-11-28-fleet-console-view, «Fleet console view»)

Written 2026-11-28T10:29:26Z. The operator is away; nothing here was asked by dialog (inputs#I1, inputs#I2). This is
the P3 block the directive asks for (inputs#I1 items 2–5): the window, the measurements, every reached row, and each
fork with its options in order, its cost, the recommendation marked. **The phase stops here**: F1–F7 are product
choices or row exceptions the build stands on, so no plan is written until his answers come back by a relay file.

## The window
- Taken up at `be4a06e2` (ci#46130927638 green 7 of 7, the only sha since the last flip). Promoted
  `2026-11-28-fleet-console-view` (tx 5333); scope rev 3 (closed, no `[inferred]` left); research rev 1.
- Row read: 7 tasks · 53 packs · 159 picker draws (1 re-spawned on ABSENT, passed) · union 1000 of 5023 picked
  (architecture 142 · security-plan 121 · design-system 110 · layout-templates 53 · test-plan 247 · obs-plan 265 ·
  a11y-plan 62), no master at `picked 0`; 7 history files, all pass.
- Playbook escalate rows read before this block: `rul-r74hshif` (boundary widening, a human's call),
  `rul-p2zl2kgl` (a widening halts for a live answer; a direction never ratifies it), `rul-o2tr55aj` (held items
  landed on a delegate's word stay provisional), `rul-bznresq5` (his own P4 answer still halts the wrap once, as a
  card).
- Read-only compositor queries run: none. Nothing of his home or `~/.claude*` read; no real `claude` run.

## The measurements (research M1–M8, `doc:2026-11-28-fleet-console-view/research`)
- **M1 what `list` reads.** `board::scan` → `board::read` → one `claude agents --json` spawn (5 s bound) →
  `board::join` (`src/cmd/list.rs:33`, `:96-106`). The view can call `scan`, `read` and the row builder in-process as
  they are. The session states idle/busy come ONLY from that spawn of the real CLI.
- **M2 the cost of one board read** (a test-owned home in the session scratchpad, median of 15, rows read as gone, so
  the cost of the reads alone). Release: 1 / 8 / 33 instances = 1.3 / 1.9 / 3.8 ms with 11 KB logs, 6.1 / 40 / 157
  ms with 1.1 MB logs. Debug: 2.6 / 7.1 / 23 ms and 60 / 465 / 1913 ms. The cost follows the fleet's total log bytes
  (about 4 ms per MB, release) because `board::read` reads every `events.ndjson` whole (`board.rs:382`). His real log
  sizes and a real `claude agents --json`'s cost are UNMEASURED.
- **M3 the board's words.** LIVE `live` · `stale` (heartbeat older than 5 s); STATUS `idle` · `busy` · `unknown`;
  WHEEL `driver` · `human` · `unknown`; DIALOG `none` · `DIALOG question|permission|plan|unknown`; CLI `<v> verified` ·
  `<v> unverified-cli` · `unknown`. matrix#v2-14's «working · idle · waiting on a dialog · stalled» maps to busy, idle
  and `DIALOG <kind>`. «stalled» has NO board word.
- **M4 last activity.** No board word, no field of the board's row (`board.rs:175-212`), and 0 master rows name it.
  On disk: the newest `events.ndjson` line's `ts` (the board read already passes over every line), or the
  heartbeat's mtime (beats each second while the wrapper lives, so that is liveness).
- **M5 width.** A wrapped board row is NAME + 62 fixed columns + the CLI cell. The narrowest wrapped row with a known
  CLI is 82 columns; the test literals are 85 (`… verified`) and 91 (`… unverified-cli`); the widest is 32 + indent +
  84. **At 80 columns no wrapped row with a known CLI fits whole**, and the board cuts only an unwrapped NAME. The
  frame at 24 rows holds 22 rows between its title and its key line.
- **M6 learning of a change.** (a) a re-read on the 100 ms tick = M2 ten times a second, plus up to 2 WARN
  `parse-rejected` lines per read while a skipped entry stands (`board.rs:458-470`); (b) a file watch: no such crate
  in the tree (`Cargo.lock`, 0 hits), so a new dependency; (c) a subscription: a new channel method, a new crossing;
  (d) a re-read on a slower cadence of its own (e.g. 1 s).
- **M7 «control-escaped»** is the board's `escape_row` on every cell (`src/human/escape.rs:20`); names are
  `ViolaName`s and cannot hold a control character. Nothing new to rule.
- **M8 «tiled … whole at quarter-screen».** The code can hold the frame whole at or above 80×24 and say so below it.
  Whether his quarter tile is ≥ 80×24 cells, and that the window opens tiled, only his compositor shows: a live
  reading, his to grant and to time, never fired by this chunk's first pass.

## Every reached row (searched by the view's terms over the seven rows files; terms: console · caption · escap ·
## redraw · spinner · footer · board · frame · tiled · stale · refresh · truncat · column · width; plus
## `rec.py find --doc all` for «last activity» (0), «quarter» (0), «tiled» (1), «stalled» (0 board rows), «fleet console» (21 rows))
- **architecture:** `decision/terminal-fleet-console` · `decision/list-is-caller-side-read` ·
  `decision/list-reads-state-directly` · `decision/board-read-model-shape` · `contract/board/status` ·
  `contract/board/agents-read-failure` · `contract/board/wrapped-merge` · `contract/list-item/liveness` ·
  `contract/list-payload/liveness` · `contract/session-liveness/stale` · `decision/codes-only-process-logs` ·
  `decision/cli-human-text-json-exit-codes` · `decision/cli-output-conventions` · `decision/new-capabilities-as-verbs`
  · `decision/untrusted-upstream-text` · `decision/replayed-snapshot-liveness` · `decision/snapshots-read-through-replay`
  · `decision/timestamp-field-names` (reached only by F2) · `name/command/console`.
- **design-system:** `decision/fleet-console-not-verb-output` · `contract/fleet-console-keys/{q,ctrl-c,unbound}` ·
  `decision/list-six-fixed-columns` · `decision/list-is-the-board` · `decision/state-carried-by-printed-word` ·
  `decision/busy-not-animated` · `decision/no-traffic-light-status-dots` · `decision/strip-board-density` ·
  `decision/wheel-column-width-23` · `decision/truncate-only-unwrapped-name` · `decision/no-hardcoded-widths-or-wrapping`
  · `contract/human-tables/width-source` · `contract/human-tables/name-truncation` · `decision/columns-never-read` ·
  `decision/list-rows-escape-all-controls` · `decision/escape-controls-in-human-output` ·
  `contract/viola-list-board/{row-escaping,name-column-width,name-cut,status,empty-board,unwrapped-row}` ·
  `contract/viola-list/columns` · `contract/viola-list-table/{columns,empty,row-order}` · `decision/no-spinner` ·
  `decision/no-spinners-in-verb-output` · `decision/cli-tty-static-expression` · `decision/plain-output-modes` ·
  `decision/stale-rows-sgr-dim` · `decision/stale-strip-dimmed-not-deleted` · `decision/colour-only-as-second-cue`.
- **layout-templates:** `decision/no-prompt-or-pager` · `decision/console-own-surface` · `decision/console-frame-floor` ·
  `contract/cli-verbs/console` · `decision/shared-captions-fixed` · `decision/no-link-column` ·
  `decision/board-ordered-by-name` · `decision/board-row-escaping` · `decision/human-output-is-contract` ·
  `decision/truncate-only-unwrapped-names` · `contract/viola-list-board/{tree,column-widths,name-truncation,unknown-width,
  status-join,stale-row-dim,stale-row-with-dialog,wheel-unknown,cli-unknown,dialog-unknown,empty-board,unwrapped-rack,
  plain-lines,agents-read-failure}` · `decision/human-tty-expression-level` · `decision/no-persistent-footer` ·
  `decision/only-header-bay-line` · `decision/plain-output-modes` · `contract/cli-token-rendering/motion-tokens` ·
  `decision/no-spinner`.
- **a11y-plan:** `decision/cli-not-keyboard-interactive` (its named exception) · `decision/cli-no-interactive-input`
  (its named exception: «redraws on a resize») · `decision/static-linear-output` (its named exception: «redraws its
  frame when the terminal's size changes») · `decision/linear-static-output` · `decision/linear-static-cli-output` ·
  `decision/no-spinners` · `decision/state-carried-by-printed-word` · `contract/viola-list-board/caption` ·
  `contract/viola-list-caption/order` · `decision/list-captions-fixed` · `decision/no-screen-content-parsing` ·
  `decision/control-character-escaping` · `decision/c0-c1-escaped` · `decision/claude-agents-read-fails-open`.
- **obs-plan:** `decision/no-interactive-terminal-interface` (its named exception: «redraws its frame») ·
  `contract/diag-line-fields/console-frame` («once per draw») · `contract/diag-line-fields/console-key` ·
  `contract/diagnostics-files/console` · `contract/console-output/stdout` · `contract/panic-outcome-by-role/console` ·
  `decision/symlink-skip-logged-as-count` («once per scan») · `contract/diag-line-fields/parse-rejected` ·
  `decision/per-parser-parse-failure-counts` · `decision/heartbeat-liveness` · `decision/heartbeat-staleness-threshold`
  · `contract/list-liveness/stale-threshold` · `decision/escape-controls-in-terminal`.
- **security-plan:** `decision/console-terminal-and-home-check` · `contract/strict-modes-refusal/console` ·
  `decision/console-key-reader-bounded` · `decision/whole-board-refusal` · `decision/board-home-read-rule` ·
  `decision/list-home-read-rule` · `decision/claude-agents-read-fail-open` · `decision/claude-agents-resolved-from-path`
  · `decision/bounded-external-readers` · `decision/events-reader-bounded-lines` · `decision/state-readers-never-write`
  · `decision/escape-terminal-controls` · `decision/human-output-escapes-controls` ·
  `decision/board-lineage-and-links-name-checked`.
- **test-plan:** `decision/tui-verdicts-from-records` · `decision/console-restore-read-on-own-terminal` ·
  `contract/strict-modes-refusal/console` · `decision/fake-agent-not-proof-of-real-cli` ·
  `decision/list-row-control-escaping` · `decision/list-reads-live-through-replay` ·
  `decision/heartbeat-staleness-pure-function` · `contract/instance-liveness/stale` · `decision/human-tty-through-outer-pty`.
- **Read, not reached:** the other picked rows (1000 picked; the list above is the subset the view's terms reach),
  each in its master's rows file in the run dir. **Read as not reached BECAUSE of the two «no verb's output» rows**
  (design-system `decision/fleet-console-not-verb-output`, layout-templates `decision/no-prompt-or-pager`) — each would
  bind the view if those two rows did not scope it off: a11y `decision/linear-static-output`,
  `decision/linear-static-cli-output`, `decision/no-spinners`; design-system `decision/no-spinner`,
  `decision/no-spinners-in-verb-output`, `decision/cli-tty-static-expression`, `decision/no-hardcoded-widths-or-wrapping`,
  `decision/truncate-only-unwrapped-name`, `contract/human-tables/width-source`; layout-templates
  `decision/no-persistent-footer`, `decision/human-tty-expression-level`, `decision/only-header-bay-line`,
  `decision/plain-output-modes`, `contract/cli-token-rendering/motion-tokens`, `decision/truncate-only-unwrapped-names`.
  Three rows are NOT scoped off by them, because each names its own console exception and that exception is narrower
  than the view: a11y `decision/static-linear-output` and `decision/cli-no-interactive-input` (a redraw on a resize
  only) and obs `decision/no-interactive-terminal-interface` (a redraw, unqualified — covers the view).

## His forks (each one the build stands on; the most conservative arm named; nothing assumed)

**F1 — the redraw on a fleet change (a row exception; the standing wrap fork, item 2, met by this chunk).** The view
redraws when the fleet changes, not only on a resize. Two NAMED exceptions say «on a resize» only.
1. **(Recommended)** Widen the two named exceptions by the same words: a11y `decision/static-linear-output` «… which
   redraws its frame when the terminal's size or the fleet it shows changes» and `decision/cli-no-interactive-input`
   «… and redraws on a resize or a change in the fleet it shows». The other rows stay scoped off by the two «no verb's
   output» rows as they stand. Cost: two row rewordings, landed by the wrap on your word.
2. Option 1, plus the wrap's earlier items 1–2 (reword the two scoping rows; name the console exception on the eight
   verb-output rows). Cost: twelve row edits; the view leans on no «no verb's output» reading.
3. Leave both. Cost: the view redraws only on a resize or a key, so «kept current as the fleet changes»
   (matrix#v2-14) is not built and v2-14 is not claimable.

**F2 — last activity (a product choice; option 2 widens what crosses `list --json` and the MCP list item).**
1. **(Recommended)** The age of the newest `events.ndjson` line, read in the board's pass the board already makes, kept
   in the board's row but out of `list --json` (no serialised field), shown only by the console as a seventh cell after
   CLI, captioned `LAST`, written `12s` · `4m` · `2h` · `3d`. Cost: one in-process field, no contract change. The six
   captions stand unchanged in their slots, with one console-only caption after them.
2. A `last_activity` UTC timestamp added to `list --json`, the MCP list item and a seventh board column. Cost: a
   boundary widening (new data crossing to drivers), a design change to the board (layout-templates
   `decision/human-output-is-contract`, `decision/list-six-fixed-columns`), the list fixtures and schemas.
3. No last activity. Cost: v2-14's acceptance clause unmet, so it is not claimable.

**F3 — «stalled» (a product choice).**
1. **(Recommended)** «stalled» is the board's `stale` (the wrapper's heartbeat older than 5 s), shown in the LIVE
   column as the board shows it. Cost: none; v2-14's word «stalled» is read as `stale` in a dated matrix note.
2. A new state: `busy` with no event for N seconds, shown as `stalled`. Cost: a new word and a threshold in
   design-system and layout-templates; a time rule you would set.
3. Drop «stalled» from v2-14's acceptance. Cost: a matrix rewording on your word.

**F4 — a row wider than the frame (a product choice; M5: at 80 columns every wrapped row with a known CLI is wider).**
1. **(Recommended)** Keep the six cells in their slots and cut each row at the terminal's width, ending the line in
   `...`, never inside an escape; a row stays one line. The CLI cell, last, is cut first. Cost: at 80 columns the CLI
   verdict word (`verified` / `unverified-cli`) is cut on most rows; a named console exception beside
   `decision/truncate-only-unwrapped-names` (the board's rule cuts only an unwrapped NAME) if that row is not read as
   scoped off.
2. Drop the CLI cell in the console (five captions). Cost: breaks «the console reuses the six captions»
   (`decision/shared-captions-fixed`, his word 2026-11-15).
3. Raise the frame floor to the widest honest row (91 columns + the indent). Cost: the floor moves past 80×24
   (`decision/console-frame-floor`), and a quarter tile may then read the one-line refusal.

**F5 — a fleet taller than the frame (a product choice; 22 rows at 24).**
1. **(Recommended)** Draw the rows in the board's forest order until one row is left, then one line
   `... <n> more`. Cost: the rows past the frame are not seen until the terminal is taller.
2. Scroll. Cost: keys, which «Console actions» owns; this chunk would wait on it.
3. Newest first. Cost: breaks `decision/board-ordered-by-name`.

**F6 — the refresh, and the agents read (a product choice; M2, M6).**
1. **(Recommended)** Re-read the board once a second and redraw only when the drawn rows change; run the
   `claude agents --json` read at most once every 5 s, off the drawing loop. Cost: up to 1 s lag for disk changes and
   5 s for idle/busy; one real-CLI spawn every 5 s on your host, its cost UNMEASURED until a live reading.
2. Re-read on the 100 ms tick. Cost: ten board reads a second (up to about 1.6 s of work per second at 33 instances
   with 1.1 MB logs, release), close to a busy loop.
3. Never run the agents read: STATUS always `unknown` in the console. Cost: «working» and «idle» are never shown.

**F7 — the skip-count line on a repeated read (a row exception).** `decision/symlink-skip-logged-as-count` writes one
WARN line «once per scan»; a console re-reading each second would write 3600 an hour while a skipped entry stands.
1. **(Recommended)** A named console exception: the console writes the line when the count changes, not on every read.
   Cost: one row reworded beside its existing `viola up` exception.
2. Leave it: a line per read. Cost: the console's role file grows by those lines while the entry stands.

**F8 — one implement window (yours by the directive).** **(Recommended)** One window holds the view on F1–F7's
recommended arms (a view module, the loop's re-read, the frame writer, the console's tests over a test-owned home and
its own pseudo-terminal). The split, if you prefer one: (a) the view and its rows, then (b) the refresh and the agents
read.

## Technical forks (the overseer's)
- **T1 — the row read again.** The closed scope names `src/console/` (the loop), which the pinned row read did not
  (it named `src/cmd/console.rs`), so the letter re-runs the row read whole on the corrected scope. Lean: run it ONCE
  at the resume, after his answers fold into the scope (F2's option 2 would add the list surface), then P4.
- **T2 — the state-read events.** Lean: `console-frame` gains `shown` and `hidden` counts (rows drawn, rows behind the
  `... <n> more` line); every list that pins its fields moves with it (obs-plan `contract/diag-line-fields/console-frame`,
  `schemas/diag-line.v1.json`'s default-deny field list, the pinned `LOG_FORMAT_EVENTS`).
- **T3 — module layout.** Lean: `src/console/view.rs` for the frame model; the board read injected into `console::run`
  as the size already is.
- **T4 — test homes.** Lean: booted fake-agent wrappers for live rows (liveness needs a living pid), planted instance
  dirs for the size cases, each home test-owned.

## Capability
- matrix#v2-14 can be claimed only on F1, F2 and F3's options 1 or 2: GATED on his live reading at his host (tiled, a
  quarter of the screen, kept current), never fired by this chunk's first pass. «usable» / «keyboard-driven» beyond `q`
  and Ctrl-C rest on «Console actions»; the plan will say whether that keeps v2-14 pooled.

His answer per fork: the option number (or his own words). It comes back by a relay file; the phase resumes at P3's
end on it.
