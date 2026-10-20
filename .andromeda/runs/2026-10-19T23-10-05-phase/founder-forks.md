# Founder forks — chunk 2026-10-19-newer-claude-cli-rows, phase P4 (away form)

Written 2026-10-19 by `/andromeda-phase` P4 while the founder is away (`inputs#I2`). The step stops here, and no
`plan.md` is written until his answer arrives. Live `claude` starts are his cap, given per chunk (`inputs#I1` item 2),
and no live start runs before his word.

## What P3 found (the context for both forks)

- **The four 2.1.302 failures are one stop, not four.** Run B's 2.1.302 transcript holds only the probe prompt and
  the long paste (`inputs#I3` R1). After the long paste's turn, verify's `box_wait` waited its full 120 s for the
  input-box literal and never saw it (`src/cmd/verify/typed.rs:402`). So the tag-like, newline, `/compact` and
  `/clear` pastes were never typed.
  - `tag-escaping` and `paste-newline` find no capture.
  - `local-command-clear` and `local-command-compact` cannot pass without those two captures, because they are
    anchored on them.
  - On 2.1.296 the same sequence ran its five added pastes, long paste to `/clear`, in 16 s (`inputs#I3` R2).
- **The cause, read from the CLI bundle and not yet from a live screen.** The input-box literal is `"for agents"`,
  the footer hint (`crates/viola-agent-claude/src/screen.rs:39`). 2.1.296 always draws it. 2.1.302 hides it once
  `agentsHintExpired` is set: a 10 s value tied to the session's attached clients (`inputs#I3` R4). What attaches in
  Run B is unmeasured. The nearest candidate is Run B's own `claude agents --json` read.
- **It reaches the product.** `run`'s pre-send readiness gate waits for the same literal on a verified CLI
  (`src/run/gate.rs:508`). A 2.1.302 stamped verified with today's literal would refuse a driver's `send`
  `input-not-ready` once the hint is gone. The fix is a measured literal the 2.1.302 resting input box keeps, added
  to the compiled set beside `"for agents"`, and recorded so CI's fake agent replays it.
- **Planned to complete now, with no live start:**
  - the fake agent reproduces the 2.1.302 resting footer, and a `cli_verify` case reads exactly the overseer's
    `stamped …  20 pass  4 fail` with the same four rows red;
  - the `input-box-signature` row also reads a screen past the hint's expiry, so it can no longer pass by timing;
  - a `send` gate case covers that screen;
  - the macOS readiness flake is measured and fixed. The two failing wrappers stalled inside `run.start` before the
    version gate (`inputs#I3` R5). Its phase is measured on `macos-latest` by timing-only pushes before any change.

## Fork 1 — the live round's start cap (his cap)

Each `viola verify` is six starts (Runs A–E plus the print probe). Every round runs under a `target/e2e-home/…` home
and never stamps `~/.viola` (test-plan §3 `run --local-live`). The overseer's own home and his pin to 2.1.296 are his
to move after the wrap. PATH `claude` is 2.1.302; the rig puts that install first and reads `claude --version` by
path before its first start.

1. **19 starts: one screen measurement (1), one record round, one `--local-live`, one re-run held for a red (Recommended)**
   - 1 — the 2.1.302 screen measurement of Fork 2 (option 1).
   - 6 — `viola verify --home <tmpfs home> --record` on 2.1.302 after the literal is added. It stamps 24/0 and
     writes `fixtures/claude/2.1.302/`, which CI then replays.
   - 6 — `scripts/agent-run.sh run --local-live` on 2.1.302: the harness's own verdict, `local-live` passed 1.
   - 6 — held, spent only if the record round reads red: one re-run after a fix, as the compact chunk's cap allowed.

   Why it wins: one start measures the cause before any code changes, one round produces the fixtures CI needs, and
   the harness witnesses it. 2.1.296 needs no live re-run: its bundle draws the hint unconditionally (`inputs#I3` R4)
   and its 24/24 stamp stands in the overseer's home. The trade-off: no live reading on 2.1.296 after the change.
2. **25 starts: option 1 plus a 2.1.296 no-regress verify (6)** — the version the overseer drives on today, read live
   against the changed literal set. It costs six more starts for a version whose footer the bundle already shows
   unchanged.
3. **7 starts: the measurement (1) and the record round (6) only** — no `--local-live` witness and no re-run held.
   The record round's own `stamped 2.1.302  24 pass  0 fail` is the only live verdict, and any red halts the chunk
   for a new word.

## Fork 2 — how the 2.1.302 resting input box is measured (a boundary: "Boundary widening", `.andromeda/playbook.md`)

The literal must be measured on the installed CLI. verify keeps only signature rows, and none of today's literals is
on the 2.1.302 screen once the hint is gone. A scratch read outside `viola verify` needs his fresh ruling: the
security history records the 2026-10-07 scratch answers as spent (`.andromeda/security-plan-amendments.md`, chunk
2026-10-07-live-rows-and-paste-shapes-on-the-dev-host).

1. **One scratch measurement start, footer rows only (Recommended)**
   - One `viola run` start of 2.1.302 under a tmpfs `--home`, its cwd a scratch dir outside the work tree, behind an
     outer PTY (80×24). No key is typed, and the start is killed at the end.
   - At ~2 s, one `viola list` against that home, its `claude agents --json` read being the suspected attach.
   - At ~15 s and ~25 s, the bottom three screen rows written to the chunk's `evidence/`. Only those footer rows are
     written, scrubbed of the home and user word, and never the CLI's text above them.
   - It counts as start 1 of Fork 1.

   Why it wins: it measures both the literal to add and whether a driver's `viola list` alone expires the hint in a
   product session (the exposure of `send`), for one start. The trade-off: one new crossing, footer rows of the live
   CLI screen into `evidence/`, which the wrap escalates as a boundary widening with his answer cited.
2. **No scratch read: take the literal from the bundle's render code** — no new crossing. The record round's
   `input-box-signature` row validates the guess. A wrong guess costs a six-start re-run, and whether `viola list`
   expires the hint stays unmeasured.

**Correction to Fork 2 option 1, made at P4 (2026-10-19T23:34Z) before his answer.** The measurement's cwd cannot be
outside the work tree. The CLI shows its workspace trust modal in an untrusted directory, and with no key typed the
footer never appears. The cwd is a fresh `.viola-footer-<pid>/` dir under the repo root, as verify's Runs B–E use
`.viola-verify-<pid>/` (a trusted parent). It is removed after the reading. Everything else in the option is
unchanged. The CLI writes its own transcript for that session under `~/.claude/projects/`, one file, the class
verify's Run B already leaves.

## The overseer's delegate direction, while he is away (2026-10-19T23:31:00Z) — PROVISIONAL, not his answer

- **Neither fork is answered.** The live-start cap and the Fork 2 crossing are his alone, and both are HELD for his own
  word (a boundary widening is never settled by a delegate). His answer, when it comes, goes under "His answer" below.
- **To keep the work moving meanwhile:** write `plan.md` with the live round (Fork 1) and the screen measurement
  (Fork 2) as the chunk's GATED CRITERION. Its block is "the founder's own word on the live-start cap and on Fork 2
  (`.andromeda/runs/2026-10-19T23-10-05-phase/founder-forks.md`)". Every item that needs no live start is planned to
  complete now: the fake agent's 2.1.302 resting footer, the `cli_verify` 20/4 case, the `input-box-signature` reading
  past the hint's expiry, the `send` gate case, and the macOS readiness flake. A cap whose acceptance rests on the live
  round is claimed gated. Implement runs the non-live work tonight. If his word has not come by the wrap, the wrap takes
  the gated arm, and his word then runs the gated part.
- **Authority:** the Viola overseer as the founder's delegate. PROVISIONAL, for his morning batch.

## His answer

(appended here verbatim when it arrives, then snapped and cited as `inputs#I{n}`; the step resumes on it)
