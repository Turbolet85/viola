# Session Handoff

**Last Updated:** 2026-10-10T09:17Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-10-windows-mutation-grade — the sixteen Windows-side survivors closed, host-excluded mutants left out by the harness, nine Windows mutation jobs green inside their ceiling

## Position
- Done: **2026-10-10-windows-mutation-grade** (49 complete, 0 pending, 0 gated; 16/53 verified; the chunk claimed no
  capability). CI green on its pre-CI commit `dd5161d`, `ci#38021000200`, 15/15, first attempt. The Windows
  mutation workflow green on the same sha, `windows-mutants#38036448183`, nine jobs, first attempt.
- Next entry: **Self-healing state** (the head of the markerless tail, the first feature entry) → `/andromeda-phase`.

## Work done
- The sixteen Windows-side survivors are closed: twelve read caught on `windows-2025`, four are no longer generated.
  The strict-modes and DACL checks have Windows-side refusal tests; three decisions moved into plain functions tested
  on every OS.
- `run --mutants` leaves a missed mutant the host never compiled out of the count and names it in the document's
  `mutants.host_excluded`. The harness document judges a run; cargo-mutants' own lines still say MISSED.
- `windows-mutants.yml` is nine `mutants (<label>)` jobs (the root package split by file). One dispatch read all nine
  green over 642 mutants (537 caught, 75 unviable, 30 left out, 0 missed, 0 timeout), the longest job 49 min.
- The session ran in three parts after a clear: orientation, implement from the plan's stop rule (step 11), this wrap.

## Drift resolved
- 26 amendments over four masters, one of them (the exception's sentence) landing in three: architecture 6 (the
  nine-item job shape, the retired "jobs read red" sentence, the tree comments, syn and proc-macro2 in §Stack, the
  exception named), security-plan 3 (the nine-item matrix, the jobs sentence without a mutation job and with
  `perf`, the exception named), test-plan 16 (the exception named, the host exclusion in
  §3 `run` step 4 and everywhere "missed" is a condition, the §9 row, the §10 Mutation gate, the `prepare` mutants
  measured, the harness's dependency line), obs-plan 3 (the Mutation row's reading and its condition). Five sidecar
  entries. Leaves re-derived: `stack.md`, `commands.md`, `tests-summary.md`, `rules/testing.md`,
  `rules/verification-harness.md`.
- 25 detector proposals from seven docs: 23 applied, 2 rejected for resting on the evidence folder and raised again
  by the orchestrator, 3 more raised by the orchestrator. No escalation. Design-system, layout-templates and
  a11y-plan returned nothing; the plan's a11y §10 entry was not applied (the sentence still holds).
- The citation sweep re-pointed nothing and printed no row.
- One playbook rule appended on the operator's word: a count in a master is amended with its rule named.
- Record: `.andromeda/runs/2026-10-10T08-56-51-wrap/` (`fanout-results.md`, `cascade-dispositions.md`, `curation.md`).

## Notes
- **The founder's dispatch word (2026-10-10T07:58:27Z, `inputs#I5` of the chunk):** one or two dispatches for that
  chunk only. One was used. The allowance ends with the chunk; founder ruling C2 stands and is reworded nowhere.
  The exception is named with its date in architecture's CI/CD approach, security-plan's CI integration and
  test-plan §10.
- **Route, this wrap (two CARRYs, no new entry, no reorder):**
  - "Windows-only live measurements": the four `src/cmd/run.rs:385:5` mutants that no host of this project
    compiles. The plan left them owed to no entry; the wrap pinned them to the nearest owner by its own call, not
    on a direction. One line from the operator moves the pin.
  - "Paste newline ledger row": three `viola-state` retry-loop mutants that read caught after 10 s of test time on
    Windows, written as a hypothesis (ended by the kill line, no failing assertion).
  - The stale security-plan sentence that the last wrap carried to this chunk is amended. The stale obs-plan
    sentence on `human::refuse` still rides "CLI output discipline".
- **For whoever dispatches `windows-mutants.yml` next:** a job's wall follows its count of viable mutants (about
  100 s each in the root package). This run: `viola-run-env` 49 min, `viola-panic-frames` 45, `viola-main` 42,
  `viola-cmd-run` 26, the rest 18 or less.
- **For the founder (carried, none resolved here):**
  - the entry "Paste newline ledger row" needs his ruling on the `viola verify` child set (a fifth Run B paste)
    at its take-up, and a cap of live starts of its own;
  - one sentence stands in architecture's capability-ledger pattern without a halt (one relied-on shape has no
    row yet); one line if he wants it out;
  - a listed local command followed by CR or CRLF classifies as that command; not shown to him yet;
  - the switch from the prototype is not made; `v1-33` is unclaimed, pinned on "The board: viola list";
  - not measured: any CLI version but 2.1.287, a long or wrapped multi-line text, an LF inside a paste on
    Windows, the wheel's return by a human `release` on a live session;
  - one CLI plan file is left in his user directory (2026-10-08). viola wrote nothing there.
- **Epoch growth:** Epoch 4 stands at 10 entries (4 complete, 6 markerless). No boundary is minted inside it: the
  founder's word, 2026-10-09 (carried).
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s harness-prefix row and its R8
  identity-floor row. A probed row for either needs its own founder ruling.
- **Setup:** `upgrade.py detect` read 0 entries for setup at this session's start (one noted, U36). A setup re-run
  must keep `.claude/rules/ci.md` and `.claude/rules/testing-src.md`. The host leaf is `host-linux.md`.
- **Standing rules (carried):**
  - a removal the permission layer denies is not done through another tool; move the dir into
    `target/e2e-home.disk/` and record it;
  - a stalled-start red is a finding about the backing: read
    `test -L target/e2e-home && findmnt -n -o FSTYPE -T target/e2e-home/`, stop and report; never re-run for green;
  - before any step that focuses a window or types a key on the dev host, read `hyprctl locked`; unlocking is the
    founder's. A headless pty rig needs neither a window nor a key.
- **Before a census run:** `bash scripts/profraw-census.sh 4800 48` loads 48 workers for about 2.5 minutes on a
  host other builders share. Write the operator one line first; a notice, not a question.
- **After a `cargo clean`:** re-make the link, `ln -s /tmp/viola-e2e-home-<uid> target/e2e-home`.
- **Route:** `BLOCKED-ON` on "Windows-only live measurements" stands: `uname -s` read Linux at this wrap.
- **For the operator's word** (carried, with this wrap's additions):
  - `v1-33`'s title and `requirements.md:48` still say "on Windows"; no wrap step may edit either;
  - the ledger's dated notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90` for the entry "Windows-only live
    measurements"; none was rewritten (a ledger note is not a master);
  - `architecture-amendments.md` is over the 120 000 B whole-read bound (153 314 B); `test-plan-amendments.md` is
    at 118 035 B and will pass it at its next entry. Phase reads the first through its index;
  - the friction ledger holds records whose version reads `0.1.0`, not `viola-0.1.0` (74 as counted on 2026-10-09,
    not re-counted);
  - a live `claude` session ending in this tree makes the session-end hook rewrite this file's last section;
  - the gate tool's stamp moved from v1.13 to v1.14 between this session's implement and its wrap (a pipeline
    update; both letters now say v1.14).
- **Curation:** two Tier-2 entries (`testing.md`: no test sends a kill at a process it did not start, the PID 1
  case named as the one standing exception; `verification-harness.md`: forecast a Windows mutation job's wall
  from its viable mutants), one extension (`testing.md` 2026-09-24: `restrict` as the exception to the shared-body
  stub), two corrections of entries the per-file split made stale (`verification-harness.md`, `ci.md`), one pointer
  in `testing-src.md`. Log: the run dir's `curation.md`.
- **Deferred learnings** (carried; one recurred):
  - `recurrence-despite-learning: ci.md 2026-10-09` (`gh run list --commit` given a short sha, at this session's
    start, before any file under that rule's path was touched);
  - a mutation run of `viola-e2e` leaves session homes on the shared test-home base, because the tool's copy of
    the tree carries the `target/e2e-home` link (cap);
  - `recurrence-despite-learning: a time written into a record ahead of the clock`;
  - `recurrence-despite-learning: host-linux.md 2026-09-28/29` (a heredoc with a file target);
  - a window-class prefix such as `viola.` also matches the operator's own desktop windows; match a chunk's
    classes whole (cap);
  - a report names the route owner beside a claim a route CARRY already owns (cap);
  - `recurrence-despite-learning: host-linux.md` Paths (a `cd` into a subdirectory; met again in this session, a
    `cd` chained before a tool call);
  - `recurrence-despite-learning: host-linux.md` Exit codes (a tool listing read through a pipe);
  - `recurrence-despite-learning: host-linux.md` Long single-line files (a cut-limited view answering a
    membership question);
  - `recurrence-despite-learning: host-linux.md` (`rm -rf` in a compound, refused);
  - `recurrence-despite-learning: host-linux.md 2026-09-25` (`pkill -f` self-match);
  - `session-learnings.md 2026-09-29` (gate.py hygiene reads `/home/<x>/` in prose);
  - `host-linux.md` (zero-is-healthy count probe);
  - `testing.md` (bounded mutant-reachable waits);
  - the "not measured here" vocabulary;
  - PID 1 as the cleanup-deadline target (now named in `testing.md` as the one standing exception);
  - the PTY master close needing no held clone;
  - let a red CI run finish before folding its fix;
  - the doubled-backslash guard recurrence;
  - `grep` on the Linux dev host is ugrep, and a counted-context `-o -E` extraction is refused as too complex;
    the host's GNU grep stalls past the tool bound on a bounded repetition of a thousand characters or more over
    a multi-KB line: read a window by offset (`cascade.py window`);
  - `inputs.py snap` takes an in-session answer with `--message-file … --origin …`, never `--source`;
  - a subagent's return can arrive with tag-like text neutralised: never paste a detector's change line that
    holds a tag;
  - a `pgrep -f` on a script name reads other projects' sessions; decide by cwd;
  - the Bash tool's `find` is an embedded finder that refuses a GNU-style `-newermt` timestamp; the ISO form
    works;
  - the private directory of a hand-driven live rig is recorded nowhere in the tree unless the chunk writes its
    path into its evidence.
- **Operator desk (the founder's word: leave them; carried, not re-counted, with this chunk's additions):**
  - from this chunk: the three witness runs' outputs under the ignored `target/witness-wmg/`; `target/wincheck/`
    (the Windows-target lint's own target dir); the nine stripped job logs of run 38036448183 in this session's
    scratchpad on `/tmp`;
  - the `.tmp*` directories in `<repo parent>/viola-mutants-scratch`; the `viola-session-*` homes of the score run
    on the tmpfs behind `target/e2e-home` (gone at a reboot); `mutants.out/` and `mutants.out.old/` at the
    repository root (ignored by git);
  - Run D's plan file and the 2026-10-08 chunk's one plan file under the CLI's plans directory in the user's home;
  - the gitignored `.viola-verify-*` dirs at the root;
  - `crates/viola-e2e/.viola-verify-2676638-plan/` (a killed verify's probe dir under a member directory);
  - `~/.viola-record-20261004T142325Z`;
  - the CLI's own transcripts of every live session;
  - on the tmpfs behind the `target/e2e-home` link, gone at a reboot: the live home `viola-live-4043089/` with
    its instance directories and the pinned copy `bin/0.1.0-b4659b98029d0f94/`; the earlier
    `viola-reverify-20261007T124408Z/`; and `$XDG_RUNTIME_DIR/vcomp/`;
  - under the implement sessions' scratch directories on `/tmp`: the rig directory `icr-rig/` and the earlier
    chunk's;
  - yours to delete: `target/e2e-home.disk`, whole, and the eleven directories under `target/profraw-census/`.
    No chunk in flight reads either.
- **Last failed command:** none.
