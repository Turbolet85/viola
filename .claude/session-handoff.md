# Session Handoff

**Last Updated:** 2026-10-10T15:49Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-10-viola-revive — viola revive resumes a dead instance in place by its newest logged session id, in the recorded cwd, with four preflight refusals

## Position
- Done: **2026-10-10-viola-revive** (51 complete, 0 pending, 0 gated; 16/53 verified; the chunk claimed no
  capability, `v1-41` is advanced and not proven). CI green on its pre-CI commit `0fad11c`, `ci#38061685124`,
  15/15, first attempt.
- Next entry: **Statusline pass-through** (the head of the markerless tail) → `/andromeda-phase`.

## Work done
- `viola revive <name> [--id <ID>] [--fork] [-- <child args>]` restarts a dead instance through `run`'s own
  start: program `claude` by name, `--resume <id>` with the newest logged session id, the child spawned in the
  snapshot's recorded `cwd`. `--list` prints the logged sessions. Four preflight refusals, exit 1, in a fixed
  order: `strict-modes-failed`, `already-live`, `no-session`, `cwd-missing`. `session-live` is not built.
- The snapshot holds an optional `cwd`. `viola-state` has a session-chain reader and an instance strict-modes
  check, and the log replay has its first product reader. The fake agent takes `--resume` and `--fork-session`.
- Three live starts on `claude` 2.1.287 (the founder's number): the resume logged cause `resume` with the first
  life's id. Record: the chunk's `evidence/live-revive.md`.
- The wrap ran in two sessions: Phase 1, then this one from Phase 2 (`resume-point.md` in the run dir).

## Drift resolved
- 75 detector proposals over six masters (design-system returned none): 74 applied, 1 rejected (an unmeasured
  inference about the harness `cleanup`, carried as a labelled hypothesis). The design-system amendment was raised
  by the wrap. 12 sidecar entries: architecture 4, security-plan 2, test-plan 2, obs-plan 1, a11y-plan 1,
  design-system 1, layout-templates 1. Five key files edited; the registry check reads clean.
- Three escalated proposals, resolved by the operator at the Phase 2 halt (`inputs#I8` of the chunk): the session
  id on the child's command line lands as fact, not ratified; `revive --list`'s missing process log is a sixth
  panic exemption that stands only until `--list` gains `--json`.
- The recorded cwd as the child's spawn directory is recorded as the founder's ratified widening (`inputs#I3`).
- Leaves re-derived: CLAUDE.md (five lines), `rules/events.md`, `rules/observability.md`, `rules/security.md`,
  `docs/stack.md`, `conventions.md`, `commands.md`, `gotchas.md`, `obs-summary.md`, `security-summary.md`,
  `tests-summary.md`, `services/viola.md`, `viola-state.md`, `viola-agent-claude.md`.
- Citation sweep: 0 re-pointed in the masters; two route rows re-pointed by hand at Phase 5.
- Record: `.andromeda/runs/2026-10-10T15-07-22-wrap/` (`fanout-results.md`, `cascade-dispositions.md`,
  `route-record.md`, `curation.md`).

## Notes
- **For the founder, new at this wrap:**
  - **his word is owed** on one crossing: `viola revive` puts the logged session id on the child's command line
    after `--resume` (closed 36-character shape, a member of the instance's log, one argv element). It was not
    shown to him as a widening. security-plan's new row says "Not ratified", and architecture and
    `rules/security.md` say his word is owed. The operator shows it to him; the next wrap records his word as his;
  - `viola revive --list` opens no process log, so a panic there writes no line (obs-plan §10, the sixth
    exemption). It ends when `--list` gains `--json` on "CLI machine contract";
  - on the dev host the bare name `claude` is 2.1.289 and the home is stamped for 2.1.287, so a revive typed
    there with no `PATH` change runs an unstamped CLI (transport-only by design).
- **Route, this wrap:** nine CARRYs from this chunk, no entry added or moved, each placement in
  `route-record.md`: "The board: viola list" (`session-live`, P4, P12; the founder, `inputs#I5`); "Paste newline
  ledger row" (the owed `--resume` row with the real payload's key set, and `/compact` through `send` exiting
  13); "CLI machine contract" (revive's `--json`, `--list`'s log); "Exit-cause code catalogue" (the revive
  causes); "Unix endpoint and home hardening" (the killed wrapper's leftover socket; a hypothesis on the harness
  `cleanup`); "Linux and macOS parity" (the child-gone reading, the tab-close leg); "Home and code-bearing file
  integrity" (no Windows case for the instance check); "Interrupted verify cleanup" (the literal `claude` in
  `src/cmd/verify.rs`).
- **For the founder (carried, with this wrap's changes):**
  - the entry "Paste newline ledger row" now holds three shapes for his ruling on the `viola verify` child set
    at its take-up (a fifth Run B paste, a resume probe, `/compact`), and needs a cap of live starts of its own;
    its phase proposes a cut if it no longer fits one window;
  - the CARRY on "Server verification before any frame" names a boundary matter: what a client's pre-check does
    with an unreadable or newer snapshot needs his rule before that entry is planned;
  - architecture's capability-ledger pattern now names two relied-on shapes with no row; one line if he wants
    the sentence out;
  - a listed local command followed by CR or CRLF classifies as that command; not shown to him yet;
  - the switch from the prototype is not made; `v1-33` is unclaimed, pinned on "The board: viola list";
  - not measured: any CLI version but 2.1.287, a long or wrapped multi-line text, an LF inside a paste on
    Windows, the wheel's return by a human `release` on a live session; and from this chunk `--fork` on the real
    CLI, a resume of a killed session on the real CLI, whether a resumed session held its earlier turns, project
    settings on a resume from another directory, the bounded removal's loop on Windows;
  - one CLI plan file is left in his user directory (2026-10-08). viola wrote nothing there.
- **Epoch growth:** Epoch 4 stands at 10 entries (6 complete, 4 markerless). No boundary is minted inside it: the
  founder's word, 2026-10-09 (carried). Epoch 5 stands at 6.
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s harness-prefix row and its R8
  identity-floor row. A probed row for either needs its own founder ruling.
- **For the operator's word** (carried, with this wrap's figures):
  - "Interrupted verify cleanup" has no `requirements.md` line and no ledger entry; one line mints it;
  - `v1-33`'s title and `requirements.md:48` still say "on Windows"; no wrap step may edit either;
  - the ledger's dated notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90` for the entry "Windows-only live
    measurements"; none was rewritten (a ledger note is not a master);
  - two sidecars are over the 120 000 B whole-read bound: `architecture-amendments.md` (166 423 B) and
    `test-plan-amendments.md` (126 456 B). Phase reads them through their index;
  - the friction ledger holds records whose version reads `0.1.0`, not `viola-0.1.0` (74 as counted on
    2026-10-09, not re-counted);
  - a live `claude` session ending in this tree makes the session-end hook rewrite this file's last section.
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
  mutants (about 100 s each in the root package). The last two chunks added mutable code to the root package,
  `viola-state`, `viola-core` and `viola-agent-claude` and ran no mutation run; the epoch boundary's audit grades
  it.
- **Route:** `BLOCKED-ON` on "Windows-only live measurements" stands: `uname -s` read Linux at this wrap.
- **Curation:** three Tier-2 entries (`testing.md`: the fake agent as `claude` first on `PATH` for a verb that
  looks its program up by name; read the `tests/support/` helpers before saying a test asserts nothing.
  `verification-harness.md`: a live rig puts the stamped CLI's install directory first on `PATH`). Log: the run
  dir's `curation.md`. The candidates of the session that ran implement were read from `resume-point.md`.
- **Deferred learnings** (two new under the cap, then the carried list; three recurred):
  - new: the key files stand under `.andromeda/registries/contracts/<master>/`, so an owner map keyed on the
    registry's top-level names reads every key file as unowned (0.8, cap);
  - new: a wrapper a test ends through its drop guard writes no coverage profile, so a case whose start path
    must count toward coverage stops its wrapper cleanly (0.6, cap);
  - `recurrence-despite-learning: host-linux.md` Paths (a `cd` outside a subshell: once at implement, four times
    in this session);
  - `recurrence-despite-learning: host-linux.md 2026-09-28/29` (a heredoc with a file target, once at implement);
  - `recurrence-despite-learning: host-linux.md` Transports (an inline python heredoc for a multi-edit of a
    source file, once at implement);
  - `recurrence-despite-learning: ci.md 2026-10-09` (`gh run list --commit` given a short sha);
  - a mutation run of `viola-e2e` leaves session homes on the shared test-home base, because the tool's copy of
    the tree carries the `target/e2e-home` link (cap);
  - `recurrence-despite-learning: a time written into a record ahead of the clock`;
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
- **Operator desk (the founder's word: leave them; carried, with this chunk's additions):**
  - from this chunk: `target/rev-live-911840/` (its removal was denied by the permission layer and not done
    another way); three instance directories added to the stamped live home
    `target/e2e-home/viola-live-4043089` (`revreh`, `revlive`, `revhand`); the CLI's own transcripts of the three
    live sessions under its project directory for `target/rev-live-911840/a`; the rig's private directory under
    the implementing session's scratchpad, named in `evidence/live-revive.md`;
  - `crates/viola-e2e/.viola-verify-227786-plan/` (the operator's own, per an earlier P5 review);
  - from the earlier chunks: `target/witness-wmg/`, `target/wincheck/`; the `.tmp*` directories in
    `<repo parent>/viola-mutants-scratch`; `mutants.out/` and `mutants.out.old/` at the repository root (ignored
    by git); the `viola-session-*` homes and the live homes on the tmpfs behind `target/e2e-home` (gone at a
    reboot); `$XDG_RUNTIME_DIR/vcomp/`; the rig directories under earlier sessions' scratch on `/tmp`;
  - Run D's plan file and the 2026-10-08 chunk's one plan file under the CLI's plans directory in the user's home;
  - `~/.viola-record-20261004T142325Z`; the CLI's own transcripts of every live session;
  - yours to delete: `target/e2e-home.disk`, whole, and the eleven directories under `target/profraw-census/`.
    No chunk in flight reads either.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-10-10 20:46:38
