# Resume point — wrap of 2026-10-05-real-cli-verify-probes

**Stopped after P1 on the operator's word** (context 61.5 %, measured). **Next: P2, Reconcile docs** (the fan-out).
On resume, Setup 2a HALTs on the existing `report.md`. Answer **resume** in this run dir
(`.andromeda/runs/2026-10-05T10-37-44-wrap/`): `fanout-results.md` does not exist yet, so P2 fans out fresh.

## State at the stop (2026-10-05T10:40Z)
- **Branch** `build/viola-0.1.0`, 0 ahead of upstream. HEAD is `012fc50`, the operator pre-CI commit; its parent
  `caae9ec` is the base.
- **CI** ci#37296910661 is green, 15/15, wall 422 s, at `012fc5089498`.
- **Master record** `2026-10-05-real-cli-verify-probes · pending` (line 46). No flip yet.
- **Uncommitted:**
  - `evidence/operator-pass.md` (the CI verdict append);
  - `chunks/…/report.md` (P1);
  - `.andromeda/friction-log.ndjson`;
  - this run dir.
- **P1 done:**
  - `report.md` is authored;
  - `gate.py scope`: `clean — changed 36 · listed 31 · recorded 5`;
  - `inputs.py verify`: I1 unchanged, I2 n/a, 0 drifted;
  - the report evolve checkpoint is appended (`2026-10-05T10:40:17Z-a`).
- **Code-graph refresh** done in the background (`.andromeda/cache/.refresh-done`): rust 4027 nodes / 19822 edges, ts
  7 / 1. P4 records it, and P7 stamps `tree.db.commit` after the commit.
- **Matrix:** this chunk claims 0 caps. P7's coverage gate will read `claimed 0 — gate no-op`.

## What P2 onward must carry (from the session; the report holds the detail)
1. **The founder's live rulings** (2026-10-05, relayed by the overseer):
   - **R-S2** (~00:00Z): the PTY-driven typed-input `viola verify` probe against the live `claude`, local to the dev
     host, stamping the version it runs.
   - **The three-way split** (05:58Z):
     - this chunk is W1 + W5;
     - "Dialog rows and re-probe" (W3 + W6) comes first, then "Local-command and paste-framing rows" (W2 + W4), both
       ahead of `:86`;
     - Epoch 3 stays one epoch.
   - **Trust** (~07:00Z, re-affirmed ~08:50Z): **two runs, never accept**. viola never types into a CLI-native dialog
     (trust or external imports), and nothing is written to `~/.claude` by viola or the implementer.
   - **The two dirs** (08:25Z):
     - **Run B** in `<cwd>/.viola-verify-<pid>/`, 0700, removed on every exit path, with a `.gitignore` line;
     - **Run A** in a fresh 0700 dir under the OS temp dir.
   - **The external-imports answer** (~08:50Z, amended ~09:05Z): the overseer set the repo-root `projects` flags on
     the founder's word, as an atomic `~/.claude.json` edit equivalent to "No, disable external imports"
     (`Approved: false`, `WarningShown: true`), with a backup kept. The overseer verified it at 09:10Z and 09:11:52Z.
     The flags were unchanged after every run.
   - **The live cap:** 12 (~08:50Z), then 15 after the first entry-7 STOP, then 18 after the second. 16 sessions were
     used (`evidence/live-sessions.md`).
2. **The plan correction** (the overseer, founder-delegated, 2026-10-05): record entries 7 and 8 were driven by hand
   with the plan's exact `run`, except `--home "$h/home"` → `--home "$h/vhome"`.
   - The record home's own `/home/` component survives the scrub and is refused as an absolute path.
   - The product works as designed.
   - Both entries are green: `stamped 2.1.288  10 pass  0 fail` is the W5 stamp, and `stamped 2.1.287  10 pass  0 fail`
     (`evidence/hand-entries-7-8.md`).
   - The report files this under Spec claims disproved 1.
3. **The named-refusal widening** (the overseer, founder-delegated): `--record`'s refusal names the file and the check
   code (`home-path` · `absolute-path` · `username` · `email`; for a screen, `row <n>` plus `seam`), never the
   content. It is recorded in `scope-record.md` with the word quoted, and proven offline before the re-run.
4. **The Run B residual** (shown at P5, accepted): each trusted run leaves a synthetic-prompt transcript under
   `~/.claude/projects/`; there are five (13 → 18 dirs). The user's global hooks and status line also run in Run B.
   Run A leaves nothing (STOP 2 clear after every run). The security-plan amendment is expected.
5. **`local-live`'s live firing is pinned on `:86`** ("First live test and self-drive"). Its ten literal rows are
   unit-proven here, and the live firing itself was not run (the cap arithmetic). P5 carries it as a CARRY on `:86`.
6. **Two new entries ahead of `:86` (P5, a trajectory edit: HALT for the dialogue):**
   - **first, "Dialog rows and re-probe":** W3 + W6, scope CARRYs 6, 7 and 9, and research M7's crossing (the capture
     arm answers nothing, so "the decision takes effect" needs an answering probe hook or the probe keying the
     rendered dialog, a founder crossing). `v1-15` is claimed there per R2;
   - **then, "Local-command and paste-framing rows":** W2 + W4, CARRYs 2, 3 and 5, and M10 (a cross-session prompt
     needs a second session). `v1-29` becomes claimable there per its notes.
   - CARRY 1 (the split record) stays with this chunk.
7. **The expected amendments** (`plan.md` §Implementation notes): every entry is dispositioned in the report's
   Expected amendments bullet, with site counts per master at `caae9ec`:
   - architecture: [Screen Model], [Delivery Confirmation], [Plugin Scope], §Occupied Resources, [CLI Version
     Compatibility] at 6 → 10;
   - security-plan: §Threat Model Summary, §Data Protection, the residual, the dev-host answer;
   - test-plan: §7 fake options and the screen class, §3 `local-live` ten ids with the firing at `:86`, the stamp walk;
   - obs-plan: §6 `verify-pty-probe`, §4 Edge flows;
   - design-system and layout-templates: §Surface: cli at `/10` plus the help paragraph;
   - a11y-plan: §4 P6.

## Remaining phases (unchanged order)
P2 reconcile (fan-out → validate → apply + cascade) → P3 curation (the session's corrections: the `/home/` component
hazard; describe, never spell, a `…/home/…` path in committed evidence; the `send_window_` no-test-deadline precedent)
→ P4 code-graph (done) → P5 route-resolve (the two entries and the `:86` CARRY, a HALT) → P6 state + handoff → P7
light gate (live and operator legs re-verified from `evidence/`, never re-run) → drift = 0 → coverage (no-op) → hygiene
and scope → flip and compact → commit → push.
