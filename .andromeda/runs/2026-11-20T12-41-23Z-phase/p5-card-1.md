# P5 review — revision 1 of 2026-11-20-permissionrequest-question-answered: «PermissionRequest question answered»

**The plan to read:** `.andromeda/cache/docs/2026-11-20-permissionrequest-question-answered/plan.md` (rev 5).
- It is the render of `doc:2026-11-20-permissionrequest-question-answered/plan`. No plan stands in the chunk's folder.
- Writing in that file changes no record until a `rec.py put`.
- The chunk's records as a diff:
  `python -X utf8 ~/.claude/skills/andromeda-tools/scripts/rec.py diff --root . 2026-11-20-permissionrequest-question-answered`.

**Why a revision:** implement run `2026-11-20T12-23-17Z-implement` stopped before `S1` at step 18, as written: the rig's
`VR` arm refuses none of the three pre-round conditions itself. No live start, nothing committed, `PQ` and `PQ-2`
unspent. Word: the operator (`inputs#I6`); what changes and what must not: the overseer's relay (`inputs#I7`).

## What revision 1 changes
- **Step 18:** the three conditions are closed against the code. Two hold by construction and take no entry; one
  becomes **entry 17**, a no-start `round` entry right before `S1` (`S1` is now entry 18; the block is 35 entries,
  every later number +1).
- **The stopped run's deviations enter as DONE steps**, so the scope record and the report agree:
  - 4a — `src/cmd/hook.rs` test module (the borrowed unmappable pair re-pointed to `plan-approve`);
  - 9a — the settle after the fourth prompt's Stop in `src/cmd/verify/typed/runs.rs`;
  - 13a — `src/run/version_gate.rs` test module (`stamps_verdict_needs_the_dialog_rows` at the 27th row);
  - 13b — `tests/support/verify.rs`'s three synthetic `question-permission-1` shapes, confirmed outside the founder's
    decline (a test-built set written at run time, committed nowhere);
  - 14a — the `cli_verify` summary counts moved by hand: UNRUN until the landing, first run by entry 27
    (the after-landing `run --integration`);
  - 15a — `FLOOR_ROWS` takes the new row, the «every relied row but four» rule kept.
- **Implement re-enters at step 16a:** name each synthetic shape as synthetic where it is built (comments only), then
  `--only 7,8` (fmt, clippy), then step 17 (the host read) and the round.
- **Nothing else moves** (`inputs#I7` item 3): the cap `--cap 1 --spare 1`, `S1`'s `run` and `expect`, the
  after-reads, the landing, the nine rows and what «verified» rests on are byte-equal. No fork was needed, so no
  `founder-forks.md` was written.
- Touchpoints: +3 files to modify (`src/cmd/hook.rs`, `src/run/version_gate.rs`, `tests/support/verify.rs`), 0
  dropped.

## The block the relay asks for (`inputs#I7` item 4)

**Entry 17 — the new round entry**
```
d="${XDG_RUNTIME_DIR:-}"; test -n "$d" && test -d "$d" && test ! -L "$d" && test "$(stat -c %u "$d")" = "$(id -u)" && test "$(stat -c %a "$d")" = 700 && echo runtime-dir: owner-only
role probe · leg round · env [] · expect ['exit 0', 'last line runtime-dir: owner-only']
```
- **Baseline:** none written. It is a `leg` entry, so `gate.py baseline` reads `leg round — exempt`, and the round
  fires it, not `--entry`.
- **Controls** (`p5-controls.md` in this run dir, read 2026-11-20T12:51:44Z). Each typed the entry's own `run` text
  unchanged as the tool types it (`bash -o pipefail -c`); only the `XDG_RUNTIME_DIR` value differed:
  - **must-pass green ×2:** the host value as inherited (`/run/user/1000`, uid 1000, 0700), and a fresh 0700 dir
    of the user. Both read exit 0 with `runtime-dir: owner-only`.
  - **planted red ×8:** unset, empty, an absent path, a regular file at 0700, a symlink to a 0700 dir, 0755, 0770,
    and another owner's 0700 dir (`/var/lib/private`, root). Each read exit 1 with no output.
- **What a red does:** the round stops at 17. `S1` is not fired, no ledger line is written, and `PQ` stays unspent.

**By construction — no entry, with their lines**
- **No standing `.viola-verify-*` in the probe cwd.**
  - `PQ`'s verify runs in the scratch root's own `cwd`: `crates/viola-e2e/src/rig/mod.rs:1182`, made by
    `Scratch::create` at `crates/viola-e2e/src/rig/isolate.rs:409-421`.
  - Only `VH` takes a given cwd: `mod.rs:1053-1056`.
  - `TrustedDir::create` removes a standing one before it makes its own: `src/cmd/verify/typed/runs.rs:301-304`.
- **No standing scratch home is reused.**
  - The root is `<base>/<pid>-<start time>` per start: `crates/viola-e2e/src/rig/procs.rs:64-68`, `mod.rs:552-555`.
  - The base `/tmp/viola-rig-<uid>` is on tmpfs (`findmnt` read `tmpfs`).
  - Named limit: `create_private_dir` admits a standing dir (`crates/viola-state/src/fs.rs:21-41`), so this rests on
    the run id, not on a refusal.
  - The paths the start writes outside the root, each with its disposition:
    - the kept record and the kept stamp: entry 15, unchanged;
    - the frames `target/live-rig/<run>/`: same run id, and no `S1` atom reads them;
    - the cap ledger: appended by design;
    - verify's Run A/F `/tmp/viola-verify-<random>/`: `tempfile`'s exclusive create (`runs.rs:41`, `:238`), so
      nothing standing is reused;
    - the CLI children's own files outside `CLAUDE_CONFIG_DIR`: unmeasured, as at the take-up.
- **Not by construction: `XDG_RUNTIME_DIR`.** That is why it is entry 17.
  - `viola verify` reads nothing there. Its only product reader is `crates/viola-channel/src/endpoint.rs:63`, which
    verify never calls.
  - But the `VR` arm keeps the variable in its child's environment (`isolate.rs:24-32`, `:188-192`), and verify passes
    it on to its `claude` children, whose reads there are unmeasured.

**Every non-zero exit and refusal in this run**
- Bash guard: 1 refusal, a `cd` into the skill's references dir. Re-issued once with absolute paths.
- Permission classifier: 1 refusal with the verdict «Undermining Oversight». The refused call was a P5 control
  script in this run dir (a `mktemp -d` with an `rm -rf` trap); it never ran. I removed the file and fired the same
  cases one call each, visibly, with the inputs in the session scratchpad and no removal.
- Edit tool: 2 «string not found» on one paragraph (its line wrapped differently). Fixed by reading the line and
  editing again.
- `gate.py scope --sync`: `FAIL 3` at its first firing (exit 0; the three added files were not in research's lists).
  Research was edited, and the second firing read `scope sync: PASS · delta +3 −0`.
- An ad-hoc python read of the stopped run's gate trail exited 1 (`AttributeError`: I assumed the wrong JSON shape).
  Read again correctly.
- The controls' 8 planted reds exit 1, by design.
- No `UNPARSED`, `INDETERMINATE` or `NOT DERIVED` line. Planlint: 0 hits. `inputs verify`: 7 entries, 0 unparsed,
  0 uncited. Hygiene: clean.

## Leans to confirm
- **The runtime dir is an entry.** The relay's exception reads «the VR arm and viola verify under it». Read
  narrowly (those two alone, neither of which reads the dir), it would be by construction and take no entry. I kept
  the entry because the arm hands the variable to the CLI children. Strike it if the narrow reading is meant; the
  card would then hold three by-construction conditions and no new entry.
- **No P2 re-run.** The corrected scope names no file or surface the take-up's tasks missed: `src/cmd/hook` is in
  them, and the rig's scratch home was already in scope.
- **The `continuation` sweep re-run reads 35 hits, not the take-up's 36.** HEAD reads 35 by the same per-file count.
  The gap is noted in research, not explained.

## Validation
- Mechanical checks 1–13 pass. On size: the prose is past ~400 lines, but the revision's done steps are not
  counted, and the §V list is the take-up's.
- Validation-1 is aligned: the scope now carries the conditions bullet and the corrected `src/cmd/hook` bullet.
- `matrix#v2-20` stays claimed, with its acceptance unchanged. The revised plan still proves it, so there is no
  concretization change and nothing to un-claim.
- `Research: lists synced to the revision's delta (+3 −0), rev 2; not re-derived since the take-up's rev 1.`

## Beyond the plan
1. (standing — `.andromeda/runs/2026-11-20T11-10-34Z-phase/p5-card-1.md`) **A new route entry right after this
   chunk:** a new `claude` version on the host is stamped automatically in the product, as its own entry (the
   founder's direction, `inputs#I3` item 6). Proposed title «A new CLI version stamped automatically». This chunk's
   wrap takes it at route-resolve.

Revision 1 adds no row of its own.

Apply? (yes / review / cancel)
