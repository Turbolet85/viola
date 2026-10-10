# Session Handoff

**Last Updated:** 2026-10-10T01:49Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-09-epoch-3-cleanup-ii — root test scaffolding shared once, three functions within the cognitive ceiling, the first whole-unit viola-e2e score on the dev host, eighteen Linux survivors killed or restated

## Position
- Done: **2026-10-09-epoch-3-cleanup-ii** (48 complete, 0 pending, 0 gated; 16/53 verified; the chunk claimed no
  capability). CI green on its pre-CI commit `632f6a7`, `ci#38012420489`, 15/15, first attempt.
- Next entry: **Windows mutation grade** (the head of the markerless tail, in Epoch 4) → `/andromeda-phase`.
- After it: **Self-healing state**, still the first feature entry.

## Work done
- The root tests' `events.ndjson` reader, its wait, the session-start boot and the started-child guard and runner
  live once in `tests/support/events.rs` and `tests/support/cli.rs`; nineteen per-file copies are gone.
  `dialog_variants`, `record` and the fake agent's `submit` are split in place. No behaviour changed.
- `viola-e2e` has its first whole-member score on the dev host: 718 mutants in 5356 s, 656 caught, 2 missed,
  0 timeout, 60 unviable. The 2 missed are Windows-only. The eighteen Linux survivors: fourteen killed by a new
  case, four gone by restating an expression.
- The wrap ran in two windows (Setup and P1, then P2 to P7 in a cleared one, on the operator's word).

## Drift resolved
- 5 amendments over two masters: architecture 1 (the Watch bound's waits are counted by pattern, 22 sites, the
  "9 root waits" list retired after its rule was read); test-plan 4 (the shared helpers named in §2, the score in
  §10, the same count and the new wall in two §3 key files). 2 sidecar entries, 2 leaves changed
  (`tests-summary.md`, `rules/testing.md`). The other five masters returned nothing.
- One detector proposal was rejected for resting on its own greps; the orchestrator raised the same amendment
  from the plan's list. One proposal carried `escalate` and was settled by the operator's recorded direction.
- The citation sweep re-pointed nothing and printed no row.
- Record: `.andromeda/runs/2026-10-10T01-25-38-wrap/` (`fanout-results.md`, `cascade-dispositions.md`).

## Notes
- **Route, this wrap (four CARRYs, no new entry, no reorder):**
  - "Windows mutation grade": the two missed `viola-e2e` mutants in `prepare` and the unread Windows grade of
    `fs.rs:290:19` (the operator's word); and a stale security-plan sentence that still lists a mutation job in
    `ci.yml`.
  - "Paste newline ledger row": the four root mutants caught only by the 10 s kill, with no failing assertion
    (the operator's answer in the route dialogue, four placements shown with prices; the recommended one was the
    next entry and was not taken).
  - "CLI output discipline": a stale obs-plan sentence that says `human::refuse` is called only by `run`.
  - The two stale sentences are not this chunk's and were not amended; two detectors flagged them outside their
    scope and both were read true.
- **Proposed, not appended (the operator's word):** a playbook rule for a count in a master that carries no rule:
  read the sidecar entry that wrote it, then amend every site that holds it to a count with its rule named.
- **For whoever plans "Windows mutation grade":** the six retry-loop mutants of `replace_private_with` are already
  graded caught on the `windows-2025` runner (run 37761947926); that CARRY owes their record, not a new test. On
  the dev host a whole-member `viola-e2e` run takes about 89 min and the four-file root witness 66 min, one at a
  time. cargo-mutants keeps one earlier run's per-mutant logs only.
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
- **Epoch growth:** Epoch 4 stands at 10 entries (3 complete, 7 markerless). No boundary is minted inside it: the
  founder's word, 2026-10-09, repeated by the operator at this wrap.
- **The unexplained file-hash difference has an owner:** a CARRY on "Home and code-bearing file integrity".
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s harness-prefix row and its R8
  identity-floor row. A probed row for either needs its own founder ruling.
- **Setup:** `upgrade.py detect` reads one entry for setup, U03 (`scripts/code-graph.py`, 27 lines behind its
  template) → `/andromeda-setup-project`. A setup re-run must keep `.claude/rules/ci.md` and
  `.claude/rules/testing-src.md`. The host leaf is `host-linux.md`.
- **Other terminals:** the wheel's closed list was measured on foot 1.28.0 only (a CARRY on "Linux and macOS
  parity").
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
- **For the operator's word** (carried):
  - `v1-33`'s title and `requirements.md:48` still say "on Windows"; no wrap step may edit either;
  - the ledger's dated notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90` for the entry "Windows-only live
    measurements"; none was rewritten (a ledger note is not a master);
  - `architecture-amendments.md` is over the 120 000 B whole-read bound (150 542 B); phase reads it through its
    index;
  - the friction ledger holds 74 records whose version reads `0.1.0`, not `viola-0.1.0` (counted at this
    session's start, not read);
  - a live `claude` session ending in this tree makes the session-end hook rewrite this file's last section.
- **Curation:** one Tier-1 entry (a count in a master carries its rule) and two Tier-2 entries in
  `verification-harness.md` (cargo-mutants' per-mutant logs; two sweep hazards over its outcome lines). Log: the
  run dir's `curation.md`.
- **Deferred learnings** (carried; one added, two recurred):
  - added, by the cap: a mutation run of `viola-e2e` leaves session homes on the shared test-home base, because
    the tool's copy of the tree carries the `target/e2e-home` link;
  - `recurrence-despite-learning: a time written into a record ahead of the clock` (again: once in the implement
    run, three stamps in this wrap);
  - `recurrence-despite-learning: host-linux.md 2026-09-28/29` (a heredoc with a file target; again in the
    implement run);
  - a window-class prefix such as `viola.` also matches the operator's own desktop windows; match a chunk's
    classes whole (cap);
  - a report names the route owner beside a claim a route CARRY already owns (cap);
  - `recurrence-despite-learning: host-linux.md` Paths (a `cd` into a subdirectory, refused);
  - `recurrence-despite-learning: host-linux.md` Exit codes (a tool listing read through a pipe);
  - `recurrence-despite-learning: host-linux.md` Long single-line files (a cut-limited view answering a
    membership question);
  - `recurrence-despite-learning: host-linux.md` (`rm -rf` in a compound, refused);
  - `recurrence-despite-learning: host-linux.md 2026-09-25` (`pkill -f` self-match);
  - `session-learnings.md 2026-09-29` (gate.py hygiene reads `/home/<x>/` in prose);
  - `host-linux.md` (zero-is-healthy count probe);
  - `testing.md` (bounded mutant-reachable waits);
  - the "not measured here" vocabulary;
  - PID 1 as the cleanup-deadline target;
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
- **Operator desk (the founder's word: leave them):**
  - from this chunk: 34 `.tmp*` directories in `<repo parent>/viola-mutants-scratch` (250 MB); 26
    `viola-session-*` homes of the score run on the tmpfs behind `target/e2e-home` (gone at a reboot);
    `mutants.out/` and `mutants.out.old/` at the repository root (ignored by git); the four raw mutation-run
    stderr captures in the implement session's scratchpad;
  - Run D's plan file and the 2026-10-08 chunk's one plan file under the CLI's plans directory in the user's home;
  - the gitignored `.viola-verify-*` dirs at the root (eleven);
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

## Session End Status
Completed normally at 2026-10-10 04:44:13
