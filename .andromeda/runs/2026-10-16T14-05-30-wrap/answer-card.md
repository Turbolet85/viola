# Answer card — 2026-10-14-resume-and-compact-ledger-rows

## The answer
- `inputs#I1`, fork 2 of chunk 2026-10-14-resume-and-compact-ledger-rows (a boundary widening of the verify child
  set, and a new id source on a child's argv).
  - The answer: "Run E: a sixth start (Recommended)".
  - Its note: «Run E, шестой старт» — the widening ratified by him.
  - Whose word: the founder's own, given live in the overseer's dialog at 2026-10-14T07:31:48Z and relayed verbatim
    by the operator.
  - What the fork showed: after Run D, a new Run E in its own 0700 dir runs `--resume <Run B's first id>`. The id is
    `is_session_id`-checked, built by `resume_args`, and is one argv element of a direct spawn under the R8 strip. Run
    E waits for a SessionStart of source `resume` with that id, settles, and is killed with no key. It adds one start
    per round and a seventh spawn pair (`verify-pty-probe`), and follows Runs A–D's stop and dir-guard rules.

## Proposals lying wholly inside it
Numbered as `fanout-results.md` numbers them:
- architecture 3 — §Established Decisions → [CLI Version Compatibility]: verify drives five interactive runs. Run E
  takes Run B's first id through `resume_target` / `resume_args`, runs in its `-resume` dir with empty input and no
  `--settings`, and a verify is six `claude` starts.
- architecture 6 — §Established Decisions → [Plugin Scope]: verify's transient unwrapped children gain Run E (the
  `-resume` dir and the `resume/` plugin), and it adds no transcript.
- architecture 10 — §Occupied Resources → Filesystem: Run E's `<cwd>/.viola-verify-<pid>-resume/` (0700, its guard).
- architecture 11 — §Occupied Resources → Filesystem: the probe dir gains `resume/` (plugin + `captures/`).
- architecture 12 — §Occupied Resources → Claude Code integration names: verify's children gain Run E's argv
  (`--resume <id>` through `resume_args`, `is_session_id`, direct spawn, R8 strip, `io::empty()`).
- security-plan 1 — §Input Validation, Session id as a child argv value: a third id source, Run B's first SessionStart
  capture inside verify.
- security-plan 2 — §Threat Model Summary, the `revive` entry point: `resume_args` gains verify's Run E as its second
  caller.
- security-plan 3 — §Threat Model Summary, the `viola verify` entry point: five interactive children, Run E's bounds
  and seven spawn pairs.
- security-plan 4 — §Input Validation, Constants (`MAX_FRAME`): the child-output clause covers five runs, A to E.
- security-plan 5 — §Data Protection, Probe captures: the `resume/` root.
- security-plan 6 — §Data Protection, Interactive probe dirs: Run E's `-resume` dir.
- security-plan 7 — §Data Protection, Accepted risk (verify's trusted runs): Run E named; measured to add no transcript
  file.

## The confirm, as given (`inputs#I6`, question 1)
Answer: "Confirm (Recommended)"
Notes, verbatim: Overseer's delegate confirm at 2026-10-16T14:24:59Z: the 12 proposals (architecture 3, 6, 10, 11, 12; security-plan 1-7) record only Run E as shown in fork 2 (inputs#I1). PROVISIONAL until the founder's Epoch 5 batch; the ratification itself stays his answer.

- Authority: delegate viola-overseer, so the confirm is PROVISIONAL. The ratification is the founder's own answer to P4
  fork 2 (`inputs#I1`), relayed verbatim by the operator. His own word on this card is owed at the Epoch 5 boundary
  batch.
