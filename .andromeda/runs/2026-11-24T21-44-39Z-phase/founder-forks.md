# Founder forks — 2026-11-24-adopt-s-resume-folder (phase run 2026-11-24T21-44-39Z-phase)

## P3 block — written at P3's end, before P4

**Window.** Builder session, phase take-up of «Adopt's resume folder» (`chk-m3dcvxca`) at `cfe2329a`, promoted as
`2026-11-24-adopt-s-resume-folder` (tx 4889). Row read: 7 tasks, 159 picker draws, 7 history agents, all first try;
1191 of 4990 rows picked. No live start, no real `claude`, nothing of `~/.claude*` read. Directive: `inputs#I1`.

### Measurements (research M1–M4, `doc:2026-11-24-adopt-s-resume-folder/research`)

- **M1, the two red children.** Firing 2: child exit status 1, 824 ms after its start (18:57:52.776Z →
  18:57:53.600Z), one SessionEnd, no SessionStart; the wrapper exited **0** at 1259 ms. Firing 1: no `session-start`
  within ~110 s; its exit code and time were not recorded. Today any child exit ends the wrapper exit 0 with nothing
  printed (`src/cmd/run.rs:866-870`). The child's status is only on the run log's `process-exit{subject:"claude-child"}`.
  To tell «no such session here» apart, viola can read only «the child ended before any session-start» (both facts
  it already holds). The CLI's own screen for a missing id is unrecorded (no fixture), and the projects store is the
  user's Claude store.
- **M2, the resume folder.** `revive` and `up` take the snapshot's recorded `cwd` (`src/cmd/revive.rs:325-334`,
  `src/cmd/up.rs:403-406`). `adopt` alone has no record and takes the caller's current directory
  (`src/cmd/adopt.rs:163`).
- **M3, the ledger: hypothesis A is FALSIFIED at the measured grain.** The relied row `resume-session` resumes Run B's
  session from Run E's folder `<cwd>/.viola-verify-<pid>-resume/`, not Run B's `<cwd>/.viola-verify-<pid>/`, which is
  already removed (`src/cmd/verify/typed/runs.rs:59,203,298`). It is recorded on 2.1.328
  (`fixtures/claude/2.1.328/SessionStart.resume-1.json`, `source` `resume`, `cwd` `…-resume`) and stamped pass on
  2.1.331 (29 rows, `r5.md:31`). Two things stay UNMEASURED: a resume from a folder that is not a sibling, and any
  git-root effect.
  - Under A alone the transcript would still stand in its maker's project folder, and the overseer's search found
    none at all. So the reds need B, or a cause C this research adds.
  - **C:** the operator's `claude` kept the session under another Claude config dir (a `CLAUDE_CONFIG_DIR` set in his
    terminal). The R5a child, started from a shell where that name was unset, does not read it. In everyday use
    viola's own strip removes that name unless `claude_env_keep` keeps it.
- **M4, telling A, B and C apart — for the overseer to put to him; this phase fires nothing.**
  - **Zero-start readings first:**
    1. Does the terminal he made the two sessions in set `CLAUDE_CONFIG_DIR`? (The name only.)
    2. If it does, a file-name search for the two ids under that dir's `projects/`.
    3. How he made the sessions and where he read the ids.
  - **If those leave it open, one live check by his hand:** a plain `claude` in folder F, one exchange, `/exit`, then
    `claude --resume <id>` from a folder G that is no sibling of F.
    - It costs two interactive `claude` starts and no viola.
    - It leaves a transcript under F's project folder (and G's if it opens one), the input-history lines, and his
      trust answers for F and G.
    - He undoes it by hand: the transcripts, and the two trust keys in `~/.claude.json`.

### Rows reached (the seven masters, `rec.py find --regex`; read, by handle)

Reached by the provisional arms below:
- `design-system · decision/no-json-after-child-held-terminal` (r2: «viola writes nothing past it»),
  `a11y-plan · contract/viola-revive-json/pump-child-error` and `a11y-plan · contract/tui-passthrough/pump-child-error`
  (both «viola writes nothing past it»): fork 2's line written after the child ends.
- `architecture · decision/no-document-after-child-held-terminal` and `contract/revive-json/pump-error`,
  `test-plan · contract/revive-json/pump-child-error` and `decision/revive-json-no-document-after-terminal`,
  `layout-templates · contract/run-revive-start-json/internal-error`: each about a pump ERROR, not a child's own
  exit. Read, not reached, unless fork 2's `--json` document is read as one.
- `obs-plan · decision/adopt-start-logs-as-run` («adding no event, detail word, field, subject or span of its own»):
  the new `process-exit` detail word.
- `obs-plan · contract/process-exit-detail/exit-1-adopt` («preflight ends exit 1 with one process-exit detail
  session-live»), `contract/exit-1-detail/session-live`, `contract/run-exit-1-detail/session-live`: a new exit-1
  detail of adopt, after the child rather than in the preflight.
- `architecture · contract/cli-json-output/adopt-refusal-codes` («only `strict-modes-failed` and `session-live` are
  adopt's own closed details»: a count of two), `a11y-plan · contract/cli-refusal/unable-line` («adopt's two own
  refusals»), `a11y-plan · contract/cli-fixed-message/form`, `a11y-plan · contract/start-json-refusal/document`,
  `design-system · contract/cli-exit-codes/1-adopt-start`: a third own refusal of adopt, `not-resumed`.
- `design-system · decision/run-prints-nothing-while-wrapped` (nothing «while the wrapped claude TUI runs»): read, NOT
  reached — the line comes after the child has ended.
- `architecture · contract/cli-verbs/adopt` and `layout-templates · contract/cli-verbs/adopt` (the verb's usage): read,
  not reached under fork 1 (i); reached under (ii).

Read, not reached by any arm: `security-plan · decision/adopt-signals-no-foreign-process`,
`decision/session-id-argv-closed-shape`, `contract/strict-modes-refusal/adopt`, `decision/claude-agents-resolved-from-path`;
`obs-plan · decision/adopt-session-id-never-logged` (the new word holds no id); every strict-modes and socket-directory
row naming adopt's start (unchanged).

Escalate patterns read first (`rule` rows, verdict escalate): `rul-r74hshif` and `rul-p2zl2kgl` (Boundary widening),
`rul-o2tr55aj` (held items), `rul-bznresq5` (widening the founder answered at a P4 fork).
- Fork 1 (ii) and (iii) each meet «Boundary widening»: (ii) a validated surface admits a new input class, and (iii)
  a new read of the user's Claude store.
- Fork 1 (i) and fork 2 meet none: no new crossing, input class or read. Fork 2 departs from ratified output rows,
  which is his word but no widening.

### Forks of his — each with its arm taken PROVISIONAL meanwhile

**Fork 1 — how `adopt` gets the session's folder (a crossing).** The options, in order:
1. **(i) Recommended.** The operator types `adopt` in the session's folder, and viola refuses with a named hint when
   the child ends at once.
   - Reads nothing new and changes no surface but fork 2's.
   - Cost: the operator must know the folder. M3 shows the folder alone did not decide the reds.
2. **(ii) A flag naming the folder** (`viola adopt … --cwd <dir>`, as `viola open` takes one). The spawn folder
   becomes a given path, the start's trust step writes a persisting trust record for it, and the verb's usage rows
   (arch, layouts), help and a11y rows change.
   - A boundary widening (a new input class), «Boundary widening».
   - Cost: one flag, its validation, about four master rows, the trust-record crossing.
3. **(iii) viola looks the id up in the CLI's projects store and starts there.**
   - What it reads: `<config>/projects/*/` listed for `<id>.jsonl`, and to get the folder, the transcript's `cwd`
     field — or the project folder's name, which is not reversible.
   - Rows it reaches: `security-plan` §Input validation (the transcript read today is the wrapper's own session's,
     from its recorded offset, a count only), the NEVER-log floor's transcript-path rows, and `obs-plan`
     `decision/adopt-session-id-never-logged`.
   - A boundary widening (a new read of the user's Claude store), «Boundary widening».
   - Cost: a new reader, a ledger row and verify probe for the store's layout, and the most reached rows. M3 says it
     would not have saved R5a under B or C.

**PROVISIONAL meanwhile: (i).**

**Fork 2 — what «a child ending at once» is, and what adopt says and records (his: it departs from «viola writes
nothing past it»).** Provisional arms:
- **What counts:** the resumed child ended before any `session-start` reached the wrapper. No time bound, so no new
  bound row. The alternative is a compiled bound (e.g. 5 s) on a new bearer: a row of its own, and a false reading
  on a slow start.
- **Word:** `not-resumed`, adopt's third own refusal. It is not revive's `no-session`, which means no logged session.
- **Exit and line:**
  - Exit 1. After the child has released the terminal, one stderr pair: `unable: <name>'s session did not resume`,
    then `hint: run viola adopt from the folder the session was made in, under the claude config it was made with`.
    It names neither the id nor a path.
  - Under `--json`: one document `{"v":1,"error":"not-resumed","detail":null}` on stdout, stderr empty.
  - The alternative honours «nothing past it» and only half meets the entry's «said»: exit 1 and the run-log word
    alone, nothing printed.
- **Recorded:** one `process-exit{subject:"self", exit_code:1, detail:"not-resumed"}` in `run-<name>.ndjson` in place
  of today's exit-0 line. This needs a schema word and an obs Decisions Log entry. No new `events.ndjson` kind: the
  log already shows `session-end` with no `session-start`.
- **Released:** nothing to release. The wrapper has ended, the instance directory stays (as R5a's `adopt-probe`), and
  the name reads `gone`, so a next start takes it over through `collision_check`.

**Fork 3 — hypothesis C (his to answer, through the overseer).** If C holds, does this entry also make the hint name
`claude_env_keep`, or add `CLAUDE_CONFIG_DIR` to a default keep list? The latter widens what crosses to the child: a
boundary widening, and a surface the pinned scope did not name, so the row read runs again.

**PROVISIONAL meanwhile:** fork 2's hint names the claude config generically, and nothing is kept by default.

### Technical leans (for the overseer; decisive, no dialog unless he contests)
- **Fake agent:** a test-only argv flag, read from `fake-args` beside the fake (`tests/support/fake.rs:53-58`), under
  which a `--resume` launch fires SessionEnd and exits 1 with no SessionStart. This is no environment seam.
- **`scripts/r5-adopt.sh`:** it changes.
  - Its live arm keeps typing in `typed-in/` under (i): the folder is the operator's to supply at the next R5a.
  - Its `r5a:` verdict line adds the adopt's own word, `start=<not-resumed|ok>`.
  - Its `--rehearse` adds a third arm over the new fake flag, fired by this chunk up to the capped step with no
    claude.

**Builds whole on the provisional arms: yes.** On to P4 without waiting.
