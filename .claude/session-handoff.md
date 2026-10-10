# Session Handoff

**Last Updated:** 2026-10-10T11:23Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-10-self-healing-state — the next append heals a torn last line, the reader counts three names, the snapshot read and the log replay stand as library code

## Position
- Done: **2026-10-10-self-healing-state** (50 complete, 0 pending, 0 gated; 16/53 verified; the chunk claimed no
  capability, `v1-41` is advanced and not proven). CI green on its pre-CI commit `0af8283`, `ci#38046300968`,
  15/15, first attempt.
- Next entry: **viola revive** (the head of the markerless tail) → `/andromeda-phase`.

## Work done
- An append to a log whose last line was cut short starts on a fresh line: one LF and the line in a single
  write, one `state-recovered` line per heal. Every earlier offset still starts the same line.
- The reader counts `unknown_kinds`, `unknown_fields` and `torn_lines`, and no longer returns a line of an
  unknown kind. No surface shows the counts yet.
- `viola-state` holds a snapshot read that tells four cases apart and the log replay, as library code with no
  caller. Nothing under `src/` changed. The crate has its first `tests/` suite; a root chaos case covers the heal.
- The session ran in three parts: orientation, implement with the operator pass, this wrap.

## Drift resolved
- 24 amendments over four masters: architecture 7 (the State Store decision sentence and its as-landed text, the
  Snapshot envelope, the directory tree, the root wait count 23 sites in 17 files, the logging dependency),
  security-plan 2 (the "Own state files on read" row, the threat model's bullet), test-plan 9 (the torn append
  as a stop and a shortened log, §4 and §5 as landed, Scenario E5 owed), obs-plan 6 (where `state-recovered` is
  written, the catalog row, the dependency wording). Six sidecar entries. Leaves re-derived: CLAUDE.md's
  modules line, `rules/events.md`, `rules/observability.md`, `docs/stack.md`, `docs/services/viola-state.md`.
- One escalation, resolved by the operator at the halt (`inputs#I5` of the chunk): architecture's decision
  sentence now reads "The next append heals a torn last line; readers count it and never rewrite the log".
- 24 detector proposals from four docs, all applied; design-system, layout-templates and a11y-plan returned
  nothing. The citation sweep re-pointed nothing and printed no row.
- Record: `.andromeda/runs/2026-10-10T11-03-09-wrap/` (`fanout-results.md`, `cascade-dispositions.md`,
  `curation.md`).

## Notes
- **Route, this wrap:** one new entry, "Interrupted verify cleanup", first in Epoch 5 ahead of "Paste newline
  ledger row" (the founder's word of 2026-10-10T10:26:30Z, relayed by the operator; the place on the operator's
  answer). It carries the killed-verify CARRY and the three dir classes. Five CARRYs from this chunk: "viola
  revive", "Budget governor", "Session links", "The board: viola list", "Server verification before any frame".
  No reorder.
- **For the operator's word, new here:** the new entry has no `requirements.md` line and no ledger entry. The
  founder's word names a route entry; one line from the operator mints the requirement.
- **For the founder (carried, with one addition):**
  - new: the CARRY on "Server verification before any frame" names a boundary matter. What a client's pre-check
    does with an unreadable or newer snapshot needs his rule before that entry is planned;
  - the entry "Paste newline ledger row" needs his ruling on the `viola verify` child set (a fifth Run B paste)
    at its take-up, and a cap of live starts of its own;
  - one sentence stands in architecture's capability-ledger pattern without a halt (one relied-on shape has no
    row yet); one line if he wants it out;
  - a listed local command followed by CR or CRLF classifies as that command; not shown to him yet;
  - the switch from the prototype is not made; `v1-33` is unclaimed, pinned on "The board: viola list";
  - not measured: any CLI version but 2.1.287, a long or wrapped multi-line text, an LF inside a paste on
    Windows, the wheel's return by a human `release` on a live session;
  - one CLI plan file is left in his user directory (2026-10-08). viola wrote nothing there.
- **Not measured at this chunk:** the two `path4` cases' zero count has no red reading; whether CI's G4 step read
  the chaos case's home (the job's artifact was not opened).
- **Epoch growth:** Epoch 4 stands at 10 entries (5 complete, 5 markerless). No boundary is minted inside it: the
  founder's word, 2026-10-09 (carried). Epoch 5 stands at 6.
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
- **For whoever dispatches `windows-mutants.yml` next (carried):** a job's wall follows its count of viable
  mutants (about 100 s each in the root package). This chunk added mutable code to `viola-state` and `viola-core`
  and ran no mutation run; the epoch boundary's audit grades it.
- **Route:** `BLOCKED-ON` on "Windows-only live measurements" stands: `uname -s` read Linux at this wrap. The
  stale obs-plan sentence on `human::refuse` still rides "CLI output discipline".
- **For the operator's word** (carried, with this wrap's figures):
  - `v1-33`'s title and `requirements.md:48` still say "on Windows"; no wrap step may edit either;
  - the ledger's dated notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90` for the entry "Windows-only live
    measurements"; none was rewritten (a ledger note is not a master);
  - two sidecars are over the 120 000 B whole-read bound: `architecture-amendments.md` (157 254 B) and, since
    this wrap, `test-plan-amendments.md` (122 187 B). Phase reads them through their index;
  - the friction ledger holds records whose version reads `0.1.0`, not `viola-0.1.0` (74 as counted on 2026-10-09,
    not re-counted);
  - a live `claude` session ending in this tree makes the session-end hook rewrite this file's last section.
- **Curation:** one Tier-2 extension (`testing.md` 2026-09-25: an absence case's control forces the behaviour
  on, and a red-before-product reading runs on stubs). The stale `events.md` sentence was corrected through its
  master and the cascade. Three candidates rejected. Log: the run dir's `curation.md`.
- **Deferred learnings** (carried; one recurred):
  - `recurrence-despite-learning: host-linux.md` Paths (a `cd` outside a subshell four times in this session;
    one moved the working directory into `.andromeda/` for one call);
  - `recurrence-despite-learning: ci.md 2026-10-09` (`gh run list --commit` given a short sha);
  - a mutation run of `viola-e2e` leaves session homes on the shared test-home base, because the tool's copy of
    the tree carries the `target/e2e-home` link (cap);
  - `recurrence-despite-learning: a time written into a record ahead of the clock`;
  - `recurrence-despite-learning: host-linux.md 2026-09-28/29` (a heredoc with a file target);
  - a window-class prefix such as `viola.` also matches the operator's own desktop windows; match a chunk's
    classes whole (cap);
  - a report names the route owner beside a claim a route CARRY already owns (cap);
  - `recurrence-despite-learning: host-linux.md` Exit codes (a tool listing read through a pipe);
  - `recurrence-despite-learning: host-linux.md` Long single-line files (a cut-limited view answering a
    membership question);
  - `recurrence-despite-learning: host-linux.md` (`rm -rf` in a compound, refused);
  - `recurrence-despite-learning: host-linux.md 2026-09-25` (`pkill -f` self-match);
  - `session-learnings.md 2026-09-29` (gate.py hygiene reads `/home/<x>/` in prose);
  - `host-linux.md` (zero-is-healthy count probe);
  - `testing.md` (bounded mutant-reachable waits);
  - the "not measured here" vocabulary;
  - PID 1 as the cleanup-deadline target (named in `testing.md` as the one standing exception);
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
- **Operator desk (the founder's word: leave them; carried, with this chunk's readings):**
  - `crates/viola-e2e/.viola-verify-227786-plan/` is the one leftover verify dir found at this session's listing
    (the operator's own, per the P5 review). The earlier list's `.viola-verify-2676638-plan/` and the root
    `.viola-verify-*` dirs were not found;
  - from the earlier chunks: `target/witness-wmg/`, `target/wincheck/`; the `.tmp*` directories in
    `<repo parent>/viola-mutants-scratch`; `mutants.out/` and `mutants.out.old/` at the repository root (ignored
    by git); the `viola-session-*` homes and the live homes on the tmpfs behind `target/e2e-home` (gone at a
    reboot); `$XDG_RUNTIME_DIR/vcomp/`; the rig directories under earlier sessions' scratch on `/tmp`;
  - Run D's plan file and the 2026-10-08 chunk's one plan file under the CLI's plans directory in the user's home;
  - `~/.viola-record-20261004T142325Z`; the CLI's own transcripts of every live session;
  - yours to delete: `target/e2e-home.disk`, whole, and the eleven directories under `target/profraw-census/`.
    No chunk in flight reads either.
- **Last failed command:** none.
