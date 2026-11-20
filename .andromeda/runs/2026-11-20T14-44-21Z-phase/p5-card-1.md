# P5 card 1 — 2026-11-20-a-new-cli-version-stamped-automatically · «A new CLI version stamped automatically»

viola-0.2.0 · Epoch 4 · run dir `.andromeda/runs/2026-11-20T14-44-21Z-phase/` · take-up resumed at P4 on the
founder's answers (`inputs#I2`).

**How to read the plan:**
- The plan is rev 4. Open its render at `.andromeda/cache/docs/2026-11-20-a-new-cli-version-stamped-automatically/plan.md`.
- No plan stands in the chunk's folder. Writing in that file changes no record until a `put`.
- `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-a-new-cli-version-stamped-automatically`
  is a diff of the chunk's records.

## The founder's word owed first: three P4 forks
P4 found three reached row families outside the P3 block's §5 list (relay item 11). Each is written whole in
`founder-forks.md` § «P4 forks». The plan is written on each one's recommended option.
- **P4-F1 — the start's own lines before the child.**
  - Rows: layout-templates `contract/viola-run/start-output` («prints nothing of its own») and a11y-plan
    `contract/tui-passthrough/viola-terminal-bytes`.
  - Options:
    1. (Recommended) Amend both at the wrap: nothing while the child runs; before it, only the closed auto-verify
       lines and verify's own output.
    2. The start prints nothing, and verify's output goes to null. Cost: a red's failed rows reach no human.
- **P4-F2 — the attempt lock `ledger/verify.lock` guards an action, not a file.**
  - Row: architecture `decision/on-disk-file-naming` («one lock file per guarded file named `<name>.lock`»).
  - Options:
    1. (Recommended) One named exception.
    2. No lock. Cost: two starts at once both verify, the spend doubles, and relay item 5's loser line loses its basis.
- **P4-F3 — keys typed while the start's verify runs** wait in the terminal until the child starts (about 40 s, or
  minutes at a deadline).
  - Rows: a11y-plan `decision/human-keystrokes-never-blocked` and `contract/tui-passthrough/human-keystrokes`.
  - Options:
    1. (Recommended) One except naming the start's verify window.
    2. Read both as not reached: they speak of the passthrough.

## Leans for the reviewer to strike (each with its alternative)
- **The place is after the endpoint bind**, not «between the version probe and the gate's verdict» (§2(A) as shown).
  - Why: a refused socket directory still refuses «before any snapshot, child or bind», with no verify child. This
    keeps three socket-dir rows (security, layouts, a11y) unreached.
  - During the verify the endpoint is bound, not served, and no snapshot names it.
  - Alternative: before the bind. Cost: those three rows are reached, and refusing a socket dir after nine `claude`
    children.
- **A stop ends the start by its own signal** (relay item 6): verify gets TERM, the pair closes, `SIG_DFL` is restored
  and the signal re-raised.
  - This is how a start interrupted today ends: no snapshot, child, document or new exit code.
  - Alternative: exit 1 with `process-exit` `interrupted`. Cost: that needs a revive `--json` code, which is a reached
    row outside §5.
- **Relay item 7, «verify's own refusal word», is met in part.**
  - The start's line names ITS closed word (`spawn-failed` · `unstamped` · `lock-refused`). Verify's own `unable:` /
    `hint:` pair, which states its refusal, stands directly above it on the inherited stderr.
  - Naming verify's word itself needs verify to log under the instance, and the start to read `cli-<name>.ndjson` from
    an offset: one more bounded reader.
  - Most verify refusals are pre-empted anyway. The start has already resolved the program, pinned the copy and read
    the version, and a refused or malformed ledger triggers no verify.
- **The green fake-agent proof needs the fake's stamping flags.** The start's verify passes no user argument, as the
  gate's `--version` does not.
  - The fake therefore reads a `fake-args` file beside its executable, prepended to argv. This is the
    beside-the-executable precedent of `agents-mode` (`src/bin/viola-fake-agent.rs:63-72`).
  - It is test-only code: no viola read, no env seam.
  - Alternative: pass the user's arguments to verify. Cost: they would reach every verify spawn, the Haiku probe
    included, and change what a stamp measures.

## The plan in brief
**Goal:** form (A) (`inputs#I2` item 1).
- With `config.json`'s `verify_new_cli: true` (default `false`, item 2), a `viola run` or a revive start whose
  version's stamp entry lacks a relied id runs `viola verify` once, as a child, before the session. It then re-reads
  the stamps and goes on verified on a green stamp.
- **Writers:** verify stays the stamps' only writer.
- **Red:** a red entry is an attempt. Later starts run no verify and print the earlier-failure line (item 3). It is
  cleared only by a human's `viola verify`, a newer relied set, or a newer CLI.
- **Two starts:** the loser of `ledger/verify.lock` starts unverified at once (item 5).
- **Stop:** a stop ends the start with no stamp and no session (item 6).
- **Unstamped:** a verify that ends without a stamp leaves the start unverified with its line (item 7).

**Steps** (30, in five phases):
- **A**, the product:
  - `attempted` in `viola-agent-claude`;
  - `try_verify_lock` in `viola-state`;
  - the key in `CONFIG_KEYS` and the schema;
  - the `auto-verify` subject;
  - `Gate.entry`;
  - the new `src/run/auto_verify.rs` and `src/human/auto_verify.rs`;
  - the step placed in `start`;
  - the fake's `fake-args`.
- **B**, six fake-agent cases in `tests/cli_version_gate.rs`, the controls-table row, and two remove-the-guard
  controls.
- **C**, the rig arm `AV` and its rehearsal pair.
- **D**, the layers, the boot smoke, the host read, then the round with ONE live start.
- **E**, the operator pass.

**Top acceptance criteria:**
- green once (one `auto-verify` pair, `cli_verified: true`);
- off by default;
- no loop (case 13, the `attempted-arm` control «holds»);
- two starts run one verify (case 14, the `verify-lock` control «holds»);
- a stop leaves no stamp and no session (case 15);
- unstamped → L5 (case 16);
- the key disables no control;
- L1–L5 pinned by literal and silent under `revive --json`;
- `AV` green on the founder's predicate within his cap;
- the layers, the pre-push gate and CI green.

**Touchpoints:** 2 new files and 20 modified ones (research's lists, synced at P4, parse 22 items).

**Test Commands:** 27 entries.
- Phase A: entries 1–5. Phase B controls: 6–7. Phase C: 8–9. Layers: 10–12. Smoke: 13–15.
- The round: 16 `path-cli:`, 17 census, 18 build and hash, 19 live sha, 20 runtime dir, 21 `AV`.
- Then 22 census (operator) and 23 `live-stamp: unmoved`.
- The operator pass: 24–27.

**The live start `AV`** (the founder's cap, `inputs#I2` item 4):
- **Where:** one `viola run` in the rig's SCRATCH viola home, keyed on with no stamp, against PATH's `claude` version
  read in the same entry. Today that is 2.1.329; if mise has moved, it is whatever unstamped version stands there.
- **Processes:** verify inside the start, then the session on `--model haiku`.
- **Typed bytes and dialogs:** the rig types no key and sends nothing. Verify types only its compiled pastes, and its
  probe dialogs are answered by the capture hook's closed ids. No byte goes into a CLI dialog.
- **The stop:** TERM to `viola run`.
- **The real home:** its credentials file is read once (copy removed). Nothing is written under `~/.claude*` or
  `~/.viola`.
- **GREEN** = `stamp-ids: all` · `verdict-match: yes` · `auto-verify-pairs: 1` + the teardown words.
- **Readings:** `stamp-verdict` and `failing`. A drifting 2.1.329 is the product working.
- **The spare `AV-2`:** only for a CONTENDED host or a rig fault before verify's first spawn.

**Capabilities:** none claimed (item 4). No concretization preview.

**Size — WARN, check 6.** 725 lines of plan prose, the fence aside, against the ~150–400 guide. The bulk is
one-fact-per-line bullets and the Expected amendments. The scope is the one window founder-forks §9 named, not split.

## The tool block (directive item 11), verbatim where a tool printed it
- The union, `rec.py rows-union` (P2, 15:11:50Z):
  - `architecture: rows 968 · packs 10 · picked 313 (191 by every draw) · standing 19 · 85953 B`
  - `security-plan: rows 623 · packs 7 · picked 184 (143 by every draw) · standing 16 · 53893 B`
  - `design-system: rows 318 · packs 4 · picked 120 (110 by every draw) · standing 34 · 28690 B`
  - `layout-templates: rows 223 · packs 3 · picked 83 (52 by every draw) · standing 2 · 18213 B`
  - `test-plan: rows 1355 · packs 14 · picked 397 (242 by every draw) · standing 100 · 98468 B`
  - `obs-plan: rows 1030 · packs 11 · picked 305 (190 by every draw) · standing 46 · 74220 B`
  - `a11y-plan: rows 235 · packs 3 · picked 128 (110 by every draw) · standing 17 · 32147 B`
  - `rows-union: 7 master(s) · picked 1530 of 4752 · rule any`
- **Rows read:** the files sum to 391 584 B, past the 250 000 B bound, so they were read by search, not whole. The
  standing lists and history files were read whole.
- **Every cited row in the union or its standing list?** NOT every one. Of 48 rows cited with their master, 46 stand
  there. The exceptions:
  - architecture `decision/claude-shapes-in-one-crate` and `decision/start-pins-the-bytes-it-checked` were looked up by
    `rec.py find`, outside the union. The second was first miscited as security-plan's and corrected in rev 4.
  - Of 43 bare handles, 1 stands in no file: `contract/diag-subject/auto-verify`, the new row the plan proposes.
- **`[RETYPED]` lines** (`fanout.py check`, P2):
  - `arch-pick-p4 · [RETYPED] line 94: contract/mcp-list-results/cacheScope is no row of the pack — read as contract/mcp-list-results/cachescope, the pack's row 94`
  - `arch-pick-p4 · [RETYPED] line 95: contract/mcp-list-results/ttlMs is no row of the pack — read as contract/mcp-list-results/ttlms, the pack's row 95`
  - `arch-pick-p8 · [RETYPED] line 20: decision/recorded-fixtures-scrubbed-and-hygiene-walked is no row of the pack — read as decision/recorded-fixtures-scrubbed, the pack's row 20`
  - `tests-pick-p9 · [RETYPED] line 30: decision/synthetic-fixture-text-only is no row of the pack — read as decision/synthetic-fixture-text, the pack's row 30`
  - `obs-pick-p8 · [RETYPED] line 50: decision/run-worker-panic-ends-session is no row of the pack — read as decision/run-worker-panic-ends-run, the pack's row 50`
  - `obs-pick-p11.d2 · [RETYPED] line 57: contract/diagnostics-line/detail-file-line-size is no row of the pack — read as contract/diagnostics-line/detail-line-size, the pack's row 57`
- **Re-spawns:** `re-spawned 1 agent(s) · STANDING_FORM 1 · wall 48 s` (the obs task; its raw twin is `.raw-obs-task.md`)
  · `respawn-kept 99 % · 73 of 74 predecessor lines, 1 strong re-spawn file(s)` · `draws-dead 0`.
- **Non-zero exits and refusals, this window:**
  - `inputs.py snap` of `founder-forks.md`, exit 3: `--source lies in this project's own repository, under
    `.andromeda/` — the pipeline's own tree; phase reads it directly, it is not an external input`. It is cited by
    path instead.
  - The Bash guard refused twice a `cd` that would move the session's working directory, and once a pipeline tool's
    output piped into a filter. Each was re-issued once in the guard's form.
  - planlint's first run: `check 4 (5) · leg = 'live' entries: 1 · Harness rules consulted: none → HIT`. Resolved by
    reading `.claude/rules/verification-harness.md` into research (rev 3). planlint now reads `hits: 0`.
  - Baselines:
    - `6 integration red · exit 0 ✗ (exit 4)`, `7 … red · exit 4`, `9 … red · exit 4`: nextest «no tests to run»,
      each binary term selecting its binary (`cli_version_gate` 9 tests, `rig_rehearsal` 38) and the new case names
      nothing yet;
    - `23 probe red · exit 0 ✗ (exit 1)`: no evidence sha file. Its must-pass in a clone at `816f9a77` read
      `23 probe green` (`p5-controls.md`);
    - entries 4 and 5 green (275 and 1141 tests).
- **`UNPARSED` / `INDETERMINATE` / `NOT DERIVED`:** none printed in this window.
- **Where a letter was wrong or silent:**
  1. The away form (`inputs-contract.md`) stops the step at a fork. The invocation said to go on to this card, and
     relay item 11 says a fork outside §5 is «written to founder-forks.md, not a fold». P4-F1…F3 were written there
     and carried here for the word, rather than stopping at P4.
  2. The plan-template's ~150–400 line guide reads WARN on a chunk whose acceptance carries six fake-agent cases, two
     controls, a rig arm and a live start.
  3. P3's block §5 called layout-templates and a11y-plan row-free and named no on-disk-naming row. P4's search of the
     picked rows found the three forks (filed as a narrow-basis friction).
  4. The relay asked to «snap this relay» and the run-dir block; the tool refuses the run-dir file as the pipeline's own
     tree. Only the relay is snapped, as `inputs#I2`.

## Carries
- **CARRY `cry-otjywzhu`** is consumed by this chunk. `AV` stamps PATH's version in a scratch home only, and the live
  home stays the product re-pin step's (CARRY `cry-fqwjpnqt`).
- **WATCH `cli_wheel path5_…`** is read on every integration run and every CI read. A recurrence is read from the job
  log and events before any mend.
- **CARRY `cry-lgkskubi`** (the rehearsal red) is acted on only at its next red: summary and `after-del` frame first.

## Scratch left for removal by hand
- `$TMPDIR/{scratchpad}/clone23` (the entry-23 control's shared clone) is not removed by a recursive delete here
  (`inputs#I1` item 10).

## Beyond the plan
none. A new capability (founder-forks §8) is not proposed, since item 4 says no capability is claimed; the live home's
2.1.329 stamp stays with the product re-pin step's existing CARRY.

---

Apply? (yes / review / cancel)
