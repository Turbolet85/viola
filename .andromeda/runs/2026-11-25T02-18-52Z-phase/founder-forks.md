# Founder forks — «Placing without moving the view» (`2026-11-25-placing-without-moving-the-view`), P3's end

Run `.andromeda/runs/2026-11-25T02-18-52Z-phase/` · taken up at `38d9b7e8` · directive `inputs#I1` · the overseer's
technical answers `inputs#I2` · the operator AWAY: no dialog raised for him; each fork below waits for his word, its
PROVISIONAL arm taken meanwhile.

## My window
- Not measurable from inside the window as a number. Spent so far: P1, P2 (task · 159 pickers in 3 parts + 1
  re-spawn · history), seven history files read whole (≈142 KB), two rule files whole, the research reads. P4 reads
  the rows files by search (their union is 332 957 B, past the 250 000 B whole-read bound).

## Measurements (each a number or a closed word; `doc:2026-11-25-placing-without-moving-the-view/research` §Measurements)
- **Move forms offered by Hyprland 0.56.2** (read-only: `hyprctl --help`, `hyprctl version`, the Lua stubs
  `/usr/share/hypr/stubs/hl.meta.lua`, the shipped `hyprland.lua`, the binary's strings; no dispatch): the Lua window
  move's own refusal string lists its arguments `direction, x+y(+relative), workspace, into_group, out_of_group` —
  **no silent / follow argument**. The legacy name `movetoworkspacesilent` stands in the binary (beside a legacy
  dispatcher translator); its reach through `hyprctl dispatch` on 0.56.2, which runs its arguments as Lua, is
  **unmeasured**. Closed word: **no measured silent form**.
- **What each does to the active workspace**: the built move takes it (R4a: 1 → 5, measured); `hl.dsp.focus({
  workspace = N })` sets it (R3, by design). No other form is measured.
- **The active workspace is readable within the set**: `Monitors` carries `focused` and `active_workspace`
  (`crates/viola-hyprland/src/reply.rs:43-50`), recorded (`Monitors.recorded-1.json`, active id 11).
- **Seam · tap · fake for another form**: the set is 8 members (5 queries, 3 dispatches; `lib.rs:43-54`, `WORDS: [&str;
  8]`); the move's argv literal is pinned at 9 `.rs` sites, 1 recorded fixture and 3 test-time derivations of it; the
  tap's `record_move` reads placement only (no active workspace); the fake replays by exact argv and holds no
  active-workspace state. A ninth member adds a `WORDS` entry, a schema enum value, a hygiene row and its own
  recorded fixture.
- **Callers**: `open`, `up` (through `revive --origin`), `revive --origin` and `adopt`'s start reach the move only
  through `open_window` → `window_step` in the shared start (graph trace: 1 product caller each); a switch only
  re-records the window line. One form serves all; no row asks a per-caller difference.
- **The window's own focus**: the built move leaves the moved window focused on the workspace it took; F4 binds the
  operator's view (the active workspace). Unmeasured: what Hyprland focuses after a workspace focus back.
- **Stays the fake's, unmeasured live**: the request sequence and the eight placement words; the active-workspace
  clause on a real compositor is a live reading only.
- **The folded item, from the code**: the automatic verify runs in the start's own process cwd (`src/cmd/run.rs:389,
  431`; `src/run/auto_verify.rs:242`); verify's Runs B–E run in `<cwd>/.viola-verify-<pid>*/`, trusted only by
  inheritance from that cwd (`src/cmd/verify/typed/runs.rs:298-305`); in an untrusted folder they meet the trust form,
  are killed with no key, and their rows are stamped `fail` (a failing row is stamped); the start's trust step runs
  only after a verified verdict (`run.rs:443-444`); the entry is then «attempted, red» and no later start verifies
  it again (`auto_verify.rs:67-76`). Verify already reads each run's first screen as a closed word, `trust` among them
  (`FirstScreen::Trust`), and refuses on none of them today.

## Reached rows (bound by this chunk), each with what happens to it
- architecture · `decision/hyprctl-direct-spawn` — holds (no new Lua form under arm M1; both forms already measured)
- architecture · `decision/hyprland-seam-crate` — holds (the set stays 8, `inputs#I2` T1)
- architecture · `contract/viola-cli-arguments/revive-origin` — holds
- architecture · `contract/viola-verify-exit-codes/1-relied-row-failed` — **amended under V1** («the failing row's
  stamp is still written» gains: a run whose own first screen read the trust form is a refusal and writes no stamp)
- architecture · `contract/verify-process-exit/interrupted` — read; **a sibling row added under V1** (the new code)
- architecture · `decision/no-automatic-reverify-of-red` · `decision/start-verifies-new-cli-version` ·
  `decision/run-start-order` · `decision/automatic-verify-attempt-lock` · `contract/config-json/verify-new-cli` ·
  `contract/instance-snapshot/cli-verified` · `decision/viola-home-resolution` · `decision/on-disk-file-naming` — hold
- security-plan · `decision/window-step-moves-own-terminal-window` — **amended under M1** (the step also reads the
  focused monitor before and after and focuses the operator's own prior workspace back when the move took it: a new
  dispatch crossing at the compositor boundary — **a boundary widening, escalate rows `rul-r74hshif` / `rul-p2zl2kgl`**)
- security-plan · `decision/hyprland-recorder-moves-own-window` · `decision/hyprland-recorder-string-allowlist` ·
  `decision/run-window-flags-checked` · `decision/focus-raises-only-ancestry-windows` — hold
- design-system · `contract/placement-word/{placed, refused, unplaced, unsupported, unreachable, oversize, malformed,
  window-unseen}` — hold (eight words kept, `inputs#I2` T3, PROVISIONAL); `contract/viola-up-outcome/live` ·
  `contract/cli-verb-output/up` — hold
- design-system · `contract/auto-verify-line/{verifying, did-not-verify, earlier-red, no-stamp}` — hold (V1's refusal
  reads the existing `no-stamp` `unstamped` line); `contract/viola-verify-output/help` — holds;
  `contract/viola-verify-output/summary-line` — holds (a refusal prints its pair, no summary, as a stop does)
- layout-templates · `contract/viola-verify-output/failing-row` — **amended under V1** (the carve-out above);
  `contract/viola-verify-output/exit-1` · `contract/viola-run/start-output` · `contract/viola-revive/no-window` ·
  `contract/cli-verbs/focus` — hold
- test-plan · `decision/window-placement-proven-live` — **amended** (gains the active-workspace clause, proven only by
  the gated live reading R7); `decision/window-reopen-proven-live` · `decision/fakes-for-external-services` ·
  `decision/fixtures-recorded-by-live-verify` · `decision/ci-never-real-claude` ·
  `contract/fake-agent-cli/argv-only-options` · `decision/human-never-blocked` ·
  `decision/run-start-verify-and-trust-after-bind` · `decision/record-only-at-zero-failing-rows` — hold
- obs-plan · `contract/process-exit-detail/verify-refusal` — **amended under V1** (one closed code added);
  `contract/run-process-log/auto-verify-stop` · `contract/config-json/known-keys` · `decision/os-apis-at-call-sites` ·
  `contract/parse-rejected/hyprctl-json` · `contract/run-span-attributes/run-window` — hold
- a11y-plan · `decision/hyprctl-read-deadline` · `decision/window-wait-bound` — hold (each added read is a 5 s bounded
  seam read; no new bound on a new bearer); `contract/tui-passthrough/human-keystrokes` ·
  `contract/tui-passthrough/viola-terminal-bytes` · `decision/human-keystrokes-never-blocked` ·
  `decision/start-verify-stop-grace-off-the-human` — hold
- **Read, not reached**: obs-plan `decision/no-error-reporter` · `decision/no-network-otlp-exporter` ·
  `decision/serde-errors-fixed-message` · `decision/panic-hook-replaced` · `decision/serde-errors-become-fixed-message` ·
  `decision/serde-errors-become-internal-error` (a «placed by» search hit inside «replaced by»); security-plan
  `decision/pinned-copy-no-clobber-placement` (the pinned copy's placement, not a window) ·
  `contract/cli-json-error/internal-error`; layout-templates `contract/viola-send-output/json-mode`.
- **Escalate rows read** (before this block): `rul-r74hshif`, `rul-p2zl2kgl` (boundary widening) — reached by arm M1
  and by arms V2, V3; `rul-o2tr55aj` — not reached; `rul-bznresq5` — reached only once he answers a widening fork.

## Fork M — the move's form (his)
Under every arm, until it lands, `open`, `up` and a revive under an origin take the screen to the placed window's
workspace: a defect against F4, not a design.
1. **M1 — restore within the set (Recommended; taken PROVISIONAL)**: `Monitors` before the move, the move as built,
   `Monitors` after, and `Workspace(prev)` only when the active workspace moved (`inputs#I2` T2). Builds now: no new
   form, no ninth request, no fixture re-recorded. Cost: a widening of the window step (above, his); a visible switch
   away and back; keyboard focus lands where Hyprland puts it on that workspace (unmeasured); **a failed restore leaves
   the view moved with no word on the line — the F4 breach is then silent to the human** (T3, kept PROVISIONAL), priced
   against a ninth placement word (amends the eight-word rows in design-system and layout-templates and the list / MCP
   item shapes) — his to choose.
2. **M2 — a silent form measured first**: the granted live re-record tries the candidate silent form and reads the
   active workspace before and after; the code changes to that form after the reading. Cost: the code step is gated
   on his grant and timing; a new measured form in `decision/hyprctl-direct-spawn` (a widening); 9 pinned sites, 1
   fixture and 3 derivations re-formed; the candidate may not exist on 0.56.2 (no Lua argument offers it).
3. **M3 — M1 now, M2 in a later chunk**: M1's cost now, and a new route entry for the silent form.
4. **M0 — no fix**: v2-07 stays unverified; F4 stays a defect.

**His answer (2026-11-25T06:22:26Z, relayed verbatim by overseer1, `inputs#I3`):** «M1 — вернуть стол после переноса
(Recommended)», no text added: the eight placement words stay, no ninth word; T3's PROVISIONAL is discharged.

## Fork V — the automatic verify's place and its self-caused red (his)
1. **V1 — verify refuses to stamp on its own failed precondition (Recommended; taken PROVISIONAL)**: when a run of
   B–E reads its first screen as the trust form (or the external-imports form), verify ends with a new closed refusal
   code (proposed `untrusted-folder`), a fixed `unable:` / `hint:` pair naming no path, exit 1, **no stamp**; the
   start's automatic verify then reads the existing `no-stamp unstamped` line, the version stays un-attempted, and a
   later start from a trusted folder verifies it. No new read, no new write to Claude's config. Cost: amends the
   ratified failing-row rows (above); one code added to the closed verify-refusal sets; a hand-run verify in an
   untrusted folder refuses too (its `--help` paragraph already asks for a trusted folder). The stamp already kept on
   his own home for 2.1.331 is not this chunk's to touch (this phase reads nothing of `~/.viola`).
2. **V2 — the automatic verify runs in a folder of viola's own**, trusted by verify's own trust record, removed after.
   Cost: widens `decision/verify-own-trust-record` (a write into Claude's config) — **a boundary widening**; more
   residue on a SIGKILL.
3. **V3 — the start declines to verify from an untrusted folder**, reading the trust walk before its verdict. Cost: a
   new read of Claude's config before a verified verdict (**a widening**) and a new start-line word (amends the
   ratified auto-verify line set).
4. **V0 — no change**: a start in an untrusted folder keeps stamping a red the CLI did not cause.

**His answer (2026-11-25T06:22:26Z, relayed verbatim by overseer1, `inputs#I3`):** «V1 — отказ без отметки
(Recommended)» — the closed code `untrusted-folder`, no stamp.

**Wording item for the epoch batch (delegate overseer, `inputs#I4` point 3):** step 9's fixed pair is new
human-facing wording his V1 answer did not see verbatim — unable: `claude did not start ready in this folder: it is
not trusted` · hint: `run viola verify from a folder you trust in Claude Code`. Built as written, PROVISIONAL; the
rows that hold it land PROVISIONAL at the wrap, his word on the words owed.

## Fork L — the live legs (his to grant and to time)
- **The granted re-record** (one terminal window, no claude, unlocked): under M1 the move's form does not change, so no
  fixture needs re-recording; the grant stays unspent (or spent on M2's measurement if he takes M2 / M3).
- **R7 — the ONE live reading v2-07 rests on** (Recommended shape): one `viola open --workspace N` under a scratch home
  outside any repository, the fake agent standing as `claude` first on its PATH (no real claude), the real `foot` and
  the real Hyprland; `hyprctl -j monitors` read before and after; verdict words from a closed set (`r7: green
  placement=placed active=unchanged` · `r7: red active=moved` · `r7: red placement=<word>` · `r7: refused locked`).
  It opens ONE terminal window, strips every inherited `VIOLA_*`, refuses while `hyprctl locked` reads `true`, ends the
  instance and its terminal on every exit path, and has a rehearsal over the fakes that cannot reach the live step.
  Alternatives: beside a live parent (two windows, R2's shape); with a real claude (one capped start). **v2-07's claim
  is planned GATED on R7.**

**His answer (2026-11-25T06:22:26Z, relayed verbatim by overseer1, `inputs#I3`):** «Вечером, когда я за столом
(Recommended)» — R7 fires this evening, he at the host, unlocked; no word for a run without him; the granted fixture
re-record stays unspent under M1; v2-07's claim stays GATED on R7.

## Fork W — one implement window (his)
- **W1 — one window (Recommended)**: the move (`src/run/window.rs`, its unit cases, the fake's terminal-mode answers,
  `cli_open` / `cli_up` / `cli_revive`, the R7 script and its rehearsal) and the folded item (`src/cmd/verify.rs`, its
  pair, the closed-code census rows, `schemas/diag-line.v1.json`, `cli_verify` / `cli_version_gate` witnesses) — about
  16 files; it fits, by the size of the last three chunks.
- **W2 — split**: the folded item as its own route entry after this one, before Epoch 5's boundary. A split is his.

**His answer (2026-11-25T06:22:26Z, relayed verbatim by overseer1, `inputs#I3`):** «W1 — одно окно (Recommended)».

## Whether the entry builds whole on the PROVISIONAL arms
**Yes** — on M1 · V1 · W1 with R7 planned as a gated, operator-fired leg, the entry builds whole now; v2-07's claim
waits on R7. The phase goes on to P4 without waiting.
