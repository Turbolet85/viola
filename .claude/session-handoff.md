# Session Handoff

**Last Updated:** 2026-10-10T23:17Z
**Branch:** build/viola-0.1.0 · 0 ahead of origin/build/viola-0.1.0 as read at this wrap (the wrap commit pushes after this file)
**Status:** clean
**Last Commit:** 2026-10-10-budget-governor — the budget gate: thresholds from config.json, budget-paused on an automated send only, lifted at the reset or by release --budget, and both instance checks moved ahead of any log

## Position
- Done: **2026-10-10-budget-governor** (53 complete, 0 pending, 0 gated; 16/53 verified; the chunk claimed no
  capability: `v1-35`, `v1-13` and `v1-45` are advanced and not proven). CI green on `c85961d`,
  `ci#38091081413`, 15/15, first attempt, no fix commit.
- Next entry: **Session links** (the head of the markerless tail) → `/andromeda-phase`.

## Work done
- The wrapper has a budget gate (`src/run/budget.rs`). It reads `budget.json` at start, at a `send` and at each
  `turn-ended`, and refuses an automated `send` as `budget-paused` (`five-hour` or `seven-day`, exit 11) while a
  window is at or over its threshold with a known reset ahead. `viola release <name> --budget` lifts the pause
  for that instance until the crossed window's reset. Thresholds are `config.json`'s `five_hour` and `seven_day`
  (defaults 90 and 85). The gate never polls and is fail-open.
- `viola revive`'s start arm and `viola hook statusline` now run their instance check before anything under the
  home is opened. A refused check writes no line on either arm.
- The wrap ran in two sessions: Phase 1, then this one from Phase 2 (`resume-point.md` in the run dir). The chunk
  was built in two implement runs.

## Drift resolved
- 76 detector proposals over six masters (design-system returned none): 76 applied, 0 rejected. Five more sites
  raised by the wrap, and four stale sentences found by the sweep. 13 sidecar entries: architecture 3,
  security-plan 2, test-plan 3, obs-plan 2, design-system 1, layout-templates 1, a11y-plan 1. Seven key files
  edited; the registry check reads clean.
- One halt, three questions, all answered by the operator (`inputs#I11`): the `budget.json` test-data rule is
  scoped (a reader's own unit cases may alter the writer's bytes); one playbook rule appended ("Held items landed
  on the operator's word", resting on the founder's standing rule of 2026-10-06 as the operator names it); the
  test owner on the route. Two more escalations were settled by the resume directions (`inputs#I10`).
- Leaves re-derived: CLAUDE.md (four lines), `rules/security.md`, `observability.md`, `events.md`, `testing.md`,
  `api.md`, `docs/conventions.md`, `commands.md`, `stack.md`, `security-summary.md`, `tests-summary.md`,
  `obs-summary.md`, `a11y-summary.md`, `services/viola.md`, `viola-state.md`, `viola-core.md`.
- Citation sweep: 0 re-pointed in the masters; one route citation re-pointed by hand at Phase 5.
- Record: `.andromeda/runs/2026-10-10T22-36-31-wrap/` (`fanout-results.md`, `cascade-dispositions.md`,
  `route-record.md`, `curation.md`).

## Notes
- **For the founder, new at this wrap: four items landed on the operator's word, none ratified by him.** Each is
  his to rule; an answer that differs comes as a revision or a new chunk. No card was confirmed for any.
  - **the wrapper refuses a driver on `budget.json`**, a file a hook arm writes that the trusted-file list does
    not hold. The masters record this boundary widening as NOT ratified, his word owed (security-plan
    §Authentication & Authorization; architecture §Occupied Resources);
  - **the `budget-paused` refusal rests on the `rate_limits` shape with no ledger row, no probe and no fixture.**
    That is outside his ruled limit "a shape no `send` relies on needs no probe". The row and its probe are owed
    on "Paste newline ledger row";
  - **a refused strict-modes check writes no line** on `revive`'s start arm and leaves no trace on the statusline
    arm. This sits against his direction of a structured log from every process (obs-plan §10, exemption 7). What
    an operator sees of a refused statusline check: the session's status line row stays empty, `budget.json`'s
    `read_at` stops moving, and `viola revive <name> --list` prints the state-files pair for the same home;
  - **two human lines no master worded** landed in planning forms: `<name>  budget gate open  wheel <holder>`,
    and the lifted line's end as `YYYY-MM-DD HH:MM:SSZ` when it falls on another UTC date. His wording is owed.
  - Beside them: the gate is fail-open (no reading, an `unknown` figure, a passed or unparseable `resets_at`, an
    unreadable or refused `budget.json`, a bad threshold), so a drift in the CLI's `resets_at` shape opens the
    gate for that window; a snapshot can read `budget_paused: true` after the reset until a `send` arrives or a
    turn ends; an override does not outlive its wrapper; the reading's `used_percentage` is still an `f64`, the
    thresholds are `Percent` (the carried line "one line if he wants the newtype" applies to the reading);
  - closed by this chunk: the defect in shipped code on `viola revive` (its log before its check), and the
    hypothesis beside it, measured true and fixed.
- **Route, this wrap:** six CARRYs, no entry added or moved, each placement in `route-record.md`: "Session links"
  (a case for `answer`, `wait`, `last` and `pause` under a closed gate; the operator's answer: a missing test is
  folded into the next chunk); "The board: viola list" (`boot --statusline-echo`, the harness-session Path 6,
  the two-instance leg, `list --json`'s budget fields); "Paste newline ledger row" (the `rate_limits` row and
  its probe, a sixth shape for his ruling on the `viola verify` child set); "MCP server for drivers" (the MCP
  form of the refusal); "Home and code-bearing file integrity" (the check goes before the role file's opening on
  every other verb; fourteen blocks there now); "Exit-cause code catalogue" (revive's strict-modes refusal has no
  logged detail).
- **Not measured:** the real `claude` against the gate; an account without `rate_limits`; the `spend_limit`
  window; the two check-order fixes on Windows (both witnesses are `cfg(unix)`); a home another user owns, or
  one with only an other-writable bit; a widened DACL on Windows; override expiry across processes and two
  instances in one home (each a unit case only); any mutation score; which `cli_revive` and `hook_statusline`
  cases `windows-2025` does not build. No live `claude` start was made by this chunk, no rig home, no ledger row,
  no re-stamp.
- **For the founder (carried, with this wrap's changes):**
  - a `--home` session shows no user status line until `statusline-source.json` is planted in that home;
  - his own `~/.claude/settings.json` holds a `statusLine` of type `command` (presence and type read, nothing
    else, never written). On the default home a wrapped session runs it through viola's hook;
  - three relied-on dependences have no ledger row: the pasted LF, `claude --resume`, and the statusline's three
    shapes. Each owed row has its entry on the route;
  - `viola revive --list` opens no process log, so a panic there writes no line (obs-plan §10, the sixth
    exemption). It ends when `--list` gains `--json` on "CLI machine contract";
  - on the dev host the bare name `claude` was 2.1.289 and the home is stamped for 2.1.287 (as read at an earlier
    wrap, not re-read here), so a revive typed there with no `PATH` change runs an unstamped CLI;
  - "Paste newline ledger row" now holds six shapes for his ruling on the `viola verify` child set at its
    take-up, and needs a cap of live starts of its own; its phase sizes the entry and proposes a cut;
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
- **Epoch growth:** Epoch 4 stands at 10 entries (8 complete, 2 markerless: "Session links", "The board: viola
  list"). No boundary is minted inside it: the founder's word, 2026-10-09 (carried). Epoch 5 stands at 6.
- **Owed, no entry minted (the founder, 2026-10-07T09:43Z, carried):** `v1-34`'s harness-prefix row and its R8
  identity-floor row. A probed row for either needs its own founder ruling.
- **For the operator's word** (carried, with this wrap's figures):
  - "Interrupted verify cleanup" has no `requirements.md` line and no ledger entry; one line mints it;
  - `v1-33`'s title and `requirements.md:48` still say "on Windows"; no wrap step may edit either;
  - the ledger's dated notes on `v1-32`, `v1-40` and `v1-31` cite a bare `:90` for the entry "Windows-only live
    measurements"; none was rewritten (a ledger note is not a master);
  - two sidecars are over the 120 000 B whole-read bound: `architecture-amendments.md` (182 323 B) and
    `test-plan-amendments.md` (137 396 B), as measured after this wrap's entries. Phase reads them through their
    index;
  - seven sentences of this chunk's plan still word the old hold; its step 15 governs (the report, Deviations 6);
  - `inputs#I11`'s origin text carries a time typed without the clock ("23:4x"); the clock read 23:03:18Z;
  - a live `claude` session ending in this tree makes the session-end hook rewrite this file's last section.
- **Setup:** a setup re-run must keep `.claude/rules/ci.md` and `.claude/rules/testing-src.md`. The host leaf is
  `host-linux.md`. `rules/security.md` has no `paths:` and gained one bullet at this wrap (the budget gate).
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
  mutants (about 100 s each in the root package). The last four chunks added mutable code to the root package
  (a 998-line module among it at this one), `viola-state`, `viola-core` and `viola-agent-claude` and ran no
  mutation run; the epoch boundary's audit grades it.
- **Route:** `BLOCKED-ON` on "Windows-only live measurements" stands: `uname -s` read Linux at this wrap.
- **Curation:** three writes. `CLAUDE.md`: a missing test for a clause a chunk landed is folded into the next
  chunk. `testing.md`: the shared test visitor records an `f64` field as text. `ci.md`: the Windows-target lint is
  read on the host before the push. Log: the run dir's `curation.md`. The operator's memory on the stamp-ahead
  hook gained two cases. The conversations of the first implement run and of this wrap's first session were gone;
  their candidates were read from `resume-point.md`.
- **Deferred learnings** (none new under the cap; five recurred, then the carried list):
  - `recurrence-despite-learning: host-linux.md` Paths (a `cd` outside a subshell opened most Bash calls);
  - `recurrence-despite-learning: host-linux.md` Transports (one inline python heredoc for a leaf edit);
  - `recurrence-despite-learning: host-linux.md 2026-09-28/29` (a `cat` heredoc with a file target, once in the
    wrap's first session);
  - `recurrence-despite-learning: host-linux.md` Long single-line files (multi-KB master lines read through a
    scratch script; `cascade.py window` wraps its output and cannot feed the Edit tool an exact string);
  - `recurrence-despite-learning: a time written into a record ahead of or without the clock` (`inputs#I11`);
  - below the threshold at this wrap: `pre-push` prints no coverage figure (the numbers are in
    `target/agent-run/artifacts/llvm-cov-summary.json`, `data[0].totals`); the CI read outlives the Bash tool's
    foreground bound and goes in the background; a foreground `sleep` is refused by the harness;
  - `recurrence-despite-learning: host-linux.md` Exit codes (one gate call's output read through a line filter);
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
  - from this chunk (none committed, none removed): nothing new under `target/` but `target/wincheck/`, used
    again for the Windows-target lint, and the pre-push run's artifacts under `target/agent-run/artifacts/`;
    throwaway scripts and three CI test-job logs in the two wrap sessions' scratchpads; nothing depends on them;
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
Completed normally at 2026-10-11 02:03:10
