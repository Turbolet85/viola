# Session Handoff

**Last Updated:** 2026-10-10T20:35Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-10-statusline-pass-through — the statusline pass-through on Unix: a named read-only source, a per-start settings override, the user's command run after the instance check, readings to budget.json

## Position
- Done: **2026-10-10-statusline-pass-through** (52 complete, 0 pending, 0 gated; 16/53 verified; the chunk
  claimed no capability, `v1-24` and `v1-45` are advanced and not proven). CI green on `67ab367`,
  `ci#38080855246`, 15/15, first attempt, after one red on its pre-CI commit `58f8720` (`ci#38080061631`: `lint`
  and `test` on `windows-2025`).
- Next entry: **Budget governor** (the head of the markerless tail) → `/andromeda-phase`.

## Work done
- On Unix `viola run` reads the user's statusline command from one named source (`<home>/statusline-source.json`,
  else for the default home the user's `.claude/settings.json`), records it in the snapshot, rewrites
  `instances/<name>/settings.json` at every start and passes `--settings`. `viola hook statusline` checks the
  instance before its log opens, writes `budget.json` when the payload holds `rate_limits`, and runs the command
  through `/bin/sh -c` under a 5 s provisional bound. On Windows no override is written and no command runs.
- Three live starts on `claude` 2.1.287 (the founder's number) read the shell, the override and the payload by
  hand. No ledger row, no probe, no fixture. Record: the chunk's `evidence/live-statusline.md`.
- The wrap ran in two sessions: Phase 1, then this one from Phase 2 (`resume-point.md` in the run dir).

## Drift resolved
- 66 detector proposals over five masters (design-system and layout-templates returned none): 65 applied, 1
  rejected for a source read the report does not carry and raised again from the orchestrator's own read. Five
  more raised by the wrap. 11 sidecar entries: architecture 4, security-plan 3, test-plan 2, obs-plan 1,
  a11y-plan 1. Six key files edited; the registry check reads clean.
- One halt, five items, all answered by the operator (`inputs#I6` of the chunk): the card of four crossings
  confirmed, the two start steps with no span recorded as by design, two route placements. One playbook rule
  appended: a widening the founder already answered still halts at the wrap, as one card with one confirm.
- The plan's acceptance 1 was unmet in one clause ("`--settings` … nowhere else in product code"): the masters
  now say what is true, by the operator's direction (`inputs#I5` item 4).
- Leaves re-derived: CLAUDE.md (five lines), `rules/security.md`, `rules/events.md`, `rules/observability.md`,
  `rules/verification-harness.md`, `docs/conventions.md`, `commands.md`, `security-summary.md`,
  `tests-summary.md`, `obs-summary.md`, `a11y-summary.md`, `services/viola.md`, `viola-state.md`,
  `viola-agent-claude.md`, `viola-core.md`.
- Citation sweep: 0 re-pointed in the masters; three route citations re-pointed by hand at Phase 5.
- Record: `.andromeda/runs/2026-10-10T19-55-42-wrap/` (`fanout-results.md`, `cascade-dispositions.md`,
  `route-record.md`, `curation.md`).

## Notes
- **For the founder, new at this wrap:**
  - **the card is his to see in the next report.** The masters and sidecars record four crossings as ratified
    by him, each with his word as relayed: the settings override written at every start and passed with
    `--settings`; the user's command through `/bin/sh -c`; the two source files; and the logged session id after
    `--resume` on a revived child. The operator's confirm at this wrap was a check that each cited word matches;
  - **a defect in shipped code, on the head entry:** `viola revive` opens its log before its instance check, and
    opening a log sets an existing home to 0700, so the check cannot refuse a widened home mode. Read in source,
    not measured on `revive`. Beside it, a hypothesis read in source and not measured: the statusline arm's own
    refusal lasts one invocation, because its log's opening narrows the home and the next invocation then passes;
  - a `--home` session shows no user status line until `statusline-source.json` is planted in that home (the
    cost of the option he chose);
  - his own `~/.claude/settings.json` holds a `statusLine` of type `command` (presence and type read, nothing
    else, never written). On the default home a wrapped session runs it through viola's hook;
  - three relied-on dependences now have no ledger row: the pasted LF, `claude --resume`, and the statusline's
    three shapes. Each owed row has its entry on the route;
  - the budget reading's `used_percentage` is an `f64` kept from 0 to 100, not the `Percent` newtype (the plan
    asked for a JSON number or `"unknown"`); one line if he wants the newtype;
  - not measured: macOS and Windows live; a user-scope status line; which of two `--settings` flags wins; the
    flag the CLI hands its shell; how the CLI cancels a running statusline script; the 5 s bound on a real
    session; the status line row with an empty output; an account without `rate_limits`; the `spend_limit` window.
- **Route, this wrap:** seven CARRYs from this chunk, no entry added or moved, each placement in
  `route-record.md`: "Budget governor" (the revive check order with its acceptance and witness; the `rate_limits`
  row; `boot --statusline-echo`); "Interrupted verify cleanup" (`verify`'s two `--settings` sites onto the
  constant); "Paste newline ledger row" (the override row, the Unix shell row, the real-CLI readings not taken);
  "Home and code-bearing file integrity" (the Windows home a test made first, its DACL unread); "Windows-only
  live measurements" (the Windows override, shell-out and row legs).
- **For the founder (carried, with this wrap's changes):**
  - `viola revive --list` opens no process log, so a panic there writes no line (obs-plan §10, the sixth
    exemption). It ends when `--list` gains `--json` on "CLI machine contract";
  - on the dev host the bare name `claude` is 2.1.289 and the home is stamped for 2.1.287, so a revive typed
    there with no `PATH` change runs an unstamped CLI (transport-only by design);
  - the entry "Paste newline ledger row" now holds five shapes for his ruling on the `viola verify` child set at
    its take-up (a fifth Run B paste, a resume probe, `/compact`, the settings override, the Unix shell), and
    needs a cap of live starts of its own; its phase sizes the entry and proposes a cut (the operator's answer,
    `inputs#I6`);
  - the CARRY on "Server verification before any frame" names a boundary matter: what a client's pre-check does
    with an unreadable or newer snapshot needs his rule before that entry is planned;
  - a listed local command followed by CR or CRLF classifies as that command; not shown to him yet;
  - the switch from the prototype is not made; `v1-33` is unclaimed, pinned on "The board: viola list";
  - not measured (earlier chunks): any CLI version but 2.1.287, a long or wrapped multi-line text, an LF inside
    a paste on Windows, the wheel's return by a human `release` on a live session, `--fork` on the real CLI, a
    resume of a killed session on the real CLI, whether a resumed session held its earlier turns, project
    settings on a resume from another directory, the bounded removal's loop on Windows;
  - one CLI plan file is left in his user directory (2026-10-08). viola wrote nothing there.
- **Epoch growth:** Epoch 4 stands at 10 entries (7 complete, 3 markerless). No boundary is minted inside it: the
  founder's word, 2026-10-09 (carried). Epoch 5 stands at 6.
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s harness-prefix row and its R8
  identity-floor row. A probed row for either needs its own founder ruling.
- **For the operator's word** (carried, with this wrap's figures):
  - "Interrupted verify cleanup" has no `requirements.md` line and no ledger entry; one line mints it;
  - `v1-33`'s title and `requirements.md:48` still say "on Windows"; no wrap step may edit either;
  - the ledger's dated notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90` for the entry "Windows-only live
    measurements"; none was rewritten (a ledger note is not a master);
  - two sidecars are over the 120 000 B whole-read bound: `architecture-amendments.md` (175 754 B) and
    `test-plan-amendments.md` (131 347 B). Phase reads them through their index;
  - the friction ledger holds records whose version reads `0.1.0`, not `viola-0.1.0` (74 as counted on
    2026-10-09, not re-counted);
  - a live `claude` session ending in this tree makes the session-end hook rewrite this file's last section.
- **Setup:** `upgrade.py detect` read 0 entries for setup at this wrap (two noted, U04 and U36). A setup re-run
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
  mutants (about 100 s each in the root package). The last three chunks added mutable code to the root package,
  `viola-state`, `viola-core` and `viola-agent-claude` and ran no mutation run; the epoch boundary's audit grades
  it.
- **Route:** `BLOCKED-ON` on "Windows-only live measurements" stands: `uname -s` read Linux at this wrap.
- **Curation:** three writes. `testing.md`: the 2026-10-03 entry on `cfg`-only imports extended (a helper called
  only by `cfg(unix)` cases is dead code on Windows; the dev host can lint the Windows target, check only).
  `host-linux.md`: `TMPDIR` is unset on this host. `session-learnings.md`: an acceptance sentence "X nowhere else
  in product code" is checked over the tree. Log: the run dir's `curation.md`. The candidates of the session that
  ran implement were read from `resume-point.md`.
- **Deferred learnings** (none new under the cap; six recurred, then the carried list):
  - `recurrence-despite-learning: host-linux.md 2026-09-28/29` (a heredoc with a file target, once at implement);
  - `recurrence-despite-learning: host-linux.md` Exit codes (one gate call's output read through a line filter);
  - `recurrence-despite-learning: ci.md 2026-10-09` (`gh run list --commit` given a wrong sha);
  - `recurrence-despite-learning: testing.md 2026-10-05` (nextest's padded duration broke a fixed-column split
    of `PASS` lines: 93 read as 72);
  - `recurrence-despite-learning: testing.md` Test data (three new tests read files after the `TestHome` that
    held them was dropped);
  - `recurrence-despite-learning: host-linux.md` Paths (a `cd` outside a subshell, three times in this session);
  - the key files stand under `.andromeda/registries/contracts/<master>/`, so an owner map keyed on the
    registry's top-level names reads every key file as unowned (cap);
  - a wrapper a test ends through its drop guard writes no coverage profile, so a case whose start path must
    count toward coverage stops its wrapper cleanly (cap);
  - `recurrence-despite-learning: host-linux.md` Transports (an inline python heredoc for a multi-edit of a
    source file);
  - a mutation run of `viola-e2e` leaves session homes on the shared test-home base, because the tool's copy of
    the tree carries the `target/e2e-home` link (cap);
  - `recurrence-despite-learning: a time written into a record ahead of the clock`;
  - a window-class prefix such as `viola.` also matches the operator's own desktop windows; match a chunk's
    classes whole (cap);
  - a report names the route owner beside a claim a route CARRY already owns (cap);
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
  - from this chunk (none committed, none removed): `target/e2e-home/viola-live-sl-20261010/` on the tmpfs (the
    unstamped rig home, three instance directories `slreh`, `slsrc`, `slbare`, its `budget.json`);
    `target/sl-live-20261010/` with `a` and `b`; the rig's private directory `sl-rig/` under the implementing
    session's scratchpad, named in `evidence/live-statusline.md`; the CLI's own files for the three sessions
    under its project directories for `a` and `b`; `target/wincheck/` was used again;
  - from the revive chunk: `target/rev-live-911840/` (its removal was denied by the permission layer and not
    done another way); three instance directories in the stamped live home
    `target/e2e-home/viola-live-4043089` (`revreh`, `revlive`, `revhand`); the CLI's own transcripts of its three
    live sessions; the rig's private directory named in that chunk's `evidence/live-revive.md`;
  - `crates/viola-e2e/.viola-verify-227786-plan/` (the operator's own, per an earlier P5 review);
  - from the earlier chunks: `target/witness-wmg/`; the `.tmp*` directories in
    `<repo parent>/viola-mutants-scratch`; `mutants.out/` and `mutants.out.old/` at the repository root (ignored
    by git); the `viola-session-*` homes and the live homes on the tmpfs behind `target/e2e-home` (gone at a
    reboot); `$XDG_RUNTIME_DIR/vcomp/`; the rig directories under earlier sessions' scratch on `/tmp`;
  - Run D's plan file and the 2026-10-08 chunk's one plan file under the CLI's plans directory in the user's home;
  - `~/.viola-record-20261004T142325Z`; the CLI's own transcripts of every live session;
  - yours to delete: `target/e2e-home.disk`, whole, and the eleven directories under `target/profraw-census/`.
    No chunk in flight reads either.
- **Last failed command:** none.

## Session End Status
Completed normally at 2026-10-10 22:00:40
