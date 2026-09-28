# Cascade dispositions — 2026-09-27-hooks-to-normalised-events (re-fan pass, 2026-09-28)

**The search:** `cascade.py sweep` over `cascade-patterns.toml` (18 patterns, every control fired on the pre-pass masters at
the pre-CI parent `273e1ab`), run after the last body amendment. The patterns key on each retired claim's wording AND its
mechanism: the `{"hooks": {}}` placeholder; "Only `run` has a producer"; the two-target fuzz list; "thiserror only"; the spine
deadline as an open item; SessionEnd's `~1 s deadline`; `viola-fuzz` depending on `viola-core` only; the `<pasted_content
id=…>` form; "once/joins with Hooks to normalised events"; forced panic / panic trigger / `HOOK_PANIC`; `4 KiB`; the
hook-stdin-parser property; `copy-target` / `CARGO_TARGET_DIR`; verification "including `hook.event`" / "every `hook`
event"; "provisionally 1.0 s"; "Registered hook events"; `--perf`; `detail-hook`. A zero-row pattern is a statement about
that pattern (its control fired), never an absence proof.

**Sections read beyond the rows:** architecture [Hook Transport], [Hook Contract], [Snapshot writer], [Deployment /
Distribution], [CLI Version Compatibility], §Standard Contracts channel frames + methods + Event `data`, §Occupied Resources
(Claude Code integration names, Workspace crates, Environment variables, Filesystem, Repository), Crate dependency direction,
the directory tree; security-plan §Authentication rows :205/:207 (by offset), §Input Validation :221–:234, §Error Handling,
§Anti-Patterns → Authentication, §Decisions Log; test-plan §1 :174/:228/:331/:385, §2 :434–:435, §3 :526/:554/:560, §5 :924,
§6 :1028/:1138/:1254/:1266/:1294/:1326–:1331, §7 :1362, §9 :1447, §10 :1512–:1527; obs-plan :419, :616–:625, :870, :1004,
:1316–:1324.

## Rows (every row the listing printed)
- `hooks-empty`, `only-run-prod`, `fuzz-two`, `fuzz-core-only` — 0 rows after the pass (controls fired at arch :99, :380,
  :395, :358): the amended sites were the only ones.
- `ac-thiserror` — `.claude/docs/services/viola-agent-claude.md:11` leaf → RE-DERIVED (as-landed dependencies).
- `spine-open` — architecture.md:69 → NO CHANGE: the [Hook Contract] DIALOG deadline, a true open item sharing the phrase.
- `sessionend-1s` — architecture.md:65 → AMENDED this pass (A12; the ~1 s budget kept on purpose).
- `paste-wrapper` — architecture.md:49 @c898 (window read: [Delivery Confirmation] says the wrapper is removed, then the
  escaping reversed) and :283 → NO CHANGE (true, abbreviated form; the full pair lives at :81); test-plan :871, :1041 → NO
  CHANGE (M3/M4 name the open form, true); `.claude/docs/gotchas.md:88` leaf → NO CHANGE (true).
- `hooks-lands` — test-plan :1778 → NO CHANGE: §12 Test Decisions Log, history.
- `forced-panic` — test-plan :331, :1254, :1266 → AMENDED (T16, T7, T8); :873 → NO CHANGE (the vt100 feed's own forced
  panic, a different subject).
- `4kib` — test-plan :924, obs-plan :621 → AMENDED (T9, O2); architecture :23 (64 KiB chunks), obs :616 (role lines under
  4 KiB), :623 (`PIPE_BUF`) → NO CHANGE (true claims sharing the token); test-plan :1734, obs :1664, :1697 → NO CHANGE (§12
  logs, history); `.claude/docs/services/viola-state.md:39` leaf → RE-DERIVED (the hook half's timing).
- `resets-prop` — test-plan :434, :1326, :1331 → AMENDED (T11, T10); security :242 and test-plan :371 → NO CHANGE (the
  parser-surface lists, still true).
- `copy-target` — test-plan :554 → AMENDED (T1); architecture :400/:401/:405, test-plan :515, :534, :558, :1773 → NO CHANGE
  (other target dirs, true; :401 is this pass's own A8); `.claude/rules/host-win32.md:90` curation home → NO CHANGE (true;
  never edited by the cascade); leaves `verification-harness.md:28` → NO CHANGE (the harness target dir), `commands.md:46`
  → RE-DERIVED (the mutation command's target dir), `commands.md:41`, `:77`, `:78` → NO CHANGE (true); the rule's
  `run --mutants` bullet (`verification-harness.md:44`, the site claim (c) named) → RE-DERIVED.
- `verify-hookevent` — security :205, :207 → AMENDED (S2, S3); :596 → NO CHANGE (the initial Decisions Log entry,
  history); :722 → this pass's own entry; test-plan :1294 → NO CHANGE (the single-writers sweep: `hook <event>` writes no
  snapshot, true); `.claude/rules/security.md:14` leaf → RE-DERIVED (the dated exception).
- `prov-1s` — test-plan :1520, obs :1321 → AMENDED (T4, O5); obs :419 → NO CHANGE: obs §1, the verbatim scope copy
  (playbook "Verbatim scope copy"; an edit applied there in this pass was reverted); test-plan :1526, :1527 → NO CHANGE (the
  gate stays provisionally 1.0 s); :1699 → NO CHANGE (§12, history).
- `registered-hooks` — architecture :350 → AMENDED (A16).
- `perf-arm` — test-plan :435, :1448, :1521 and obs :1316 → AMENDED (T5, T6, T4, O3); architecture :407 (`target/perf/`),
  test-plan :514, :519, :541, :559, :798, :1515–:1517 → NO CHANGE (the planned arm's contract; §10's status note carries the
  timing); `.claude/docs/services/viola.md:41` leaf → RE-DERIVED.
- `diag-hook-prod` — architecture :69, :381, test-plan :924, obs :621, :1690 → NO CHANGE beyond this pass's own edits
  (true: the file and its producers).

## Leaves re-derived (step 3)
- architecture → `.claude/docs/services/viola-agent-claude.md` (dependencies; the `hook` module), `viola-state.md` (the
  replace retry, `try_append_event`, the >4 KiB timing), `viola-channel.md` (`notify`, `connect_by`, the served method, the
  interim exception), `viola.md` (the `--perf` timing), `commands.md` (the mutation target dir). CLAUDE.md `GENERATED:setup:*`
  blocks recomputed against the amended sections: no line states an amended claim (overview, modules, warnings, pointer
  table, architecture unchanged). `stack.md` (serde_path_to_error already listed), `conventions.md` (`hook.event` already a
  notification), `gotchas.md`, `workflow.md` → no change.
- security-plan → `.claude/rules/security.md` (the dated exception), `security-summary.md` (the interim gap), `api.md`
  (`VIOLA_DIR` interim), CLAUDE.md warnings (no server-verification line: no change).
- test-plan → `.claude/rules/verification-harness.md` (the `run --mutants` target dir), `tests-summary.md` (the perf row),
  `testing.md` (no amended claim: no change).
- obs-plan → `obs-summary.md` (the perf row); `observability.md` (no amended claim: no change).
- Binds: test-plan §3 ↔ obs-plan §3 — the §3 harness change (T1, `target/mutants`) has no obs-plan counterpart (obs §3 has
  no mutation-target text); the `boot` readiness move (T2) is mirrored in obs :870 (O1). a11y ↔ obs schema: untouched.
- Judgment bases (`playbook.md`, `drift-base.md`): no row.
