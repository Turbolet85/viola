# Adaptation record — 0-pending wrap 2026-11-16T06-05-33 (viola-0.2.0, Epoch 3)

Path: SKILL Setup 6, the no-op path with an operator route-adaptation request (route-resolve §Operator-requested
adaptation), the requirement step, the arm that ratifies PROVISIONAL marks (row operations main writes itself, and new
sidecar entries where a mark stands in entry text alone), and curation for the founder's project rule. 0 pending,
0 gated. HEAD `3ac557bd`. The tree was dirty only with expected-transient bookkeeping: the journal
`.andromeda/db/log/2026-11-16T04-11-42-2026-11-16-send-confirmation.ndjson`, the handoff, and the last wrap's
`evolve-2026-11-16-send-confirmation.json` trail.

Inputs (snapped `--no-chunk` into `.andromeda/runs/2026-11-16T06-05-33-wrap/inputs/`):
- inputs#I2 — the overseer's answer to this wrap's one dialog: v2-34 and v2-35 land as drafted
  (`relay-1.md`).
- inputs#I1 — the Viola overseer's relay of the founder's six rulings of 2026-11-16,
  `../additional/viola-overseer/e3-mid-route-adaptation.md` (copy, no-repo). The founder's words in it were given in the
  Viola overseer's two question dialogs of 2026-11-16, 06:01Z–06:04Z.

## Items

1. A — a new entry «Granular API-error cause»: `turn-ended`'s closed `cause` set grows from `api-error` to nine words;
   only the word crosses, the error text reaches nothing. His word «Подробный список (Recommended)» (inputs#I1 §1 A).
2. B — a new entry «Scheduled-wake count on a turn end»: Stop's `session_crons` carried as a count beside
   `background_tasks` on `turn-ended`; `wait`'s wake rule unchanged. His word «Ратифицировать + добавить session_crons
   записью (Recommended)» (inputs#I1 §1 B).
3. PROVISIONAL 1 — «API-error turn end» F1: StopFailure registered with no ledger row and no verify probe, the live
   reading owned by «The 2.1.325 recorded fixture set». His word «Ратифицировать без строки ledger (Recommended)».
4. PROVISIONAL 2 — «API-error turn end» F2, the one word `api-error`: superseded by A, not ratified as final.
5. PROVISIONAL 3 — «`wait` wake condition» F1: `wait` documented and pinned as built; «a test that fails before the
   fix» read as each case's remove-the-guard control. His word: B's answer.
6. PROVISIONAL 4 — «`wait` wake condition» wrap Q1: security-plan's fourth way a test knows a directory is its own
   (another build's stamp dir, removed only when its lock is taken). His word «Ратифицировать четвёртый способ
   (Recommended)».
7. PROVISIONAL 5 — «Send confirmation» Q1: the send's match tolerates a trailing run only; «either end» waits for the
   live reading carried on «Standing live rig with a screen model». His word «Только хвостовые, ведущие — после
   измерения (Recommended)».
8. PROVISIONAL 6 — a project rule, his word «Так и делать (Recommended)»: a CI red not the chunk's is measured for its
   place and pinned open on a named owner; ONE re-run of the failed job for the chunk's verdict is allowed; a green
   re-run never closes the red, only its fix does.
9. Anchors — A and B at the end of Epoch 3 after «Survey key keeps the wheel», A before B; B's place his word
   («конец Эпохи 3»), A's place beside it the overseer's, PROVISIONAL. «Local-command delivery» stays next; nothing
   else moves. A and B take requirement rows by this wrap's dialogue, the overseer answering.
10. The three leaves saying «all twenty-four» relied ledger rows against the code's 21
    (`.claude/rules/verification-harness.md:28`, `.claude/docs/commands.md:23-24`, `.claude/docs/tests-summary.md:21`).
11. The no-op path's own duties: gated premises, the seal of earlier run dirs.

## Dispositions

1. APPLIED — `rec.py put` TX 1873: the new entry `chk-z6paw647` «Granular API-error cause» (v2-34) with its carry
   `cry-zriq6bde` (the nine words, the founder's word verbatim, the F2 supersession, the placement PROVISIONAL as the
   overseer's). Requirement: +v2-34 «A turn end's API-error cause named from a closed list», planned, unclaimed,
   theme (d) after v2-33 (`matrix.py add`, TX 1871); its five fields the overseer's answer, inputs#I2.
2. APPLIED — TX 1873: the new entry `chk-76zccprp` «Scheduled-wake count on a turn end» (v2-35) with its carry
   `cry-ljmfpqi3` (the founder's word verbatim, his placement «конец Эпохи 3»). Requirement: +v2-35 «A scheduled wake
   counted on a turn end», planned, unclaimed, after v2-34 (TX 1872); fields per inputs#I2.
3. APPLIED — row operations of main's own (`ops-main-{doc}.md`, basis adaptation:3), fit 6/6 `fits` type 2: the
   «provisionally» left the `except` cells of architecture `decision/capability-ledger`, `decision/capability-ledger-gate`,
   `decision/cli-capability-ledger` (TX 1856), security-plan `decision/unverified-cli-gate` (TX 1857) and test-plan
   `decision/hook-payloads-recorded-by-verify` (TX 1858). Marks standing in entry text alone: a new entry beside each
   marked «2026-11-15-api-error-turn-end» entry (`rec.py amend`, TX 1874–1877; every entry `on-form`).
4. DISCHARGED by A — no row changed: `contract/event-data/turn-ended-cause` (architecture), security-plan
   `decision/api-error-turn-end-text-dropped` and `decision/api-error-classes-adapter-only`, design-system
   `decision/api-error-turn-end-shown-as-turn-end` keep their one-word text and their provisional mark until
   «Granular API-error cause» lands; the entries beside the marked F1/F2 entries name the supersession, and A's carry
   names it.
5. DISCHARGED here — no row or sidecar entry carries the mark: the `wait` wake condition's F1 rows (test-plan
   `contract/viola-wait/wake` and its kin) stand as built with no provisional cell, and the marked text lives only in
   the sealed phase run `2026-11-16T00-29-34-phase`'s founder-forks.md. His word: B's answer (inputs#I1 §2 item 3).
6. APPLIED — security-plan `decision/test-removal-own-dirs-only` `state` provisional → standing (TX 1857, a state set,
   no fit); its sidecar entry names the ratification.
7. APPLIED — `state` provisional → standing on architecture `decision/submitted-form-prompt-match` (TX 1856),
   security-plan `decision/delivery-confirmation-trailing-tolerance` (TX 1857), test-plan
   `decision/send-confirmation-trailing-trim-match` (TX 1858); the «provisional(ly)» left the promises of design-system
   `contract/send-readback/prompt` (TX 1859), layout-templates `contract/viola-send-output/read-back` (TX 1860), obs-plan
   `contract/confirm-window-close/ordinary-send` (TX 1861), each fit `fits` type 3. Entries beside the marked
   «2026-11-16-send-confirmation» entries (TX 1874–1879). The carry `cry-vdpx2jqz` on «Standing live rig with a screen
   model» rewritten (TX 1873): the trailing-only tolerance ratified, «either end» waits for its reading.
   Cascade: no sweep pattern (prose-only sets); the leaf pass (14 groups, 25 leaves) found 11 stale «provisional»
   statements, 10 applied by `cascade.py`, 1 by hand (CLAUDE.md, TX 1869) — `cascade-dispositions.md`.
8. APPLIED — curation Tier 2: a dated entry in `.claude/rules/testing.md` `## Session Additions`, after the timing-red
   rule, worded so the «never re-run for green» rule stands (`rec.py harness land`, TX 1870). Filter 3 tension, not
   applied: CLAUDE.md's Tier 1 learning «a red met during a chunk folds into that chunk even outside its diff» reads
   against «pinned open on a named owner» — to the handoff for the operator.
9. APPLIED — the anchors, read back: `markerless` lists «Local-command delivery» → «Survey key keeps the wheel» →
   «Granular API-error cause» → «Scheduled-wake count on a turn end», then Epoch 4 unchanged; Epoch 3 reads 8 entries,
   4 markerless; `abstained` 0 rows; every touched carry reads introducer `CARRY:`.
10. PINNED — the 0-pending path fixes no leaf outside a cascade (SKILL Setup 6: a divergence noticed here waits for its
    chunk wrap), and this wrap's pass-mode leaf comparers did not reach the count. A carry on «Version done-check»
    (`cry-2snj2igc`, TX 1873) names the three leaves.
11. Gated premises: the `gated` view holds 0 rows. No consolidation and no seed-rule supersession requested. The
    coverage line after the writes reads `verified 11/35 · deferred 1 · planned 23 · implemented 0 · unclaimed 23`.
