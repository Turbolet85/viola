# P5 review — 2026-11-13-rows-proposal-rule-work · «Rows-proposal rule work»

The plan to read in an editor: `.andromeda/cache/docs/2026-11-13-rows-proposal-rule-work/plan.md` (rendered from
`doc:2026-11-13-rows-proposal-rule-work/plan`, rev 4). No plan stands in a chunk's folder; writing in that file changes
no record until a `put`. A diff of the chunk's records: `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-13-rows-proposal-rule-work`.

## Goal
Close the eleven rule lines the rows landing 2026-11-13T01-15-10 owed the route, prove the link replay test a guard of
the replay row, and fold `digit_runs` into one shared copy. Every item narrows a boundary or tightens a test; no new
refusal reason, exit code, event kind, wire field or seam.

## Steps (12) and touchpoints (1 new · 25 modified)
1. Hints: `unverified-cli` on `send`, every `not-delivered` detail on `answer`, the design strings byte for byte
   (`src/human/hints.rs`, `src/cmd/send.rs`, `src/cmd/answer.rs`); `unknown` keeps none.
2. `--record`'s `text_fault` refuses an email on every non-screen kind (`ledger.rs`).
3. `tests/support/hygiene.rs` gains the `email` class; every walk and a planted email per kind.
4. `run` parses through `send::parse_name`.
5. `strict.rs` `unix::reading` stops following a link → `Refused::Symlink` at every trusted-state check.
6. `fs.rs` `open_private_append`: `O_NOFOLLOW`, mode set on the handle.
7. `tui_pty_seam.rs`: no key after the resize.
8. `tui_channel_fds.rs`: waits on `start`, no self-dump, no count, `/proc/<pid>/fd` always; the `GATES` reason.
9. veil `Redact` on `hook::Normalised`, `hook::Known`, `dialog::Dialog`, `dialog::Response` and the five MCP input
   types; veil edges into `viola-agent-claude` and `viola-mcp`.
10. The replay guard: no code change.
11. `tests/support/digits.rs`; the three copies go; one `digit_runs_reads_whole_numbers_only` case stays.
12. Six remove-the-guard controls through `gate.py control` at implement (link-replay · strict-symlink ·
    append-nofollow · record-email · hygiene-email · redact-hook).

## Top acceptance criteria (11 in the plan)
- Every refusal but `unknown` ends with its hint; no two codes share one; none names release.
- Email refused at the record (`Unclean::Email`) and on every hygiene walk; every committed fixture still clean.
- A symlink at the home, `instances/`, an instance dir, `snapshot.json` or `stamps.json` reads `Refused::Symlink`.
- The append open refuses a link and narrows through the handle.
- `e5_the_replay_reads_the_link_the_wrapper_wrote` green alone, and red with the replay's link pass removed.
- Each of the six controls `holds`; `pre-push` `"ok":true`; the operator pass's CI `verdict: green`.

## Test commands
35 `[[gate]]` entries: fmt · clippy · G1 · lint-probes · orphans-check · check · `cargo deny check` · the per-sync-crate
bans · nextest `viola-state` / `viola-agent-claude` / `viola-mcp` · the focused `cli_links` E5 and
`contract_fixture_hygiene` runs · `agent-run.sh run --unit / --integration / --e2e` · five grep probes · release-check ·
the smoke boot/status/cleanup with G2 and G4 · `pre-push` · the operator trio (hygiene, push, `ci.py`). 18 entries
`new = true`, each baseline-run now (the five probes red as predicted, everything else green); 14 seen green at the
last wrap; 3 operator legs.

## Capabilities
None claimed: none of the 28 pooled `viola-0.2.0` capabilities is this chunk's (rule work, no affordance), so no
concretization preview.

## Directive item 5 (`inputs#I1`)
- The union, as `rec.py rows-union` printed it:
  ```
  architecture: rows 987 · packs 10 · picked 262 (131 by every draw) · standing 8 · 72616 B
  security-plan: rows 615 · packs 7 · picked 185 (114 by every draw) · standing 4 · 48661 B
  design-system: rows 325 · packs 4 · picked 136 (106 by every draw) · standing 0 · 32450 B
  layout-templates: rows 224 · packs 3 · picked 79 (55 by every draw) · standing 0 · 16923 B
  test-plan: rows 1375 · packs 14 · picked 343 (187 by every draw) · standing 57 · 80830 B
  obs-plan: rows 1058 · packs 11 · picked 252 (149 by every draw) · standing 13 · 64273 B
  a11y-plan: rows 241 · packs 3 · picked 94 (59 by every draw) · standing 14 · 25135 B
  rows-union: 7 master(s) · picked 1351 of 4825 · rule any
  ```
- Every row the plan cites is in the union: 38 handles cited, 37 found in their master's rows file, plus
  `design-system · contract/refusal-hints/not-delivered-*`, a wildcard over six rows that are all in the union.
  None missing. One citation was misspelt `arch ·` and is now `architecture ·` (plan rev 4).
- `gate.py control`: no control ran at phase. The six controls are implement's one-shot run (plan step 12), each
  on a gate entry that read green on this tree at P5: entries 10, 11, 13 and 14. Their clone is named only when they
  fire.
- `[RETYPED]` lines, verbatim:
  - `arch-pick-p1.d3 · [RETYPED] line 85: decision/budget-gate-never-polls-and-fails-open is no row of the pack — read as decision/budget-gate-fails-open, the pack's row 85`
  - `arch-pick-p4 · [RETYPED] line 22: decision/fake-agent-test-seam-variables is no row of the pack — read as decision/fake-agent-test-seams, the pack's row 22`
  - `tests-pick-p7.d3 · [RETYPED] line 98: decision/os-gated-tests-each-named-with-reason is no row of the pack — read as decision/os-gated-tests-each-named, the pack's row 98`
  - `tests-pick-p9 · [RETYPED] line 63: decision/synthetic-fixture-text-only is no row of the pack — read as decision/synthetic-fixture-text, the pack's row 63`
  - `obs-pick-p1.d2 · [RETYPED] line 84: decision/dialog-text-never-logged-on-answer is no row of the pack — read as decision/dialog-text-never-logged, the pack's row 84`
  - `obs-pick-p6 · [RETYPED] line 1: contract/hook-decision/socket-dir-refused is no row of the pack — read as contract/hook-decision-detail/socket-dir-refused, the pack's row 1`
  - `obs-pick-p8.d3 · [RETYPED] line 45: decision/claude-agents-rows-never-logged-verbatim is no row of the pack — read as decision/claude-agents-rows-never-logged, the pack's row 45`
- `[EVERY_ROW]` re-spawn: `arch-pick-p9.d2 · [EVERY_ROW] line 18: decision/sanitised-error-surfaces — the pack's order
  holds decision/statusline-source-follows-home here`. It was re-spawned once in `pick-p1-r1` and passed. The same
  re-spawn took `layouts-pick-p2.d2 · [ABSENT]`, whose agent returned without writing its file. It also passed.
- Non-zero exits, refusals and verdict lines; places a letter was wrong or silent; row-read surprises:
  - The CARRY was wrong: it says `fn digit_runs` stands twice, but HEAD holds three copies (the third is
    `tests/security_negatives_strict_modes.rs:60`, added at `af6c67ef`). The scope marks it
    `[premise-corrected]`.
  - At P5 the must-pass control found a defect in my own probe 19. As first written (`fn digit_runs`) it read red on
    the built input because it also matched the kept case `fn digit_runs_reads_whole_numbers_only`. Re-spelt
    `fn digit_runs(`, it reads green there (`mustpass-controls.md`). The first baseline loop fired the old spelling;
    entry 19 was re-baselined after the fix.
  - The code-graph query regenerated its plane and outlived the Bash tool's 120 s limit. It finished in the
    background (exit 0, 50 rows).
  - Hook and harness refusals, each re-issued once in a form they allow:
    - the Bash guard refused two `cd` calls (into the skills dir and into the run dir);
    - it refused a `rec.py render` piped into `head`;
    - the stamp-ahead hook refused an estimated `Generated` stamp in the plan;
    - the harness refused a `sleep` (I waited for the notification instead).
  - The baseline reads on the untouched tree are the predicted reds of the five probes (entries 19-23). No other
    exit was non-zero. No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line was printed anywhere in the run. At
    Setup, CI on `cb0b16ab` had no verdict yet; read again at P5 it was `verdict: green · checks 7/7 · wall 683 s`
    (ci#43499052863).
  - Row-read surprises: design-system picked 136 rows and layout-templates 79 for a chunk whose only human-output
    change is hints. 327 of the 1351 picked rows were picked by one draw only.
  - My own row-citation checker first bound every handle of a long list to the first master it named. I fixed it
    before taking the reading above.
- No new keyed contract is needed, and no NOT YET MEASURED piece of a mark is cleared.

## Validation
- Mechanical checks: all pass. Size is about 220 prose lines.
- planlint: 0 hits. It listed two «unchanged» claims, and both hold:
  - line 95: redaction changes `Debug` only, and the MCP input types are never serialised;
  - line 411: `run`'s bad-name refusal text is identical.
- `inputs.py verify`: I1 unchanged. The scope lists parse (26 items).
- Validation-1: aligned. Each of scope items 1-13 has its step, and the boundaries hold.

## Beyond the plan
none
