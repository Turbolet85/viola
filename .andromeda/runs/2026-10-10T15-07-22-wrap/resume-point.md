# Resume point — wrap of 2026-10-10-viola-revive

Written 2026-10-10T15:13Z, at the end of Phase 1. **Next: Phase 2** (the citation sweep first, then the
fan-out). The operator ended this session after Phase 1 on a context reading past the 60 % line (`inputs#I6`)
and resumes the wrap in a fresh one, with the route directions on the resume line.

## Where the wrap stands

| phase | state |
|---|---|
| Setup | done in this run dir. No flag was given. One pending record, `2026-10-10-viola-revive`. |
| Phase 1 — report | done: `viola-0.1.0/chunks/2026-10-10-viola-revive/report.md`, 572 lines, sha256 prefix `aa6ff1cff93b89ad` at 15:12:56Z, its last section the pasted listing |
| Phase 2 — reconcile | **not started**: no `cites.py apply`, no fan-out, no master or sidecar touched |
| Phases 3 to 7 | not started |

A resumed Setup meets step 2a (the report exists): the answer is **resume in this run dir**, at Phase 2. The
report is reused as it is. No `.andromeda/` master or sidecar has changed in the tree.

## What Setup read (step 4), for the resumed session to read again and not to trust

- Branch `build/viola-0.1.0`; HEAD `0fad11c`, 0 ahead of its upstream at 15:12:56Z. The operator pass ran:
  the oldest `chore(2026-10-10-viola-revive): operator pre-CI commit` is `0fad11c`, its parent `00c73fd`. Every
  "parent commit" basis of this wrap is `00c73fd`.
- `git log --format='%h %s' 00c73fd..HEAD`: one line, the pre-CI commit.
- `state.yaml`: `last_wrap` 2026-10-10T11:23:19Z, `session_count` 59.
- The tree at 15:12:56Z, all of it this wrap's or the pass's: modified `.andromeda/friction-log.ndjson`,
  `evidence/operator-pass.md` (the sections written after the pre-CI commit), `inputs/andromeda-inputs.json`
  (`I6` added), `research.md` (four lines indented on the operator's word, `inputs#I6`); new `report.md`,
  `inputs/I6-relay-1.md.txt`, this run dir.
- The code-graph refresh was fired at Setup and ended: `.refresh-done` at 15:08:15Z, `tree-refresh[rust]: 4735
  nodes / 24935 edges - 33s`, `tree-refresh[ts]: 7 nodes / 1 edges - 0s`. `tree.db.commit` still names
  `00c73fda87ef`; the stamp to the new HEAD is Phase 7's. A resumed Setup need not fire the refresh again unless
  source moves, and the wrap moves none.

## What Phase 1 ran

- `gate.py scope`: first `UNPARSED 4` (research.md lines 171 to 174), then, after the operator's word and the
  indent, `scope: clean — changed 21 · listed 21 · recorded 0 · absorbed 0 · excluded 88` (15:08:21Z, base
  `00c73fda`). A resume re-fires it.
- `inputs.py snap` → `I6` (this run dir's `relay-1.md`: the wrap invocation and the answer at the scope halt);
  `inputs.py verify` after the report: 6 entries, unchanged 1, n/a 5, uncited 0, unparsed 0.
- `cites.py added --out new-text.md`: 21 files, 3 new, 96 added ranges, 2090 lines, 178 blocks, 224 listing
  lines; `splice.py append … --lines 224`: `report.md` 348 → 572 lines. A resume re-fires both and expects
  `unchanged` and `skipped`.
- The evolve checkpoint of the report step: two records, ids `2026-10-10T15:12:30Z-a` and `-b`.

## What Phase 2 will meet in the report

- Thirteen expected amendments, each carried by a Changes bullet, with site counts per master and key file.
- One entry under **Spec claims disproved by measurement**: the plan's `wait_endpoint_gone` for a killed
  wrapper (a plan claim; check 6 wants it DISPOSED).
- One acceptance criterion met with a limit and raised for P2: the program name `claude` also stands at
  `src/cmd/verify.rs:105`, which predates the chunk.
- One coverage flag written ✗: no widened-DACL case for an instance tree on Windows.

## For Phase 3, from this conversation (it will be gone)

Corrections and findings only this window held. None is curated yet; each is a candidate for the five filters.

1. A wrapper killed on Unix leaves its socket file, so an endpoint wait keyed on the file being absent never
   ends after a kill; the kill path needs a connect probe (`holder_gone`), the stop path keeps the old rule.
2. Every test that types `viola revive` puts the fake agent as `claude` first on its child's `PATH`: a preflight
   that wrongly passed would otherwise start the host's own CLI by name.
3. A wrapper a test ends through its drop guard writes no coverage profile; a case whose start path must count
   toward coverage stops its wrapper cleanly.
4. Prose appended to `research.md` after the lists check has to keep the list grammar (an indented line belongs
   to the item above); the lists check is re-fired after any later edit of the two sections.
5. The key files stand under `.andromeda/registries/contracts/<master>/`: a site count that maps owners by the
   registry's top-level names reads every key file as unowned.
6. On the live rig: a second `--plugin-dir` after `--` ran its own SessionStart hook beside viola's on 2.1.287;
   the model alias `haiku` is taken; the bare name `claude` on the dev host is 2.1.289, so a rig that needs the
   stamped version puts the 2.1.287 install directory first on its host's `PATH`.
7. The write guard refuses the Write tool under `target/`; rig scratch there is made by a rig script kept in
   the chunk's evidence.
8. Recurrences of `host-linux.md` in this session: a heredoc with a file target (once), a `cd` outside a
   subshell (once), an inline python heredoc for a multi-edit of a source file (once).
9. A count of test names carrying a filter's tokens also counts an integration case whose name holds one (71
   lines against the unit filter's 70).

## For Phase 5 and Phase 6, from this conversation

- The plan's route-step items (plan.md, its last note) and the founder's answers (`inputs#I5`): the
  `session-live` CARRY on "The board: viola list" with P4 and P12; the owner of the owed `--resume` ledger row;
  the revive causes on "Exit-cause code catalogue"; revive's `--json` on "CLI machine contract"; the Unix pin
  of a child ending with its killed wrapper, read green on both Unix runners with the fake agent; the wheel
  answer (a revived start keeps `driver`, `start`).
- Found at implement, with no owner yet: the real resume payload's five more key names against the fake
  agent's; `viola send` of `/compact` refused `no-prompt-submitted` while the command runs; the dev host's bare
  `claude` at 2.1.289 against a home stamped for 2.1.287.
- Not measured (the lists are in `evidence/live-revive.md` and `evidence/operator-pass.md`): `--fork` on the
  real CLI; a resume of a killed session on the real CLI; whether a resumed session held its earlier turns;
  project settings on a resume from another directory; the bounded removal's loop on Windows; a widened DACL on
  an instance tree.
- On the operator's desk: `target/rev-live-911840/` (its removal was denied by the permission layer and not
  done another way); three instance directories added to the stamped live home `target/e2e-home/viola-live-4043089`
  (`revreh`, `revlive`, `revhand`); the CLI's own transcripts of the three live sessions under its project
  directory for `target/rev-live-911840/a`; the rig's private directory under the implementing session's
  scratchpad, named in `evidence/live-revive.md`.
- Last failed command: none.
