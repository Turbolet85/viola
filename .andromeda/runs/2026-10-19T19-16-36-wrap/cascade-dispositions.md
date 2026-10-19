# Cascade dispositions — 2026-10-19-overseer-switch-to-viola

**The search.** `cascade-patterns.toml` holds nine patterns; one is refused unfired (`c-driftexp`, `expected` is always /
always `string`: no master states it, the refusal line is its basis). The eight run: `cascade.py sweep … --pattern
c-turnshape c-frame c-qshape c-hold c-from c-upstream c-residual c-takeover`, baseline f23586fb, trail
`cascade-2026-10-19-overseer-switch-to-viola.json`.

Each pattern spans an amended claim's old wording and its mechanism:
- the `turn-ended` shape `{last_assistant_message}` and the Stop → `turn-ended` mapping's data;
- the old example frame;
- the question shape `options:[label]` / "questions and options" / "option labels";
- the hold's first-live-reading placement;
- the `from` / `from_trust` presence claim;
- the upstream-text lists;
- verify's `~/.claude` residual and the probe dir;
- the takeover clause's count.

Counts, copied from the listing: `total (8 patterns) · 19 rows over 9 files` · `per class · new 2/2 · standing 11/3 · leaf 5/5 ·
curation 1/1 · base 0/0`. `c-frame` and `c-takeover` print 0 rows, each control having fired: the frame and the clause
stand only as amended.

## Rows
| row | disposition |
|---|---|
| `.andromeda/architecture.md:256` c-qshape new | amended: the new shape keeps `options:[<label>]` beside `option_details`, current truth |
| `.andromeda/architecture.md:77` c-hold standing edited ×2 | amended: the new text names "Version done-check"; the line is re-read and holds no duplicate of the old placement |
| `.andromeda/obs-plan.md:610` c-from standing edited ×2 | amended: both matches are the new presence rule |
| `.andromeda/security-plan.md:527` c-upstream new | amended: R1's list now names question and option text |
| `.andromeda/architecture.md:110` c-residual standing edited mixed ×3 | amended: the print-probe residual is added; the two other matches (`ledger/probes/<pid>/` the probe plugin dirs) are true, no change |
| `.andromeda/architecture.md:326` c-residual standing | no change: the probe plugin under `ledger/probes/<pid>/plugin/`, true |
| `.andromeda/architecture.md:365` c-residual standing | no change: the probe dir tree, true |
| `.andromeda/security-plan.md:115` c-residual standing @c5434 | no change: "viola writes nothing under `~/.claude`", true (the new residual is the CLI's write) |
| `.andromeda/security-plan.md:251` c-residual standing | no change: `plugin install` writes nothing under `~/.claude`, true |
| `.andromeda/security-plan.md:252` c-residual standing | no change: the probe captures dir, true |
| `.andromeda/security-plan.md:261` c-residual standing edited mixed ×3 | amended: the print-probe residual is added after "Run A leaves nothing"; the other two matches are true |
| `.andromeda/security-plan.md:262` c-residual standing | no change: viola itself writes nothing under `~/.claude`, true |
| `.andromeda/obs-plan.md:666` c-residual standing | no change: verify's captures under `ledger/probes/<pid>/`, true |
| `.claude/rules/events.md:20` c-turnshape leaf | joins step 3: its Stop → `turn-ended` data omits `background_tasks` |
| `.claude/rules/observability.md:41` c-from leaf | joins step 3: the expiry line's no-`from` statement, read against obs-plan:610 |
| `CLAUDE.md:34` c-upstream leaf | joins step 3: the upstream-text list, read against security-plan:527 |
| `.claude/rules/security.md:17` c-residual leaf | joins step 3: verify's residual sentence, read against security-plan:261 |
| `.claude/docs/security-summary.md:73` c-residual leaf | joins step 3 |
| `.claude/rules/testing.md:78` c-residual curation | no change: a curated entry on the record scrub's cwd under `ledger/probes/<pid>`, a different claim; P3 has nothing to extend |

No hit in a judgment base (`playbook.md`, `drift-base.md`).

## Leaves
Mode: **whole** (`cascade.py leaves --all`; this chunk closes Epoch 5, `route.py epoch`: Epoch 5 · markerless 0 · pending 1;
the wrap directive `inputs#I19` named it). The step-2 leaf rows joined by `--add` (`events.md`, `observability.md`,
`CLAUDE.md`, `security.md`, `security-summary.md`). 14 groups · 28 leaves · 344041 B.
- Unmapped `.claude/rules/host-linux.md (host)`: the host's own leaf, no comparer.
- Unmapped `.claude/rules/testing-src.md`: a pointer-only file into `testing.md` that states no master text, no comparer.

**Path check.** Before `tree c1df5956df1732f47e548927dc7911faa2a7e99c`, after `tree 9993df3a7bbd3652aa0c2ad420388fab4b687329`,
index and head unchanged. The 14 `A leaf-{group}.md` lines and ONE more:
`A .andromeda/runs/2026-10-19T19-16-36-wrap/route-no-marker.json`.
- That file is the orchestrator's own trail of read-only `route.py markerless` / `route.py epoch` calls made with
  `--run-dir` between the two snapshots, which the contract forbids.
- Ruled "Accept, recorded" by the overseer's technical answer, 2026-10-19T19:40Z (a delegate's). The snapshot pair's
  purpose holds: no comparer wrote outside its file.

**Counts.** From the tool's `total` line: `total · blocks 39 · applied 30 · preserved 8 · reshaped 1`.

| group | findings | applied | by hand | declined |
|---|---|---|---|---|
| a11y-plan | 9 | 4 (`a11y-summary.md:47`, `:53`; `a11y.md:12` ×2) | 0 | 5: `a11y.md:3`–`:7`, frontmatter globs (`crates/viola-ui/assets/**`, `e2e-web/…`) → carried to the founder's next batch, already on the handoff ("`.claude/rules/a11y.md`, `testing.md`, `api.md` / `frontend.md` still glob `e2e-web/**` and `crates/viola-ui/**`") |
| architecture-1 | 3 | 3 (`viola-channel.md:25`; `viola-agent-claude.md:24`, `:25`) | 0 | 0 |
| architecture-2 | 3 | 3 (`viola-mcp.md:32`, `:36` tree; `viola-pty.md:27`) | 0 | 0 |
| architecture-3 | 2 | 2 (`events.md:20`; `stack.md:23`) | 0 | 0 |
| architecture+design-system+layout-templates | 3 | 1 (`frontend.md:28`) | 0 | 2: `frontend.md:3`, frontmatter glob (the same founder batch); `frontend.md:27`, a heading the tool preserves, a scope detail the section's body already carries |
| architecture+security-plan | 5 | 3 (`api.md:24`, `:25`, `:30`) | 1: `api.md:35` (the tool read the `\|` in `control-character\|empty-text` as a table cell; it is a list item, written with words in place of the pipe) | 1: `api.md:4`, frontmatter glob (the same founder batch) |
| architecture+test-plan+obs-plan | 3 | 3 (`commands.md:13`, `:33`, `:49`) | 0 | 0 |
| design-system+layout-templates | 1 | 1 (`design-summary.md:50`) | 0 | 0 |
| masters | 2 | 2 (`CLAUDE.md:34`, `:35`) | 0 | 0 |
| obs-plan | 0 | 0 | 0 | 0 |
| security-plan | 4 | 4 (`security-summary.md:27`, `:72`; `security.md:17`, `:25`) | 0 | 0 |
| security-plan+obs-plan | 1 | 1 (`ci.md:12`) | 0 | 0 |
| test-plan | 2 | 2 (`tests-summary.md:27`, `:28`) | 0 | 0 |
| test-plan+obs-plan | 1 | 1 (`observability.md:27`) | 0 | 0 |

Every applied finding's source side is the comparer's cited master line as this pass left it. Each finding lies in a
generated part, never a `USER:` block or `## Session Additions`.
