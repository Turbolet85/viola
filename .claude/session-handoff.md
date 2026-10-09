# Session Handoff

**Last Updated:** 2026-10-09T21:07Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-09-inner-cr-and-crlf-in-a-sent-text — send types a CR or a CR LF inside a text as one LF, so the send is confirmed and the driver keeps the wheel

## Position
- Done: **2026-10-09-inner-cr-and-crlf-in-a-sent-text** (47 complete, 0 pending, 0 gated; 16/53 verified; the chunk
  claimed no capability). CI green on its pre-CI commit `308099b`, `ci#37981185305`, 15/15, first attempt.
- Next entry: **Epoch 3 cleanup II** (the head of the markerless tail, in Epoch 4) → `/andromeda-phase`.
- After it: **Windows mutation grade**, then **Self-healing state**, still the first feature entry.

## Work done
- `send` types every CR LF pair and every other CR inside a text as one LF, and still drops the trailing CR and LF
  characters. No `send` types a CR now. A text with a CR or a CR LF inside it is confirmed and the driver keeps the
  wheel; before, it was delivered, reported not delivered, and took the wheel.
- The rule lives in `typed_text` alone. Validation, the refusal order, the exact match and every catalog are
  unchanged. It is pinned by unit cases, a 512-case property, a cross-process test and the raw-channel test.
- One live start of the founder's cap of two, on `claude` 2.1.287, headless: six sends, all confirmed. The control,
  read first: an LF typed inside a paste is submitted unchanged. Record: the chunk's `evidence/live-run.md`.

## Drift resolved
- 22 amendments over four masters (architecture 10, test-plan 6, security-plan 4, obs-plan 2), 4 sidecar entries,
  8 leaf files re-derived. design-system, layout-templates and a11y-plan returned nothing.
- Five proposals carried a detector's `escalate` and were resolved on the founder's recorded rulings, no halt: the
  four security-plan ones (the rule is his ruling of 2026-10-09T16:51Z; no boundary widens) and one in
  architecture (below, for the founder).
- The citation sweep re-pointed nothing and printed no row.
- Record: `.andromeda/runs/2026-10-09T20-50-14-wrap/` (`fanout-results.md`, `cascade-dispositions.md`).

## Notes
- **For the founder:**
  - **his inner-CR ruling is landed** and measured live. Not measured: any CLI version but 2.1.287, a long or
    wrapped multi-line text, an LF inside a paste on Windows.
  - **his word of 2026-10-09T20:58Z is on the route:** the owed `v1-34` row for an LF typed inside a paste is the
    new entry "Paste newline ledger row", first in Epoch 5. It needs his ruling on the `viola verify` child set
    (a fifth Run B paste) at its take-up, and a cap of live starts of its own.
  - **one sentence was written into architecture without a halt:** the capability-ledger pattern now says that
    one shape `send` relies on has no row yet, and names that entry as its owner. It states the fact he was shown
    and rules nothing about the requirement's reach. If he wants it out or reworded, it is one line.
  - **one consequence of the CR strip is still not shown to him** (carried): a listed local command followed by
    CR or CRLF classifies as that command (`/clear` and a CRLF is `/clear`). This chunk added no new case of it.
  - **the switch from the prototype is not made**; `v1-33` is unclaimed. It is pinned on "The board: viola list"
    with the 2026-10-08 chunk's `evidence/gap-list.md`. His at that entry's take-up.
  - **one CLI plan file is left in his user directory** (the CLI's default plans directory, 2026-10-08). viola
    wrote nothing there.
  - still his: the wheel's return by a human `release` on a live session is not measured.
- **The unexplained file-hash difference has an owner:** a CARRY on "Home and code-bearing file integrity"
  (Epoch 6), the operator's answer at this wrap. The final build's file differs from the live start's; their six
  loaded sections are equal. No record says why.
- **Epoch growth:** Epoch 4 stands at 10 entries (2 complete, 8 markerless). No boundary is minted inside it: the
  founder's word, 2026-10-09.
- **Not read:** the CI run's wall was 717 s against the last chunk's 416 s. Which job took longer was not read.
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s harness-prefix row and its R8
  identity-floor row. A probed row for either needs its own founder ruling.
- **For whoever plans "Epoch 3 cleanup II":** the reads made for it stand in chunk 2026-10-09-epoch-3-cleanup's
  `scope.md` §1, §2, §3 and §5 and its `research.md`, coordinates at `59e791e`.
- **For whoever plans "Windows mutation grade":** the six retry-loop mutants of `replace_private_with` are
  already graded caught on the `windows-2025` runner (run 37761947926). That CARRY owes their record, not a new
  test.
- **A later setup re-run must keep** `.claude/rules/ci.md` and `.claude/rules/testing-src.md`: a chunk wrote
  them as its own stated work, not setup. CLAUDE.md's first Workflow line now reads `**Key commands:**`, the
  template's line, on the operator's word at this wrap.
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
- **Route:** `BLOCKED-ON` on "Windows-only live measurements" stands: no interactive Windows host, the dev host
  read Linux at this wrap.
- **For the operator's word** (carried):
  - `v1-33`'s title and `requirements.md:48` still say "on Windows"; no wrap step may edit either;
  - the ledger's dated notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90` for the entry "Windows-only live
    measurements"; none was rewritten (a ledger note is not a master);
  - `architecture-amendments.md` is over the 120 000 B whole-read bound (149 119 B); phase reads it through its
    index;
  - a live `claude` session ending in this tree makes the session-end hook rewrite this file's last section.
- **Curation:** two Tier-2 entries: `gh run list --commit` needs the full sha (`ci.md`); a file hash does not
  prove two builds load the same code, the loaded sections do (`verification-harness.md`). Log: the run dir's
  `curation.md`.
- **Deferred learnings** (carried, none added):
  - `recurrence-despite-learning: a time written into a record ahead of the clock`;
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
  - the private directory of a hand-driven live rig is recorded nowhere in the tree unless the chunk writes its
    path into its evidence (this chunk did, in `evidence/live-preconditions.md`).
- **Operator desk (the founder's word: leave them):**
  - Run D's plan file and the 2026-10-08 chunk's one plan file under the CLI's plans directory in the user's home;
  - the gitignored `.viola-verify-*` dirs at the root (eleven);
  - `crates/viola-e2e/.viola-verify-2676638-plan/` (a killed verify's probe dir under a member directory);
  - `~/.viola-record-20261004T142325Z`;
  - the CLI's own transcripts of every live session, this chunk's one (six short turns) included;
  - on the tmpfs behind the `target/e2e-home` link, gone at a reboot: the live home `viola-live-4043089/` with
    the two instance directories this chunk added (`icrrehearse`, `icrlive`) beside the five of the last chunk,
    and the pinned copy of this chunk's first build (`bin/0.1.0-b4659b98029d0f94/`); the earlier
    `viola-reverify-20261007T124408Z/`; and `$XDG_RUNTIME_DIR/vcomp/`;
  - under the implement sessions' scratch directories on `/tmp`: this chunk's rig directory `icr-rig/` (the sent
    texts, the journal, the row files) and the last chunk's;
  - yours to delete: `target/e2e-home.disk`, whole, and the eleven directories under `target/profraw-census/`.
    No chunk in flight reads either.
- **Setup:** the upgrade U48 ran at `fe4f47f`; the host leaf is `host-linux.md`.
- **Last failed command:** none.
