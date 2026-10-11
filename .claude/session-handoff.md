# Session Handoff

**Last Updated:** 2026-10-11T00:59Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap's Setup (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-10-session-links — the implicit session link: two event kinds, a link recorded on a send or an answer from another instance, the link set restored from the log, the replay's link half; `viola link` and `viola unlink` held for the founder's word

## Position
- Done: **2026-10-10-session-links**, Part A of its plan (54 complete, 0 pending, 0 gated; the chunk claimed no
  capability: `v1-36` and `v1-13` are advanced and not proven). CI green on `3cae2ce`, `ci#38098238731`, 15/15,
  first attempt, no fix commit.
- Next entry: **The board: viola list** (the last markerless entry of Epoch 4) → `/andromeda-phase`.

## Work done
- The driven wrapper records a link `driver → driven` when a `send` is issued or an `answer` is taken from another
  instance (`src/run/links.rs`, one `LinkSet` per wrapper). It appends one `link` line (`source: wrapper`) per
  new pair, directly before that send's `send-issued`, and mirrors the set to the snapshot's `links`. The set is
  restored from the log at every start, and the replay derives it too (`viola_state::replay::apply_link_line`).
  `EventKind` holds 15 kinds.
- **Part B is not built.** `viola link`, `viola unlink` and their two channel methods are held by the plan's
  step 11. The wrapper answers `link` and `unlink` `-32601`. No product path appends an `unlink` line.
- The test the last wrap folded into this chunk landed:
  `path6_a_closed_gate_leaves_answer_wait_last_and_pause_alone` (`tests/cli_budget.rs`).
- The wrap ran in two sessions: Setup and Phase 1, then this one from Phase 2 (`resume-point.md` in the run dir).

## Drift resolved
- 27 detector proposals over five masters (design-system and layout-templates returned none), all applied, 0
  rejected. Two more amendments raised by the wrap: architecture's `link` / `unlink` row says neither is built,
  and test-plan §6 Scenario E3 gained an As landed bullet. Seven sidecar entries: architecture 2, security-plan
  1, test-plan 2, obs-plan 1, a11y-plan 1. Three key files edited; the registry check reads clean.
- No halt. One class escalated by rule ("Held items landed on the operator's word") and was settled by the
  operator's direction with the resume (`inputs#I6`, item 3). The plan's last expected amendment ("only when
  Part B lands") was not carried.
- No master says `link` or `unlink` is served: the sweep's `served` pattern, every row read.
- The one disproved claim (the plan's note that its `schema-check` entry reads the new cases' lines) is a plan
  note only; no master says it. It went to curation.
- Leaves re-derived: CLAUDE.md (two Modules lines), `rules/events.md`, `api.md`, `observability.md`,
  `docs/services/viola.md`, `viola-state.md`, `viola-core.md`, `tests-summary.md`, `a11y-summary.md`,
  `obs-summary.md`.
- Citation sweep: 0 re-pointed in the masters; one route citation re-pointed by hand at Phase 5.
- Record: `.andromeda/runs/2026-10-11T00-33-08-wrap/` (`fanout-results.md`, `cascade-dispositions.md`,
  `route-record.md`, `curation.md`).

## Notes
- **For the founder, new at this wrap:**
  - **the explicit `viola link` / `viola unlink` wait for his own word on a boundary widening.** It crosses the
    local trust boundary in three places: the wrapper serving two more channel methods; the two verbs writing
    their frame after the liveness-only pre-check; their snapshot read without the strict-modes check. His three
    options, as the plan's step 11 words them: the two methods served and the two verbs under a dated gap of
    their own until the Epoch 6 entries; the instance check ahead of the snapshot read, as `viola revive` takes
    it; no explicit verbs before Epoch 6. The route entry "The board: viola list" carries the hold. Nothing of
    it is built, and nothing was ratified;
  - **five shapes of the implicit link landed on the operator's word, none ruled by him** (architecture [Links]
    marks each): a refused `send` or `answer` records no link; a `from` equal to the wrapper's own name links
    nothing; a `link` line's `source` is `wrapper`; the record sits directly before `send-issued`, so an issued
    send that then reads `not-delivered` has linked; a `link` line with no string `ts` reads `since: "unknown"`
    (no writer of this build makes such a line). An answer of his that differs comes as a revision or a new
    chunk.
- **For the founder, from the last wrap (carried, none ratified by him):**
  - the wrapper refuses a driver on `budget.json`, a file a hook arm writes that the trusted-file list does not
    hold. The masters record that widening as NOT ratified;
  - the `budget-paused` refusal rests on the `rate_limits` shape with no ledger row, no probe and no fixture. The
    row and its probe are owed on "Paste newline ledger row";
  - a refused strict-modes check writes no line on `revive`'s start arm and leaves no trace on the statusline arm
    (obs-plan §10, exemption 7);
  - two human lines no master worded landed in planning forms: `<name>  budget gate open  wheel <holder>`, and
    the lifted line's end with its date when it falls on another UTC date;
  - beside them: the budget gate is fail-open; a snapshot can read `budget_paused: true` after the reset until a
    `send` arrives or a turn ends; an override does not outlive its wrapper; the reading's `used_percentage` is
    still an `f64`.
- **Route, this wrap:** one CARRY, on "The board: viola list" (steps 12 to 15 of this chunk's plan, held, by
  the operator's direction `inputs#I6`). No entry added or moved. The entry holds eleven CARRY blocks. The two
  CARRYs this chunk's entry carried are discharged (`route-record.md`).
- **Not measured by this chunk:** any mutation score; anything against the real `claude`; anything of Part B;
  the link code under two wrappers of one home (each case boots one `builder`); a log holding an `unlink`
  written by a product path (none exists); `viola revive` over a log with links on a live session (the cases
  restart through `run`); where the link set's restore stands against the start's strict-modes check (the
  report states no order, and no master claims one). No live `claude` start, no rig home, no ledger row, no
  re-stamp.
- **Not measured, from the last wrap (carried):** the real `claude` against the budget gate; an account without
  `rate_limits`; the `spend_limit` window; the two check-order fixes on Windows; a home another user owns, or one
  with only an other-writable bit; a widened DACL on Windows; override expiry across processes and two instances
  in one home (each a unit case only).
- **For the founder (carried):**
  - a `--home` session shows no user status line until `statusline-source.json` is planted in that home;
  - his own `~/.claude/settings.json` holds a `statusLine` of type `command` (presence and type read, nothing
    else, never written). On the default home a wrapped session runs it through viola's hook;
  - three relied-on dependences have no ledger row: the pasted LF, `claude --resume`, and the statusline's three
    shapes. Each owed row has its entry on the route;
  - `viola revive --list` opens no process log, so a panic there writes no line (obs-plan §10, the sixth
    exemption). It ends when `--list` gains `--json` on "CLI machine contract";
  - on the dev host the bare name `claude` was 2.1.289 and the home is stamped for 2.1.287 (as read at an earlier
    wrap, not re-read here), so a revive typed there with no `PATH` change runs an unstamped CLI;
  - "Paste newline ledger row" holds six shapes for his ruling on the `viola verify` child set at its take-up,
    and needs a cap of live starts of its own; its phase sizes the entry and proposes a cut;
  - the CARRY on "Server verification before any frame" names a boundary matter: what a client's pre-check does
    with an unreadable or newer snapshot needs his rule before that entry is planned;
  - a listed local command followed by CR or CRLF classifies as that command; not shown to him yet;
  - the switch from the prototype is not made; `v1-33` is unclaimed, pinned on "The board: viola list";
  - not measured (earlier chunks): macOS and Windows live for the statusline; a user-scope status line; which
    of two `--settings` flags wins; how the CLI cancels a running statusline script; the 5 s bound on a real
    session; any CLI version but 2.1.287; a long or wrapped multi-line text; an LF inside a paste on Windows;
    the wheel's return by a human `release` on a live session; `--fork` on the real CLI; a resume of a killed
    session on the real CLI; whether a resumed session held its earlier turns; the bounded removal's loop on
    Windows;
  - one CLI plan file is left in his user directory (2026-10-08). viola wrote nothing there.
- **Epoch growth:** Epoch 4 stands at 10 entries (9 complete, 1 markerless: "The board: viola list"). No boundary
  is minted inside it: the founder's word, 2026-10-09 (carried). Its last entry closes it, and that wrap runs the
  sidecar consolidation. Epoch 5 stands at 6.
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s harness-prefix row and its R8
  identity-floor row. A probed row for either needs its own founder ruling.
- **For the operator's word** (new, then carried):
  - the link rule takes a logged `driver` / `driven` as a string and does not parse it as a name again
    (security-plan, row "Own state files on read", states it; the wrapper wrote both names itself). The
    security detector raised it for a human's judgment. Whether it is work for an Epoch 6 entry is not decided,
    and no CARRY was minted;
  - `docs/conventions.md` and `docs/stack.md` still list `viola link|unlink` and the `link · unlink` subcommands,
    as architecture §Conventions and §Stack do: design lists, no amendment was made to either;
  - "Interrupted verify cleanup" has no `requirements.md` line and no ledger entry; one line mints it;
  - `v1-33`'s title and `requirements.md:48` still say "on Windows"; no wrap step may edit either;
  - the ledger's dated notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90` for the entry "Windows-only live
    measurements"; none was rewritten (a ledger note is not a master);
  - two sidecars are over the 120 000 B whole-read bound: `architecture-amendments.md` (186 843 B) and
    `test-plan-amendments.md` (140 677 B), as measured after this wrap's entries. Phase reads them through their
    index;
  - seven sentences of the budget-governor chunk's plan still word the old hold; its step 15 governs;
  - a live `claude` session ending in this tree makes the session-end hook rewrite this file's last section.
- **Setup:** a setup re-run must keep `.claude/rules/ci.md` and `.claude/rules/testing-src.md`. The host leaf is
  `host-linux.md`. `rules/security.md` has no `paths:`; it was read and not changed at this wrap.
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
  mutants (about 100 s each in the root package). The last five chunks added mutable code and ran no mutation
  run: this one to the root package (`src/run/links.rs`, 105 product lines, and blocks in `send.rs`,
  `dialog.rs`, `cmd/run.rs`), `viola-state` (`apply_link_line`) and `viola-core`; the epoch boundary's audit
  grades it.
- **Route:** `BLOCKED-ON` on "Windows-only live measurements" stands: `uname -s` read Linux at this wrap.
- **Curation:** two writes, both Tier 2. `verification-harness.md`: `schema-check` on the dev host reads kept
  homes only. `testing.md`: `clippy::cloned_ref_to_slice_refs` on a one-element array in `assert_eq!`. Log: the
  run dir's `curation.md`. The conversation of implement and of this wrap's first session was gone; its
  candidates were read from `resume-point.md`.
- **Deferred learnings** (none new under the cap; five recurred, then the carried list):
  - `recurrence-despite-learning: host-linux.md` Transports (one inline python heredoc for a source edit at
    implement; inline python heredocs for multi-line reads in this session);
  - `recurrence-despite-learning: a time written into a record ahead of or without the clock` (caught before it
    was pasted, at Phase 1);
  - `recurrence-despite-learning: host-linux.md` Paths (one `cd` at the head of a compound, this session);
  - `recurrence-despite-learning: host-linux.md` Exit codes (one verdict tool read through a line filter);
  - `recurrence-despite-learning: host-linux.md` Long single-line files (multi-KB master lines read through a
    scratch script; `cascade.py window` wraps its output and cannot feed the Edit tool an exact string);
  - below the threshold at this wrap: a plan step that puts an operation in the unheld part while its only caller
    sits behind the stop makes dead code under `-D warnings` (the route carries the fact); a plan's forced-on
    edit must name a handle the target holds; restore a scratch edit from a kept copy and read SHA-256 back,
    never by a reverse Edit; a harness `run` document names its archive; `gate.py show --n` tells a designed red
    from a real one; the gate tool prints nothing until its block ends;
  - carried, below the threshold earlier: `pre-push` prints no coverage figure (the numbers are in
    `target/agent-run/artifacts/llvm-cov-summary.json`, `data[0].totals`); the CI read outlives the Bash tool's
    foreground bound and goes in the background; a foreground `sleep` is refused by the harness;
  - `recurrence-despite-learning: ci.md 2026-10-09` (`gh run list --commit` given a wrong sha);
  - `recurrence-despite-learning: testing.md 2026-10-05` (nextest's padded duration broke a fixed-column split);
  - `recurrence-despite-learning: testing.md` Test data (tests read files after the `TestHome` was dropped);
  - the key files stand under `.andromeda/registries/contracts/<master>/`, so an owner map keyed on the
    registry's top-level names reads every key file as unowned (cap);
  - a wrapper a test ends through its drop guard writes no coverage profile, so a case whose start path must
    count toward coverage stops its wrapper cleanly (cap);
  - a mutation run of `viola-e2e` leaves session homes on the shared test-home base, because the tool's copy of
    the tree carries the `target/e2e-home` link (cap);
  - a window-class prefix such as `viola.` also matches the operator's own desktop windows; match a chunk's
    classes whole (cap);
  - a report names the route owner beside a claim a route CARRY already owns (cap);
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
  - from this chunk (none committed, none removed): scratch files in the two wrap sessions' and the implement
    session's scratchpads (the kept copies of `send.rs` and `dialog.rs` from the forced-on runs, run logs,
    scripts, two sweep listings); the gate logs under the OS temp dir's `andromeda-gate/2026-10-10-session-links/`;
    `target/wincheck/`, used again; `target/run-archive/` entries of this session's harness runs;
    `target/agent-run/artifacts/`. Six `viola-test-*` homes dated 2026-10-08 stand on the test-home base and are
    earlier chunks'. Nothing depends on any of them;
  - from the statusline chunk: `target/e2e-home/viola-live-sl-20261010/` on the tmpfs (three instance
    directories `slreh`, `slsrc`, `slbare`, its `budget.json`); `target/sl-live-20261010/` with `a` and `b`; the
    rig's private directory `sl-rig/` named in that chunk's `evidence/live-statusline.md`; the CLI's own files
    for its three sessions;
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
Completed normally at 2026-10-11 09:38:44
