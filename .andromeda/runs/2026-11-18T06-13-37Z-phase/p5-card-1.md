# P5 card 1 — 2026-11-18-trust-record-row-read-live, Revision 2 (the changed lines only)

Plan: `doc:2026-11-18-trust-record-row-read-live/plan` rev 6, rendered to
`.andromeda/cache/docs/2026-11-18-trust-record-row-read-live/plan.md` to open in an editor. No plan stands in the
chunk's folder; writing in that file changes no record until a `put`. `python -X utf8
~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-18-trust-record-row-read-live` diffs the chunk's
records.

**Cause:** implement stopped before the operator pass on S0R-2 RED (`7/23`, `seed-markers: written`; B, C, D, F
`none`, each `lit=rule@1 adj=no first=rule col=80`, the frame of Run A's trust dialog). **Word:** inputs#I7 (the
operator), inputs#I8 (the founder's answer to F1, «Один старт claude на риге», and the overseer's relay).

## What changed
- **Metadata** gains the Revision 2 line.
- **New section «Revision 2»:**
  - What is done: steps 12–16, with S0R-2 RED and step 16's second end taken. S0R and S0R-2 are not re-fired.
  - One premise correction: step 15's fake-agent oracles (see Acceptance below).
  - The word, verbatim in substance.
  - The start's shape, measured against the code:
    - argv: Run F's, `--model haiku` alone.
    - environment: exact, with no difference left. The rig's `child_env` is what verify's child got, because verify
      sets nothing and strips only `CLAUDE*` names outside `claude_env_keep`, and that list keeps
      `CLAUDE_CONFIG_DIR`.
    - pty: `Size::DEFAULT` 80×24.
    - config: the V seed (git root, trust record, six markers), with the seeded root as the cwd. Unwrapped.
    - What is left of verify's shape, named: the timing (8.5 s against the rig's settle of 60 s) and the sequence
      (the print probe, then Run A killed).
  - Implement re-enters at **step 17**.
- **Steps 17–20 (new):**
  - 17 — the `VC` arm in `rig/mod.rs`, `rig/screen.rs` and `viola-live-rig.rs`:
    - It runs the V seed, starts the child at 80×24 (the rig's `Model` gets a size; every other arm stays at 210×45)
      and prints `pty: 80x24`.
    - It reads `first_frame`, then prints `screen:` (verify's own classifier) and `facts:`. The facts row is built in
      the stamp's `first_facts` shape and printed through the existing `facts_line`, so it reads in S0R-2's
      spelling.
    - It prints `keys: none` and stops with `stop_no_key`.
    - The label `S0C` alone maps to it, with no spare label.
  - 18 — the cases:
    - `rig_rehearsal_vc_reads_its_first_frame_at_80x24_and_stops_with_no_key`, on the fake with `--trust-config`. It
      checks the fake's `size` receipt at 80×24, `box: up`, `screen: box`, `stop: killed-no-key`, no key receipted
      and the teardown.
    - `the_arm_words_and_labels_parse` is extended.
    - `a_sized_model_reads_its_rows_at_its_own_width` (new).
  - 19 — the block green and landed before the start.
  - 20 — the naming start `S0C`:
    - It runs under its OWN ledger `evidence/name-start.ndjson` with `--cap 1 --spare 0`. The R6 ledger is
      untouched, so `S0R-3` stays unspent.
    - Implement reads the kept local frame and writes only closed words: the name (one of the six), the option
      count, the selected ordinal, owner-action yes/no and literal-match yes/no. It sets the `facts:` row beside
      S0R-2's `facts-b` and `facts-f`.
    - `box: up` is a reading: verify's own doing. There is no second start either way.
    - Then implement STOPS before the operator pass and appends F1's update with the three roads.
- **Touchpoints:** two files added — `crates/viola-e2e/src/rig/screen.rs` and
  `crates/viola-e2e/src/bin/viola-live-rig.rs`. `rig/mod.rs` and `rig_rehearsal.rs` gain their Revision 2 lines.
- **Test Commands:**
  - The prose now names the naming start and the stop.
  - The viola build/hash entry is removed, since `S0C` runs no viola.
  - The S0R-2 live entry is replaced by:
    `cargo run -q -p viola-e2e --bin viola-live-rig -- live S0C --cli 2.1.325 --deadline 300 --settle 60 --cap 1
    --spare 0 --ledger .andromeda/cache/chunks/2026-11-18-trust-record-row-read-live/evidence/name-start.ndjson`.
    - Its timeout is 600.
    - Its atoms: `exit 0`, `cli: 2.1.325`, `git-root: written`, `seed-markers: written`, `pty: 80x24`, `modal:`,
      `box:`, `box-text:`, `trust:`, `screen:`, `facts:`, `keys: none`, `stop: killed-no-key`, `child: stopped`,
      `census: 0`, `credentials: removed`, `scratch: absent`, `user-config:`, lacks `changed-by-rig`, lacks
      `user-config: unread`, lacks `refused:`, lacks `stopped-by:`.
    - Its note: the floor does not apply.
  - The census notes now name S0C.
  - One control is added, `vc-nokey`, with `--timeout 900`. It switches the VC stop from `stop_no_key` to `stop`, its
    case is the vc rehearsal, and it fires on entry 5.
  - Entries 1–7 are unchanged and keep their baselines (`gate.py baseline`: 2 seen green, 5 new (key) carried over).
    Entries 12–18 are unchanged (operator).
- **Acceptance:**
  - Corrected: over the fake, A reads `lit=none adj=no first=text col=27` and B–F read `lit=footer-agents@23 adj=no
    first=text col=33` (as built). This replaces «B–F `rule_above_prompt` true», which the first run disproved.
    Adjacency stays proven at the unit tier.
  - New (tests, Rev 2): the vc rehearsal, the labels and the sized model.
  - New (tests, Rev 2): `vc-nokey` holds.
  - New (live, Rev 2): `S0C` runs once and the dialog is named in closed words. No spare, `S0R-3` unspent, and
    implement stops before the operator pass.
- **Constraints:**
  - No second naming start, no spare and no `S0R-3`.
  - No byte into the dialog.
  - No frame text, account detail or config value in a record.
  - Nothing accepted for the owner.
- **Implementation notes:**
  - The lean: no boundary widening. Security `decision/live-session-screens-kept-local` keeps the frame local with
    closed words landing, and the `IM` arm already starts an unwrapped `claude` with no key. So no escalate pattern
    is met and no fork is asked.
  - The atom line numbers are read on the tree Revision 1 left; implement re-reads them.
  - An expected amendment: test-plan `decision/standing-live-rig` add-to (the `VC` arm).
  - Provenance.
- **Scope rev 5:**
  - The Revision 1 cause bullet is `[premise-corrected]` by S0R-2's reading.
  - The naming-start bullet is new, closed against the code (the environment is exact).

## Checks
- Dry-run: `entries 18 · not-run 18`.
- planlint: `hits: 0`. The 12 (d) claims were read: the R6 ledger «not touched» holds, since no entry writes it now.
- `inputs.py verify`: 8 entries, 0 drifted. I6 (the Revision 1 word) is uncited, as it stood.
- Research lists: `scope lists: parsed 16 items`.
- List sync: `scope sync: PASS · delta +2 −0`.
- Host: `~/.local/share/mise/installs/claude/2.1.325/claude --version` → `2.1.325 (Claude Code)`.
  `mise ls claude` now reads 2.1.327 as `latest`. It is not used: the word names 2.1.325.
- `evidence/name-start.ndjson` is absent, so a fresh ledger starts.
- Validation-1: aligned with scope rev 5 (the naming-start bullet) and the working entry.
- Research: lists synced to the revision's delta (+2 −0), rev 4; not re-derived since the take-up's rev 2.
- No capability is claimed (the chunk has no coverage row).

## Beyond the plan
- (standing — `.andromeda/runs/2026-11-18T01-31-02Z-phase/p5-card-1.md`) none, as carried from
  `.andromeda/runs/2026-11-18T00-06-23Z-phase/p5-card-2.md`.
- (standing — `.andromeda/runs/2026-11-18T01-31-02Z-phase/p5-card-1.md`) F2, the product finding (verify in a
  first-run config at 80×24 reads its interactive rows `fail`): its options are the founder's, no owner route entry
  proposed; recorded in `founder-forks.md` only. S0R-2 did not confirm its premise, so it stands as a question for
  whatever the naming start names.
- (standing — `.andromeda/runs/2026-11-18T01-31-02Z-phase/p5-card-1.md`) S0R-3 is left for the founder's word (R6's
  second spare), not spent by this chunk; Revision 2 keeps it unspent.

Apply? (yes / review / cancel)
