# Report — 0-pending wrap at the Epoch 4 boundary (Phase 1 of two)

**Chunk:** none. Master 55 records · 55 complete · 0 pending · 0 gated (`route.py cursor`, and the anchored grep
`^[^ ]+ · (pending|gated) · ` over `.andromeda/master-route.md` exit 1). No chunk is wrapped; no `gated` premise to
re-verify.
**Date:** 2026-10-13T06:38:35Z (this run dir's name)
**Commits:** none since `last_wrap` 2026-10-11T08:52:06Z. HEAD `dffad3e` is that wrap's own commit
(2026-10-11T08:55:14Z). 0 ahead of `origin/build/viola-0.1.0`, branch `build/viola-0.1.0`.
**Path:** Setup step 6. The tree at entry is dirty only with the expected-transient set
(`session-handoff.md`, `friction-log.ndjson`, `code-metrics.ndjson`, the last wrap's
`evolve-2026-10-11-the-board-viola-list.json`) and two untracked boundary run dirs. By the letter that reads as
git-clean, so the no-op family applies, and this wrap's commit carries all of it.
**Form:** split on the operator's word at invocation: Phase 1 only, then stop. The reconcile runs in a fresh window.
That window resumes from this run dir. There is no separate resume file (directive, header).
**Marker:** `no-marker`, as the step-6 family uses (trails `cascade-no-marker-pre-pass.json`).

## Changes (structured — detectors read this)
- **Files:** no source file changed. At entry the tree holds `.andromeda/code-metrics.ndjson` +1 line (the Epoch 4
  audit's record, `ts 2026-10-12T21:23:40Z`), `.andromeda/friction-log.ndjson` +3 records plus this wrap's own appends,
  the last wrap's `evolve-2026-10-11-the-board-viola-list.json` (+40), `.claude/session-handoff.md` (+11, an
  automatic `Session End Status` line and two unattended auto-compact notes), untracked
  `.andromeda/runs/2026-10-12T06-42-44-evolve-diagnose/` and `.andromeda/runs/2026-10-12T06-54-29-code-audit/`
  (basis: `git status --short`, `git diff --stat` at Setup).
- **Symbols / APIs:** none.
- **Crates / modules:** none.
- **Dependencies:** none.
- **Schema / config:** none.
- **Spec-master edits:** none in this window. The ones owed to window 2 are listed under *The directive's items* below.
- **Counts / qualifiers moved:** none — verified (no chunk, no source delta).
- **Dev-tool versions:** none.
- **Harness / gate surface:** none.
- **Cross-project / external claims:**
  - I1 · the operator's directive `boundary-wrap-e4-directive.md` (the overseer's workspace,
    `~/dev/projects/additional/viola-overseer/`, not a git repository) · copy · read 2026-10-13T06:38Z, file mtime
    2026-10-13T06:13:45Z · 72 lines, 6822 B, sha256 `42a40aff…0e634267220`. Copied byte for byte (`cp -p`) to this run
    dir as `relay-1.md`; the copy's sha256 is identical. `inputs.py snap` was NOT fired. Its `--chunk-dir` takes only
    `{version}/chunks/{marker}/` (`inputs.py:137`), and a 0-pending wrap has no chunk folder. Snapping into one would
    mint a chunk folder for no chunk. The copy in the run dir is the snapshot, following the Epoch 3 boundary wrap's
    form (`2026-10-09T14-44-30-wrap/adaptation-record.md`, its **Input** line). Window 2 cites it as `relay-1.md`.
  - The directive's header still reads "DRAFT until the founder's batch is in". Its §4 holds the batch (read 05:59Z),
    and the operator's invocation is the sending.
  - §4's founder answers reach this wrap relayed. They are his own answers by dialog, the option labels he chose
    quoted verbatim in Russian, relayed by the Viola overseer through the operator's directive. They are not a
    delegate's word: the directive presents them as his.
  - CI run `38215104120` (the Windows mutation leg's red baseline): read by the audit on 2026-10-12
    (`code-audit/c-windows-readback.json`, its `read_back`), not re-read here.
- **Reverted / negative API facts:** none.
- **Insufficient fixes:** none.
- **Spec claims disproved by measurement:**
  - The directive's §4.3 cites `architecture.md:407` for the `budget.json` "not ratified" mark. That mark stands at
    `architecture.md:408`; line 407 is the `instances/<ViolaName>/…` bullet. Basis: the `notratified` row below, and
    `sed -n 407p` / `408p`. Every other coordinate the directive cites read true at HEAD: `security-plan-amendments.md:509`
    and `:519` (each a `**Why:**` line holding "the operator confirmed the card at this wrap"), `playbook.md:60-62` (the
    pattern, verdict and note of "Boundary widening the founder already answered"), `security-plan.md:213` and `:241`,
    `working-route.md:142` (Explicit session links), `:145` (Windows endpoint admission), `:147` (Server verification
    before any frame), `src/cmd/hook/statusline.rs:510` (`assert!(trusted(&instance));` in
    `statusline_an_instance_viola_laid_down_passes_its_check`).
- **Expected amendments (from plan):** none listed (no plan). The directive's items stand in that place, below.
- **Coverage of new surfaces:** none.

### The site sweep (basis for every master site named below)
`cascade.py sweep --pre-pass --patterns-file {run_dir}/report-patterns.toml --run-dir {run_dir} --marker no-marker`,
baseline `dffad3e0`. The trail is `cascade-no-marker-pre-pass.json`.
- `cardconfirm` (`confirmed the card`, case-insensitive): refused, `control: cardconfirm never fired over the seven
  masters' pre-pass text (dffad3e0)`. No master holds the words. The two sites are in the sidecar, which the sweep does
  not read, and were read by `sed -n 505,522p`.
- `notratified` (`not ratified`, case-insensitive): standing 4 · leaf 2 · base 1 — rows below.
- `opword` (`operator(’|')s word`, case-insensitive): standing 12 rows over 6 masters · leaf 3 · curation 1 · base 4 —
  rows below.
- `provisional` (`PROVISIONAL`): standing 8 · leaf 6. Every row is a deadline constant (`DIALOG_DEADLINE` 60 s,
  `STATUSLINE_DEADLINE` 5 s), so none is a mark of the founder's batch. In the masters, the batch's marks are worded
  "the operator's word" / "not ruled" / "not ratified", never "PROVISIONAL".

| site | pattern | what it marks | the batch item | disposition owed (window 2) |
|---|---|---|---|---|
| `architecture.md:408` | notratified, opword | the wrapper refusing a driver on `budget.json` | §4.3 «Ратифицировать» | rewrite as ratified, his word 2026-10-13, relay named |
| `security-plan.md:213` | notratified, opword | the same widening (`~/.viola/` access control) | §4.3 | the same |
| `security-plan.md:241` | notratified | the same (Own state files on read) | §4.3 | the same |
| `architecture.md:71` | notratified, opword | the explicit `link`/`unlink` hold (not ratified) and five implicit-link shapes on the operator's word | §4.4 for the hold; the implicit-link shapes are in no batch item | the hold stays recorded as unratified (§4.4: "stay unbuilt and stay recorded as such"); the five implicit-link shapes stand |
| `architecture.md:247` | opword | a `link` line with no string `ts` gets `since: "unknown"` | none (an implicit-link shape) | stands |
| `test-plan.md:552` | opword | the same `since: "unknown"` shape | none | stands |
| `architecture.md:471` | opword | the `budget-paused` refusal resting on `rate_limits` with no row, "the founder has not ruled on it" | §4.2 «Строку к Paste newline ledger row» | rewrite: his ruling, the build stays and the row and probe are owed on "Paste newline ledger row" |
| `design-system.md:749` | opword | WHEEL padded to 23; the five board forms | §4.1 (a), (b) | rewrite as his |
| `layout-templates.md:397` | opword | the five board forms | §4.1 (a) | rewrite as his |
| `design-system.md:805` | opword | the budget-governor date form and the `budget gate open` line | §4.1 (e), item 4 ("the two human lines no master words") | rewrite as his. **Read for window 2:** the batch ratified the lines, but whether that counts as "worded by him" (the masters say "his wording is owed") is the operator's to confirm |
| `layout-templates.md:471` | opword | the same two lines | §4.1 (e) | as `design-system.md:805` |
| `obs-plan.md:394` | opword | three refused-check readings that write no line: `hook statusline`'s instance check · the checked read of `budget.json` (`read_budget_stamped`) · `viola list`'s scan | the first by §4.1 (e) item 3; the third by §4.1 (a) ("no log opened on a refusal"); **the middle one by no batch item as worded** | rewrite the two covered as his; the `budget.json` read's no-line shape is the operator's question for window 2, never folded in by reading |
| `test-plan.md:1076` | opword | the revive fake-agent `SessionStart` payload shape (the operator's word at the revive P4) | none | stands |
| `.claude/rules/security.md:30`, `.claude/docs/security-summary.md:69` | notratified, opword (leaf) | the `budget.json` widening | §4.3 | re-derived by the cascade |
| `.claude/rules/ci.md:18` | opword (leaf) | the mutation workflow's dispatch on the operator's word | none | stands |
| `.claude/rules/testing.md:87` | opword (curation) | a PID-1 note | none | stands |
| `.andromeda/playbook.md:58, 62, 64, 66` | opword, notratified (base) | the rules' own provenance and text | §3 touches `:60-62` | only through the supersede door (§3) |

The listing reaches these patterns' wording and no further. A site stating a batch item in other words (for example
"not worded by the founder" with no "operator's word" beside it) is not in it. Window 2's detector-side read, and the
sidecars, which the sweep does not read, remain the check on how far it reaches.

## The directive's items, read and placed (each owned by a phase of window 2)
- **§1 the cascade's whole mode** (founder-ruled, F252 D2): `cascade.py leaves --all` in P2's cascade step 3, not the
  pass mode. This is the first wrap since `leaves` deployed. Owner: P2.
- **§2 two sidecar entries record a card confirm as ratifying** (F245): `security-plan-amendments.md:509` (the
  statusline override's `**Why:**`) and `:519` (the revive session id's `**Why:**`), both read. Correct each to the
  answer-card form: the operator's confirm is PROVISIONAL (`delegate viola-overseer`), superseded by the founder's own
  word, §4.1 (d) of 2026-10-13. Owner: P2. The form of a correction to a sidecar entry (in place, or a new entry
  naming the old) is P2's to settle by `sidecar-contract.md`.
- **§3 the playbook rule is broader than his ruling:** `playbook.md:60-62`. Through the supersede door: append a new
  rule keyed on the founder's OWN answer to a fork P4 put, at P4 or later, given by him or relayed verbatim, never a
  delegate's. Keep the old rule verbatim, its `note:` opening `SUPERSEDED 2026-10-13 (by the rule {name}) —`. Owner:
  P2 (an approved playbook append on the operator's word).
- **§4.1 the batch «Подтвердить всё»:** (a) the board's shapes · (b) WHEEL 23 and the exit-1 paths of an unlistable
  `instances/` · (c) the overseer's placement of the three new Epoch 5 entries · (d) the F245 card of the
  statusline-pass-through wrap (W1, W2, W3, the revive session id) · (e) Budget governor's items 3 and 4. Each mark
  becomes his, dated 2026-10-13, the relay named (the Viola overseer), with the provisional record kept as
  `superseded 2026-10-13`. Master sites are in the table above. Sidecar and route sites (the CARRYs naming
  `inputs#I7`, `inputs#I9` on `working-route.md:130`, `:132` and `:142`) and the handoff's "for the founder" lists
  are read at their phases. Owners: P2 (masters, sidecars), P5 (route CARRYs), P6 (handoff).
- **§4.2 `rate_limits`:** the build stays as it is. "Paste newline ledger row" (`working-route.md:128`) already holds
  the CARRY moved from "Budget governor" ("the `rate_limits` ledger row and its `viola verify` probe are owed here … that
  reliance landed on the operator's word and the founder has not ruled on it"). Annotate that CARRY with his ruling.
  Owners: P5, plus P2 for `architecture.md:471`.
- **§4.3 the `budget.json` widening «Ратифицировать»:** `security-plan.md:213`, `:241`, `architecture.md:408` (not
  `:407`), the two leaves through the cascade, and `.claude/rules/security.md:30`'s sentence ("NOT ratified (landed on
  the operator's word, the founder's word owed)"). Owner: P2.
- **§4.4 «Не делать до Эпохи 6»:** move "Explicit session links" (`:142`, last in Epoch 5) into Epoch 6, after
  "Windows endpoint admission" (`:145`) and "Server verification before any frame" (`:147`). Its CARRYs move with it
  (E3's steps 1 and 3 are in its first CARRY, read at `:142`). `v1-13`'s link half moves too: `v1-13` reads
  `chunk: null`, `planned`, its notes naming the `link` half (`verification-matrix.json`, read). That is a dated
  ledger note at P7.3's carve-out (`matrix.py note`), never an acceptance change. `architecture.md:71`'s hold stays
  unratified. Owners: P5 (the route move), the step-6 commit (the ledger note, `--marker no-marker`).
- **§5 route adaptation, one corrective entry:** recommended: "Epoch 4 cleanup", the head of Epoch 5, before
  "Interrupted verify cleanup" (`:126`); only the anchor is pinned. Its four items, with the basis read here:
  1. the `viola` survivors, 32: `typed.rs` 18 · `wheel.rs` 9 · `dialog.rs` 2 · `hook.rs` 1 · `cmd/send.rs` 1 ·
     `run/send.rs` 1 (`code-audit/proposals.md:73`). Other units 15: state 10 · agent-claude 2 · e2e 2 · pty 1
     (`:72`). No mutation run, witness run or workflow dispatch inside the chunk (the founder's standing direction).
  2. the Windows mutation leg's red baseline: run `38215104120` (`c-windows-readback.json`), the unmutated
     `statusline_an_instance_viola_laid_down_passes_its_check` failing at `statusline.rs:510` inside the cargo-mutants
     copy on windows-2025, green in CI on `acad7af`.
  3. split `src/run/send.rs` (2088 code lines, 1464 at the baseline) and `src/human.rs` (1064, 509 at the baseline)
     (`proposals.md:17`, `:22`). `ledger.rs` is not split (2709, +31).
  4. a check-only cross-target clippy for `x86_64-pc-windows-msvc` as a gate entry (the hand run at
     2026-10-10T22:01:41Z-a, a ledger id, not read here).
  Owner: P5's trajectory dialogue, recommended-first, item by item.
- **§6 curation candidates:** under *Decisions & corrections*. Owner: P3.
- **The boundary's run dirs and ledger line:** the diagnosis (`proposals.md` titled `viola-0.1.0 · Epoch 4 — Session
  state & governance`), the code audit (`proposals.md` and `record.json`, with `resume-note.md` from its stopped first
  window) and the `code-metrics.ndjson` line ride this wrap's commit. At the Epoch 3 boundary, hygiene was held on the
  operator's word for the audit's evidence. Whether that holds again is window 2's question at P7.3c.

## Deviations from intent
- No `inputs.py snap` (see I1): the tool has no 0-pending form, so the directive is a copy in this run dir.
- No new-text listing (`cites.py added`): no chunk diff exists to list. No `gate.py scope`: no chunk folder.
- No code-graph refresh fired: `.andromeda/cache/tree.db.commit` reads `dffad3e0…` = HEAD, and no source moved.
- scope record: none — no chunk.

## Decisions & corrections
- **The founder-rulings memory, drained here (directive §6, item 1).** A planned action or master amendment that
  departs from a founder ruling is never a decisive lean and never a card confirm item. Plan up to it, hold it behind
  a stop rule naming what counts as his word, and raise it at P4 as a fork. Evidence: the auto-memory
  `founder-rulings-stand-until-his-word` (2026-10-10, the windows-mutation-grade P5 review; the operator's words quoted
  there: "the founder ruling that the mutation workflow is dispatched only at the epoch boundary stands until his own
  word"). "Drained at this boundary" is the operator's word: once P3 writes its project home, the memory file and its
  `MEMORY.md` line are removed.
- **A root case booting a home starts from a stamped one (directive §6, item 2).** On Windows, `seed_conpty` leaves an
  unstamped home default-permissioned, and the strict-modes check refuses it. Evidence: the board chunk's CI red
  `ci#38123340339` (four `list` cases on `windows-2025`), fixed by `acad7af`. The handoff carries it as a deferred
  learning, `recurrence-despite-learning: test-plan / tests-summary`. The same shape came before it in the
  statusline-pass-through fix `67ab367`.
- **test-plan: a root test's home is removed at its end, and a schema check reads only kept homes (directive §6,
  item 3).** The directive gives no evidence beside it. P3 reads its basis or asks.
- **The answer-card rule applied to recorded words (directive §2).** A confirm that is not the founder's own word is
  PROVISIONAL (`delegate viola-overseer`), superseded by his own word in the epoch batch. Two sidecar entries were
  worded as if the confirm settled it. This is a correction of record (P2), and a curation candidate only where no
  letter or rule already holds it (`amendment-flow.md` §Escalate holds it).
- **A playbook rule written broader than the ruling it records (directive §3).** It was keyed on "a word given at
  planning or at implement" where the ruling keys on his own answer to a P4 fork. Corrected through the supersede door
  (P2). A candidate for how a rule records a ruling: the rule's key is the ruling's own wording.
- **Sweep hazard, measured this window:** the batch's provisional marks are not worded "PROVISIONAL" in the masters.
  `PROVISIONAL` hits only the two deadline constants (8 master rows, 6 leaf rows), while the marks read "the
  operator's word", "not ruled", "not worded by the founder", "not ratified". A sweep for the batch on the token
  `PROVISIONAL` reads every real mark as absent.
- **Sweep hazard, measured this window:** `confirmed the card` is in no master; the pre-pass refused its pattern
  (control unfired). Sidecar wording is outside `cascade.py sweep`'s reach, so a sidecar site is read by its own
  command (`sed -n` at the cited lines).
- **A cited coordinate off by one in the operator's directive:** `architecture.md:407` → `:408`. A directive's line
  citation is read at the line before it is acted on.
- **A decision of this window:** with no chunk folder, the external input went into the run dir as a verbatim copy
  with its sha256, not through `inputs.py`. That is the Epoch 3 boundary wrap's precedent; the letter names no
  0-pending form of the inputs channel.
- **Guard refusal (this session's dashboard, before the wrap):** the Bash guard refused a `cd` into `.andromeda/`.
  Re-issued once with `git -C` and absolute paths, and it ran.
- **Carried from the handoff's deferred learnings, for P3 to weigh:** `recurrence-despite-learning`: a time written
  into a record ahead of the clock (four times in implement); the `~/.claude/CLAUDE.md` Shell bounded repetition; the
  `host-linux.md` Transports inline python heredoc; plus the below-threshold list there.

## Outcome
- No acceptance criteria: no chunk.
- Gates: none. A 0-pending wrap has no plan `## Test Commands` and runs no light gate.
- Coverage: `matrix.py coverage` reads verified 16/53 · deferred 0 · unclaimed 37 (this session's dashboard read).
  Window 2's `v1-13` note changes no status.
- Outcome basis: the directive (I1), the boundary run dirs as read above, and HEAD `dffad3e`. No implement run exists.
- Process hygiene: no process started by this window beyond the read-only tools.
- **Resume (window 2):** `/andromeda-wrap-session`, 0-pending, at this run dir `.andromeda/runs/2026-10-13T06-38-35-wrap/`.
  It reads this report and `relay-1.md`, then runs P2 (the citation sweep; the applies of §2, §3, §4.1, §4.2's master
  half and §4.3; the cascade with `leaves --all`; the sidecar entries), P3 (curation, including the memory drain), P5
  (§5's dialogue, §4.2's annotation, §4.4's move, the §4.1 (c) route marks), P6, then the step-6 commit and push. The
  open questions for the operator are marked **Read for window 2** in the table (`design-system.md:805` /
  `layout-templates.md:471`, and `obs-plan.md:394`'s `budget.json` read), plus the commit message of a wrap that does
  more than one step-6 door.
