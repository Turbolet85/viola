# Session Handoff

**Last Updated:** 2026-10-09T17:36Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-09-epoch-3-cleanup — send types a text without its trailing CR and LF and refuses an empty typed text; the viola-e2e baseline passes under the mutants profile; three rule homes

## Position
- Done: **2026-10-09-epoch-3-cleanup** (46 complete, 0 pending, 0 gated; 16/53 verified; the chunk claimed no
  capability). CI green on its pre-CI commit `211da16`, `ci#37963309241`, 15/15, first attempt.
- Next entry: **Inner CR and CRLF in a sent text** (the head of the markerless tail, in Epoch 4) → `/andromeda-phase`.
- After it: **Epoch 3 cleanup II**, then **Windows mutation grade**, then **Self-healing state**, still the first
  feature entry.

## Work done
- `send` types a text without its trailing CR and LF characters, so a text ending in LF, CR or CRLF is confirmed
  and the driver keeps the wheel. A text with nothing left to type is refused at once, `not-delivered` /
  `empty-text`, exit 13, by the client before any frame and by the wrapper before the first wheel read.
- The killed `viola-e2e` test carries the `verify_window_` name and that class's override stands first in the
  `mutants` profile: the unit's unmutated baseline reads 258 of 258 under it.
- New rule files `.claude/rules/ci.md` and `.claude/rules/testing-src.md`; one line each in the harness rule and
  the gotchas.
- Four live starts on `claude` 2.1.287, headless, of the founder's cap of five. Records: the chunk's
  `evidence/live-run.md`.

## Drift resolved
- 52 amendments over six masters (architecture 16, security-plan 10, test-plan 14, obs-plan 7, design-system 3,
  layout-templates 2), 12 sidecar entries, 11 leaf files re-derived. Seven carried a detector's `escalate`
  severity and were resolved on the founder's recorded rulings, no halt.
- Every stale bare route number found is cited by title, ten sites with their leaves: seven for the two Epoch 6
  entries (security-plan four, test-plan two, obs-plan one) and three for "Self-healing state" (architecture, its
  key file, security-plan). The citation sweep printed no row.
- Record: `.andromeda/runs/2026-10-09T17-10-00-wrap/` (`fanout-results.md`, `cascade-dispositions.md`).

## Notes
- **For the founder:**
  - **the inner-CR ruling is placed, not landed** (his, 2026-10-09T16:51Z, relayed): `send` types a CR or a CRLF
    inside a text as the LF the CLI submits. It is the route's head entry, on the operator's answer at this wrap.
    Until it lands, a text with a CR or a CRLF inside it is delivered, answered, reported not delivered, and
    takes the wheel (measured on 2.1.287, two readings). Not measured: several inner CRs, an inner LF CR, any
    other CLI version.
  - **a live confirmation for that entry needs a cap of starts of its own.** The cap of five given for this
    chunk has one start unused; it was given for the trailing readings.
  - **one consequence of the CR strip was not shown to him:** a listed local command followed by CR or CRLF
    now classifies as that command (`/clear` and a CRLF is `/clear`), as `/clear` and an LF already did. It is
    pinned by a unit case and stated in architecture [Delivery Confirmation].
  - **the switch from the prototype is not made**; `v1-33` is unclaimed. It is pinned on "The board: viola list"
    with the 2026-10-08 chunk's `evidence/gap-list.md`. His at that entry's take-up.
  - **one CLI plan file is left in his user directory** (the CLI's default plans directory, 2026-10-08). viola
    wrote nothing there.
  - still his: the wheel's return by a human `release` on a live session is not measured.
- **Epoch growth:** Epoch 4 stands at 10 entries after this wrap's two insertions (1 complete, 9 markerless). A
  boundary here would restore the diagnose and audit cadence; the split is the operator's word.
- **For whoever plans "Epoch 3 cleanup II":** the reads made for it stand in this chunk's `scope.md` §1, §2, §3
  and §5 and its `research.md`, coordinates at `59e791e`.
- **For whoever plans "Windows mutation grade":** the six retry-loop mutants of `replace_private_with` are
  already graded caught on the `windows-2025` runner (run 37761947926). That CARRY owes their record, not a new
  test.
- **A later setup re-run must keep** `.claude/rules/ci.md` and `.claude/rules/testing-src.md`: a chunk wrote
  them as its own stated work, not setup.
- **The diagnosis's other proposals** (the pipeline's) were not read or placed; only P21 and P31 were.
- **The coverage-profile WATCH is retired.** The limit stands: ci#36529038462 and ci#36481260151 are not claimed
  closed.
- **Other terminals:** the wheel's closed list was measured on foot 1.28.0 only (a CARRY on "Linux and macOS
  parity").
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s two rows, the harness-prefix row
  and the R8 identity-floor row. A probed row for either needs its own founder ruling.
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
- **Route:** `BLOCKED-ON` on "Windows-only live measurements" stands: no interactive Windows host, the dev host
  read Linux at this wrap.
- **For the operator's word** (carried):
  - `v1-33`'s title and `requirements.md:48` still say "on Windows"; no wrap step may edit either;
  - the ledger's dated notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90` for the entry "Windows-only live
    measurements"; none was rewritten (a ledger note is not a master);
  - `architecture-amendments.md` is over the 120 000 B whole-read bound (146 387 B); phase reads it through its
    index;
  - a live `claude` session ending in this tree makes the session-end hook rewrite this file's last section.
- **Curation:** one Tier-1 entry (a route entry is named by its title, never by a bare line number); one Tier-3
  extension (a gate entry that only reads a file takes no `artifact` key). Log: the run dir's `curation.md`.
- **Deferred learnings** (carried, with one recurrence):
  - `recurrence-despite-learning: a time written into a record ahead of the clock` (the report's date, typed 6
    minutes ahead; the stamp hook refused it, one edit after a clock read corrected it);
  - a window-class prefix such as `viola.` also matches the operator's own desktop windows; match a chunk's
    classes whole (cap);
  - a report names the route owner beside a claim a route CARRY already owns (cap);
  - `recurrence-despite-learning: host-linux.md` Paths (a `cd` into a subdirectory, refused);
  - `recurrence-despite-learning: host-linux.md 2026-09-28/29` (a heredoc with a file target);
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
  - the private directory of a hand-driven live rig is recorded nowhere in the tree, so a re-entry after a
    context reset has to search for it.
- **Operator desk (the founder's word: leave them):**
  - Run D's plan file and the 2026-10-08 chunk's one plan file under the CLI's plans directory in the user's home;
  - the gitignored `.viola-verify-*` dirs at the root (eleven);
  - `crates/viola-e2e/.viola-verify-2676638-plan/` (a killed verify's probe dir under a member directory);
  - `~/.viola-record-20261004T142325Z`;
  - the CLI's own transcripts of every live session, this chunk's four included;
  - on the tmpfs behind the `target/e2e-home` link, gone at a reboot: the live home `viola-live-4043089/` with the
    five instance directories this chunk added (`rehearse`, `e3crlf`, `e3crcr`, `e3after`, `e3innercr`), the
    earlier `viola-reverify-20261007T124408Z/`; and `$XDG_RUNTIME_DIR/vcomp/`;
  - under the first implement session's scratch directory on `/tmp`: the rig's private directory (the sent
    texts, the journal, the row files);
  - yours to delete: `target/e2e-home.disk`, whole, and the eleven directories under `target/profraw-census/`.
    No chunk in flight reads either.
- **Setup:** the upgrade U48 ran at `fe4f47f`; the host leaf is `host-linux.md`.
- **Last failed command:** none.
