# Cascade dispositions — wrap 2026-10-05T14-27-34 (chunk 2026-10-05-dialog-rows-and-re-probe)

**The search.** `cascade.py sweep` over `cascade-patterns.toml` (12 patterns, written after the last body amendment),
listing in `cascade-sweep.txt`; baseline `75198e53` (the oldest pre-CI commit's parent). Every pattern's known-positive
control fired on the pre-pass masters. The patterns cover each retired claim by its wording and by its mechanism:
the row count (`ten rows` · `ten-row` · `/10` · `10 pass` · `8 pass`), the run count (`two interactive` · `both
interactive` · `neither run` · `Run A, then Run B` · `both dirs`), the spawn count (`four spawns` · `four child spawns`),
the obligation owner (`Dialog rows and re-probe`), the gap (`dated gap|exception` · `ten-row stamp`), the relayed tier
(`relayed dialog` · `RELAYED.md` · `relay script`), the capture arm's output and naming (`empty stdout` · `first free` ·
`unparsed` · `stays silent`), the S7 approve form (`approve only via|through PreToolUse` · `PreToolUse allow`), the
fake agent's option count (`three argv options`), the verify window (`two GATE_MAX_WAIT`s · `verify_window_`) and the
permission owner (`no recorded fixture yet` · `permission` wake/kind). Sections read beside the listing: arch [CLI
Version Compatibility] (:73–:91), [Hook Contract] (:69), [Plugin Scope] (:100), §Occupied Resources (:355–:365,
:399–:412), §Cross-cutting (:459); security-plan :126, :226, :231, :244, :274–:283, :503, :608; test-plan :408, :438,
:558, :653, :663, :757, :779, :781, :796, :1041, :1051, :1076, :1084; obs-plan :735–:737, :866–:867; design-system
:801; layout-templates :467–:484; the five edited key files.

## Master rows (`new` · `standing`)
- arch:91 ten-rows / dated-gap (edited) — the new text says the ten-row gap "is closed": a true historical statement. No change.
- arch:91 permission-e2e (new) — the new owner sentence. No change.
- arch:69 capture-out (new) — "from the first free `k`, the next `k` on AlreadyExists": the current mechanism. No change.
- arch:75 s7-approve (edited) — the new approve form. No change.
- arch:412 relayed (edited ×2) — the relayed set now named superseded and kept. No change.
- arch:47 / :322 / :390 capture-out — DA1 silence, the async-tier hooks' empty stdout, the statusline: other paths, true. No change.
- security-plan:608 ten-rows / slash-ten / dated-gap (edited) — the closed gap and the `10 pass  4 fail` example: true. No change.
- security-plan:126 two-runs (edited ×2) — the quoted ruling name "two runs, never accept": a quotation of the founder's ruling. No change.
- security-plan:126 dialog-owner (new) — names the chunk's plan review as the ruling's venue: true. No change.
- security-plan:206 / :528 dated-gap — the Epoch 6 IPC exceptions: unrelated. No change.
- security-plan:226 capture-out (new) — the current mechanism. No change.
- security-plan:235 capture-out — the event-path fail-open: true. No change.
- test-plan:438 / :1041 / :1051 / :1084 relayed (edited) — superseded, kept. No change.
- test-plan:1060 relayed — "the relayed dialog fixtures carry `tool_name`": still true of the kept relayed set. No change.
- test-plan:252 s7-approve — the S3 question body (`updatedInput.answers`): true. No change.
- test-plan:558 / :796 s7-approve (edited) — the new approve form. No change.
- test-plan:647 / :849 / :885, obs-plan:678 / :723 / :904 / :1019 capture-out — other hook and fail-open paths: true. No change.
- test-plan:757 / :779 permission-e2e (edited) — re-owed to "Permission end to end". No change.
- obs-plan:660 dated-gap — the Epoch 6 server-verification gap: unrelated. No change.
- a11y keyboard-test-harness.md:8 capture-out — "stays silent" about a screen reader: unrelated. No change.
- bootstrap-phases key file :10 / :11 verify-window (edited) — the new overrides. No change.
- Check 4 (obs O5's absence claim): obs-plan §4 table :583, §2 :512 and §10 exemption 5 :1066 drew no `capture-out`
  hit; they state no obs init / no role file / exit 0, which still hold. Disposed: no change.
- `fake-argv` drew 0 rows after the pass (its control fired at arch:355): the retired count stands nowhere.

## Leaf rows → step 3 (re-derived)
- CLAUDE.md:15 (GENERATED:setup:overview, relayed) · CLAUDE.md:43 (GENERATED:setup:warnings, ten-row / dated / owner).
- `.claude/rules/verification-harness.md`:28 (ten rows, both runs) · :46 (relayed).
- `.claude/rules/security.md`:19 (two interactive children) — body text, not Session Additions; :14 is the Epoch 6
  exception, unrelated, no change.
- `.claude/rules/observability.md`:37 — "`hook` → exit 0, empty stdout" is the panic outcome of the event path: true, no change.
- `.claude/docs/commands.md`:25 (ten rows, `[NN/10]`, two runs).
- `.claude/docs/security-summary.md`:38 (the dated gap) · :71 (two children).
- `.claude/docs/services/viola.md`:6 (ten rows, two runs) · :31 (four spawns).
- `.claude/docs/services/viola-agent-claude.md`:11 (two runs) · :20 (no decision = empty stdout: true, no change) ·
  :23 (owner) · :31 (S7 approve form).
- `.claude/docs/stack.md`:14 (two runs).
- `.claude/docs/tests-summary.md`:21 (ten rows, relayed, owner) · :27 / :28 (permission owner).
- `.claude/docs/gotchas.md`:75 (the dated gap) · :117 (DA1 silence: unrelated, no change).
- Floor beyond the listing (the DAG table, by provenance): obs-summary, design-summary and layout leaves drew no
  hit; each is re-read for the amended sections in step 3.
- Curation homes: 0 rows. Judgment bases: 0 rows.
