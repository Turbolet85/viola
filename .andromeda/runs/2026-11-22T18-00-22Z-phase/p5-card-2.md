# P5 review — «Epoch 4 cleanup» (`2026-11-22-epoch-4-cleanup`), card 2 — plan rev 4

Card 1 (`.andromeda/runs/2026-11-22T18-00-22Z-phase/p5-card-1.md`) answered `review` with the overseer's relay,
snapped `inputs#I2` (delegate: the overseer; no word of the operator's moves). Folded at P4, these five points and
only these. The plan's design stands. **Open the plan:** `.andromeda/cache/docs/2026-11-22-epoch-4-cleanup/plan.md`
(rev 4); a diff of the chunk's records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root .
2026-11-22-epoch-4-cleanup`.

## The changed lines
- **plan rev 3 → 4** (`put` tx 4032: 2 new · 5 revised): Goal — the agents read «gets a named constant of its own,
  keeping its standing 5 s … the real CLI's cold agents read was not measured … the finding … is NOT mended here: it
  stays a carry»; step 3 — the constant «documented as the read's own and as not measured against the real CLI»
  (was «measured 2026-11-22 quiet: 171 ms first, 108–113 ms after»); the (bound) criterion — «its value the standing
  5 s … does not claim the stalled-host false red mended»; Constraints — the raise rejected because «the real CLI's
  cold read is unmeasured», and a new line rejecting «measured» for the 5 s anywhere; Implementation notes — the
  security-plan Expected amendment «still 5 s, the version probe's value kept and not measured against the real CLI»,
  and a new route-freight line: the false red stays a CARRY owing the real CLI's cold agents read, quiet and
  IO-throttled, a start the operator caps. The `[[gate]]` fence is byte-equal: every baseline stands.
- **research rev 2 → 3** (tx 4030): measurement 3 re-titled «the FAKE agent's only», its conclusion rewritten as
  above; measurement 7 added (the feasibility probe, below).
- **scope rev 3 → 4** (tx 4031): under `cry-nk4gcrnw`, `[premise-corrected: …]` — the measurement it asks cannot be
  made in this chunk; this chunk takes the named constant; the finding stays a carry (validation-1: intent-incomplete,
  the scope amended).
- **founder-forks.md**: a dated correction appended under its own heading (19:00:06Z), nothing above edited.

## Item 4, as it now stands (relay point 1)
A named constant for the agents read, value 5 s kept, a case pinning it. **Not mended:** «verify's agents read reads a
false red under a stalled host» — a CARRY for this chunk's wrap to re-pin, owing the real CLI's cold agents read, quiet
and with its disk reads throttled; it needs a start of the real `claude` the operator caps. No sentence calls the 5 s
measured for the real CLI (`grep -n -i measured` over the plan: the eight hits are the unmeasured wording, the chrono
heading and the SIGKILL residual).

## The feasibility probe (relay point 2; research measurement 7; no `claude`, no plan step)
- Device: `/dev/mapper/root` (`dm-0`, 253:0, `crypt`) under both `~/.local/share/mise` and `target/`.
- Controllers, read only: `user-1000.slice` → `cpu memory pids`; `user@1000.service` → `cpu memory pids` (its
  `subtree_control` the same). **`io` is not delegated** (systemd 262.1).
- Reader `target/debug/viola-harness` (121 408 448 B), each under `timeout 60`: no limit — `cat` **87 ms**, `dd
  iflag=direct bs=1M` **14 ms**; with `systemd-run --user --scope -p "IOReadBandwidthMax=/dev/mapper/root 1M" -- …` —
  `cat` **20 ms**, `dd` **18 ms** (1 MB/s would need ~116 s).
- **The limit did not take, and nothing refused it:** systemd started the scope (`Running as unit:
  run-p1170764-i739034345.scope`) and set no limit; the same scope's cgroup lists **`io.max absent`**. The reads ran at
  page-cache speed (10–11 GB/s), so a cold read also needs the file out of the cache, which no unprivileged call gives.
- **Answer:** one child's disk reads cannot be throttled as this user on this host; it needs `io` delegated to the
  user manager (a root change of the host's systemd configuration, the operator's) or a root-run scope.

## The seven provisional rows — why each waits, and whether anything has supplied it (relay point 3; no row changed)
Read from each row's source fact (`src:`, the text masters before the rows conversion, tx 803) and its sidecar entries.
- **architecture `decision/compact-confirm-window`** — 40 s «PROVISIONAL, validated per CLI version by the
  `local-command-compact` row against the measured `compact_latency_ms`» (`fct-va43dnf2`): it waits for each version's
  measured compact latency to stand inside 40 s. Supplied in part: the row checks it per stamp (`ledger.rs:848`,
  `within(compact_latency_ms, COMPACT_CONFIRM_WINDOW)`), one live reading 21 413 ms on 2.1.287 (chunk
  2026-10-18-compact-post-condition-ledger-row); no live driver `/compact` send, and no word ending the mark.
- **obs-plan `decision/compact-confirm-window`** — the same 40 s, «PROVISIONAL» in three source facts (`fct-o5zzhzdu`,
  `fct-xfymn57j`, `fct-u2zbdxyd`) with no reason of their own beyond the architecture fact's; the same state as above.
- **test-plan `decision/compact-confirm-window`** — the same 40 s, «PROVISIONAL» (`fct-qbytqn26`); the same state.
- **architecture `decision/hook-connect-deadline`** — `CONNECT_DEADLINE` = 750 ms named «(provisional)» at chunk
  2026-10-04-readiness-gate-and-timing-constants (`amd-42usnaum`: the timing constants «PROVISIONAL and not measured»,
  owed «to the first live test or the founder's ruling»); it waits for a measurement of the hook's connect time or his
  word. Not supplied: its siblings got ledger rows (the confirm window's `confirm-window` row), this one none, and the
  perf gate measures the whole hook under `SPINE_DEADLINE`, not the connect.
- **test-plan `decision/hook-connect-deadline`** — the same 750 ms, «provisional» (`fct-ggkcj6j7`, `amd-ehj3ctqx`); the
  same state.
- **obs-plan `decision/spine-hook-budget`** — «tests-owned spine deadline, provisionally 1.0 s» (`fct-onnnq5uj`): it
  waited for architecture to name the constant. **Supplied** at chunk 2026-10-04-readiness-gate-and-timing-constants:
  `viola_core::SPINE_DEADLINE` = 1 s, and test-plan's own entry there dropped the same word («was a provisional
  `1.0 s` "until arch names it"», `amd-ehj3ctqx`); the obs row was never amended, so its mark reads stale.
- **security-plan `decision/hyprctl-read-deadline`** — the 5 s hyprctl deadline «applied ahead of its settlement
  against a measured need» (`amd-uc6w5fo7`, chunk 2026-11-14-hyprland-seam): it waits for a measured hyprctl reply time.
  Not supplied; the carry `cry-bcosdllb` on «`viola open`» names it (the boundary wrap's `founder-forks.md` item 1).

## The two equivalent mutants (relay point 4)
- **`crates/viola-agent-claude/src/ledger.rs:1327:26` · `replace + with * in question_permission_answered`** —
  `turn[at * 1..]` differs from `turn[at + 1..]` only by adding `turn[at]`, the PreToolUse `position` found; the
  `any(c.is(HookEvent::PermissionRequest, ASK))` over the slice cannot match a PreToolUse, so the result is equal for
  every input.
- **`crates/viola-agent-claude/src/screen.rs:135:20` · `replace + with * in Signatures::box_below`** —
  `rows.get(i * 1..)` differs from `rows.get(i + 1..)` only by adding row `i` to `holds_box`'s `windows(2)`; a window
  opening at row `i` needs `is_box_rule(row i)` (the rule char alone, ≥ 40), but `box_below` is reached only for a row
  that `holds_modal` (`:142`), which holds a modal literal and so is never a rule row: the result is equal for every
  input.

## Checks re-run on plan rev 4 (relay point 5), every one
`gate.py run --dry-run`: 22 entries, parsed, exit 0 · `gate.py baseline`: seen green 5 · not seen 0 · new 10 · leg 7,
exit 0 (the ten `baseline` keys stand: the fence is byte-equal) · `planlint.py check`: hits 0, exit 0 (12 (d) lists the
same three claims as card 1, each holding after the steps) · `gate.py scope --lists-only`: `parsed 10 items` · `inputs.py
verify`: `2 entries — unchanged 2 … uncited 0 · unparsed 0` · validation-1: intent-incomplete, scope amended (rev 4),
then aligned.

## Beyond the plan (card 1's rows standing, then this card's)
| # | Proposal the word would approve | Owner / place |
|---|---|---|
| 1 | (standing — `.andromeda/runs/2026-11-22T18-00-22Z-phase/p5-card-1.md`) The advisory re-read moves to the Epoch 5 boundary | the Epoch 5 boundary, by this chunk's wrap |
| 2 | (standing — card 1) (8) `framing_turns`' step function — not taken | the next boundary's code audit (M1) |
| 3 | (standing — card 1) (9) the one-source relied-row count and the stamped-set table — not taken | the next boundary's code audit (M3) |
| 4 | (standing — card 1) the real CLI's own `projects` entry per verify probe cwd, and the found entry's writer — unmeasured | «Version done-check», by the wrap's route adaptation |
| 5 | (standing — card 1) the scratch clone `$TMPDIR/{scratchpad}/ctl-clone` left in place | the operator, by hand |
| 6 | The finding «verify's agents read reads a false red under a stalled host» stays a CARRY, owing the real CLI's cold agents read quiet and IO-throttled — a start the operator caps, on a host where `io` is delegated (measurement 7: not here) | re-pinned by this chunk's wrap; its entry the operator's to name (it was «Version done-check»'s before the fold) |
| 7 | obs-plan `decision/spine-hook-budget`'s «provisionally» reads stale (its premise supplied 2026-10-04) | the operator's word on the seven rows; no row changed here |

Apply? (yes / review / cancel)
