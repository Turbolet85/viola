# Adaptation record — the 2026-11-08T21-01-50-wrap 0-pending wrap (the 0.2.0 census, part 2)

**Request:** finish the 0.2.0 version census (`doc:viola-0.2.0/census`) after `0db04b87`, on the same card word (the
founder's own, typed 2026-11-08T19:59:58Z, «confirm the card as proposed»), skipping every row
`.andromeda/runs/2026-11-08T20-32-52-wrap/adaptation-record.md` marks applied, noted or skipped. The directive:
`inputs#I1` (the Viola overseer, 2026-11-08). The operator's answer on `doc:input`: `inputs#I2`.

**Status: COMPLETE.** Every census row now reads applied, noted, held with a reason, or skipped. No row is left open.

## The four points the first pass raised
- **C-240 — not applied: the premise is false.** The row retires a11y from test-plan §1's downstream readers of the
  test tier (fct-imyfbj3w). But the a11y plan stands: fct-loqatd4n (`A11y tier: Standard (1)`, cli and tui scope), and
  its pyramid reads the test-plan unit tier (fct-lchghj7u; a11y-plan/aria-patterns-roles fct-uok334yy). The route entry
  «Fleet a11y verification» owns its 0.2.0 scope too. Removing a11y as a reader would write a false statement (the
  overseer's answer, `inputs#I1` item 2).
- **C-301 — held for the route entry «Status-line interception removed».** Its subject, the status line's `/bin/sh -c`
  shell-out, is built (search S-B: `statusline_command` 22 files), so its `built: no` was wrong. That chunk's wrap
  retires the text along with the code (`inputs#I1` item 2).
- **C-010 — applied with the page,** as the 2026-11-08T20-32-52-wrap record states. No change (`inputs#I1` item 2).
- **The 14 `clause retired · note now` rows — noted.** Each is a dated `matrix.py note --dir viola-0.1.0` that cites its
  `C-{nn}`, with status, ref and acceptance untouched (every one reads `verified`). The clause text is re-said at the
  0.2.0 chunk that claims the successor. C-660 v1-02 · C-662 v1-04 · C-666 v1-08 · C-667 v1-09 · C-668 v1-10 ·
  C-672 v1-14 · C-676 v1-18 · C-683 v1-25 · C-687 v1-29 · C-698 v1-40 · C-700 v1-42 · C-701 v1-43 · C-702 v1-44 ·
  C-704 v1-46 (`note-v1-*.md`; each landed `1 entry changed`, trail `matrix-no-marker.json`).

## `doc:input` — 23 rows, not applied
C-459, C-460, C-461, C-462, C-463, C-465, C-466, C-467, C-468, C-469, C-470, C-471, C-472, C-474, C-475, C-476, C-478,
C-479, C-481, C-483, C-485, C-486, C-487: **not applied — doc:input is never mutated (written once at arch).** Its
current truth lives in the masters and the 0.2.0 intent. Whose word: the Viola overseer's technical answer
(founder-delegated) to this wrap's dialog, `inputs#I2`. The wrap had proposed a dated in-place tag. The answer notes
that a tag is still a mutation. The overseer files the census population's inclusion of doc:input as a pipeline finding.

## `.andromeda/project.yaml` — C-488 applied
`description` now reads «A standalone, Linux-only bridge that lets one interactive Claude Code session drive another on
the user's own subscription, its fleet of sessions tiled on Hyprland.» The edit was made with the Edit tool. The file is
plain and not in the store.

## Preserve-verbatim homes — corrected in place with a dated tag, nothing deleted
Each entry keeps its text and gains `[retired 2026-11-08, census C-{nn} of doc:viola-0.2.0/census…]`. A section entry
gains a `**Retired 2026-11-08** (census C-{nn} …)` line under its heading. Each file was drafted whole
(`draft-{path}.md`) and landed by `rec.py harness land`, with a `--dry-run` first. `gate.py hygiene` refused two drafts
because they copy host-path examples that stand in the landed files: `draft-.claude-docs-gotchas.md:38` (drive) and
`draft-.claude-docs-session-learnings.md:255` (tmp). Both were moved to the session scratchpad, by hand. Their text is
the landed renders, TX 794 and TX 793.
- `.claude/rules/ci.md` (TX 789): C-395, C-396. Both already carried the 2026-11-06 history tag.
- `.claude/rules/testing.md` (TX 790): C-438 through C-444. In C-442 the Unix fact stands, and the tag says so.
- `.claude/rules/testing-src.md` (TX 791): C-445 through C-448, the pointers.
- `.claude/rules/verification-harness.md` (TX 792): C-456, C-458, and C-457, where only the Windows `TMP` half is
  retired and the Linux NOCOW half stands.
- `.claude/docs/session-learnings.md` (TX 793): C-596, C-600, C-601, C-607.
- `.claude/docs/gotchas.md` (TX 794):
  - C-556: the What-breaks line is tagged and the warning clause struck through with its tag. The stdin / `--file`
    rule (C-557, keep) stands.
  - C-560 and C-563: the Windows measurements are tagged. The product window and the test-only rule (C-561 applied,
    C-562 keep) stand.
  - C-564: the sideloaded ConPTY DA1 stall section is tagged.
  - C-566: the Windows pipe section is tagged, and its Unix stop and kill waits are named as not retired.
  - **C-565 held — premise false:** the `Piped` DA1 answer is built. `tests/support/piped.rs:11-45` (`DA1`,
    `DA1_ANSWER`, the answer write) still stands, though the row reads `built: no · search S-O`. The C-564 tag names
    it as held.

## The four doc leaves no comparer flagged — applied as plain replaces
- C-522: `.claude/docs/conventions.md` (TX 795), the `http_` and `sse_` prefixes dropped.
- C-582: `.claude/docs/security-summary.md` (TX 796), «on every OS» → «on Linux».
- C-635: `.claude/docs/services/viola-core.md` (TX 797), `SSE` dropped from the names `ViolaName::try_new` takes.
- C-657: `.claude/docs/services/viola-state.md` (TX 798), «on every OS» → «on Linux».

## Observed, not acted on
`crates/viola-e2e/src/harness/run.rs:349`, `:1026`, `:1097` still hold `#[cfg(not(unix))]` other-OS ledger-row and
live-pass constants, which are dead on the one target. The census's S-O basis names only the OS-refusal sites
(`tests/contract_os_gated_tests.rs`, `crates/viola-e2e/src/harness/run/mutants/host.rs`). These leftovers sit outside
any census row and are reported to the operator. This wrap edits no source.

## Not done here
No gated record stands (the `pending` query reads 0 rows). No curation: the conversation carried no correction to
curate. The census rows are applied by their own rule here, not through P3.
