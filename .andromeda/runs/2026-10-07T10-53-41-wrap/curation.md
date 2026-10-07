CLAUDE.md ecosystem curated:
  Tier 1 (CLAUDE.md USER:session-learnings):  none
  Tier 2 (.claude/rules/*):
    + host-win32.md: "A removal the permission layer denied is never done through another tool: move a scratch home this builder made by full name into target/e2e-home.disk, read back, record where it stands"
      Proof: the overseer's answer at implement (the chunk's inputs#I5), after a plain `rm -r` of an own-made home under `target/e2e-home` was denied; the three homes were moved and read back at 2026-10-07T10:25:45Z (`evidence/hint-window.md`, After the two runs).
    + host-win32.md: "A backgrounded command's output file ends with the harness's own exit line, so a wait that tests the last line for a summary runs to its bound"
      Proof: the implement run's wait on the gate block ran its full 570 s over a block whose summary line had been printed; the file's last line was the harness's `[exited with code 0]` line (friction record 2026-10-07T10:41:20Z-c).
    + testing.md: "A product command a live round starts from the bridge-wrapped builder runs with VIOLA_NAME, VIOLA_DIR and VIOLA_BIN removed and --home given; a viola run meant to record the session's own CLAUDE* names keeps them"
      Proof: research M1 (the three names read in the session's tool environment; `src/cmd/client.rs` reads `VIOLA_NAME` as `from`) and both hint runs, where the claude-child start line listed the ten names the strip removed (`evidence/hint-window.md`).
  Tier 3 (.claude/docs/session-learnings.md): none
  Filters: 0 dup · 2 task-specific (a shell quote slip; a plan-ordering note that is the pipeline's) · 0 conflict · 3 deferred (→ handoff)
  Load-bearing: "a product command a live round starts from the bridge-wrapped builder…" → the next promotable route entry (the hint-gate entry this wrap mints: its by-path re-verify is started from this session)
  CLAUDE.md size: 124/200 · T1 1.8 KB, 0 over 600 B

Deferred (the max-3 cap), carried in the handoff:
  - `inputs.py snap`: an answer that arrives in the session is snapshotted with `--message-file … --origin …`; `--source` refuses a file inside the repository and one under the temp dir.
  - A subagent's return can arrive with tag-like text neutralised (a backslash after the angle bracket), which on this project reads as the CLI's own escaped form: never paste a detector's change line that holds a tag.
  - Extension candidate for host-win32.md 2026-10-07 (a process named `viola`): a `pgrep -f` on a script name also reads other projects' sessions; decide by the process's cwd.

Recurrence, not a duplicate:
  - `recurrence-despite-learning: host-win32.md 2026-09-28/29` — the Bash guard refused a `cat` heredoc with a file target again (once, at implement).
