# Answer card — Q1 · 2026-11-24-viola-up · wrap 2026-11-24T07-06-28Z

## The answer it rests on
- **U2 arm 1 — «viola revive с флагом origin (Recommended)»**: the operator's own answer to fork U2 (his dialog of
  2026-11-24T05:21:35Z), relayed verbatim by overseer1 — `inputs#I6` item 3 (the revision relay), cited by the plan's
  Expected amendments and by the wrap directive `inputs#I16` item 2. The arm: `foot` with the closed argv
  `--app-id=viola.<name> --title=<name> --working-directory=<recorded cwd> <pinned viola> --home <home> revive <name>
  --origin <cli|agent>`, one element per value, no shell; revive gains the hidden `--origin` alone, and its window step
  moves the window to the workspace its OWN snapshot recorded (the record never leaves the snapshot onto an argv).
  A boundary widening (playbook `rul-r74hshif` / `rul-p2zl2kgl`).

## The operations read as lying inside it (11)
- ops-architecture.md · decision/revive-resumes-in-place · established-decisions · fit: fits type 1 — under the hidden --origin the revive also replays the recorded workspace
- ops-architecture.md · contract/cli-verbs/revive · conventions · fit: fits type 3 — `--list` conflicts with the hidden `--origin` too
- ops-architecture.md · contract/instance-snapshot/cwd · standard-contracts · fit: fits type 3 — the recorded cwd's second reader, up, opening each terminal there (`--working-directory=<recorded cwd>` of the arm)
- ops-architecture.md · contract/viola-cli-arguments/revive-origin · occupied-resources · fit: fits type 3 — revive's hidden `--origin <cli|agent>`, its window at the recorded workspace; its «else none» (a hand-started session) rests on U3, a delegate's arm, PROVISIONAL
- ops-security-plan.md · decision/open-terminal-direct-spawn · fit: fits type 1 — up's terminal starts only the pinned `viola revive <name> --origin` in the recorded cwd
- ops-security-plan.md · decision/run-window-flags-checked · fit: fits type 1 — revive's hidden `--origin` held to the same two closed words, conflicting with `--list`
- ops-security-plan.md · contract/revive-arguments/id · fit: fits type 3 — `--list` conflicts with `--origin` too
- ops-security-plan.md · decision/cwd-only-in-snapshot · fit: fits type 1 — the recorded cwd also stands as the working directory on up's terminal argv (the arm's own argv)
- ops-security-plan.md · decision/spawn-cwd-snapshot-only · fit: fits type 1 — the same, on the twin row
- ops-layout-templates.md · contract/viola-revive/no-window · surface-cli · fit: fits type 3 — a revive places a window only under the hidden `--origin`; its «reads unplaced when none was recorded» rests on U3, PROVISIONAL
- ops-layout-templates.md · contract/viola-revive/usage · surface-cli · fit: fits type 3 — `--list` conflicts with the hidden `--origin`

Named beside the card, not asked: none fit read `not_master`.

## The answer, as given
«Name the ones outside» — notes, verbatim: «delegate overseer, 2026-11-24T07:45:31Z: THREE are outside his answer —
security-plan decision/cwd-only-in-snapshot, security-plan decision/spawn-cwd-snapshot-only, architecture
contract/instance-snapshot/cwd. The overseer read the question he was shown (overseer1's dialog, answered
2026-11-24T05:21:35Z): its arm 1 names foot started directly, the pinned viola revive <name> --origin cli|agent, and
revive placing its own window from its own snapshot. It does NOT say the RECORDED cwd crosses onto the terminal's argv.
That crossing is a widening of its own he has not seen: those three do not land as his. The other eight are confirmed
inside his answer, their U3 parts as you marked them — and U3 is since answered by him too (arm 1, «Где поставит
композитор (Recommended)», his dialog of 2026-11-24T07:15:07Z; a relay with every answer of this morning follows at
your next turn end).»

- Confirmed inside U2 arm 1 (8): architecture decision/revive-resumes-in-place, contract/cli-verbs/revive,
  contract/viola-cli-arguments/revive-origin; security-plan decision/open-terminal-direct-spawn,
  decision/run-window-flags-checked, contract/revive-arguments/id; layout-templates contract/viola-revive/no-window,
  contract/viola-revive/usage.
- Outside (3), escalated as Q2 — the recorded cwd onto up's terminal argv, a widening of its own: security-plan
  decision/cwd-only-in-snapshot, decision/spawn-cwd-snapshot-only; architecture contract/instance-snapshot/cwd.
- Q2 resolved (`inputs#I19`, delegate overseer): «Q2: land the three cwd rows PROVISIONAL on the delegate word, the
  crossing written to founder-forks.md as a fork of its own (it is asked of him now).»

# Answer card — Q3 · the operator's own answers of this morning (`inputs#I18`)

## The answers it rests on (relayed verbatim by overseer1, `inputs#I18`)
- U1 arm 1 «Прерванные перезагрузкой (Recommended)» — dialog 2026-11-24T07:15:07Z.
- U3 arm 1 «Где поставит композитор (Recommended)» — dialog 2026-11-24T07:15:07Z.
- U5 and U7 «Подтвердить оба (Recommended)» — dialog 2026-11-24T07:27:29Z.
- U8 arm 1 «Оставить, подсказка объясняет (Recommended)» — dialog 2026-11-24T07:27:29Z.
- F1 option 1 «Три отдельные строки (Recommended)» — dialog 2026-11-24T07:10:56Z.

## The operations read as lying inside them
- ops-main-architecture.md · decision/up-selection-rule · established-decisions · fit: fits type 1 — U1 arm 1 (lands with no provisional mark)
- ops-architecture.md · contract/process-exit-detail/boot-time-unread · occupied-resources · fit: fits type 3 — U7
- ops-design-system.md · contract/viola-up-outcome/would-open · surface-cli · fit: fits type 3 — U7
- ops-main-design-system.md · contract/viola-up-outcome/not-taken · surface-cli · fit: fits type 3 — U7
- ops-design-system.md · contract/fixed-refusal-pairs/boot-time-unread · surface · fit: fits type 3 — U7
- ops-design-system.md · contract/viola-up-output/not-live-hint · surface · fit: fits type 3 — U7, U8 arm 1
- ops-obs-plan.md · contract/process-exit-detail/boot-time-unread · log-coverage · fit: fits type 3 — U7
- Q1's U3 parts (architecture contract/viola-cli-arguments/revive-origin «else none»; layout-templates
  contract/viola-revive/no-window «reads unplaced») — now his whole, U3 arm 1.
- ops-main-a11y-plan.2.md · decision/open-live-wait-bound, decision/window-wait-bound, decision/hyprctl-read-deadline
  — F1 option 1 (fit judges them after the two list rows land).

## Beside the card, not asked — not_master, landing only on his own word
- ops-main-a11y-plan.md · decision/time-bounds-never-on-human · fit: not_master type 5
- ops-main-a11y-plan.md · decision/time-bounds-fall-on-the-driver · fit: not_master type 5
  F1 option 1 is his own answer to reword these two list rows. The store's row form refuses his «These» as a lead (it
  reads alone), so «Eight» stands in its place, the eight named bounds beside it. They land against the `not_master`
  on his word, `inputs#I18`, the departure from «These» named here and in the sidecar.

## The answer (Q3), as given
«Name the ones outside» — notes, verbatim: «delegate overseer, 2026-11-24T07:49:50Z: every operation of the list lies
inside his answers and lands as his, EXCEPT the WORDING of the two a11y list rows (decision/time-bounds-never-on-human,
decision/time-bounds-fall-on-the-driver). His answer's words for them are «These time bounds»; the store refuses that
lead and the rows would read «Eight time bounds». The substance is his (the lists stop claiming to hold every bound,
the three bounds stand as rows of their own) — land both sets, but with the lead «Eight» marked PROVISIONAL on the
delegate word and the store's refusal quoted in the sidecar Why, so that he sees the word that is not his. The three
new a11y rows and every other listed operation: confirmed as his.»

- Done so: the two sets land on his word (`inputs#I18`, the substance); then `ops-main-a11y-plan.3.md` sets each row's
  `state` to `provisional` on the delegate word, its Why quoting the store's refusal of «These».
