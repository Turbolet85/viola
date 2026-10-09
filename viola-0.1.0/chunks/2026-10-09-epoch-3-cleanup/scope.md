# Scope — 2026-10-09-epoch-3-cleanup · Epoch 3 cleanup

**Working entry** (`viola-0.1.0/working-route.md:105`, head of `### Epoch 4 — Session state & governance`):
Epoch 3 cleanup — test scaffolding shared once, three functions within the cognitive ceiling, viola-e2e scored,
eighteen Linux survivors disposed, trailing CR confirmed, newline-only send refused at once — plus five CARRY blocks
(all five folded below; completeness check `route.py pins` → the run dir's trail: five `:105` rows, 1534 · 1291 · 943 ·
1594 · 1252 chars, no abstention).

**The operator's direction at take-up** (`inputs#I1`, the invocation, verbatim in the snapshot):
- the founder's cap for this chunk is **5 live `claude` starts, 2.1.287 by path**, for the CRLF and several-CR readings;
  the plan stays inside it and every start is ledgered before it is made;
- the form of the `viola-e2e` kill is a technical fork, brought to the operator at P4 with a recommendation;
- one item is folded with its own acceptance: the two stale Epoch 6 citations (§8);
- the chunk is sized against one builder window; if its five CARRYs do not fit, a priced split card goes to the
  operator at P4. A split is the founder's word, so nothing leaves this scope before it.

**Split (P4, 2026-10-09 — the founder's word, live in the overseer's dialog at 15:27:46Z, the three cuts shown with their
prices; relayed in the operator's answer, `inputs#I3`).** The chunk is split in two.
- **Stays:** §4 (the `viola-e2e` kill, with the unit's baseline passing under the `mutants` profile), §6 (the trailing
  CR and CRLF strip and the empty-text refusal, with the live readings), §7 (the three rule homes and the gotchas
  line) and §8 (the stale citations).
- **Leaves:** §1 (M1), §2 (M2), §3 (M3), §5 (the eighteen Linux survivors) and the `viola-e2e` whole-unit score. They
  are **owed, not dropped**: this chunk's wrap (route-resolve) gives them their own markerless entry at the head, ahead
  of "Windows mutation grade", carrying CARRY 1 and CARRY 3 verbatim and the score's half of CARRY 2. Their sections
  below are kept as folded, for that entry.

## What this chunk builds

### 1. Audit M1 — test scaffolding shared once
**Leaves this chunk with the split (`inputs#I3`); owed to the entry this chunk's wrap mints.**
- Source: the Epoch 3 boundary audit, `.andromeda/runs/2026-10-08T10-08-51-code-audit/proposals.md` §M1, read at
  `e304994` (CARRY 1; placed at the 2026-10-09 0-pending wrap on the operator's answer, relay item 1).
- The measured movement: duplication 1.35 % → 3.13 % (clones 53 → 189; 1668 duplicated lines of 53261). By duplicated
  lines a file takes part in: `tests/cli_verify.rs` 363, `tests/cli_answer.rs` 336, `src/run/send.rs` 254,
  `tests/cli_send.rs` 198, `tests/cli_wheel.rs` 188, `tests/hook_fail_open.rs` 170, `tests/tui_wheel.rs` 150.
- The largest cross-file fragments are per-file copies of one scaffolding, and their shared home is `tests/support/`
  (present at P1: `fake.rs`, `home.rs`, `hygiene.rs`, `mod.rs`, `ndjson.rs`, `outer_pty.rs`, `piped.rs`, `verify.rs`,
  `watch.rs`):
  - a child-run-and-wait helper (`tests/cli_answer.rs:134` / `tests/cli_wait_last.rs:132`);
  - a wrapper boot that waits for `session-start`, with an `events()` reader (`tests/cli_answer.rs:56` /
    `tests/tui_wheel.rs:33` / `tests/cli_wheel.rs:36`).
- 104 of the 189 pairs sit inside a single file; the audit names `tests/cli_verify.rs` and `src/run/send.rs` as
  candidates for one helper each.
- Done is read on the audit's own instrument: the duplication scalar re-taken with the audit record's
  duplication command over the same file population, lower than 3.13 %, with the named cross-file fragments gone from
  its top list. The entry states a direction and no target number; P3 reads the record's command and P4 names the
  number or the form the criterion takes.
- Lifting shared scaffolding moves no assertion: every test that used a per-file copy asserts what it
  asserted before. The line coordinates above are the audit's at `e304994`; P3 re-reads them at HEAD.

### 2. Audit M2 — three functions within the cognitive ceiling
**Leaves this chunk with the split (`inputs#I3`); owed to the entry this chunk's wrap mints.**
- Source: the same audit §M2 (CARRY 1). Three functions entered over cognitive 15 this epoch, each re-read at P1 at the
  line the entry names:
  - `dialog_variants` 19 (`crates/viola-agent-claude/src/ledger.rs:874`);
  - `record` 17 (`src/cmd/verify.rs:388`);
  - `submit` 16 (`src/bin/viola-fake-agent.rs:452`).
- `sgr_attributes` 23 (`tests/cli_output_plain.rs:36`) stood over the ceiling at the Epoch 2b baseline too. The entry's
  title counts three, so it is named here and not one of the three. Its disposition (split with the others,
  or recorded as standing) is settled at P4 against the size of the chunk.
- The audit's direction: split each of the three where its branches already separate. No behaviour changes.
- Done is read on the audit's own instrument: `complexity.over_ceiling` re-taken with the audit record's
  complexity command, the three no longer over 15.

### 3. Audit M3 — both scalars rose at each of the last two boundaries
**Leaves this chunk with the split (`inputs#I3`); owed to the entry this chunk's wrap mints.**
- Source: the same audit §M3 (CARRY 1): `duplication.pct` 1.29 → 1.35 → 3.13; `complexity.over_ceiling` 0 → 1 → 4.
- No separate work: §1 and §2 are its subjects. Its own statement in this chunk is the two re-taken scalars, recorded
  beside the three earlier records.

### 4. Audit F1 — `viola-e2e` scored
- Source: the same audit §F1 (CARRY 2; relay item 2, placed on the operator's answer). `viola-e2e` is unscored for the
  second boundary running.
- The measured fact: `harness_lifecycle` `boot_with_an_unknown_cli_version_is_verify_failed`
  (`crates/viola-e2e/tests/harness_lifecycle.rs:265`, read at P1) passes in 34.477 s under profile `ci` and is killed
  under profile `mutants` on both hosts: at 30.003 s on the dev host (257 of 258 passed) and at 30.009 s on the
  `windows-2025` runner (run 37761947926, 249 of 250 passed, `mutants-exit-4`, its 68 mutants ungraded).
- Re-read at P1 in `.config/nextest.toml`: the `mutants` profile's `package(viola-e2e)` override (15 s × 2) is its first
  override; the `verify_window_` override (15 s × 3 = 45 s) comes third and matches a test name this test does not carry.
- **The fork (the operator's at P4, with a recommendation, per `inputs#I1`):** the test takes a kill above its designed
  wait, either by a name the `verify_window_` override matches or by an override of its own. The entry leaves the form
  to take-up. Under either form the override order matters: the first matching override wins.
- [premise-corrected: research.md §Scope premise closure — the mechanism is re-derived by read, the four durations
  are not measured] The entry's `hypothesis: its 34.5 s is four waits of the readiness gate's 8.5 s maximum (the
  audit's suspected shape, not measured)`. By read at HEAD: the test boots with `cli_version` `9.9.9`, no
  `fixtures/claude/9.9.9/` exists, so the fake agent draws no screen literal, and verify's `settled`
  (`src/cmd/verify/typed.rs:478-498`) ends each of its four interactive runs only at `GATE_MAX_WAIT` (8.5 s): at least
  34 s by construction, above the 30 s kill. The four waits' own durations in this test are still unmeasured, and the
  chunk reads them before the kill moves. A kill sized on an unmeasured floor would be a bound moved without its
  measurement (testing.md 2026-09-28, extended 2026-10-05: a floor's bound moves only with a planted-hang control, both
  readings in `evidence/`).
- "Scored" means the unit's unmutated baseline passes under the `mutants` profile on the dev host and the
  boundary tier's form (`run --mutants --package viola-e2e`) returns a score. The audit's report-only run tested 165 of
  718 mutants in 1314 s and put the whole unit at about 2.5 h. Settled by the split (`inputs#I3`): this chunk proves
  the baseline (the unit's unmutated tests pass under the `mutants` profile on the dev host, so the boundary form no
  longer ends `mutants-exit-4`); the whole-unit score leaves with the split-off entry.
- **The fork, answered (the operator, P4, `inputs#I3`): the `verify_window_` name.** The test is renamed into the
  existing class, and the class's override moves above `package(viola-e2e)` in the `mutants` profile, because the first
  matching override wins. Under the `ci` profile the class's override is already first, so the test's kill there
  becomes the class's 60 s (today the profile's 120 s). `tests/contract_lints.rs` reads the first `mutants` override
  and changes with the order.
- The red CI run Setup read (below) has this as one of its failing jobs: `mutants (viola-e2e)`, job
  113260250076 of `windows-mutants` run 37761947926 on `e304994ae0413c5cf5bf679a5b3d19abded0e472`, failure. It is this
  item's subject. Its Windows grade (the 68 mutants) is the next entry's; this chunk removes the kill that stopped it.
- Beside it, the Epoch 3 diagnosis's P21
  (`.andromeda/runs/2026-10-08T09-45-26-evolve-diagnose/proposals.md:588`): the `mutants` profile kills a root test at
  10 s, equal to `CONFIRM_WINDOW_FALLBACK`, found late at chunk 2026-10-04-confirmed-send-with-cl-1-records' synthesis,
  and owed a line in the project's gotchas. testing.md's 2026-09-24 entry already carries a 2026-10-06
  extension on the 10 s kill for a root test; P3 reads whether `.claude/docs/gotchas.md` holds the line and what is
  still owed.

### 5. Eighteen Linux survivors, each killed or disposed
**Leaves this chunk with the split (`inputs#I3`); owed to the entry this chunk's wrap mints.**
- Source: the same audit's Linux survivor table (CARRY 3; relay item 4, split by host on the operator's answer). 18 of
  the 24 survivors on `x86_64-unknown-linux-gnu` are this chunk's. Each is killed by a test or disposed with a recorded
  argument.
- viola-state, 6: a `NotFound` match guard replaced by `true` at `crates/viola-state/src/events.rs:96:19`,
  `events.rs:149:19`, `stamps.rs:28:19` and `strict.rs:34:23` (the last missed on the Windows runner too);
  `events.rs:209:24`, `<` → `>` in `LoggedLines::next_line`; `fs.rs:290:19`, the content guard of
  `replace_private_shared`.
- viola-agent-claude, 6: `crates/viola-agent-claude/src/ledger.rs:647:81`, `:670:33`, `:685:51`, `:900:76`, `:1172:5`,
  `:1181:74`.
- viola, 6: `src/bin/viola-fake-agent.rs:328:9`, `:437:74`, `:473:46`, `:473:57` (a test-side bin of the root package);
  `src/cmd/mod.rs:133:5`; `src/cmd/hook.rs:214:72`.
- The gate is test-plan §10's (`missed == 0`, `timeout == 0`, `unviable <= caught`), judged at the boundary
  since the 2026-09-28 ruling, so this chunk's own evidence is a direct cargo-mutants run on each survivor's file in
  its owning package, reading the eighteen caught or argued. `run --mutants --file` adds `--in-diff`, so a survivor
  line this chunk does not touch is not regenerated by it (testing.md 2026-09-25); no `[[gate]]` entry carries a
  mutation run (testing.md 2026-10-04).
- §2 splits `dialog_variants` and `submit`, and three survivors sit inside them (`ledger.rs:900:76`,
  `viola-fake-agent.rs:473:46`, `:473:57`). The split moves their coordinates, so those three are matched by mutation
  text after it, and the order of §2 and §5 on those two files is a plan decision.
- `strict.rs:34:23` is listed in both this entry's Linux table and the next entry's Windows list (missed on
  both hosts). This chunk kills or argues it on Linux; its Windows grade stays the next entry's.

### 6. The trailing CR and the newline-only send
- Source: chunk 2026-10-08-first-live-test-and-self-drive (CARRY 4; moved here from "Self-healing state" at the
  2026-10-09 0-pending wrap on the operator's answer).
- The measured facts, re-read at P1 in that chunk's `evidence/live-readings.ndjson`:
  - `trailing-cr` (line 10): a driver text ending in one CR is delivered and answered, filed `human`, not confirmed;
    `send` ends `not-delivered` / `no-prompt-submitted`, exit 13, the wheel at the human;
  - `only-newlines` (line 9): a text of only newlines ends the same refusal when the 10 s window closes, the wheel
    unmoved.
- `The CLI submits it without the CR (measured at that chunk on live 2.1.287, evidence/live-readings.ndjson,
  trailing-cr)`, so the exact match on the typed text fails. The reading holds `prompt_equals_sent_text_without_cr:
  true` for one CR. The mechanism for the mismatch is the entry's statement; P3 reads the match site.
- **The founder's ruling** (live in the overseer's dialog, options shown, relayed in the wrap's dialogue answers; the
  overseer's stamps 2026-10-09T14:44:20Z and 14:48:17Z; recorded in
  `.andromeda/runs/2026-10-09T14-44-30-wrap/adaptation-record.md`): a trailing CR and a trailing CRLF are stripped as the
  trailing LF is, and a text of only newlines is refused at once, the new refusal detail included.
- **Not measured, and measured here before a test pins it:** what the CLI submits for a CRLF ending and for several
  CRs (one CR was read). The live readings run under the founder's cap of 5 starts on 2.1.287 by path (`inputs#I1`).
  The standing rule applies to any step that focuses a window or types a key on the dev host: read
  `hyprctl locked` first and stop on `true`.
- Today, re-read at P1: `viola_agent_claude::hook::typed_text` is `text.trim_end_matches('\n')`
  (`crates/viola-agent-claude/src/hook.rs:202-204`), with the rstest
  `typed_text_drops_every_trailing_newline_and_nothing_else` at `:666`.
- security-plan §Input Validation and its leaf `.claude/rules/security.md` say the typed text is the text "without its
  trailing LF characters" with nothing else removed. The ruling changes that sentence. No master was amended at the
  wrap; the amendment belongs to this chunk's wrap.
- [premise-corrected: research.md §Graph impact — the closed list has more consumers than the entry names, and no
  schema closes it] A refusal detail joins a closed list. The list is `viola_core::NotDelivered` (5 variants, 33
  reference sites in 5 files). The hint table in `src/human.rs` with its three unit-test cause lists is a second closed
  list the new cause joins. `schemas/diag-line.v1.json` leaves `send-refused.detail` an open string, so no schema
  moves. The detail rides `not-delivered`, exit 13. Both check sites (the client's `deliver`, the wrapper's `send`) are
  in the root bin and can read the typed text; `viola-core` cannot, and a check inside `validate_paste_text` would
  reach `answer`. The new detail's name, its rung against `control-character` and the two wheel reads, and its hint's
  wording are contract changes shown at P4.
- **Answered at P4 (`inputs#I3`).** The founder's word (the overseer's dialog, 2026-10-09T15:27:46Z, relayed in the
  operator's answer): the detail is `empty-text`, for any text with nothing to type, the empty text included, and its
  hint is `the text is empty once its trailing newlines are removed; send a text with content`. The overseer's answer:
  its rung is beside `control-character`, checked by the client before any frame and again first in the wrapper, ahead
  of both wheel reads. So the condition is that the typed text is empty, and an empty text, which today is issued and
  waits out the window, is refused at once too.
- **Planned at P4, shown on the review card.** Three starts of the five: one reading a start on the present build
  (a CRLF ending; two CRs), because a prompt the wrapper does not claim hands the wheel to the human and only a human
  hands it back, then one start on the changed build that reads the three endings confirmed and the empty-text
  refusal. The plan adds one reading the entry does not name, read last in that third start and costing none: a CRLF
  inside a text, which the strip leaves in place and nothing has measured. A consequence of the ruled strip not yet
  shown to the founder: a listed local command followed by CR or CRLF classifies as that command, as one followed by
  LF does.
- **Directed at the P5 review (the operator, `inputs#I4`).** The live sessions run headless: no compositor and no
  window. The nested compositor start of the 2026-10-08 chunk is not repeated on the locked desktop, since its one
  recorded start ended the desktop shell. `live-start.sh` runs on a plain pty the rig opens itself, the form the
  harness boots `viola run` on, and that is rehearsed under the fake agent first, with no live start. A rehearsal
  that cannot hold a session stops the live work before any live start; a compositor start is then not the
  builder's to make.
- **Revised after implement stopped on the plan (the operator's word, `inputs#I7`, 2026-10-09T16:35Z).** The plan's
  steps 1 to 15 stand as done. Two things change and nothing else:
  - The two gate entries that read the live records (the start ledger's reader and the readings' reader) carry no
    `artifact` key. They only read files the live steps wrote, so the key's freshness atom read `STALE` at every
    firing while both filters printed `true` with exit 0 (the implement run dir's gate trail, the run of 16:24:23Z
    and its two `show` calls; gate-contract.md §What the tool reads: an artifact is `fresh` when its mtime is after
    the command began).
  - One reading is added inside the cap, start 4 of 5, on the changed build: a text with one lone CR inside it and
    no newline at its end, recorded as read. No behaviour changes with it. Three `start` rows stand in
    `evidence/live-sessions.ndjson` with `cap` 5, and `evidence/live-run.md` names this shape as not measured. The
    two readers and the acceptance line follow the count: four starts and eight named readings.
- No test at any tier pins either outcome yet. Done = both outcomes pinned by tests at the tier that owns them, the
  strip covering CR, CRLF and LF endings as measured.

### 7. Three rule homes
- Source: the Epoch 3 diagnosis's P31 (`…/2026-10-08T09-45-26-evolve-diagnose/proposals.md:773`) and the drained
  builder memory (CARRY 5; relay item 7, pinned here on the operator's answer as the nearest owner, the form left to
  take-up).
- **A home for the CI and operator-pass surface.** No `.claude/rules/` file's `paths:` covers `.github/` or the
  operator pass, so the dispatch-reading learnings of chunk 2026-10-04-windows-boundary-mutation-workflow went to
  Tier 3.
- **The testing rule's reach into `src/`.** `testing.md`'s `paths:` (re-read at P1: `tests/**`, `crates/*/tests/**`,
  `crates/viola-e2e/tests/**`, `e2e-web/tests/**`, `fixtures/**`, `fuzz/**`, `**/proptest-regressions/**`,
  `.config/nextest.toml`) do not load it for a test module inside `src/`. The entry counts 70 files under `src/` and
  `crates/*/src/` carrying `#[cfg(test)]`; the rule is 19.5 KB, so `paths:` reaching all of `src/` load it on every
  product-code edit. Widening it, or a smaller src-scoped file, is chosen at take-up (P4). Leaned at P4 on that
  measured cost, and shown on the review card: a smaller src-scoped file of pointers; `testing.md` is not edited.
- **The host-pressure reader.** It stands in no tier and is owed to `verification-harness.md`:
  `python3 ~/dev/projects/additional/andromedaV3/andromeda-overseer/docs/tools/hostwatch.py read --trail <gate trail>
  --for viola` (also `read --last 15`, `read --from <UTC> --to <UTC>`, `status`). The operator's statement at the wrap,
  measured by him on the dev host. It lives outside this repository by design, so the rule names the path and nothing
  is vendored. The file is present at that path and committed in its own repository (`inputs#I2`, a pointer
  at `7d315d4e`); its flags `--trail`, `--for`, `--last`, `--from`, `--to` were read at P1, the `read` / `status` verbs
  not yet. P3 reads them before the rule line is worded.
- These are `.claude/rules/` leaves, which the wrap's curation owns as a rule; here they are the chunk's
  own stated work. Whether implement writes them or they are the wrap's amendments is a plan decision.

### 8. Two stale citations, cited by title
- Source: the operator's direction (`inputs#I1`); the 2026-10-09 wrap left them for the operator's word.
- `.andromeda/obs-plan.md:660` reads "the dated gap owned by Epoch 6, `:125` / `:127`" and `.claude/rules/security.md:14`
  reads "until Epoch 6 `:125` / `:127`". The two entries are "Server verification before any frame"
  (`working-route.md:133`) and "Home and code-bearing file integrity" (`:135`), both read at P1 from
  `route.py markerless`.
- They are cited by title, as the a11y key file now does
  (`.andromeda/registries/contracts/a11y-plan/keyboard-test-harness.md:8`: "the working-route entry "Windows-only live
  measurements"").
- Its own acceptance: neither file holds a bare `:125` / `:127` for those entries, each names them by title, and the
  leaf agrees with its master.
- [premise-corrected: research.md §Scope premise closure — the two named sites are not the only ones] The same bare
  numbers stand at `.andromeda/security-plan.md:209` (four times, the dated gaps `security.md:14` is the leaf of) and
  `.andromeda/test-plan.md:992` (`:127`, twice). A third stale bare number, of another entry, stands in the
  architecture key file `registries/contracts/architecture/project-directory-structure.md:49` (`:93` for the
  self-healing entry, which `architecture.md:50` cites as `working-route.md:109`). The direction names two sites.
  Whether the item covers the master the leaf mirrors, and the other two, is shown to the operator at P4; a leaf
  re-worded without its master would disagree with it.
- **Answered at P4 (the operator, `inputs#I3`): every stale site found.** The item covers `obs-plan.md:660`,
  `.claude/rules/security.md:14`, `security-plan.md:209` (four times), `test-plan.md:992` (twice) and the architecture
  key file's `:93`. Each is cited by title at this chunk's wrap.
- obs-plan is a spec master and phase amends none. The edit is this chunk's stated work with its sidecar entry, in the
  form the plan names.

## Excluded — owed to other route entries
- **The six retry-loop mutants of `replace_private_with`** (`crates/viola-state/src/fs.rs:270:18`, `:274:20`, `:275:21`,
  `:274:29` ×3): the next entry's, "Windows mutation grade" (`working-route.md:107`).
- **The 15 Windows-side survivors** (audit F3) and **the Windows workflow** (audit F2, the 24 twins, the `viola` job's
  ceiling): the same next entry's.
- **The Windows grade of `viola-e2e`'s 68 mutants**: the next entry's, once §4 removes the kill.

## Boundaries
- No control retired or relaxed, and no boundary widened. The strip in §6 removes trailing newline characters only;
  `validate_paste_text` still refuses every other C0, DEL and C1 and never strips a refused character.
- No route reorder or removal here. A split, if P4's card is answered with one, is the founder's word and the wrap's
  write.
- Live `claude` starts: at most 5, 2.1.287 by path, each ledgered before it is made (`inputs#I1`). viola writes nothing
  under the user's `.claude` directory.
- Evidence lives in `chunks/2026-10-09-epoch-3-cleanup/evidence/`.

## CI read at Setup (the last wrap's flip `e304994` → HEAD)
| sha | verdict | wall |
|---|---|---|
| `59e791e9d321` | in progress, verdict not yet available (ci#37948581286, 15 checks) | — |
| `fe4f47fbd430` | green · 15/15 · ci#37945992768 | 442 s |
| `e304994ae041` | red · 21/21 · first-fail +2277 s `mutants (viola-e2e)` · ci#37758116153 green 15/15 · windows-mutants#37761947926 red | 9238 s |

The red is the boundary audit's own report-only dispatch of `windows-mutants.yml` (run 37761947926), read per check
through the commit's check-runs: `mutants (viola-e2e)` failure, `mutants (viola-state)` failure, `mutants
(viola-channel)` failure, `mutants (viola-pty)` failure, `mutants (viola)` cancelled, `mutants (viola-agent-claude)`
success. Disposition:
- `mutants (viola-e2e)` intersects this chunk and is folded as §4's runner-only bullet, closed at P3 against the run's
  own job log (research.md, Platform issues consulted).
- The other four non-success jobs do not intersect it. They are not absorbed here and not dropped: the operator minted
  their owner at the 2026-10-09 0-pending wrap, the route entry "Windows mutation grade" (`working-route.md:107`), whose
  three CARRYs name this run, its 15 survivors, its 24 twins and the cancelled `viola` job. Recorded as owned there.
- `59e791e` is not read as green. Its verdict is read again before the plan's review.
